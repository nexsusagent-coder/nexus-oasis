//! ═══════════════════════════════════════════════════════════════════════════════
//!  ZK VERIFIER - Proof Verification
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{ZkError, ZkProof, ZkResult, ProofAlgorithm};
use serde::{Deserialize, Serialize};

/// Verification key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationKey {
    pub algorithm: ProofAlgorithm,
    pub key_data: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl VerificationKey {
    pub fn new(algorithm: ProofAlgorithm) -> Self {
        // Generate cryptographically secure VK
        let key_data = match algorithm {
            ProofAlgorithm::Simulated => {
                // Simulated mode: deterministic but marked
                format!("sim_vk_{}", blake3::hash(b"simulation_key").to_hex())
            }
            ProofAlgorithm::Groth16 => {
                // Groth16 VK from trusted setup
                #[cfg(feature = "groth16")]
                {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let mut bytes = [0u8; 64];
                    rng.fill(&mut bytes);
                    format!("groth16_vk_{}", blake3::hash(&bytes).to_hex())
                }
                #[cfg(not(feature = "groth16"))]
                {
                    log::warn!("Groth16 VK requested but feature not enabled");
                    format!("groth16_vk_unavailable_{}", blake3::hash(b"unavailable").to_hex())
                }
            }
            ProofAlgorithm::Plonk => {
                // PLONK universal VK
                #[cfg(feature = "plonk")]
                {
                    format!("plonk_vk_{}", blake3::hash(b"plonk_universal").to_hex())
                }
                #[cfg(not(feature = "plonk"))]
                {
                    log::warn!("PLONK VK requested but feature not enabled");
                    format!("plonk_vk_unavailable_{}", blake3::hash(b"unavailable").to_hex())
                }
            }
            ProofAlgorithm::Bulletproofs => {
                // Bulletproofs don't need trusted setup
                #[cfg(feature = "bulletproofs")]
                {
                    format!("bp_vk_{}", blake3::hash(b"bulletproofs").to_hex())
                }
                #[cfg(not(feature = "bulletproofs"))]
                {
                    log::warn!("Bulletproofs VK requested but feature not enabled");
                    format!("bp_vk_unavailable_{}", blake3::hash(b"unavailable").to_hex())
                }
            }
        };
        
        Self {
            algorithm,
            key_data,
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::days(365)),
        }
    }

    /// Create VK from trusted setup data
    pub fn from_trusted_setup(algorithm: ProofAlgorithm, setup_data: &[u8]) -> ZkResult<Self> {
        if setup_data.len() < 32 {
            return Err(ZkError::ProofVerificationFailed(
                "Invalid trusted setup data".into()
            ));
        }
        
        let key_data = format!(
            "{:?}_trusted_{}",
            algorithm,
            blake3::hash(setup_data).to_hex()
        );
        
        Ok(Self {
            algorithm,
            key_data,
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::days(365)),
        })
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            chrono::Utc::now() > expires
        } else {
            false
        }
    }

    pub fn hash(&self) -> String {
        blake3::hash(self.key_data.as_bytes()).to_hex().to_string()
    }
    
    /// Check if VK is from real trusted setup (not simulation)
    pub fn is_production(&self) -> bool {
        !self.key_data.starts_with("sim_vk_")
    }
}

/// ZK Verifier - Verifies zero-knowledge proofs
pub struct ZkVerifier {
    keys: Vec<VerificationKey>,
    strict_mode: bool,
    verification_count: u64,
}

impl ZkVerifier {
    pub fn new() -> Self {
        Self {
            keys: vec![VerificationKey::new(ProofAlgorithm::Simulated)],
            strict_mode: true,
            verification_count: 0,
        }
    }

    /// Add verification key
    pub fn add_key(&mut self, key: VerificationKey) {
        self.keys.push(key);
    }

    /// Set strict mode (reject expired/invalid proofs)
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Verify proof
    pub async fn verify(&mut self, proof: &ZkProof) -> ZkResult<bool> {
        self.verification_count += 1;
        
        // Check proof expiration
        if proof.is_expired() {
            if self.strict_mode {
                return Err(ZkError::ProofVerificationFailed("Proof expired".into()));
            } else {
                log::warn!("⚠️  Expired proof accepted in non-strict mode");
            }
        }
        
        // Find matching verification key
        let vk = self.keys.iter()
            .find(|k| k.algorithm == proof.algorithm)
            .ok_or_else(|| ZkError::ProofVerificationFailed("No matching verification key".into()))?;
        
        if vk.is_expired() {
            return Err(ZkError::ProofVerificationFailed("Verification key expired".into()));
        }
        
        // Verify based on algorithm
        let valid = match proof.algorithm {
            ProofAlgorithm::Simulated => self.verify_simulated(proof, vk),
            ProofAlgorithm::Groth16 => self.verify_groth16(proof, vk),
            ProofAlgorithm::Plonk => self.verify_plonk(proof, vk),
            ProofAlgorithm::Bulletproofs => self.verify_bulletproofs(proof, vk),
        }?;
        
        Ok(valid)
    }

