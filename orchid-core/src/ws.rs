use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    http::Response,
};
use axum_extra::TypedHeader;
use futures::{sink::SinkExt, stream::StreamExt};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info};

use uuid::Uuid;

use crate::{
    err::{OrchidError, OrchidResult},
    twitch::chat::manager::SubscriptionManager,
};

pub struct WebsocketInfo {
    pub username: String,
    pub client_id: String,
}

pub struct WebsocketCollection {
    /// Hashmap of client ids to websocket connection states
    ws: HashMap<String, Arc<Option<Mutex<WebsocketState>>>>,
    /// Map of usernames to socket infos
    users: HashMap<String, Vec<WebsocketInfo>>,
    sub_manager: Arc<Mutex<SubscriptionManager>>,
}

impl WebsocketCollection {
    pub fn new(sub_manager: Arc<Mutex<SubscriptionManager>>) -> Self {
        Self {
            ws: HashMap::new(),
            users: HashMap::new(),
            sub_manager,
        }
    }

    pub fn add_handler(
        &mut self,
        username: &str,
        client_id: &str,
        ws: Arc<Option<Mutex<WebsocketState>>>,
    ) {
        debug!(
            "Adding new handler - username: {}, client_id: {}",
            username, client_id
        );
        debug!(
            "Current number of connections before add: {}",
            self.ws.len()
        );
        self.ws.insert(client_id.to_string(), ws);
        self.users
            .entry(username.to_string())
            .or_default()
            .push(WebsocketInfo {
                username: username.to_string(),
                client_id: client_id.to_string(),
            });
        debug!("Current number of connections after add: {}", self.ws.len());
    }

    async fn send_message(
        &self,
        ws: &Option<Mutex<WebsocketState>>,
        message: WsMessage,
    ) -> OrchidResult<()> {
        if let Some(state) = ws {
            Ok(state
                .lock()
                .await
                .send_message(message.clone())
                .await
                .map_err(|e| {
                    OrchidError::ConnectionError(
                        "Failed to send message".to_string() + e.to_string().as_str(),
                    )
                })?)
        } else {
            Err(OrchidError::ConnectionError(
                "Failed to send message".to_string(),
            ))
        }
    }

    pub async fn broadcast_message(&self, message: WsMessage) -> OrchidResult<()> {
        debug!("Broadcasting message to {} clients", self.ws.len());
        for (client_id, ws) in self.ws.iter() {
            debug!("Attempting to send to client: {}", client_id);
            match self.send_message(ws.as_ref(), message.clone()).await {
                Ok(_) => debug!("Successfully sent to client: {}", client_id),
                Err(e) => error!("Failed to send to client {}: {:?}", client_id, e),
            }
        }
        Ok(())
    }

    pub async fn broadcast_message_to_user(
        &self,
        username: &String,
        message: WsMessage,
    ) -> OrchidResult<()> {
        // username is located in WebsocketInfo
        let infos = match self.users.get(username) {
            Some(infos) => infos,
            None => return Err(OrchidError::UserNotFound(username.to_string())),
        };

        for info in infos {
            if let Some(ws) = self.ws.get(&info.client_id) {
                self.send_message(ws.as_ref(), message.clone()).await?;
            } else {
                return Err(OrchidError::ConnectionError(
                    "Failed to send message: client not found".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub async fn send_to_client(&self, client_id: &str, message: WsMessage) -> OrchidResult<()> {
        if let Some(sender) = self.ws.get(client_id) {
            self.send_message(sender.as_ref(), message.clone()).await?;
            Ok(())
        } else {
            Err(OrchidError::ConnectionError(
                "Failed to send message: client not found".to_string(),
            ))
        }
    }

    pub async fn remove_handler(&mut self, username: String) {
        debug!("Removing handler for {}", username);

        // Find all client IDs associated with this username
        if let Some(infos) = self.users.get(&username) {
            for info in infos {
                // Remove from websocket connections
                self.ws.remove(&info.client_id);

                // Remove all subscriptions for this client
                self.sub_manager.lock().await.remove_client(&info.client_id);
            }
        }

        // Remove from users map
        self.users.remove(&username);
    }
}

// Message types for your websocket communication
#[derive(Debug, Clone)]
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
    // Add other message types as needed
}

pub struct WebsocketState {
    // Channel sender to send messages to the websocket handler
    tx: mpsc::Sender<WsMessage>,
}

impl WebsocketState {
    pub async fn send_message(
        &self,
        message: WsMessage,
    ) -> Result<(), mpsc::error::SendError<WsMessage>> {
        self.tx.send(message).await
    }
}

pub struct WebsocketHandler {
    pub user_agent: Option<String>,
    pub addr: SocketAddr,
    pub ws: WebSocketUpgrade,
    pub state: Arc<Mutex<Option<WebsocketState>>>,
}

impl WebsocketHandler {
    pub fn new(
        ws: WebSocketUpgrade,
        user_agent: Option<TypedHeader<headers::UserAgent>>,
        addr: SocketAddr,
    ) -> WebsocketHandler {
        let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
            user_agent.to_string()
        } else {
            String::from("Unknown browser")
        };
        info!("`{user_agent}` at {addr} connected.");

        WebsocketHandler {
            user_agent: Some(user_agent),
            ws,
            addr,
            state: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn ws_upgrade(
        self,
        username: String,
        ws_collection: Arc<Mutex<WebsocketCollection>>,
    ) -> Response<Body> {
        let addr = self.addr;

        // Create a channel for sending messages to the websocket
        let (tx, rx) = mpsc::channel::<WsMessage>(100);

        let state = Arc::new(Some(Mutex::new(WebsocketState { tx })));

        // create a random uid for the client
        let client_id = Uuid::new_v4();

        debug!(
            "Initializing websocket for {} - id: {}",
            username, client_id
        );

        {
            let mut collection = ws_collection.lock().await;
            collection.add_handler(&username, &client_id.to_string(), state.clone());
        }

        self.ws
            .on_upgrade(move |socket| handle_socket(socket, addr, rx, username, ws_collection))
    }
}

async fn handle_socket(
    socket: WebSocket,
    addr: SocketAddr,
    mut rx: mpsc::Receiver<WsMessage>,
    ws_name: String,
    ws_collection: Arc<Mutex<WebsocketCollection>>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Task for sending messages
    let send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            let ws_msg = match message {
                WsMessage::Text(text) => Message::Text(text),
                WsMessage::Binary(data) => Message::Binary(data),
            };

            if let Err(e) = sender.send(ws_msg).await {
                error!("Error sending message: {}", e);
                break;
            }
        }
    });

    // Task for receiving messages
    // Copying a few things to avoid borrowing issues
    let ws_name_cpy = ws_name.clone();
    let ws_collection_cpy = ws_collection.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    info!("Received text message from {}: {}", addr, text);
                    // Handle text message
                    // For now, if the message starts with 'echo', echo it back
                    if text.starts_with("echo") {
                        let echo = text.replace("echo", "");
                        let _ = ws_collection_cpy
                            .lock()
                            .await
                            .broadcast_message_to_user(&ws_name_cpy, WsMessage::Text(echo))
                            .await;
                    }
                }
                Message::Binary(data) => {
                    info!("Received binary message from {}: {:?}", addr, data);
                    // Handle binary message
                }
                Message::Close(_) => {
                    info!("Client {} disconnected", addr);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => debug!("Send task completed"),
        _ = receive_task => debug!("Receive task completed"),
    }

    // Remove the handler from the collection
    {
        let mut collection = ws_collection.lock().await;
        collection.remove_handler(ws_name).await;
    }

    debug!("Cleaned up connection for {}", addr);
}
