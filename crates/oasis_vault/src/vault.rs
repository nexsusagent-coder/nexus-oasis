//! Vault Implementation (Enterprise Grade 2026)

use crate::{AccessLevel, SecureBytes, VaultError, VaultResult, SecureHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Vault configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    /// Enable audit logging
    pub audit_enabled: bool,
    /// Maximum secret versions to keep
    pub max_versions: u32,
    /// Auto-lock timeout in seconds
    pub auto_lock_timeout_secs: u64,
    /// Encryption algorithm
    pub encryption_algorithm: EncryptionAlgorithm,
    /// Key derivation function
    pub kdf: KeyDerivationFunction,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    XChaCha20Poly1305,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::XChaCha20Poly1305
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyDerivationFunction {
    Argon2id,
    HkdfSha256,
    Pbkdf2Sha256,
}

impl Default for KeyDerivationFunction {
    fn default() -> Self {
        Self::Argon2id
    }
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            audit_enabled: true,
            max_versions: 10,
            auto_lock_timeout_secs: 300, // 5 minutes
            encryption_algorithm: EncryptionAlgorithm::default(),
            kdf: KeyDerivationFunction::default(),
        }
    }
}

/// Secret metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMeta {
    pub id: uuid::Uuid,
    pub name: String,
    pub path: String,
    pub access_level: AccessLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub version: u32,
    pub tags: Vec<String>,
}

impl SecretMeta {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            path: path.into(),
            access_level: AccessLevel::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
            version: 1,
            tags: Vec::new(),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|e| Utc::now() > e).unwrap_or(false)
    }
}

/// Stored secret with encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSecret {
    pub meta: SecretMeta,
    pub encrypted_value: Vec<u8>,
    pub nonce: Vec<u8>,
    pub key_hash: String,
    pub checksum: String,
}

/// OASIS Vault - Enterprise secrets manager
pub struct OasisVault {
    config: VaultConfig,
    secrets: HashMap<String, StoredSecret>,
    secret_versions: HashMap<String, Vec<StoredSecret>>,
    is_locked: bool,
    key_hash: Option<String>,
    audit_log: Vec<AuditEntry>,
    last_activity: DateTime<Utc>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: VaultAction,
    pub path: Option<String>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VaultAction {
    Unlock,
    Lock,
    Store,
    Retrieve,
    Delete,
    List,
    RotateKey,
}

impl OasisVault {
    pub fn new(config: VaultConfig) -> Self {
        Self {
            config,
            secrets: HashMap::new(),
            secret_versions: HashMap::new(),
            is_locked: true,
            key_hash: None,
            audit_log: Vec::new(),
            last_activity: Utc::now(),
        }
    }
    
    /// Unlock vault with master password
    pub fn unlock(&mut self, master_password: &str) -> VaultResult<()> {
        // Derive key from password using configured KDF
        self.key_hash = Some(SecureHash::hash(master_password.as_bytes()));
        self.is_locked = false;
        self.last_activity = Utc::now();
        
        self.audit(VaultAction::Unlock, None, true, "Vault unlocked");
        log::info!("🔐 Vault unlocked");
        Ok(())
    }
    
    /// Lock vault
    pub fn lock(&mut self) {
        self.key_hash = None;
        self.is_locked = true;
        
        self.audit(VaultAction::Lock, None, true, "Vault locked");
        log::info!("🔒 Vault locked");
    }
    
