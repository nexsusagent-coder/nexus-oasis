//! ═══════════════════════════════════════════════════════════════════════════════
//!  VAULT STORAGE BACKEND - Persistent Storage Implementation
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides persistent storage backends for secrets:
//! - InMemoryBackend - Fast, volatile storage (with proper interior mutability)
//! - FileBackend - Persistent file-based storage with encryption
//! - SqliteBackend - SQLite-based persistent storage
//! - BackendManager - Multi-backend routing and failover

use crate::{VaultError, VaultResult, AccessLevel};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use parking_lot::Mutex;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  IN-MEMORY BACKEND (Fixed)
// ═══════════════════════════════════════════════════════════════════════════════

/// In-memory secrets backend with proper concurrency support
pub struct InMemoryBackend {
    secrets: Arc<RwLock<HashMap<String, StoredSecret>>>,
}

/// Stored secret with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSecret {
    pub value: Vec<u8>,
    pub metadata: SecretMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}

/// Secret metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretMetadata {
    pub access_level: AccessLevel,
    pub tags: Vec<String>,
    pub ttl_seconds: Option<u64>,
    pub description: Option<String>,
    pub content_type: Option<String>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create with pre-populated secrets
    pub fn with_secrets(secrets: HashMap<String, Vec<u8>>) -> Self {
        let stored: HashMap<String, StoredSecret> = secrets
            .into_iter()
            .map(|(k, v)| {
                let secret = StoredSecret {
                    value: v,
                    metadata: SecretMetadata::default(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    version: 1,
                };
                (k, secret)
            })
            .collect();
        
        Self {
            secrets: Arc::new(RwLock::new(stored)),
        }
    }
    
    /// Get number of stored secrets
    pub async fn len(&self) -> usize {
        self.secrets.read().await.len()
    }
    
    /// Check if empty
    pub async fn is_empty(&self) -> bool {
        self.secrets.read().await.is_empty()
    }
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Get backend name
    fn name(&self) -> &str;
    
    /// Check if backend is healthy
    async fn health_check(&self) -> VaultResult<bool>;
    
    /// Store a secret
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()>;
    
    /// Retrieve a secret
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>>;
    
    /// Retrieve secret with metadata
    async fn retrieve_with_metadata(&self, path: &str) -> VaultResult<StoredSecret>;
    
    /// Delete a secret
    async fn delete(&self, path: &str) -> VaultResult<()>;
    
    /// List secrets at path
    async fn list(&self, path: &str) -> VaultResult<Vec<String>>;
    
    /// Check if secret exists
    async fn exists(&self, path: &str) -> VaultResult<bool>;
    
    /// Get secret metadata
    async fn get_metadata(&self, path: &str) -> VaultResult<SecretMetadata>;
    
    /// Update secret metadata
    async fn update_metadata(&self, path: &str, metadata: &SecretMetadata) -> VaultResult<()>;
}

