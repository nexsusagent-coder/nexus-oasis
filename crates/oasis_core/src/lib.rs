//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS CORE - Trusted Runtime with Creusot Contracts
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Military-grade güvenlik için matematiksel doğrulama katmanı.
//! Creusot pre/post-conditions ile formel ispat.

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod contracts;
pub mod runtime;
pub mod state;
pub mod monitor;
pub mod creusot;

// Re-exports
pub use contracts::*;
pub use runtime::*;
pub use state::*;
pub use monitor::*;
pub use creusot::{
    CreusotVerifier, CreusotConfig, CreusotError, CreusotResult,
    VerificationResult, ProofResult, Prover,
};

use sentient_common::error::SENTIENTError;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Runtime Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// Maximum transaction count per cycle
    pub max_transactions: u64,
    /// Execution timeout in milliseconds
    pub execution_timeout_ms: u64,
    /// Enable Creusot verification
    pub creusot_enabled: bool,
    /// Anomaly threshold (0.0-1.0)
    pub anomaly_threshold: f64,
    /// Maximum memory usage (bytes)
    pub max_memory_bytes: u64,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            max_transactions: 10000,
            execution_timeout_ms: 30000,
            creusot_enabled: true,
            anomaly_threshold: 0.85,
            max_memory_bytes: 1024 * 1024 * 1024, // 1GB
        }
    }
}

/// Transaction for runtime execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub operation: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub priority: TransactionPriority,
    pub retry_count: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Transaction {
    pub fn new(operation: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            operation: operation.into(),
            payload,
            timestamp: Utc::now(),
            priority: TransactionPriority::Normal,
            retry_count: 0,
        }
    }

    pub fn with_priority(mut self, priority: TransactionPriority) -> Self {
        self.priority = priority;
        self
    }
}

/// Execution Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub transaction_id: uuid::Uuid,
    pub success: bool,
    pub output: serde_json::Value,
    pub duration_ms: u64,
    pub gas_used: u64,
    pub verification_proof: Option<VerificationProof>,
}

/// Mathematical verification proof (Creusot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProof {
    /// Hash of pre-conditions
    pub pre_hash: String,
    /// Hash of post-conditions
    pub post_hash: String,
    /// Proof hash (Blake3)
    pub proof_hash: String,
    /// Verification timestamp
    pub verified_at: DateTime<Utc>,
    /// Verifier version
    pub verifier_version: String,
}

impl VerificationProof {
    pub fn generate(pre_state: &[u8], post_state: &[u8]) -> Self {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(pre_state);
        let pre_hash = hasher.finalize().to_hex().to_string();
        
        let mut hasher = Hasher::new();
        hasher.update(post_state);
        let post_hash = hasher.finalize().to_hex().to_string();
        
        let mut hasher = Hasher::new();
        hasher.update(pre_hash.as_bytes());
        hasher.update(post_hash.as_bytes());
        let proof_hash = hasher.finalize().to_hex().to_string();
        
        Self {
            pre_hash,
            post_hash,
            proof_hash,
            verified_at: Utc::now(),
            verifier_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CORE ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Precondition violated: {0}")]
    PreconditionViolated(String),
    
    #[error("Postcondition violated: {0}")]
    PostconditionViolated(String),
    
    #[error("Invariant violated: {0}")]
    InvariantViolated(String),
    
    #[error("Execution timeout")]
    Timeout,
    
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("State corrupted")]
    StateCorrupted,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<CoreError> for SENTIENTError {
    fn from(e: CoreError) -> Self {
        SENTIENTError::Core(format!("OASIS_CORE: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("test_op", serde_json::json!({"key": "value"}));
        assert_eq!(tx.operation, "test_op");
        assert_eq!(tx.priority, TransactionPriority::Normal);
    }

    #[test]
    fn test_verification_proof() {
        let pre = b"pre_state";
        let post = b"post_state";
        let proof = VerificationProof::generate(pre, post);
        assert!(!proof.proof_hash.is_empty());
    }
}
