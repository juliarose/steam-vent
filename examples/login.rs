/// Demonstrates logging in and replying to the chat message "hello"
/// along with the number of messages received from that user while running

use std::{collections::HashMap, sync::{Arc, RwLock}};
use steamid_ng::SteamID;
use steam_vent::proto::{
    enums_clientserver::EMsg,
    steammessages_friendmessages_steamclient::CFriendMessages_IncomingMessage_Notification,
    steammessages_clientserver_login::{
        CMsgClientLogon,
        CMsgClientAccountInfo,
    },
};
use steam_vent::{
    connection::Connection,
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
    PersonaName(String),
    Disconnected,
}

fn read_messages(
    mut rest: Receiver<Result<RawNetMessage, NetworkError>>,
    tx: Sender<Message>,
) -> tokio::task::JoinHandle<()> {
    async fn read_message(
        msg_result: Result<RawNetMessage, NetworkError>,
        tx: &Sender<Message>,
    ) -> Result<(), NetworkError> {
        let msg = msg_result?;
            
        match msg.kind {
            EMsg::k_EMsgServiceMethod => service_method(msg, &tx).await?,
            EMsg::k_EMsgClientAccountInfo => {
                let message = msg.into_message::<CMsgClientAccountInfo>()?;
                let persona_name = message.get_persona_name();
                let _ = tx.send(Message::PersonaName(persona_name.to_owned())).await;
            },
            _ => {},
        }
        
        Ok(())
    }
    
    tokio::spawn(async move {
        while let Some(msg_result) = rest.recv().await {
            if let Err(error) = read_message(
                msg_result,
                &tx
            ).await {
                // Probably shouldn't have any errors occur here
                panic!("Error reading message: {}", error);
            }
        }
        
        // no more messages means connection was most likely disconnected
        let _ = tx.send(Message::Disconnected).await;
    })
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
                let steamid = SteamID::from(msg.get_steamid_friend());
                let _ = tx.send(Message::ChatMessage {
                    message,
                    steamid,
                }).await;
            }
        },
        _target_job_name => {},
    }
    
    Ok(())
}

async fn handle_connection(
    state: Arc<RwLock<State>>,
    mut connection: Connection,
    tx: Sender<Message>,
    mut rx: Receiver<Message>,
) -> Result<(), NetworkError> {
    while let Some(message) = rx.recv().await {
        match message {
            Message::ChatMessage {
                message,
                steamid,
            }  => {
                let count = {
                    let mut state_guard = state.write().unwrap();
                    let count = state_guard.messages_received
                        .entry(steamid.clone())
                        .and_modify(|count| *count += 1)
                        .or_insert_with(|| 1);
                    
                    *count 
                };
                
                if message == "hello" {
                    let message = format!("hi #{}", count);
                    
                    connection.chat_message(steamid, message).await?;
                }
            },
            Message::PersonaName(persona_name) => {
                println!("Logged in as {}", persona_name);
                state.write().unwrap().personaname = persona_name;
            },
            Message::SendHeartbeat => {
                connection.send_heartbeat().await?;
            },
            Message::Disconnected => {
                println!("Connection lost; Attempting to reconnect...");
                
                let mut credentials = state.read().unwrap().credentials.clone();
                let mut retries = 3;
                
                loop {
                    let two_factor_code = prompt("Two factor code?");
    
                    credentials.set_two_factor_code(two_factor_code);
                    
                    match connection.reconnect(
                        credentials.clone(),
                    ).await {
                        Ok(rest) => {
                            let _handle = read_messages(rest, tx.clone());
                            break;
                        },
                        Err(error) if retries <= 0 => {
                            panic!("Error reconnecting: {}", error);
                        },
                        Err(error) => {
                            println!("Error reconnecting: {}", error);
                            println!("Trying again...");
                            retries -= 1;
                            async_std::task::sleep(std::time::Duration::from_secs(10)).await;
                        },
                    }
                }
            }
        }
    }
    
    Ok(())
}

// This will continue to send a heartbeat to the connection
fn poll_heartbeat(
    interval: u64,
    tx: Sender<Message>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_secs(interval)).await;
            
            let _ = tx.send(Message::SendHeartbeat).await;
        }
    })
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    
    input.trim().to_string()
}

struct State {
    credentials: CMsgClientLogon,
    personaname: String,
    messages_received: HashMap<SteamID, i32>,
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
    
    let state = State {
        credentials,
        personaname: String::new(),
        messages_received: HashMap::new(),
    };
    let (
        mut connection,
        rest,
    ) = Connection::login(state.credentials.clone()).await?;
    // We're logged in
    let out_of_game_heartbeat_seconds = connection.send_heartbeat().await?;
    let (tx, rx) = mpsc::channel::<Message>(10);
    let heartbeat_tx = tx.clone();
    // state that can be shared across tasks
    let state = Arc::new(RwLock::new(state));
    let handles = vec![
        poll_heartbeat(out_of_game_heartbeat_seconds, heartbeat_tx),
        // read messages from senders
        read_messages(rest, tx.clone()),
        // read messages from connection
        // all messages sent through tx will be to this task
        tokio::spawn(async move {
            handle_connection(
                state,
                connection,
                tx,
                rx,
            ).await.unwrap();
        }),
    ];
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    Ok(())
}
