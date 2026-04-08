//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Python Wrappers - Native Rust wrapper'ları
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  CrewAI, Browser-Use, Mem0 ve OpenManus için yüksek seviyeli Rust API'leri
//!
//!  NOT: Bu modül sadece veri modelleri ve yardımcı fonksiyonlar sağlar.
//!  Gerçek Python çağrıları sentient_python::PythonBridge üzerinden yapılır.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use sentient_common::error::{SENTIENTError, SENTIENTResult};

// ═══════════════════════════════════════════════════════════════════════════════
//  CREWAI WRAPPER
// ═══════════════════════════════════════════════════════════════════════════════

/// CrewAI ajan tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewAgent {
    pub id: Uuid,
    pub role: String,
    pub goal: String,
    pub backstory: Option<String>,
    pub verbose: bool,
}

impl CrewAgent {
    /// Yeni ajan oluştur
    pub fn new(role: &str, goal: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: role.to_string(),
            goal: goal.to_string(),
            backstory: None,
            verbose: true,
        }
    }

    /// Geçmiş ekle
    pub fn with_backstory(mut self, backstory: &str) -> Self {
        self.backstory = Some(backstory.to_string());
        self
    }
}

/// CrewAI görev tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewTask {
    pub id: Uuid,
    pub description: String,
    pub expected_output: String,
    pub agent_id: Option<Uuid>,
}

impl CrewTask {
    /// Yeni görev oluştur
    pub fn new(description: &str, expected_output: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            expected_output: expected_output.to_string(),
            agent_id: None,
        }
    }

    /// Ajana ata
    pub fn assign_to(mut self, agent_id: Uuid) -> Self {
        self.agent_id = Some(agent_id);
        self
    }
}

/// CrewAI sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewResult {
    pub success: bool,
    pub output: String,
    pub token_usage: TokenUsage,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BROWSER WRAPPER
// ═══════════════════════════════════════════════════════════════════════════════

/// Browser arama sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub position: u32,
}

impl SearchResult {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            title: json.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            url: json.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            snippet: json.get("snippet").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            position: json.get("position").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        }
    }
}

/// Browser görev sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: String,
    pub steps: Vec<TaskStep>,
    pub duration_ms: u64,
}

impl TaskResult {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            success: json.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            output: json.get("output").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            steps: json.get("steps")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().map(TaskStep::from_json).collect())
                .unwrap_or_default(),
            duration_ms: json.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStep {
    pub action: String,
    pub description: String,
    pub success: bool,
}

impl TaskStep {
    fn from_json(json: &serde_json::Value) -> Self {
        Self {
            action: json.get("action").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            description: json.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            success: json.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY WRAPPER (Mem0)
// ═══════════════════════════════════════════════════════════════════════════════

/// Bellek kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub user_id: String,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
}

impl MemoryEntry {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            id: json.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            content: json.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            user_id: json.get("user_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            metadata: HashMap::new(),
            created_at: json.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SANDBOX WRAPPER (OpenManus)
// ═══════════════════════════════════════════════════════════════════════════════

/// Sandbox sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

impl SandboxResult {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            success: json.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            exit_code: json.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(-1) as i32,
            stdout: json.get("stdout").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            stderr: json.get("stderr").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            duration_ms: json.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.success && self.exit_code == 0
    }

    pub fn output(&self) -> String {
        if self.is_ok() {
            self.stdout.clone()
        } else {
            format!("HATA: {}", self.stderr)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crew_agent() {
        let agent = CrewAgent::new("Developer", "Write code");
        assert_eq!(agent.role, "Developer");
        assert_eq!(agent.goal, "Write code");
        assert!(agent.backstory.is_none());
    }

    #[test]
    fn test_crew_agent_with_backstory() {
        let agent = CrewAgent::new("Developer", "Write code")
            .with_backstory("Expert programmer");
        assert_eq!(agent.backstory, Some("Expert programmer".to_string()));
    }

    #[test]
    fn test_crew_task() {
        let task = CrewTask::new("Research AI", "Summary of findings");
        assert_eq!(task.description, "Research AI");
        assert!(task.agent_id.is_none());
    }

    #[test]
    fn test_search_result() {
        let json = serde_json::json!({
            "title": "Test",
            "url": "https://example.com",
            "snippet": "Test snippet",
            "position": 1
        });
        
        let result = SearchResult::from_json(&json);
        assert_eq!(result.title, "Test");
        assert_eq!(result.position, 1);
    }

    #[test]
    fn test_memory_entry() {
        let json = serde_json::json!({
            "id": "mem-123",
            "content": "Test memory",
            "user_id": "user-1",
            "created_at": "2024-01-01"
        });
        
        let entry = MemoryEntry::from_json(&json);
        assert_eq!(entry.id, "mem-123");
        assert_eq!(entry.content, "Test memory");
    }

    #[test]
    fn test_sandbox_result() {
        let json = serde_json::json!({
            "success": true,
            "exit_code": 0,
            "stdout": "Hello World",
            "stderr": "",
            "duration_ms": 100
        });
        
        let result = SandboxResult::from_json(&json);
        assert!(result.is_ok());
        assert_eq!(result.output(), "Hello World");
    }

    #[test]
    fn test_sandbox_result_error() {
        let json = serde_json::json!({
            "success": false,
            "exit_code": 1,
            "stdout": "",
            "stderr": "Error occurred",
            "duration_ms": 50
        });
        
        let result = SandboxResult::from_json(&json);
        assert!(!result.is_ok());
        assert!(result.output().contains("HATA"));
    }
}
