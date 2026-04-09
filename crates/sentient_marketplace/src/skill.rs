//! ─── Skill Definition ───

use serde::{Deserialize, Serialize};
use semver::Version;
use chrono::{DateTime, Utc};

/// Skill manifest (skill.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub skill: SkillMetadata,
    pub dependencies: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
    pub config: Option<toml::Value>,
}

/// Skill metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub readme: Option<String>,
    pub changelog: Option<String>,
    pub min_sentient_version: Option<String>,
}

/// Marketplace skill (with additional metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    
    /// Latest stable version
    pub latest_version: Option<String>,
    
    /// All available versions
    pub versions: Vec<SkillVersion>,
    
    /// Download count
    pub downloads: u64,
    
    /// Star count
    pub stars: u64,
    
    /// Rating (0-5)
    pub rating: f32,
    
    /// Review count
    pub review_count: u32,
    
    /// Featured status
    pub featured: bool,
    
    /// Verified status
    pub verified: bool,
    
    /// Created at
    pub created_at: DateTime<Utc>,
    
    /// Updated at
    pub updated_at: DateTime<Utc>,
    
    /// Icon URL
    pub icon_url: Option<String>,
    
    /// Screenshot URLs
    pub screenshots: Vec<String>,
}

/// Skill version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillVersion {
    pub version: String,
    pub released_at: DateTime<Utc>,
    pub download_url: String,
    pub checksum: String,
    pub changelog: Option<String>,
    pub min_sentient_version: Option<String>,
    pub deprecated: bool,
    pub yanked: bool,
}

impl SkillManifest {
    /// Load from directory
    pub fn from_dir(path: &str) -> Result<Self, crate::MarketplaceError> {
        let manifest_path = format!("{}/skill.toml", path);
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Self = toml::from_str(&content)?;
        Ok(manifest)
    }
    
    /// Save to directory
    pub fn save(&self, path: &str) -> Result<(), crate::MarketplaceError> {
        let content = toml::to_string_pretty(self)?;
        let manifest_path = format!("{}/skill.toml", path);
        std::fs::write(&manifest_path, content)?;
        Ok(())
    }
    
    /// Validate manifest
    pub fn validate(&self) -> Result<(), String> {
        if self.skill.id.is_empty() {
            return Err("Skill ID is required".into());
        }
        if self.skill.name.is_empty() {
            return Err("Skill name is required".into());
        }
        if self.skill.version.is_empty() {
            return Err("Version is required".into());
        }
        
        // Validate version format
        Version::parse(&self.skill.version)
            .map_err(|e| format!("Invalid version: {}", e))?;
        
        Ok(())
    }
}

impl Default for SkillManifest {
    fn default() -> Self {
        Self {
            skill: SkillMetadata {
                id: "my-skill".into(),
                name: "My Skill".into(),
                version: "0.1.0".into(),
                description: "A SENTIENT skill".into(),
                authors: vec!["Anonymous".into()],
                license: "MIT".into(),
                repository: None,
                homepage: None,
                keywords: vec![],
                categories: vec!["general".into()],
                readme: None,
                changelog: None,
                min_sentient_version: None,
            },
            dependencies: None,
            permissions: None,
            config: None,
        }
    }
}
