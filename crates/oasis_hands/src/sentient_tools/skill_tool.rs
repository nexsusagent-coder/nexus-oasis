//! ═══════════════════════════════════════════════════════════════════════════════
//!  SKILL TOOL - SENTIENT Skill Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

/// Skill Tool - SENTIENT skill yükleme, arama ve çalıştırma
pub struct SkillTool {
    skills_dir: PathBuf,
    loaded_skills: HashMap<String, LoadedSkill>,
}

/// Yüklenmiş skill
#[derive(Debug, Clone)]
struct LoadedSkill {
    id: String,
    name: String,
    description: String,
    category: String,
    yaml_path: PathBuf,
}

/// Skill aksiyonları
#[derive(Debug, Clone)]
pub enum SkillAction {
    /// Skill yükle
    Load { skill_id: String },
    /// Skill çalıştır
    Execute { skill_id: String, params: HashMap<String, serde_json::Value> },
    /// Skill ara
    Search { query: String, category: Option<String> },
    /// Kategorileri listele
    ListCategories,
    /// Skill listele
    List { category: Option<String> },
    /// Skill bilgisi
    Info { skill_id: String },
}

impl SkillTool {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self {
            skills_dir,
            loaded_skills: HashMap::new(),
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new(PathBuf::from("data/skills"))
    }
    
