//! RAG Engine errors

use thiserror::Error;

/// RAG error types
#[derive(Debug, Error)]
pub enum RagError {
    #[error("Document error: {0}")]
    Document(String),

    #[error("Chunking error: {0}")]
    Chunking(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Retrieval error: {0}")]
    Retrieval(String),

    #[error("Store error: {0}")]
    Store(String),

    #[error("Pipeline error: {0}")]
    Pipeline(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Document not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Processing timeout")]
    Timeout,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },

    #[error("Index already exists: {0}")]
    AlreadyExists(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),

    #[error("Tokenization error: {0}")]
    Tokenization(String),

    #[error("Cache error: {0}")]
    Cache(String),
}

impl RagError {
    /// Create document error
    pub fn document(msg: impl Into<String>) -> Self {
        Self::Document(msg.into())
    }

    /// Create chunking error
    pub fn chunking(msg: impl Into<String>) -> Self {
        Self::Chunking(msg.into())
    }

    /// Create embedding error
    pub fn embedding(msg: impl Into<String>) -> Self {
        Self::Embedding(msg.into())
    }

    /// Create index error
    pub fn index(msg: impl Into<String>) -> Self {
        Self::Index(msg.into())
    }

    /// Create retrieval error
    pub fn retrieval(msg: impl Into<String>) -> Self {
        Self::Retrieval(msg.into())
    }

    /// Create store error
    pub fn store(msg: impl Into<String>) -> Self {
        Self::Store(msg.into())
    }

    /// Create config error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create not found error
    pub fn not_found(id: impl Into<String>) -> Self {
        Self::NotFound(id.into())
    }

    /// Create invalid input error
    pub fn invalid_input(msg: impl Into<String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Timeout | Self::RateLimit | Self::Cache(_)
        )
    }
}

/// Result type for RAG operations
pub type Result<T> = std::result::Result<T, RagError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = RagError::document("test error");
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_dimension_mismatch() {
        let err = RagError::DimensionMismatch {
            expected: 768,
            actual: 512,
        };
        assert!(err.to_string().contains("768"));
        assert!(err.to_string().contains("512"));
    }

    #[test]
    fn test_recoverable() {
        assert!(RagError::Timeout.is_recoverable());
        assert!(!RagError::not_found("test").is_recoverable());
    }
}
