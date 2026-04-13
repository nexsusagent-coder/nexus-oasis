//! ─── Embedding Error Types ───

use thiserror::Error;

pub type EmbedResult<T> = Result<T, EmbedError>;

#[derive(Debug, Error, Clone)]
pub enum EmbedError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("No embeddings returned")]
    NoEmbeddings,

    #[error("Invalid dimensions")]
    InvalidDimensions,

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Channel closed")]
    ChannelClosed,

    #[error("Timeout")]
    Timeout,

    #[error("Rate limited")]
    RateLimited,
}
