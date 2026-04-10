//! WhatsApp Business API Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct WhatsAppChannel {
    phone_number_id: String,
    access_token: String,
    connected: bool,
}

impl WhatsAppChannel {
    pub fn new(phone_number_id: String, access_token: String) -> Self {
        Self {
            phone_number_id,
            access_token,
            connected: false,
        }
    }
}

#[async_trait]
impl Channel for WhatsAppChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::WhatsApp
    }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = format!(
            "https://graph.facebook.com/v18.0/{}/messages",
            self.phone_number_id
        );

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "messaging_product": "whatsapp",
                "to": message.chat_id,
                "type": "text",
                "text": { "body": text }
            }),
            MessageContent::Image { url, caption } => serde_json::json!({
                "messaging_product": "whatsapp",
                "to": message.chat_id,
                "type": "image",
                "image": {
                    "link": url,
                    "caption": caption
                }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
            Ok(json["messages"][0]["id"].as_str().unwrap_or("sent").to_string())
        } else {
            Err(ChannelError::ApiError(response.status().to_string()))
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        Ok(Vec::new())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}
