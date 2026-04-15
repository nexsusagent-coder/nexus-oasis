//! ─── Agent Farm ───
//!
//! Manage 20+ parallel agents with:
//! - Lock-based file coordination
//! - Auto-recovery
//! - Context window management
//! - Task distribution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::{AgentError, AgentResult};

/// Agent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Working,
    Waiting,
    Error,
    Stopped,
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Idle
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    pub model: String,
    pub max_context_tokens: u32,
    pub priority: u8,
}

impl AgentConfig {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            model: "gpt-4".into(),
            max_context_tokens: 128_000,
            priority: 5,
        }
    }
    
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
    
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Running agent info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub config: AgentConfig,
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub tokens_used: u64,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub error_count: u32,
}

impl AgentInfo {
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            status: AgentStatus::Idle,
            current_task: None,
            tokens_used: 0,
            last_heartbeat: chrono::Utc::now(),
            error_count: 0,
        }
    }
    
    pub fn touch(&mut self) {
        self.last_heartbeat = chrono::Utc::now();
    }
    
    pub fn is_stale(&self, timeout_secs: u64) -> bool {
        let elapsed = (chrono::Utc::now() - self.last_heartbeat).num_seconds() as u64;
        elapsed > timeout_secs
    }
}

/// Task for agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FarmTask {
    pub id: String,
    pub task_type: String,
    pub payload: serde_json::Value,
    pub priority: u8,
    pub assigned_agent: Option<String>,
    pub retries: u32,
    pub max_retries: u32,
}

impl FarmTask {
    pub fn new(task_type: &str, payload: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            task_type: task_type.to_string(),
            payload,
            priority: 5,
            assigned_agent: None,
            retries: 0,
            max_retries: 3,
        }
    }
    
    pub fn high_priority(mut self) -> Self {
        self.priority = 10;
        self
    }
}

/// Agent Farm configuration
#[derive(Debug, Clone)]
pub struct FarmConfig {
    pub max_agents: usize,
    pub lock_dir: PathBuf,
    pub heartbeat_timeout_secs: u64,
    pub max_retries: u32,
}

impl Default for FarmConfig {
    fn default() -> Self {
        Self {
            max_agents: 20,
            lock_dir: PathBuf::from("/tmp/sentient_farm"),
            heartbeat_timeout_secs: 60,
            max_retries: 3,
        }
    }
}

/// Agent Farm - manages multiple parallel agents
pub struct AgentFarm {
    config: FarmConfig,
    agents: Arc<RwLock<HashMap<String, AgentInfo>>>,
    tasks: Arc<RwLock<Vec<FarmTask>>>,
    running: bool,
}

impl AgentFarm {
    pub fn new(config: FarmConfig) -> Self {
        Self {
            config,
            agents: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(RwLock::new(Vec::new())),
            running: false,
        }
    }
    
    /// Spawn a new agent
    pub async fn spawn(&self, config: AgentConfig) -> AgentResult<String> {
        let agents = self.agents.read().await;
        if agents.len() >= self.config.max_agents {
            return Err(AgentError::CapacityReached);
        }
        drop(agents);
        
        let id = config.id.clone();
        let info = AgentInfo::new(config);
        
        let mut agents = self.agents.write().await;
        agents.insert(id.clone(), info);
        
        tracing::info!("Spawned agent: {}", id);
        Ok(id)
    }
    
    /// Stop an agent by ID
    pub async fn stop_agent(&self, id: &str) -> Option<AgentInfo> {
        let mut agents = self.agents.write().await;
        if let Some(mut info) = agents.remove(id) {
            info.status = AgentStatus::Stopped;
            tracing::info!("Stopped agent: {}", id);
            return Some(info);
        }
        None
    }
    
    /// Submit a task
    pub async fn submit(&self, task: FarmTask) -> String {
        let id = task.id.clone();
        let mut tasks = self.tasks.write().await;
        tasks.push(task);
        tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
        id
    }
    
