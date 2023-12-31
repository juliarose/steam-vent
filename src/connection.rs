use crate::auth::{begin_password_auth, AuthConfirmationHandler};
use crate::gc::ClientToGCMessage;
use crate::message::{
    NetMessage, ServiceMethodMessage, ServiceMethodNotification, ServiceMethodResponseMessage,
};
use futures_util::future::Join;
use steam_vent_crypto::create_sha1;
use crate::net::{NetMessageHeader, NetworkError, RawNetMessage};
use crate::proto::enums_clientserver::EMsg;
use crate::proto::steammessages_clientserver_login::CMsgClientHeartBeat;
use crate::serverlist::ServerList;
use crate::service_method::ServiceMethodRequest;
use crate::session::{anonymous, hello, login, ConnectionError, LoginOptions, Session};
use crate::transport::websocket::connect;
use dashmap::DashMap;
use futures_util::{Sink, SinkExt};
use steam_vent_proto::steammessages_clientserver_2::{CMsgClientUpdateMachineAuth, CMsgClientUpdateMachineAuthResponse};
use steam_vent_proto::steammessages_clientserver_login::{CMsgClientLogon, CMsgClientNewLoginKey, CMsgClientNewLoginKeyAccepted};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use steamid_ng::SteamID;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};
use tokio::task::{spawn, JoinHandle};
use tokio::time::{sleep, timeout};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};
use tracing::{debug, error};

type Result<T, E = NetworkError> = std::result::Result<T, E>;

pub struct Connection {
    pub(crate) session: Session,
    filter: MessageFilter,
    rest: mpsc::Receiver<Result<RawNetMessage>>,
    write: Arc<Mutex<Box<dyn Sink<RawNetMessage, Error = NetworkError> + Unpin + Send + Sync>>>,
    timeout: Duration,
}

impl Connection {
    async fn connect(
        server_list: ServerList,
    ) -> Result<Self, ConnectionError> {
        let addr = server_list.pick_ws().await;
        let (read, write) = connect(&addr).await?;
        let (filter, rest) = MessageFilter::new(read);
        let mut connection = Connection {
            session: Session::default(),
            filter,
            rest,
            write: Arc::new(Mutex::new(Box::new(write))),
            timeout: Duration::from_secs(10),
        };
        hello(&mut connection).await?;
        Ok(connection)
    }
    
    pub async fn reconnect<H: AuthConfirmationHandler>(
        &mut self,
        server_list: ServerList,
        account: &str,
        password: &str,
        mut confirmation_handler: H,
    ) -> Result<(), ConnectionError> {
        let addr = server_list.pick_ws().await;
        let (read, write) = connect(&addr).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        *(self.write.lock().await) = Box::new(write);
        self.filter = filter;
        self.rest = rest;
        
        let begin = begin_password_auth(self, account, password).await?;
        let steam_id = SteamID::from(begin.steam_id());
        let confirmation_action = confirmation_handler
            .handle_confirmation(begin.allowed_confirmations())
            .await;
        let pending = begin
            .submit_confirmation(self, confirmation_action)
            .await?;
        let tokens = pending.wait_for_tokens(self).await?;
        self.session = login(
            self,
            account,
            steam_id,
            // yes we send the refresh token as access token, yes it makes no sense, yes this is actually required
            tokens.refresh_token.as_ref(),
        )
        .await?;
        // heartbeat is already setup
        
        Ok(())
    }
    
    pub async fn anonymous(server_list: ServerList) -> Result<Self, ConnectionError> {
        let mut connection = Self::connect(server_list).await?;
        connection.session = anonymous(&mut connection).await?;
        connection.setup_heartbeat();

        Ok(connection)
    }
    
    pub async fn login<H: AuthConfirmationHandler>(
        server_list: ServerList,
        account: &str,
        password: &str,
        options: LoginOptions,
        mut confirmation_handler: H,
    ) -> Result<Self, ConnectionError> {
        let mut connection = Self::connect(server_list).await?;
        let begin = begin_password_auth(&mut connection, account, password).await?;
        let steam_id = SteamID::from(begin.steam_id());
        let confirmation_action = confirmation_handler
            .handle_confirmation(begin.allowed_confirmations())
            .await;
        let pending = begin
            .submit_confirmation(&mut connection, confirmation_action)
            .await?;
        let tokens = pending.wait_for_tokens(&mut connection).await?;
        connection.session = login(
            &mut connection,
            account,
            steam_id,
            // yes we send the refresh token as access token, yes it makes no sense, yes this is actually required
            tokens.refresh_token.as_ref(),
        )
        .await?;
        connection.setup_heartbeat();

        Ok(connection)
    }
    
