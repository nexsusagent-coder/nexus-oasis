//! Web server error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Web server error type
#[derive(Debug, Error)]
pub enum WebError {
    /// Server error
    #[error("Server error: {0}")]
    Server(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Invalid token
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    /// Token expired
    #[error("Token expired")]
    TokenExpired,

    /// Unauthorized
    #[error("Unauthorized")]
    Unauthorized,

    /// Forbidden
    #[error("Forbidden")]
    Forbidden,

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Bad request
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// JWT error
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            WebError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            WebError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            WebError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            WebError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            WebError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            WebError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            WebError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            WebError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            WebError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".to_string()),
            WebError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

/// Result type for web operations
pub type Result<T> = std::result::Result<T, WebError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = WebError::BadRequest("Invalid input".to_string());
        assert!(err.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_not_found() {
        let err = WebError::NotFound("Resource".to_string());
        assert!(err.to_string().contains("Not found"));
    }

    #[test]
    fn test_into_response() {
        let err = WebError::BadRequest("test".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