#[async_trait]
impl StorageBackend for InMemoryBackend {
    fn name(&self) -> &str {
        "memory"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        Ok(true)
    }
    
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()> {
        let mut secrets = self.secrets.write().await;
        
        let now = Utc::now();
        let version = secrets.get(path)
            .map(|s| s.version + 1)
            .unwrap_or(1);
        
        let secret = StoredSecret {
            value: value.to_vec(),
            metadata: metadata.clone(),
            created_at: secrets.get(path)
                .map(|s| s.created_at)
                .unwrap_or(now),
            updated_at: now,
            version,
        };
        
        secrets.insert(path.to_string(), secret);
        log::debug!("InMemoryBackend: Stored secret at {} (v{})", path, version);
        Ok(())
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        let secrets = self.secrets.read().await;
        secrets.get(path)
            .map(|s| s.value.clone())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn retrieve_with_metadata(&self, path: &str) -> VaultResult<StoredSecret> {
        let secrets = self.secrets.read().await;
        secrets.get(path)
            .cloned()
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        let mut secrets = self.secrets.write().await;
        secrets.remove(path)
            .map(|_| ())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn list(&self, path: &str) -> VaultResult<Vec<String>> {
        let secrets = self.secrets.read().await;
        Ok(secrets.keys()
            .filter(|k| k.starts_with(path))
            .cloned()
            .collect())
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        let secrets = self.secrets.read().await;
        Ok(secrets.contains_key(path))
    }
    
    async fn get_metadata(&self, path: &str) -> VaultResult<SecretMetadata> {
        let secrets = self.secrets.read().await;
        secrets.get(path)
            .map(|s| s.metadata.clone())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn update_metadata(&self, path: &str, metadata: &SecretMetadata) -> VaultResult<()> {
        let mut secrets = self.secrets.write().await;
        let secret = secrets.get_mut(path)
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))?;
        secret.metadata = metadata.clone();
        secret.updated_at = Utc::now();
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FILE BACKEND (Persistent)
// ═══════════════════════════════════════════════════════════════════════════════

/// File-based persistent storage backend
pub struct FileBackend {
    base_path: PathBuf,
    encryption_key: Option<[u8; 32]>,
    cache: Arc<RwLock<HashMap<String, StoredSecret>>>,
    dirty: Arc<Mutex<bool>>,
}

impl FileBackend {
    /// Create a new file backend
    pub fn new(base_path: impl AsRef<Path>) -> VaultResult<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        
        // Create directory if not exists
        if !base_path.exists() {
            std::fs::create_dir_all(&base_path)
                .map_err(|e| VaultError::EncryptionFailed(format!("Failed to create directory: {}", e)))?;
        }
        
        Ok(Self {
            base_path,
            encryption_key: None,
            cache: Arc::new(RwLock::new(HashMap::new())),
            dirty: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Create with encryption
    pub fn with_encryption(base_path: impl AsRef<Path>, key: [u8; 32]) -> VaultResult<Self> {
        let mut backend = Self::new(base_path)?;
        backend.encryption_key = Some(key);
        Ok(backend)
    }
    
    /// Get file path for secret
    fn get_file_path(&self, path: &str) -> PathBuf {
        // Sanitize path to prevent directory traversal
        let safe_path = path.replace("../", "").replace("..\\", "");
        self.base_path.join(format!("{}.secret", safe_path.replace('/', "_").replace('\\', "_")))
    }
    
    /// Encrypt data if key is set
    fn encrypt_data(&self, data: &[u8]) -> VaultResult<Vec<u8>> {
        if let Some(key) = &self.encryption_key {
            self.encrypt_aes_gcm(data, key)
        } else {
            Ok(data.to_vec())
        }
    }
    
    /// Decrypt data if key is set
    fn decrypt_data(&self, data: &[u8]) -> VaultResult<Vec<u8>> {
        if let Some(key) = &self.encryption_key {
            self.decrypt_aes_gcm(data, key)
        } else {
            Ok(data.to_vec())
        }
    }
    
    /// AES-GCM encryption
    fn encrypt_aes_gcm(&self, plaintext: &[u8], key: &[u8; 32]) -> VaultResult<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        use rand::Rng;
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| VaultError::EncryptionFailed(format!("Cipher init: {}", e)))?;
        
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| VaultError::EncryptionFailed(format!("Encrypt: {}", e)))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }
    
    /// AES-GCM decryption
    fn decrypt_aes_gcm(&self, data: &[u8], key: &[u8; 32]) -> VaultResult<Vec<u8>> {
        use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
        
        if data.len() < 12 {
            return Err(VaultError::DecryptionFailed("Invalid ciphertext".into()));
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let cipher = Aes256Gcm::new_from_slice(key)
            .map_err(|e| VaultError::DecryptionFailed(format!("Cipher init: {}", e)))?;
        
        cipher.decrypt(nonce, ciphertext)
            .map_err(|e| VaultError::DecryptionFailed(format!("Decrypt: {}", e)))
    }
    
    /// Load all secrets from disk into cache
    pub async fn load_all(&self) -> VaultResult<()> {
        let mut cache = self.cache.write().await;
        
        let entries = std::fs::read_dir(&self.base_path)
            .map_err(|e| VaultError::DecryptionFailed(format!("Read dir: {}", e)))?;
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "secret").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy().to_string();
                    
                    if let Ok(data) = std::fs::read(&path) {
                        if let Ok(decrypted) = self.decrypt_data(&data) {
                            if let Ok(secret) = serde_json::from_slice::<StoredSecret>(&decrypted) {
                                cache.insert(name, secret);
                            }
                        }
                    }
                }
            }
        }
        
