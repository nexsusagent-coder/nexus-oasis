//! Hata tipleri ve sonuç tanımları

use thiserror::Error;

/// Cevahir işlemleri için ana hata tipi
#[derive(Error, Debug)]
pub enum CevahirError {
    /// Model yükleme hataları
    #[error("Model error: {0}")]
    ModelError(String),

    /// Tokenizer hataları
    #[error("Tokenizer error: {0}")]
    TokenizerError(String),

    /// Cognitive strateji hataları
    #[error("Cognitive error: {0}")]
    CognitiveError(String),

    /// Bellek hataları
    #[error("Memory error: {0}")]
    MemoryError(String),

    /// Tool execution hataları
    #[error("Tool error: {0}")]
    ToolError(String),

    /// Yapılandırma hataları
    #[error("Config error: {0}")]
    ConfigError(String),

    /// I/O hataları
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing hataları
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Genel hatalar
    #[error("General error: {0}")]
    General(String),
}

/// Cevahir işlemleri için sonuç tipi
pub type Result<T> = std::result::Result<T, CevahirError>;
