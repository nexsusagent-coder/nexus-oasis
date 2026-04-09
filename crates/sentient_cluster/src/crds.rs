//! ─── Custom Resource Definitions ───

use kube::{CustomResource, Resource, Api};
use kube::api::{ObjectMeta, PostParams};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use k8s_openapi::api::apps::v1::Deployment;

/// SENTIENT Agent CRD
#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[kube(
    group = "sentient.ai",
    version = "v1",
    kind = "SentientAgent",
    namespaced,
    status = "SentientAgentStatus",
    derive = "PartialEq",
    derive = "Default"
)]
pub struct SentientAgentSpec {
    /// Number of replicas
    #[serde(default = "default_replicas")]
    pub replicas: i32,
    
    /// Agent type
    pub agent_type: AgentType,
    
    /// Enabled channels
    #[serde(default)]
    pub channels: Vec<String>,
    
    /// Model configuration
    pub model: ModelConfig,
    
    /// Resource requirements
    #[serde(default)]
    pub resources: ResourceRequirements,
    
    /// Voice enabled
    #[serde(default)]
    pub voice_enabled: bool,
    
    /// Skills to install
    #[serde(default)]
    pub skills: Vec<String>,
    
    /// Environment variables
    #[serde(default)]
    pub env: std::collections::BTreeMap<String, String>,
    
    /// Secret references
    #[serde(default)]
    pub secrets: Vec<SecretRef>,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SentientAgentStatus {
    /// Current replicas
    pub current_replicas: i32,
    
    /// Ready replicas
    pub ready_replicas: i32,
    
    /// Agent phase
    pub phase: AgentPhase,
    
    /// Conditions
    #[serde(default)]
    pub conditions: Vec<AgentCondition>,
    
    /// Last update time
    pub last_update: Option<String>,
}

/// Agent type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
pub enum AgentType {
    #[default]
    Worker,
    Orchestrator,
    Gateway,
    Voice,
}

/// Agent phase
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
pub enum AgentPhase {
    #[default]
    Pending,
    Running,
    Degraded,
    Failed,
    Terminating,
}

/// Agent condition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentCondition {
    pub type_: String,
    pub status: String,
    pub reason: Option<String>,
    pub message: Option<String>,
    pub last_transition_time: Option<String>,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    #[serde(default)]
    pub api_key_secret: Option<String>,
    #[serde(default)]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: "openai".into(),
            model: "gpt-4o".into(),
            api_key_secret: None,
            parameters: std::collections::BTreeMap::new(),
        }
    }
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ResourceRequirements {
    pub memory: Option<String>,
    pub cpu: Option<String>,
    pub gpu: Option<String>,
}

/// Secret reference
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SecretRef {
    pub name: String,
    pub key: String,
}

fn default_replicas() -> i32 { 1 }

/// SENTIENT Task CRD
#[derive(CustomResource, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[kube(
    group = "sentient.ai",
    version = "v1",
    kind = "SentientTask",
    namespaced,
    status = "SentientTaskStatus"
)]
pub struct SentientTaskSpec {
    /// Task type
    pub task_type: TaskType,
    
    /// Task input
    pub input: serde_json::Value,
    
    /// Target agents (empty = any available)
    #[serde(default)]
    pub target_agents: Vec<String>,
    
    /// Priority (1-10)
    #[serde(default = "default_priority")]
    pub priority: i32,
    
    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    
    /// Retry count
    #[serde(default)]
    pub retries: u32,
    
    /// Callback URL
    #[serde(default)]
    pub callback_url: Option<String>,
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct SentientTaskStatus {
    pub phase: TaskPhase,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub assigned_agent: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Task type
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum TaskType {
    Chat,
    Voice,
    Channel,
    Skill,
    Custom,
}

/// Task phase
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
pub enum TaskPhase {
    #[default]
    Pending,
    Queued,
    Running,
    Completed,
    Failed,
    Timeout,
}

fn default_priority() -> i32 { 5 }
fn default_timeout() -> u32 { 300 }

/// Apply all CRDs to cluster
pub async fn apply_all(client: kube::Client) -> Result<(), kube::Error> {
    let crds = vec![
        SentientAgent::crd(),
        SentientTask::crd(),
    ];
    
    let api: Api<k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition> = 
        Api::all(client);
    
    for crd in crds {
        let pp = PostParams::default();
        api.create(&pp, &crd).await?;
    }
    
    Ok(())
}
