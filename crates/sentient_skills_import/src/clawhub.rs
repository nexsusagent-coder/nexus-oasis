//! ─── ClawHub Integration ───
//!
//! Import skills from ClawHub (OpenClaw-compatible)

use crate::{Skill, SkillsError, SkillsImporter};
use serde::{Deserialize, Serialize};

const CLAWHUB_API: &str = "https://api.clawhub.ai/v1";

impl SkillsImporter {
    /// Search ClawHub
    pub async fn search_clawhub(&self, query: &str) -> Result<Vec<Skill>, SkillsError> {
        let url = format!("{}/skills/search?q={}", CLAWHUB_API, 
            urlencoding::encode(query));
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| SkillsError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(SkillsError::Network(format!("HTTP {}", response.status())));
        }
        
        let result: ClawHubSearchResult = response.json().await
            .map_err(|e| SkillsError::Parse(e.to_string()))?;
        
        Ok(result.skills.into_iter().map(|s| s.into()).collect())
    }
    
    /// Get skill details from ClawHub
    pub async fn get_clawhub_skill(&self, skill_id: &str) -> Result<Skill, SkillsError> {
        let url = format!("{}/skills/{}", CLAWHUB_API, skill_id);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| SkillsError::Network(e.to_string()))?;
        
        let skill: ClawHubSkill = response.json().await
            .map_err(|e| SkillsError::Parse(e.to_string()))?;
        
        Ok(skill.into())
    }
    
    /// Download skill from ClawHub
    pub async fn download_clawhub_skill(&self, skill_id: &str) -> Result<Vec<u8>, SkillsError> {
        let url = format!("{}/skills/{}/download", CLAWHUB_API, skill_id);
        
        let response = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| SkillsError::Network(e.to_string()))?;
        
        let bytes = response.bytes().await
            .map_err(|e| SkillsError::Network(e.to_string()))?;
        
        Ok(bytes.to_vec())
    }
}

/// ClawHub search result
#[derive(Debug, Deserialize)]
struct ClawHubSearchResult {
    skills: Vec<ClawHubSkill>,
}

/// ClawHub skill format
#[derive(Debug, Deserialize, Serialize)]
struct ClawHubSkill {
    id: String,
    name: String,
    description: String,
    author: String,
    version: String,
    category: Option<String>,
    tags: Vec<String>,
    rating: Option<f32>,
    downloads: Option<u32>,
    source_url: Option<String>,
    manifest_url: Option<String>,
}

impl From<ClawHubSkill> for Skill {
    fn from(clawhub: ClawHubSkill) -> Self {
        use crate::skill::{SkillManifest, SkillMetadata, SkillSource};
        
        Self {
            id: clawhub.id,
            manifest: SkillManifest {
                name: clawhub.name,
                version: clawhub.version,
                description: clawhub.description,
                author: clawhub.author,
                main: "index.js".into(),
                dependencies: vec![],
                config: None,
            },
            metadata: SkillMetadata {
                category: clawhub.category.unwrap_or_default(),
                tags: clawhub.tags,
                rating: clawhub.rating,
                downloads: clawhub.downloads,
                created_at: None,
                updated_at: None,
            },
            source: SkillSource::ClawHub,
            installed: false,
            local_path: None,
        }
    }
}

/// URL encoding
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