    fn setup_sentry(&self) -> JoinHandle<()> {
        let write = self.write.clone();
        let mut header = NetMessageHeader {
            session_id: self.session.session_id,
            source_job_id: u64::MAX,
            target_job_id: u64::MAX,
            steam_id: self.steam_id(),
            ..NetMessageHeader::default()
        };
        let sentry_message = self.filter.one_kind(EMsg::k_EMsgClientUpdateMachineAuth);
        
        spawn(async move {
            if let Ok(msg) = sentry_message.await {
                let source_job_id: u64 = msg.header.source_job_id;
                
                if let Ok(message) =  msg.into_message::<CMsgClientUpdateMachineAuth>() {
                    let mut req = CMsgClientUpdateMachineAuthResponse::new();
                    let bytes = message.bytes();
                    let hash = create_sha1(bytes);
                    
                    req.set_sha_file(hash);
                    header.target_job_id = source_job_id;
                    
                    if let Ok(msg) = RawNetMessage::from_message(header, req) {
                        let mut writer = write.lock().await;
                        
                        if let Err(e) = writer.send(msg).await {
                            error!(error = ?e, "Failed to send heartbeat message");
                        }
                    }
                }
            }
        })
    }
    
    fn setup_login_key(&self) -> JoinHandle<()> {
        let write = self.write.clone();
        let header = NetMessageHeader {
            session_id: self.session.session_id,
            source_job_id: u64::MAX,
            target_job_id: u64::MAX,
            steam_id: self.steam_id(),
            ..NetMessageHeader::default()
        };
        let login_key_message = self.filter.one_kind(EMsg::k_EMsgClientNewLoginKey);
        
        spawn(async move {
            if let Ok(msg) = login_key_message.await {
                if let Ok(message) =  msg.into_message::<CMsgClientNewLoginKey>() {
                    let mut req = CMsgClientNewLoginKeyAccepted::new();
                    let login_key = message.login_key();
                    let unique_id = message.unique_id();
                    let mut req = CMsgClientNewLoginKeyAccepted::new();
                    
                    req.set_unique_id(unique_id);
                    
                    if let Ok(msg) = RawNetMessage::from_message(header, req) {
                        let mut writer = write.lock().await;
                        
                        if let Err(e) = writer.send(msg).await {
                            error!(error = ?e, "Failed to send heartbeat message");
                        }
                    }
                }
            }
        })
    }
    
    fn setup_heartbeat(&self) -> JoinHandle<()> {
        let write = self.write.clone();
        let interval = self.session.heartbeat_interval;
        let header = NetMessageHeader {
            session_id: self.session.session_id,
            source_job_id: u64::MAX,
            target_job_id: u64::MAX,
            steam_id: self.steam_id(),
            ..NetMessageHeader::default()
        };
        
        spawn(async move {
            loop {
                sleep(interval).await;
                match RawNetMessage::from_message(header.clone(), CMsgClientHeartBeat::default()) {
                    Ok(msg) => {
                        let mut writer = write.lock().await;
                        if let Err(e) = writer.send(msg).await {
                            error!(error = ?e, "Failed to send heartbeat message");
                        }
                    }
                    Err(e) => {
                        error!(error = ?e, "Failed to prepare heartbeat message")
                    }
                }
            }
        })
    }

    pub fn steam_id(&self) -> SteamID {
        self.session.steam_id
    }

    fn prepare(&self) -> NetMessageHeader {
        self.session.header()
    }

    pub async fn send<Msg: NetMessage>(&self, header: NetMessageHeader, msg: Msg) -> Result<()> {
        let msg = RawNetMessage::from_message(header, msg)?;
        self.write.lock().await.send(msg).await?;
        Ok(())
    }
    
