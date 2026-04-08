//! ═══════════════════════════════════════════════════════════════════════════════
//!  SKILLS HUB - Yetenekler Paneli
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Skill metadata (YAML+MD formatından)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub tools: Vec<String>,
    pub permissions: SkillPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPermissions {
    pub file_read: bool,
    pub file_write: bool,
    pub bash_execute: bool,
    pub network_access: bool,
}

/// Skills Hub - Yüklenmiş skill'leri yönet
pub struct SkillsHub {
    skills: HashMap<String, SkillMetadata>,
    skills_dir: String,
}

impl SkillsHub {
    pub fn new(skills_dir: &str) -> Self {
        Self {
            skills: HashMap::new(),
            skills_dir: skills_dir.to_string(),
        }
    }
    
    /// Skill'i yükle
    pub fn load_skill(&mut self, name: &str) -> Result<(), String> {
        // YAML'den metadata oku
        let yaml_path = format!("{}/{}/skill.yaml", self.skills_dir, name);
        
        // Basit implementation - production'da proper YAML parser kullan
        let metadata = SkillMetadata {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("{} yeteneği", name),
            category: "general".to_string(),
            tools: vec![],
            permissions: SkillPermissions {
                file_read: true,
                file_write: false,
                bash_execute: false,
                network_access: false,
            },
        };
        
        self.skills.insert(name.to_string(), metadata);
        Ok(())
    }
    
    /// Tüm skill'leri listele
    pub fn list_skills(&self) -> Vec<&SkillMetadata> {
        self.skills.values().collect()
    }
    
    /// Skill getir
    pub fn get_skill(&self, name: &str) -> Option<&SkillMetadata> {
        self.skills.get(name)
    }
    
    /// Skill'in tool'larını getir
    pub fn get_skill_tools(&self, name: &str) -> Vec<String> {
        self.skills.get(name)
            .map(|s| s.tools.clone())
            .unwrap_or_default()
    }
}

impl Default for SkillsHub {
    fn default() -> Self {
        Self::new("/root/SENTIENT_CORE/skills")
    }
}
