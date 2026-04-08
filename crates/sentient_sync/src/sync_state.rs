//! Sync State - Durum yönetimi

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use parking_lot::RwLock;
use std::sync::Arc;

/// Sync durumu
#[derive(Debug, Clone, Default)]
pub struct SyncState {
    inner: Arc<RwLock<SyncStateInner>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SyncStateInner {
    /// Repo bazlı durumlar
    repos: HashMap<String, RepoState>,
    
    /// Son global sync
    last_global_sync: Option<DateTime<Utc>>,
    
    /// Toplam güncelleme sayısı
    total_updates: u64,
    
    /// Başarısız güncelleme sayısı
    failed_updates: u64,
    
    /// Başlangıç zamanı
    started_at: Option<DateTime<Utc>>,
}

/// Repo durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoState {
    /// Son commit
    pub last_commit: String,
    
    /// Son güncelleme
    pub last_update: DateTime<Utc>,
    
    /// Toplam güncelleme sayısı
    pub update_count: u64,
    
    /// Son hata (varsa)
    pub last_error: Option<String>,
    
    /// Son hata zamanı
    pub last_error_time: Option<DateTime<Utc>>,
}

impl SyncState {
    /// State yükle veya oluştur
    pub async fn load_or_create(_path: &Path) -> Result<Self, crate::SyncError> {
        // Şimdilik in-memory state kullan
        // İleride SQLite backend eklenecek
        
        let inner = SyncStateInner {
            started_at: Some(Utc::now()),
            ..Default::default()
        };
        
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
    }
    
    /// Repo güncellemesini kaydet
    pub fn record_update(&mut self, repo_name: &str, commit: &str) {
        let mut inner = self.inner.write();
        
        let state = inner.repos.entry(repo_name.to_string()).or_insert(RepoState {
            last_commit: commit.to_string(),
            last_update: Utc::now(),
            update_count: 0,
            last_error: None,
            last_error_time: None,
        });
        
        state.last_commit = commit.to_string();
        state.last_update = Utc::now();
        state.update_count += 1;
        
        inner.total_updates += 1;
        inner.last_global_sync = Some(Utc::now());
    }
    
    /// Hata kaydet
    pub fn record_error(&mut self, repo_name: &str, error: &str) {
        let mut inner = self.inner.write();
        
        let state = inner.repos.entry(repo_name.to_string()).or_insert(RepoState {
            last_commit: String::new(),
            last_update: Utc::now(),
            update_count: 0,
            last_error: None,
            last_error_time: None,
        });
        
        state.last_error = Some(error.to_string());
        state.last_error_time = Some(Utc::now());
        
        inner.failed_updates += 1;
    }
    
    /// Son sync zamanını al
    pub fn get_last_sync(&self, repo_name: &str) -> Option<DateTime<Utc>> {
        let inner = self.inner.read();
        inner.repos.get(repo_name).map(|s| s.last_update)
    }
    
    /// Repo durumunu al
    pub fn get_repo_state(&self, repo_name: &str) -> Option<RepoState> {
        let inner = self.inner.read();
        inner.repos.get(repo_name).cloned()
    }
    
    /// Tüm repo durumlarını al
    pub fn get_all_states(&self) -> HashMap<String, RepoState> {
        self.inner.read().repos.clone()
    }
    
    /// İstatistikler
    pub fn stats(&self) -> SyncStats {
        let inner = self.inner.read();
        
        SyncStats {
            total_repos: inner.repos.len(),
            total_updates: inner.total_updates,
            failed_updates: inner.failed_updates,
            last_global_sync: inner.last_global_sync,
            uptime: inner.started_at.map(|s| Utc::now() - s),
        }
    }
}

/// Sync istatistikleri
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub total_repos: usize,
    pub total_updates: u64,
    pub failed_updates: u64,
    pub last_global_sync: Option<DateTime<Utc>>,
    pub uptime: Option<chrono::Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_update() {
        // Test implementation
    }
}
