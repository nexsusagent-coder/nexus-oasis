//! Backup error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackupError {
    #[error("Backup failed: {0}")]
    BackupFailed(String),

    #[error("Restore failed: {0}")]
    RestoreFailed(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Invalid backup file: {0}")]
    InvalidBackup(String),

    #[error("Backup not found: {0}")]
    NotFound(String),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("Backup already in progress")]
    AlreadyInProgress,

    #[error("Retention policy violation: backup too recent")]
    RetentionViolation,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Schedule error: {0}")]
    ScheduleError(String),
}

pub type Result<T> = std::result::Result<T, BackupError>;
