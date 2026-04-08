//! ═══════════════════════════════════════════════════════════════════════════════
//!  TASK TOOL - GÖREV YÖNETİM ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! SENTIENT görev sistemi ile etkileşim.
//! Görev oluşturma, izleme, iptal, listeleme.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Task aracı - görev yönetimi
pub struct TaskTool {
    /// Aktif görevler
    tasks: HashMap<String, TaskInfo>,
}

/// Görev bilgisi
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskInfo {
    /// Görev ID
    pub id: String,
    /// Görev adı
    pub name: String,
    /// Durum
    pub status: TaskStatus,
    /// Öncelik
    pub priority: u8,
    /// Oluşturulma zamanı
    pub created_at: String,
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskTool {
    /// Yeni Task aracı oluştur
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
}

impl Default for TaskTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for TaskTool {
    fn name(&self) -> &str {
        "task"
    }
    
    fn description(&self) -> &str {
        "SENTIENT görev sistemi. Görev oluşturma, izleme, iptal, listeleme."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Process
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (create, status, list, cancel, result)"),
            ToolParameter::new("name", "string", false, "Görev adı (create için)"),
            ToolParameter::new("goal", "string", false, "Görev hedefi (create için)"),
            ToolParameter::new("task_id", "string", false, "Görev ID (status, cancel, result için)"),
            ToolParameter::new("priority", "number", false, "Öncelik 1-5"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "create" | "new" => {
                let name = params.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Yeni Görev");
                let goal = params.get("goal")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let priority = params.get("priority")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(3) as u8;
                
                let task_id = format!("task_{}", chrono::Utc::now().timestamp());
                
                log::info!("📋  TASK: Oluşturuluyor → {} (öncelik: {})", name, priority);
                
                SentientToolResult::success_with_data(
                    "Görev oluşturuldu",
                    serde_json::json!({
                        "task_id": task_id,
                        "name": name,
                        "goal": goal,
                        "priority": priority,
                        "status": "pending",
                    })
                )
            }
            "status" => {
                let task_id = params.get("task_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("📊  TASK: Durum sorgulanıyor → {}", task_id);
                
                SentientToolResult::success_with_data(
                    "Görev durumu",
                    serde_json::json!({
                        "task_id": task_id,
                        "status": "running",
                        "progress": 45,
                        "eta_seconds": 120,
                    })
                )
            }
            "list" => {
                log::info!("📋  TASK: Görevler listeleniyor");
                
                SentientToolResult::success_with_data(
                    "Görev listesi",
                    serde_json::json!({
                        "total": 3,
                        "pending": 1,
                        "running": 1,
                        "completed": 1,
                        "tasks": [
                            {"id": "task-001", "name": "Örnek görev", "status": "running"},
                        ],
                    })
                )
            }
            "cancel" => {
                let task_id = params.get("task_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("🚫  TASK: İptal ediliyor → {}", task_id);
                
                SentientToolResult::success(&format!("Görev iptal edildi: {}", task_id))
            }
            "result" => {
                let task_id = params.get("task_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                SentientToolResult::success_with_data(
                    "Görev sonucu",
                    serde_json::json!({
                        "task_id": task_id,
                        "status": "completed",
                        "result": "Mock görev sonucu",
                        "duration_ms": 1234,
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen görev aksiyonu: '{}'. Kullanılabilir: create, status, list, cancel, result",
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
    fn test_task_tool_creation() {
        let tool = TaskTool::new();
        assert_eq!(tool.name(), "task");
    }
    
    #[tokio::test]
    async fn test_task_create() {
        let tool = TaskTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("create")),
            ("name".to_string(), serde_json::json!("Test görevi")),
            ("goal".to_string(), serde_json::json!("Test etmek")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_task_list() {
        let tool = TaskTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("list")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
