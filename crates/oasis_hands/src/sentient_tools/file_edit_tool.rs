//! ═══════════════════════════════════════════════════════════════════════════════
//!  FILE EDIT TOOL
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FileEditTool {
    policy: SovereignPolicy,
}

impl FileEditTool {
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
    
    pub fn edit_file(
        &self,
        path: &str,
        old_str: &str,
        new_str: &str,
        replace_all: bool,
        base_dir: &PathBuf,
    ) -> crate::error::HandsResult<serde_json::Value> {
        let resolved = self.resolve_path(base_dir, path);
        let path_str = resolved.to_str().unwrap_or(path);
        
        // SOVEREIGN doğrulama
        self.policy.validate_file_access(path_str, true)?;
        
        if !resolved.exists() {
            return Ok(serde_json::json!({
                "message": format!("Dosya bulunamadı: {}", path_str),
                "replacements": 0,
                "is_error": true
            }));
        }
        
        let original = std::fs::read_to_string(&resolved)?;
        
        if !original.contains(old_str) {
            return Ok(serde_json::json!({
                "message": "old_str dosyada bulunamadı",
                "replacements": 0,
                "is_error": true
            }));
        }
        
        let (updated, count) = if replace_all {
            let count = original.matches(old_str).count();
            (original.replace(old_str, new_str), count)
        } else {
            (original.replacen(old_str, new_str, 1), 1)
        };
        
        std::fs::write(&resolved, &updated)?;
        
        Ok(serde_json::json!({
            "message": format!("{} değişiklik yapıldı: {}", count, path_str),
            "replacements": count,
            "is_error": false
        }))
    }
}

#[async_trait]
impl SentientTool for FileEditTool {
    fn name(&self) -> &str { "edit_file" }
    
    fn description(&self) -> &str {
        "Mevcut dosyada string değiştirme yapar"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::FileSystem }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("path", "string", true, "Düzenlenecek dosya yolu"),
            ToolParameter::new("old_str", "string", true, "Değiştirilecek metin"),
            ToolParameter::new("new_str", "string", true, "Yeni metin"),
            ToolParameter::with_default("replace_all", "boolean", "Tümünü değiştir", serde_json::json!(false)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let path = params.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let old_str = params.get("old_str").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let new_str = params.get("new_str").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let replace_all = params.get("replace_all").and_then(|v| v.as_bool()).unwrap_or(false);
        
        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        
        match self.edit_file(&path, &old_str, &new_str, replace_all, &base_dir) {
            Ok(value) => SentientToolResult::success_with_data("Dosya düzenlendi", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for FileEditTool {
    fn default() -> Self { Self::default_tool() }
}
