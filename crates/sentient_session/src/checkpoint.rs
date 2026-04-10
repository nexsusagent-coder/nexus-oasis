//! ─── CHECKPOINT ───
//!
//! Oturum checkpoint sistemi

use crate::{Session, SessionError, SessionResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

/// Checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Benzersiz ID
    pub id: Uuid,
    /// Oturum ID
    pub session_id: Uuid,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Checkpoint tipi
    pub checkpoint_type: CheckpointType,
    /// Hash
    pub hash: String,
    /// Boyut (byte)
    pub size: u64,
    /// Metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Checkpoint tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointType {
    /// Manuel
    Manual,
    /// Otomatik (periyodik)
    Auto,
    /// Önceki hata sonrası
    Recovery,
    /// Oturum sonu
    End,
}

/// Checkpoint yöneticisi
pub struct CheckpointManager {
    base_path: PathBuf,
}

impl CheckpointManager {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
    
    /// Checkpoint kaydet
    pub async fn save(&self, session: &Session) -> SessionResult<Checkpoint> {
        // Dizin oluştur
        std::fs::create_dir_all(&self.base_path)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        // Session'ı serialize et
        let content = serde_json::to_string_pretty(session)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        // Hash hesapla
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = hex::encode(hasher.finalize());
        
        // Checkpoint oluştur
        let checkpoint = Checkpoint {
            id: Uuid::new_v4(),
            session_id: session.id,
            timestamp: Utc::now(),
            checkpoint_type: CheckpointType::End,
            hash: hash.clone(),
            size: content.len() as u64,
            metadata: std::collections::HashMap::new(),
        };
        
        // Dosyaya yaz
        let path = self.checkpoint_path(session.id);
        std::fs::write(&path, content)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        // Checkpoint metadata yaz
        let meta_path = self.metadata_path(checkpoint.id);
        let meta_content = serde_json::to_string_pretty(&checkpoint)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        std::fs::write(&meta_path, meta_content)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        Ok(checkpoint)
    }
    
    /// Checkpoint yükle
    pub async fn load(&self, session_id: Uuid) -> SessionResult<Session> {
        let path = self.checkpoint_path(session_id);
        
        let content = std::fs::read_to_string(&path)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        let session: Session = serde_json::from_str(&content)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        
        Ok(session)
    }
    
    /// Mevcut checkpoint'ları listele
    pub fn list(&self) -> SessionResult<Vec<Checkpoint>> {
        let mut checkpoints = Vec::new();
        
        if !self.base_path.exists() {
            return Ok(checkpoints);
        }
        
        for entry in std::fs::read_dir(&self.base_path)
            .map_err(|e| SessionError::CheckpointError(e.to_string()))?
        {
            let entry = entry.map_err(|e| SessionError::CheckpointError(e.to_string()))?;
            let path = entry.path();
            
            if path.extension().map(|e| e == "meta").unwrap_or(false) {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
                
                if let Ok(checkpoint) = serde_json::from_str::<Checkpoint>(&content) {
                    checkpoints.push(checkpoint);
                }
            }
        }
        
        // Tarihe göre sırala (yeniden eskiye)
        checkpoints.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        Ok(checkpoints)
    }
    
    /// Checkpoint sil
    pub fn delete(&self, session_id: Uuid) -> SessionResult<()> {
        let path = self.checkpoint_path(session_id);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| SessionError::CheckpointError(e.to_string()))?;
        }
        Ok(())
    }
    
    /// Checkpoint var mı?
    pub fn exists(&self, session_id: Uuid) -> bool {
        self.checkpoint_path(session_id).exists()
    }
    
    fn checkpoint_path(&self, session_id: Uuid) -> PathBuf {
        self.base_path.join(format!("{}.json", session_id))
    }
    
    fn metadata_path(&self, checkpoint_id: Uuid) -> PathBuf {
        self.base_path.join(format!("{}.meta", checkpoint_id))
    }
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new("data/sessions")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::SessionConfig;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_checkpoint_save_load() {
        let dir = tempdir().expect("operation failed");
        let manager = CheckpointManager::new(dir.path());
        
        let mut session = Session::new(SessionConfig::default());
        session.start().expect("operation failed");
        
        let checkpoint = manager.save(&session).await.expect("operation failed");
        assert_eq!(checkpoint.session_id, session.id);
        
        let loaded = manager.load(session.id).await.expect("operation failed");
        assert_eq!(loaded.id, session.id);
    }
}
