use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod ffz;
pub mod firstparty;

/// Represents an Twitch emote
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emote {
    pub source: String,
    pub id: String,
    pub name: String,
    /// The channel the emote is associated with
    pub channel: String,
    pub effect: Option<i64>,
    pub url: Vec<String>,
}

#[async_trait]
pub trait EmoteManager: Send + Sync {
    async fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn get_emote(&mut self, user_name: &str, channel_name: &str, id: &str) -> Option<Emote>;
}

pub struct EmoteHandler {
    pub managers: Vec<Box<dyn EmoteManager>>,
}

impl EmoteHandler {
    pub fn new() -> Self {
        Self { managers: vec![] }
    }
    pub fn add_manager(&mut self, manager: Box<dyn EmoteManager>) {
        self.managers.push(manager);
    }

    // Get emote from managers
    async fn get_emote(&mut self, user_name: &str, channel_name: &str, id: &str) -> Option<Emote> {
        for manager in self.managers.iter_mut() {
            if let Some(emote) = manager.get_emote(user_name, channel_name, id).await {
                return Some(emote);
            }
        }
        None
    }

    // Replace detected emote names with image tags
    // <!:id:url:effect:overlay>
    pub async fn replace_emotes(&self, message: &str, emotes: &HashMap<String, Emote>) -> String {
        let mut new_message = message.to_string();
        let replacement = |emote: &Emote| {
            let mut result = format!("<!{}", emote.id);
            if !emote.url.is_empty() {
                result.push_str(&format!(":{}", emote.url.join(",")));
            }
            if let Some(effect) = emote.effect {
                result.push_str(&format!(":{}", effect));
            }
            if !emote.name.is_empty() {
                result.push_str(&format!(":{}", emote.name));
            }
            result.push('>');
            result
        };
        for emote in emotes {
            new_message = new_message.replace(emote.0, &replacement(emote.1));
        }
        new_message
    }
    /// Process a message with emotes, using the emote managers provided.
    pub async fn process_message_with_emotes(
        &mut self,
        message: &str,
        user_name: &str,
        channel_name: &str,
    ) -> String {
        let mut found_emotes = HashMap::new();
        for manager in self.managers.iter_mut() {
            // have to process each word :(
            for word in message.split_whitespace() {
                if let Some(emote) = manager.get_emote(user_name, channel_name, word).await {
                    found_emotes.insert(word.to_string(), emote);
                }
            }
        }
        self.replace_emotes(message, &found_emotes).await
    }
}

impl Default for EmoteHandler {
    fn default() -> Self {
        Self::new()
    }
}
