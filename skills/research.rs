//! ═══════════════════════════════════════════════════════════════════════════════
//!  Research Skill - Web Araştırması
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Skill, SkillInput, SkillOutput, Artifact, ArtifactType};
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Web araştırması yapan skill
pub struct ResearchSkill {
    id: Uuid,
    config: ResearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchConfig {
    pub max_results: usize,
    pub timeout_secs: u64,
    pub sources: Vec<String>,
    pub cache_enabled: bool,
}

impl Default for ResearchConfig {
    fn default() -> Self {
        Self {
            max_results: 10,
            timeout_secs: 30,
            sources: vec!["web".into(), "academic".into()],
            cache_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub query: String,
    pub results: Vec<ResearchEntry>,
    pub summary: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchEntry {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source: String,
    pub relevance: f32,
}

impl ResearchSkill {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            config: ResearchConfig::default(),
        }
    }
    
    /// Araştırma gerçekleştir
    async fn perform_research(&self, query: &str) -> SENTIENTResult<ResearchResult> {
        // Gerçek impl'de Browser-Use entegrasyonu
        // Şimdilik mock sonuç döndür
        
        let results = vec![
            ResearchEntry {
                title: format!("{} hakkında bilgi", query),
                url: "https://example.com/1".to_string(),
                snippet: format!("{} ile ilgili detaylı bilgi...", query),
                source: "web".to_string(),
                relevance: 0.95,
            },
            ResearchEntry {
                title: format!("{} rehberi", query),
                url: "https://example.com/2".to_string(),
                snippet: format!("Kapsamlı {} rehberi...", query),
                source: "web".to_string(),
                relevance: 0.85,
            },
        ];
        
        Ok(ResearchResult {
            query: query.to_string(),
            summary: format!("{} hakkında {} sonuç bulundu.", query, results.len()),
            results,
            sources: vec!["web".to_string()],
        })
    }
}

impl Skill for ResearchSkill {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn name(&self) -> &str {
        "research"
    }
    
    fn description(&self) -> &str {
        "Web araştırması ve bilgi toplama"
    }
    
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    fn execute(&self, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        // Tokio runtime'da çalıştır
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.perform_research(&input.query))?;
        
        let output = SkillOutput::success(&result.summary)
            .with_artifact(Artifact {
                name: "research_results.json".to_string(),
                artifact_type: ArtifactType::Data,
                content: serde_json::to_string_pretty(&result).unwrap_or_default(),
                mime_type: Some("application/json".to_string()),
            });
        
        Ok(output)
    }
    
    fn load_config(&mut self, _path: &std::path::Path) -> SENTIENTResult<()> {
        // Config dosyasını yükle
        Ok(())
    }
}

impl Default for ResearchSkill {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_research_skill_creation() {
        let skill = ResearchSkill::new();
        assert_eq!(skill.name(), "research");
    }
    
    #[test]
    fn test_research_config_defaults() {
        let config = ResearchConfig::default();
        assert_eq!(config.max_results, 10);
    }
}
