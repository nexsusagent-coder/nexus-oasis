//! Mattermost Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct MattermostChannel {
    server_url: String,
    access_token: String,
    connected: bool,
}

impl MattermostChannel {
    pub fn new(server_url: String, access_token: String) -> Self {
        Self { server_url, access_token, connected: false }
    }
}

#[async_trait]
impl Channel for MattermostChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Mattermost }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = format!("{}/api/v4/posts", self.server_url);

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "channel_id": message.chat_id,
                "message": text
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "channel_id": message.chat_id,
                "message": text
            }),
            MessageContent::File { name, url } => serde_json::json!({
                "channel_id": message.chat_id,
                "message": name,
                "file_ids": [url]
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
            Ok(json["id"].as_str().unwrap_or("sent").to_string())
        } else {
            Err(ChannelError::ApiError(response.status().to_string()))
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> { Ok(Vec::new()) }
    fn is_connected(&self) -> bool { self.connected }
}
