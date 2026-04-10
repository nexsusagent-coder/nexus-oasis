//! Restore operations

use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{BackupError, Result, BackupManifest};

/// Restore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreConfig {
    /// Backup file to restore
    pub backup_file: PathBuf,
    /// Destination path
    pub destination: PathBuf,
    /// Overwrite existing files
    pub overwrite: bool,
    /// Restore specific files only
    pub selective_files: Option<Vec<PathBuf>>,
    /// Verify checksums
    pub verify: bool,
    /// Dry run (no actual restore)
    pub dry_run: bool,
}

impl Default for RestoreConfig {
    fn default() -> Self {
        Self {
            backup_file: PathBuf::from("./backups/backup.tar.gz"),
            destination: PathBuf::from("./restored"),
            overwrite: false,
            selective_files: None,
            verify: true,
            dry_run: false,
        }
    }
}

/// Restore status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RestoreStatus {
    Pending,
    InProgress,
    Verifying,
    Completed,
    Failed(String),
    Cancelled,
}

/// Restore operation handle
#[derive(Debug, Clone)]
pub struct RestoreHandle {
    pub id: uuid::Uuid,
    pub backup_name: String,
    pub status: RestoreStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub bytes_restored: u64,
    pub files_restored: u64,
    pub verification_passed: bool,
}

/// Restore engine
pub struct RestoreEngine {
    config: RestoreConfig,
    cancel_token: tokio_util::sync::CancellationToken,
}

impl RestoreEngine {
    /// Create new restore engine
    pub fn new(config: RestoreConfig) -> Self {
        Self {
            config,
            cancel_token: tokio_util::sync::CancellationToken::new(),
        }
    }

    /// Execute restore operation
    pub async fn execute(&self) -> Result<RestoreHandle> {
        let id = uuid::Uuid::new_v4();
        let started_at = Utc::now();

        tracing::info!(
            restore_id = %id,
            backup_file = %self.config.backup_file.display(),
            "Starting restore operation"
        );

        // Verify backup exists
        if !self.config.backup_file.exists() {
            return Err(BackupError::NotFound(
                self.config.backup_file.display().to_string()
            ));
        }

        // Load manifest
        let manifest = self.load_manifest().await?;
        let backup_name = manifest.name.clone();

        let mut handle = RestoreHandle {
            id,
            backup_name,
            status: RestoreStatus::InProgress,
            started_at,
            completed_at: None,
            bytes_restored: 0,
            files_restored: 0,
            verification_passed: false,
        };

        // Verify checksum
        if self.config.verify {
            handle.status = RestoreStatus::Verifying;
            match self.verify_checksum(&manifest).await {
                Ok(valid) => {
                    if !valid {
                        handle.status = RestoreStatus::Failed("Checksum verification failed".to_string());
                        return Ok(handle);
                    }
                }
                Err(e) => {
                    handle.status = RestoreStatus::Failed(format!("Verification error: {}", e));
                    return Ok(handle);
                }
            }
        }

        // Create destination
        fs::create_dir_all(&self.config.destination)
            .await
            .map_err(BackupError::Io)?;

        // Extract archive
        if !self.config.dry_run {
            match self.extract_archive().await {
                Ok((files, bytes)) => {
                    handle.status = RestoreStatus::Completed;
                    handle.completed_at = Some(Utc::now());
                    handle.bytes_restored = bytes;
                    handle.files_restored = files;
                    handle.verification_passed = true;

                    tracing::info!(
                        restore_id = %id,
                        files = files,
                        bytes = bytes,
                        "Restore completed successfully"
                    );
                }
                Err(e) => {
                    handle.status = RestoreStatus::Failed(e.to_string());
                    handle.completed_at = Some(Utc::now());
                }
            }
        } else {
            handle.status = RestoreStatus::Completed;
            handle.completed_at = Some(Utc::now());
            tracing::info!(restore_id = %id, "Dry run completed");
        }

        Ok(handle)
    }

    /// Cancel restore operation
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    /// Load backup manifest
    async fn load_manifest(&self) -> Result<BackupManifest> {
        let manifest_path = self.config.backup_file.with_extension("manifest.json");
        
        if !manifest_path.exists() {
            return Err(BackupError::NotFound(
                format!("Manifest not found: {}", manifest_path.display())
            ));
        }

        let content = fs::read_to_string(&manifest_path)
            .await
            .map_err(BackupError::Io)?;

        serde_json::from_str(&content)
            .map_err(|e| BackupError::Serialization(e.to_string()))
    }

    /// Verify backup checksum
    async fn verify_checksum(&self, manifest: &BackupManifest) -> Result<bool> {
        let data = fs::read(&self.config.backup_file).await.map_err(BackupError::Io)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hex::encode(hasher.finalize());

        if hash != manifest.checksum {
            tracing::error!(
                expected = %manifest.checksum,
                actual = %hash,
                "Checksum mismatch"
            );
            return Ok(false);
        }

        Ok(true)
    }

    /// Extract backup archive
    async fn extract_archive(&self) -> Result<(u64, u64)> {
        use flate2::read::GzDecoder;
        use std::fs::File;
        use tar::Archive;

        let file = File::open(&self.config.backup_file)
            .map_err(|e| BackupError::Io(e))?;

        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        let mut files_count = 0u64;
        let mut bytes_count = 0u64;

        for entry in archive.entries().map_err(BackupError::Io)? {
            if self.cancel_token.is_cancelled() {
                return Err(BackupError::RestoreFailed("Restore cancelled".to_string()));
            }

            let mut entry = entry.map_err(BackupError::Io)?;
            
            // Check selective files
            if let Some(ref selective) = self.config.selective_files {
                let path = entry.path().map_err(BackupError::Io)?;
                if !selective.iter().any(|p| path.starts_with(p)) {
                    continue;
                }
            }

            let size = entry.size();
            entry.unpack_in(&self.config.destination).map_err(BackupError::Io)?;
            
            files_count += 1;
            bytes_count += size;
        }

        Ok((files_count, bytes_count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_config_default() {
        let config = RestoreConfig::default();
        assert!(!config.overwrite);
        assert!(config.verify);
        assert!(!config.dry_run);
    }
}
