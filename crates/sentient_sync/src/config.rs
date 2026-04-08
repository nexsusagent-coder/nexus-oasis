//! Sync Engine Configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Ana konfigürasyon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Repoların bulunduğu kök dizin
    pub integrations_path: PathBuf,
    
    /// State dosyası yolu
    pub state_path: PathBuf,
    
    /// Senkronizasyon aralığı (dakika)
    pub sync_interval_minutes: u64,
    
    /// Eş zamanlı güncelleme limiti
    pub max_concurrent_updates: usize,
    
    /// Ağ zaman aşımı (saniye)
    pub network_timeout_secs: u64,
    
    /// Conflict çözüm stratejisi
    pub conflict_strategy: ConflictStrategy,
    
    /// Otomatik merge denemesi
    pub auto_merge: bool,
    
    /// Güncelleme sonrası build kontrolü
    pub verify_build: bool,
    
    /// Log seviyesi
    pub log_level: LogLevel,
    
    /// Discord/Telegram bildirimleri (sadece kritik hatalar)
    pub notifications: NotificationConfig,
    
    /// Güvenlik ayarları
    pub security: SecurityConfig,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            integrations_path: PathBuf::from("./integrations"),
            state_path: PathBuf::from("./data/sync_state.db"),
            sync_interval_minutes: 30, // Her 30 dakikada bir kontrol
            max_concurrent_updates: 5,
            network_timeout_secs: 60,
            conflict_strategy: ConflictStrategy::PreferTheirs,
            auto_merge: true,
            verify_build: false, // İlk başta kapalı
            log_level: LogLevel::Info,
            notifications: NotificationConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

/// Conflict çözüm stratejisi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictStrategy {
    /// Upstream'ı tercih et (sessizce üzerine yaz)
    PreferTheirs,
    /// Local değişiklikleri koru
    PreferOurs,
    /// Conflict varsa atla
    Skip,
    /// Conflict varsa manuel müdahale iste (önerilmez)
    Manual,
}

/// Log seviyesi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Silent, // Hiç log yazma
}

/// Bildirim konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Sadece kritik hatalar için bildirim
    pub critical_only: bool,
    
    /// Discord webhook URL (opsiyonel)
    pub discord_webhook: Option<String>,
    
    /// Telegram bot token (opsiyonel)
    pub telegram_token: Option<String>,
    
    /// Telegram chat ID
    pub telegram_chat_id: Option<String>,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            critical_only: true,
            discord_webhook: None,
            telegram_token: None,
            telegram_chat_id: None,
        }
    }
}

/// Güvenlik konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// İmza doğrulama
    pub verify_signatures: bool,
    
    /// İzin verilen branch'ler
    pub allowed_branches: Vec<String>,
    
    /// Blocklist (güvenlik açığı olan versiyonlar)
    pub blocked_commits: Vec<String>,
    
    /// Maximum dosya boyutu (MB)
    pub max_file_size_mb: u64,
    
    /// Hassas dosya kontrolü (.env, credentials, etc.)
    pub scan_secrets: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            verify_signatures: false,
            allowed_branches: vec!["main".to_string(), "master".to_string()],
            blocked_commits: vec![],
            max_file_size_mb: 10,
            scan_secrets: true,
        }
    }
}

impl SyncConfig {
    /// Varsayılan yol ile config oluştur
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Belirli bir integrations yolu ile config oluştur
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        Self {
            integrations_path: path.into(),
            ..Self::default()
        }
    }
    
    /// Config dosyasından yükle
    pub async fn load(path: &std::path::Path) -> Result<Self, crate::SyncError> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| crate::SyncError::Config(format!("Failed to read config: {}", e)))?;
        
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| crate::SyncError::Config(format!("Invalid config format: {}", e)))?;
        
        Ok(config)
    }
    
    /// Config dosyasına kaydet
    pub async fn save(&self, path: &std::path::Path) -> Result<(), crate::SyncError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::SyncError::Config(format!("Failed to serialize config: {}", e)))?;
        
        tokio::fs::write(path, content).await
            .map_err(|e| crate::SyncError::Config(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }
    
    /// Sync interval'i Duration olarak döndür
    pub fn sync_interval(&self) -> Duration {
        Duration::from_secs(self.sync_interval_minutes * 60)
    }
}
