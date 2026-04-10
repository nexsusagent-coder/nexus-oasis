//! Storage backends

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::Result;

/// Storage backend trait
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store backup
    async fn store(&self, name: &str, data: &[u8]) -> Result<()>;
    
    /// Retrieve backup
    async fn retrieve(&self, name: &str) -> Result<Vec<u8>>;
    
    /// Delete backup
    async fn delete(&self, name: &str) -> Result<()>;
    
    /// List backups
    async fn list(&self) -> Result<Vec<BackupInfo>>;
    
    /// Check if backup exists
    async fn exists(&self, name: &str) -> Result<bool>;
}

/// Backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub name: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
}

/// Local filesystem storage
#[cfg(feature = "local")]
pub struct LocalStorage {
    base_path: PathBuf,
}

#[cfg(feature = "local")]
impl LocalStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[cfg(feature = "local")]
#[async_trait]
impl StorageBackend for LocalStorage {
    async fn store(&self, name: &str, data: &[u8]) -> Result<()> {
        let path = self.base_path.join(name);
        tokio::fs::write(&path, data).await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        Ok(())
    }

    async fn retrieve(&self, name: &str) -> Result<Vec<u8>> {
        let path = self.base_path.join(name);
        tokio::fs::read(&path).await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let path = self.base_path.join(name);
        tokio::fs::remove_file(&path).await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<BackupInfo>> {
        let mut entries = tokio::fs::read_dir(&self.base_path).await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        
        let mut backups = Vec::new();
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))? 
        {
            let metadata = entry.metadata().await
                .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
            
            if metadata.is_file() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".tar.gz") {
                    let created_at = metadata.modified()
                        .ok()
                        .and_then(|t| {
                            let duration = t.duration_since(std::time::UNIX_EPOCH).ok()?;
                            chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                        })
                        .unwrap_or_else(Utc::now);
                    
                    backups.push(BackupInfo {
                        name,
                        size: metadata.len(),
                        created_at,
                        checksum: String::new(), // Would need to calculate
                    });
                }
            }
        }
        
        Ok(backups)
    }

    async fn exists(&self, name: &str) -> Result<bool> {
        let path = self.base_path.join(name);
        Ok(path.exists())
    }
}

/// S3 storage backend
#[cfg(feature = "s3")]
pub struct S3Storage {
    bucket: String,
    prefix: String,
    client: aws_sdk_s3::Client,
}

#[cfg(feature = "s3")]
impl S3Storage {
    pub async fn new(bucket: String, prefix: String, region: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);
        
        Self { bucket, prefix, client }
    }
}

#[cfg(feature = "s3")]
#[async_trait]
impl StorageBackend for S3Storage {
    async fn store(&self, name: &str, data: &[u8]) -> Result<()> {
        let key = format!("{}/{}", self.prefix, name);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(data.to_vec().into())
            .send()
            .await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        Ok(())
    }

    async fn retrieve(&self, name: &str) -> Result<Vec<u8>> {
        let key = format!("{}/{}", self.prefix, name);
        let output = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        
        let data = output.body.collect().await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?
            .into_bytes()
            .to_vec();
        
        Ok(data)
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let key = format!("{}/{}", self.prefix, name);
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<BackupInfo>> {
        let prefix = format!("{}/", self.prefix);
        let output = self.client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&prefix)
            .send()
            .await
            .map_err(|e| crate::BackupError::StorageError(e.to_string()))?;
        
        let backups = output.contents()
            .unwrap_or_default()
            .iter()
            .filter_map(|obj| {
                Some(BackupInfo {
                    name: obj.key()?.strip_prefix(&prefix)?.to_string(),
                    size: obj.size()? as u64,
                    created_at: chrono::DateTime::from(obj.last_modified()?.to_millis()? as i64),
                    checksum: obj.e_tag()?.to_string(),
                })
            })
            .collect();
        
        Ok(backups)
    }

    async fn exists(&self, name: &str) -> Result<bool> {
        let key = format!("{}/{}", self.prefix, name);
        let result = self.client
            .head_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await;
        
        Ok(result.is_ok())
    }
}
