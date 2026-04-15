//! Digest error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DigestError {
    #[error("Collection failed: {0}")]
    CollectionFailed(String),

    #[error("Composition failed: {0}")]
    CompositionFailed(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Section error: {0}")]
    SectionError(String),

    #[error("Schedule error: {0}")]
    ScheduleError(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("No data available")]
    NoData,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type DigestResult<T> = Result<T, DigestError>;
