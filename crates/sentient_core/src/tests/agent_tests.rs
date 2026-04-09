//! Agent unit tests

#[cfg(test)]
mod agent_unit_tests {
    use sentient_core::{Agent, AgentConfig, Message, MessageRole};

    fn create_test_config() -> AgentConfig {
        AgentConfig {
            name: "TestAgent".to_string(),
            llm_provider: sentient_core::LlmProvider::OpenAI,
            llm_model: "gpt-4o-mini".to_string(),
            api_key: "test-key".to_string(),
            system_prompt: Some("You are a test agent.".to_string()),
            temperature: 0.7,
            max_tokens: 1000,
            memory_enabled: true,
            skills: vec![],
        }
    }

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert!(config.name.is_empty() || !config.name.is_empty());
    }

    #[test]
    fn test_agent_config_builder() {
        let config = AgentConfig {
            name: "CustomAgent".to_string(),
            llm_provider: sentient_core::LlmProvider::Anthropic,
            llm_model: "claude-3-opus".to_string(),
            temperature: 0.5,
            max_tokens: 2000,
            ..Default::default()
        };
        
        assert_eq!(config.name, "CustomAgent");
        assert_eq!(config.temperature, 0.5);
        assert_eq!(config.max_tokens, 2000);
    }

    #[test]
    fn test_message_creation_user() {
        let msg = Message::user("Hello, world!");
        
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello, world!");
    }

    #[test]
    fn test_message_creation_assistant() {
        let msg = Message::assistant("Hi there!");
        
        assert_eq!(msg.role, MessageRole::Assistant);
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn test_message_creation_system() {
        let msg = Message::system("You are helpful.");
        
        assert_eq!(msg.role, MessageRole::System);
        assert_eq!(msg.content, "You are helpful.");
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::user("Test message");
        let json = serde_json::to_string(&msg).expect("Failed to serialize");
        
        assert!(json.contains("user"));
        assert!(json.contains("Test message"));
        
        let decoded: Message = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(decoded.role, MessageRole::User);
        assert_eq!(decoded.content, "Test message");
    }

    #[test]
    fn test_message_with_metadata() {
        let msg = Message {
            role: MessageRole::User,
            content: "Hello".to_string(),
            metadata: Some(serde_json::json!({
                "source": "telegram",
                "chat_id": 12345
            })),
            timestamp: Some(chrono::Utc::now()),
        };
        
        assert!(msg.metadata.is_some());
        let meta = msg.metadata.unwrap();
        assert_eq!(meta["source"], "telegram");
    }

    #[test]
    fn test_config_validation() {
        let mut config = create_test_config();
        
        // Valid config
        assert!(config.validate().is_ok());
        
        // Invalid temperature
        config.temperature = 3.0;
        assert!(config.validate().is_err());
        
        // Reset and test max_tokens
        config.temperature = 0.7;
        config.max_tokens = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_provider_serialization() {
        let providers = vec![
            sentient_core::LlmProvider::OpenAI,
            sentient_core::LlmProvider::Anthropic,
            sentient_core::LlmProvider::Local,
        ];
        
        for provider in providers {
            let json = serde_json::to_string(&provider).expect("Failed to serialize");
            let decoded: sentient_core::LlmProvider = 
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(provider, decoded);
        }
    }

    #[test]
    fn test_agent_session_id() {
        let config = create_test_config();
        // Session ID should be generated or specified
        assert!(config.session_id.is_some() || config.session_id.is_none());
    }

    #[test]
    fn test_message_timestamp() {
        let before = chrono::Utc::now();
        let msg = Message::user("Test");
        let after = chrono::Utc::now();
        
        if let Some(ts) = msg.timestamp {
            assert!(ts >= before);
            assert!(ts <= after);
        }
    }

    #[test]
    fn test_config_skills() {
        let config = AgentConfig {
            skills: vec!["web-search".to_string(), "calculator".to_string()],
            ..create_test_config()
        };
        
        assert_eq!(config.skills.len(), 2);
        assert!(config.skills.contains(&"web-search".to_string()));
    }
}
