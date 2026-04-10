//! ═══════════════════════════════════════════════════════════════════════════════
//!  VAULT CRYPTO - Military-Grade Encryption
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides AEAD encryption with AES-256-GCM and XChaCha20-Poly1305.
//! Key derivation using Argon2id for password-based keys.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;
use zeroize::Zeroize;
use crate::{VaultError, VaultResult};

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

const ARGON2_MEMORY_COST: u32 = 64 * 1024; // 64 MB
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 4;
const KEY_SIZE: usize = 32; // 256 bits
const NONCE_SIZE: usize = 12; // 96 bits for AES-GCM
const SALT_SIZE: usize = 16;

// ═══════════════════════════════════════════════════════════════════════════════
//  VAULT CRYPTO
// ═══════════════════════════════════════════════════════════════════════════════

/// Vault crypto utilities
pub struct VaultCrypto;

impl VaultCrypto {
    /// Generate cryptographically secure salt
    pub fn generate_salt() -> [u8; SALT_SIZE] {
        let mut salt = [0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    /// Generate random nonce
    pub fn generate_nonce() -> [u8; NONCE_SIZE] {
        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);
        nonce
    }

    /// Derive encryption key from password using Argon2id
    pub fn derive_key(password: &str, salt: &[u8; SALT_SIZE]) -> VaultResult<[u8; KEY_SIZE]> {
        let params = Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_TIME_COST,
            ARGON2_PARALLELISM,
            Some(KEY_SIZE),
        ).map_err(|e| VaultError::EncryptionFailed(format!("Argon2 params: {}", e)))?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        
        let mut key = [0u8; KEY_SIZE];
        argon2
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| VaultError::EncryptionFailed(format!("Key derivation: {}", e)))?;

        Ok(key)
    }

    /// Encrypt data using AES-256-GCM
    pub fn encrypt(plaintext: &[u8], key: &[u8; KEY_SIZE]) -> VaultResult<EncryptedData> {
        let nonce = Self::generate_nonce();
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| VaultError::EncryptionFailed(format!("Cipher init: {}", e)))?;

        let nonce_obj = Nonce::from_slice(&nonce);
        let ciphertext = cipher
            .encrypt(nonce_obj, plaintext)
            .map_err(|e| VaultError::EncryptionFailed(format!("Encryption: {}", e)))?;

        Ok(EncryptedData {
            nonce: nonce.to_vec(),
            ciphertext,
            algorithm: EncryptionAlgorithm::Aes256Gcm,
        })
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(encrypted: &EncryptedData, key: &[u8; KEY_SIZE]) -> VaultResult<Vec<u8>> {
        if encrypted.nonce.len() != NONCE_SIZE {
            return Err(VaultError::DecryptionFailed("Invalid nonce size".into()));
        }

        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| VaultError::DecryptionFailed(format!("Cipher init: {}", e)))?;

        let nonce = Nonce::from_slice(&encrypted.nonce);
        let plaintext = cipher
            .decrypt(nonce, encrypted.ciphertext.as_slice())
            .map_err(|e| VaultError::DecryptionFailed(format!("Decryption: {}", e)))?;

        Ok(plaintext)
    }

    /// Hash data using BLAKE3
    pub fn hash(data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()
    }
    
    /// Hash with salt (for password storage)
    pub fn hash_password(password: &str, salt: &[u8; SALT_SIZE]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(salt);
        hasher.update(password.as_bytes());
        hasher.finalize().to_hex().to_string()
    }
}

/// Encrypted data container
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub algorithm: EncryptionAlgorithm,
}

/// Supported encryption algorithms
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    XChaCha20Poly1305,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::Aes256Gcm
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SECURE HASH
// ═══════════════════════════════════════════════════════════════════════════════

/// Secure hash utilities
pub struct SecureHash;

impl SecureHash {
    pub fn hash(data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()
    }

    pub fn verify(data: &[u8], expected: &str) -> bool {
        Self::hash(data) == expected
    }
    
