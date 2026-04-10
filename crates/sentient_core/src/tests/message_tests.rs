//! Message unit tests

#[cfg(test)]
mod message_unit_tests {
    use sentient_core::{Message, MessageRole};

    #[test]
    fn test_message_roles() {
        let roles = vec![
            (MessageRole::User, "user"),
            (MessageRole::Assistant, "assistant"),
            (MessageRole::System, "system"),
            (MessageRole::Tool, "tool"),
        ];
        
        for (role, expected) in roles {
            assert_eq!(role.to_string(), expected);
        }
    }

    #[test]
    fn test_message_eq() {
        let msg1 = Message::user("Hello");
        let msg2 = Message::user("Hello");
        let msg3 = Message::user("World");
        
        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_message_clone() {
        let msg = Message::user("Original");
        let cloned = msg.clone();
        
        assert_eq!(msg, cloned);
    }

    #[test]
    fn test_message_debug() {
        let msg = Message::user("Test");
        let debug = format!("{:?}", msg);
        
        assert!(debug.contains("user"));
        assert!(debug.contains("Test"));
    }

    #[test]
    fn test_message_empty_content() {
        let msg = Message::user("");
        assert!(msg.content.is_empty());
    }

    #[test]
    fn test_message_long_content() {
        let long_content = "x".repeat(10000);
        let msg = Message::user(&long_content);
        
        assert_eq!(msg.content.len(), 10000);
    }

    #[test]
    fn test_message_unicode() {
        let unicode_content = "你好世界 🌍 مرحبا";
        let msg = Message::user(unicode_content);
        
        assert_eq!(msg.content, unicode_content);
    }

    #[test]
    fn test_message_json_roundtrip() {
        let original = Message {
            role: MessageRole::User,
            content: "Test message with special chars: \n\t\"quotes\"".to_string(),
            metadata: Some(serde_json::json!({"key": "value"})),
            timestamp: Some(chrono::Utc::now()),
        };
        
        let json = serde_json::to_string(&original).expect("operation failed");
        let decoded: Message = serde_json::from_str(&json).expect("operation failed");
        
        assert_eq!(original.role, decoded.role);
        assert_eq!(original.content, decoded.content);
    }

    #[test]
    fn test_message_array_serialization() {
        let messages = vec![
            Message::system("You are helpful"),
            Message::user("Hi"),
            Message::assistant("Hello!"),
        ];
        
        let json = serde_json::to_string(&messages).expect("operation failed");
        let decoded: Vec<Message> = serde_json::from_str(&json).expect("operation failed");
        
        assert_eq!(messages.len(), decoded.len());
    }

    #[test]
    fn test_message_from_str() {
        let msg: Message = "Hello".into();
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_message_builder() {
        let msg = Message::builder()
            .role(MessageRole::User)
            .content("Hello")
            .metadata(serde_json::json!({"source": "test"}))
            .build();
        
        assert_eq!(msg.role, MessageRole::User);
        assert!(msg.metadata.is_some());
    }
}