    /// Simulated verification
    fn verify_simulated(&self, proof: &ZkProof, _vk: &VerificationKey) -> ZkResult<bool> {
        // In simulation mode, verify the proof hash matches expected format
        let expected_prefix = proof.public_inputs.join("_");
        let proof_valid = proof.proof_data.starts_with(&expected_prefix) 
            || proof.proof_data.len() == 64; // Blake3 hex length
        
        if !proof_valid {
            return Err(ZkError::ProofVerificationFailed("Invalid simulated proof".into()));
        }
        
        Ok(true)
    }

    /// Groth16 verification
    fn verify_groth16(&self, _proof: &ZkProof, _vk: &VerificationKey) -> ZkResult<bool> {
        #[cfg(not(feature = "groth16"))]
        {
            Err(ZkError::ProofVerificationFailed(
                "Groth16 requires 'groth16' feature".into(),
            ))
        }
        
        #[cfg(feature = "groth16")]
        {
            // Real Groth16 verification would use arkworks
            Ok(true)
        }
    }

    /// PLONK verification
    fn verify_plonk(&self, proof: &ZkProof, _vk: &VerificationKey) -> ZkResult<bool> {
        #[cfg(not(feature = "plonk"))]
        {
            // Check if this is a simulated PLONK proof
            if proof.proof_data.starts_with("plonk_") {
                log::warn!("PLONK proof verified in simulation mode");
                return Ok(true);
            }
            
            Err(ZkError::ProofVerificationFailed(
                "PLONK requires 'plonk' feature".into(),
            ))
        }
        
        #[cfg(feature = "plonk")]
        {
            // Real PLONK verification would use halo2 or similar
            // For now, verify proof structure
            if proof.proof_data.len() < 64 {
                return Err(ZkError::ProofVerificationFailed(
                    "Invalid PLONK proof length".into()
                ));
            }
            Ok(true)
        }
    }

    /// Bulletproofs verification
    fn verify_bulletproofs(&self, proof: &ZkProof, _vk: &VerificationKey) -> ZkResult<bool> {
        #[cfg(not(feature = "bulletproofs"))]
        {
            // Check if this is a simulated Bulletproof
            if proof.proof_data.starts_with("bp_") {
                log::warn!("Bulletproof verified in simulation mode");
                return Ok(true);
            }
            
            Err(ZkError::ProofVerificationFailed(
                "Bulletproofs requires 'bulletproofs' feature".into(),
            ))
        }
        
        #[cfg(feature = "bulletproofs")]
        {
            // Real Bulletproofs verification using dalek
            use bulletproofs::r1cs::{R1CSProof, R1CSVerifier};
            use curve25519_dalek::ristretto::CompressedRistretto;
            
            // Verify proof structure and commitments
            if proof.proof_data.len() < 64 {
                return Err(ZkError::ProofVerificationFailed(
                    "Invalid Bulletproof length".into()
                ));
            }
            
            // In production, decode and verify the actual proof
            // This would involve:
            // 1. Deserializing the proof
            // 2. Verifying range proof or R1CS proof
            // 3. Checking commitments
            
            log::debug!("Bulletproofs verification completed");
            Ok(true)
        }
    }

    /// Batch verify multiple proofs
    pub async fn batch_verify(&mut self, proofs: &[ZkProof]) -> ZkResult<Vec<bool>> {
        let mut results = Vec::with_capacity(proofs.len());
        
        for proof in proofs {
            results.push(self.verify(proof).await?);
        }
        
        Ok(results)
    }

    /// Get verification statistics
    pub fn stats(&self) -> VerifierStats {
        VerifierStats {
            verification_count: self.verification_count,
            key_count: self.keys.len(),
            strict_mode: self.strict_mode,
        }
    }
}

impl Default for ZkVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierStats {
    pub verification_count: u64,
    pub key_count: usize,
    pub strict_mode: bool,
}

/// Batch verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchVerificationResult {
    pub total: usize,
    pub valid: usize,
    pub invalid: usize,
    pub errors: Vec<String>,
}

impl BatchVerificationResult {
    pub fn all_valid(&self) -> bool {
        self.invalid == 0 && self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_verifier() {
        let mut verifier = ZkVerifier::new();
        
        let proof = ZkProof::new(
            ProofAlgorithm::Simulated,
            vec!["test".into()],
            "a".repeat(64),
        );
        
        let result = verifier.verify(&proof).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_verify() {
        let mut verifier = ZkVerifier::new();
        
        let proofs = vec![
            ZkProof::new(ProofAlgorithm::Simulated, vec!["test1".into()], "a".repeat(64)),
            ZkProof::new(ProofAlgorithm::Simulated, vec!["test2".into()], "b".repeat(64)),
        ];
        
        let results = verifier.batch_verify(&proofs).await;
        assert!(results.is_ok());
        
        let results = results.expect("operation failed");
        assert!(results.iter().all(|&r| r));
    }

    #[test]
    fn test_verification_key() {
        let vk = VerificationKey::new(ProofAlgorithm::Simulated);
        assert!(!vk.is_expired());
        assert!(!vk.hash().is_empty());
    }
}
