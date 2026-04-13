//! ─── SENTIENT FORGE ───
//!
//! Verilerden otomatik arac uretimi.
//! n8n workflow'lari ve Python script generatoru.

pub mod templates;
pub mod generators;
pub mod validators;
pub mod formats;
pub mod forge_ext;

// Re-exports
pub use templates::*;
pub use generators::*;
pub use validators::*;
pub use formats::*;

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Forge - Otomatik kod uretimi
pub struct Forge {
    /// Sablon deposu
    template_library: TemplateLibrary,
    /// Validator
    validator: CodeValidator,
    /// Yapilandirma
    config: ForgeConfig,
}

/// Uretim yapilandirmasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// Cikis dizini
    pub output_dir: String,
    /// Dil versiyonlari
    pub python_version: String,
    pub n8n_version: String,
    /// Formatlama
    pub format_code: bool,
    /// Validation aktif
    pub validate: bool,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            output_dir: "./generated".into(),
            python_version: "3.11".into(),
            n8n_version: "1.0".into(),
            format_code: true,
            validate: true,
        }
    }
}

/// Uretilen arac tipi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolType {
    /// n8n workflow JSON'u
    N8nWorkflow,
    /// Python betigi
    PythonScript,
    /// Node.js modulu
    NodeModule,
    /// Shell betigi
    ShellScript,
    /// GitHub Actions
    GitHubAction,
    /// Docker Compose
    DockerCompose,
}

/// Uretilen arac
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTool {
    /// Arac kimligi
    pub id: uuid::Uuid,
    /// Arac adi
    pub name: String,
    /// Arac tipi
    pub tool_type: ToolType,
    /// Kaynak veri ozeti
    pub source_summary: String,
    /// Uretilen kod
    pub code: String,
    /// Meta bilgiler
    pub metadata: HashMap<String, String>,
    /// Uretilme zamani
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Validation sonucu
    pub validation_result: Option<ValidationResult>,
}

/// Uretim istegi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeRequest {
    /// Istek ID
    pub id: uuid::Uuid,
    /// Arac adi
    pub name: String,
    /// Arac tipi
    pub tool_type: ToolType,
    /// Kaynak veri
    pub source_data: serde_json::Value,
    /// Aciklama
    pub description: String,
    /// Ek parametreler
    pub parameters: HashMap<String, String>,
}

impl Forge {
    /// Yeni Forge olustur
    pub fn new(config: ForgeConfig) -> Self {
        Self {
            template_library: TemplateLibrary::default(),
            validator: CodeValidator::new(),
            config,
        }
    }
    
    /// Arac uret
    pub async fn forge(&self, request: ForgeRequest) -> SENTIENTResult<GeneratedTool> {
        log::info!("[Forge] '{}' uretiliyor ({:?})", request.name, request.tool_type);
        
        // Generator sec
        let generator = match request.tool_type {
            ToolType::N8nWorkflow => generators::create_n8n_generator(),
            ToolType::PythonScript => generators::create_python_generator(),
            ToolType::NodeModule => generators::create_node_generator(),
            ToolType::ShellScript => generators::create_shell_generator(),
            ToolType::GitHubAction => generators::create_github_action_generator(),
            ToolType::DockerCompose => generators::create_docker_generator(),
        };
        
        // Kod uret
        let code = generator.generate(&request)?;
        
        // Validation
        let validation_result = if self.config.validate {
            Some(self.validator.validate(&request.tool_type, &code)?)
        } else {
            None
        };
        
        // Kaynak ozeti
        let source_summary = summarize_source(&request.source_data);
        
        Ok(GeneratedTool {
            id: uuid::Uuid::new_v4(),
            name: request.name,
            tool_type: request.tool_type,
            source_summary,
            code,
            metadata: request.parameters,
            generated_at: chrono::Utc::now(),
            validation_result,
        })
    }
    
    /// n8n workflow'u kaydet
    pub async fn save_n8n_workflow(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        if tool.tool_type != ToolType::N8nWorkflow {
            return Err(SENTIENTError::General("Bu bir n8n workflow'u degil".into()));
        }
        
        let filename = format!("{}.json", tool.name);
        let path = format!("{}/workflows/{}", self.config.output_dir, filename);
        
        // Dizin olustur
        std::fs::create_dir_all(format!("{}/workflows", self.config.output_dir))?;
        
        // Yaz
        std::fs::write(&path, &tool.code)?;
        
        log::info!("[Forge] n8n workflow kaydedildi: {}", path);
        Ok(path)
    }
    
    /// Python script kaydet
    pub async fn save_python_script(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        if tool.tool_type != ToolType::PythonScript {
            return Err(SENTIENTError::General("Bu bir Python scripti degil".into()));
        }
        
        let filename = format!("{}.py", tool.name);
        let path = format!("{}/scripts/{}", self.config.output_dir, filename);
        
        // Dizin olustur
        std::fs::create_dir_all(format!("{}/scripts", self.config.output_dir))?;
        
        // Yaz
        std::fs::write(&path, &tool.code)?;
        
        // Calistirilabilir yap
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
        }
        
        log::info!("[Forge] Python script kaydedildi: {}", path);
        Ok(path)
    }
}

fn summarize_source(data: &serde_json::Value) -> String {
    match data {
        serde_json::Value::Array(arr) => format!("{} oge iceren liste", arr.len()),
        serde_json::Value::Object(obj) => format!("{} alan iceren obje", obj.len()),
        serde_json::Value::String(s) => format!("Metin: {} karakter", s.len()),
        _ => "Bilinmeyen veri tipi".into(),
    }
}

/// Kayit yontemi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Dosya
    File,
    /// stdout
    Stdout,
    /// HTTP POST
    HttpPost { url: String },
    /// GitHub'a push
    GitHubPush { repo: String, branch: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_forge_config_default() {
        let config = ForgeConfig::default();
        assert_eq!(config.python_version, "3.11");
    }
    
    #[test]
    fn test_forge_new() {
        let forge = Forge::new(ForgeConfig::default());
        assert_eq!(forge.config.output_dir, "./generated");
    }
}
