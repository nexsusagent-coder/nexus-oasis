// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Schema Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SchemaError>;

/// Schema-related errors
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Schema validation failed: {0}")]
    ValidationError(String),

    #[error("Extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("Function call failed: {0}")]
    FunctionCallFailed(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid response from LLM: {0}")]
    InvalidResponse(String),

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,

    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl SchemaError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SchemaError::ProviderError(_) |
            SchemaError::HttpError(_)
        )
    }
}
