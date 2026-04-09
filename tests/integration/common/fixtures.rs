//! Test fixtures

use sentient_core::{AgentConfig, Message, MessageRole};

/// Sample agent configurations for testing
pub fn agent_configs() -> Vec<AgentConfig> {
    vec![
        AgentConfig {
            name: "BasicAgent".to_string(),
            ..Default::default()
        },
        AgentConfig {
            name: "CreativeAgent".to_string(),
            temperature: 0.9,
            max_tokens: 4000,
            ..Default::default()
        },
        AgentConfig {
            name: "PreciseAgent".to_string(),
            temperature: 0.1,
            max_tokens: 500,
            ..Default::default()
        },
    ]
}

/// Sample messages for testing
pub fn sample_messages() -> Vec<Message> {
    vec![
        Message::system("You are a helpful assistant."),
        Message::user("Hello, how are you?"),
        Message::assistant("I'm doing well, thank you for asking!"),
        Message::user("Can you help me with something?"),
        Message::assistant("Of course! What do you need help with?"),
    ]
}

/// Sample conversations for testing
pub fn sample_conversations() -> Vec<Vec<Message>> {
    vec![
        vec![
            Message::user("What is 2 + 2?"),
            Message::assistant("2 + 2 equals 4."),
        ],
        vec![
            Message::user("Tell me a joke."),
            Message::assistant("Why don't scientists trust atoms? Because they make up everything!"),
        ],
        vec![
            Message::user("Hello"),
            Message::assistant("Hi there! How can I help you today?"),
            Message::user("I need help with Rust"),
            Message::assistant("I'd be happy to help with Rust! What specifically do you need?"),
        ],
    ]
}

/// Test payloads for events
pub fn test_payloads() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({ "type": "greeting", "content": "Hello" }),
        serde_json::json!({ "type": "query", "question": "What is AI?" }),
        serde_json::json!({ "type": "command", "action": "search", "params": ["rust", "async"] }),
        serde_json::json!({
            "user": {
                "id": "user123",
                "name": "Test User",
                "preferences": {
                    "language": "en",
                    "theme": "dark"
                }
            }
        }),
    ]
}
