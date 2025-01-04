use std::{collections::HashSet, sync::Arc};

use message::{TwitchChatMessage, TwitchInstructionMessage};
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};
use tracing::{error, warn};
// what the heck twitch chat!!
use twitch_irc::{
    login::{LoginCredentials, StaticLoginCredentials},
    message::{RGBColor, ServerMessage},
    transport::tcp::{TCPTransport, TLS},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use crate::ws::{WebsocketCollection, WsMessage};

use manager::SubscriptionManager;

use super::emote::EmoteHandler;

pub mod manager;
pub mod message;

pub async fn setup_twitch_chat(
    state: Arc<Mutex<WebsocketCollection>>,
    sub_manager: Arc<Mutex<SubscriptionManager>>,
    emote_manager: Arc<Mutex<EmoteHandler>>,
) {
    let chat = TwitchChatClient::<StaticLoginCredentials>::new();
    let (chat, mut receiver) = chat.get_pair().await;

    // Store chat client in subscription manager
    sub_manager.lock().await.set_chat_client(chat);

    let join_handle = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    let channel = msg.channel_login.as_str();

                    // Get subscribers for this channel
                    let subscribers = sub_manager.lock().await.get_channel_subscribers(channel);

                    // format message in our own format if it is a privmsg
                    match TwitchChatMessage::try_from(msg.to_owned()) {
                        Ok(mut msg) => {
                            // postprocess message with emote parsing
                            {
                                let mut emote_manager = emote_manager.lock().await;
                                msg.message = emote_manager
                                    .process_message_with_emotes(
                                        &msg.message,
                                        msg.user.user_name.as_str(),
                                        msg.channel.as_str(),
                                    )
                                    .await;
                            }
                            let json = serde_json::to_string(&msg).unwrap();
                            send_twitchchat_msg_to_subscribers(state.clone(), subscribers, json)
                                .await;
                        }
                        Err(e) => {
                            error!("Error converting message to TwitchChatMessage: {:?}", e);
                            continue;
                        }
                    }
                }
                ServerMessage::ClearChat(msg) => {
                    // get subscribers for this channel
                    let subscribers = sub_manager
                        .lock()
                        .await
                        .get_channel_subscribers(msg.channel_login.as_str());

                    // Parse message into own format
                    let obj = match msg.action {
                        // Request to clear chat
                        twitch_irc::message::ClearChatAction::ChatCleared => {
                            &TwitchInstructionMessage {
                                msg_type: "CLEARCHAT".to_string(),
                                msg_subtype: "CLEAR_CHAT".to_string(),
                                associated_id: "".to_string(),
                            }
                        }
                        // The below two are when a user is removed from chat.
                        // I think the Twitch API just sends their messages once they are unbanned.
                        // Should be fine.
                        twitch_irc::message::ClearChatAction::UserBanned {
                            user_login: _,
                            user_id,
                        } => &TwitchInstructionMessage {
                            msg_type: "CLEARCHAT".to_string(),
                            msg_subtype: "REMOVE_USER_MESSAGES".to_string(),
                            associated_id: user_id.to_string(),
                        },
                        twitch_irc::message::ClearChatAction::UserTimedOut {
                            user_login: _,
                            user_id,
                            timeout_length: _,
                        } => &TwitchInstructionMessage {
                            msg_type: "CLEARCHAT".to_string(),
                            msg_subtype: "REMOVE_USER_MESSAGES".to_string(),
                            associated_id: user_id.to_string(),
                        },
                    };

                    let json = serde_json::to_string(obj).unwrap();
                    send_twitchchat_msg_to_subscribers(state.clone(), subscribers, json).await;
                }
                ServerMessage::ClearMsg(msg) => {
                    let obj = TwitchInstructionMessage {
                        msg_type: "CLEARMSG".to_string(),
                        msg_subtype: "SINGLE".to_string(),
                        associated_id: msg.message_id.to_string(),
                    };
                    let subscribers = sub_manager
                        .lock()
                        .await
                        .get_channel_subscribers(msg.channel_login.as_str());
                    let json = serde_json::to_string(&obj).unwrap();
                    send_twitchchat_msg_to_subscribers(state.clone(), subscribers, json).await;
                }
                ServerMessage::Notice(msg) => {
                    // Print out to console (warn)
                    warn!(
                        "Channel {:?} sent NOTICE: {}",
                        msg.channel_login, msg.message_text
                    );
                }
                _ => {}
            }
        }
    });

    join_handle.await.unwrap();
}

pub async fn send_twitchchat_msg_to_subscribers(
    state: Arc<Mutex<WebsocketCollection>>,
    subscribers: HashSet<String>,
    json: String,
) {
    if subscribers.contains("global") {
        let wsm = WsMessage::Text(json);
        let _ = state.lock().await.broadcast_message(wsm).await;
    } else {
        for client_id in subscribers {
            let wsm = WsMessage::Text(json.clone());
            let _ = state.lock().await.send_to_client(&client_id, wsm).await;
        }
    }
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

/// Converts a username to a consistent color based on hash passed in
pub fn username_to_color(hash: &str) -> (u8, u8, u8) {
    let hash: u32 = hash
        .chars()
        .fold(0, |acc, c| acc.wrapping_add(c as u32).wrapping_mul(31));

    // Use hash to generate hue (0-360)
    let hue = (hash % 360) as f64;

    // Fixed saturation and lightness for vibrant colors
    let saturation = 0.75; // 75%
    let lightness: f64 = 0.65; // 65%

    // Convert HSL to RGB
    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = lightness - c / 2.0;

    let (r, g, b) = match (hue / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

pub fn triple_to_rgbcolor(triple: (u8, u8, u8)) -> RGBColor {
    RGBColor {
        r: triple.0,
        g: triple.1,
        b: triple.2,
    }
}
