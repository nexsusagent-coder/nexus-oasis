//! Event unit tests

#[cfg(test)]
mod event_unit_tests {
    use sentient_common::events::{SENTIENTEvent, EventType};

    #[test]
    fn test_event_creation() {
        let event = SENTIENTEvent::new(
            EventType::MessageReceived,
            "test_source",
            serde_json::json!({"data": "test"}),
        );
        
        assert_eq!(event.event_type, EventType::MessageReceived);
        assert_eq!(event.source, "test_source");
    }

    #[test]
    fn test_event_types() {
        let types = vec![
            EventType::SystemStart,
            EventType::SystemShutdown,
            EventType::MessageReceived,
            EventType::MessageSent,
            EventType::Error,
            EventType::Warning,
        ];
        
        for event_type in types {
            let event = SENTIENTEvent::new(event_type.clone(), "test", serde_json::json!({}));
            assert_eq!(event.event_type, event_type);
        }
    }

    #[test]
    fn test_event_timestamp() {
        let before = chrono::Utc::now();
        let event = SENTIENTEvent::new(EventType::MessageReceived, "test", serde_json::json!({}));
        let after = chrono::Utc::now();
        
        assert!(event.timestamp >= before);
        assert!(event.timestamp <= after);
    }

    #[test]
    fn test_event_serialization() {
        let event = SENTIENTEvent::new(
            EventType::MessageReceived,
            "telegram",
            serde_json::json!({"chat_id": 12345}),
        );
        
        let json = serde_json::to_string(&event).expect("operation failed");
        let decoded: SENTIENTEvent = serde_json::from_str(&json).expect("operation failed");
        
        assert_eq!(event.id, decoded.id);
        assert_eq!(event.event_type, decoded.event_type);
    }

    #[test]
    fn test_event_payload() {
        let payload = serde_json::json!({
            "message": "Hello",
            "user_id": "user123",
            "metadata": {
                "platform": "telegram",
                "language": "en"
            }
        });
        
        let event = SENTIENTEvent::new(EventType::MessageReceived, "test", payload.clone());
        
        assert_eq!(event.payload["message"], "Hello");
        assert_eq!(event.payload["metadata"]["platform"], "telegram");
    }

    #[test]
    fn test_event_duration() {
        let mut event = SENTIENTEvent::new(EventType::MessageReceived, "test", serde_json::json!({}));
        
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        event.mark_complete();
        
        let duration = event.duration().expect("Should have duration");
        assert!(duration >= std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_event_chain() {
        let event1 = SENTIENTEvent::new(EventType::MessageReceived, "telegram", serde_json::json!({}));
        let event2 = SENTIENTEvent::new_with_parent(
            EventType::MessageSent,
            "core",
            serde_json::json!({}),
            event1.id,
        );
        
        assert!(event2.parent_id.is_some());
        assert_eq!(event2.parent_id.expect("operation failed"), event1.id);
    }

    #[test]
    fn test_event_tags() {
        let mut event = SENTIENTEvent::new(EventType::MessageReceived, "test", serde_json::json!({}));
        
        event.add_tag("priority", "high");
        event.add_tag("source", "api");
        
        assert_eq!(event.tags.get("priority"), Some(&"high".to_string()));
    }
}
