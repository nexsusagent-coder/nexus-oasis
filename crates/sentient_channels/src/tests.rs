//! ─── SENTIENT Channels Tests ───

#[cfg(test)]
mod tests {
    use sentient_channels::{ChannelType, Message, MessageContent};

    #[test]
    fn test_channel_type_from_str() {
        assert!(matches!(ChannelType::from_str("telegram"), Ok(ChannelType::Telegram)));
        assert!(matches!(ChannelType::from_str("discord"), Ok(ChannelType::Discord)));
        assert!(matches!(ChannelType::from_str("whatsapp"), Ok(ChannelType::WhatsApp)));
        assert!(matches!(ChannelType::from_str("slack"), Ok(ChannelType::Slack)));
        assert!(matches!(ChannelType::from_str("signal"), Ok(ChannelType::Signal)));
        assert!(matches!(ChannelType::from_str("matrix"), Ok(ChannelType::Matrix)));
        assert!(matches!(ChannelType::from_str("irc"), Ok(ChannelType::IRC)));
    }

    #[test]
    fn test_message_creation() {
        let message = Message {
            id: "msg-1".into(),
            channel: ChannelType::Telegram,
            sender: "user-1".into(),
            content: MessageContent::Text("Hello".into()),
            timestamp: chrono::Utc::now(),
            reply_to: None,
            metadata: serde_json::Value::Null,
        };
        
        assert_eq!(message.id, "msg-1");
        assert_eq!(message.channel, ChannelType::Telegram);
        assert_eq!(message.sender, "user-1");
    }

    #[test]
    fn test_message_content() {
        let text = MessageContent::Text("Hello".into());
        assert_eq!(text.as_text(), Some("Hello"));
        
        let image = MessageContent::Image { 
            url: "https://example.com/image.jpg".into(),
            caption: Some("Test image".into()),
        };
        assert!(image.as_text().is_none());
    }
}
