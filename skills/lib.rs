//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Skills - Modüler Yetenek Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Skill'ler, SENTIENT'nın genişletilebilir yetenek sistemidir.
//!  Her skill belirli bir görev kategorisini yerine getirir.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub mod research;
pub mod codegen;
pub mod analysis;
pub mod automation;

// ═══════════════════════════════════════════════════════════════════════════════
//  SKILL TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Tüm skill'lerin implement ettiği temel arayüz
pub trait Skill: Send + Sync {
    /// Skill benzersiz ID'si
    fn id(&self) -> Uuid;
    
    /// Skill ismi
    fn name(&self) -> &str;
    
    /// Skill açıklaması
    fn description(&self) -> &str;
    
    /// Skill versiyonu
    fn version(&self) -> &str;
    
    /// Skill'i çalıştır
    fn execute(&self, input: SkillInput) -> sentient_common::error::SENTIENTResult<SkillOutput>;
    
    /// Skill yapılandırmasını yükle
    fn load_config(&mut self, path: &Path) -> sentient_common::error::SENTIENTResult<()>;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SKILL VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

/// Skill girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInput {
    pub query: String,
    pub context: HashMap<String, serde_json::Value>,
    pub options: SkillOptions,
}

impl SkillInput {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            context: HashMap::new(),
            options: SkillOptions::default(),
        }
    }
    
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        self.context.insert(key.to_string(), value);
        self
    }
}

/// Skill seçenekleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillOptions {
    pub timeout_secs: u64,
    pub max_results: usize,
    pub verbose: bool,
    pub cache_enabled: bool,
}

impl Default for SkillOptions {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            max_results: 10,
            verbose: false,
            cache_enabled: true,
        }
    }
}

/// Skill çıktısı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillOutput {
    pub success: bool,
    pub result: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub artifacts: Vec<Artifact>,
}

impl SkillOutput {
    pub fn success(result: &str) -> Self {
        Self {
            success: true,
            result: result.to_string(),
            metadata: HashMap::new(),
            artifacts: Vec::new(),
        }
    }
    
    pub fn failure(error: &str) -> Self {
        Self {
            success: false,
            result: error.to_string(),
            metadata: HashMap::new(),
            artifacts: Vec::new(),
        }
    }
    
    pub fn with_artifact(mut self, artifact: Artifact) -> Self {
        self.artifacts.push(artifact);
        self
    }
}

/// Skill tarafından üretilen dosya/çıktı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub content: String,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Document,
    Image,
    Data,
    Log,
    Config,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SKILL CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Skill yapılandırma dosyası
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub skill: SkillMeta,
    pub config: HashMap<String, toml::Value>,
    pub triggers: SkillTriggers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTriggers {
    pub keywords: Vec<String>,
    pub patterns: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SKILL MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Skill yöneticisi
pub struct SkillManager {
    skills: HashMap<String, Box<dyn Skill>>,
}

impl SkillManager {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
        }
    }
    
    /// Skill kaydet
    pub fn register(&mut self, skill: Box<dyn Skill>) {
        let name = skill.name().to_string();
        log::info!("🛠️  SKILL: '{}' yüklendi", name);
        self.skills.insert(name, skill);
    }
    
    /// Skill çalıştır
    pub fn execute(&self, name: &str, input: SkillInput) -> sentient_common::error::SENTIENTResult<SkillOutput> {
        if let Some(skill) = self.skills.get(name) {
            skill.execute(input)
        } else {
            Err(sentient_common::error::SENTIENTError::SkillNotFound(name.to_string()))
        }
    }
    
    /// Yüklü skill'leri listele
    pub fn list(&self) -> Vec<&str> {
        self.skills.keys().map(String::as_str).collect()
    }
    
    /// Trigger'dan skill bul
    pub fn find_by_trigger(&self, text: &str) -> Option<&str> {
        let text_lower = text.to_lowercase();
        
        for (_name, _skill) in &self.skills {
            // Basit keyword eşleştirme
            // Gerçek impl'de regex pattern kullanılır
        }
        
        // Varsayılan: research
        if text_lower.contains("ara") || text_lower.contains("bul") {
            Some("research")
        } else if text_lower.contains("kod") || text_lower.contains("yaz") {
            Some("codegen")
        } else if text_lower.contains("analiz") || text_lower.contains("veri") {
            Some("analysis")
        } else if text_lower.contains("otomatik") || text_lower.contains("workflow") {
            Some("automation")
        } else {
            None
        }
    }
}

impl Default for SkillManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_input() {
        let input = SkillInput::new("Test query")
            .with_context("user", serde_json::json!("test_user"));
        
        assert_eq!(input.query, "Test query");
        assert!(input.context.contains_key("user"));
    }
    
    #[test]
    fn test_skill_output() {
        let output = SkillOutput::success("Test result");
        assert!(output.success);
        assert_eq!(output.result, "Test result");
    }
    
    #[test]
    fn test_skill_manager() {
        let manager = SkillManager::new();
        let skills = manager.list();
        assert!(skills.is_empty());
    }
    
    #[test]
    fn test_find_by_trigger() {
        let manager = SkillManager::new();
        
        let result = manager.find_by_trigger("Python hakkında araştır");
        assert_eq!(result, Some("research"));
        
        let result = manager.find_by_trigger("Kod yaz");
        assert_eq!(result, Some("codegen"));
    }
}
