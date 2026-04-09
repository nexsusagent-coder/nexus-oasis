//! ─── Skill Registry ───

use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{MarketplaceSkill, MarketplaceError, SkillManifest};

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub api_url: String,
    pub api_key: Option<String>,
    pub cache_ttl_secs: u64,
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

/// Skill registry client
pub struct SkillRegistry {
    config: RegistryConfig,
    client: Client,
    cache: std::collections::HashMap<String, (MarketplaceSkill, std::time::Instant)>,
}

impl SkillRegistry {
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Search skills
    pub async fn search(&self, query: &str) -> Result<Vec<crate::SearchResult>, MarketplaceError> {
        let url = format!("{}/skills/search?q={}", self.config.api_url, 
            urlencoding::encode(query));
        
        let mut request = self.client.get(&url);
        
        if let Some(ref api_key) = self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(MarketplaceError::Network(format!("Search failed: {}", response.status())));
        }
        
        let results: Vec<crate::SearchResult> = response.json().await
            .map_err(|e| MarketplaceError::Json(e))?;
        
        Ok(results)
    }
    
    /// Get skill by ID
    pub async fn get(&self, id: &str) -> Result<MarketplaceSkill, MarketplaceError> {
        // Check cache
        if let Some((skill, timestamp)) = self.cache.get(id) {
            let elapsed = timestamp.elapsed().as_secs();
            if elapsed < self.config.cache_ttl_secs {
                return Ok(skill.clone());
            }
        }
        
        let url = format!("{}/skills/{}", self.config.api_url, id);
        
        let mut request = self.client.get(&url);
        
        if let Some(ref api_key) = self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        if response.status().as_u16() == 404 {
            return Err(MarketplaceError::NotFound(id.into()));
        }
        
        if !response.status().is_success() {
            return Err(MarketplaceError::Network(format!("Get failed: {}", response.status())));
        }
        
        let skill: MarketplaceSkill = response.json().await
            .map_err(|e| MarketplaceError::Json(e))?;
        
        Ok(skill)
    }
    
    /// Get categories
    pub async fn categories(&self) -> Result<Vec<crate::Category>, MarketplaceError> {
        let url = format!("{}/categories", self.config.api_url);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        let categories: Vec<crate::Category> = response.json().await
            .map_err(|e| MarketplaceError::Json(e))?;
        
        Ok(categories)
    }
    
    /// Get trending skills
    pub async fn trending(&self, limit: usize) -> Result<Vec<MarketplaceSkill>, MarketplaceError> {
        let url = format!("{}/skills/trending?limit={}", self.config.api_url, limit);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        let skills: Vec<MarketplaceSkill> = response.json().await
            .map_err(|e| MarketplaceError::Json(e))?;
        
        Ok(skills)
    }
    
    /// Get featured skills
    pub async fn featured(&self) -> Result<Vec<MarketplaceSkill>, MarketplaceError> {
        let url = format!("{}/skills/featured", self.config.api_url);
        
        let response = self.client.get(&url).send().await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        let skills: Vec<MarketplaceSkill> = response.json().await
            .map_err(|e| MarketplaceError::Json(e))?;
        
        Ok(skills)
    }
    
    /// Publish skill
    pub async fn publish(&self, manifest: &SkillManifest) -> Result<(), MarketplaceError> {
        let url = format!("{}/skills", self.config.api_url);
        
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| MarketplaceError::PermissionDenied("API key required for publishing".into()))?;
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(manifest)
            .send()
            .await
            .map_err(|e| MarketplaceError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(MarketplaceError::Internal(format!("Publish failed: {}", error)));
        }
        
        Ok(())
    }
}

/// URL encoding
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
