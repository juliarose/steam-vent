/// Demonstrates logging in and replying to the chat message "hello".
/// 
/// This example could be much shorter but is comprehensive to demonstrate most of everything else 
/// that is needed for building a basic bot. I may change the module to do most of the work for 
/// you as much of it is fairly redundant. For now this is how the module is consumed.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use steam_vent::enums::EPersonaState;
use steam_vent::enums::eresult::EResult;
use steam_vent_proto::steammessages_clientserver_login::CMsgClientRequestWebAPIAuthenticateUserNonceResponse;
use steamid_ng::SteamID;
use steam_vent::proto::steammessages_clientserver_2::{CMsgClientUpdateMachineAuthResponse, CMsgClientUpdateMachineAuth};
use steam_vent::proto::enums_clientserver::EMsg;
use steam_vent::proto::steammessages_friendmessages_steamclient::CFriendMessages_IncomingMessage_Notification;
use steam_vent::proto::steammessages_clientserver_login::{
    CMsgClientLogon,
    CMsgClientAccountInfo,
    CMsgClientNewLoginKey,
    CMsgClientNewLoginKeyAccepted,
};
use steam_vent::connection::Connection;
use steam_vent::message::ServiceMethodRequestMessage;
use steam_vent::net::{NetworkError, RawNetMessage};
use steam_vent::service_method::ServiceMethodRequest;
use tokio::sync::mpsc::Sender;

/// Helper methods.
mod helpers {
    use another_steam_totp::generate_auth_code;
    use sha1::{Sha1, Digest};
    use std::path::PathBuf;
    use async_std::io::WriteExt;
    use async_std::fs::File;
    
    /// Prompts user for input.
    pub fn prompt(message: &str) -> String {
        println!("{}", message);
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        
        input.trim().to_string()
    }
    
    /// Creates a sha1 hash.
    pub fn create_sha1(input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha1::new();
        
        hasher.update(input);
        hasher.finalize().to_vec()
    }
    
    /// Gets two factor code either using the shared secret from an environment variable or from a 
    /// prompt.
    pub fn get_two_factor_code() -> String {
        if let Ok(shared_secret) = std::env::var("SHARED_SECRET") {    
            // Generates the 5-character time-based one-time password using your shared_secret.
            if let Ok(two_factor_code) = generate_auth_code(shared_secret, None) {
                return two_factor_code;
            }
        }
        
        prompt("Two factor code?")
    }
    
    /// Writes a file atomically.
    pub async fn atomic_write<T: Into<PathBuf>>(
        filepath: T,
        bytes: &[u8],
    ) -> std::io::Result<()> {
        let filepath = filepath.into();
        let mut temp_filepath = filepath.clone();
        
        temp_filepath.set_extension("tmp");
        
        let mut temp_file = File::create(&temp_filepath).await?;
        
        if let Err(error) = temp_file.write_all(bytes).await {
            // something went wrong writing to this file...
            async_std::fs::remove_file(&temp_filepath).await?;
            return Err(error);
        }
        
        temp_file.flush().await?;
        async_std::fs::rename(&temp_filepath,&filepath).await?;
        Ok(())
    }
}

/// Spawned tasks.
mod tasks {
    use super::{Bot, Message, MessageHandler, on_connection};
    use std::sync::Arc;
    use steam_vent::connection::Connection;
    use steam_vent::net::{RawNetMessage, NetworkError};
    use tokio::sync::mpsc::{self, Receiver, Sender};
    use tokio::task::JoinHandle;
    
