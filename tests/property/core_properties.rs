//! Property-based tests using proptest

use proptest::prelude::*;
use sentient_core::{Message, MessageRole, AgentConfig};

proptest! {
    /// Any string should be valid message content
    #[test]
    fn test_message_content_arbitrary(content in ".*") {
        let msg = Message::user(&content);
        prop_assert_eq!(msg.content, content);
        prop_assert_eq!(msg.role, MessageRole::User);
    }

    /// Message serialization should be idempotent
    #[test]
    fn test_message_serialization_roundtrip(
        role in prop_oneof![Just(MessageRole::User), Just(MessageRole::Assistant), Just(MessageRole::System)],
        content in ".*"
    ) {
        let msg = Message { role, content, metadata: None, timestamp: None };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: Message = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(msg.role, decoded.role);
        prop_assert_eq!(msg.content, decoded.content);
    }

    /// Temperature should be within valid bounds
    #[test]
    fn test_temperature_bounds(temp in 0.0f64..=2.0) {
        let config = AgentConfig {
            temperature: temp,
            ..Default::default()
        };
        prop_assert!(config.validate().is_ok());
    }

    /// Max tokens should accept valid values
    #[test]
    fn test_max_tokens_valid(tokens in 1usize..=128000) {
        let config = AgentConfig {
            max_tokens: tokens,
            ..Default::default()
        };
        prop_assert!(config.validate().is_ok());
    }

    /// Message array serialization
    #[test]
    fn test_message_array_roundtrip(messages in prop::collection::vec(
        prop_oneof![Just(MessageRole::User), Just(MessageRole::Assistant)],
        0..10
    )) {
        let msgs: Vec<Message> = messages.into_iter()
            .enumerate()
            .map(|(i, role)| Message { role, content: format!("Message {}", i), metadata: None, timestamp: None })
            .collect();
        
        let json = serde_json::to_string(&msgs).unwrap();
        let decoded: Vec<Message> = serde_json::from_str(&json).unwrap();
        
        prop_assert_eq!(msgs.len(), decoded.len());
    }

    /// JSON metadata should be serializable
    #[test]
    fn test_metadata_serialization(
        key in "[a-z]+",
        value in prop_oneof![
            "[a-zA-Z0-9]+".prop_map(|s| serde_json::json!(s)),
            (0i64..100).prop_map(|n| serde_json::json!(n)),
            proptest::bool::ANY.prop_map(|b| serde_json::json!(b))
        ]
    ) {
        let metadata = serde_json::json!({ key.clone(): value.clone() });
        let msg = Message {
            role: MessageRole::User,
            content: "test".to_string(),
            metadata: Some(metadata.clone()),
            timestamp: None,
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: Message = serde_json::from_str(&json).unwrap();
        
        prop_assert_eq!(decoded.metadata.unwrap()[key], value);
    }

    /// Unicode content should be handled correctly
    #[test]
    fn test_unicode_handling(content in "[\u{0020}-\u{1FFFF}]+") {
        let msg = Message::user(&content);
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: Message = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(msg.content, decoded.content);
    }

    /// Very long content should be serializable
    #[test]
    fn test_long_content(length in 1usize..50000) {
        let content = "x".repeat(length);
        let msg = Message::user(&content);
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: Message = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(msg.content.len(), decoded.content.len());
    }
}

/// Additional property tests for edge cases
mod edge_case_tests {
    use super::*;

    proptest! {
        #[test]
        fn test_empty_string_message(content in "") {
            let msg = Message::user(&content);
            prop_assert!(msg.content.is_empty());
        }

        #[test]
        fn test_whitespace_only(content in r"\s*") {
            let msg = Message::user(&content);
            let json = serde_json::to_string(&msg).unwrap();
            let decoded: Message = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(msg.content, decoded.content);
        }

        #[test]
        fn test_special_characters(content in r"[\\\"\n\t\r]+") {
            let msg = Message::user(&content);
            let json = serde_json::to_string(&msg).unwrap();
            let decoded: Message = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(msg.content, decoded.content);
        }
    }
}
