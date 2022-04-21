use crate::message::{flatten_multi, NetMessage, ServiceMethodResponseMessage};
use crate::net::{connect, NetworkError, RawNetMessage};
use crate::service_method::ServiceMethodRequest;
use crate::session::{anonymous, logged_in, Session, SessionError};
use crate::enums::EPersonaState;
use crate::game_coordinator::ClientToGCMessage;
use steamid_ng::{SteamID};
use dashmap::DashMap;
use futures_sink::Sink;
use futures_util::SinkExt;
use std::sync::Arc;
use std::time::Duration;
use steam_vent_proto::enums_clientserver::EMsg;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::timeout;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use protobuf::RepeatedField;
use crate::proto::{
    steammessages_clientserver_friends::{
        CMsgClientAddFriend,
        CMsgClientChangeStatus
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
    filter: MessageFilter,
    write: Box<dyn Sink<RawNetMessage, Error = NetworkError> + Unpin + Send + Sync>
}

impl Connection {
    pub async fn anonymous() -> Result<Login, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = anonymous(&mut read, &mut write).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        Ok((Connection {
            session,
            filter,
            write: Box::new(write),
        }, rest))
    }
  
    pub async fn login(
        credentials: CMsgClientLogon,
        steamid: &SteamID,
    ) -> Result<Login, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = logged_in(&mut read, &mut write, credentials, steamid).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        Ok((Connection {
            session,
            filter,
            write: Box::new(write),
        }, rest))
    }
    
    pub async fn reconnect(
        &mut self,
        credentials: CMsgClientLogon,
        steamid: &SteamID,
    ) -> Result<mpsc::Receiver<Result<RawNetMessage>>, SessionError> {
        let (read, mut write) = connect(SERVER_IP).await?;
        let mut read = flatten_multi(read);
        let session = logged_in(&mut read, &mut write, credentials, steamid).await?;
        let (filter, rest) = MessageFilter::new(read);
        
        self.session = session;
        self.write = Box::new(write);
        self.filter = filter;
        
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
    ) -> Result<Msg::Response> {
        let job_id = self.send(msg).await?;
        let raw_message = timeout(Duration::from_secs(10), self.filter.on_job_id(job_id))
            .await
            .map_err(|_| NetworkError::Timeout)?
            .map_err(|_| NetworkError::Timeout)?;
        let message = raw_message.into_message::<ServiceMethodResponseMessage>()?;
        
        message.into_response::<Msg>()
    }
    
    pub async fn send_heartbeat(
        &mut self,
    ) -> Result<u64> {
        self.send(CMsgClientHeartBeat::new()).await?;
        
        Ok(self.session.out_of_game_heartbeat_seconds as u64)
    }
    
    pub async fn disconnect(
        &mut self,
    ) -> Result<()> {
        self.send(CMsgClientLogOff::new()).await?;
        
        Ok(())
    }
    
    pub async fn add_friend(
        &mut self,
        friend: &SteamID,
    ) -> Result<u64> {
        let mut req = CMsgClientAddFriend::new();
        
        req.set_steamid_to_add(u64::from(*friend));
        
        let job_id = self.send(req).await?;
        
        Ok(job_id)
    }
    
    pub async fn set_persona_state(
        &mut self,
        persona_state: &EPersonaState,
    ) -> Result<u64> {
        let mut req = CMsgClientChangeStatus::new();
        
        req.set_persona_state(persona_state.clone() as u32);
        
        self.send(req).await
    }
    
    pub async fn set_persona_name(
        &mut self,
        persona_name: &str,
    ) -> Result<u64> {
        let mut req = CMsgClientAccountInfo::new();
        
        req.set_persona_name(persona_name.into());
        
        self.send(req).await
    }
    
    pub async fn request_web_api_authenticate_user_nonce(&mut self) -> Result<u64> {
        self.send(CMsgClientRequestWebAPIAuthenticateUserNonce::new()).await
    }
    
    pub async fn chat_message(
        &mut self,
        friend: &SteamID,
        message: &str,
    ) -> Result<CFriendMessages_SendMessage_Response> {
        let mut req = CFriendMessages_SendMessage_Request::new();
        
        req.set_steamid(u64::from(friend.clone()));
        // EChatEntryType::ChatMsg
        req.set_chat_entry_type(1);
        req.set_message(message.to_string());
        req.set_contains_bbcode(false);
        
        self.service_method(req).await
    }
    
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
