//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS VAULT - Military-Grade Secrets Manager
//! ═══════════════════════════════════════════════════════════════════════════════

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod crypto;
pub mod vault;
pub mod secrets;
pub mod audit;
pub mod backends;
pub mod storage_backend;

pub use crypto::*;
pub use vault::*;
pub use secrets::*;
pub use audit::*;
pub use backends::*;
pub use storage_backend::{
    InMemoryBackend, FileBackend, BackendManager, StorageBackend,
    StoredSecret, SecretMetadata, WriteMode,
};

use sentient_common::error::SENTIENTError;
use serde::{Deserialize, Serialize};

/// Vault Error
#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("Secret not found: {0}")]
    SecretNotFound(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Vault locked")]
    VaultLocked,
}

impl From<VaultError> for SENTIENTError {
    fn from(e: VaultError) -> Self {
        SENTIENTError::Core(format!("OASIS_VAULT: {}", e))
    }
}

/// Result type for vault operations
pub type VaultResult<T> = Result<T, VaultError>;

/// Secret access level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum AccessLevel {
    Public = 0,
    #[default]
    Internal = 1,
    Confidential = 2,
    Secret = 3,
    TopSecret = 4,
}

/// Secure bytes (simplified)
#[derive(Clone)]
pub struct SecureBytes(Vec<u8>);

impl SecureBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
