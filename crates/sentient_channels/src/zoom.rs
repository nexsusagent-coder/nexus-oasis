//! Zoom Chat Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct ZoomChannel {
    account_id: String,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    connected: bool,
}

impl ZoomChannel {
    pub fn new(account_id: String, client_id: String, client_secret: String) -> Self {
        Self { account_id, client_id, client_secret, access_token: None, connected: false }
    }
}

#[async_trait]
impl Channel for ZoomChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Zoom }

    async fn init(&mut self) -> Result<(), ChannelError> {
        let url = "https://zoom.us/oauth/token";

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .query(&[
                ("account_id", self.account_id.as_str()),
                ("grant_type", "account_credentials"),
            ])
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
            self.access_token = json["access_token"].as_str().map(|s| s.to_string());
            self.connected = true;
            Ok(())
        } else {
            Err(ChannelError::AuthFailed("Failed to get access token".into()))
        }
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| ChannelError::AuthFailed("No access token".into()))?;

        let url = "https://api.zoom.us/v2/chat/users/me/messages";

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "message": text,
                "to_channel": message.chat_id
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "message": text,
                "to_channel": message.chat_id
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
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
