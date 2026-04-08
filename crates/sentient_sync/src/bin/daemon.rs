//! SENTIENT SYNC Daemon - Arka plan servisi
//! 
//! Kullanıcı tarafından doğrudan çalıştırılmaz, sistem tarafından yönetilir

use sentient_sync::{SyncEngine, SyncConfig};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging setup
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("🚀 SENTIENT SYNC Daemon starting...");
    
    // Config yükle
    let config_path = PathBuf::from("./data/sync_config.json");
    let config = if config_path.exists() {
        SyncConfig::load(&config_path).await?
    } else {
        let config = SyncConfig::default();
        // Config dosyasını oluştur
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        config.save(&config_path).await?;
        config
    };
    
    tracing::info!("📁 Integrations path: {:?}", config.integrations_path);
    tracing::info!("⏱️  Sync interval: {} minutes", config.sync_interval_minutes);
    
    // Sync engine oluştur
    let engine = SyncEngine::new(config).await?;
    
    // Motoru başlat
    tracing::info!("🔄 Starting sync engine...");
    engine.start().await?;
    
    Ok(())
}
