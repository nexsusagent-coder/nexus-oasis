//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Marketplace - Skills Discovery and Management
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Features:
//!  - Skill discovery and search
//!  - Install/uninstall skills
//!  - Publish skills
//!  - Version management
//!  - Ratings and reviews
//!  - Categories and tags
//!
//!  Similar to:
//!  - OpenClaw ClawHub
//!  - Homebrew
//!  - npm

pub mod registry;
pub mod skill;
pub mod install;
pub mod publish;
pub mod search;
pub mod config;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use skill::{MarketplaceSkill, SkillManifest, SkillMetadata};
pub use registry::{SkillRegistry, RegistryConfig};
pub use install::{SkillInstaller, InstallResult};
pub use search::{SkillSearch, SearchResult};
pub use config::MarketplaceConfig;

/// ─── Marketplace Client ───

pub struct Marketplace {
    config: MarketplaceConfig,
    registry: Arc<SkillRegistry>,
    installer: Arc<SkillInstaller>,
    local_skills: Arc<RwLock<Vec<InstalledSkill>>>,
}

impl Marketplace {
    /// Create new marketplace client
    pub fn new(config: MarketplaceConfig) -> Self {
        let registry = Arc::new(SkillRegistry::new(config.registry.clone()));
        let installer = Arc::new(SkillInstaller::new(config.skills_dir.clone()));
        
        Self {
            config,
            registry,
            installer,
            local_skills: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Search for skills
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, MarketplaceError> {
        self.registry.search(query).await
    }
    
    /// Get skill details
    pub async fn get_skill(&self, id: &str) -> Result<MarketplaceSkill, MarketplaceError> {
        self.registry.get(id).await
    }
    
    /// Install skill
    pub async fn install(&self, id: &str, version: Option<&str>) -> Result<InstallResult, MarketplaceError> {
        let skill = self.registry.get(id).await?;
        let version = version.map(|v| v.to_string()).or(skill.latest_version.clone());
        
        let result = self.installer.install(&skill, version.as_deref()).await?;
        
        // Add to local skills
        let mut local = self.local_skills.write().await;
        local.push(InstalledSkill {
            id: skill.id.clone(),
            name: skill.name.clone(),
            version: result.version.clone(),
            installed_at: chrono::Utc::now(),
        });
        
        Ok(result)
    }
    
    /// Uninstall skill
    pub async fn uninstall(&self, id: &str) -> Result<(), MarketplaceError> {
        self.installer.uninstall(id).await?;
        
        // Remove from local skills
        let mut local = self.local_skills.write().await;
        local.retain(|s| s.id != id);
        
        Ok(())
    }
    
    /// Update skill
    pub async fn update(&self, id: &str) -> Result<InstallResult, MarketplaceError> {
        let skill = self.registry.get(id).await?;
        self.installer.update(&skill).await
    }
    
    /// List installed skills
    pub async fn list_installed(&self) -> Vec<InstalledSkill> {
        self.local_skills.read().await.clone()
    }
    
    /// List available updates
    pub async fn list_updates(&self) -> Result<Vec<AvailableUpdate>, MarketplaceError> {
        let local = self.local_skills.read().await;
        let mut updates = Vec::new();
        
        for skill in local.iter() {
            let remote = self.registry.get(&skill.id).await.ok();
            if let Some(remote) = remote {
                if remote.latest_version.as_ref() != Some(&skill.version) {
                    updates.push(AvailableUpdate {
                        id: skill.id.clone(),
                        name: skill.name.clone(),
                        current_version: skill.version.clone(),
                        latest_version: remote.latest_version.clone().unwrap_or_default(),
                    });
                }
            }
        }
        
        Ok(updates)
    }
    
    /// Publish skill
    pub async fn publish(&self, path: &str) -> Result<(), MarketplaceError> {
        let manifest = self.installer.load_manifest(path)?;
        self.registry.publish(&manifest).await
    }
    
    /// Get categories
    pub async fn categories(&self) -> Result<Vec<Category>, MarketplaceError> {
        self.registry.categories().await
    }
    
    /// Get trending skills
    pub async fn trending(&self, limit: usize) -> Result<Vec<MarketplaceSkill>, MarketplaceError> {
        self.registry.trending(limit).await
    }
    
    /// Get featured skills
    pub async fn featured(&self) -> Result<Vec<MarketplaceSkill>, MarketplaceError> {
        self.registry.featured().await
    }
}

/// Installed skill record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub id: String,
    pub name: String,
    pub version: String,
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

/// Available update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableUpdate {
    pub id: String,
    pub name: String,
    pub current_version: String,
    pub latest_version: String,
}

/// Skill category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub skill_count: u32,
}

/// ─── Errors ───

#[derive(Debug, thiserror::Error)]
pub enum MarketplaceError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Skill not found: {0}")]
    NotFound(String),
    
    #[error("Version not found: {0}")]
    VersionNotFound(String),
    
    #[error("Install failed: {0}")]
    InstallFailed(String),
    
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    
    #[error("Already installed: {0}")]
    AlreadyInstalled(String),
    
    #[error("Not installed: {0}")]
    NotInstalled(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config() {
        let config = MarketplaceConfig::default();
        assert!(!config.registry.api_url.is_empty());
    }
}
