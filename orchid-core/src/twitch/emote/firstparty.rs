use async_trait::async_trait;
use std::collections::HashMap;

/// Gets FrankerFaceZ emotes
use crate::twitch::emote::Emote;
use crate::twitch::emote::EmoteManager;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

pub struct FirstPartyEmoteManager {
    client: Client,
}

impl Default for FirstPartyEmoteManager {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmoteManager for FrankerFaceZEmoteManager {
    async fn fetch(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get("https://api.frankerfacez.com/v1/set/global")
            .send()
            .await?
            .json::<FFZGlobalEmoteResponse>()
            .await?;

        // Cache default sets
        self.cache_sets(response.sets);
        // Cache user sets
        for (user_id, user_sets) in response.users {
            self.user_sets.insert(
                user_id,
                user_sets.into_iter().map(|s| s.parse().unwrap()).collect(),
            );
        }
        // Cache global sets (as username "@global")
        for set in response.default_sets {
            self.emotes.insert("@global".to_string(), set.to_string());
        }

        Ok(())
    }

    /// Fetches the allowed emote sets for a user. Mutable because we may add new sets.
    async fn get_emote(&mut self, user_name: &str, channel_name: &str, id: &str) -> Option<Emote> {
        // check global emotes
        if let Some(channel) = self.emotes.get("@global") {
            if let Some(set) = self.set.get(channel) {
                return set.iter().find(|emote| emote.id == id).cloned();
            }
        }

        // check channel emotes
        let channel_emotes = match self.emotes.get(channel_name) {
            Some(channel) => Some(channel),
            None => {
                // fetch channel sets
                match self.fetch_channel_sets(channel_name.to_string()).await {
                    Ok(_) => self.emotes.get(channel_name),
                    Err(_) => None,
                }
            }
        };
        if let Some(channel) = channel_emotes {
            if let Some(set) = self.set.get(channel) {
                return set.iter().find(|emote| emote.id == id).cloned();
            }
        }

        // check user emotes
        let user_set = match self.user_sets.get(user_name) {
            Some(user_set) => Some(user_set),
            None => {
                // fetch user sets
                match self.fetch_user_sets(user_name.to_string()).await {
                    Ok(_) => self.user_sets.get(user_name),
                    Err(_) => None,
                }
            }
        };
        if let Some(user_set) = user_set {
            for setid in user_set {
                // fetch set
                if let Some(set) = self.set.get(setid) {
                    return set.iter().find(|emote| emote.id == id).cloned();
                }
            }
        }
        None
    }
}
