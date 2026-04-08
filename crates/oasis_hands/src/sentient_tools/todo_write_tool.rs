//! ═══════════════════════════════════════════════════════════════════════════════
//!  TODO WRITE TOOL - Görev/Yapılacak Listesi Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Todo Item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: TodoStatus,
    pub priority: TodoPriority,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, PartialOrd)]
pub enum TodoPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Todo Write Tool
pub struct TodoWriteTool {
    todos: Arc<Mutex<Vec<TodoItem>>>,
    todo_file: PathBuf,
}

impl TodoWriteTool {
    pub fn new(todo_dir: PathBuf) -> Self {
        let todo_file = todo_dir.join("todos.json");
        let todos = Self::load_todos(&todo_file).unwrap_or_default();
        
        Self {
            todos: Arc::new(Mutex::new(todos)),
            todo_file,
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new(PathBuf::from("data"))
    }
    
    fn load_todos(path: &PathBuf) -> Option<Vec<TodoItem>> {
        if path.exists() {
            let content = std::fs::read_to_string(path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    
    fn save_todos(&self) -> bool {
        if let Ok(todos) = self.todos.lock() {
            if let Ok(content) = serde_json::to_string_pretty(&*todos) {
                std::fs::write(&self.todo_file, content).is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn generate_id() -> String {
        format!("todo_{}", chrono::Utc::now().timestamp_millis())
    }
    
    pub fn add_todo(&self, content: &str, priority: TodoPriority) -> TodoItem {
        let now = chrono::Utc::now().to_rfc3339();
        let id = Self::generate_id();
        
        let todo = TodoItem {
            id: id.clone(),
            content: content.to_string(),
            status: TodoStatus::Pending,
            priority,
            created_at: now.clone(),
            updated_at: now,
        };
        
        if let Ok(mut todos) = self.todos.lock() {
            todos.push(todo.clone());
        }
        self.save_todos();
        
        todo
    }
    
    pub fn update_status(&self, id: &str, status: TodoStatus) -> Option<TodoItem> {
        if let Ok(mut todos) = self.todos.lock() {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.status = status.clone();
                todo.updated_at = chrono::Utc::now().to_rfc3339();
                let updated = todo.clone();
                drop(todos);
                self.save_todos();
                return Some(updated);
            }
        }
        None
    }
    
    pub fn list_todos(&self, status_filter: Option<TodoStatus>) -> Vec<TodoItem> {
        if let Ok(todos) = self.todos.lock() {
            match status_filter {
                Some(status) => todos.iter().filter(|t| t.status == status).cloned().collect(),
                None => todos.iter().cloned().collect(),
            }
        } else {
            Vec::new()
        }
    }
    
    pub fn remove_todo(&self, id: &str) -> bool {
        if let Ok(mut todos) = self.todos.lock() {
            let len_before = todos.len();
            todos.retain(|t| t.id != id);
            let removed = todos.len() < len_before;
            drop(todos);
            if removed {
                self.save_todos();
            }
            return removed;
        }
        false
    }
}

#[async_trait]
impl SentientTool for TodoWriteTool {
    fn name(&self) -> &str { "todo_write" }
    
    fn description(&self) -> &str {
        "Yapılacak listesi yönetimi. Ekle, güncelle, sil, listele."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Productivity }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon: add, update, remove, list, clear"),
            ToolParameter::new("id", "string", false, "Todo ID (update/remove için)"),
            ToolParameter::new("content", "string", false, "Todo içeriği (add için)"),
            ToolParameter::new("status", "string", false, "Durum: pending, in_progress, completed"),
            ToolParameter::new("priority", "string", false, "Öncelik: low, medium, high, critical"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "add" => {
                let content = params.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                let priority = match params.get("priority").and_then(|v| v.as_str()) {
                    Some("low") => TodoPriority::Low,
                    Some("high") => TodoPriority::High,
                    Some("critical") => TodoPriority::Critical,
                    _ => TodoPriority::Medium,
                };
                
                if content.is_empty() {
                    return SentientToolResult::failure("Todo içeriği boş olamaz");
                }
                
                let todo = self.add_todo(content, priority);
                
                SentientToolResult::success_with_data(
                    "Todo eklendi",
                    serde_json::to_value(todo).unwrap_or(serde_json::json!({}))
                )
            }
            "update" => {
                let id = params.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let status = match params.get("status").and_then(|v| v.as_str()) {
                    Some("pending") => Some(TodoStatus::Pending),
                    Some("in_progress") => Some(TodoStatus::InProgress),
                    Some("completed") => Some(TodoStatus::Completed),
                    _ => None,
                };
                
                if let Some(status) = status {
                    if let Some(todo) = self.update_status(id, status) {
                        return SentientToolResult::success_with_data(
                            "Todo güncellendi",
                            serde_json::to_value(todo).unwrap_or(serde_json::json!({}))
                        );
                    }
                }
                
                SentientToolResult::failure("Todo güncellenemedi")
            }
            "remove" => {
                let id = params.get("id").and_then(|v| v.as_str()).unwrap_or("");
                
                if self.remove_todo(id) {
                    SentientToolResult::success(&format!("Todo silindi: {}", id))
                } else {
                    SentientToolResult::failure(&format!("Todo bulunamadı: {}", id))
                }
            }
            "list" => {
                let status_filter = match params.get("status").and_then(|v| v.as_str()) {
                    Some("pending") => Some(TodoStatus::Pending),
                    Some("in_progress") => Some(TodoStatus::InProgress),
                    Some("completed") => Some(TodoStatus::Completed),
                    _ => None,
                };
                
                let todos = self.list_todos(status_filter);
                
                SentientToolResult::success_with_data(
                    &format!("{} todo bulundu", todos.len()),
                    serde_json::json!({
                        "count": todos.len(),
                        "todos": todos
                    })
                )
            }
            "clear" => {
                if let Ok(mut todos) = self.todos.lock() {
                    let count = todos.len();
                    todos.clear();
                    drop(todos);
                    self.save_todos();
                    SentientToolResult::success(&format!("{} todo temizlendi", count))
                } else {
                    SentientToolResult::failure("Todo listesi temizlenemedi")
                }
            }
            _ => SentientToolResult::failure(&format!("Bilinmeyen aksiyon: {}", action))
        }
    }
}

impl Clone for TodoWriteTool {
    fn clone(&self) -> Self {
        Self {
            todos: Arc::clone(&self.todos),
            todo_file: self.todo_file.clone(),
        }
    }
}

impl Default for TodoWriteTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_creation() {
        let tool = TodoWriteTool::default_tool();
        assert_eq!(tool.name(), "todo_write");
    }
    
    #[test]
    fn test_add_todo() {
        let tool = TodoWriteTool::default_tool();
        let todo = tool.add_todo("Test todo", TodoPriority::High);
        
        assert!(todo.id.starts_with("todo_"));
        assert_eq!(todo.content, "Test todo");
        assert_eq!(todo.status, TodoStatus::Pending);
    }
}
