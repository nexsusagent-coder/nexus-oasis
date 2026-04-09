//! ─── Skills Registry ───

use crate::{Skill, SkillsError};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// Local skills registry
pub struct SkillsRegistry {
    registry_path: PathBuf,
    skills: Vec<Skill>,
}

impl SkillsRegistry {
    pub fn new() -> Self {
        let registry_path = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sentient")
            .join("skills");
        
        Self {
            registry_path,
            skills: Vec::new(),
        }
    }
    
    /// Initialize registry
    pub fn init(&mut self) -> Result<(), SkillsError> {
        std::fs::create_dir_all(&self.registry_path)?;
        self.load_registry()?;
        Ok(())
    }
    
    /// Load registry from disk
    fn load_registry(&mut self) -> Result<(), SkillsError> {
        let registry_file = self.registry_path.join("registry.json");
        
        if !registry_file.exists() {
            return Ok(());
        }
        
        let content = std::fs::read_to_string(registry_file)?;
        let entries: Vec<RegistryEntry> = serde_json::from_str(&content)?;
        
        self.skills = entries
            .into_iter()
            .filter_map(|entry| {
                // Load skill manifest
                let skill_path = self.registry_path.join(&entry.id);
                let manifest_path = skill_path.join("skill.yaml");
                
                if manifest_path.exists() {
                    if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                        if let Ok(manifest) = serde_yaml::from_str(&content) {
                            return Some(Skill {
                                id: entry.id,
                                manifest,
                                metadata: entry.metadata,
                                source: entry.source,
                                installed: true,
                                local_path: Some(skill_path.to_string_lossy().into()),
                            });
                        }
                    }
                }
                None
            })
            .collect();
        
        Ok(())
    }
    
    /// Save registry to disk
    fn save_registry(&self) -> Result<(), SkillsError> {
        let entries: Vec<RegistryEntry> = self.skills
            .iter()
            .map(|s| RegistryEntry {
                id: s.id.clone(),
                metadata: s.metadata.clone(),
                source: s.source,
            })
            .collect();
        
        let content = serde_json::to_string_pretty(&entries)?;
        let registry_file = self.registry_path.join("registry.json");
        std::fs::write(registry_file, content)?;
        
        Ok(())
    }
    
    /// Search skills
    pub fn search(&self, query: &str) -> Result<Vec<Skill>, SkillsError> {
        let query_lower = query.to_lowercase();
        
        let results: Vec<Skill> = self.skills
            .iter()
            .filter(|s| {
                s.manifest.name.to_lowercase().contains(&query_lower)
                    || s.manifest.description.to_lowercase().contains(&query_lower)
                    || s.metadata.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect();
        
        Ok(results)
    }
    
    /// List installed skills
    pub fn list_installed(&self) -> Result<Vec<Skill>, SkillsError> {
        Ok(self.skills.clone())
    }
    
    /// Get skill by ID
    pub fn get(&self, id: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.id == id)
    }
    
    /// Register skill
    pub fn register(&mut self, skill: Skill) -> Result<(), SkillsError> {
        if self.get(&skill.id).is_some() {
            return Err(SkillsError::Install(format!("Skill {} already installed", skill.id)));
        }
        
        self.skills.push(skill);
        self.save_registry()?;
        
        Ok(())
    }
    
    /// Unregister skill
    pub fn unregister(&mut self, id: &str) -> Result<(), SkillsError> {
        self.skills.retain(|s| s.id != id);
        self.save_registry()?;
        
        // Remove skill directory
        let skill_path = self.registry_path.join(id);
        if skill_path.exists() {
            std::fs::remove_dir_all(skill_path)?;
        }
        
        Ok(())
    }
    
    /// Get skill directory
    pub fn skill_path(&self, id: &str) -> PathBuf {
        self.registry_path.join(id)
    }
}

impl Default for SkillsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry entry
#[derive(Debug, Serialize, Deserialize)]
struct RegistryEntry {
    id: String,
    metadata: crate::skill::SkillMetadata,
    source: crate::skill::SkillSource,
}
