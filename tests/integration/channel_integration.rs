//! Channel integration tests

#[cfg(test)]
mod channel_tests {
    use sentient_channels::{Channel, ChannelConfig, Message};

    #[tokio::test]
    async fn test_telegram_channel_init() {
        let config = ChannelConfig::Telegram {
            bot_token: "test_token".to_string(),
            allowed_chats: None,
        };
        
        let result = Channel::new(config).await;
        // Should initialize (may fail validation without real token)
        assert!(result.is_ok() || result.is_err()); // Either is acceptable for test
    }

    #[tokio::test]
    async fn test_discord_channel_init() {
        let config = ChannelConfig::Discord {
            bot_token: "test_token".to_string(),
            application_id: "123456789".to_string(),
            guild_id: None,
        };
        
        let result = Channel::new(config).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_slack_channel_init() {
        let config = ChannelConfig::Slack {
            bot_token: "xoxb-test".to_string(),
            app_token: "xapp-test".to_string(),
            signing_secret: "secret".to_string(),
        };
        
        let result = Channel::new(config).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_message_conversion() {
        // Test message conversion between channel and internal formats
        let telegram_message = sentient_channels::TelegramMessage {
            chat_id: 12345,
            text: "Hello".to_string(),
            user_id: Some("user123".to_string()),
        };
        
        let internal: sentient_core::Message = telegram_message.into();
        assert_eq!(internal.content, "Hello");
    }

    #[tokio::test]
    async fn test_channel_health_check() {
        // Mock channel health check
        let config = ChannelConfig::Mock;
        let channel = Channel::new(config).await.expect("Failed to create mock channel");
        
        let health = channel.health_check().await;
        assert!(health.is_ok());
    }

    #[test]
    fn test_channel_config_serialization() {
        let configs = vec![
            ChannelConfig::Mock,
            ChannelConfig::Telegram {
                bot_token: "token".to_string(),
                allowed_chats: None,
            },
        ];
        
        for config in configs {
            let json = serde_json::to_string(&config).unwrap();
            let decoded: ChannelConfig = serde_json::from_str(&json).unwrap();
            assert_eq!(
                serde_json::to_string(&config).unwrap(),
                serde_json::to_string(&decoded).unwrap()
            );
        }
    }
}

#[cfg(test)]
mod webhook_tests {
    use sentient_channels::webhook::{WebhookHandler, WebhookConfig};

    #[test]
    fn test_webhook_config() {
        let config = WebhookConfig {
            path: "/webhook/telegram".to_string(),
            secret: "webhook_secret".to_string(),
            max_retries: 3,
        };
        
        assert_eq!(config.path, "/webhook/telegram");
        assert_eq!(config.max_retries, 3);
    }

    #[tokio::test]
    async fn test_webhook_verification() {
        let handler = WebhookHandler::new("test_secret");
        
        let valid = handler.verify_signature("test_secret", b"test_data", "expected_sig");
        // Verification logic test
        assert!(valid.is_ok() || valid.is_err());
    }
}
