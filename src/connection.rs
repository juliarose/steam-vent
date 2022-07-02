use crate::{
    message::{flatten_multi, ClientSetIgnoreFriend, NetMessage, ServiceMethodResponseMessage},
    net::{connect, NetworkError, RawNetMessage},
    service_method::ServiceMethodRequest,
    session::{anonymous, logged_in, Session, SessionError},
    enums::EPersonaState,
    gc::ClientToGCMessage,
    login,
};
use std::{sync::Arc, time::Duration};
use steamid_ng::SteamID;
use dashmap::DashMap;
use futures_sink::Sink;
use futures_util::SinkExt;
use steam_vent_proto::enums_clientserver::EMsg;
use tokio::{
    sync::{broadcast, mpsc, oneshot},
    task::spawn,
    time::timeout,
};
use tokio_stream::{Stream, StreamExt};
use protobuf::RepeatedField;
use crate::proto::{
    steammessages_clientserver_friends::{
        CMsgClientAddFriend,
        CMsgClientChangeStatus,
        CMsgClientRemoveFriend,
    },
    steammessages_clientserver::{
        CMsgClientGamesPlayed,
        CMsgClientGamesPlayed_GamePlayed,
    },
    steammessages_clientserver_login::{
        CMsgClientAccountInfo,
        CMsgClientHeartBeat,
        CMsgClientLogOff,
        CMsgClientLogon,
        CMsgClientRequestWebAPIAuthenticateUserNonce,
    },
    steammessages_friendmessages_steamclient::{
        CFriendMessages_SendMessage_Request,
        CFriendMessages_SendMessage_Response,
    },
};

type Result<T, E = NetworkError> = std::result::Result<T, E>;
type Login = (Connection, mpsc::Receiver<Result<RawNetMessage>>);

const SERVER_IP: &str = "162.254.196.67:27017";

pub struct Connection {
    pub session: Session,
    filter: Arc<MessageFilter>,
    write: Box<dyn Sink<RawNetMessage, Error = NetworkError> + Unpin + Send + Sync>,
}

impl Connection {
    /// Creates the base message to login. Pass this to [`Connection::login`] to login.  If you 
    /// are logging in with a two factor code or a login key, you will need to modify this 
    /// message to include them.
    pub fn default_login_message(
        account_name: String,
        password: String,
    ) -> CMsgClientLogon {
        login::create_logon(
            account_name,
            password,
        )
    }
    
