//! Amazon Chime Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct ChimeChannel {
    webhook_url: String,
    connected: bool,
}

impl ChimeChannel {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url, connected: false }
    }
}

#[async_trait]
impl Channel for ChimeChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Chime }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "Content": text
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "Content": text,
                "ContentType": "markdown"
            }),
            MessageContent::Card { title, description, .. } => serde_json::json!({
                "Content": format!("**{}**\n{}", title, description)
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
