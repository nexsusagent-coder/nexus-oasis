//! Vault Crypto

use crate::{VaultError, VaultResult};

/// Vault crypto utilities
pub struct VaultCrypto;

impl VaultCrypto {
    /// Generate salt
    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        for i in 0..16 {
            salt[i] = rand::random();
        }
        salt
    }

    /// Hash data
    pub fn hash(data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()
    }
}

/// Secure hash utilities
pub struct SecureHash;

impl SecureHash {
    pub fn hash(data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()
    }

    pub fn verify(data: &[u8], expected: &str) -> bool {
        Self::hash(data) == expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let hash = SecureHash::hash(b"test_data");
        assert!(SecureHash::verify(b"test_data", &hash));
    }
}
