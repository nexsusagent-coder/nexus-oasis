//! ─── SENTIENT Cluster Tests ───

#[cfg(test)]
mod tests {
    use crate::crds::{SentientAgentSpec, AgentType, ModelConfig};

    #[test]
    fn test_agent_spec_default() {
        let spec = SentientAgentSpec::default();
        
        assert_eq!(spec.replicas, 1);
        assert_eq!(spec.agent_type, AgentType::Worker);
        assert!(!spec.voice_enabled);
        assert!(spec.channels.is_empty());
        assert!(spec.skills.is_empty());
    }

    #[test]
    fn test_model_config_default() {
        let config = ModelConfig::default();
        
        assert_eq!(config.provider, "openai");
        assert_eq!(config.model, "gpt-4o");
        assert!(config.api_key_secret.is_none());
    }
}
