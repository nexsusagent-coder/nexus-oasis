//! Twitter/X DM Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct TwitterChannel {
    bearer_token: String,
    api_key: String,
    api_secret: String,
    connected: bool,
}

impl TwitterChannel {
    pub fn new(bearer_token: String, api_key: String, api_secret: String) -> Self {
        Self { bearer_token, api_key, api_secret, connected: false }
    }
}

#[async_trait]
impl Channel for TwitterChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Twitter }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://api.twitter.com/2/dm_events";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "event": {
                    "type": "message_create",
                    "message_create": {
                        "target": { "recipient_id": message.chat_id },
                        "message_data": { "text": text }
                    }
                }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bearer_token))
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