    /// Skill ara
    pub fn search_skills(&self, query: &str, category: Option<&str>) -> Vec<LoadedSkill> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        // YAML dosyalarında ara
        if let Ok(entries) = std::fs::read_dir(&self.skills_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "yaml").unwrap_or(false) {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        let content_lower = content.to_lowercase();
                        
                        // Kategori kontrolü
                        if let Some(cat) = category {
                            if !content_lower.contains(&format!("category: {}", cat.to_lowercase())) {
                                continue;
                            }
                        }
                        
                        // Query kontrolü
                        if content_lower.contains(&query_lower) {
                            if let Some(skill) = self.parse_skill_yaml(&content, &path) {
                                results.push(skill);
                            }
                        }
                    }
                }
            }
        }
        
        // Alt dizinlerde de ara
        if let Ok(subdirs) = std::fs::read_dir(&self.skills_dir) {
            for subdir in subdirs.flatten() {
                let subdir_path = subdir.path();
                if subdir_path.is_dir() {
                    if let Ok(files) = std::fs::read_dir(&subdir_path) {
                        for file in files.flatten() {
                            let file_path = file.path();
                            if file_path.extension().map(|e| e == "yaml").unwrap_or(false) {
                                if let Ok(content) = std::fs::read_to_string(&file_path) {
                                    let content_lower = content.to_lowercase();
                                    
                                    if let Some(cat) = category {
                                        if !content_lower.contains(&format!("category: {}", cat.to_lowercase())) {
                                            continue;
                                        }
                                    }
                                    
                                    if content_lower.contains(&query_lower) {
                                        if let Some(skill) = self.parse_skill_yaml(&content, &file_path) {
                                            results.push(skill);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        results
    }
    
    /// YAML'den skill parse et
    fn parse_skill_yaml(&self, content: &str, path: &PathBuf) -> Option<LoadedSkill> {
        // Basit YAML parse (serde_yaml olmadan)
        let mut id = String::new();
        let mut name = String::new();
        let mut description = String::new();
        let mut category = String::new();
        
        for line in content.lines() {
            if line.starts_with("id:") {
                id = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("name:") {
                name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("description:") {
                description = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("category:") {
                category = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
        
        if !id.is_empty() {
            Some(LoadedSkill {
                id,
                name,
                description,
                category,
                yaml_path: path.clone(),
            })
        } else {
            None
        }
    }
    
    /// Kategorileri listele
    pub fn list_categories(&self) -> Vec<(String, usize)> {
        let mut categories: HashMap<String, usize> = HashMap::new();
        
        // Tüm YAML dosyalarını tara
        fn scan_dir(dir: &PathBuf, cats: &mut HashMap<String, usize>) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        scan_dir(&path, cats);
                    } else if path.extension().map(|e| e == "yaml").unwrap_or(false) {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            for line in content.lines() {
                                if line.starts_with("category:") {
                                    let cat = line.split(':').nth(1).unwrap_or("unknown").trim().to_string();
                                    *cats.entry(cat).or_insert(0) += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        scan_dir(&self.skills_dir, &mut categories);
        
        let mut result: Vec<_> = categories.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
}

#[async_trait]
impl SentientTool for SkillTool {
    fn name(&self) -> &str { "skill" }
    
    fn description(&self) -> &str {
        "SENTIENT skill yönetimi. Skill yükle, ara, çalıştır ve yönet."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::System }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon: load, execute, search, list_categories, list, info"),
            ToolParameter::new("skill_id", "string", false, "Skill ID"),
            ToolParameter::new("query", "string", false, "Arama sorgusu"),
            ToolParameter::new("category", "string", false, "Kategori filtresi"),
            ToolParameter::new("params", "object", false, "Skill parametreleri"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "search" => {
                let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
                let category = params.get("category").and_then(|v| v.as_str());
                
                let results = self.search_skills(query, category);
                let count = results.len();
                
                SentientToolResult::success_with_data(
                    &format!("{} skill bulundu", count),
                    serde_json::json!({
                        "count": count,
                        "skills": results.iter().take(50).map(|s| serde_json::json!({
                            "id": s.id,
                            "name": s.name,
                            "description": s.description,
                            "category": s.category
                        })).collect::<Vec<_>>()
                    })
                )
            }
            "list_categories" => {
                let categories = self.list_categories();
                let total: usize = categories.iter().map(|(_, c)| c).sum();
                
                SentientToolResult::success_with_data(
                    &format!("{} kategori, {} skill", categories.len(), total),
                    serde_json::json!({
                        "categories": categories.iter().map(|(cat, count)| serde_json::json!({
                            "name": cat,
                            "count": count
                        })).collect::<Vec<_>>()
                    })
                )
            }
            "list" => {
                let category = params.get("category").and_then(|v| v.as_str());
                
                if let Some(cat) = category {
                    let results = self.search_skills("", Some(cat));
                    SentientToolResult::success_with_data(
                        &format!("{} skill '{}' kategorisinde", results.len(), cat),
                        serde_json::json!({
                            "category": cat,
                            "skills": results.iter().take(100).map(|s| serde_json::json!({
                                "id": s.id,
                                "name": s.name,
                                "description": s.description
                            })).collect::<Vec<_>>()
                        })
                    )
                } else {
                    let categories = self.list_categories();
                    SentientToolResult::success_with_data(
                        "Tüm kategoriler",
                        serde_json::json!({ "categories": categories })
                    )
                }
            }
            "info" => {
                let skill_id = params.get("skill_id").and_then(|v| v.as_str()).unwrap_or("");
                let results = self.search_skills(skill_id, None);
                
                if let Some(skill) = results.first() {
                    SentientToolResult::success_with_data(
                        &format!("Skill: {}", skill.name),
                        serde_json::json!({
                            "id": skill.id,
                            "name": skill.name,
                            "description": skill.description,
                            "category": skill.category,
                            "path": skill.yaml_path.to_string_lossy()
                        })
                    )
                } else {
                    SentientToolResult::failure(&format!("Skill bulunamadı: {}", skill_id))
                }
            }
            _ => SentientToolResult::failure(&format!("Bilinmeyen aksiyon: {}", action))
        }
    }
}

impl Clone for SkillTool {
    fn clone(&self) -> Self {
        Self {
            skills_dir: self.skills_dir.clone(),
            loaded_skills: HashMap::new(),
        }
    }
}

impl Default for SkillTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_creation() {
        let tool = SkillTool::default_tool();
        assert_eq!(tool.name(), "skill");
    }
}