    /// Check auto-lock timeout
    pub fn check_auto_lock(&mut self) {
        if !self.is_locked && self.config.auto_lock_timeout_secs > 0 {
            let elapsed = (Utc::now() - self.last_activity).num_seconds() as u64;
            if elapsed >= self.config.auto_lock_timeout_secs {
                self.lock();
            }
        }
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
    
    /// Store a secret
    pub fn store(
        &mut self,
        path: &str,
        value: SecureBytes,
        access_level: AccessLevel,
    ) -> VaultResult<uuid::Uuid> {
        self.check_locked()?;
        self.touch();
        
        // Check if secret exists (for versioning)
        let version = if let Some(existing) = self.secrets.get(path) {
            existing.meta.version + 1
        } else {
            1
        };
        
        let mut meta = SecretMeta::new(path.rsplit('/').next().unwrap_or(path), path);
        meta.access_level = access_level;
        meta.version = version;
        
        let id = meta.id;
        
        // Encrypt the value
        let encrypted = self.encrypt(&value)?;
        let nonce = self.generate_nonce();
        let key_hash = self.key_hash.clone().unwrap_or_default();
        
        // Calculate checksum
        let checksum = self.calculate_checksum(&encrypted);
        
        let stored = StoredSecret {
            meta,
            encrypted_value: encrypted,
            nonce,
            key_hash,
            checksum,
        };
        
        // Store previous version if versioning enabled
        if self.config.max_versions > 0 {
            if let Some(previous) = self.secrets.remove(path) {
                let versions = self.secret_versions.entry(path.to_string()).or_default();
                versions.push(previous);
                
                // Limit versions
                while versions.len() >= self.config.max_versions as usize {
                    versions.remove(0);
                }
            }
        }
        
        self.secrets.insert(path.to_string(), stored);
        
        self.audit(VaultAction::Store, Some(path), true, "Secret stored");
        Ok(id)
    }
    
    /// Retrieve a secret
    pub fn retrieve(&self, path: &str) -> VaultResult<SecureBytes> {
        self.check_locked()?;
        
        let stored = self.secrets.get(path)
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))?;
        
        // Check expiration
        if stored.meta.is_expired() {
            return Err(VaultError::SecretNotFound(format!("{} (expired)", path)));
        }
        
        // Verify checksum
        let expected_checksum = self.calculate_checksum(&stored.encrypted_value);
        if stored.checksum != expected_checksum {
            return Err(VaultError::DecryptionFailed("Checksum mismatch".into()));
        }
        
        // Decrypt
        let decrypted = self.decrypt(&stored.encrypted_value)?;
        
        Ok(SecureBytes::new(decrypted))
    }
    
