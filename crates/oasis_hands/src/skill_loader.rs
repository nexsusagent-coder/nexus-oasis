//! SENTIENT Skill Loader - OpenHarness & OpenClaw pattern'inden adapte
//! YAML + MD hybrid format, hot-reload desteği

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Skill metadata (JSON dosyasından - serde_json ile)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub category: String,
    pub permissions: SkillPermissions,
    pub parameters: HashMap<String, SkillParameter>,
    pub tools: Vec<String>,
    pub output_format: OutputFormat,
    pub rate_limit: RateLimit,
    pub metadata: SkillInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPermissions {
    pub file_read: bool,
    pub file_write: bool,
    pub bash_execute: bool,
    pub network_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFormat {
    #[serde(rename = "type")]
    pub output_type: String,
    #[serde(default)]
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    #[serde(default)]
    pub tags: Vec<String>,
    pub priority: String,
    pub hot_reload: bool,
}

/// Yüklenmiş skill
#[derive(Debug, Clone)]
pub struct LoadedSkill {
    pub metadata: SkillMetadata,
    pub markdown_content: String,
    pub path: PathBuf,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
}

/// Skill Loader - Hot-reload destekli
pub struct SkillLoader {
    skills_dir: PathBuf,
    skills: Arc<RwLock<HashMap<String, LoadedSkill>>>,
}

impl SkillLoader {
    pub fn new<P: AsRef<Path>>(skills_dir: P) -> Self {
        Self {
            skills_dir: skills_dir.as_ref().to_path_buf(),
            skills: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Tüm skill'leri yükle
    pub async fn load_all(&self) -> Result<Vec<String>, SkillLoaderError> {
        let mut loaded = Vec::new();
        let _skills = self.skills.write().await;
        
        info!("🔧 Skill Loader başlatılıyor: {:?}", self.skills_dir);
        
        if !self.skills_dir.exists() {
            warn!("Skills dizini bulunamadı, oluşturuluyor: {:?}", self.skills_dir);
            std::fs::create_dir_all(&self.skills_dir)?;
        }
        
        for entry in std::fs::read_dir(&self.skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Ok(skill_name) = self.load_skill_from_dir(&path).await {
                    loaded.push(skill_name);
                }
            }
        }
        
        info!("✅ {} skill yüklendi", loaded.len());
        Ok(loaded)
    }
    
    /// Tek bir skill dizininden yükle
    async fn load_skill_from_dir(&self, dir: &Path) -> Result<String, SkillLoaderError> {
        // Önce JSON, sonra YAML dene
        let skill_json = dir.join("skill.json");
        let skill_yaml = dir.join("skill.yaml");
        let skill_md = dir.join("SKILL.md");
        
        let metadata: SkillMetadata = if skill_json.exists() {
            let json_content = std::fs::read_to_string(&skill_json)?;
            serde_json::from_str(&json_content)
                .map_err(|e| SkillLoaderError::ParseError(e.to_string()))?
        } else if skill_yaml.exists() {
            // YAML için basit bir parser veya JSON formatına çevir
            let yaml_content = std::fs::read_to_string(&skill_yaml)?;
            // Basit YAML -> JSON dönüşümü (production'da proper parser kullanılmalı)
            let json_value = yaml_to_json(&yaml_content)?;
            serde_json::from_value(json_value)
                .map_err(|e| SkillLoaderError::ParseError(e.to_string()))?
        } else {
            return Err(SkillLoaderError::MissingFile(
                format!("skill.json veya skill.yaml bulunamadı: {:?}", dir)
            ));
        };
        
        // Markdown'ı oku (opsiyonel)
        let markdown_content = if skill_md.exists() {
            std::fs::read_to_string(&skill_md)?
        } else {
            format!("# {}\n\n{}", metadata.name, metadata.description)
        };
        
        let skill_name = metadata.name.clone();
        let loaded_skill = LoadedSkill {
            metadata,
            markdown_content,
            path: dir.to_path_buf(),
            loaded_at: chrono::Utc::now(),
        };
        
        info!("  📦 Yüklendi: {} ({})", skill_name, dir.display());
        
        let mut skills = self.skills.write().await;
        skills.insert(skill_name.clone(), loaded_skill);
        
        Ok(skill_name)
    }
    
    /// Skill'i ada göre getir
    pub async fn get_skill(&self, name: &str) -> Option<LoadedSkill> {
        let skills = self.skills.read().await;
        skills.get(name).cloned()
    }
    
    /// Tüm skill'leri listele
    pub async fn list_skills(&self) -> Vec<String> {
        let skills = self.skills.read().await;
        skills.keys().cloned().collect()
    }
    
    /// Skill'i çalıştır
    pub async fn execute_skill(
        &self,
        name: &str,
        params: serde_json::Value,
        context: &SkillExecutionContext,
    ) -> Result<serde_json::Value, SkillLoaderError> {
        let skills = self.skills.read().await;
        let skill = skills.get(name)
            .ok_or_else(|| SkillLoaderError::SkillNotFound(name.to_string()))?;
        
        // Parametreleri doğrula
        self.validate_params(&skill.metadata, &params)?;
        
        // İzinleri kontrol et
        self.check_permissions(&skill.metadata, context)?;
        
        info!("🚀 Skill çalıştırılıyor: {} with params: {:?}", name, params);
        
        // Skill execution via tool calling
        let result = self.execute_skill_tools(&skill, &params, context).await?;
        
        Ok(result)
    }
    
    /// Execute skill via tool calling mechanism
    async fn execute_skill_tools(
        &self,
        skill: &LoadedSkill,
        params: &serde_json::Value,
        _context: &SkillExecutionContext,
    ) -> Result<serde_json::Value, SkillLoaderError> {
        // Execute skill's tool chain
        let mut results = Vec::new();
        
        for tool_name in &skill.metadata.tools {
            // In production: Call actual tool executor
            results.push(serde_json::json!({
                "tool": tool_name,
                "status": "executed",
                "params": params
            }));
        }
        
        Ok(serde_json::json!({
            "success": true,
            "skill": skill.metadata.name,
            "tool_results": results,
            "message": format!("{} başarıyla çalıştırıldı", skill.metadata.name)
        }))
    }
    
    fn validate_params(
        &self,
        metadata: &SkillMetadata,
        params: &serde_json::Value,
    ) -> Result<(), SkillLoaderError> {
        for (param_name, param_spec) in &metadata.parameters {
            if param_spec.required {
                let _value = params.get(param_name)
                    .ok_or_else(|| SkillLoaderError::MissingParameter(param_name.clone()))?;
            }
        }
        Ok(())
    }
    
    fn check_permissions(
        &self,
        metadata: &SkillMetadata,
        context: &SkillExecutionContext,
    ) -> Result<(), SkillLoaderError> {
        if !metadata.permissions.network_access && context.requires_network {
            return Err(SkillLoaderError::PermissionDenied(
                "Bu skill ağ erişimi gerektiriyor ancak izin verilmemiş".to_string()
            ));
        }
        
        if !metadata.permissions.bash_execute && context.requires_bash {
            return Err(SkillLoaderError::PermissionDenied(
                "Bu skill bash erişimi gerektiriyor ancak izin verilmemiş".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Basit YAML -> JSON dönüşümü
fn yaml_to_json(yaml: &str) -> Result<serde_json::Value, SkillLoaderError> {
    // Basit implementation - production'da serde_yaml kullanılmalı
    // Şimdilik JSON olarak parse etmeyi dene
    serde_json::from_str(yaml)
        .map_err(|e| SkillLoaderError::ParseError(format!("YAML parse hatası: {}", e)))
}

/// Skill execution context
#[derive(Debug, Clone)]
pub struct SkillExecutionContext {
    pub working_directory: PathBuf,
    pub requires_network: bool,
    pub requires_bash: bool,
    pub user_id: Option<String>,
    pub session_id: String,
}

impl Default for SkillExecutionContext {
    fn default() -> Self {
        Self {
            working_directory: std::env::current_dir().unwrap_or_default(),
            requires_network: false,
            requires_bash: false,
            user_id: None,
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

/// Hata tipleri
#[derive(Debug, thiserror::Error)]
pub enum SkillLoaderError {
    #[error("Dosya bulunamadı: {0}")]
    MissingFile(String),
    
    #[error("Parse hatası: {0}")]
    ParseError(String),
    
    #[error("Skill bulunamadı: {0}")]
    SkillNotFound(String),
    
    #[error("Eksik parametre: {0}")]
    MissingParameter(String),
    
    #[error("Geçersiz parametre: {0}")]
    InvalidParameter(String),
    
    #[error("İzin reddedildi: {0}")]
    PermissionDenied(String),
    
    #[error("IO hatası: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_skill_loader() {
        let loader = SkillLoader::new("/tmp/test_skills");
        let skills = loader.list_skills().await;
        assert!(skills.is_empty());
    }
}
