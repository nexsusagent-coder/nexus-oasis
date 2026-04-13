//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SETTINGS MANAGER v4.0.0 - Multi-Key Vault & Dynamic Routing
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//!  Merkezi ayar yönetimi. Tüm konfigürasyonlar buradan yönetilir.
//!  Universal LLM Gateway + 20+ Messaging Channel Support
//!  Multi-Key Vault: Sınırsız API key ve provider desteği
//!  Dynamic Routing: Görev zorluğuna göre otomatik model seçimi

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod general;
pub mod llm;
pub mod security;
pub mod automation;
pub mod integrations;
pub mod memory;
pub mod api;
pub mod custom_provider;
pub mod channels;

// Human Emulation Modülü
pub mod human_emulation;

// Multi-Key Vault & Dynamic Routing (v4.0.0)
pub mod keyring;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use general::GeneralSettings;
pub use llm::LlmSettings;
pub use security::SecuritySettings;
pub use automation::AutomationSettings;
pub use integrations::IntegrationSettings;
pub use memory::MemorySettings;
pub use custom_provider::{CustomProviderConfig, ApiFormat, ModelInfo, PREDEFINED_PROVIDERS};
pub use channels::{ChannelConfig, ChannelType, ChannelFeatures, ChannelSetupGuide};

// Human Emulation Re-exports
pub use human_emulation::{
    HumanEmulationSettings, HumanEmulationManager, RuntimeStats,
    MouseEmulationSettings, TypingEmulationSettings, ProxyEmulationSettings,
    CaptchaEmulationSettings, BehaviorEmulationSettings, AdvancedEmulationSettings,
    MovementPatternSetting, ProxyRotationMode, CaptchaTypeSetting, ProxyEntry,
};

// Key Ring Re-exports (Multi-Key Vault & Dynamic Routing)
pub use keyring::{
    KeyRing, KeyRingManager, ApiKeyEntry, KeyStatus,
    ModelInfo as KeyRingModelInfo, ComplexityLevel, RoutingMode,
    ModelApprovalRequest, ModelApprovalResponse,
};

/// Ana ayarlar yapısı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub general: GeneralSettings,
    pub llm: LlmSettings,
    pub security: SecuritySettings,
    pub automation: AutomationSettings,
    pub integrations: IntegrationSettings,
    pub memory: MemorySettings,
    
    /// Custom LLM Providers (Universal Gateway)
    #[serde(default)]
    pub custom_providers: Vec<CustomProviderConfig>,
    
    /// Messaging Channels (20+ platforms)
    #[serde(default)]
    pub channels: Vec<ChannelConfig>,
    
    /// Human Emulation (İnsan Taklidi)
    #[serde(default)]
    pub human_emulation: HumanEmulationSettings,
    
    /// Multi-Key Vault (v4.0.0)
    #[serde(default)]
    pub keyring: KeyRing,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            llm: LlmSettings::default(),
            security: SecuritySettings::default(),
            automation: AutomationSettings::default(),
            integrations: IntegrationSettings::default(),
            memory: MemorySettings::default(),
            custom_providers: vec![],
            channels: vec![],
            human_emulation: HumanEmulationSettings::default(),
            keyring: KeyRing::default(),
        }
    }
}

/// Ayarlar Yöneticisi
pub struct SettingsManager {
    settings: Arc<RwLock<Settings>>,
    config_path: PathBuf,
    watchers: Vec<Box<dyn Fn(&Settings) + Send + Sync>>,
}

impl SettingsManager {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("settings.toml");
        
        let settings = if config_path.exists() {
            Self::load_settings(&config_path).unwrap_or_default()
        } else {
            Settings::default()
        };
        
