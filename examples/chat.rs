use std::io::stdin;
use steam_vent::auth::{AuthConfirmationHandler, ConfirmationAction, ConfirmationMethodClass, ConfirmationMethod, SteamGuardToken};
use steam_vent::{Connection, ConnectionError, ServerList};
use steam_vent_proto::steammessages_clientserver_login::CMsgClientAccountInfo;
use steam_vent_proto::steammessages_friendmessages_steamclient::{
    CFriendMessages_IncomingMessage_Notification, CFriendMessages_SendMessage_Request,
};
use steamid_ng::SteamID;
use tokio::spawn;
use tokio_stream::StreamExt;
use async_trait::async_trait;
use another_steam_totp::generate_auth_code;

pub struct ShareSecretConfirmationHandler {
    shared_secret: String,
}

#[async_trait]
impl AuthConfirmationHandler for ShareSecretConfirmationHandler {
    async fn handle_confirmation(
        &mut self,
        allowed_confirmations: Vec<ConfirmationMethod>,
    ) -> ConfirmationAction {
        for method in allowed_confirmations {
            println!("{:?}", method);
            if method.class() == ConfirmationMethodClass::Code {
                // if this fails, there is something wrong with the secret and the program should crash
                let code = generate_auth_code(&self.shared_secret, None).unwrap();
                let token = SteamGuardToken(code);
                
                return ConfirmationAction::GuardToken(token, method.guard_type());
            }
        }
        
        ConfirmationAction::NotSupported
    }
}

#[tokio::main]
async fn main() -> Result<(), ConnectionError> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    
    let account = std::env::var("ACCOUNT").expect("no ACCOUNT env variable set");
    let password = std::env::var("PASSWORD").expect("no PASSWORD env variable set");
    let target_steam_id = SteamID::try_from(
        std::env::var("TARGET_STEAMID").expect("no TARGET_STEAMID env variable set").as_ref()
    ).expect("invalid steam id");
    let shared_secret = std::env::var("SHARED_SECRET").expect("no SHARED_SECRET env variable set");
    let server_list = ServerList::discover().await?;
    let connection = Connection::login(
        server_list,
        &account,
        &password,
        Default::default(),
        ShareSecretConfirmationHandler { shared_secret },
    ).await?;
    let mut incoming_messages = connection.on::<CFriendMessages_IncomingMessage_Notification>();
    let account_info_message = connection.one::<CMsgClientAccountInfo>();
    let _handles = vec![
        spawn(async move {
            while let Some(Ok(incoming)) = incoming_messages.next().await {
                println!("{}: {}", incoming.steamid_friend(), incoming.message());
            }
        }),
        spawn(async move {
            if let Ok((_header, account_info)) = account_info_message.await {
                let persona_name = account_info.persona_name();
                
                println!("persona_name: {persona_name}");
            }
        })
    ];
    let mut read_buff = String::with_capacity(32);
    
    loop {
        read_buff.clear();
        stdin().read_line(&mut read_buff).expect("stdin error");
        let input = read_buff.trim();
        if !input.is_empty() {
            let req = CFriendMessages_SendMessage_Request {
                steamid: Some(target_steam_id.into()),
                message: Some(input.into()),
                chat_entry_type: Some(1),
                ..CFriendMessages_SendMessage_Request::default()
            };
            connection.service_method(req).await?;
        }
    }
}
