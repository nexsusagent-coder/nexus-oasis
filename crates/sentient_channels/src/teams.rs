//! Microsoft Teams Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct TeamsChannel {
    tenant_id: String,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    connected: bool,
}

impl TeamsChannel {
    pub fn new(tenant_id: String, client_id: String, client_secret: String) -> Self {
        Self { tenant_id, client_id, client_secret, access_token: None, connected: false }
    }
}

#[async_trait]
impl Channel for TeamsChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Teams }

    async fn init(&mut self) -> Result<(), ChannelError> {
        // Get access token from Microsoft Graph
        let url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            self.tenant_id
        );

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("scope", "https://graph.microsoft.com/.default"),
                ("grant_type", "client_credentials"),
            ])
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

        let url = format!(
            "https://graph.microsoft.com/v1.0/chats/{}/messages",
            message.chat_id
        );

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "body": { "content": text, "contentType": "text" }
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "body": { "content": text, "contentType": "html" }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
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
