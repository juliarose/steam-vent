/// Demonstrates logging in and replying to the chat message "hello"
use steam_vent::connection::Connection;
use steamid_ng::SteamID;
use steam_vent::proto::{
    enums_clientserver::EMsg,
    steammessages_clientserver_login::CMsgClientAccountInfo,
    steammessages_friendmessages_steamclient::CFriendMessages_IncomingMessage_Notification,
};
use steam_vent::{
    message::ServiceMethodRequestMessage,
    net::{NetworkError, RawNetMessage},
    service_method::ServiceMethodRequest,
};
use tokio::sync::mpsc::{self, Sender, Receiver};

#[derive(Debug)]
enum Message {
    ChatMessage {
        message: String,
        steamid: SteamID,
    },
    SendHeartbeat,
}

async fn read_messages(
    mut rest: Receiver<Result<RawNetMessage, NetworkError>>,
    tx: Sender<Message>,
) -> Result<(), NetworkError> {
    while let Some(msg) = rest.recv().await {
        let msg = msg?;
        
        match msg.kind {
            EMsg::k_EMsgServiceMethod => service_method(msg, &tx).await?,
            EMsg::k_EMsgClientAccountInfo => {
                let message = msg.into_message::<CMsgClientAccountInfo>()?;
                let persona_name = message.get_persona_name();
                
                println!("Logged in as {}", persona_name);
            },
            _ => {},
        }
    }
    
    // connection dropped
    Err(NetworkError::EOF)
}

async fn service_method(
    msg: RawNetMessage,
    tx: &Sender<Message>,
) -> Result<(), NetworkError> {
    fn get_service_request<Request: ServiceMethodRequest>(
        msg: RawNetMessage,
    ) -> Result<Request, NetworkError> {
        let msg = msg.into_message::<ServiceMethodRequestMessage>()?;
        
        msg.into_message::<Request>()
    }
    
    let target_job_name = msg.header.target_job_name.as_ref()
        .ok_or(NetworkError::InvalidHeader)?;
    
    match target_job_name.as_ref() {
        CFriendMessages_IncomingMessage_Notification::NAME => {
            let msg = get_service_request::<CFriendMessages_IncomingMessage_Notification>(msg)?;
            let message = msg.get_message().to_string();
            
            if !message.is_empty() {
                let _ = tx.send(Message::ChatMessage {
                    message,
                    steamid: SteamID::from(msg.get_steamid_friend()),
                }).await;
            }
        },
        _target_job_name => {},
    }
    
    Ok(())
}

async fn handle_connection(
    mut connection: Connection,
    mut rx: Receiver<Message>,
) -> Result<(), NetworkError> {
    while let Some(message) = rx.recv().await {
        match message {
            Message::ChatMessage {
                message,
                steamid,
            } if message == "hello" => {
                connection.chat_message(steamid, String::from("hi :)")).await?;
            },
            Message::ChatMessage { .. } => {},
            Message::SendHeartbeat => {
                connection.send_heartbeat().await?;
            },
        }
    }
    
    Ok(())
}

// This will continue to send a heartbeat to the connection
async fn poll_heartbeat(
    interval: u64,
    tx: Sender<Message>,
) {
    loop {
        async_std::task::sleep(std::time::Duration::from_secs(interval)).await;
        
        let _ = tx.send(Message::SendHeartbeat).await;
    }
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let account_name = prompt("Account name?");
    let password = prompt("Password?");
    let two_factor_code = prompt("Two factor code?");
    let mut credentials = Connection::default_login_message(
        account_name,
        password,
    );
    
    credentials.set_two_factor_code(two_factor_code);
    
    let (
        mut connection,
        rest,
    ) = Connection::login(credentials).await?;
    // We're logged in
    let out_of_game_heartbeat_seconds = connection.send_heartbeat().await?;
    let (tx, rx) = mpsc::channel::<Message>(10);
    let heartbeat_tx = tx.clone();
    let handles = vec![
        tokio::spawn(async move {
            poll_heartbeat(out_of_game_heartbeat_seconds, heartbeat_tx).await;
        }),
        tokio::spawn(async move {
            read_messages(rest, tx).await.unwrap();
        }),
        tokio::spawn(async move {
            handle_connection(connection, rx).await.unwrap();
        }),
    ];
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    Ok(())
}
