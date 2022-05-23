use crate::net::{NetMessageHeader, NetworkError, RawNetMessage};
use crate::eresult::EResult;
use crate::proto::{
    enums_clientserver::EMsg,
    steammessages_clientserver_login::CMsgClientLogonResponse,
    steammessages_base::CMsgIPAddress,
    steammessages_clientserver_login::CMsgClientLogon,
};
use futures_util::{Sink, SinkExt};
use steamid_ng::{AccountType, Instance, SteamID, Universe};
use thiserror::Error;
use tokio_stream::{Stream, StreamExt};

type Result<T, E = SessionError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Network error: {0:#}")]
    Network(#[from] NetworkError),
    #[error("Login failed: {:?}", .0)]
    Login(EResult),
}

#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: i32,
    last_source_id: u64,
    pub steam_id: SteamID,
    pub out_of_game_heartbeat_seconds: i32,
}

impl Session {
    
    pub fn header(&mut self) -> NetMessageHeader {
        self.last_source_id += 1;
        NetMessageHeader {
            session_id: self.session_id,
            source_job_id: self.last_source_id,
            target_job_id: u64::MAX,
            steam_id: self.steam_id,
            target_job_name: None,
            routing_appid: None,
        }
    }
}

pub async fn anonymous<
    Read: Stream<Item = Result<RawNetMessage, NetworkError>> + Unpin,
    Write: Sink<RawNetMessage, Error = NetworkError> + Unpin,
>(
    read: &mut Read,
    write: &mut Write,
) -> Result<Session> {
    let logon = get_anon_login_msg();
    let steamid = SteamID::new(0, Instance::All, AccountType::AnonUser, Universe::Public);
    
    login(
        read,
        write,
        &steamid,
        logon
    )
    .await
}

pub async fn logged_in<
    Read: Stream<Item = Result<RawNetMessage, NetworkError>> + Unpin,
    Write: Sink<RawNetMessage, Error = NetworkError> + Unpin,
>(
    read: &mut Read,
    write: &mut Write,
    logon: CMsgClientLogon,
    steamid: &SteamID,
) -> Result<Session> {
    login(
        read,
        write,
        steamid,
        logon
    )
    .await
}

fn get_anon_login_msg() -> CMsgClientLogon {
    let mut logon = CMsgClientLogon::new();
    logon.set_protocol_version(65580);
    logon.set_client_os_type(203);
    logon.set_anon_user_target_account_name(String::from("anonymous"));
    logon.set_should_remember_password(false);
    logon.set_supports_rate_limit_response(false);

    let mut ip = CMsgIPAddress::new();
    ip.set_v4(0);
    logon.set_obfuscated_private_ip(ip);
    logon.set_client_language(String::new());
    logon.set_machine_name(String::new());
    logon.set_steamguard_dont_remember_computer(false);
    logon.set_chat_mode(2);
    
    logon
}

pub async fn login<
    Read: Stream<Item = Result<RawNetMessage, NetworkError>> + Unpin,
    Write: Sink<RawNetMessage, Error = NetworkError> + Unpin,
>(
    read: &mut Read,
    write: &mut Write,
    steamid: &SteamID,
    logon: CMsgClientLogon,
) -> Result<Session> {
    let header = NetMessageHeader {
        session_id: 0,
        source_job_id: u64::MAX,
        target_job_id: u64::MAX,
        steam_id: *steamid,
        target_job_name: None,
        routing_appid: None,
    };

    let msg = RawNetMessage::from_message(header, logon)?;
    write.send(msg).await?;

    while let Some(result) = read.next().await {
        let msg: RawNetMessage = result?;
        
        match msg.kind {
            EMsg::k_EMsgClientLogOnResponse => {
                let session_id = msg.header.session_id;
                let steam_id = msg.header.steam_id;
                let response = msg.into_message::<CMsgClientLogonResponse>()?;
                let out_of_game_heartbeat_seconds = response.get_out_of_game_heartbeat_seconds();
                let eresult: EResult = response.get_eresult().into();
                
                if eresult == EResult::OK {
                    return Ok(Session {
                        session_id,
                        steam_id,
                        last_source_id: 0,
                        out_of_game_heartbeat_seconds,
                    });
                } else {
                    return Err(SessionError::Login(eresult));
                }
            }
            _ => {}
        }
    }
    Err(NetworkError::EOF.into())
}
