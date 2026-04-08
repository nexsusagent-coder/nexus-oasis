//! Skill Manager - Skill yönetim ve execution

use crate::{Skill, SkillLoader, SkillResult, SkillCategory};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn};

/// Skill Manager
pub struct SkillManager {
    /// Skill loader
    loader: SkillLoader,
    
    /// Skill registry (name -> skill)
    skills: Arc<RwLock<HashMap<String, Skill>>>,
    
    /// Category index
    category_index: Arc<RwLock<HashMap<SkillCategory, Vec<String>>>>,
}

impl SkillManager {
    /// Yeni manager oluştur
    pub fn new() -> Self {
        Self {
            loader: SkillLoader::new(),
            skills: Arc::new(RwLock::new(HashMap::new())),
            category_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Skill dizini ekle
    pub fn add_skill_dir(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.loader = self.loader.add_skill_dir(path);
        self
    }
    
    /// Varsayılan dizinlerle yükle
    pub fn with_defaults(self) -> Self {
        let mut manager = Self {
            loader: self.loader.with_default_dirs(),
            skills: self.skills,
            category_index: self.category_index,
        };
        
        // Otomatik yükle
        if let Err(e) = manager.load_skills() {
            warn!("Failed to load default skills: {}", e);
        }
        
        manager
    }
    
    /// Tüm skill'leri yükle
    pub fn load_skills(&mut self) -> SkillResult<usize> {
        let count = self.loader.load_all()?;
        
        // Registry'yi güncelle
        let mut skills = self.skills.write();
        let mut category_index = self.category_index.write();
        
        for skill in self.loader.get_skills() {
            let name = skill.metadata.name.clone();
            let category = skill.metadata.category.clone();
            
            // Skill ekle
            skills.insert(name.clone(), skill.clone());
            
            // Category index güncelle
            category_index
                .entry(category)
                .or_insert_with(Vec::new)
                .push(name);
        }
        
        info!("Loaded {} skills into registry", count);
        Ok(count)
    }
    
    /// Skill'i ada göre al
    pub fn get_skill(&self, name: &str) -> Option<Skill> {
        let skills = self.skills.read();
        skills.get(name).cloned()
    }
    
    /// Tüm skill isimlerini al
    pub fn list_skills(&self) -> Vec<String> {
        let skills = self.skills.read();
        skills.keys().cloned().collect()
    }
    
    /// Kategoriye göre skill'leri al
    pub fn get_skills_by_category(&self, category: SkillCategory) -> Vec<Skill> {
        let skills = self.skills.read();
        let category_index = self.category_index.read();
        
        category_index
            .get(&category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| skills.get(name).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Input'a göre eşleşen skill'leri bul
    pub fn find_matching_skills(&self, input: &str) -> Vec<Skill> {
        let skills = self.skills.read();
        
        skills
            .values()
            .filter(|skill| skill.is_triggered_by(input))
            .cloned()
            .collect()
    }
    
    /// En iyi eşleşen skill'i bul
    pub fn find_best_match(&self, input: &str) -> Option<Skill> {
        let matches = self.find_matching_skills(input);
        
        // En yüksek öncelikliyi seç
        matches
            .into_iter()
            .max_by_key(|skill| {
                skill.metadata
                    .triggers
                    .iter()
                    .filter(|_t| skill.is_triggered_by(input))
                    .map(|t| t.priority as i32)
                    .max()
                    .unwrap_or(0)
            })
    }
    
    /// Skill sayısı
    pub fn skill_count(&self) -> usize {
        self.skills.read().len()
    }
    
    /// Kategori istatistikleri
    pub fn category_stats(&self) -> HashMap<SkillCategory, usize> {
        let category_index = self.category_index.read();
        category_index
            .iter()
            .map(|(cat, skills)| (cat.clone(), skills.len()))
            .collect()
    }
    
    /// Skill içeriklerini formatlanmış şekilde al
    pub fn get_formatted_skills(&self) -> String {
        let skills = self.skills.read();
        let mut result = String::new();
        
        for (name, skill) in skills.iter() {
            result.push_str(&format!(
                "## {}\n{}\n\n{}\n\n",
                name,
                skill.metadata.description,
                "-".repeat(40)
            ));
        }
        
        result
    }
}

impl Default for SkillManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global skill manager
static SKILL_MANAGER: once_cell::sync::Lazy<SkillManager> = 
    once_cell::sync::Lazy::new(|| SkillManager::new().with_defaults());

/// Global manager'ı al
pub fn global_skill_manager() -> &'static SkillManager {
    &SKILL_MANAGER
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_manager() {
        let manager = SkillManager::new();
        assert_eq!(manager.skill_count(), 0);
    }
}
