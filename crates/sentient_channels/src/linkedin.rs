//! LinkedIn Messaging Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct LinkedInChannel {
    access_token: String,
    connected: bool,
}

impl LinkedInChannel {
    pub fn new(access_token: String) -> Self {
        Self { access_token, connected: false }
    }
}

#[async_trait]
impl Channel for LinkedInChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::LinkedIn }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://api.linkedin.com/v2/messages";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "recipients": { "values": [{ "person": { "id": message.chat_id } }] },
                "subject": "Message",
                "body": text
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
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
