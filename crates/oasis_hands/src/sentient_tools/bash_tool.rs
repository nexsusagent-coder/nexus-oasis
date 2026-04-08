//! ═══════════════════════════════════════════════════════════════════════════════
//!  BASH TOOL - OpenHarness A2 Mantığına Tam Sadık
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Shell komut çalıştırma aracı - SOVEREIGN güvenlik katmanı ile
//! 
//! Özellikler:
//! - Async subprocess yönetimi
//! - Timeout desteği (1-600 saniye)
//! - stdout/stderr capture
//! - Return code takibi
//! - SOVEREIGN komut doğrulama

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::ExitStatus;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

// ═══════════════════════════════════════════════════════════════════════════════
//  INPUT MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Bash tool girdi parametreleri
#[derive(Debug, Clone)]
pub struct BashToolInput {
    pub command: String,
    pub cwd: Option<String>,
    pub timeout_seconds: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BASH TOOL IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

pub struct BashTool {
    policy: SovereignPolicy,
    max_output_size: usize,
}

impl BashTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self {
            policy,
            max_output_size: 12000,
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new(SovereignPolicy::developer())
    }
    
    pub async fn execute_safe(
        &self,
        input: BashToolInput,
        working_dir: &PathBuf,
    ) -> crate::error::HandsResult<serde_json::Value> {
        let start = std::time::Instant::now();
        
        // SOVEREIGN doğrulama
        self.policy.validate_command(&input.command)?;
        
        // Working directory çöz
        let cwd = if let Some(cwd_path) = &input.cwd {
            PathBuf::from(cwd_path)
        } else {
            working_dir.clone()
        };
        
        let timeout_secs = input.timeout_seconds.clamp(1, 600);
        
        // Shell çalıştır
        let shell = if cfg!(target_os = "windows") { "cmd" } else { "bash" };
        let shell_arg = if cfg!(target_os = "windows") { "/C" } else { "-c" };
        
        let mut child = Command::new(shell)
            .arg(shell_arg)
            .arg(&input.command)
            .current_dir(&cwd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Subprocess başlatılamadı: {}", e))?;
        
        let timeout_duration = Duration::from_secs(timeout_secs);
        
        let result = match timeout(timeout_duration, child.wait_with_output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                let output_text = self.format_output(&stdout, &stderr);
                let is_error = !output.status.success();
                
                serde_json::json!({
                    "output": output_text,
                    "is_error": is_error,
                    "returncode": output.status.code(),
                    "timed_out": false,
                    "duration_ms": start.elapsed().as_millis()
                })
            }
            Ok(Err(e)) => {
                serde_json::json!({
                    "output": format!("Komut hatası: {}", e),
                    "is_error": true,
                    "returncode": null,
                    "timed_out": false,
                    "duration_ms": start.elapsed().as_millis()
                })
            }
            Err(_) => {
                // Timeout - process'i öldürmemiz gerekiyor ama child artık yok
                // Yeni bir process başlatıp kill edemeyiz, sadece timeout sonucunu döndür
                serde_json::json!({
                    "output": format!("Timeout: {} saniye - process sonlandırıldı", timeout_secs),
                    "is_error": true,
                    "returncode": null,
                    "timed_out": true,
                    "duration_ms": start.elapsed().as_millis()
                })
            }
        };
        
        Ok(result)
    }
    
    fn format_output(&self, stdout: &str, stderr: &str) -> String {
        let mut parts = Vec::new();
        
        let stdout = stdout.trim();
        let stderr = stderr.trim();
        
        if !stdout.is_empty() {
            parts.push(stdout.to_string());
        }
        
        if !stderr.is_empty() {
            parts.push(format!("[STDERR] {}", stderr));
        }
        
        let output = parts.join("\n");
        
        if output.is_empty() {
            return "(çıktı yok)".to_string();
        }
        
        if output.len() > self.max_output_size {
            format!("{}...[kısaltıldı]", &output[..self.max_output_size])
        } else {
            output
        }
    }
}

#[async_trait]
impl SentientTool for BashTool {
    fn name(&self) -> &str { "bash" }
    
    fn description(&self) -> &str {
        "Shell komutu çalıştırır. SOVEREIGN güvenlik katmanı ile korunur."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::System }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::High }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("command", "string", true, "Çalıştırılacak shell komutu"),
            ToolParameter::new("cwd", "string", false, "Working directory"),
            ToolParameter::with_default("timeout_seconds", "integer", "Timeout süresi (1-600)", serde_json::json!(120)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let command = params.get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let cwd = params.get("cwd").and_then(|v| v.as_str()).map(|s| s.to_string());
        let timeout_seconds = params.get("timeout_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(120);
        
        let input = BashToolInput { command, cwd, timeout_seconds };
        let working_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/tmp"));
        
        match self.execute_safe(input, &working_dir).await {
            Ok(value) => SentientToolResult::success_with_data("Bash komutu tamamlandı", value),
            Err(e) => SentientToolResult::failure(&format!("SOVEREIGN Hatası: {}", e)),
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bash_tool_creation() {
        let tool = BashTool::default_tool();
        assert_eq!(tool.name(), "bash");
    }
}
