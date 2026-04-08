//! ═══════════════════════════════════════════════════════════════════════════════
//!  FILE WRITE TOOL
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FileWriteTool {
    policy: SovereignPolicy,
}

impl FileWriteTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self { policy }
    }
    
    pub fn default_tool() -> Self {
        Self::new(SovereignPolicy::developer())
    }
    
    fn resolve_path(&self, base: &PathBuf, candidate: &str) -> PathBuf {
        let path = PathBuf::from(candidate);
        
        let path = if path.starts_with("~") {
            if let Some(home) = std::env::var_os("HOME") {
                PathBuf::from(home).join(path.strip_prefix("~").unwrap_or(&path))
            } else {
                path
            }
        } else {
            path
        };
        
        if path.is_absolute() { path } else { base.join(path) }
    }
    
    pub fn write_file(
        &self,
        path: &str,
        content: &str,
        create_dirs: bool,
        base_dir: &PathBuf,
    ) -> crate::error::HandsResult<serde_json::Value> {
        let resolved = self.resolve_path(base_dir, path);
        let path_str = resolved.to_str().unwrap_or(path);
        
        // SOVEREIGN doğrulama (write=true)
        self.policy.validate_file_access(path_str, true)?;
        
        if create_dirs {
            if let Some(parent) = resolved.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }
        
        std::fs::write(&resolved, content)?;
        
        Ok(serde_json::json!({
            "message": format!("Dosya yazıldı: {}", path_str),
            "bytes_written": content.len(),
            "path": path_str
        }))
    }
}

#[async_trait]
impl SentientTool for FileWriteTool {
    fn name(&self) -> &str { "write_file" }
    
    fn description(&self) -> &str {
        "Text dosyası oluşturur veya üzerine yazar"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::FileSystem }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("path", "string", true, "Yazılacak dosya yolu"),
            ToolParameter::new("content", "string", true, "Dosya içeriği"),
            ToolParameter::with_default("create_directories", "boolean", "Dizin oluştur", serde_json::json!(true)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let path = params.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let create_dirs = params.get("create_directories").and_then(|v| v.as_bool()).unwrap_or(true);
        
        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        
        match self.write_file(&path, &content, create_dirs, &base_dir) {
            Ok(value) => SentientToolResult::success_with_data("Dosya yazıldı", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for FileWriteTool {
    fn default() -> Self { Self::default_tool() }
}