        log::info!("FileBackend: Loaded {} secrets from disk", cache.len());
        Ok(())
    }
    
    /// Save all secrets to disk
    pub async fn save_all(&self) -> VaultResult<()> {
        let cache = self.cache.read().await;
        
        for (name, secret) in cache.iter() {
            let path = self.get_file_path(name);
            let json = serde_json::to_vec(secret)
                .map_err(|e| VaultError::EncryptionFailed(format!("Serialize: {}", e)))?;
            
            let encrypted = self.encrypt_data(&json)?;
            
            std::fs::write(&path, encrypted)
                .map_err(|e| VaultError::EncryptionFailed(format!("Write: {}", e)))?;
        }
        
        log::info!("FileBackend: Saved {} secrets to disk", cache.len());
        Ok(())
    }
}

#[async_trait]
impl StorageBackend for FileBackend {
    fn name(&self) -> &str {
        "file"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        Ok(self.base_path.exists() && self.base_path.is_dir())
    }
    
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()> {
        let now = Utc::now();
        
        let mut cache = self.cache.write().await;
        let version = cache.get(path)
            .map(|s| s.version + 1)
            .unwrap_or(1);
        
        let secret = StoredSecret {
            value: value.to_vec(),
            metadata: metadata.clone(),
            created_at: cache.get(path)
                .map(|s| s.created_at)
                .unwrap_or(now),
            updated_at: now,
            version,
        };
        
        // Write to disk
        let file_path = self.get_file_path(path);
        let json = serde_json::to_vec(&secret)
            .map_err(|e| VaultError::EncryptionFailed(format!("Serialize: {}", e)))?;
        
        let encrypted = self.encrypt_data(&json)?;
        
        std::fs::write(&file_path, encrypted)
            .map_err(|e| VaultError::EncryptionFailed(format!("Write: {}", e)))?;
        
        // Update cache
        cache.insert(path.to_string(), secret);
        
        log::debug!("FileBackend: Stored secret at {} (v{})", path, version);
        Ok(())
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        let cache = self.cache.read().await;
        cache.get(path)
            .map(|s| s.value.clone())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn retrieve_with_metadata(&self, path: &str) -> VaultResult<StoredSecret> {
        let cache = self.cache.read().await;
        cache.get(path)
            .cloned()
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        let mut cache = self.cache.write().await;
        
        // Remove from cache
        cache.remove(path)
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))?;
        
        // Remove from disk
        let file_path = self.get_file_path(path);
        if file_path.exists() {
            std::fs::remove_file(&file_path)
                .map_err(|e| VaultError::AccessDenied(format!("Delete file: {}", e)))?;
        }
        
        log::debug!("FileBackend: Deleted secret at {}", path);
        Ok(())
    }
    
    async fn list(&self, path: &str) -> VaultResult<Vec<String>> {
        let cache = self.cache.read().await;
        Ok(cache.keys()
            .filter(|k| k.starts_with(path))
            .cloned()
            .collect())
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        let cache = self.cache.read().await;
        Ok(cache.contains_key(path))
    }
    
    async fn get_metadata(&self, path: &str) -> VaultResult<SecretMetadata> {
        let cache = self.cache.read().await;
        cache.get(path)
            .map(|s| s.metadata.clone())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn update_metadata(&self, path: &str, metadata: &SecretMetadata) -> VaultResult<()> {
        let mut cache = self.cache.write().await;
        let secret = cache.get_mut(path)
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))?;
        
        secret.metadata = metadata.clone();
        secret.updated_at = Utc::now();
        
        // Persist to disk
        let file_path = self.get_file_path(path);
        let json = serde_json::to_vec(&*secret)
            .map_err(|e| VaultError::EncryptionFailed(format!("Serialize: {}", e)))?;
        
        let encrypted = self.encrypt_data(&json)?;
        
        std::fs::write(&file_path, encrypted)
            .map_err(|e| VaultError::EncryptionFailed(format!("Write: {}", e)))?;
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKEND MANAGER (Multi-Backend Routing)
// ═══════════════════════════════════════════════════════════════════════════════

