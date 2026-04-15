//! ═══════════════════════════════════════════════════════════════════════════════
//!  Multi-Agent Coordination System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Coordination and communication between multiple agents:
//! - Agent registry
//! - Task delegation
//! - Inter-agent messaging
//! - Conflict resolution

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentCapability {
    CodeGeneration,
    CodeReview,
    Testing,
    Documentation,
    Research,
    DataAnalysis,
    WebBrowsing,
    FileOperations,
    SystemCommands,
    APIIntegration,
    Custom(String),
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Busy { task_id: String },
    Error { message: String },
    Offline,
}

/// Agent info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent type/specialization
    pub agent_type: String,
    /// Capabilities
    pub capabilities: Vec<AgentCapability>,
    /// Current status
    pub status: AgentStatus,
    /// Current load (0.0 - 1.0)
    pub load: f32,
    /// Performance metrics
    pub metrics: AgentMetrics,
    /// Tags for routing
    pub tags: Vec<String>,
    /// Last heartbeat
    pub last_heartbeat: DateTime<Utc>,
}

/// Agent performance metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// Tasks completed
    pub tasks_completed: u64,
    /// Tasks failed
    pub tasks_failed: u64,
    /// Average task duration (ms)
    pub avg_duration_ms: f64,
    /// Total tokens used
    pub tokens_used: u64,
    /// Success rate
    pub success_rate: f32,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COORDINATION MESSAGES
// ═══════════════════════════════════════════════════════════════════════════════

/// Inter-agent message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Message ID
    pub id: String,
    /// Sender ID
    pub from: String,
    /// Recipient ID (or "broadcast")
    pub to: String,
    /// Message type
    pub message_type: AgentMessageType,
    /// Payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Correlation ID for request/response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}

/// Message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessageType {
    /// Task delegation
    TaskDelegation,
    /// Task result
    TaskResult,
    /// Status update
    StatusUpdate,
    /// Request for help
    HelpRequest,
    /// Knowledge sharing
    KnowledgeShare,
    /// Conflict notification
    Conflict,
    /// Coordination request
    Coordination,
}

/// Task for delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedTask {
    /// Task ID
    pub id: String,
    /// Task description
    pub description: String,
    /// Required capabilities
    pub required_capabilities: Vec<AgentCapability>,
    /// Priority (1-10)
    pub priority: u8,
    /// Deadline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    /// Parent task ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Context data
    pub context: HashMap<String, serde_json::Value>,
}

/// Task result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,
    /// Agent that completed the task
    pub agent_id: String,
    /// Success status
    pub success: bool,
    /// Result data
    pub result: Option<serde_json::Value>,
    /// Error message
    pub error: Option<String>,
    /// Duration (ms)
    pub duration_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COORDINATION ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum CoordinationError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Agent unavailable: {0}")]
    AgentUnavailable(String),
    
    #[error("No suitable agent for task")]
    NoSuitableAgent,
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Conflict detected: {0}")]
    Conflict(String),
    
    #[error("Timeout")]
    Timeout,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COORDINATION MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Coordination manager
pub struct CoordinationManager {
    agents: Arc<RwLock<HashMap<String, AgentInfo>>>,
    message_queues: Arc<RwLock<HashMap<String, VecDeque<AgentMessage>>>>,
    pending_tasks: Arc<RwLock<HashMap<String, DelegatedTask>>>,
    task_results: Arc<RwLock<HashMap<String, TaskResult>>>,
}

impl CoordinationManager {
    /// Create a new coordination manager
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            message_queues: Arc::new(RwLock::new(HashMap::new())),
            pending_tasks: Arc::new(RwLock::new(HashMap::new())),
            task_results: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register an agent
    pub async fn register_agent(&self, agent: AgentInfo) {
        let id = agent.id.clone();
        self.agents.write().await.insert(id.clone(), agent);
        self.message_queues.write().await.insert(id, VecDeque::new());
    }
    
    /// Unregister an agent
    pub async fn unregister_agent(&self, agent_id: &str) -> Result<(), CoordinationError> {
        let mut agents = self.agents.write().await;
        agents.remove(agent_id)
            .ok_or_else(|| CoordinationError::AgentNotFound(agent_id.to_string()))?;
        
        self.message_queues.write().await.remove(agent_id);
        Ok(())
    }
    
