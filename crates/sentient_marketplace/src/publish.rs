//! ─── Skill Publishing ───

use crate::{MarketplaceError, SkillManifest};

/// Skill publisher
pub struct SkillPublisher {
    api_key: String,
    registry_url: String,
}

impl SkillPublisher {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            registry_url: "https://registry.sentient.ai/api/v1".into(),
        }
    }
    
    /// Validate skill for publishing
    pub fn validate(&self, manifest: &SkillManifest) -> Result<(), String> {
        manifest.validate()?;
        
        // Additional publishing checks
        if manifest.skill.repository.is_none() {
            return Err("Repository URL is required for publishing".into());
        }
        
        if manifest.skill.authors.is_empty() {
            return Err("At least one author is required".into());
        }
        
        if manifest.skill.license.is_empty() {
            return Err("License is required".into());
        }
        
        Ok(())
    }
    
    /// Package skill for publishing
    pub fn package(&self, path: &str) -> Result<Vec<u8>, MarketplaceError> {
        use std::io::Write;
        
        // Create tar.gz of skill directory
        let mut bytes = Vec::new();
        
        // Basic packaging implementation
        // 1. Read skill.toml
        let skill_toml = std::fs::read_to_string(format!("{}/skill.toml", path))
            .unwrap_or_else(|_| "[skill]\nname = \"unknown\"".to_string());
        
        // 2. Read source files (simplified)
        let src = std::fs::read_to_string(format!("{}/src/main.rs", path))
            .unwrap_or_else(|_| "// Skill source".to_string());
        
        // 3. Create simple archive format (production: use tar+gzip)
        writeln!(bytes, "SKILL_TOML:").ok();
        writeln!(bytes, "{}", skill_toml).ok();
        writeln!(bytes, "SOURCE:").ok();
        writeln!(bytes, "{}", src).ok();
        
        log::info!("📦 Packaged skill from: {}", path);
        
        Ok(bytes)
    }
    
    /// Publish to registry
    pub async fn publish(&self, manifest: &SkillManifest, package: &[u8]) -> Result<(), MarketplaceError> {
        self.validate(manifest).map_err(MarketplaceError::InvalidManifest)?;
        
        let client = reqwest::Client::new();
        
        // Upload package
        let part = reqwest::multipart::Part::bytes(package.to_vec())
            .file_name("skill.tar.gz")
            .mime_str("application/gzip")
            .map_err(|e| MarketplaceError::Internal(e.to_string()))?;
        
        let form = reqwest::multipart::Form::new()
            .part("package", part)
            .text("manifest", serde_json::to_string(manifest)?);
        
        let response = client
            .post(format!("{}/skills/publish", self.registry_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(MarketplaceError::Internal(format!("Publish failed: {}", error)));
        }
        
        log::info!("Published skill {} v{}", manifest.skill.id, manifest.skill.version);
        
        Ok(())
    }
}
