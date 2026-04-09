//! ─── Agent Deployment ───

use crate::crds::{SentientAgent, SentientAgentSpec, AgentType, ModelConfig, ResourceRequirements};

/// Agent deployment builder
pub struct AgentDeployment {
    name: String,
    spec: SentientAgentSpec,
}

impl AgentDeployment {
    /// Create new agent deployment
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            spec: SentientAgentSpec::default(),
        }
    }
    
    /// Set number of replicas
    pub fn with_replicas(mut self, replicas: i32) -> Self {
        self.spec.replicas = replicas;
        self
    }
    
    /// Set agent type
    pub fn with_type(mut self, agent_type: AgentType) -> Self {
        self.spec.agent_type = agent_type;
        self
    }
    
    /// Set enabled channels
    pub fn with_channels(mut self, channels: Vec<&str>) -> Self {
        self.spec.channels = channels.into_iter().map(String::from).collect();
        self
    }
    
    /// Set model
    pub fn with_model(mut self, provider: &str, model: &str) -> Self {
        self.spec.model = ModelConfig {
            provider: provider.into(),
            model: model.into(),
            api_key_secret: None,
            parameters: std::collections::BTreeMap::new(),
        };
        self
    }
    
    /// Set model with API key secret
    pub fn with_model_secret(mut self, provider: &str, model: &str, secret: &str) -> Self {
        self.spec.model = ModelConfig {
            provider: provider.into(),
            model: model.into(),
            api_key_secret: Some(secret.into()),
            parameters: std::collections::BTreeMap::new(),
        };
        self
    }
    
    /// Set resource requirements
    pub fn with_resources(mut self, memory: &str, cpu: &str) -> Self {
        self.spec.resources = ResourceRequirements {
            memory: Some(memory.into()),
            cpu: Some(cpu.into()),
            gpu: None,
        };
        self
    }
    
    /// Set GPU requirement
    pub fn with_gpu(mut self, gpu: &str) -> Self {
        self.spec.resources.gpu = Some(gpu.into());
        self
    }
    
    /// Enable voice
    pub fn with_voice(mut self, enabled: bool) -> Self {
        self.spec.voice_enabled = enabled;
        self
    }
    
    /// Add skills
    pub fn with_skills(mut self, skills: Vec<&str>) -> Self {
        self.spec.skills = skills.into_iter().map(String::from).collect();
        self
    }
    
    /// Add environment variable
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.spec.env.insert(key.into(), value.into());
        self
    }
    
    /// Add secret reference
    pub fn with_secret(mut self, name: &str, key: &str) -> Self {
        self.spec.secrets.push(crate::crds::SecretRef {
            name: name.into(),
            key: key.into(),
        });
        self
    }
    
    /// Build the CRD
    pub fn build(self) -> SentientAgent {
        SentientAgent {
            metadata: kube::api::ObjectMeta {
                name: Some(self.name),
                ..Default::default()
            },
            spec: self.spec,
            status: None,
        }
    }
    
    /// Deploy to cluster
    pub async fn deploy(self) -> Result<SentientAgent, DeployError> {
        let agent = self.build();
        
        // Get Kubernetes client
        let config = kube::Config::infer().await
            .map_err(|e| DeployError::Config(e.to_string()))?;
        
        let client = kube::Client::try_from(config)
            .map_err(|e| DeployError::Config(e.to_string()))?;
        
        let namespace = agent.metadata.namespace.as_deref().unwrap_or("default");
        let api: kube::Api<SentientAgent> = kube::Api::namespaced(client, namespace);
        
        // Create the agent
        let created = api.create(&kube::api::PostParams::default(), &agent).await
            .map_err(DeployError::Kube)?;
        
        Ok(created)
    }
}

/// Deployment error
#[derive(Debug, thiserror::Error)]
pub enum DeployError {
    #[error("Config error: {0}")]
    Config(String),
    
    #[error("Kubernetes error: {0}")]
    Kube(#[from] kube::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_builder() {
        let agent = AgentDeployment::new("test-agent")
            .with_replicas(3)
            .with_type(AgentType::Worker)
            .with_channels(vec!["telegram", "discord"])
            .with_model("openai", "gpt-4o")
            .with_resources("2Gi", "1")
            .with_voice(true)
            .with_skills(vec!["translator", "code-review"])
            .build();
        
        assert_eq!(agent.spec.replicas, 3);
        assert_eq!(agent.spec.channels.len(), 2);
        assert!(agent.spec.voice_enabled);
    }
}
