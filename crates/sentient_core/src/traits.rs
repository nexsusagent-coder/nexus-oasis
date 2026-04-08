//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT CORE TRAITS - Modüler Arayüz Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  TEMEL BİLEŞEN TRAIT'I
// ═══════════════════════════════════════════════════════════════════════════════

/// Tüm SENTIENT bileşenlerinin temel kimliği.
pub trait SENTIENTComponent: Send + Sync + std::fmt::Debug {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn component_type(&self) -> ComponentType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Core,
    Memory,
    Agent,
    Tool,
    Gateway,
    Guard,
    Bridge,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BİLEŞEN DURUMU
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentState {
    Uninitialized,
    Initializing,
    Ready,
    Running,
    Paused,
    Error,
    ShuttingDown,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub last_check: DateTime<Utc>,
    pub message: String,
    pub metrics: HashMap<String, f64>,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            healthy: true,
            last_check: Utc::now(),
            message: "OK".to_string(),
            metrics: HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BELLEK VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub content: String,
    pub memory_type: MemoryType,
    pub embedding: Option<Vec<f32>>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub access_count: usize,
    pub importance: f32,
}

impl MemoryEntry {
    pub fn new(content: &str, memory_type: MemoryType) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.to_string(),
            memory_type,
            embedding: None,
            metadata: HashMap::new(),
            created_at: Utc::now(),
            expires_at: None,
            access_count: 0,
            importance: 0.5,
        }
    }

    pub fn with_ttl(mut self, seconds: i64) -> Self {
        self.expires_at = Some(self.created_at + chrono::Duration::seconds(seconds));
        self
    }

    pub fn with_importance(mut self, importance: f32) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| Utc::now() > exp)
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Procedural,
    Working,
    Meta,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AJAN VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentGoal {
    pub id: Uuid,
    pub description: String,
    pub priority: u8,
    pub deadline: Option<DateTime<Utc>>,
    pub parent: Option<Uuid>,
    pub sub_goals: Vec<Uuid>,
}

impl AgentGoal {
    pub fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            priority: 5,
            deadline: None,
            parent: None,
            sub_goals: Vec::new(),
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: Uuid,
    pub description: String,
    pub context: HashMap<String, serde_json::Value>,
    pub status: TaskStatus,
}

impl AgentTask {
    pub fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            context: HashMap::new(),
            status: TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: Uuid,
    pub action_type: ActionType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Think,
    Browse,
    Code,
    Execute,
    Communicate,
    Wait,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: Uuid,
    pub success: bool,
    pub output: String,
    pub artifacts: Vec<String>,
    pub metrics: HashMap<String, f64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ARAÇ (TOOL) VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub duration_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GÜVENLİK (GUARDRAIL) VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailVerdict {
    pub allowed: bool,
    pub threats: Vec<Threat>,
    pub sanitized_content: Option<String>,
}

impl GuardrailVerdict {
    pub fn clean() -> Self {
        Self {
            allowed: true,
            threats: Vec::new(),
            sanitized_content: None,
        }
    }

    pub fn blocked(threats: Vec<Threat>) -> Self {
        Self {
            allowed: false,
            threats,
            sanitized_content: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub threat_type: ThreatType,
    pub severity: Severity,
    pub matched_pattern: String,
    pub location: Option<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    PromptInjection,
    DataExfiltration,
    CodeInjection,
    PathTraversal,
    CommandInjection,
    InformationDisclosure,
    HarmfulContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LLM VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stream: bool,
}

impl LlmRequest {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_string(),
            messages: Vec::new(),
            max_tokens: Some(4096),
            temperature: Some(0.7),
            stream: false,
        }
    }

    pub fn with_message(mut self, role: MessageRole, content: &str) -> Self {
        self.messages.push(ChatMessage {
            role,
            content: content.to_string(),
        });
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub usage: TokenUsage,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCall,
    Error,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EVENT VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub topic: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    pub fn new(topic: &str, source: &str, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic: topic.to_string(),
            source: source.to_string(),
            payload,
            timestamp: Utc::now(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  IMPLEMENTATION MACRO
// ═══════════════════════════════════════════════════════════════════════════════

#[macro_export]
macro_rules! impl_sentient_component {
    ($ty:ty, $name:expr, $version:expr, $ctype:expr) => {
        impl $crate::traits::SENTIENTComponent for $ty {
            fn id(&self) -> uuid::Uuid {
                self.id
            }
            
            fn name(&self) -> &str {
                $name
            }
            
            fn version(&self) -> &str {
                $version
            }
            
            fn component_type(&self) -> $crate::traits::ComponentType {
                $ctype
            }
        }
    };
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_types() {
        assert!(matches!(ComponentType::Core, ComponentType::Core));
        assert_ne!(ComponentType::Agent, ComponentType::Tool);
    }

    #[test]
    fn test_component_states() {
        let states = vec![
            ComponentState::Uninitialized,
            ComponentState::Ready,
            ComponentState::Running,
            ComponentState::Error,
        ];
        assert_eq!(states.len(), 4);
    }

    #[test]
    fn test_memory_entry() {
        let entry = MemoryEntry::new("Test", MemoryType::Working)
            .with_ttl(3600)
            .with_importance(0.8);
        
        assert_eq!(entry.content, "Test");
        assert_eq!(entry.memory_type, MemoryType::Working);
        assert!(entry.expires_at.is_some());
        assert_eq!(entry.importance, 0.8);
    }

    #[test]
    fn test_llm_request() {
        let request = LlmRequest::new("gpt-4")
            .with_message(MessageRole::System, "You are helpful")
            .with_message(MessageRole::User, "Hello");
        
        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.messages.len(), 2);
    }

    #[test]
    fn test_guardrail_verdict() {
        let clean = GuardrailVerdict::clean();
        assert!(clean.allowed);
        assert!(clean.threats.is_empty());
    }

    #[test]
    fn test_event() {
        let event = Event::new("test.topic", "test_source", serde_json::json!({"key": "value"}));
        assert_eq!(event.topic, "test.topic");
    }
}
