//! Google Chat Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct GoogleChatChannel {
    webhook_url: String,
    connected: bool,
}

impl GoogleChatChannel {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url, connected: false }
    }
}

#[async_trait]
impl Channel for GoogleChatChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::GoogleChat }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "text": text
            }),
            MessageContent::Card { title, description, image, url } => serde_json::json!({
                "cards": [{
                    "sections": [{
                        "widgets": [{
                            "keyValue": {
                                "topLabel": title,
                                "content": description,
                                "contentMultiline": true,
                                "onClick": url.as_ref().map(|u| serde_json::json!({
                                    "openLink": { "url": u }
                                }))
                            }
                        }]
                    }]
                }]
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&self.webhook_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            Ok("sent".to_string())
        } else {
            Err(ChannelError::ApiError(response.status().to_string()))
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> { Ok(Vec::new()) }
    fn is_connected(&self) -> bool { self.connected }
}
