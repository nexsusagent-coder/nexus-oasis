//! ═══════════════════════════════════════════════════════════════════════════════
//!  MODULE GENERATOR
//! ═══════════════════════════════════════════════════════════════════════════════

use std::fs;
use std::path::PathBuf;

/// Generate sonucu
#[derive(Debug, Clone)]
pub struct GenerateResult {
    pub files_created: Vec<PathBuf>,
}

/// Module Generator
pub struct ModuleGenerator;

impl ModuleGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Yeni modül oluştur
    pub fn generate(&self, name: &str, module_type: &str) -> anyhow::Result<GenerateResult> {
        let mut files_created = Vec::new();
        
        // Crate dizini
        let crate_dir = PathBuf::from("crates").join(format!("sentient_{}", name));
        let src_dir = crate_dir.join("src");
        
        fs::create_dir_all(&src_dir)?;
        
        // Cargo.toml
        let cargo_toml = crate_dir.join("Cargo.toml");
        let cargo_content = self.generate_cargo_toml(name, module_type);
        fs::write(&cargo_toml, cargo_content)?;
        files_created.push(cargo_toml);
        
        // lib.rs
        let lib_rs = src_dir.join("lib.rs");
        let lib_content = self.generate_lib_rs(name, module_type);
        fs::write(&lib_rs, lib_content)?;
        files_created.push(lib_rs);
        
        // Modül tipine göre ek dosyalar
        match module_type {
            "api" => {
                // API modülü için handler.rs
                let handler = src_dir.join("handler.rs");
                fs::write(&handler, self.generate_handler_rs(name))?;
                files_created.push(handler);
            }
            "tool" => {
                // Tool modülü için tools.rs
                let tools = src_dir.join("tools.rs");
                fs::write(&tools, self.generate_tools_rs(name))?;
                files_created.push(tools);
            }
            "agent" => {
                // Agent modülü için agent.rs
                let agent = src_dir.join("agent.rs");
                fs::write(&agent, self.generate_agent_rs(name))?;
                files_created.push(agent);
            }
            _ => {}
        }
        
        // Workspace Cargo.toml'a ekle
        self.add_to_workspace(name)?;
        
        Ok(GenerateResult { files_created })
    }
    
    /// Cargo.toml içeriği
    fn generate_cargo_toml(&self, name: &str, module_type: &str) -> String {
        let dependencies = match module_type {
            "api" => r#"
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
axum = "0.7"
tower = "0.4"

sentient_common = { path = "../sentient_common" }
"#,
            "tool" => r#"
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }

sentient_common = { path = "../sentient_common" }
"#,
            "agent" => r#"
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }

sentient_common = { path = "../sentient_common" }
sentient_core = { path = "../sentient_core" }
"#,
            _ => r#"
tokio = { workspace = true }
serde = { workspace = true }

