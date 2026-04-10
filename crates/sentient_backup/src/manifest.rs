//! Backup manifest

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::BackupType;

/// Backup manifest - metadata about a backup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Unique backup ID
    pub id: Uuid,
    /// Backup name
    pub name: String,
    /// Backup type
    pub backup_type: BackupType,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Completion timestamp
    pub completed_at: DateTime<Utc>,
    /// SHA256 checksum
    pub checksum: String,
    /// Total size in bytes
    pub size_bytes: u64,
    /// Number of files
    pub files_count: u64,
    /// Source paths
    pub sources: Vec<String>,
    /// Parent backup ID (for incremental)
    #[serde(default)]
    pub parent_id: Option<Uuid>,
    /// Custom tags
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Backup metadata for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Backup ID
    pub id: Uuid,
    /// Backup name
    pub name: String,
    /// Backup type
    pub backup_type: BackupType,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Size in bytes
    pub size: u64,
    /// Checksum
    pub checksum: String,
    /// Is encrypted
    pub encrypted: bool,
    /// Is compressed
    pub compressed: bool,
}

impl From<BackupManifest> for BackupMetadata {
    fn from(manifest: BackupManifest) -> Self {
        Self {
            id: manifest.id,
            name: manifest.name,
            backup_type: manifest.backup_type,
            created_at: manifest.created_at,
            size: manifest.size_bytes,
            checksum: manifest.checksum,
            encrypted: false,
            compressed: true,
        }
    }
}

/// Backup catalog - index of all backups
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupCatalog {
    /// All known backups
    pub backups: HashMap<Uuid, BackupMetadata>,
    /// Last modified
    pub last_modified: DateTime<Utc>,
}

impl BackupCatalog {
    /// Create new catalog
    pub fn new() -> Self {
        Self {
            backups: HashMap::new(),
            last_modified: Utc::now(),
        }
    }

    /// Add backup to catalog
    pub fn add(&mut self, metadata: BackupMetadata) {
        self.backups.insert(metadata.id, metadata);
        self.last_modified = Utc::now();
    }

    /// Remove backup from catalog
    pub fn remove(&mut self, id: &Uuid) -> Option<BackupMetadata> {
        let result = self.backups.remove(id);
        self.last_modified = Utc::now();
        result
    }

    /// Get backup by ID
    pub fn get(&self, id: &Uuid) -> Option<&BackupMetadata> {
        self.backups.get(id)
    }

    /// List all backups sorted by date
    pub fn list_by_date(&self) -> Vec<&BackupMetadata> {
        let mut backups: Vec<_> = self.backups.values().collect();
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        backups
    }

    /// Get total size of all backups
    pub fn total_size(&self) -> u64 {
        self.backups.values().map(|b| b.size).sum()
    }

    /// Get backup count
    pub fn count(&self) -> usize {
        self.backups.len()
    }

    /// Find backups older than
    pub fn find_older_than(&self, days: u64) -> Vec<&BackupMetadata> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        self.backups
            .values()
            .filter(|b| b.created_at < cutoff)
            .collect()
    }

    /// Load catalog from file
    pub async fn load(path: &std::path::Path) -> crate::Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| crate::BackupError::Io(e))?;

        serde_json::from_str(&content)
            .map_err(|e| crate::BackupError::Serialization(e.to_string()))
    }

    /// Save catalog to file
    pub async fn save(&self, path: &std::path::Path) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::BackupError::Serialization(e.to_string()))?;

        tokio::fs::write(path, content)
            .await
            .map_err(|e| crate::BackupError::Io(e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_operations() {
        let mut catalog = BackupCatalog::new();
        assert_eq!(catalog.count(), 0);

        let metadata = BackupMetadata {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            backup_type: BackupType::Full,
            created_at: Utc::now(),
            size: 1024,
            checksum: "abc".to_string(),
            encrypted: false,
            compressed: true,
        };

        catalog.add(metadata.clone());
        assert_eq!(catalog.count(), 1);

        let retrieved = catalog.get(&metadata.id);
        assert!(retrieved.is_some());

        catalog.remove(&metadata.id);
        assert_eq!(catalog.count(), 0);
    }
}
