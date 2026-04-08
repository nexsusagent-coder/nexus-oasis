//! ─── RECOVERY ───
//!
//! Kurtarma noktası yönetimi

use crate::{Ratchet, RatchetError, RatchetResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Kurtarma noktası
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPoint {
    /// Benzersiz ID
    pub id: String,
    /// Ratchet ID
    pub ratchet_id: Uuid,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Adım numarası
    pub step_number: u64,
    /// Hash
    pub hash: String,
    /// Boyut
    pub size: u64,
}

/// Kurtarma yöneticisi
pub struct RecoveryManager {
    base_path: PathBuf,
}

impl RecoveryManager {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
    
    /// Ratchet'i kaydet
    pub async fn save(&self, ratchet: &Ratchet) -> RatchetResult<String> {
        std::fs::create_dir_all(&self.base_path)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        
        let content = serde_json::to_string_pretty(ratchet)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        
        let checkpoint_id = format!("{}_{}", ratchet.id, ratchet.state.step_count);
        let path = self.checkpoint_path(&checkpoint_id);
        
        std::fs::write(&path, content)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        
        Ok(checkpoint_id)
    }
    
    /// Ratchet'i yükle
    pub async fn load(&self, checkpoint_id: &str) -> RatchetResult<Ratchet> {
        let path = self.checkpoint_path(checkpoint_id);
        
        let content = std::fs::read_to_string(&path)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        
        let ratchet: Ratchet = serde_json::from_str(&content)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        
        Ok(ratchet)
    }
    
    /// Mevcut kurtarma noktalarını listele
    pub fn list(&self) -> RatchetResult<Vec<RecoveryPoint>> {
        let mut points = Vec::new();
        
        if !self.base_path.exists() {
            return Ok(points);
        }
        
        for entry in std::fs::read_dir(&self.base_path)
            .map_err(|e| RatchetError::RecoveryError(e.to_string()))?
        {
            let entry = entry.map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
            let path = entry.path();
            
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy();
                    let parts: Vec<&str> = name.split('_').collect();
                    
                    if parts.len() == 2 {
                        if let (Ok(ratchet_id), Ok(step_number)) = 
                            (Uuid::parse_str(parts[0]), parts[1].parse::<u64>()) 
                        {
                            let metadata = std::fs::metadata(&path)
                                .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
                            
                            points.push(RecoveryPoint {
                                id: name.to_string(),
                                ratchet_id,
                                timestamp: chrono::Utc::now(),
                                step_number,
                                hash: String::new(),
                                size: metadata.len(),
                            });
                        }
                    }
                }
            }
        }
        
        // Tarihe göre sırala
        points.sort_by(|a, b| b.step_number.cmp(&a.step_number));
        
        Ok(points)
    }
    
    /// Kurtarma noktası sil
    pub fn delete(&self, checkpoint_id: &str) -> RatchetResult<()> {
        let path = self.checkpoint_path(checkpoint_id);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| RatchetError::RecoveryError(e.to_string()))?;
        }
        Ok(())
    }
    
    fn checkpoint_path(&self, checkpoint_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", checkpoint_id))
    }
}

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new("data/ratchet")
    }
}
