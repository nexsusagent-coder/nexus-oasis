//! Facebook Messenger Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct MessengerChannel {
    page_id: String,
    access_token: String,
    connected: bool,
}

impl MessengerChannel {
    pub fn new(page_id: String, access_token: String) -> Self {
        Self { page_id, access_token, connected: false }
    }
}

#[async_trait]
impl Channel for MessengerChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Messenger }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://graph.facebook.com/v18.0/me/messages";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "recipient": { "id": message.chat_id },
                "message": { "text": text }
            }),
            MessageContent::Image { url, .. } => serde_json::json!({
                "recipient": { "id": message.chat_id },
                "message": {
                    "attachment": {
                        "type": "image",
                        "payload": { "url": url }
                    }
                }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .query(&[("access_token", &self.access_token)])
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
            Ok(json["message_id"].as_str().unwrap_or("sent").to_string())
        } else {
            Err(ChannelError::ApiError(response.status().to_string()))
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> { Ok(Vec::new()) }
    fn is_connected(&self) -> bool { self.connected }
}