    /// Update agent heartbeat
    pub async fn heartbeat(&self, agent_id: &str) -> Result<(), CoordinationError> {
        let mut agents = self.agents.write().await;
        let agent = agents.get_mut(agent_id)
            .ok_or_else(|| CoordinationError::AgentNotFound(agent_id.to_string()))?;
        
        agent.last_heartbeat = Utc::now();
        Ok(())
    }
    
    /// Find best agent for a task
    pub async fn find_agent(&self, task: &DelegatedTask) -> Option<String> {
        let agents = self.agents.read().await;
        
        let mut best_agent: Option<(&String, &AgentInfo)> = None;
        let mut best_score = 0.0;
        
        for (id, agent) in agents.iter() {
            // Check status
            if !matches!(agent.status, AgentStatus::Idle) {
                continue;
            }
            
            // Check capabilities
            let capability_match = task.required_capabilities.iter()
                .filter(|c| agent.capabilities.contains(c))
                .count();
            
            if capability_match < task.required_capabilities.len() {
                continue;
            }
            
            // Calculate score (lower load = better)
            let score = (1.0 - agent.load) * (agent.metrics.success_rate)
                + (capability_match as f32 * 0.1);
            
            if score > best_score {
                best_score = score;
                best_agent = Some((id, agent));
            }
        }
        
        best_agent.map(|(id, _)| id.clone())
    }
    
    /// Delegate a task to an agent
    pub async fn delegate_task(&self, task: DelegatedTask) -> Result<String, CoordinationError> {
        // Find suitable agent
        let agent_id = self.find_agent(&task).await
            .ok_or(CoordinationError::NoSuitableAgent)?;
        
        // Update agent status
        {
            let mut agents = self.agents.write().await;
            let agent = agents.get_mut(&agent_id).unwrap();
            agent.status = AgentStatus::Busy { task_id: task.id.clone() };
            agent.load = (agent.load + 0.2).min(1.0);
        }
        
        // Send delegation message
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: "coordinator".to_string(),
            to: agent_id.clone(),
            message_type: AgentMessageType::TaskDelegation,
            payload: serde_json::to_value(&task).unwrap(),
            timestamp: Utc::now(),
            correlation_id: None,
        };
        
        self.send_message(message).await;
        
        // Store pending task
        self.pending_tasks.write().await.insert(task.id.clone(), task);
        
        Ok(agent_id)
    }
    
    /// Submit task result
    pub async fn submit_result(&self, result: TaskResult) -> Result<(), CoordinationError> {
        // Update agent status
        {
            let mut agents = self.agents.write().await;
            let agent = agents.get_mut(&result.agent_id)
                .ok_or_else(|| CoordinationError::AgentNotFound(result.agent_id.clone()))?;
            
            agent.status = AgentStatus::Idle;
            agent.load = (agent.load - 0.2).max(0.0);
            
            // Update metrics
            agent.metrics.tasks_completed += 1;
            if !result.success {
                agent.metrics.tasks_failed += 1;
            }
        }
        
        // Remove from pending
        self.pending_tasks.write().await.remove(&result.task_id);
        
        // Store result
        self.task_results.write().await.insert(result.task_id.clone(), result);
        
        Ok(())
    }
    
    /// Send message between agents
    pub async fn send_message(&self, message: AgentMessage) {
        let mut queues = self.message_queues.write().await;
        
        if message.to == "broadcast" {
            // Send to all agents
            for queue in queues.values_mut() {
                queue.push_back(message.clone());
            }
        } else if let Some(queue) = queues.get_mut(&message.to) {
            queue.push_back(message);
        }
    }
    
    /// Receive messages for an agent
    pub async fn receive_messages(&self, agent_id: &str) -> Vec<AgentMessage> {
        let mut queues = self.message_queues.write().await;
        
        if let Some(queue) = queues.get_mut(agent_id) {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }
    
    /// Get all agents
    pub async fn get_agents(&self) -> Vec<AgentInfo> {
        self.agents.read().await.values().cloned().collect()
    }
    
    /// Get agent by ID
    pub async fn get_agent(&self, agent_id: &str) -> Option<AgentInfo> {
        self.agents.read().await.get(agent_id).cloned()
    }
    
    /// Request help from other agents
    pub async fn request_help(
        &self,
        from_agent: &str,
        required_capabilities: Vec<AgentCapability>,
        description: String,
    ) -> Result<String, CoordinationError> {
        let task = DelegatedTask {
            id: uuid::Uuid::new_v4().to_string(),
            description,
            required_capabilities,
            priority: 5,
            deadline: None,
            parent_id: None,
            context: HashMap::new(),
        };
        
        // Find agent (excluding requester)
        let agent_id = {
            let agents = self.agents.read().await;
            agents.iter()
                .filter(|(id, a)| {
                    *id != from_agent &&
                    matches!(a.status, AgentStatus::Idle) &&
                    task.required_capabilities.iter().all(|c| a.capabilities.contains(c))
                })
                .min_by(|a, b| a.1.load.partial_cmp(&b.1.load).unwrap())
                .map(|(id, _)| id.clone())
                .ok_or(CoordinationError::NoSuitableAgent)?
        };
        
        // Send help request
        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            from: from_agent.to_string(),
            to: agent_id.clone(),
            message_type: AgentMessageType::HelpRequest,
            payload: serde_json::to_value(&task).unwrap(),
            timestamp: Utc::now(),
            correlation_id: None,
        };
        
        self.send_message(message).await;
        
        Ok(agent_id)
    }
    
    /// Check for conflicts
    pub async fn detect_conflicts(&self) -> Vec<(String, String)> {
        // Detect agents working on same/similar tasks
        let agents = self.agents.read().await;
        let mut conflicts = Vec::new();
        
        let busy_agents: Vec<_> = agents.iter()
            .filter(|(_, a)| matches!(a.status, AgentStatus::Busy { .. }))
            .collect();
        
        for i in 0..busy_agents.len() {
            for j in (i + 1)..busy_agents.len() {
                let (id1, a1) = busy_agents[i];
                let (id2, a2) = busy_agents[j];
                
                // Check if they have overlapping capabilities
                let overlap = a1.capabilities.iter()
                    .filter(|c| a2.capabilities.contains(c))
                    .count();
                
                if overlap > 2 {
                    conflicts.push((id1.clone(), id2.clone()));
                }
            }
        }
        
        conflicts
    }
}

