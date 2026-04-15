//! A2A Protocol - Main protocol implementation

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    Agent, AgentId, AgentRegistry, RegistryConfig,
    Message, MessageType, MessageBuilder,
    Transport, TransportConfig, HttpTransport,
    A2AError, A2AResult,
};

/// Protocol configuration
#[derive(Debug, Clone)]
pub struct ProtocolConfig {
    /// Registry config
    pub registry: RegistryConfig,
    /// Transport config
    pub transport: TransportConfig,
    /// Default timeout
    pub default_timeout: u64,
    /// Enable message logging
    pub log_messages: bool,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            registry: RegistryConfig::default(),
            transport: TransportConfig::default(),
            default_timeout: 30,
            log_messages: true,
        }
    }
}

/// A2A Protocol implementation
pub struct A2AProtocol {
    registry: AgentRegistry,
    config: ProtocolConfig,
    local_agent: Arc<RwLock<Option<Agent>>>,
}

impl A2AProtocol {
    pub fn new(config: ProtocolConfig) -> Self {
        Self {
            registry: AgentRegistry::new(config.registry.clone()),
            config,
            local_agent: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize with local agent
    pub async fn init(&self, agent: Agent) -> A2AResult<()> {
        self.registry.register(agent.clone()).await?;
        let mut local = self.local_agent.write().await;
        *local = Some(agent);
        Ok(())
    }

    /// Send message to agent
    pub async fn send(&self, message: Message) -> A2AResult<()> {
        if self.config.log_messages {
            log::info!("A2A send: {} -> {} ({:?})", 
                message.from, message.to, message.message_type);
        }

        // Check if target exists
        if !message.is_broadcast() {
            let target_id = AgentId::new("", &message.to, "");
            // For now, just check if any agent with that name exists
            let agents = self.registry.list().await;
            let found = agents.iter().any(|a| a.id.name == message.to);
            if !found {
                return Err(A2AError::AgentNotFound(message.to.clone()));
            }
        }

        // Create transport for target
        // In real implementation, we'd look up the agent's endpoint
        Ok(())
    }

    /// Send request and wait for response
    pub async fn request(&self, to: &str, payload: serde_json::Value, timeout_ms: u64) -> A2AResult<Message> {
        let local = self.local_agent.read().await;
        let from = local
            .as_ref()
            .map(|a| a.id.as_str())
            .unwrap_or_else(|| "unknown".to_string());
        drop(local);

        let request = MessageBuilder::new(&from, to)
            .message_type(MessageType::Request)
            .payload(payload)
            .build();

        self.send(request.clone()).await?;

        // Wait for response (simplified)
        // In real implementation, we'd wait on a response channel
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let response = MessageBuilder::new(to, &from)
            .message_type(MessageType::Response)
            .correlation_id(&request.id)
            .payload(serde_json::json!({ "status": "ok" }))
            .build();

        Ok(response)
    }

    /// Broadcast to all agents
    pub async fn broadcast(&self, payload: serde_json::Value) -> A2AResult<Vec<Message>> {
        let local = self.local_agent.read().await;
        let from = local
            .as_ref()
            .map(|a| a.id.as_str())
            .unwrap_or_else(|| "unknown".to_string());
        drop(local);

        let message = MessageBuilder::new(&from, "broadcast")
            .message_type(MessageType::Notification)
            .payload(payload)
            .build();

        self.send(message).await?;

        // Return empty for now
        Ok(Vec::new())
    }

    /// Discover agents with capability
    pub async fn discover(&self, capability: &str) -> Vec<Agent> {
        self.registry.find_by_capability(capability).await
    }

    /// Discover best agent for capability
    pub async fn discover_best(&self, capability: &str) -> Option<Agent> {
        self.registry.find_best_for_capability(capability).await
    }

    /// Register agent
    pub async fn register_agent(&self, agent: Agent) -> A2AResult<()> {
        self.registry.register(agent).await
    }

    /// Unregister agent
    pub async fn unregister_agent(&self, id: &AgentId) -> A2AResult<()> {
        self.registry.unregister(id).await
    }

    /// List all agents
    pub async fn list_agents(&self) -> Vec<Agent> {
        self.registry.list().await
    }

    /// Get registry stats
    pub async fn stats(&self) -> crate::RegistryStats {
        self.registry.stats().await
    }

    /// Heartbeat
    pub async fn heartbeat(&self) -> A2AResult<()> {
        let local = self.local_agent.read().await;
        if let Some(ref agent) = *local {
            self.registry.heartbeat(&agent.id).await
        } else {
            Err(A2AError::AgentNotFound("local".to_string()))
        }
    }
}

impl Default for A2AProtocol {
    fn default() -> Self {
        Self::new(ProtocolConfig::default())
    }
}