    /// Routes messages from the receiver for the Steam client connection. This exists independently 
    /// from the sender, if a connection is lost the receiver will no longer receive messages.
    pub fn route_steam_messages(
        mut rx: Receiver<Result<RawNetMessage, NetworkError>>,
        tx: Sender<Message>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(msg_result) = rx.recv().await {
                match msg_result {
                    Ok(msg) => {
                        let _ = tx.send(Message::NetMessage(msg)).await;
                    },
                    Err(error) => {
                        // Unlikely to occur, something is possibly wrong with the configuration
                        panic!("Error reading message: {}", error);
                    },
                }
            }
            
            // No more messages means connection was most likely closed - send a notice
            let _ = tx.send(Message::Disconnected).await;
        })
    }
    
    /// Handles the connection.
    pub async fn handle(
        bot: Arc<Bot>,
        mut connection: Connection,
        connection_rx: Receiver<Result<RawNetMessage, NetworkError>>, 
    ) -> Result<Vec<JoinHandle<()>>, NetworkError> {
        /// Sends a heartbeat to the connection at an interval determined by the connection.
        async fn poll_heartbeat(
            connection: &mut Connection,
            tx: Sender<Message>,
        ) -> Result<JoinHandle<()>, NetworkError> {
            let interval = connection.send_heartbeat().await?;
            
            Ok(tokio::spawn(async move {
                loop {
                    async_std::task::sleep(std::time::Duration::from_secs(interval)).await;
                    let _ = tx.send(Message::SendHeartbeat).await;
                }
            }))
        }
        
        /// Listens to updates and handles the connection.
        fn handle_connection(
            bot: Arc<Bot>,
            connection: Connection,
            tx: Sender<Message>,
            mut rx: Receiver<Message>,
        ) -> JoinHandle<()> {
            tokio::spawn(async move {
                let mut handler = MessageHandler {
                    tx,
                    bot,
                    connection,
                    web_session_callbacks: Vec::new(),
                };
                
                while let Some(message) = rx.recv().await {
                    if let Err(error) = handler.handle_message(message).await {
                        log::warn!("Error handling message: {error}");
                    }
                }
            })
        }
        
        let (tx, rx) = mpsc::channel::<Message>(50);
        // Receiver for when the web session is ready
        let web_session_rx = on_connection(&mut connection, &tx).await?;
        
        Ok(vec![
            // send a heartbeat to keep the connection alive
            poll_heartbeat(&mut connection, tx.clone()).await?,
            // route messages from Steam connection
            route_steam_messages(connection_rx, tx.clone()),
            tokio::spawn(async move {
                if web_session_rx.await.is_ok() {
                    log::info!("Web session ready");
                }
            }),
            // read messages from connection
            // all messages sent through tx will be to this task
            handle_connection(
                bot,
                connection,
                tx,
                rx,
            ),
        ])
    }
}

/// Stores data associated with bot.
pub struct Bot {
    steamid: SteamID,
    account_name: String,
    data_directory: Option<PathBuf>,
    state: Arc<RwLock<State>>,
}

