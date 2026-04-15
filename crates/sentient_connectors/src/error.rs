//! Connector error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("OAuth error: {0}")]
    OAuthError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Sync error: {0}")]
    SyncError(String),

    #[error("Connector not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type ConnectorResult<T> = Result<T, ConnectorError>;

impl From<reqwest::Error> for ConnectorError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            ConnectorError::Network(format!("Timeout: {}", e))
        } else if e.is_connect() {
            ConnectorError::ConnectionFailed(e.to_string())
        } else {
            ConnectorError::ApiError(e.to_string())
        }
    }
}

impl From<serde_json::Error> for ConnectorError {
    fn from(e: serde_json::Error) -> Self {
        ConnectorError::ParseError(e.to_string())
    }
}
