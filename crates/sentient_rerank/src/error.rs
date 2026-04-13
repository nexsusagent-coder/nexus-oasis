//! ─── Reranker Error Types ───

use thiserror::Error;

pub type RerankResult<T> = Result<T, RerankError>;

#[derive(Debug, Error)]
pub enum RerankError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Timeout")]
    Timeout,

    #[error("Rate limited")]
    RateLimited,
}
