//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT ZK-MCP - Zero-Knowledge Proofs for MCP Protocol
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! MCP (Model Context Protocol) protokolü için ZK-SNARK tabanlı
//! Sıfır Bilgi İspatı (Zero-Knowledge Proof) entegrasyonu.
//!
//! ## Güvenlik Özellikleri:
//!
//! - Dış araçlara veri sızıntısı önleme

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - İşlem doğrulama without revealing data
//! - Audit trail without compromising privacy

pub mod proof;
pub mod circuit;
pub mod mcp;
pub mod verifier;

pub use proof::*;
pub use circuit::{Circuit, CircuitBuilder as CircuitBuild, Wire, CompiledCircuit, CircuitCompiler};
pub use mcp::*;
pub use verifier::*;

use sentient_common::error::SENTIENTError;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  ZK ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum ZkError {
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    
    #[error("Proof verification failed: {0}")]
    ProofVerificationFailed(String),
    
    #[error("Invalid proof format")]
    InvalidProofFormat,
    
    #[error("Circuit compilation failed: {0}")]
    CircuitCompilationFailed(String),
    
    #[error("Trusted setup required")]
    TrustedSetupRequired,
    
    #[error("Privacy violation detected")]
    PrivacyViolation,
    
    #[error("MCP protocol error: {0}")]
    McpError(String),
}

impl From<ZkError> for SENTIENTError {
    fn from(e: ZkError) -> Self {
        SENTIENTError::Core(format!("SENTIENT_ZK_MCP: {}", e))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROOF TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// ZK Proof algorithm
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofAlgorithm {
    /// Groth16 - Standard SNARK
    Groth16,
    /// PLONK - Universal setup
    Plonk,
    /// Bulletproofs - No trusted setup
    Bulletproofs,
    /// Simplified simulation (development)
    Simulated,
}

impl Default for ProofAlgorithm {
    fn default() -> Self {
        Self::Simulated
    }
}

/// Proof status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofStatus {
    Pending,
    Valid,
    Invalid,
    Expired,
}

/// Zero-Knowledge Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    /// Proof ID
    pub id: uuid::Uuid,
    
    /// Algorithm used
    pub algorithm: ProofAlgorithm,
    
    /// Public inputs (revealed)
    pub public_inputs: Vec<String>,
    
    /// Proof data (opaque)
    pub proof_data: String,
    
    /// Verification key hash
    pub vk_hash: String,
    
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Expiry timestamp
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Status
    pub status: ProofStatus,
}

impl ZkProof {
    /// Create new proof
    pub fn new(algorithm: ProofAlgorithm, public_inputs: Vec<String>, proof_data: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            algorithm,
            public_inputs,
            proof_data,
            vk_hash: "simulation_vk".to_string(),
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            status: ProofStatus::Pending,
        }
    }

    /// Check if proof is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            chrono::Utc::now() > expires
        } else {
            false
        }
    }

    /// Validate proof
    pub fn validate(&self) -> ZkResult<bool> {
        if self.is_expired() {
            return Err(ZkError::ProofVerificationFailed("Proof expired".into()));
        }
        
        // In simulation mode, always valid
        Ok(matches!(self.algorithm, ProofAlgorithm::Simulated))
    }
}

/// ZK Proof context for MCP requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofContext {
    /// Tool name being called
    pub tool_name: String,
    
    /// Request hash (commitment)
    pub request_hash: String,
    
    /// Privacy level
    pub privacy_level: PrivacyLevel,
    
    /// Whether to include response in proof
    pub include_response: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyLevel {
    /// Only prove request was made
    RequestOnly,
    /// Prove request parameters hash
    ParameterHash,
    /// Prove full request structure
    FullRequest,
}

impl Default for PrivacyLevel {
    fn default() -> Self {
        Self::ParameterHash
    }
}

pub type ZkResult<T> = Result<T, ZkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_creation() {
        let proof = ZkProof::new(
            ProofAlgorithm::Simulated,
            vec!["input1".into()],
            "proof_data".into(),
        );
        
        assert_eq!(proof.status, ProofStatus::Pending);
        assert!(!proof.is_expired());
    }

    #[test]
    fn test_proof_expiry() {
        let mut proof = ZkProof::new(
            ProofAlgorithm::Simulated,
            vec!["input1".into()],
            "proof_data".into(),
        );
        
        proof.expires_at = Some(chrono::Utc::now() - chrono::Duration::seconds(1));
        
        assert!(proof.is_expired());
    }
}
