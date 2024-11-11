use axum::{http::StatusCode, Json};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrchidError {
    #[error("Twitch IRC Validation error: {0}")]
    TwitchValidationError(#[from] twitch_irc::validate::Error),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to connect to Twitch chat: {0}")]
    ConnectionError(String),

    #[error("Channel operation failed: {0}")]
    ChannelError(String),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type OrchidResult<T> = Result<T, OrchidError>;

impl From<OrchidError> for (StatusCode, Json<serde_json::Value>) {
    fn from(err: OrchidError) -> Self {
        let (status, error_message) = match err {
            OrchidError::TwitchValidationError(_) => {
                (StatusCode::BAD_GATEWAY, "Failed to validate with Twitch")
            }
            OrchidError::ConnectionError(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Failed to connect to service",
            ),
            OrchidError::ChannelError(_) => (StatusCode::BAD_REQUEST, "Invalid channel operation"),
            OrchidError::AuthError(_) => (StatusCode::UNAUTHORIZED, "Authentication failed"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (
            status,
            Json(serde_json::json!({
                "error": error_message,
                "details": err.to_string(),
            })),
        )
    }
}
