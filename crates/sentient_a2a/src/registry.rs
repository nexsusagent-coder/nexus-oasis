//! A2A Agent Registry

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::{Agent, AgentId, AgentCapability, A2AError, A2AResult};

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Max agents
    pub max_agents: usize,
    /// Heartbeat timeout (seconds)
    pub heartbeat_timeout: u64,
    /// Enable auto-discovery
    pub auto_discovery: bool,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            max_agents: 1000,
            heartbeat_timeout: 30,
            auto_discovery: true,
        }
    }
}

/// Agent registry
pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<String, Agent>>>,
    capability_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    config: RegistryConfig,
}

impl AgentRegistry {
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            capability_index: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register an agent
    pub async fn register(&self, agent: Agent) -> A2AResult<()> {
        let id_str = agent.id.as_str();
        
        // Check capacity
        let agents = self.agents.read().await;
        if agents.len() >= self.config.max_agents {
            return Err(A2AError::RegistrationFailed("Registry full".to_string()));
        }
        drop(agents);

        // Update capability index
        let mut cap_index = self.capability_index.write().await;
        for cap in &agent.capabilities {
            cap_index
                .entry(cap.name.clone())
                .or_insert_with(Vec::new)
                .push(id_str.clone());
        }
        drop(cap_index);

        // Register agent
        let mut agents = self.agents.write().await;
        agents.insert(id_str, agent);

        Ok(())
    }

    /// Unregister an agent
    pub async fn unregister(&self, id: &AgentId) -> A2AResult<()> {
        let id_str = id.as_str();
        
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.remove(&id_str) {
            // Update capability index
            let mut cap_index = self.capability_index.write().await;
            for cap in &agent.capabilities {
                if let Some(agent_ids) = cap_index.get_mut(&cap.name) {
                    agent_ids.retain(|aid| aid != &id_str);
                }
            }
            Ok(())
        } else {
            Err(A2AError::AgentNotFound(id_str))
        }
    }

    /// Get agent by ID
    pub async fn get(&self, id: &AgentId) -> A2AResult<Agent> {
        let agents = self.agents.read().await;
        agents
            .get(&id.as_str())
            .cloned()
            .ok_or_else(|| A2AError::AgentNotFound(id.as_str()))
    }

    /// Find agents by capability
    pub async fn find_by_capability(&self, capability: &str) -> Vec<Agent> {
        let cap_index = self.capability_index.read().await;
        let agents = self.agents.read().await;

        cap_index
            .get(capability)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| agents.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Find best agent for capability (lowest load)
    pub async fn find_best_for_capability(&self, capability: &str) -> Option<Agent> {
        let agents = self.find_by_capability(capability).await;
        agents
            .into_iter()
            .filter(|a| a.is_available())
            .min_by(|a, b| a.load_factor().partial_cmp(&b.load_factor()).unwrap())
    }

    /// List all agents
    pub async fn list(&self) -> Vec<Agent> {
        let agents = self.agents.read().await;
        agents.values().cloned().collect()
    }

    /// List active agents
    pub async fn list_active(&self) -> Vec<Agent> {
        let agents = self.agents.read().await;
        agents
            .values()
            .filter(|a| a.is_available())
            .cloned()
            .collect()
    }

    /// Update heartbeat
    pub async fn heartbeat(&self, id: &AgentId) -> A2AResult<()> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&id.as_str()) {
            agent.metadata.last_seen = Utc::now();
            Ok(())
        } else {
            Err(A2AError::AgentNotFound(id.as_str()))
        }
    }

    /// Remove stale agents
    pub async fn cleanup_stale(&self) -> usize {
        let timeout = chrono::Duration::seconds(self.config.heartbeat_timeout as i64);
        let now = Utc::now();

        let mut agents = self.agents.write().await;
        let stale: Vec<String> = agents
            .iter()
            .filter(|(_, a)| now - a.metadata.last_seen > timeout)
            .map(|(id, _)| id.clone())
            .collect();

        let removed = stale.len();
        for id in stale {
            agents.remove(&id);
        }

        removed
    }

    /// Get registry stats
    pub async fn stats(&self) -> RegistryStats {
        let agents = self.agents.read().await;
        let total = agents.len();
        let active = agents.values().filter(|a| a.is_available()).count();
        let capabilities = self.capability_index.read().await.len();

        RegistryStats {
            total_agents: total,
            active_agents: active,
            total_capabilities: capabilities,
        }
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new(RegistryConfig::default())
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_capabilities: usize,
}
