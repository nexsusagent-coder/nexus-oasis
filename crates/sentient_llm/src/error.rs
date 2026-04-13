//! ─── LLM Error Types ───

use std::fmt;
use thiserror::Error;

/// LLM operation errors
#[derive(Error, Debug)]
pub enum LlmError {
    /// API key missing or invalid
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// API request failed
    #[error("API error: {0}")]
    ApiError(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {0:?}")]
    RateLimitExceeded(std::time::Duration),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Context length exceeded
    #[error("Context length exceeded: max {max}, provided {provided}")]
    ContextLengthExceeded { max: usize, provided: usize },

    /// Content filtered
    #[error("Content filtered: {0}")]
    ContentFiltered(String),

    /// Timeout
    #[error("Request timed out after {0:?}")]
    Timeout(std::time::Duration),

    /// Stream error
    #[error("Stream error: {0}")]
    StreamError(String),

    /// Provider unavailable
    #[error("Provider {0} is unavailable: {1}")]
    ProviderUnavailable(String, String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Token counting error
    #[error("Token counting error: {0}")]
    TokenCounting(String),

    /// Response parsing error
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Quota exceeded
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    /// Server error
    #[error("Server error ({0}): {1}")]
    ServerError(u16, String),

    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),

    /// No healthy nodes available
    #[error("No healthy nodes available")]
    NoHealthyNodes,

    /// Request failed
    #[error("Request failed: {0}")]
    RequestFailed(String),
}

impl LlmError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            LlmError::RateLimitExceeded(_)
                | LlmError::Timeout(_)
                | LlmError::ProviderUnavailable(_, _)
                | LlmError::Network(_)
                | LlmError::ServerError(502, _)
                | LlmError::ServerError(503, _)
                | LlmError::ServerError(504, _)
        )
    }

    /// Check if error is authentication related
    pub fn is_auth_error(&self) -> bool {
        matches!(self, LlmError::Authentication(_))
    }

    /// Check if error is rate limit
    pub fn is_rate_limit(&self) -> bool {
        matches!(self, LlmError::RateLimitExceeded(_))
    }

    /// Check if error is context length
    pub fn is_context_error(&self) -> bool {
        matches!(self, LlmError::ContextLengthExceeded { .. })
    }

    /// Get retry delay if applicable
    pub fn retry_delay(&self) -> Option<std::time::Duration> {
        match self {
            LlmError::RateLimitExceeded(delay) => Some(*delay),
            LlmError::Timeout(delay) => Some(*delay),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for LlmError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            LlmError::Timeout(std::time::Duration::from_secs(30))
        } else if err.is_connect() {
            LlmError::Network(err.to_string())
        } else if err.is_status() {
            match err.status() {
                Some(status) => {
                    match status.as_u16() {
                        401 => LlmError::Authentication("Invalid API key".into()),
                        403 => LlmError::Authentication("Access forbidden".into()),
                        404 => LlmError::ModelNotFound("Model not found".into()),
                        429 => LlmError::RateLimitExceeded(std::time::Duration::from_secs(60)),
                        500..=599 => LlmError::ServerError(status.as_u16(), err.to_string()),
                        _ => LlmError::ApiError(err.to_string()),
                    }
                }
                None => LlmError::ApiError(err.to_string()),
            }
        } else {
            LlmError::Network(err.to_string())
        }
    }
}

impl From<serde_json::Error> for LlmError {
    fn from(err: serde_json::Error) -> Self {
        LlmError::ParseError(err.to_string())
    }
}

impl From<std::io::Error> for LlmError {
    fn from(err: std::io::Error) -> Self {
        LlmError::Network(err.to_string())
    }
}

/// Result type for LLM operations
pub type LlmResult<T> = Result<T, LlmError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_retryable() {
        let err = LlmError::RateLimitExceeded(std::time::Duration::from_secs(60));
        assert!(err.is_retryable());

        let err = LlmError::Authentication("test".into());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_error_classification() {
        let err = LlmError::Authentication("test".into());
        assert!(err.is_auth_error());

        let err = LlmError::RateLimitExceeded(std::time::Duration::from_secs(60));
        assert!(err.is_rate_limit());

        let err = LlmError::ContextLengthExceeded { max: 4096, provided: 5000 };
        assert!(err.is_context_error());
    }
}