impl Bot {
    /// Reads a file associated with account
    fn read_account_file(
        filename: &str,
        account_name: &str,
        data_directory: &PathBuf,
    ) -> std::io::Result<Vec<u8>> {
        let filepath = Self::get_account_filepath(filename, account_name, data_directory);
        let mut bytes: Vec<u8> = Vec::new();
        let mut file = File::open(filepath)?;
        
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    
    /// Gets the filepath for a file.
    fn get_account_filepath(
        filename: &str,
        account_name: &str,
        data_directory: &PathBuf,
    ) -> PathBuf {
        data_directory.join(format!("{account_name}/{filename}"))
    }
    
    /// Gets login credentials
    fn get_credentials(
        data_directory: &Option<PathBuf>,
    ) -> CMsgClientLogon {
        // Attempt to take account name and password from env
        let account_name = std::env::var("ACCOUNT_NAME")
            .unwrap_or_else(|_e| helpers::prompt("Account name?"));
        let password = std::env::var("PASSWORD")
            .unwrap_or_else(|_e| helpers::prompt("Password?"));
        let mut credentials = Connection::default_login_message(
            account_name.clone(),
            password,
        );
        
        if let Some(data_directory) = data_directory {
            let machine_id_filepath = data_directory.join("machineid");
            
            if let Err(error) = Connection::set_machine_id_from_file(&mut credentials, &machine_id_filepath) {
                log::info!("Error setting machineid from {machine_id_filepath:?}: {error}");
            }
            
            match Bot::read_account_file("sentry", &account_name, &data_directory) {
                Ok(bytes) => {
                    let sentry = helpers::create_sha1(&bytes);
                    
                    credentials.set_sha_sentryfile(sentry);
                    credentials.set_eresult_sentryfile(1);
                },
                // Don't display a message if a file does not exist
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
                Err(error) => {
                    log::warn!("Error reading sentry file: {error}");
                },
            }
            
            match Bot::read_account_file("login_key", &account_name, &data_directory) {
                Ok(bytes) => {
                    let login_key = String::from_utf8(bytes).unwrap();
                    
                    credentials.set_login_key(login_key);
                    return credentials;
                },
                // Don't display a message if a file does not exist
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {},
                Err(error) => {
                    log::warn!("Error reading login key file: {error}");
                },
            }
        }
        
        credentials.set_two_factor_code(helpers::get_two_factor_code());
        credentials
    }
    
    /// Saves a file associated with account
    async fn save_file(
        &self,
        filename: &str,
        bytes: &[u8],
    ) -> std::io::Result<Option<PathBuf>> {
        if let Some(data_directory) = &self.data_directory {
            let account_directory = data_directory.join(&self.account_name);
            let filepath = Self::get_account_filepath(filename, &self.account_name, data_directory);
            // Create the directory
            std::fs::create_dir_all(&account_directory)?;
            helpers::atomic_write(&filepath, bytes).await?;
            return Ok(Some(filepath));
        }
        
        Ok(None)
    }
}

/// State associated with [`Bot`].
struct State {
    personaname: Option<String>,
    messages_received: HashMap<SteamID, i32>,
    credentials: CMsgClientLogon,
}

type WebSession = (String, Vec<String>);
type WebSessionCallback = tokio::sync::oneshot::Sender<WebSession>;
type WebSessionReceiver = tokio::sync::oneshot::Receiver<WebSession>;

/// A message.
#[derive(Debug)]
pub enum Message {
    /// A chat message was received.
    ChatMessage {
        steamid: SteamID,
        message: String,
    },
    /// A request to send a heartbeat was received.
    SendHeartbeat,
    /// A persona name was received.
    PersonaName(String),
    /// A disconnect was received.
    Disconnected,
    /// A [`RawNetMessage`] was received from the [`tokio::sync::mpsc::Receiver`] for [`Connection`].
    NetMessage(RawNetMessage),
    /// A request to authenticate web session was received.
    WebAuthenticateWithCallback(WebSessionCallback),
    /// A web session was received.
    WebSession {
        sessionid: String,
        cookies: Vec<String>,
    },
}

/// Handles messages associated with [`Connection`].
pub struct MessageHandler {
    tx: Sender<Message>,
    bot: Arc<Bot>,
    connection: Connection,
    web_session_callbacks: Vec<WebSessionCallback>,
}

impl MessageHandler {
    pub async fn handle_service_message(
        &self,
        msg: RawNetMessage,
    ) -> Result<(), NetworkError> {
        fn get_service_request<Request: ServiceMethodRequest>(
            msg: RawNetMessage,
        ) -> Result<Request, NetworkError> {
            msg.into_message::<ServiceMethodRequestMessage>()?.into_message::<Request>()
        }
        
        let target_job_name = msg.header.target_job_name.as_ref()
            .ok_or(NetworkError::InvalidHeader)?;
        
        match target_job_name.as_ref() {
            CFriendMessages_IncomingMessage_Notification::NAME => {
                let msg: CFriendMessages_IncomingMessage_Notification = get_service_request(msg)?;
                let message = msg.get_message().to_owned();
                
                if !message.is_empty() {
                    let steamid = SteamID::from(msg.get_steamid_friend());
                    let _ = self.tx.send(Message::ChatMessage {
                        steamid,
                        message,
                    }).await;
                }
            },
            _target_job_name => {},
        }
        
        Ok(())
    }
    
