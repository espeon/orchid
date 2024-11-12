use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ChannelSubscription {
    pub channel_name: String,
    /// a client id, or 'global' if the subscription is global
    pub client_id: String,
}

pub struct SubscriptionManager {
    subscriptions: HashMap<String, HashSet<String>>, // channel -> set of client_ids
    client_channels: HashMap<String, HashSet<String>>, // client_id -> set of channels
    chat_client: Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>,
}

impl SubscriptionManager {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            subscriptions: HashMap::new(),
            client_channels: HashMap::new(),
            chat_client: None,
        }))
    }

    pub fn set_chat_client(
        &mut self,
        client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    ) {
        self.chat_client = Some(client);
    }

    pub async fn subscribe(
        &mut self,
        channel: String,
        client_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Add to channel -> clients mapping
        self.subscriptions
            .entry(channel.clone())
            .or_default()
            .insert(client_id.clone());

        // Add to client -> channels mapping
        self.client_channels
            .entry(client_id)
            .or_default()
            .insert(channel.clone());

        // If this is a new channel, join it in Twitch chat
        if let Some(chat_client) = &self.chat_client {
            if self.subscriptions[&channel].len() == 1 {
                chat_client.join(channel)?;
            }
        }

        Ok(())
    }

    pub async fn unsubscribe(&mut self, channel: &str, client_id: &str) {
        // Remove from channel -> clients mapping
        if let Some(clients) = self.subscriptions.get_mut(channel) {
            clients.remove(client_id);
            if clients.is_empty() {
                self.subscriptions.remove(channel);
                // Optionally leave the channel if no more clients are subscribed
                if let Some(chat_client) = &self.chat_client {
                    chat_client.part(channel.to_string());
                }
            }
        }

        // Remove from client -> channels mapping
        if let Some(channels) = self.client_channels.get_mut(client_id) {
            channels.remove(channel);
            if channels.is_empty() {
                self.client_channels.remove(client_id);
            }
        }
    }

    pub fn get_channel_subscribers(&self, channel: &str) -> HashSet<String> {
        self.subscriptions.get(channel).cloned().unwrap_or_default()
    }

    pub fn get_client_subscriptions(&self, client_id: &str) -> HashSet<String> {
        self.client_channels
            .get(client_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn remove_client(&mut self, client_id: &str) {
        // Get all channels this client was subscribed to
        if let Some(channels) = self.client_channels.remove(client_id) {
            // Remove client from each channel's subscription list
            for channel in channels {
                if let Some(clients) = self.subscriptions.get_mut(&channel) {
                    clients.remove(client_id);
                    if clients.is_empty() {
                        self.subscriptions.remove(&channel);
                        // Optionally leave the channel if no more clients are subscribed
                        if let Some(chat_client) = &self.chat_client {
                            chat_client.part(channel.clone());
                        }
                    }
                }
            }
        }
    }
}