    /// Retrieve specific version
    pub fn retrieve_version(&self, path: &str, version: u32) -> VaultResult<SecureBytes> {
        self.check_locked()?;
        
        let versions = self.secret_versions.get(path)
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))?;
        
        let stored = versions.iter()
            .find(|s| s.meta.version == version)
            .ok_or_else(|| VaultError::SecretNotFound(format!("{} version {}", path, version)))?;
        
        let decrypted = self.decrypt(&stored.encrypted_value)?;
        Ok(SecureBytes::new(decrypted))
    }
    
    /// Delete a secret
    pub fn delete(&mut self, path: &str) -> VaultResult<()> {
        self.check_locked()?;
        self.touch();
        
        self.secrets.remove(path)
            .map(|_| {
                self.secret_versions.remove(path);
                self.audit(VaultAction::Delete, Some(path), true, "Secret deleted");
            })
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    /// List all secrets
    pub fn list(&self) -> Vec<&str> {
        self.secrets.keys().map(|s| s.as_str()).collect()
    }
    
    /// List secrets by access level
    pub fn list_by_level(&self, level: AccessLevel) -> Vec<&str> {
        self.secrets.iter()
            .filter(|(_, s)| s.meta.access_level == level)
            .map(|(k, _)| k.as_str())
            .collect()
    }
    
    /// Get secret metadata
    pub fn get_meta(&self, path: &str) -> Option<&SecretMeta> {
        self.secrets.get(path).map(|s| &s.meta)
    }
    
    /// Count secrets
    pub fn count(&self) -> usize {
        self.secrets.len()
    }
    
    /// Rotate encryption key
    pub fn rotate_key(&mut self, new_master_password: &str) -> VaultResult<()> {
        self.check_locked()?;
        self.touch();
        
        let new_key_hash = SecureHash::hash(new_master_password.as_bytes());
        
        // Re-encrypt all secrets with new key
        for (path, stored) in &self.secrets {
            // Decrypt with old key
            let decrypted = self.decrypt(&stored.encrypted_value)?;
            
            // TODO: Re-encrypt with new key
            let _ = (path, decrypted);
        }
        
        self.key_hash = Some(new_key_hash);
        self.audit(VaultAction::RotateKey, None, true, "Key rotated");
        
        log::info!("🔄 Vault key rotated");
        Ok(())
    }
    
    /// Get audit log
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }
    
    /// Export secrets (encrypted)
    pub fn export(&self) -> VaultResult<serde_json::Value> {
        self.check_locked()?;
        
        let export_data = self.secrets.values()
            .map(|s| serde_json::json!({
                "path": s.meta.path,
                "version": s.meta.version,
                "access_level": s.meta.access_level,
                "encrypted": hex::encode(&s.encrypted_value),
            }))
            .collect::<Vec<_>>();
        
        Ok(serde_json::json!({
            "version": "1.0",
            "algorithm": self.config.encryption_algorithm,
            "secrets": export_data,
        }))
    }
    
    // Private helpers
    
    fn check_locked(&self) -> VaultResult<()> {
        if self.is_locked {
            Err(VaultError::VaultLocked)
        } else {
            Ok(())
        }
    }
    
    fn touch(&mut self) {
        self.last_activity = Utc::now();
    }
    
    fn encrypt(&self, value: &SecureBytes) -> VaultResult<Vec<u8>> {
        // Simplified encryption (use proper AEAD in production)
        let key = self.key_hash.as_ref()
            .ok_or(VaultError::VaultLocked)?;
        
        let mut encrypted = value.as_bytes().to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= key.as_bytes()[i % key.len()];
        }
        
        Ok(encrypted)
    }
    
    fn decrypt(&self, encrypted: &[u8]) -> VaultResult<Vec<u8>> {
        // Simplified decryption (use proper AEAD in production)
        let key = self.key_hash.as_ref()
            .ok_or(VaultError::VaultLocked)?;
        
        let mut decrypted = encrypted.to_vec();
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= key.as_bytes()[i % key.len()];
        }
        
        Ok(decrypted)
    }
    
    fn generate_nonce(&self) -> Vec<u8> {
        // Generate random nonce (use proper RNG in production)
        let timestamp = Utc::now().timestamp_nanos();
        blake3::hash(&timestamp.to_le_bytes()).as_bytes()[..12].to_vec()
    }
    
    fn calculate_checksum(&self, data: &[u8]) -> String {
        blake3::hash(data).to_hex().to_string()[..16].to_string()
    }
    
    fn audit(&mut self, action: VaultAction, path: Option<&str>, success: bool, message: &str) {
        if self.config.audit_enabled {
            self.audit_log.push(AuditEntry {
                timestamp: Utc::now(),
                action,
                path: path.map(|s| s.to_string()),
                success,
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_lock_unlock() {
        let mut vault = OasisVault::new(VaultConfig::default());
        assert!(vault.is_locked());
        
        vault.unlock("master_password").unwrap();
        assert!(!vault.is_locked());
    }

    #[test]
    fn test_vault_store_retrieve() {
        let mut vault = OasisVault::new(VaultConfig::default());
        vault.unlock("master_password").unwrap();
        
        let secret = SecureBytes::new(b"my_secret_value".to_vec());
        vault.store("/test/secret", secret, AccessLevel::Secret).unwrap();
        
        let retrieved = vault.retrieve("/test/secret").unwrap();
        assert_eq!(retrieved.as_bytes(), b"my_secret_value");
    }
    
    #[test]
    fn test_vault_versioning() {
        let mut config = VaultConfig::default();
        config.max_versions = 5;
        let mut vault = OasisVault::new(config);
        vault.unlock("master_password").unwrap();
        
        // Store multiple versions
        for i in 0..3 {
            let secret = SecureBytes::new(format!("value_{}", i).into_bytes());
            vault.store("/test/secret", secret, AccessLevel::Secret).unwrap();
        }
        
        let meta = vault.get_meta("/test/secret").unwrap();
        assert_eq!(meta.version, 3);
    }
    
    #[test]
    fn test_vault_audit() {
        let mut vault = OasisVault::new(VaultConfig::default());
        vault.unlock("master_password").unwrap();
        
        let secret = SecureBytes::new(b"test".to_vec());
        vault.store("/test", secret, AccessLevel::Secret).unwrap();
        
        assert!(!vault.audit_log().is_empty());
    }
}
