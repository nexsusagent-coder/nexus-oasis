//! Instagram DM Channel (via Instagram Graph API)

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct InstagramChannel {
    account_id: String,
    access_token: String,
    connected: bool,
}

impl InstagramChannel {
    pub fn new(account_id: String, access_token: String) -> Self {
        Self { account_id, access_token, connected: false }
    }
}

#[async_trait]
impl Channel for InstagramChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::Instagram }

    async fn init(&mut self) -> Result<(), ChannelError> {
        self.connected = true;
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let url = format!(
            "https://graph.facebook.com/v18.0/{}/messages",
            self.account_id
        );

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "recipient": { "id": message.chat_id },
                "message": { "text": text }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .query(&[("access_token", &self.access_token)])
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
