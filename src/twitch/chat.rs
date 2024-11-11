use std::sync::Arc;

use tokio::sync::{mpsc::UnboundedReceiver, Mutex};
// what the heck twitch chat!!
use twitch_irc::{
    login::{LoginCredentials, StaticLoginCredentials},
    message::ServerMessage,
    transport::tcp::{TCPTransport, TLS},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use crate::ws::{WebsocketCollection, WsMessage};

use super::chat_manager::SubscriptionManager;

pub async fn setup_twitch_chat(
    state: Arc<Mutex<WebsocketCollection>>,
    sub_manager: Arc<Mutex<SubscriptionManager>>,
) {
    let chat = TwitchChatClient::<StaticLoginCredentials>::new();
    let (chat, mut receiver) = chat.get_pair().await;

    // Store chat client in subscription manager
    sub_manager.lock().await.set_chat_client(chat);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if let ServerMessage::Privmsg(msg) = &message {
                let channel = msg.channel_login.as_str();

                // Get subscribers for this channel
                let subscribers = sub_manager.lock().await.get_channel_subscribers(channel);

                // if 'global' is in the subscribers, send to all clients
                if subscribers.contains("global") {
                    let json = serde_json::to_string(&msg).unwrap();
                    let wsm = WsMessage::Text(json);
                    let _ = state.lock().await.broadcast_message(wsm).await;
                } else {
                    for client_id in subscribers {
                        let json = serde_json::to_string(&msg).unwrap();
                        let wsm = WsMessage::Text(json);
                        let _ = state.lock().await.send_to_client(&client_id, wsm).await;
                    }
                }
            }
        }
    });

    join_handle.await.unwrap();
}

pub struct TwitchChatClient<C: LoginCredentials> {
    client: TwitchIRCClient<SecureTCPTransport, C>,
    receiver: UnboundedReceiver<ServerMessage>,
}

impl<C: LoginCredentials> TwitchChatClient<C> {
    pub fn new_with_config(config: ClientConfig<C>) -> Arc<Self> {
        let (incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, C>::new(config);
        Arc::new(Self {
            client,
            receiver: incoming_messages,
        })
    }

    /// Consumes the client, gets a pair of the client and the receiver.
    pub async fn get_pair(
        self,
    ) -> (
        TwitchIRCClient<TCPTransport<TLS>, C>,
        UnboundedReceiver<ServerMessage>,
    ) {
        (self.client, self.receiver)
    }
}

impl TwitchChatClient<StaticLoginCredentials> {
    pub fn new() -> Self {
        let config = ClientConfig::default();
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        Self {
            client,
            receiver: incoming_messages,
        }
    }
}

impl Default for TwitchChatClient<StaticLoginCredentials> {
    fn default() -> Self {
        Self::new()
    }
}
