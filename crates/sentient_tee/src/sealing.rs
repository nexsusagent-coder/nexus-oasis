//! ═══════════════════════════════════════════════════════════════════════════════
//!  TEE SEALING - Hardware-Backed Data Protection
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Sealing encrypts data so it can only be unsealed by the same TEE (or specific
//! policy). Uses hardware-backed keys from AMD SEV-SNP or Intel TDX.

use crate::{TeeError, TeeMeasurement, TeePlatform, TeeResult};
use crate::hardware::HardwareKeyDerivation;
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use rand::RngCore;

// ═══════════════════════════════════════════════════════════════════════════════
//  SEALED DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

/// Sealed data container with AES-256-GCM encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedData {
    /// Encrypted ciphertext
    pub ciphertext: Vec<u8>,
    /// Nonce/IV for AES-GCM
    pub nonce: Vec<u8>,
    /// Authentication tag
    pub tag: Vec<u8>,
    /// Measurement hash when sealed
    pub measurement_hash: String,
    /// Platform used for sealing
    pub platform: TeePlatform,
    /// Sealing policy
    pub policy: SealingPolicy,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Version for future compatibility
    pub version: u32,
}

/// Sealing policy determines unsealing conditions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SealingPolicy {
    /// Data can only be unsealed by same measurement (same code)
    MeasurementBinding,
    /// Data can be unsealed by same signer/family
    FamilyBinding,
    /// Data can be unsealed on same platform
    PlatformBinding,
    /// Data sealed for specific migration target
    MigrationBinding,
}

/// Sealing key derived from hardware
pub struct SealingKey {
    key: [u8; 32],
    measurement_hash: String,
    platform: TeePlatform,
    policy: SealingPolicy,
}

impl SealingKey {
    /// Derive sealing key from TEE measurement
    /// 
    /// The key is derived from hardware-protected secrets:
    /// - AMD SEV-SNP: Derived from VCEK and measurement
    /// - Intel TDX: Derived from RTMR and MRTD
    /// - Simulation: Software-derived (not secure)
    pub fn derive(measurement: &TeeMeasurement, policy: SealingPolicy) -> TeeResult<Self> {
        let hkd = HardwareKeyDerivation::new(measurement.platform);
        
        // Create context for key derivation
        let context = Self::create_context(&measurement.hash, policy);
        let key_bytes = hkd.derive_key(&context, 32)?;
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes[..32]);

