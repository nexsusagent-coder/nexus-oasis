//! ─── Marketplace Configuration ───

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Marketplace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceConfig {
    /// Skills installation directory
    pub skills_dir: PathBuf,
    
    /// Registry configuration
    pub registry: RegistryConfig,
    
    /// Enable auto-update
    pub auto_update: bool,
    
    /// Update check interval (hours)
    pub update_check_interval: u32,
    
    /// Cache directory
    pub cache_dir: PathBuf,
    
    /// Max cache size (MB)
    pub max_cache_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Registry API URL
    pub api_url: String,
    
    /// API key for publishing
    pub api_key: Option<String>,
    
    /// Cache TTL (seconds)
    pub cache_ttl_secs: u64,
}

impl Default for MarketplaceConfig {
    fn default() -> Self {
        let base_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient");
        
        Self {
            skills_dir: base_dir.join("skills"),
            registry: RegistryConfig::default(),
            auto_update: true,
            update_check_interval: 24,
            cache_dir: base_dir.join("cache"),
            max_cache_size: 500,
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            api_url: "https://registry.sentient.ai/api/v1".into(),
            api_key: None,
            cache_ttl_secs: 3600,
        }
    }
}