    pub async fn handle_net_message(
        &mut self,
        msg: RawNetMessage,
    ) -> Result<(), NetworkError> {
        match msg.kind {
            EMsg::k_EMsgServiceMethod => self.handle_service_message(msg).await?,
            EMsg::k_EMsgClientAccountInfo => {
                // Account info
                let message = msg.into_message::<CMsgClientAccountInfo>()?;
                let persona_name = message.get_persona_name();
                
                // Send a notice that we received the personaname
                let _ = self.tx.send(Message::PersonaName(persona_name.to_owned())).await;
            },
            EMsg::k_EMsgClientUpdateMachineAuth => {
                // We received a sentry
                let source_job_id = msg.header.source_job_id;
                let message = msg.into_message::<CMsgClientUpdateMachineAuth>()?;
                let bytes = message.get_bytes();
                
                match self.bot.save_file("sentry", bytes).await {
                    Ok(Some(filepath)) => {
                        log::info!("Sentry saved to {filepath:?}");
                        
                        let mut req = CMsgClientUpdateMachineAuthResponse::new();
                        let hash = helpers::create_sha1(bytes);
                        
                        // Write the sentry to our login credentials for reconnects
                        {
                            let mut state = self.bot.state.write().unwrap();
                            
                            state.credentials.set_sha_sentryfile(hash.clone());
                            state.credentials.set_eresult_sentryfile(1);
                        }
                        
                        req.set_sha_file(hash);
                        // Send a response that it was received
                        self.connection.send_response(req, source_job_id).await?;
                    },
                    // No data directory set
                    Ok(None) => {},
                    Err(error) => {
                        log::warn!("Error saving sentry: {error}");
                    },
                }
            },
            EMsg::k_EMsgClientNewLoginKey => {
                // We received a login key
                let message = msg.into_message::<CMsgClientNewLoginKey>()?;
                let login_key = message.get_login_key();
                
                match self.bot.save_file("login_key", login_key.as_bytes()).await {
                    Ok(Some(filepath)) => {
                        log::info!("Login key saved to {filepath:?}");
                        
                        // Write the login key to our login credentials for reconnects
                        self.bot.state.write().unwrap().credentials.set_login_key(login_key.to_owned());
                        
                        let unique_id = message.get_unique_id();
                        let mut req = CMsgClientNewLoginKeyAccepted::new();
                        
                        req.set_unique_id(unique_id);
                        // Send a response that it was received
                        self.connection.send(req).await?;
                    },
                    // No data directory set
                    Ok(None) => {},
                    Err(error) => {
                        log::warn!("Error saving login key: {error}");
                    },
                }
            },
            EMsg::k_EMsgClientRequestWebAPIAuthenticateUserNonceResponse => {
                // Got a web api nonce
                let message = msg.into_message::<CMsgClientRequestWebAPIAuthenticateUserNonceResponse>()?;
                let eresult: EResult = message.get_eresult().into();
                let nonce = message.get_webapi_authenticate_user_nonce();
                
                if eresult == EResult::OK {
                    let mut retries = 0;
                    
                    loop {
                        match self.connection.web_api_authenticate(nonce).await {
                            Ok((sessionid, cookies)) => {
                                // Send a notice that we received a web session
                                let _ = self.tx.send(Message::WebSession {
                                    sessionid,
                                    cookies,
                                }).await;
                                break;
                            },
                            Err(error) if retries >= 3 => {
                                log::error!("Error authenticating {}: {error:?}", u64::from(self.connection.session.steam_id));
                                panic!("Error authenticating {}", error);
                            },
                            Err(error) => {
                                log::warn!("Error authenticating {}: {error:?}", u64::from(self.connection.session.steam_id));
                                async_std::task::sleep(std::time::Duration::from_secs(10)).await;
                                retries += 1;
                                continue;
                            },
                        }
                    }
                } else {
                    // Something went wrong with authentication - something is likely incorrectly configured 
                    panic!("Error authenticating: {:?}", eresult);
                }
            },
            _ => {},
        }
        
        Ok(())
    }
    
    pub async fn respond_to_chat_message(
        &self,
        steamid: SteamID,
        message: String,
    ) -> Option<String> {
        // increment the message count for this user
        let count = {
            let mut state_guard = self.bot.state.write().unwrap();
            let count = state_guard.messages_received
                .entry(steamid.clone())
                .and_modify(|count| *count += 1)
                .or_insert_with(|| 1);
            
            *count 
        };
                
        match message.as_str() {
            // says "hello" back
            "hello" => {
                Some(format!("Hello!"))
            },
            // displays the current count
            "count" if count == 1 => {
                Some(format!("This is the first message you've sent me!"))
            },
            "count" => {
                Some(format!("You've sent me {count} messages!"))
            },
            _ => None,
        }
    }
    