        Ok(Self {
            key,
            measurement_hash: measurement.hash.clone(),
            platform: measurement.platform,
            policy,
        })
    }
    
    fn create_context(measurement_hash: &str, policy: SealingPolicy) -> Vec<u8> {
        let mut context = Vec::new();
        context.extend_from_slice(b"SENTIENT_TEE_SEALING_KEY_V2");
        context.extend_from_slice(measurement_hash.as_bytes());
        context.extend_from_slice(&[policy as u8]);
        context
    }

    /// Seal (encrypt) data using AES-256-GCM
    /// 
    /// The sealed data can only be unsealed by a TEE with the same
    /// measurement (or according to the sealing policy).
    pub fn seal(&self, plaintext: &[u8]) -> TeeResult<SealedData> {
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        
        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| TeeError::SealingFailed(format!("Cipher init failed: {}", e)))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| TeeError::SealingFailed(format!("Encryption failed: {}", e)))?;
        
        // AES-GCM appends tag to ciphertext, split it
        let (ct, tag) = ciphertext.split_at(ciphertext.len().saturating_sub(16));
        
        Ok(SealedData {
            ciphertext: ct.to_vec(),
            nonce: nonce_bytes.to_vec(),
            tag: tag.to_vec(),
            measurement_hash: self.measurement_hash.clone(),
            platform: self.platform,
            policy: self.policy,
            created_at: chrono::Utc::now(),
            version: 2,
        })
    }

    /// Unseal (decrypt) data
    /// 
    /// Returns the original plaintext if:
    /// - The sealing policy is satisfied
    /// - The measurement matches (for MeasurementBinding)
    /// - The tag verifies (integrity check passes)
    pub fn unseal(&self, sealed: &SealedData) -> TeeResult<Vec<u8>> {
        // Verify policy matches
        if sealed.policy != self.policy {
            return Err(TeeError::UnsealingFailed("Policy mismatch".into()));
        }
        
        // Verify measurement (for measurement binding)
        if sealed.policy == SealingPolicy::MeasurementBinding {
            if sealed.measurement_hash != self.measurement_hash {
                return Err(TeeError::UnsealingFailed("Measurement mismatch".into()));
            }
        }
        
        // Verify version
        if sealed.version > 2 {
            return Err(TeeError::UnsealingFailed(format!("Unsupported version: {}", sealed.version)));
        }
        
        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| TeeError::UnsealingFailed(format!("Cipher init failed: {}", e)))?;
        
        // Reconstruct ciphertext with tag
        let mut ct_with_tag = sealed.ciphertext.clone();
        ct_with_tag.extend_from_slice(&sealed.tag);
        
        let nonce = Nonce::from_slice(
            sealed.nonce.as_slice().try_into()
                .map_err(|_| TeeError::UnsealingFailed("Invalid nonce".into()))?
        );
        
        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ct_with_tag.as_slice())
            .map_err(|e| TeeError::UnsealingFailed(format!("Decryption failed: {}", e)))?;
        
        Ok(plaintext)
    }
    
    /// Get the measurement hash
    pub fn measurement_hash(&self) -> &str {
        &self.measurement_hash
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SEALING SERVICE
// ═══════════════════════════════════════════════════════════════════════════════

/// High-level sealing service
pub struct SealingService {
    platform: TeePlatform,
    default_policy: SealingPolicy,
}

impl SealingService {
    pub fn new(platform: TeePlatform) -> Self {
        Self {
            platform,
            default_policy: SealingPolicy::MeasurementBinding,
        }
    }
    
    /// Set default sealing policy
    pub fn with_default_policy(mut self, policy: SealingPolicy) -> Self {
        self.default_policy = policy;
        self
    }

    /// Seal data with measurement binding
    pub fn seal_data(&self, data: &[u8], measurement: &TeeMeasurement) -> TeeResult<SealedData> {
        self.seal_with_policy(data, measurement, self.default_policy)
    }
    
    /// Seal data with specific policy
    pub fn seal_with_policy(
        &self,
        data: &[u8],
        measurement: &TeeMeasurement,
        policy: SealingPolicy,
    ) -> TeeResult<SealedData> {
        let key = SealingKey::derive(measurement, policy)?;
        key.seal(data)
    }
    
    /// Unseal data
    pub fn unseal_data(
        &self,
        sealed: &SealedData,
        measurement: &TeeMeasurement,
    ) -> TeeResult<Vec<u8>> {
        let key = SealingKey::derive(measurement, sealed.policy)?;
        key.unseal(sealed)
    }
    
    /// Migrate sealed data to new measurement
    /// 
    /// This unseals with old measurement and reseals with new one.
    /// Requires MigrationBinding policy or special migration key.
    pub fn migrate(
        &self,
        sealed: &SealedData,
        old_measurement: &TeeMeasurement,
        new_measurement: &TeeMeasurement,
        new_policy: SealingPolicy,
    ) -> TeeResult<SealedData> {
        // Verify migration is allowed
        match sealed.policy {
            SealingPolicy::MigrationBinding | SealingPolicy::FamilyBinding => {
                // Migration allowed
            }
            _ => {
                return Err(TeeError::SealingFailed(
                    "Migration not allowed with current policy".into()
                ));
            }
        }
        
        // Unseal with old measurement
        let plaintext = self.unseal_data(sealed, old_measurement)?;
        
        // Reseal with new measurement
        self.seal_with_policy(&plaintext, new_measurement, new_policy)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SEALED STORAGE (Persistent Sealed Data)
// ═══════════════════════════════════════════════════════════════════════════════

/// Persistent sealed storage
pub struct SealedStorage {
    base_path: std::path::PathBuf,
    service: SealingService,
}

impl SealedStorage {
    pub fn new(base_path: std::path::PathBuf, platform: TeePlatform) -> Self {
        Self {
            base_path,
            service: SealingService::new(platform),
        }
    }
    
    /// Store sealed data with a key
    pub fn store(
        &self,
        key: &str,
        data: &[u8],
        measurement: &TeeMeasurement,
    ) -> TeeResult<()> {
        let sealed = self.service.seal_data(data, measurement)?;
        let serialized = serde_json::to_vec(&sealed)
            .map_err(|e| TeeError::SealingFailed(e.to_string()))?;
        
        let path = self.base_path.join(format!("{}.sealed", key));
        std::fs::create_dir_all(&self.base_path)?;
        std::fs::write(path, serialized)?;
        
        Ok(())
    }
    
    /// Retrieve sealed data
    pub fn retrieve(
        &self,
        key: &str,
        measurement: &TeeMeasurement,
    ) -> TeeResult<Vec<u8>> {
        let path = self.base_path.join(format!("{}.sealed", key));
        let serialized = std::fs::read(path)
            .map_err(|e| TeeError::UnsealingFailed(e.to_string()))?;
        
        let sealed: SealedData = serde_json::from_slice(&serialized)
            .map_err(|e| TeeError::UnsealingFailed(e.to_string()))?;
        
        self.service.unseal_data(&sealed, measurement)
    }
    
    /// Check if sealed data exists
    pub fn exists(&self, key: &str) -> bool {
        self.base_path.join(format!("{}.sealed", key)).exists()
    }
    
    /// Delete sealed data
    pub fn delete(&self, key: &str) -> TeeResult<()> {
        let path = self.base_path.join(format!("{}.sealed", key));
        std::fs::remove_file(path)
            .map_err(|e| TeeError::UnsealingFailed(e.to_string()))?;
        Ok(())
    }
    
    /// List all sealed keys
    pub fn list_keys(&self) -> TeeResult<Vec<String>> {
        let mut keys = Vec::new();
        
        let entries = std::fs::read_dir(&self.base_path)
            .map_err(|e| TeeError::UnsealingFailed(e.to_string()))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".sealed") {
                        keys.push(name.trim_end_matches(".sealed").to_string());
                    }
                }
            }
        }
        
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing_key_derivation() {
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        let key = SealingKey::derive(&measurement, SealingPolicy::MeasurementBinding);
        assert!(key.is_ok());
    }

    #[test]
    fn test_seal_unseal() {
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        let key = SealingKey::derive(&measurement, SealingPolicy::MeasurementBinding).expect("operation failed");
        
        let plaintext = b"Hello, TEE World!";
        let sealed = key.seal(plaintext).expect("operation failed");
        
        // Verify ciphertext is different from plaintext
        assert_ne!(sealed.ciphertext.as_slice(), plaintext);
        assert!(!sealed.nonce.is_empty());
        assert_eq!(sealed.tag.len(), 16);
        
        // Unseal
        let unsealed = key.unseal(&sealed).expect("operation failed");
        assert_eq!(unsealed.as_slice(), plaintext);
    }
    
    #[test]
    fn test_sealing_service() {
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        let service = SealingService::new(TeePlatform::Simulation);
        
        let data = b"Secret data to seal";
        let sealed = service.seal_data(data, &measurement).expect("operation failed");
        
        let unsealed = service.unseal_data(&sealed, &measurement).expect("operation failed");
        assert_eq!(unsealed.as_slice(), data);
    }
    
    #[test]
    fn test_measurement_binding() {
        let measurement1 = TeeMeasurement::from_binary(b"test1", TeePlatform::Simulation);
        let measurement2 = TeeMeasurement::from_binary(b"test2", TeePlatform::Simulation);
        
        let key1 = SealingKey::derive(&measurement1, SealingPolicy::MeasurementBinding).expect("operation failed");
        let key2 = SealingKey::derive(&measurement2, SealingPolicy::MeasurementBinding).expect("operation failed");
        
        let sealed = key1.seal(b"secret").expect("operation failed");
        
        // Should fail with different measurement
        let result = key2.unseal(&sealed);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_sealed_storage() {
        let temp_dir = tempfile::tempdir().expect("operation failed");
        let storage = SealedStorage::new(temp_dir.path().to_path_buf(), TeePlatform::Simulation);
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        
        // Store
        storage.store("mykey", b"secret value", &measurement).expect("operation failed");
        assert!(storage.exists("mykey"));
        
        // Retrieve
        let retrieved = storage.retrieve("mykey", &measurement).expect("operation failed");
        assert_eq!(retrieved.as_slice(), b"secret value");
        
        // List
        let keys = storage.list_keys().expect("operation failed");
        assert!(keys.contains(&"mykey".to_string()));
        
        // Delete
        storage.delete("mykey").expect("operation failed");
        assert!(!storage.exists("mykey"));
    }
}
