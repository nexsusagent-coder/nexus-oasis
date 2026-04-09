//! ─── Skill Definition ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Unique identifier
    pub id: String,
    
    /// Skill manifest
    pub manifest: SkillManifest,
    
    /// Additional metadata
    pub metadata: SkillMetadata,
    
    /// Source
    pub source: SkillSource,
    
    /// Installation status
    pub installed: bool,
    
    /// Local path (if installed)
    pub local_path: Option<String>,
}

/// Skill manifest (skill.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    /// Display name
    pub name: String,
    
    /// Semantic version
    pub version: String,
    
    /// Description
    pub description: String,
    
    /// Author
    pub author: String,
    
    /// Main entry point
    pub main: String,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Configuration schema
    pub config: Option<ConfigSchema>,
}

/// Configuration schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSchema {
    pub properties: serde_json::Map<String, serde_json::Value>,
    pub required: Vec<String>,
}

/// Skill metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Category (e.g., "productivity", "communication")
    pub category: String,
    
    /// Tags for search
    pub tags: Vec<String>,
    
    /// Community rating (0-5)
    pub rating: Option<f32>,
    
    /// Download count
    pub downloads: Option<u32>,
    
    /// Creation date
    pub created_at: Option<DateTime<Utc>>,
    
    /// Last update date
    pub updated_at: Option<DateTime<Utc>>,
}

/// Skill source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillSource {
    ClawHub,
    Git,
    Local,
    Npm,
    Custom,
}

impl Skill {
    /// Create a new skill
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            manifest: SkillManifest {
                name: name.into(),
                version: "0.1.0".into(),
                description: String::new(),
                author: "unknown".into(),
                main: "index.js".into(),
                dependencies: vec![],
                config: None,
            },
            metadata: SkillMetadata {
                category: "general".into(),
                tags: vec![],
                rating: None,
                downloads: None,
                created_at: None,
                updated_at: None,
            },
            source: SkillSource::Local,
            installed: false,
            local_path: None,
        }
    }
    
    /// Set version
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.manifest.version = version.into();
        self
    }
    
    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.manifest.description = desc.into();
        self
    }
    
    /// Set author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.manifest.author = author.into();
        self
    }
    
    /// Add tag
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.metadata.tags.push(tag.into());
        self
    }
    
    /// Set category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.metadata.category = category.into();
        self
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{} by {} - {}",
            self.manifest.name,
            self.manifest.version,
            self.manifest.author,
            self.manifest.description
        )
    }
}

/// Example skill manifest (skill.yaml)
pub const EXAMPLE_MANIFEST: &str = r#"
name: "translator"
version: "1.0.0"
description: "Multi-language translation skill"
author: "sentient"
main: "index.js"
dependencies:
  - "@translate/core"
config:
  properties:
    defaultLanguage:
      type: string
      default: "tr"
    apiKey:
      type: string
      secret: true
  required:
    - apiKey
"#;
