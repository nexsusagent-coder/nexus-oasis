//! Encryption utilities for backups

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use serde::{Deserialize, Serialize};

use crate::Result;

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
    /// Key derivation iterations
    pub iterations: u32,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            kdf: KeyDerivationFunction::Pbkdf2,
            iterations: 100_000,
        }
    }
}

/// Supported encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    /// AES-256 in GCM mode (recommended)
    Aes256Gcm,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
}

/// Key derivation functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    /// PBKDF2
    Pbkdf2,
    /// Argon2id
    Argon2id,
    /// HKDF
    Hkdf,
}

/// Encrypt data
pub fn encrypt(data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| crate::BackupError::EncryptionError(e.to_string()))?;
    
    let nonce = Nonce::from_slice(nonce);
    
    cipher.encrypt(nonce, data)
        .map_err(|e| crate::BackupError::EncryptionError(e.to_string()))
}

/// Decrypt data
pub fn decrypt(encrypted: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| crate::BackupError::EncryptionError(e.to_string()))?;
    
    let nonce = Nonce::from_slice(nonce);
    
    cipher.decrypt(nonce, encrypted)
        .map_err(|e| crate::BackupError::EncryptionError(e.to_string()))
}

/// Derive key from password
pub fn derive_key(password: &str, salt: &[u8], iterations: u32) -> Result<[u8; 32]> {
    use sha2::Sha256;
    
    let mut key = [0u8; 32];
    
    pbkdf2::pbkdf2::<hmac::Hmac<Sha256>>(
        password.as_bytes(),
        salt,
        iterations as u32,
        &mut key,
    ).map_err(|_| crate::BackupError::EncryptionError("Key derivation failed".to_string()))?;
    
    Ok(key)
}

/// Generate random nonce
pub fn generate_nonce() -> [u8; 12] {
    use rand::RngCore;
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

/// Generate random salt
pub fn generate_salt() -> [u8; 16] {
    use rand::RngCore;
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let data = b"test data for encryption";
        let key = [0u8; 32];
        let nonce = [0u8; 12];

        let encrypted = encrypt(data, &key, &nonce).unwrap();
        let decrypted = decrypt(&encrypted, &key, &nonce).unwrap();

        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_derive_key() {
        let password = "test_password";
        let salt = b"test_salt";
        
        let key1 = derive_key(password, salt, 1000).unwrap();
        let key2 = derive_key(password, salt, 1000).unwrap();
        
        assert_eq!(key1, key2);
    }
}
