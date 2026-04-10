//! Viber Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct ViberChannel {
    auth_token: String,
    connected: bool,
}

impl ViberChannel {
    pub fn new(auth_token: String) -> Self {
        Self { auth_token, connected: false }
    }
}

#[async_trait]
impl Channel for ViberChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Viber }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://chatapi.viber.com/pa/send_message";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "receiver": message.chat_id,
                "type": "text",
                "text": text
            }),
            MessageContent::Image { url, caption } => serde_json::json!({
                "receiver": message.chat_id,
                "type": "picture",
                "media": url,
                "text": caption
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("X-Viber-Auth-Token", &self.auth_token)
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