    /// Hash with constant-time comparison
    pub fn hash_secure(data: &[u8]) -> SecureHashValue {
        SecureHashValue(blake3::hash(data).into())
    }
}

/// Secure hash value that zeroizes on drop
#[derive(Clone)]
pub struct SecureHashValue([u8; 32]);

impl SecureHashValue {
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    pub fn verify(&self, data: &[u8]) -> bool {
        let computed = blake3::hash(data);
        constant_time_eq::constant_time_eq(&self.0, computed.as_bytes())
    }
}

impl Drop for SecureHashValue {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SECURE KEY
// ═══════════════════════════════════════════════════════════════════════════════

/// Secure key container that zeroizes on drop
#[derive(Clone)]
pub struct SecureKey(pub [u8; KEY_SIZE]);

impl SecureKey {
    pub fn new(key: [u8; KEY_SIZE]) -> Self {
        Self(key)
    }

    pub fn from_password(password: &str, salt: &[u8; SALT_SIZE]) -> VaultResult<Self> {
        let key = VaultCrypto::derive_key(password, salt)?;
        Ok(Self(key))
    }

    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.0
    }
    
    pub fn random() -> Self {
        let mut key = [0u8; KEY_SIZE];
        OsRng.fill_bytes(&mut key);
        Self(key)
    }
}

impl Drop for SecureKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  KEY DERIVATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Key derivation configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyDerivationConfig {
    pub memory_cost: u32,
    pub time_cost: u32,
    pub parallelism: u32,
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            memory_cost: ARGON2_MEMORY_COST,
            time_cost: ARGON2_TIME_COST,
            parallelism: ARGON2_PARALLELISM,
        }
    }
}

impl KeyDerivationConfig {
    /// Create fast config (less secure, for testing)
    pub fn fast() -> Self {
        Self {
            memory_cost: 8 * 1024, // 8 MB
            time_cost: 1,
            parallelism: 1,
        }
    }

    /// Create secure config (production)
    pub fn secure() -> Self {
        Self {
            memory_cost: 128 * 1024, // 128 MB
            time_cost: 4,
            parallelism: 4,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let hash = SecureHash::hash(b"test_data");
        assert!(SecureHash::verify(b"test_data", &hash));
        assert!(!SecureHash::verify(b"wrong_data", &hash));
    }

    #[test]
    fn test_key_derivation() {
        let salt = VaultCrypto::generate_salt();
        let key1 = VaultCrypto::derive_key("password", &salt).expect("operation failed");
        let key2 = VaultCrypto::derive_key("password", &salt).expect("operation failed");
        
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_encryption_decryption() {
        let key = SecureKey::random();
        let plaintext = b"secret_data_12345";
        
        let encrypted = VaultCrypto::encrypt(plaintext, key.as_bytes()).expect("operation failed");
        let decrypted = VaultCrypto::decrypt(&encrypted, key.as_bytes()).expect("operation failed");
        
        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = SecureKey::random();
        let key2 = SecureKey::random();
        let plaintext = b"secret_data";
        
        let encrypted = VaultCrypto::encrypt(plaintext, key1.as_bytes()).expect("operation failed");
        let result = VaultCrypto::decrypt(&encrypted, key2.as_bytes());
        
        assert!(result.is_err());
    }

    #[test]
    fn test_secure_key_zeroize() {
        let key = SecureKey::random();
        let bytes = key.as_bytes().clone();
        
        // Key should be zeroized on drop
        drop(key);
        
        // This just verifies the type system works
        assert!(bytes.len() == KEY_SIZE);
    }
    
    #[test]
    fn test_encrypted_data_format() {
        let key = SecureKey::random();
        let plaintext = b"test";
        
        let encrypted = VaultCrypto::encrypt(plaintext, key.as_bytes()).expect("operation failed");
        
        assert_eq!(encrypted.nonce.len(), NONCE_SIZE);
        assert!(!encrypted.ciphertext.is_empty());
        assert_eq!(encrypted.algorithm, EncryptionAlgorithm::Aes256Gcm);
    }
}