    /// Get next available task
    pub async fn get_task(&self, agent_id: &str) -> Option<FarmTask> {
        let mut tasks = self.tasks.write().await;
        for task in tasks.iter_mut() {
            if task.assigned_agent.is_none() {
                task.assigned_agent = Some(agent_id.to_string());
                return Some(task.clone());
            }
        }
        None
    }
    
    /// Complete a task
    pub async fn complete_task(&self, task_id: &str, success: bool) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
            if success {
                tasks.retain(|t| t.id != task_id);
            } else {
                task.retries += 1;
                if task.retries >= task.max_retries {
                    tasks.retain(|t| t.id != task_id);
                } else {
                    task.assigned_agent = None;
                }
            }
        }
    }
    
    /// Start the farm
    pub async fn start(&mut self) -> AgentResult<()> {
        if self.running { return Ok(()); }
        
        self.running = true;
        
        // Create lock directory
        tokio::fs::create_dir_all(&self.config.lock_dir).await?;
        
        tracing::info!("Agent farm started (max: {})", self.config.max_agents);
        Ok(())
    }
    
    /// Shutdown the farm
    pub async fn shutdown(&mut self) {
        self.running = false;
        
        let mut agents = self.agents.write().await;
        for (_, info) in agents.iter_mut() {
            info.status = AgentStatus::Stopped;
        }
        
        tracing::info!("Agent farm stopped");
    }
    
    /// Recover stale agents
    pub async fn recover_stale(&self) -> Vec<String> {
        let mut recovered = Vec::new();
        let mut agents = self.agents.write().await;
        
        for (id, info) in agents.iter_mut() {
            if info.is_stale(self.config.heartbeat_timeout_secs) {
                tracing::warn!("Recovering stale agent: {}", id);
                info.status = AgentStatus::Idle;
                info.current_task = None;
                info.error_count += 1;
                recovered.push(id.clone());
            }
        }
        
        recovered
    }
    
    /// Get farm status
    pub async fn status(&self) -> FarmStatus {
        let agents = self.agents.read().await;
        let tasks = self.tasks.read().await;
        
        let mut status = FarmStatus::default();
        status.total_agents = agents.len();
        status.idle_agents = agents.values().filter(|a| a.status == AgentStatus::Idle).count();
        status.working_agents = agents.values().filter(|a| a.status == AgentStatus::Working).count();
        status.queued_tasks = tasks.len();
        status.pending_tasks = tasks.iter().filter(|t| t.assigned_agent.is_none()).count();
        
        status
    }
    
    /// Acquire file lock for coordination
    pub async fn acquire_lock(&self, agent_id: &str, resource: &str) -> AgentResult<bool> {
        let lock_path = self.config.lock_dir.join(format!("{}.lock", resource));
        
        match tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
            .await
        {
            Ok(mut file) => {
                tokio::io::AsyncWriteExt::write_all(&mut file, agent_id.as_bytes()).await?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
    
    /// Release file lock
    pub async fn release_lock(&self, resource: &str) -> AgentResult<()> {
        let lock_path = self.config.lock_dir.join(format!("{}.lock", resource));
        tokio::fs::remove_file(&lock_path).await.ok();
        Ok(())
    }
}

impl Default for AgentFarm {
    fn default() -> Self {
        Self::new(FarmConfig::default())
    }
}

/// Farm status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FarmStatus {
    pub total_agents: usize,
    pub idle_agents: usize,
    pub working_agents: usize,
    pub queued_tasks: usize,
    pub pending_tasks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_farm_spawn() {
        let farm = AgentFarm::default();
        let config = AgentConfig::new("Test Agent");
        let id = farm.spawn(config).await.unwrap();
        assert!(!id.is_empty());
    }
    
    #[test]
    fn test_agent_config() {
        let config = AgentConfig::new("Test")
            .with_model("claude-3")
            .with_priority(10);
        assert_eq!(config.model, "claude-3");
    }
}