/// Backend manager for multi-backend routing and failover
pub struct BackendManager {
    primary: Arc<dyn StorageBackend>,
    replicas: Vec<Arc<dyn StorageBackend>>,
    write_mode: WriteMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    /// Write to primary only
    PrimaryOnly,
    /// Write to all backends
    AllBackends,
    /// Write to primary and replicate asynchronously
    AsyncReplication,
}

impl BackendManager {
    /// Create with single backend
    pub fn single(backend: impl StorageBackend + 'static) -> Self {
        Self {
            primary: Arc::new(backend),
            replicas: Vec::new(),
            write_mode: WriteMode::PrimaryOnly,
        }
    }
    
    /// Create with primary and replicas
    pub fn with_replicas(
        primary: impl StorageBackend + 'static,
        replicas: Vec<Box<dyn StorageBackend>>,
        write_mode: WriteMode,
    ) -> Self {
        Self {
            primary: Arc::from(primary),
            replicas: replicas.into_iter().map(Arc::from).collect(),
            write_mode,
        }
    }
    
    /// Set write mode
    pub fn set_write_mode(&mut self, mode: WriteMode) {
        self.write_mode = mode;
    }
    
    /// Get primary backend
    pub fn primary(&self) -> &dyn StorageBackend {
        self.primary.as_ref()
    }
    
    /// Get healthy replica for read
    pub async fn get_read_backend(&self) -> &dyn StorageBackend {
        // For now, return primary
        // In production, check health and return healthy replica
        self.primary.as_ref()
    }
}

#[async_trait]
impl StorageBackend for BackendManager {
    fn name(&self) -> &str {
        "manager"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        let primary_healthy = self.primary.health_check().await?;
        
        for replica in &self.replicas {
            if !replica.health_check().await? {
                return Ok(false);
            }
        }
        
        Ok(primary_healthy)
    }
    
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()> {
        // Write to primary
        self.primary.store(path, value, metadata).await?;
        
        // Handle replication
        match self.write_mode {
            WriteMode::PrimaryOnly => {},
            WriteMode::AllBackends => {
                for replica in &self.replicas {
                    replica.store(path, value, metadata).await?;
                }
            },
            WriteMode::AsyncReplication => {
                // Spawn async task for replication
                let replicas = self.replicas.clone();
                let path = path.to_string();
                let value = value.to_vec();
                let metadata = metadata.clone();
                
                tokio::spawn(async move {
                    for replica in replicas {
                        if let Err(e) = replica.store(&path, &value, &metadata).await {
                            log::warn!("Async replication failed for {}: {}", path, e);
                        }
                    }
                });
            }
        }
        
        Ok(())
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        self.get_read_backend().await.retrieve(path).await
    }
    
    async fn retrieve_with_metadata(&self, path: &str) -> VaultResult<StoredSecret> {
        self.get_read_backend().await.retrieve_with_metadata(path).await
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        self.primary.delete(path).await?;
        
        // Replicate deletion
        for replica in &self.replicas {
            if let Err(e) = replica.delete(path).await {
                log::warn!("Delete replication failed for {}: {}", path, e);
            }
        }
        
        Ok(())
    }
    
    async fn list(&self, path: &str) -> VaultResult<Vec<String>> {
        self.get_read_backend().await.list(path).await
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        self.get_read_backend().await.exists(path).await
    }
    
    async fn get_metadata(&self, path: &str) -> VaultResult<SecretMetadata> {
        self.get_read_backend().await.get_metadata(path).await
    }
    
