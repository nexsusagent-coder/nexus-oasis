//! ─── Skills Importer ───
//!
//! Import skills from various sources:
//! - ClawHub (OpenClaw-compatible)
//! - Custom Git repositories
//! - Local directories
//! - NPM packages
//!
//! Usage:
//! ```rust
//! let importer = SkillsImporter::new();
//! 
//! // Search ClawHub
//! let skills = importer.search("translator").await?;
//! 
//! // Install skill
//! importer.install("translator-pro").await?;
//! ```

pub mod clawhub;
pub mod git;
pub mod local;
pub mod registry;
pub mod skill;
pub mod install;

pub use skill::{Skill, SkillManifest, SkillMetadata};
pub use registry::SkillsRegistry;
pub use install::{Installer, InstallProgress};

/// Skills importer
pub struct SkillsImporter {
    registry: SkillsRegistry,
    http: reqwest::Client,
}

impl SkillsImporter {
    pub fn new() -> Self {
        Self {
            registry: SkillsRegistry::new(),
            http: reqwest::Client::new(),
        }
    }
    
    /// Search skills across all sources
    pub async fn search(&self, query: &str) -> Result<Vec<Skill>, SkillsError> {
        let mut results = Vec::new();
        
        // Search ClawHub
        if let Ok(clawhub_skills) = self.search_clawhub(query).await {
            results.extend(clawhub_skills);
        }
        
        // Search local registry
        if let Ok(local_skills) = self.registry.search(query) {
            results.extend(local_skills);
        }
        
        Ok(results)
    }
    
    /// Install skill by ID
    pub async fn install(&self, skill_id: &str) -> Result<(), SkillsError> {
        let installer = install::Installer::new();
        installer.install(skill_id).await
    }
    
    /// List installed skills
    pub fn list_installed(&self) -> Result<Vec<Skill>, SkillsError> {
        self.registry.list_installed()
    }
    
    /// Uninstall skill
    pub async fn uninstall(&self, skill_id: &str) -> Result<(), SkillsError> {
        self.registry.uninstall(skill_id)
    }
    
    /// Update skill
    pub async fn update(&self, skill_id: &str) -> Result<(), SkillsError> {
        let installer = install::Installer::new();
        installer.update(skill_id).await
    }
}

impl Default for SkillsImporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Skills error
#[derive(Debug, thiserror::Error)]
pub enum SkillsError {
    #[error("Skill not found: {0}")]
    NotFound(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Install error: {0}")]
    Install(String),
    
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
