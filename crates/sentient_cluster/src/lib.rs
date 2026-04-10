//! ─── SENTIENT Kubernetes Operator ───
//!
//! Deploy and manage distributed SENTIENT agents on Kubernetes.

pub mod metrics;
pub mod health;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// SENTIENT Agent CRD
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, PartialEq)]
pub struct SentientAgentSpec {
    /// Number of replicas
    #[serde(default = "default_replicas")]
    pub replicas: i32,
    
    /// Agent type
    pub agent_type: String,
    
    /// Enabled channels
    #[serde(default)]
    pub channels: Vec<String>,
}

fn default_replicas() -> i32 { 1 }

/// SENTIENT Task CRD
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SentientTaskSpec {
    /// Task type
    pub task_type: String,
    
    /// Task input
    pub input: serde_json::Value,
    
    /// Priority (1-10)
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_priority() -> i32 { 5 }

/// SentientAgent wrapper
#[derive(Debug, Clone)]
pub struct SentientAgent {
    pub spec: SentientAgentSpec,
}

impl SentientAgent {
    pub fn new(spec: SentientAgentSpec) -> Self {
        Self { spec }
    }
}

/// SentientTask wrapper
#[derive(Debug, Clone)]
pub struct SentientTask {
    pub spec: SentientTaskSpec,
}

impl SentientTask {
    pub fn new(spec: SentientTaskSpec) -> Self {
        Self { spec }
    }
}

/// Kubernetes Operator
pub struct Operator {
    namespace: String,
}

impl Operator {
    pub fn new(namespace: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
        }
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting SENTIENT operator in namespace: {}", self.namespace);
        Ok(())
    }
}

/// Agent Deployment helper
pub struct AgentDeployment {
    name: String,
    replicas: i32,
}

impl AgentDeployment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            replicas: 1,
        }
    }
    
    pub fn with_replicas(mut self, replicas: i32) -> Self {
        self.replicas = replicas;
        self
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn replicas(&self) -> i32 {
        self.replicas
    }
}

/// Task Dispatcher
pub struct TaskDispatcher {
    queue_size: usize,
}

impl TaskDispatcher {
    pub fn new() -> Self {
        Self { queue_size: 100 }
    }
    
    pub fn with_queue_size(mut self, size: usize) -> Self {
        self.queue_size = size;
        self
    }
}

impl Default for TaskDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
