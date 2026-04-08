//! TEE Sealing

use crate::{TeeError, TeeMeasurement, TeePlatform, TeeResult};
use serde::{Deserialize, Serialize};

/// Sealed data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub measurement_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Sealing policy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SealingPolicy {
    MeasurementBinding,
    FamilyBinding,
    PlatformBinding,
}

/// Sealing key
pub struct SealingKey {
    key: [u8; 32],
    measurement_hash: String,
}

impl SealingKey {
    pub fn derive(measurement: &TeeMeasurement, policy: SealingPolicy) -> TeeResult<Self> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"SENTIENT_TEE_SEALING_KEY_V1");
        hasher.update(measurement.hash.as_bytes());
        hasher.update(&[policy as u8]);
        
        let derived = hasher.finalize();
        let key: [u8; 32] = derived.as_bytes()[..32].try_into()
            .map_err(|_| TeeError::SealingFailed("Key derivation failed".into()))?;

        Ok(Self {
            key,
            measurement_hash: measurement.hash.clone(),
        })
    }

    pub fn seal(&self, plaintext: &[u8], policy: SealingPolicy) -> TeeResult<SealedData> {
        // Simplified sealing - in production would use AES-GCM
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.key);
        hasher.update(plaintext);
        let sealed_hash = hasher.finalize();
        
        Ok(SealedData {
            ciphertext: sealed_hash.as_bytes()[..plaintext.len().min(32)].to_vec(),
            nonce: vec![0u8; 12],
            measurement_hash: self.measurement_hash.clone(),
            created_at: chrono::Utc::now(),
        })
    }

    pub fn unseal(&self, _sealed: &SealedData) -> TeeResult<Vec<u8>> {
        // Simplified unsealing
        Ok(vec![])
    }
}

/// Sealing service
pub struct SealingService {
    platform: TeePlatform,
}

impl SealingService {
    pub fn new(platform: TeePlatform) -> Self {
        Self { platform }
    }

    pub fn seal_data(&self, data: &[u8], measurement: &TeeMeasurement) -> TeeResult<SealedData> {
        let key = SealingKey::derive(measurement, SealingPolicy::MeasurementBinding)?;
        key.seal(data, SealingPolicy::MeasurementBinding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing_key() {
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        let key = SealingKey::derive(&measurement, SealingPolicy::MeasurementBinding);
        assert!(key.is_ok());
    }
}