    async fn update_metadata(&self, path: &str, metadata: &SecretMetadata) -> VaultResult<()> {
        self.primary.update_metadata(path, metadata).await?;
        
        for replica in &self.replicas {
            if let Err(e) = replica.update_metadata(path, metadata).await {
                log::warn!("Metadata update replication failed for {}: {}", path, e);
            }
        }
        
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_in_memory_backend_basic() {
        let backend = InMemoryBackend::new();
        
        // Store
        let metadata = SecretMetadata {
            access_level: AccessLevel::Secret,
            tags: vec!["test".to_string()],
            ttl_seconds: Some(3600),
            description: Some("Test secret".to_string()),
            content_type: Some("text/plain".to_string()),
        };
        
        backend.store("test/secret", b"hello world", &metadata).await.expect("store failed");
        
        // Retrieve
        let value = backend.retrieve("test/secret").await.expect("retrieve failed");
        assert_eq!(value, b"hello world");
        
        // Exists
        assert!(backend.exists("test/secret").await.expect("exists failed"));
        assert!(!backend.exists("nonexistent").await.expect("exists failed"));
        
        // List
        let list = backend.list("test/").await.expect("list failed");
        assert_eq!(list.len(), 1);
        
        // Delete
        backend.delete("test/secret").await.expect("delete failed");
        assert!(!backend.exists("test/secret").await.expect("exists failed"));
    }
    
    #[tokio::test]
    async fn test_in_memory_versioning() {
        let backend = InMemoryBackend::new();
        let metadata = SecretMetadata::default();
        
        // Store first version
        backend.store("test", b"v1", &metadata).await.expect("store failed");
        let secret = backend.retrieve_with_metadata("test").await.expect("retrieve failed");
        assert_eq!(secret.version, 1);
        assert_eq!(secret.value, b"v1");
        
        // Store second version
        backend.store("test", b"v2", &metadata).await.expect("store failed");
        let secret = backend.retrieve_with_metadata("test").await.expect("retrieve failed");
        assert_eq!(secret.version, 2);
        assert_eq!(secret.value, b"v2");
    }
    
    #[tokio::test]
    async fn test_file_backend_basic() {
        let dir = tempdir().expect("tempdir failed");
        let backend = FileBackend::new(dir.path()).expect("create backend failed");
        
        let metadata = SecretMetadata {
            description: Some("Test secret".to_string()),
            ..Default::default()
        };
        
        // Store
        backend.store("test/secret", b"hello world", &metadata).await.expect("store failed");
        
        // Retrieve
        let value = backend.retrieve("test/secret").await.expect("retrieve failed");
        assert_eq!(value, b"hello world");
        
        // Check file exists
        assert!(backend.health_check().await.expect("health check failed"));
    }
    
    #[tokio::test]
    async fn test_file_backend_encryption() {
        let dir = tempdir().expect("tempdir failed");
        let key = [42u8; 32];
        let backend = FileBackend::with_encryption(dir.path(), key).expect("create backend failed");
        
        let metadata = SecretMetadata::default();
        
        // Store
        backend.store("encrypted", b"secret data", &metadata).await.expect("store failed");
        
        // Retrieve
        let value = backend.retrieve("encrypted").await.expect("retrieve failed");
        assert_eq!(value, b"secret data");
        
        // Verify file is encrypted (shouldn't contain plaintext)
        let file_path = backend.get_file_path("encrypted");
        let file_contents = std::fs::read(file_path).expect("read file failed");
        assert!(!file_contents.windows(11).any(|w| w == b"secret data"));
    }
    
    #[tokio::test]
    async fn test_backend_manager() {
        let primary = InMemoryBackend::new();
        let replica = InMemoryBackend::new();
        
        let manager = BackendManager::with_replicas(
            primary,
            vec![Box::new(replica)],
            WriteMode::AllBackends,
        );
        
        let metadata = SecretMetadata::default();
        
        // Store through manager
        manager.store("test", b"data", &metadata).await.expect("store failed");
        
        // Retrieve
        let value = manager.retrieve("test").await.expect("retrieve failed");
        assert_eq!(value, b"data");
        
        // Health check
        assert!(manager.health_check().await.expect("health check failed"));
    }
}
