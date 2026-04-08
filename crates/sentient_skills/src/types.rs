//! Skill Types - DeerFlow SKILL.md format

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Skill - Ana skill yapısı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Metadata (frontmatter)
    pub metadata: SkillMetadata,
    
    /// Skill içeriği (markdown)
    pub content: String,
    
    /// Yükleme zamanı
    pub loaded_at: DateTime<Utc>,
    
    /// Kaynak dosya yolu
    pub source_path: Option<String>,
}

/// Skill Metadata - Frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// Skill adı
    pub name: String,
    
    /// Açıklama
    pub description: String,
    
    /// Versiyon
    #[serde(default = "default_version")]
    pub version: String,
    
    /// Kategori
    #[serde(default)]
    pub category: SkillCategory,
    
    /// Tetikleyiciler
    #[serde(default)]
    pub triggers: Vec<SkillTrigger>,
    
    /// Gerekli tool'lar
    #[serde(default)]
    pub required_tools: Vec<String>,
    
    /// Önerilen model
    #[serde(default)]
    pub recommended_model: Option<String>,
    
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Yazar
    #[serde(default)]
    pub author: Option<String>,
    
    /// Timeout (saniye)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    
    /// Maksimum retry
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
}

fn default_version() -> String { "1.0.0".to_string() }
fn default_timeout() -> u64 { 300 }
fn default_max_retries() -> u32 { 3 }

/// Skill Kategorileri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SkillCategory {
    /// Web araştırma
    Research,
    
    /// İçerik üretimi
    ContentGeneration,
    
    /// Kod yazma
    Coding,
    
    /// Veri analizi
    DataAnalysis,
    
    /// Görsel üretim
    MediaGeneration,
    
    /// Otomasyon
    Automation,
    
    /// Yardımcı
    Utility,
    
    /// Diğer
    Other,
}

impl Default for SkillCategory {
    fn default() -> Self {
        Self::Other
    }
}

/// Skill Trigger - Ne zaman çalıştırılacağı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrigger {
    /// Trigger tipi
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    
    /// Pattern (regex veya keyword)
    pub pattern: String,
    
    /// Öncelik (yüksek = önce)
    #[serde(default)]
    pub priority: u8,
}

/// Trigger Tipleri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    /// Keyword match
    Keyword,
    
    /// Regex pattern
    Regex,
    
    /// Intent classification
    Intent,
    
    /// Manuel çağrı
    Manual,
}

impl SkillMetadata {
    /// Yeni metadata oluştur
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: default_version(),
            category: SkillCategory::default(),
            triggers: Vec::new(),
            required_tools: Vec::new(),
            recommended_model: None,
            tags: Vec::new(),
            author: None,
            timeout_secs: default_timeout(),
            max_retries: default_max_retries(),
        }
    }
    
    /// Trigger ekle
    pub fn with_trigger(mut self, trigger: SkillTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }
    
    /// Tool ekle
    pub fn with_tool(mut self, tool: impl Into<String>) -> Self {
        self.required_tools.push(tool.into());
        self
    }
    
    /// Tag ekle
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
}

impl Skill {
    /// Yeni skill oluştur
    pub fn new(metadata: SkillMetadata, content: impl Into<String>) -> Self {
        Self {
            metadata,
            content: content.into(),
            loaded_at: Utc::now(),
            source_path: None,
        }
    }
    
    /// Tetikleniyor mu kontrol et
    pub fn is_triggered_by(&self, input: &str) -> bool {
        for trigger in &self.metadata.triggers {
            match trigger.trigger_type {
                TriggerType::Keyword => {
                    if input.to_lowercase().contains(&trigger.pattern.to_lowercase()) {
                        return true;
                    }
                }
                TriggerType::Regex => {
                    if let Ok(re) = regex::Regex::new(&trigger.pattern) {
                        if re.is_match(input) {
                            return true;
                        }
                    }
                }
                TriggerType::Intent | TriggerType::Manual => {
                    // Intent ve Manual için harici processor gerekir
                }
            }
        }
        false
    }
    
    /// Özet al (ilk 200 karakter)
    pub fn summary(&self) -> String {
        let content = self.content.trim();
        if content.len() > 200 {
            format!("{}...", &content[..200])
        } else {
            content.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_creation() {
        let metadata = SkillMetadata::new("deep-research", "Deep web research skill")
            .with_trigger(SkillTrigger {
                trigger_type: TriggerType::Keyword,
                pattern: "research".to_string(),
                priority: 1,
            })
            .with_tool("web_search")
            .with_tag("research");
        
        let skill = Skill::new(metadata, "# Deep Research\n\nContent here...");
        
        assert_eq!(skill.metadata.name, "deep-research");
        assert!(skill.is_triggered_by("research this topic"));
        assert!(!skill.is_triggered_by("write code"));
    }
    
    #[test]
    fn test_trigger_regex() {
        let metadata = SkillMetadata::new("test", "Test skill")
            .with_trigger(SkillTrigger {
                trigger_type: TriggerType::Regex,
                pattern: r"what is \w+".to_string(),
                priority: 1,
            });
        
        let skill = Skill::new(metadata, "content");
        
        assert!(skill.is_triggered_by("what is AI?"));
        assert!(!skill.is_triggered_by("explain AI"));
    }
}