        Self {
            settings: Arc::new(RwLock::new(settings)),
            config_path,
            watchers: Vec::new(),
        }
    }
    
    /// Ayarları yükle
    fn load_settings(path: &PathBuf) -> anyhow::Result<Settings> {
        let content = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }
    
    /// Ayarları kaydet
    pub async fn save(&self) -> anyhow::Result<()> {
        let settings = self.settings.read().await.clone();
        let content = toml::to_string_pretty(&settings)?;
        
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(&self.config_path, content)?;
        
        // Değişiklikleri bildir
        self.notify_watchers(&settings);
        
        Ok(())
    }
    
    /// Ayarları al
    pub async fn get(&self) -> Settings {
        self.settings.read().await.clone()
    }
    
    /// Ayarları güncelle
    pub async fn update<F>(&self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Settings)
    {
        let mut settings = self.settings.write().await;
        f(&mut settings);
        drop(settings);
        self.save().await
    }
    
    /// Tek bir değeri güncelle
    pub async fn set(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()> {
        let mut settings = self.settings.write().await;
        
        // Nokta ile ayrılmış key'i parse et
        let parts: Vec<&str> = key.split('.').collect();
        
        match parts.as_slice() {
            ["general", "language"] => {
                if let Some(v) = value.as_str() {
                    settings.general.language = v.to_string();
                }
            }
            ["general", "theme"] => {
                if let Some(v) = value.as_str() {
                    settings.general.theme = v.to_string();
                }
            }
            ["llm", "provider"] => {
                if let Some(v) = value.as_str() {
                    settings.llm.provider = v.to_string();
                }
            }
            ["llm", "model"] => {
                if let Some(v) = value.as_str() {
                    settings.llm.model = v.to_string();
                }
            }
            ["llm", "temperature"] => {
                if let Some(v) = value.as_f64() {
                    settings.llm.temperature = v as f32;
                }
            }
            ["llm", "max_tokens"] => {
                if let Some(v) = value.as_u64() {
                    settings.llm.max_tokens = v as usize;
                }
            }
            ["automation", "autonomous_mode"] => {
                if let Some(v) = value.as_bool() {
                    settings.automation.autonomous_mode = v;
                }
            }
            ["automation", "screen_recording"] => {
                if let Some(v) = value.as_bool() {
                    settings.automation.screen_recording = v;
                }
            }
            ["automation", "keyboard_control"] => {
                if let Some(v) = value.as_bool() {
                    settings.automation.keyboard_control = v;
                }
            }
            ["automation", "mouse_control"] => {
                if let Some(v) = value.as_bool() {
                    settings.automation.mouse_control = v;
                }
            }
            ["security", "vgate_enabled"] => {
                if let Some(v) = value.as_bool() {
                    settings.security.vgate_enabled = v;
                }
            }
            ["security", "guardrails_enabled"] => {
                if let Some(v) = value.as_bool() {
                    settings.security.guardrails_enabled = v;
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown setting key: {}", key));
            }
        }
        
        drop(settings);
        self.save().await
    }
    
    /// Değişiklik izleyici ekle
    pub fn add_watcher<F>(&mut self, watcher: F)
    where
        F: Fn(&Settings) + Send + Sync + 'static
    {
        self.watchers.push(Box::new(watcher));
    }
    
    /// İzleyicileri bildir
    fn notify_watchers(&self, settings: &Settings) {
        for watcher in &self.watchers {
            watcher(settings);
        }
    }
    
    /// Ayarları sıfırla
    pub async fn reset(&self) -> anyhow::Result<()> {
        let mut settings = self.settings.write().await;
        *settings = Settings::default();
        drop(settings);
        self.save().await
    }
    
    /// Belirli bir kategoriyi sıfırla
    pub async fn reset_category(&self, category: &str) -> anyhow::Result<()> {
        let mut settings = self.settings.write().await;
        
        match category {
            "general" => settings.general = GeneralSettings::default(),
            "llm" => settings.llm = LlmSettings::default(),
            "security" => settings.security = SecuritySettings::default(),
            "automation" => settings.automation = AutomationSettings::default(),
            "integrations" => settings.integrations = IntegrationSettings::default(),
            "memory" => settings.memory = MemorySettings::default(),
            _ => return Err(anyhow::anyhow!("Unknown category: {}", category)),
        }
        
        drop(settings);
        self.save().await
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self::new()
    }
}
