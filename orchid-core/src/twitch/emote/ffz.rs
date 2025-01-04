use async_trait::async_trait;
use std::collections::HashMap;

/// Gets FrankerFaceZ emotes
use crate::twitch::emote::Emote;
use crate::twitch::emote::EmoteManager;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

pub struct FrankerFaceZEmoteManager {
    // Emote caches
    /// Set cache
    set: HashMap<String, Vec<Emote>>,
    /// Channel-Set cache
    emotes: HashMap<String, String>,
    /// User-Set cache
    user_sets: HashMap<String, Vec<String>>,
    client: Client,
}

impl FrankerFaceZEmoteManager {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            set: HashMap::new(),
            emotes: HashMap::new(),
            user_sets: HashMap::new(),
        }
    }
    fn cache_sets(&mut self, sets: HashMap<String, EmoteSet>) {
        for set in sets {
            // Convert FFZEmote to your Emote type
            let emotes: Vec<Emote> = set
                .1
                .emoticons
                .iter()
                .map(|ffz_emote| Emote {
                    source: "FrankerFaceZ".to_string(),
                    id: ffz_emote.id.to_string(),
                    name: ffz_emote.name.clone(),
                    channel: "global".to_string(),
                    url: ffz_emote.urls.values().map(|v| v.to_owned()).collect(),
                    effect: Some(ffz_emote.modifier_flags),
                })
                .collect();

            self.set.insert(set.0, emotes);
        }
    }
    async fn fetch_user_sets(&mut self, user: String) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("https://api.frankerfacez.com/v1/user/{}", user))
            .send()
            .await?;

        // Check if response is 404
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            // Insert null set for user
            self.user_sets.insert(user, vec![]);
            return Ok(());
        }

        let data = response.json::<FFZSetsResponse>().await?;
        let keys = data.sets.keys().map(|s| s.to_owned()).collect();
        self.cache_sets(data.sets);
        self.user_sets.insert(user, keys);
        Ok(())
    }

    async fn fetch_channel_sets(
        &mut self,
        channel: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(format!("https://api.frankerfacez.com/v1/room/{}", channel))
            .send()
            .await?;

        // Check if response is 404
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            // Handle 404 case - channel not found
            return Ok(()); // or return an appropriate error
        }

        let data = response.json::<FFZSetsResponse>().await?;
        let key = data
            .sets
            .keys()
            .next()
            .cloned()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No sets found"))?;
        self.cache_sets(data.sets);
        self.emotes.insert(channel, key);
        Ok(())
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

#[derive(Debug, Serialize, Deserialize)]
struct FFZSetsResponse {
    sets: HashMap<String, EmoteSet>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FFZGlobalEmoteResponse {
    default_sets: Vec<i64>,
    sets: HashMap<String, EmoteSet>,
    users: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmoteSet {
    id: i64,
    #[serde(rename = "_type")]
    type_: i64,
    icon: Option<String>,
    title: String,
    css: Option<String>,
    emoticons: Vec<FFZEmote>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FFZEmote {
    id: i64,
    name: String,
    height: i32,
    width: i32,
    #[serde(rename = "public")]
    is_public: bool,
    hidden: bool,
    modifier: bool,
    modifier_flags: i64,
    offset: Option<serde_json::Value>,
    margins: Option<serde_json::Value>,
    css: Option<String>,
    owner: Owner,
    artist: Option<serde_json::Value>,
    urls: HashMap<String, String>,
    status: i32,
    usage_count: i32,
    created_at: String,
    last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    #[serde(rename = "_id")]
    id: i64,
    name: String,
    display_name: String,
}
