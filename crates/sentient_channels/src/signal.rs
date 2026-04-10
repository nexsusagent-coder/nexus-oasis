//! Signal Channel (via signal-cli REST API)

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct SignalChannel {
    phone_number: String,
    api_url: String,
    connected: bool,
}

impl SignalChannel {
    pub fn new(phone_number: String, api_url: String) -> Self {
        Self { phone_number, api_url, connected: false }
    }
}

#[async_trait]
impl Channel for SignalChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Signal }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = format!("{}/v2/send", self.api_url);

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "number": self.phone_number,
                "recipients": [message.chat_id],
                "message": text
            }),
            MessageContent::Image { url, caption } => serde_json::json!({
                "number": self.phone_number,
                "recipients": [message.chat_id],
                "message": caption,
                "base64_attachment": url
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
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
