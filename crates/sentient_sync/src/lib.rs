//! SENTIENT SYNC - Silent Auto-Update Engine
//! 
//! Bu motor sistem içinde entegre edilen GitHub repolarını takip eder,
//! güncellemeleri sessizce çeker ve kullanıcıya hissettirmeden entegre eder.

pub mod config;
pub mod tracker;
pub mod updater;
pub mod diff;
pub mod sync_state;
pub mod webhook;
pub mod scheduler;

pub use config::SyncConfig;
pub use tracker::RepoTracker;
pub use updater::SilentUpdater;
pub use sync_state::SyncState;

/// Sentinel Sync Engine - Ana motor
pub struct SyncEngine {
    config: SyncConfig,
    tracker: RepoTracker,
    updater: SilentUpdater,
    state: SyncState,
}

impl SyncEngine {
    /// Yeni sync motoru oluştur
    pub async fn new(config: SyncConfig) -> Result<Self, SyncError> {
        let state = SyncState::default();
        let tracker = RepoTracker::new(&config)?;
        let updater = SilentUpdater::new(&config)?;
        
        Ok(Self {
            config,
            tracker,
            updater,
            state,
        })
    }
    
    /// Motoru başlat (arka planda çalışır)
    pub async fn start(&self) -> Result<(), SyncError> {
        // Scheduler'ı başlat
        let scheduler = scheduler::SyncScheduler::new(
            self.config.clone(),
            RepoTracker::new(&self.config)?,
            SilentUpdater::new(&self.config)?,
            self.state.clone(),
        );
        
        scheduler.run().await
    }
    
    /// Tüm repoları manuel senkronize et
    pub async fn sync_all(&self) -> Result<SyncReport, SyncError> {
        let repos = self.tracker.discover_repos().await?;
        let mut report = SyncReport::default();
        
        for repo in repos {
            match self.updater.update_repo(&repo).await {
                Ok(result) => {
                    report.updated += 1;
                    report.changes.push(result);
                }
                Err(e) => {
                    report.failed += 1;
                    report.errors.push(format!("{}: {}", repo.name, e));
                }
            }
        }
        
        Ok(report)
    }
}

/// Senkronizasyon raporu
#[derive(Debug, Default)]
pub struct SyncReport {
    pub updated: usize,
    pub failed: usize,
    pub changes: Vec<UpdateResult>,
    pub errors: Vec<String>,
}

/// Güncelleme sonucu
#[derive(Debug, Clone)]
pub struct UpdateResult {
    pub repo_name: String,
    pub old_commit: String,
    pub new_commit: String,
    pub files_changed: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Hata tipleri
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Sync conflict in {repo}: {message}")]
    Conflict { repo: String, message: String },
    
    #[error("Network error: {0}")]
    Network(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sync_engine_creation() {
        // Test implementation
    }
}
