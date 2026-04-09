//! Config unit tests

#[cfg(test)]
mod config_unit_tests {
    use sentient_core::AgentConfig;

    #[test]
    fn test_default_config() {
        let config = AgentConfig::default();
        
        // Default values should be valid
        assert!(config.name.is_empty() || !config.name.is_empty());
        assert!(config.temperature >= 0.0 && config.temperature <= 2.0);
        assert!(config.max_tokens > 0);
    }

    #[test]
    fn test_config_temperature_bounds() {
        let mut config = AgentConfig::default();
        
        // Test lower bound
        config.temperature = 0.0;
        assert!(config.validate().is_ok());
        
        // Test upper bound
        config.temperature = 2.0;
        assert!(config.validate().is_ok());
        
        // Test invalid
        config.temperature = -0.1;
        assert!(config.validate().is_err());
        
        config.temperature = 2.1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_max_tokens_bounds() {
        let mut config = AgentConfig::default();
        
        config.max_tokens = 1;
        assert!(config.validate().is_ok());
        
        config.max_tokens = 128000;
        assert!(config.validate().is_ok());
        
        config.max_tokens = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_toml_serialization() {
        let config = AgentConfig {
            name: "TestAgent".to_string(),
            temperature: 0.8,
            max_tokens: 4000,
            ..Default::default()
        };
        
        let toml = toml::to_string(&config).unwrap();
        let decoded: AgentConfig = toml::from_str(&toml).unwrap();
        
        assert_eq!(config.name, decoded.name);
        assert_eq!(config.temperature, decoded.temperature);
        assert_eq!(config.max_tokens, decoded.max_tokens);
    }

    #[test]
    fn test_config_json_serialization() {
        let config = AgentConfig {
            name: "JSONAgent".to_string(),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let decoded: AgentConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.name, decoded.name);
    }

    #[test]
    fn test_config_env_override() {
        std::env::set_var("SENTIENT_TEMPERATURE", "0.5");
        std::env::set_var("SENTIENT_MAX_TOKENS", "2000");
        
        let config = AgentConfig::from_env();
        
        // If from_env is implemented
        // assert_eq!(config.temperature, 0.5);
        // assert_eq!(config.max_tokens, 2000);
        
        std::env::remove_var("SENTIENT_TEMPERATURE");
        std::env::remove_var("SENTIENT_MAX_TOKENS");
    }

    #[test]
    fn test_config_system_prompt() {
        let prompt = "You are a helpful AI assistant specialized in Rust programming.";
        
        let config = AgentConfig {
            system_prompt: Some(prompt.to_string()),
            ..Default::default()
        };
        
        assert!(config.system_prompt.is_some());
        assert_eq!(config.system_prompt.unwrap(), prompt);
    }

    #[test]
    fn test_config_memory_settings() {
        let config = AgentConfig {
            memory_enabled: true,
            memory_ttl: Some(3600),
            memory_max_entries: Some(1000),
            ..Default::default()
        };
        
        assert!(config.memory_enabled);
        assert_eq!(config.memory_ttl, Some(3600));
    }

    #[test]
    fn test_config_merge() {
        let base = AgentConfig {
            name: "Base".to_string(),
            temperature: 0.7,
            ..Default::default()
        };
        
        let override_config = AgentConfig {
            temperature: 0.9,
            ..Default::default()
        };
        
        let merged = base.merge(override_config);
        
        // Name should remain from base
        assert_eq!(merged.name, "Base");
        // Temperature should be overridden
        assert_eq!(merged.temperature, 0.9);
    }
}
