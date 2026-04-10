//! iMessage / Apple Business Chat Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct IMessageChannel {
    business_id: String,
    api_key: String,
    connected: bool,
}

impl IMessageChannel {
    pub fn new(business_id: String, api_key: String) -> Self {
        Self { business_id, api_key, connected: false }
    }
}

#[async_trait]
impl Channel for IMessageChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::iMessage }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        // Apple Business Chat API
        let url = format!(
            "https://api.business.apple.com/v1/businesses/{}/messages",
            self.business_id
        );

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "type": "text",
                "recipient": message.chat_id,
                "body": text
            }),
            MessageContent::Image { url, caption } => serde_json::json!({
                "type": "image",
                "recipient": message.chat_id,
                "url": url,
                "caption": caption
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
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
