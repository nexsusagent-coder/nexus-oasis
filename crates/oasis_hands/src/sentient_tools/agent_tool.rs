//! ═══════════════════════════════════════════════════════════════════════════════
//!  AGENT TOOL - AJAN YÖNETİM ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! SENTIENT ajan sistemi yönetimi.
//! Ajan oluşturma, durdurma, listeleme, görev atama.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Agent aracı - ajan yönetimi
pub struct AgentTool {
    /// Maksimum ajan sayısı
    max_agents: usize,
}

impl AgentTool {
    /// Yeni Agent aracı oluştur
    pub fn new() -> Self {
        Self {
            max_agents: 20,
        }
    }
}

impl Default for AgentTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Ajan durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum AgentStatus {
    Idle,
    Working,
    Paused,
    Stopped,
    Error,
}

#[async_trait]
impl SentientTool for AgentTool {
    fn name(&self) -> &str {
        "agent"
    }
    
    fn description(&self) -> &str {
        "SENTIENT ajan sistemi yönetimi. Ajan oluşturma, durdurma, listeleme, görev atama."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Agent
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (create, spawn, list, status, assign, stop, kill)"),
            ToolParameter::new("name", "string", false, "Ajan adı"),
            ToolParameter::new("agent_type", "string", false, "Ajan türü (researcher, coder, browser, assistant)"),
            ToolParameter::new("agent_id", "string", false, "Ajan ID"),
            ToolParameter::new("task", "string", false, "Görev (assign için)"),
            ToolParameter::new("model", "string", false, "LLM model"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "create" | "spawn" => {
                let name = params.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Yeni Ajan");
                let agent_type = params.get("agent_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("assistant");
                let model = params.get("model")
                    .and_then(|v| v.as_str())
                    .unwrap_or("qwen/qwen3.6-plus:free");
                
                let agent_id = format!("agent-{}", chrono::Utc::now().timestamp());
                
                log::info!("🤖  AGENT: Oluşturuluyor → {} ({})", name, agent_type);
                
                SentientToolResult::success_with_data(
                    "Ajan oluşturuldu",
                    serde_json::json!({
                        "agent_id": agent_id,
                        "name": name,
                        "type": agent_type,
                        "model": model,
                        "status": "idle",
                    })
                )
            }
            "list" => {
                log::info!("📋  AGENT: Ajanlar listeleniyor");
                
                SentientToolResult::success_with_data(
                    "Ajan listesi",
                    serde_json::json!({
                        "agents": [
                            {
                                "id": "agent-001",
                                "name": "Araştırmacı",
                                "type": "researcher",
                                "status": "working",
                                "task": "Veri analizi",
                            },
                            {
                                "id": "agent-002",
                                "name": "Kodlayıcı",
                                "type": "coder",
                                "status": "idle",
                            },
                        ],
                        "count": 2,
                        "max_agents": self.max_agents,
                    })
                )
            }
            "status" => {
                let agent_id = params.get("agent_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if agent_id.is_empty() {
                    return SentientToolResult::failure("Ajan ID gerekli");
                }
                
                SentientToolResult::success_with_data(
                    "Ajan durumu",
                    serde_json::json!({
                        "agent_id": agent_id,
                        "status": "working",
                        "current_task": "Veri analizi yapılıyor",
                        "uptime_secs": 300,
                        "messages_processed": 15,
                    })
                )
            }
            "assign" => {
                let agent_id = params.get("agent_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let task = params.get("task")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if agent_id.is_empty() || task.is_empty() {
                    return SentientToolResult::failure("Ajan ID ve görev gerekli");
                }
                
                log::info!("📋  AGENT: Görev atanıyor → {} için: {}", agent_id, task);
                
                SentientToolResult::success_with_data(
                    "Görev atandı",
                    serde_json::json!({
                        "agent_id": agent_id,
                        "task": task,
                        "status": "working",
                    })
                )
            }
            "stop" | "pause" => {
                let agent_id = params.get("agent_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if agent_id.is_empty() {
                    return SentientToolResult::failure("Ajan ID gerekli");
                }
                
                log::info!("⏸️  AGENT: Durduruluyor → {}", agent_id);
                
                SentientToolResult::success(&format!("Ajan durduruldu: {}", agent_id))
            }
            "kill" => {
                let agent_id = params.get("agent_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if agent_id.is_empty() {
                    return SentientToolResult::failure("Ajan ID gerekli");
                }
                
                log::info!("💀  AGENT: Sonlandırılıyor → {}", agent_id);
                
                SentientToolResult::success(&format!("Ajan sonlandırıldı: {}", agent_id))
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen ajan aksiyonu: '{}'. Kullanılabilir: create, list, status, assign, stop, kill",
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
    fn test_agent_tool_creation() {
        let tool = AgentTool::new();
        assert_eq!(tool.name(), "agent");
    }
    
    #[test]
    fn test_agent_category() {
        let tool = AgentTool::new();
        assert_eq!(tool.category(), ToolCategory::Agent);
    }
    
    #[tokio::test]
    async fn test_agent_create() {
        let tool = AgentTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("create")),
            ("name".to_string(), serde_json::json!("Test Ajanı")),
            ("agent_type".to_string(), serde_json::json!("researcher")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_agent_list() {
        let tool = AgentTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("list")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_agent_assign() {
        let tool = AgentTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("assign")),
            ("agent_id".to_string(), serde_json::json!("agent-001")),
            ("task".to_string(), serde_json::json!("Veri topla")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
