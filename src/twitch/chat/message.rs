use serde::{Deserialize, Serialize};

use super::{triple_to_rgbcolor, username_to_color};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchChatMessage {
    pub msg_type: String,
    /// The channel name the message was sent in
    pub channel: String,
    /// The channel ID the message was sent in
    pub channel_id: String,
    /// The user that sent the message
    pub user: TwitchChatUser,
    /// User's current badges (name, URL)
    pub user_badges: Vec<(String, String)>,
    pub nickname_color: (u8, u8, u8),
    /// The message, with Discord-esque emote formatting
    pub message: String,
    pub message_id: String,
    pub server_timestamp: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchChatUser {
    pub user_id: String,
    /// The user's "Login" name
    pub user_name: String,
    /// The user's display name
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitchInstructionMessage {
    pub msg_type: String,
    pub msg_subtype: String,
    pub associated_id: String,
}

// twitch_irc ServerMessage to TwitchChatMessage
impl TryFrom<twitch_irc::message::PrivmsgMessage> for TwitchChatMessage {
    type Error = twitch_irc::message::ServerMessage;

    fn try_from(msg: twitch_irc::message::PrivmsgMessage) -> Result<Self, Self::Error> {
        let msg_type = "PRIVMSG".to_string();
        let channel = msg.channel_login;
        let channel_id = msg.channel_id;
        let user = TwitchChatUser {
            user_id: msg.sender.id,
            user_name: msg.sender.login,
            display_name: msg.sender.name,
        };
        let user_badges = msg
            .badges
            .into_iter()
            .map(|badge| (badge.name.to_string(), badge.version.to_string()))
            .collect();
        let color = msg
            .name_color
            .unwrap_or(triple_to_rgbcolor(username_to_color(&user.user_name)));
        let nickname_color: (u8, u8, u8) = (color.r, color.g, color.b);
        // TODO: Parse emotes
        let message = msg.message_text.to_string();
        let message_id = msg.message_id.to_string();
        let server_timestamp = msg.server_timestamp.to_string();

        Ok(TwitchChatMessage {
            msg_type,
            channel,
            channel_id,
            user,
            user_badges,
            nickname_color,
            message,
            message_id,
            server_timestamp,
        })
    }
}
