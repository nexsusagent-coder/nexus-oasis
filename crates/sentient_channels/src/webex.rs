//! Cisco Webex Teams Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct WebexChannel {
    access_token: String,
    connected: bool,
}

impl WebexChannel {
    pub fn new(access_token: String) -> Self {
        Self { access_token, connected: false }
    }
}

#[async_trait]
impl Channel for WebexChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Webex }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = "https://api.ciscospark.com/v1/messages";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "roomId": message.chat_id,
                "text": text
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "roomId": message.chat_id,
                "markdown": text
            }),
            MessageContent::Image { url, .. } => serde_json::json!({
                "roomId": message.chat_id,
                "files": [url]
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
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