sentient_common = { path = "../sentient_common" }
"#,
        };
        
        format!(
            r#"[package]
name = "sentient_{}"
version = "0.1.0"
edition = "2021"
description = "SENTIENT {} module"
authors = ["SENTIENT Core Team"]

[lib]
name = "sentient_{}"
path = "src/lib.rs"

[dependencies]{}
"#,
            name,
            name,
            name,
            dependencies
        )
    }
    
    /// lib.rs içeriği
    fn generate_lib_rs(&self, name: &str, module_type: &str) -> String {
        let type_specific = match module_type {
            "api" => "\npub mod handler;\n",
            "tool" => "\npub mod tools;\n",
            "agent" => "\npub mod agent;\n",
            _ => ""
        };
        
        format!(
            r#"//! ═════════════════════════════════════════════════════════════════
//!  SENTIENT {} MODULE
//! ═════════════════════════════════════════════════════════════════

//! {} modülü
//! 
//! Bu modül SENTIENT ekosisteminin bir parçasıdır.

{}

/// Modül yapılandırması
pub struct Config {{
    /// Modül adı
    pub name: String,
}}

impl Default for Config {{
    fn default() -> Self {{
        Self {{
            name: "{}".to_string(),
        }}
    }}
}}

impl Config {{
    pub fn new() -> Self {{
        Self::default()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_config() {{
        let config = Config::new();
        assert_eq!(config.name, "{}");
    }}
}}
"#,
            name.to_uppercase(),
            name,
            type_specific,
            name,
            name
        )
    }
    
    /// handler.rs içeriği
    fn generate_handler_rs(&self, name: &str) -> String {
        format!(
            r#"//! API Handler

use axum::{{
    routing::get,
    Router,
    Json,
    extract::State,
}};

/// Handler state
#[derive(Clone)]
pub struct AppState {{
    // State fields
}}

/// Router oluştur
pub fn create_router() -> Router<AppState> {{
    Router::new()\n        .route("/health", get(health))
        .route("/{}/info", get(info))
}}

async fn health() -> &'static str {{
    "OK"
}}

async fn info(State(_state): State<AppState>) -> Json<serde_json::Value> {{
    Json(serde_json::json!({{
        "module": "{}",
        "status": "active"
    }}))
}}
"#,
            name, name
        )
    }
    
    /// tools.rs içeriği
    fn generate_tools_rs(&self, name: &str) -> String {
        format!(
            r#"//! Tools

use serde_json::Value;

/// Tool tanımı
pub struct Tool {{
    pub name: String,
    pub description: String,
    pub parameters: Vec<String>,
}}

impl Tool {{
    pub fn new(name: &str, description: &str) -> Self {{
        Self {{
            name: name.to_string(),
            description: description.to_string(),
            parameters: Vec::new(),
        }}
    }}
    
    /// Tool'u çalıştır
    pub async fn execute(&self, _input: Value) -> Result<Value, String> {{
        Ok(serde_json::json!({{
            "tool": self.name,
            "result": "executed"
        }}))
    }}
}}

/// Kullanılabilir tool'lar
pub fn get_tools() -> Vec<Tool> {{
    vec![
        Tool::new("example", "Example tool for {}", name),
    ]
}}
"#,
            name
        )
    }
    
    /// agent.rs içeriği
    fn generate_agent_rs(&self, name: &str) -> String {
        format!(
            r#"//! Agent

use std::sync::Arc;
use tokio::sync::RwLock;

/// Agent durumu
#[derive(Debug, Clone)]
pub enum AgentState {{
    Idle,
    Running,
    Completed,
    Failed,
}}

/// Agent
pub struct Agent {{
    name: String,
    state: Arc<RwLock<AgentState>>,
}}

impl Agent {{
    pub fn new(name: &str) -> Self {{
        Self {{
            name: name.to_string(),
            state: Arc::new(RwLock::new(AgentState::Idle)),
        }}
    }}
    
    /// Agent'ı başlat
    pub async fn start(&self, task: &str) -> Result<String, String> {{
        let mut state = self.state.write().await;
        *state = AgentState::Running;
        
        // Task işleme
        let result = format!("Task completed: {{}}", task);
        
        *state = AgentState::Completed;
        Ok(result)
    }}
    
    /// Durumu al
    pub async fn get_state(&self) -> AgentState {{
        self.state.read().await.clone()
    }}
}}

/// {} Agent
pub fn create_{}_agent() -> Agent {{
    Agent::new("{}")
}}
"#,
            name, name, name
        )
    }
    
    /// Workspace'e ekle
    fn add_to_workspace(&self, name: &str) -> anyhow::Result<()> {
        let workspace_toml = PathBuf::from("Cargo.toml");
        
        if workspace_toml.exists() {
            let content = fs::read_to_string(&workspace_toml)?;
            
            // Zaten ekli mi kontrol et
            if content.contains(&format!("sentient_{}", name)) {
                return Ok(());
            }
            
            // members listesine ekle
            let new_content = content.replace(
                "members = [",
                &format!("members = [\n    \"crates/sentient_{}\",", name)
            );
            
            fs::write(&workspace_toml, new_content)?;
        }
        
        Ok(())
    }
}

impl Default for ModuleGenerator {
    fn default() -> Self {
        Self::new()
    }
}
