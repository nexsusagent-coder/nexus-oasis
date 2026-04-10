//! LINE Messaging Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct LineChannel {
    channel_access_token: String,
    channel_secret: String,
    connected: bool,
}

impl LineChannel {
    pub fn new(channel_access_token: String, channel_secret: String) -> Self {
        Self { channel_access_token, channel_secret, connected: false }
    }
}

#[async_trait]
impl Channel for LineChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Line }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://api.line.me/v2/bot/message/push";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "to": message.chat_id,
                "messages": [{ "type": "text", "text": text }]
            }),
            MessageContent::Image { url, .. } => serde_json::json!({
                "to": message.chat_id,
                "messages": [{ "type": "image", "originalContentUrl": url, "previewImageUrl": url }]
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.channel_access_token))
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
