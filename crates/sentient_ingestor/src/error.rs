//! ═══════════════════════════════════════════════════════════════════════════════
//!  INGESTOR ERROR HANDLING
//! ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IngestorError {
    #[error("Parse hatası: {0}")]
    ParseError(String),
    
    #[error("Dosya okuma hatası: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("YAML deserialize hatası: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("JSON deserialize hatası: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Regex hatası: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("Walkdir hatası: {0}")]
    WalkdirError(String),
    
    #[error("Veritabanı hatası: {0}")]
    DatabaseError(String),
    
    #[error("SQLite hatası: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    #[error("Geçersiz skill formatı: {0}")]
    InvalidSkillFormat(String),
    
    #[error("Kategori bulunamadı: {0}")]
    CategoryNotFound(String),
    
    #[error("Duplicate skill: {0}")]
    DuplicateSkill(String),
    
    #[error("Boş skill adı")]
    EmptySkillName,
    
    #[error("Boş skill açıklaması")]
    EmptySkillDescription,
    
    #[error("Walker hatası: {0}")]
    WalkerError(String),
}

pub type IngestorResult<T> = Result<T, IngestorError>;
