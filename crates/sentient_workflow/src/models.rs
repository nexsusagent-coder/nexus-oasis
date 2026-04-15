//! ─── Workflow Models ───

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub triggers: Vec<Trigger>,
    pub variables: HashMap<String, serde_json::Value>,
    pub status: crate::WorkflowStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Workflow {
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            nodes: vec![],
            connections: vec![],
            triggers: vec![],
            variables: HashMap::new(),
            status: crate::WorkflowStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn add_node(&mut self, node: Node) -> String {
        let id = node.id.clone();
        self.nodes.push(node);
        self.touch();
        id
    }
    
    pub fn connect(&mut self, conn: Connection) {
        self.connections.push(conn);
        self.touch();
    }
    
    pub fn add_trigger(&mut self, trigger: Trigger) {
        self.triggers.push(trigger);
        self.touch();
    }
    
    pub fn activate(&mut self) {
        self.status = crate::WorkflowStatus::Active;
        self.touch();
    }
    
    pub fn pause(&mut self) {
        self.status = crate::WorkflowStatus::Paused;
        self.touch();
    }
    
    fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    pub fn get_start_nodes(&self) -> Vec<&Node> {
        let connected_inputs: Vec<&str> = self.connections.iter()
            .map(|c| c.target_node.as_str())
            .collect();
        
        self.nodes.iter()
            .filter(|n| !connected_inputs.contains(&n.id.as_str()))
            .collect()
    }
    
    /// Validate workflow for cycles and connectivity
    pub fn validate(&self) -> crate::WorkflowResult<()> {
        if self.nodes.is_empty() {
            return Err(crate::WorkflowError::Validation("No nodes in workflow".into()));
        }
        
        // Check for cycles using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        for node in self.get_start_nodes() {
            if self.has_cycle(node.id.as_str(), &mut visited, &mut rec_stack)? {
                return Err(crate::WorkflowError::CycleDetected);
            }
        }
        
        Ok(())
    }
    
    fn has_cycle(
        &self,
        node_id: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> crate::WorkflowResult<bool> {
        if rec_stack.contains(node_id) {
            return Ok(true);
        }
        
        if visited.contains(node_id) {
            return Ok(false);
        }
        
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        
        // Get outgoing connections
        for conn in &self.connections {
            if conn.source_node == node_id {
                if self.has_cycle(&conn.target_node, visited, rec_stack)? {
                    return Ok(true);
                }
            }
        }
        
        rec_stack.remove(node_id);
        Ok(false)
    }
}

/// A node in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub position: Position,
    pub config: serde_json::Value,
    pub on_error: ErrorHandling,
}

impl Node {
    pub fn new(name: &str, node_type: NodeType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            node_type,
            position: Position::default(),
            config: serde_json::json!({}),
            on_error: ErrorHandling::Stop,
        }
    }
    
    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }
    
    pub fn with_config(mut self, config: serde_json::Value) -> Self {
        self.config = config;
        self
    }
}

/// Node types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    // Triggers
    Trigger(TriggerType),
    
    // Actions
    Http(HttpConfig),
    Email(EmailConfig),
    Script(ScriptConfig),
    Llm(LlmConfig),
    Condition(ConditionConfig),
    Delay(DelayConfig),
    Loop(LoopConfig),
    
    // Integrations
    Telegram(TelegramConfig),
    Discord(DiscordConfig),
    HomeAssistant(HomeAssistantConfig),
    
    // Custom
    Custom(CustomConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Connection between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub source_node: String,
    pub source_output: String,
    pub target_node: String,
    pub target_input: String,
    pub label: Option<String>,
}

impl Connection {
    pub fn new(source: &str, source_output: &str, target: &str, target_input: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_node: source.to_string(),
            source_output: source_output.to_string(),
            target_node: target.to_string(),
            target_input: target_input.to_string(),
            label: None,
        }
    }
}

/// Trigger definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub id: String,
    pub trigger_type: TriggerType,
    pub enabled: bool,
}

impl Trigger {
    pub fn new(trigger_type: TriggerType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            trigger_type,
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TriggerType {
    Manual,
    Schedule { cron: String },
    Webhook { path: String, method: String },
    Event { event_type: String },
    Voice { phrase: String },
    FileWatch { path: String, pattern: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptConfig {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub prompt: String,
    pub model: String,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    pub expression: String,
    pub true_output: String,
    pub false_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelayConfig {
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConfig {
    pub iterations: u32,
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub chat_id: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub channel_id: u64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeAssistantConfig {
    pub entity_id: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomConfig {
    pub action_id: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorHandling {
    Stop,
    Continue,
    Retry { max_attempts: u32 },
}

impl Default for ErrorHandling {
    fn default() -> Self {
        Self::Stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_creation() {
        let wf = Workflow::new("Test Workflow");
        assert_eq!(wf.name, "Test Workflow");
        assert!(matches!(wf.status, crate::WorkflowStatus::Draft));
    }
    
    #[test]
    fn test_node_creation() {
        let node = Node::new("HTTP Request", NodeType::Http(HttpConfig {
            url: "https://api.example.com".into(),
            method: "GET".into(),
            headers: HashMap::new(),
            body: None,
        }));
        assert_eq!(node.name, "HTTP Request");
    }
}
