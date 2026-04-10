//! Skill Loader - DeerFlow SKILL.md format parser
//!
//! DeerFlow skill format:
//! ```markdown
//! ---
//! name: deep-research
//! description: Deep research skill
//! ---
//! 
//! # Deep Research Skill
//! Content here...
//! ```

use crate::{Skill, SkillMetadata, SkillError, SkillResult, SkillCategory, SkillTrigger, TriggerType};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use tracing::{info, warn, debug};

/// Skill Loader
pub struct SkillLoader {
    /// Skill dizinleri
    skill_dirs: Vec<PathBuf>,
    
    /// Yüklenen skill'ler
    loaded_skills: Vec<Skill>,
}

impl SkillLoader {
    /// Yeni loader oluştur
    pub fn new() -> Self {
        Self {
            skill_dirs: Vec::new(),
            loaded_skills: Vec::new(),
        }
    }
    
    /// Skill dizini ekle
    pub fn add_skill_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.skill_dirs.push(path.as_ref().to_path_buf());
        self
    }
    
    /// Varsayılan dizinleri ekle
    pub fn with_default_dirs(self) -> Self {
        let default_paths = [
            "./skills",
            "./integrations/skills/deerflow-skills",
            "./crates/sentient_skills/src/skills",
        ];
        
        let mut result = self;
        for path in default_paths {
            if Path::new(path).exists() {
                result = result.add_skill_dir(path);
            }
        }
        result
    }
    
    /// Tüm skill'leri yükle
    pub fn load_all(&mut self) -> SkillResult<usize> {
        let mut count = 0;
        
        let dirs = self.skill_dirs.clone();
        for dir in &dirs {
            match self.load_from_dir(dir) {
                Ok(n) => {
                    info!("Loaded {} skills from {:?}", n, dir);
                    count += n;
                }
                Err(e) => {
                    warn!("Failed to load skills from {:?}: {}", dir, e);
                }
            }
        }
        
        Ok(count)
    }
    
    /// Dizinden skill'leri yükle
    pub fn load_from_dir(&mut self, dir: &Path) -> SkillResult<usize> {
        let mut count = 0;
        
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // SKILL.md veya skill.md dosyalarını bul
            if let Some(filename) = path.file_name() {
                let filename = filename.to_string_lossy().to_lowercase();
                if filename == "skill.md" {
                    match self.load_skill_file(path) {
                        Ok(skill) => {
                            debug!("Loaded skill: {}", skill.metadata.name);
                            self.loaded_skills.push(skill);
                            count += 1;
                        }
                        Err(e) => {
                            warn!("Failed to load skill from {:?}: {}", path, e);
                        }
                    }
                }
            }
        }
        
        Ok(count)
    }
    
    /// Tek bir skill dosyası yükle
    pub fn load_skill_file(&self, path: &Path) -> SkillResult<Skill> {
        let content = std::fs::read_to_string(path)?;
        self.parse_skill(&content, Some(path))
    }
    
    /// Skill içeriğini parse et
    pub fn parse_skill(&self, content: &str, source_path: Option<&Path>) -> SkillResult<Skill> {
        // Frontmatter'ı ayıkla (--- arası)
        let (frontmatter, body) = self.extract_frontmatter(content)?;
        
        // Metadata'yı parse et
        let metadata: SkillMetadata = if frontmatter.is_empty() {
            // Frontmatter yoksa dosya adından metadata oluştur
            let name = source_path
                .and_then(|p| p.parent())
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            
            SkillMetadata::new(&name, "No description")
        } else {
            // YAML frontmatter'ı parse et
            let mut metadata: SkillMetadata = serde_yaml::from_str(frontmatter)
                .map_err(|e| SkillError::ParseError(format!("YAML parse error: {}", e)))?;
            
            // DeerFlow formatından category çıkar
            if metadata.category == SkillCategory::Other {
                metadata.category = self.infer_category(&metadata.name, body);
            }
            
            // Otomatik trigger ekle
            if metadata.triggers.is_empty() {
                metadata.triggers = self.infer_triggers(&metadata.name, &metadata.description);
            }
            
            metadata
        };
        
        // Skill oluştur
        let mut skill = Skill::new(metadata, body);
        skill.loaded_at = chrono::Utc::now();
        skill.source_path = source_path.map(|p| p.to_string_lossy().to_string());
        
        Ok(skill)
    }
    
    /// Frontmatter ayıkla
    fn extract_frontmatter<'a>(&self, content: &'a str) -> SkillResult<(&'a str, &'a str)> {
        let content = content.trim_start();
        
        if !content.starts_with("---") {
            return Ok(("", content));
        }
        
        // İlk --- sonrası
        let after_first = &content[3..];
        
        // İkinci --- bul
        if let Some(end_pos) = after_first.find("\n---") {
            let frontmatter = after_first[..end_pos].trim();
            let body = after_first[end_pos + 4..].trim();
            Ok((frontmatter, body))
        } else if let Some(end_pos) = after_first.find("---") {
            let frontmatter = after_first[..end_pos].trim();
            let body = after_first[end_pos + 3..].trim();
            Ok((frontmatter, body))
        } else {
            Ok(("", content))
        }
    }
    
    /// Skill adından kategori çıkar
    fn infer_category(&self, name: &str, body: &str) -> SkillCategory {
        let name_lower = name.to_lowercase();
        let body_lower = body.to_lowercase();
        
        // İsim bazlı kategori
        if name_lower.contains("research") || name_lower.contains("search") {
            return SkillCategory::Research;
        }
        if name_lower.contains("ppt") || name_lower.contains("presentation") {
            return SkillCategory::ContentGeneration;
        }
        if name_lower.contains("code") || name_lower.contains("coding") {
            return SkillCategory::Coding;
        }
        if name_lower.contains("data") || name_lower.contains("analysis") {
            return SkillCategory::DataAnalysis;
        }
        if name_lower.contains("video") || name_lower.contains("image") || name_lower.contains("podcast") {
            return SkillCategory::MediaGeneration;
        }
        if name_lower.contains("deploy") || name_lower.contains("automation") {
            return SkillCategory::Automation;
        }
        
        // İçerik bazlı kategori
        if body_lower.contains("web search") || body_lower.contains("research") {
            return SkillCategory::Research;
        }
        if body_lower.contains("generate") || body_lower.contains("create content") {
            return SkillCategory::ContentGeneration;
        }
        
        SkillCategory::Other
    }
    
    /// Otomatik trigger oluştur
    fn infer_triggers(&self, name: &str, description: &str) -> Vec<SkillTrigger> {
        let mut triggers = Vec::new();
        
        // İsimden keyword trigger
        triggers.push(SkillTrigger {
            trigger_type: TriggerType::Keyword,
            pattern: name.to_string(),
            priority: 1,
        });
        
        // Description'dan keyword'ler çıkar
        let keywords: Vec<&str> = description
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .take(3)
            .collect();
        
        for keyword in keywords {
            triggers.push(SkillTrigger {
                trigger_type: TriggerType::Keyword,
                pattern: keyword.to_lowercase(),
                priority: 0,
            });
        }
        
        triggers
    }
    
    /// Yüklenen skill'leri al
    pub fn get_skills(&self) -> &[Skill] {
        &self.loaded_skills
    }
    
    /// Skill sayısı
    pub fn skill_count(&self) -> usize {
        self.loaded_skills.len()
    }
}

impl Default for SkillLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_skill_with_frontmatter() {
        let loader = SkillLoader::new();
        let content = r#"---
name: test-skill
description: A test skill
version: 1.0.0
---

# Test Skill

This is the skill content.
"#;
        
        let skill = loader.parse_skill(content, None).expect("operation failed");
        
        assert_eq!(skill.metadata.name, "test-skill");
        assert_eq!(skill.metadata.description, "A test skill");
        assert!(skill.content.contains("Test Skill"));
    }
    
    #[test]
    fn test_parse_skill_without_frontmatter() {
        let loader = SkillLoader::new();
        let content = "# Simple Skill\n\nNo frontmatter here.";
        
        let skill = loader.parse_skill(content, Some(Path::new("skills/simple-skill/SKILL.md"))).expect("operation failed");
        
        assert_eq!(skill.metadata.name, "simple-skill");
    }
}
