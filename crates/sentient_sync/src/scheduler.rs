//! Scheduler - Zamanlanmış görev yönetimi

use crate::{SyncConfig, SyncError, SyncReport, tracker::RepoTracker, updater::SilentUpdater, sync_state::SyncState};
use tokio::time::interval;
use chrono::Utc;

/// Sync scheduler
pub struct SyncScheduler {
    config: SyncConfig,
    tracker: RepoTracker,
    updater: SilentUpdater,
    state: SyncState,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl SyncScheduler {
    /// Yeni scheduler oluştur
    pub fn new(
        config: SyncConfig,
        tracker: RepoTracker,
        updater: SilentUpdater,
        state: SyncState,
    ) -> Self {
        Self {
            config,
            tracker,
            updater,
            state,
            running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Scheduler'ı başlat
    pub async fn run(&self) -> Result<(), SyncError> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        tracing::info!("🔄 Sync Scheduler started (interval: {}min)", self.config.sync_interval_minutes);
        
        let mut ticker = interval(self.config.sync_interval());
        
        // İlk çalıştırmayı beklemeden yap
        self.run_sync_cycle().await?;
        
        loop {
            if !self.running.load(std::sync::atomic::Ordering::SeqCst) {
                tracing::info!("Sync Scheduler stopped");
                break;
            }
            
            ticker.tick().await;
            
            if let Err(e) = self.run_sync_cycle().await {
                tracing::error!("Sync cycle failed: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Scheduler'ı durdur
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Sync döngüsü çalıştır
    async fn run_sync_cycle(&self) -> Result<SyncReport, SyncError> {
        let start = Utc::now();
        tracing::debug!("Starting sync cycle...");
        
        // Repoları keşfet
        let repos = self.tracker.discover_repos().await?;
        tracing::debug!("Found {} repositories", repos.len());
        
        let mut report = SyncReport::default();
        
        // Her repo için güncelleme kontrolü
        for repo in repos {
            // Aktif güncelleme varsa atla
            if self.updater.is_updating(&repo.name) {
                tracing::debug!("Skipping {} - already updating", repo.name);
                continue;
            }
            
            // Güncelleme var mı kontrol et
            let mut repo = repo;
            let has_update = self.tracker.check_for_updates(&mut repo).await?;
            
            if has_update {
                tracing::info!("📦 Update available for: {}", repo.name);
                
                match self.updater.update_repo(&repo).await {
                    Ok(result) => {
                        report.updated += 1;
                        report.changes.push(result);
                    }
                    Err(e) => {
                        report.failed += 1;
                        report.errors.push(format!("{}: {}", repo.name, e));
                        tracing::error!("Failed to update {}: {}", repo.name, e);
                    }
                }
            }
        }
        
        let duration = (Utc::now() - start).num_seconds();
        tracing::info!(
            "✅ Sync cycle completed in {}s: {} updated, {} failed",
            duration, report.updated, report.failed
        );
        
        // Kritik hata varsa bildir
        if report.failed > 0 && self.config.notifications.critical_only {
            self.send_critical_notification(&report).await;
        }
        
        Ok(report)
    }
    
    /// Kritik hata bildirimi gönder
    async fn send_critical_notification(&self, report: &SyncReport) {
        // Discord webhook
        if let Some(ref webhook) = self.config.notifications.discord_webhook {
            let content = format!(
                "⚠️ **SENTIENT SYNC Alert**\n{} updates failed\n\nErrors:\n{}",
                report.failed,
                report.errors.join("\n")
            );
            
            let payload = serde_json::json!({
                "content": content
            });
            
            if let Err(e) = reqwest::Client::new()
                .post(webhook)
                .json(&payload)
                .send()
                .await
            {
                tracing::warn!("Failed to send Discord notification: {}", e);
            }
        }
        
        // Telegram bildirimi (implementasyon benzer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_scheduler() {
        // Test implementation
    }
}