    pub async fn send_gc(
        &mut self,
        msg: ClientToGCMessage,
    ) -> Result<u64> {
        let mut header = self.session.header();
        
        header.routing_appid = Some(msg.0.appid());
        
        let id = header.source_job_id;
        let msg = RawNetMessage::from_message(header, msg)?;
        
        self.write.lock().await.send(msg).await?;
        Ok(id)
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub async fn service_method<Msg: ServiceMethodRequest>(
        &self,
        msg: Msg,
    ) -> Result<Msg::Response> {
        let header = self.prepare();
        let recv = self.filter.on_job_id(header.source_job_id);
        self.send(header, ServiceMethodMessage(msg)).await?;
        let message = timeout(self.timeout, recv)
            .await
            .map_err(|_| NetworkError::Timeout)?
            .map_err(|_| NetworkError::EOF)?
            .into_message::<ServiceMethodResponseMessage>()?;
        message.into_response::<Msg>()
    }

    pub(crate) async fn service_method_un_authenticated<Msg: ServiceMethodRequest>(
        &self,
        msg: Msg,
    ) -> Result<Msg::Response> {
        let header = self.prepare();
        let recv = self.filter.on_job_id(header.source_job_id);
        let msg = RawNetMessage::from_message_with_kind(
            header,
            ServiceMethodMessage(msg),
            EMsg::k_EMsgServiceMethodCallFromClientNonAuthed,
        )?;
        self.write.lock().await.send(msg).await?;
        let message = timeout(self.timeout, recv)
            .await
            .map_err(|_| NetworkError::Timeout)?
            .map_err(|_| NetworkError::Timeout)?
            .into_message::<ServiceMethodResponseMessage>()?;
        message.into_response::<Msg>()
    }

    pub async fn next(&mut self) -> Result<RawNetMessage> {
        self.rest.recv().await.ok_or(NetworkError::EOF)?
    }

    pub fn on<T: ServiceMethodRequest>(&self) -> impl Stream<Item = Result<T>> {
        BroadcastStream::new(self.filter.on_notification(T::REQ_NAME))
            .filter_map(|res| res.ok())
            .map(|raw| raw.into_notification())
    }

    pub fn one<T: NetMessage>(&self) -> impl Future<Output = Result<(NetMessageHeader, T)>> {
        // async block instead of async fn so we don't have to tie the lifetime of the returned future
        // to the lifetime of &self
        let fut = self.filter.one_kind(T::KIND);
        async move {
            let raw = fut.await.map_err(|_| NetworkError::EOF)?;
            Ok((raw.header.clone(), raw.into_message()?))
        }
    }
}

#[derive(Clone)]
struct MessageFilter {
    job_id_filters: Arc<DashMap<u64, oneshot::Sender<RawNetMessage>>>,
    notification_filters: Arc<DashMap<&'static str, broadcast::Sender<ServiceMethodNotification>>>,
    kind_filters: Arc<DashMap<EMsg, broadcast::Sender<RawNetMessage>>>,
    oneshot_kind_filters: Arc<DashMap<EMsg, oneshot::Sender<RawNetMessage>>>,
}

impl MessageFilter {
    pub fn new<Input: Stream<Item = Result<RawNetMessage>> + Send + Unpin + 'static>(
        mut source: Input,
    ) -> (Self, mpsc::Receiver<Result<RawNetMessage>>) {
        let (rest_tx, rx) = mpsc::channel(16);
        let filter = MessageFilter {
            job_id_filters: Default::default(),
            kind_filters: Default::default(),
            notification_filters: Default::default(),
            oneshot_kind_filters: Default::default(),
        };

        let filter_send = filter.clone();
        spawn(async move {
            while let Some(res) = source.next().await {
                if let Ok(message) = res {
                    debug!(job_id = message.header.target_job_id, kind = ?message.kind, "processing message");
                    if let Some((_, tx)) = filter_send
                        .job_id_filters
                        .remove(&message.header.target_job_id)
                    {
                        tx.send(message).ok();
                    } else if let Some((_, tx)) =
                        filter_send.oneshot_kind_filters.remove(&message.kind)
                    {
                        tx.send(message).ok();
                    } else if message.kind == EMsg::k_EMsgServiceMethod {
                        if let Ok(notification) =
                            message.into_message::<ServiceMethodNotification>()
                        {
                            debug!(
                                job_name = notification.job_name.as_str(),
                                "processing notification"
                            );
                            if let Some(tx) = filter_send
                                .notification_filters
                                .get(notification.job_name.as_str())
                            {
                                tx.send(notification).ok();
                            }
                        }
                    } else if let Some(tx) = filter_send.kind_filters.get(&message.kind) {
                        tx.send(message).ok();
                    } else {
                        rest_tx.send(Ok(message)).await.ok();
                    }
                } else {
                    rest_tx.send(res).await.ok();
                }
            }
        });
        (filter, rx)
    }

    pub fn on_job_id(&self, id: u64) -> oneshot::Receiver<RawNetMessage> {
        let (tx, rx) = oneshot::channel();
        self.job_id_filters.insert(id, tx);
        rx
    }

    pub fn on_notification(
        &self,
        job_name: &'static str,
    ) -> broadcast::Receiver<ServiceMethodNotification> {
        let tx = self
            .notification_filters
            .entry(job_name)
            .or_insert_with(|| broadcast::channel(16).0);
        tx.subscribe()
    }

    pub fn one_kind(&self, kind: EMsg) -> oneshot::Receiver<RawNetMessage> {
        let (tx, rx) = oneshot::channel();
        self.oneshot_kind_filters.insert(kind, tx);
        rx
    }
}
