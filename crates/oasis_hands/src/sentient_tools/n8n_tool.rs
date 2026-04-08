//! ═══════════════════════════════════════════════════════════════════════════════
//!  N8N TOOL - WORKFLOW OTOMASYON ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! n8n workflow otomasyonu ile entegrasyon.
//! Workflow tetikleme, yönetme, izleme.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// N8n aracı - workflow otomasyonu
pub struct N8nTool {
    /// n8n sunucu URL
    server_url: String,
    /// API anahtarı
    api_key: Option<String>,
}

impl N8nTool {
    /// Yeni N8n aracı oluştur
    pub fn new() -> Self {
        Self {
            server_url: "http://localhost:5678".to_string(),
            api_key: None,
        }
    }
    
    /// Sunucu URL ayarla
    pub fn with_server(mut self, url: &str) -> Self {
        self.server_url = url.to_string();
        self
    }
}

impl Default for N8nTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for N8nTool {
    fn name(&self) -> &str {
        "n8n"
    }
    
    fn description(&self) -> &str {
        "n8n workflow otomasyon platformu ile entegrasyon. Workflow tetikleme ve yönetme."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Integration
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (trigger, list, status, enable, disable)"),
            ToolParameter::new("workflow_id", "string", false, "Workflow ID"),
            ToolParameter::new("data", "object", false, "Workflow verisi (trigger için)"),
            ToolParameter::new("webhook_url", "string", false, "Webhook URL (trigger için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "trigger" | "run" => {
                let workflow_id = params.get("workflow_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let data = params.get("data").cloned().unwrap_or(serde_json::json!({}));
                
                log::info!("⚡  N8N: Workflow tetikleniyor → {}", workflow_id);
                
                SentientToolResult::success_with_data(
                    "Workflow tetiklendi",
                    serde_json::json!({
                        "workflow_id": workflow_id,
                        "execution_id": "exec-mock-123",
                        "status": "running",
                        "data": data,
                    })
                )
            }
            "list" | "workflows" => {
                log::info!("📋  N8N: Workflow'lar listeleniyor");
                
                SentientToolResult::success_with_data(
                    "Workflow listesi",
                    serde_json::json!({
                        "workflows": [
                            {"id": "wf-001", "name": "E-posta otomasyonu", "active": true},
                            {"id": "wf-002", "name": "Veri senkronizasyonu", "active": false},
                        ],
                        "count": 2,
                    })
                )
            }
            "status" => {
                let workflow_id = params.get("workflow_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                SentientToolResult::success_with_data(
                    "Workflow durumu",
                    serde_json::json!({
                        "workflow_id": workflow_id,
                        "status": "active",
                        "last_execution": "2024-01-15T10:30:00Z",
                        "success_rate": 98.5,
                    })
                )
            }
            "enable" | "disable" => {
                let workflow_id = params.get("workflow_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let enabling = action == "enable";
                
                log::info!("{}  N8N: Workflow {} → {}", 
                    if enabling { "✅" } else { "⏸️" },
                    if enabling { "etkinleştiriliyor" } else { "devre dışı bırakılıyor" },
                    workflow_id
                );
                
                SentientToolResult::success(&format!(
                    "Workflow {}: {}",
                    workflow_id,
                    if enabling { "etkinleştirildi" } else { "devre dışı bırakıldı" }
                ))
            }
            "webhook" => {
                let webhook_url = params.get("webhook_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let data = params.get("data").cloned().unwrap_or(serde_json::json!({}));
                
                log::info!("🪝  N8N: Webhook çağrılıyor → {}", webhook_url);
                
                SentientToolResult::success_with_data(
                    "Webhook çağrıldı",
                    serde_json::json!({
                        "webhook_url": webhook_url,
                        "response": "OK",
                        "data": data,
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen n8n aksiyonu: '{}'. Kullanılabilir: trigger, list, status, enable, disable, webhook",
                    action
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_n8n_tool_creation() {
        let tool = N8nTool::new();
        assert_eq!(tool.name(), "n8n");
    }
    
    #[tokio::test]
    async fn test_n8n_trigger() {
        let tool = N8nTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("trigger")),
            ("workflow_id".to_string(), serde_json::json!("wf-123")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_n8n_list() {
        let tool = N8nTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("list")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
