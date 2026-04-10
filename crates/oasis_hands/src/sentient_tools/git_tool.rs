//! ═══════════════════════════════════════════════════════════════════════════════
//!  GIT TOOL - VERSİYON KONTROL ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Git komutlarını güvenli şekilde çalıştırır.
//! Tehlikeli işlemler (force push, history rewrite) engellenir.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;
use std::process::Command;

/// Git aracı - versiyon kontrol işlemleri
pub struct GitTool {
    /// Çalışma dizini
    working_dir: String,
}

impl GitTool {
    /// Yeni Git aracı oluştur
    pub fn new() -> Self {
        Self {
            working_dir: ".".to_string(),
        }
    }
    
    /// Çalışma dizinini ayarla
    pub fn with_working_dir(mut self, dir: &str) -> Self {
        self.working_dir = dir.to_string();
        self
    }
    
    /// Git komutu çalıştır (güvenli)
    fn run_git(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .args(args)
            .current_dir(&self.working_dir)
            .output()
            .map_err(|e| format!("Git çalıştırılamadı: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
    
    /// Tehlikeli komut mu kontrol et
    fn is_dangerous_command(&self, args: &[&str]) -> bool {
        let dangerous_patterns = [
            "push --force",
            "push -f",
            "reset --hard",
            "clean -fd",
            "checkout --",
            "rebase -i",
            "filter-branch",
        ];
        
        let cmd = args.join(" ");
        dangerous_patterns.iter().any(|p| cmd.contains(p))
    }
}

impl Default for GitTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for GitTool {
    fn name(&self) -> &str {
        "git"
    }
    
    fn description(&self) -> &str {
        "Git versiyon kontrol sistemi. Commit, branch, status, log işlemleri."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Process
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("command", "string", true, "Git komutu (status, log, diff, branch, commit)"),
            ToolParameter::new("args", "array", false, "Komut argümanları"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let command = params.get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("status");
        
        let mut args = vec![command];
        
        if let Some(extra_args) = params.get("args").and_then(|a| a.as_array()) {
            for arg in extra_args {
                if let Some(s) = arg.as_str() {
                    args.push(s);
                }
            }
        }
        
        // Tehlikeli komut kontrolü
        let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        if self.is_dangerous_command(&args_str) {
            return SentientToolResult::failure(
                "TEHLİKELİ İŞLEM: Bu git komutu engellendi. Force push, reset --hard gibi tehlikeli işlemler yasaktır."
            );
        }
        
        // İzin verilen komutlar
        let allowed_commands = ["status", "log", "diff", "branch", "commit", "add", "push", "pull", "clone", "fetch"];
        if !allowed_commands.contains(&command) {
            return SentientToolResult::failure(&format!(
                "İzin verilmeyen git komutu: '{}'. İzin verilenler: {}",
                command,
                allowed_commands.join(", ")
            ));
        }
        
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        match self.run_git(&args_refs) {
            Ok(output) => SentientToolResult::success_with_data(
                "Git komutu başarıyla çalıştırıldı",
                serde_json::json!({
                    "command": command,
                    "output": output,
                })
            ),
            Err(e) => SentientToolResult::failure(&format!("Git hatası: {}", e)),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_git_tool_creation() {
        let tool = GitTool::new();
        assert_eq!(tool.name(), "git");
    }
    
    #[test]
    fn test_git_tool_category() {
        let tool = GitTool::new();
        assert_eq!(tool.category(), ToolCategory::Process);
    }
    
    #[test]
    fn test_git_tool_risk_level() {
        let tool = GitTool::new();
        assert_eq!(tool.risk_level(), RiskLevel::Medium);
    }
    
    #[test]
    fn test_dangerous_command_detection() {
        let tool = GitTool::new();
        assert!(tool.is_dangerous_command(&["push", "--force"]));
        assert!(tool.is_dangerous_command(&["reset", "--hard", "HEAD~1"]));
        assert!(!tool.is_dangerous_command(&["status"]));
    }
    
    #[test]
    fn test_parameters() {
        let tool = GitTool::new();
        let params = tool.parameters();
        assert!(params.len() >= 1);
        assert_eq!(params[0].name, "command");
    }
    
    #[tokio::test]
    async fn test_blocked_dangerous_command() {
        let tool = GitTool::new();
        let params = HashMap::from([
            ("command".to_string(), serde_json::json!("push")),
            ("args".to_string(), serde_json::json!(["--force"])),
        ]);
        let result = tool.execute(params).await;
        assert!(!result.success);
        assert!(result.error.expect("operation failed").contains("TEHLİKELİ"));
    }
}