    pub async fn handle_message(
        &mut self,
        message: Message
    ) -> Result<(), NetworkError> {
        match message {
            Message::ChatMessage {
                steamid,
                message,
            } => if let Some(message) = self.respond_to_chat_message(steamid, message).await {
                self.connection.chat_message(steamid, message).await?;
            },
            Message::PersonaName(persona_name) => {
                let mut state = self.bot.state.write().unwrap();
                
                if state.personaname.is_none() {
                    log::info!("Logged in as {persona_name}");
                }
                
                state.personaname = Some(persona_name);
            },
            Message::SendHeartbeat => {
                self.connection.send_heartbeat().await?;
            },
            Message::Disconnected => {
                let credentials = self.bot.state.read().unwrap().credentials.clone();
                
                reconnect(
                    credentials,
                    &mut self.connection,
                    self.tx.clone(),
                ).await;
            },
            Message::NetMessage(msg) => if let Err(error) = self.handle_net_message(msg).await {
                log::warn!("Error handling net message: {error}");
            },
            Message::WebSession {
                sessionid,
                cookies, 
            } => {
                log::info!("Got cookies");
                // Empty the callbacks
                for sender in std::mem::take(&mut self.web_session_callbacks) {
                    // Call send for each sender
                    let _ = sender.send((sessionid.clone(), cookies.clone()));
                }
            },
            Message::WebAuthenticateWithCallback(sender) => {
                self.web_session_callbacks.push(sender);
                
                let _ = self.connection.request_web_api_authenticate_user_nonce().await;
            },
        }
        
        Ok(())
    }
}

async fn reconnect(
    mut credentials: CMsgClientLogon,
    connection: &mut Connection,
    tx: Sender<Message>,
) {
    log::warn!("Connection lost; Attempting to reconnect...");
    let mut retries = 3;
    
    loop {
        credentials.set_two_factor_code(helpers::get_two_factor_code());
        
        // Attempt to reconnect
        match connection.reconnect(
            credentials.clone(),
        ).await {
            // We got a new receiver
            Ok(rx) => {
                log::info!("We're back online!");
                // Read messages from it
                let _handle = tasks::route_steam_messages(rx, tx.clone());
                let _ = on_connection(connection, &tx).await;
                break;
            },
            Err(error) if retries <= 0 => {
                panic!("Error reconnecting: {}", error);
            },
            Err(error) => {
                log::warn!("Error reconnecting: {error}");
                log::info!("Trying again...");
                retries -= 1;
                async_std::task::sleep(std::time::Duration::from_secs(10)).await;
            },
        }
    }
}

/// Calls initialization methods for connection.
async fn on_connection(
    connection: &mut Connection,
    tx: &Sender<Message>,
) -> Result<WebSessionReceiver, NetworkError> {
    // Set the bot to online
    connection.set_persona_state(EPersonaState::Online).await?;
    // Get a web session
    let (
        web_session_tx,
        web_session_rx,
    ) = tokio::sync::oneshot::channel::<WebSession>();
    let _ = tx.send(Message::WebAuthenticateWithCallback(web_session_tx)).await;
    
    Ok(web_session_rx)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .format_timestamp(None)
        .filter(None, log::LevelFilter::Info)
        .init();
    
    let data_directory: Option<PathBuf> = std::env::var("DATA_DIRECTORY").ok().map(|s| s.into());
    let credentials = Bot::get_credentials(&data_directory);
    let (
        connection,
        connection_rx,
    ) = Connection::login(credentials.clone()).await?;
    // Data associated with bot - can be shared across tasks
    let bot = Arc::new(Bot {
        steamid: connection.session.steam_id,
        account_name: credentials.get_account_name().to_owned(),
        data_directory,
        // Shared mutable state
        state: Arc::new(RwLock::new(State {
            personaname: None,
            messages_received: HashMap::new(),
            credentials,
        })),
    });
    
    // We're logged in
    log::info!("Logged in as {}", u64::from(connection.session.steam_id));
    for handle in tasks::handle(bot, connection, connection_rx).await? {
        handle.await?;
    }
    Ok(())
}