impl Default for CoordinationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_registration() {
        let manager = CoordinationManager::new();
        
        let agent = AgentInfo {
            id: "agent-1".to_string(),
            name: "Test Agent".to_string(),
            agent_type: "coder".to_string(),
            capabilities: vec![AgentCapability::CodeGeneration],
            status: AgentStatus::Idle,
            load: 0.0,
            metrics: AgentMetrics::default(),
            tags: vec![],
            last_heartbeat: Utc::now(),
        };
        
        manager.register_agent(agent).await;
        
        let agents = manager.get_agents().await;
        assert_eq!(agents.len(), 1);
    }
    
    #[tokio::test]
    async fn test_task_delegation() {
        let manager = CoordinationManager::new();
        
        // Register agent
        manager.register_agent(AgentInfo {
            id: "agent-1".to_string(),
            name: "Test".to_string(),
            agent_type: "coder".to_string(),
            capabilities: vec![AgentCapability::CodeGeneration],
            status: AgentStatus::Idle,
            load: 0.0,
            metrics: AgentMetrics::default(),
            tags: vec![],
            last_heartbeat: Utc::now(),
        }).await;
        
        // Delegate task
        let task = DelegatedTask {
            id: "task-1".to_string(),
            description: "Write code".to_string(),
            required_capabilities: vec![AgentCapability::CodeGeneration],
            priority: 5,
            deadline: None,
            parent_id: None,
            context: HashMap::new(),
        };
        
        let agent_id = manager.delegate_task(task).await.unwrap();
        assert_eq!(agent_id, "agent-1");
        
        // Check agent is busy
        let agent = manager.get_agent("agent-1").await.unwrap();
        assert!(matches!(agent.status, AgentStatus::Busy { .. }));
    }
    
    #[tokio::test]
    async fn test_messaging() {
        let manager = CoordinationManager::new();
        
        manager.register_agent(AgentInfo {
            id: "agent-1".to_string(),
            name: "Test".to_string(),
            agent_type: "test".to_string(),
            capabilities: vec![],
            status: AgentStatus::Idle,
            load: 0.0,
            metrics: AgentMetrics::default(),
            tags: vec![],
            last_heartbeat: Utc::now(),
        }).await;
        
        // Send message
        let msg = AgentMessage {
            id: "msg-1".to_string(),
            from: "system".to_string(),
            to: "agent-1".to_string(),
            message_type: AgentMessageType::StatusUpdate,
            payload: serde_json::json!({}),
            timestamp: Utc::now(),
            correlation_id: None,
        };
        
        manager.send_message(msg).await;
        
        // Receive
        let messages = manager.receive_messages("agent-1").await;
        assert_eq!(messages.len(), 1);
    }
}
