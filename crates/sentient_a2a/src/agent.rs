//! A2A Agent types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent ID
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentId {
    pub namespace: String,
    pub name: String,
    pub version: String,
}

impl AgentId {
    pub fn new(namespace: &str, name: &str, version: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn as_str(&self) -> String {
        format!("{}:{}:{}", self.namespace, self.name, self.version)
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.namespace, self.name, self.version)
    }
}

/// Agent capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    /// Capability name
    pub name: String,
    /// Description
    pub description: String,
    /// Input schema (JSON Schema)
    pub input_schema: Option<serde_json::Value>,
    /// Output schema
    pub output_schema: Option<serde_json::Value>,
    /// Tags
    pub tags: Vec<String>,
    /// Cost (compute units)
    pub cost: Option<f32>,
}

impl AgentCapability {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: None,
            output_schema: None,
            tags: Vec::new(),
            cost: None,
        }
    }

    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.into_iter().map(|s| s.to_string()).collect();
        self
    }
}

/// Agent metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    /// Agent name
    pub name: String,
    /// Description
    pub description: String,
    /// Author
    pub author: Option<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last seen
    pub last_seen: DateTime<Utc>,
    /// Status
    pub status: AgentStatus,
    /// Tags
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

impl Default for AgentMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            author: None,
            created_at: Utc::now(),
            last_seen: Utc::now(),
            status: AgentStatus::Active,
            tags: Vec::new(),
            custom: HashMap::new(),
        }
    }
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    Active,
    Inactive,
    Busy,
    Error,
    Offline,
}

/// Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent ID
    pub id: AgentId,
    /// Capabilities
    pub capabilities: Vec<AgentCapability>,
    /// Metadata
    pub metadata: AgentMetadata,
    /// Endpoint URL
    pub endpoint: String,
    /// Supported transports
    pub transports: Vec<String>,
    /// Max concurrent tasks
    pub max_concurrent: u32,
    /// Current load
    pub current_load: u32,
}

impl Agent {
    pub fn new(id: AgentId, endpoint: &str) -> Self {
        Self {
            id,
            capabilities: Vec::new(),
            metadata: AgentMetadata::default(),
            endpoint: endpoint.to_string(),
            transports: vec!["http".to_string()],
            max_concurrent: 10,
            current_load: 0,
        }
    }

    pub fn with_capability(mut self, capability: AgentCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.metadata.name = name.to_string();
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.metadata.description = desc.to_string();
        self
    }

    pub fn has_capability(&self, name: &str) -> bool {
        self.capabilities.iter().any(|c| c.name == name)
    }

    pub fn get_capability(&self, name: &str) -> Option<&AgentCapability> {
        self.capabilities.iter().find(|c| c.name == name)
    }

    pub fn is_available(&self) -> bool {
        self.metadata.status == AgentStatus::Active && self.current_load < self.max_concurrent
    }

    pub fn load_factor(&self) -> f32 {
        if self.max_concurrent == 0 {
            1.0
        } else {
            self.current_load as f32 / self.max_concurrent as f32
        }
    }
}