    pub async fn anonymous() -> Result<Login, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = anonymous(&mut read, &mut write).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        Ok((Connection {
            session,
            filter: Arc::new(filter),
            write: Box::new(write),
        }, rest))
    }
  
    pub async fn login(
        credentials: CMsgClientLogon,
    ) -> Result<Login, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = logged_in(&mut read, &mut write, credentials).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        Ok((Connection {
            session,
            filter: Arc::new(filter),
            write: Box::new(write),
        }, rest))
    }
    
    pub async fn reconnect(
        &mut self,
        credentials: CMsgClientLogon,
    ) -> Result<mpsc::Receiver<Result<RawNetMessage>>, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = logged_in(&mut read, &mut write, credentials).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        self.session = session;
        self.write = Box::new(write);
        self.filter = Arc::new(filter);
        
        Ok(rest)
    }

    pub async fn send<Msg: NetMessage>(
        &mut self,
        msg: Msg,
    ) -> Result<u64> {
        let header = self.session.header();
        let id = header.source_job_id;
        let msg = RawNetMessage::from_message(header, msg)?;
        
        self.write.send(msg).await?;
        Ok(id)
    }

    pub async fn send_gc(
        &mut self,
        msg: ClientToGCMessage,
    ) -> Result<u64> {
        let mut header = self.session.header();
        
        header.routing_appid = Some(msg.0.get_appid());
        
        let id = header.source_job_id;
        let msg = RawNetMessage::from_message(header, msg)?;
        
        self.write.send(msg).await?;
        Ok(id)
    }

    pub async fn send_response<Msg: NetMessage>(
        &mut self,
        msg: Msg,
        target_job_id: u64,
    ) -> Result<u64> {
        let mut header = self.session.header();
        
        header.target_job_id = target_job_id;
        
        let id = header.source_job_id;
        let msg = RawNetMessage::from_message(header, msg)?;
        self.write.send(msg).await?;
        Ok(id)
    }
    
    pub async fn service_method<Msg: ServiceMethodRequest>(
        &mut self,
        msg: Msg,
    ) -> Result<oneshot::Receiver<Result<Msg::Response>>>
    where
        <Msg as ServiceMethodRequest>::Response: Send + Sized
    {
        async fn wait_for_response<Msg: ServiceMethodRequest>(
            filter: &MessageFilter,
            job_id: u64,
        ) -> Result<Msg::Response> {
            let raw_message = timeout(Duration::from_secs(10), filter.on_job_id(job_id))
                .await
                .map_err(|_| NetworkError::Timeout)?
                .map_err(|_| NetworkError::Timeout)?;            
            let message = raw_message.into_message::<ServiceMethodResponseMessage>()?;
            
            message.into_response::<Msg>()
        }
        
        let job_id = self.send(msg).await?;
        let filter = Arc::clone(&self.filter);
        let (tx, rx) = oneshot::channel::<Result<Msg::Response>>();
        
        spawn(async move {
            let response = wait_for_response::<Msg>(
                &filter,
                job_id
            ).await;
            let _ = tx.send(response);
        });
        
        // this could return the JoinHandle or a receiver
        // either one works, I guess?
        Ok(rx)
    }
    
    /// Sends a heartbeat to keep the connection alive.
    pub async fn send_heartbeat(
        &mut self,
    ) -> Result<u64> {
        self.send(CMsgClientHeartBeat::new()).await?;
        
        Ok(self.session.out_of_game_heartbeat_seconds as u64)
    }
    
    /// Sends message to disconnect connection.
    pub async fn disconnect(
        &mut self,
    ) -> Result<()> {
        self.send(CMsgClientLogOff::new()).await?;
        
        Ok(())
    }
    
    /// Adds a friend.
    pub async fn add_friend(
        &mut self,
        friend: SteamID,
    ) -> Result<u64> {
        let mut req = CMsgClientAddFriend::new();
        
        req.set_steamid_to_add(u64::from(friend));
        
        let job_id = self.send(req).await?;
        
        Ok(job_id)
    }
    
    /// Removes a friend.
    pub async fn remove_friend(
        &mut self,
        friend: SteamID,
    ) -> Result<u64> {
        let mut req = CMsgClientRemoveFriend::new();
        
        req.set_friendid(u64::from(friend));
        
        let job_id = self.send(req).await?;
        
        Ok(job_id)
    }
    
    /// Blocks a user.
    pub async fn block_user(
        &mut self,
        steamid: SteamID,
    ) -> Result<u64> {
        let req = ClientSetIgnoreFriend {
            steamid: u64::from(self.session.steam_id),
            steamid_other: u64::from(steamid),
            block: 1,
        };
        let job_id = self.send(req).await?;
        
        Ok(job_id)
    }
    
    /// Unblocks a user.
    pub async fn unblock_user(
        &mut self,
        steamid: SteamID,
    ) -> Result<u64> {
        let req = ClientSetIgnoreFriend {
            steamid: u64::from(self.session.steam_id),
            steamid_other: u64::from(steamid),
            block: 0,
        };
        let job_id = self.send(req).await?;
        
        Ok(job_id)
    }
    
    /// Sets the persona state e.g. "Online".
    pub async fn set_persona_state(
        &mut self,
        persona_state: EPersonaState,
    ) -> Result<u64> {
        let mut req = CMsgClientChangeStatus::new();
        
        req.set_persona_state(persona_state as u32);
        
        self.send(req).await
    }
    
    /// Sets the persona name.
    pub async fn set_persona_name(
        &mut self,
        persona_name: String,
    ) -> Result<u64> {
        let mut req = CMsgClientChangeStatus::new();
        
        req.set_player_name(persona_name);
        
        self.send(req).await
    }
    
    /// Requests the message to authenticate your web session (cookies and sessionid).
    pub async fn request_web_api_authenticate_user_nonce(&mut self) -> Result<u64> {
        self.send(CMsgClientRequestWebAPIAuthenticateUserNonce::new()).await
    }
    
    /// Sends a chat message to the given user.
    pub async fn chat_message(
        &mut self,
        friend: SteamID,
        message: String,
    ) -> Result<oneshot::Receiver<Result<CFriendMessages_SendMessage_Response>>> {
        let mut req = CFriendMessages_SendMessage_Request::new();
        
        req.set_steamid(u64::from(friend));
        // EChatEntryType::ChatMsg
        req.set_chat_entry_type(1);
        req.set_message(message);
        req.set_contains_bbcode(false);
        
        self.service_method(req).await
    }
    
    /// Sets games played. Can be any number of games.
    pub async fn set_games_played(
        &mut self,
        games: &[u64],
    ) -> Result<u64> {
        let mut message = CMsgClientGamesPlayed::new();
        let games_played = games
            .iter()
            .map(|game_id| {
                let mut game = CMsgClientGamesPlayed_GamePlayed::new();
                
                game.set_game_id(*game_id);
                game
            })
            .collect::<Vec<_>>();
        
        message.set_games_played(RepeatedField::from_vec(games_played));
        
        self.send(message).await
    }
}

#[derive(Clone)]
struct MessageFilter {
    job_id_filters: Arc<DashMap<u64, oneshot::Sender<RawNetMessage>>>,
    kind_filters: Arc<DashMap<EMsg, broadcast::Sender<RawNetMessage>>>,
}

impl MessageFilter {
    pub fn new<Input: Stream<Item = Result<RawNetMessage>> + Send + Unpin + 'static>(
        mut source: Input,
    ) -> (Self, mpsc::Receiver<Result<RawNetMessage>>) {
        let (rest_tx, rx) = mpsc::channel(16);
        let filter = MessageFilter {
            job_id_filters: Default::default(),
            kind_filters: Default::default(),
        };

        let filter_send = filter.clone();
        spawn(async move {
            while let Some(res) = source.next().await {
                if let Ok(message) = res {
                    if let Some((_, tx)) = filter_send
                        .job_id_filters
                        .remove(&message.header.target_job_id)
                    {
                        tx.send(message).ok();
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

    pub fn on_kind(&self, kind: EMsg) -> broadcast::Receiver<RawNetMessage> {
        let tx = self
            .kind_filters
            .entry(kind)
            .or_insert_with(|| broadcast::channel(16).0);
        tx.subscribe()
    }
}
