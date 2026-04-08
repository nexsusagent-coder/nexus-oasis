//! ═══════════════════════════════════════════════════════════════════════════════
//!  FILE READ TOOL - OpenHarness Mantığına Sadık
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct FileReadTool {
    policy: SovereignPolicy,
    max_limit: usize,
}

impl FileReadTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self {
            policy,
            max_limit: 2000,
        }
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
    
    pub fn read_file(
        &self,
        path: &str,
        offset: usize,
        limit: usize,
        base_dir: &PathBuf,
    ) -> crate::error::HandsResult<serde_json::Value> {
        let resolved = self.resolve_path(base_dir, path);
        let path_str = resolved.to_str().unwrap_or(path);
        
        // SOVEREIGN doğrulama
        self.policy.validate_file_access(path_str, false)?;
        
        if !resolved.exists() {
            return Ok(serde_json::json!({
                "content": "",
                "total_lines": 0,
                "lines_read": 0,
                "is_error": true,
                "error_message": format!("Dosya bulunamadı: {}", path_str)
            }));
        }
        
        if resolved.is_dir() {
            return Ok(serde_json::json!({
                "content": "",
                "total_lines": 0,
                "lines_read": 0,
                "is_error": true,
                "error_message": format!("Dizin okunamaz: {}", path_str)
            }));
        }
        
        let raw = std::fs::read(&resolved)?;
        if raw.contains(&0x00) {
            return Ok(serde_json::json!({
                "content": "",
                "is_error": true,
                "error_message": format!("Binary dosya text olarak okunamaz: {}", path_str)
            }));
        }
        
        let text = String::from_utf8_lossy(&raw);
        let lines: Vec<&str> = text.lines().collect();
        let total = lines.len();
        
        let limit = limit.min(self.max_limit);
        let offset = offset.min(total);
        let end = (offset + limit).min(total);
        
        let numbered: Vec<String> = lines[offset..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{:>6}\t{}", offset + i + 1, line))
            .collect();
        
        Ok(serde_json::json!({
            "content": numbered.join("\n"),
            "total_lines": total,
            "lines_read": numbered.len(),
            "is_error": false,
            "error_message": null
        }))
    }
}

#[async_trait]
impl SentientTool for FileReadTool {
    fn name(&self) -> &str { "read_file" }
    
    fn description(&self) -> &str {
        "UTF-8 text dosyasını satır numaralı olarak okur"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::FileSystem }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("path", "string", true, "Okunacak dosya yolu"),
            ToolParameter::with_default("offset", "integer", "Başlangıç satırı", serde_json::json!(0)),
            ToolParameter::with_default("limit", "integer", "Okunacak satır", serde_json::json!(200)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let path = params.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let offset = params.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(200) as usize;
        
        let base_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        
        match self.read_file(&path, offset, limit, &base_dir) {
            Ok(value) => SentientToolResult::success_with_data("Dosya okundu", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for FileReadTool {
    fn default() -> Self { Self::default_tool() }
}
