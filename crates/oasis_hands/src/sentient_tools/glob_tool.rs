//! ═══════════════════════════════════════════════════════════════════════════════
//!  GLOB TOOL
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use glob::glob;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct GlobTool {
    policy: SovereignPolicy,
    max_results: usize,
}

impl GlobTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self {
            policy,
            max_results: 1000,
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new(SovereignPolicy::developer())
    }
    
    pub fn search(
        &self,
        pattern: &str,
        base_path: Option<&str>,
        base_dir: &PathBuf,
    ) -> crate::error::HandsResult<serde_json::Value> {
        let search_dir = if let Some(p) = base_path {
            let resolved = PathBuf::from(p);
            if resolved.is_absolute() { resolved } else { base_dir.join(p) }
        } else {
            base_dir.clone()
        };
        
        // SOVEREIGN doğrulama
        self.policy.validate_file_access(search_dir.to_str().unwrap_or(""), false)?;
        
        let full_pattern = if pattern.starts_with('/') || pattern.starts_with('~') {
            pattern.to_string()
        } else {
            format!("{}/{}", search_dir.display(), pattern)
        };
        
        let mut results = Vec::new();
        
        match glob(&full_pattern) {
            Ok(paths) => {
                for entry in paths.take(self.max_results) {
                    if let Ok(path) = entry {
                        results.push(path.display().to_string());
                    }
                }
            }
            Err(e) => {
                return Ok(serde_json::json!({
                    "files": [],
                    "total": 0,
                    "is_error": true,
                    "error_message": format!("Pattern hatası: {}", e)
                }));
            }
        }
        
        Ok(serde_json::json!({
            "files": results,
            "total": results.len(),
            "is_error": false
        }))
    }
}

#[async_trait]
impl SentientTool for GlobTool {
    fn name(&self) -> &str { "glob" }
    
    fn description(&self) -> &str {
        "Dosya arama (glob pattern desteği)"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::FileSystem }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("pattern", "string", true, "Arama pattern'i (örn: **/*.rs)"),
            ToolParameter::new("path", "string", false, "Başlangıç dizini"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let pattern = params.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("**/*")
            .to_string();
        let base_path = params.get("path").and_then(|v| v.as_str()).map(|s| s.to_string());
        
        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        
        match self.search(&pattern, base_path.as_deref(), &base_dir) {
            Ok(value) => SentientToolResult::success_with_data("Arama tamamlandı", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for GlobTool {
    fn default() -> Self { Self::default_tool() }
}
