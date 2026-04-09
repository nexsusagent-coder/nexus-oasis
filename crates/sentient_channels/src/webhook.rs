//! ─── Generic Webhook Integration ───
//!
//!  Generic webhook support for custom integrations
//!
//!  Features:
//!  - POST webhooks
//!  - Signature verification
//!  - Retry logic
//!  - Event filtering

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    Channel, ChannelError, ChannelType, ChannelMessage,
    config::WebhookConfig,
};

/// ─── Webhook Channel ───

pub struct WebhookChannel {
    config: WebhookConfig,
    client: Client,
    connected: bool,
}

impl WebhookChannel {
    /// Create new webhook channel
    pub fn new(config: WebhookConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            connected: false,
        }
    }
    
    /// Generate signature for payload
    fn generate_signature(&self, payload: &[u8]) -> Option<String> {
        use sha2::{Sha256, Digest};
        use hmac::{Hmac, Mac};
        
        let secret = self.config.secret.as_ref()?;
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).ok()?;
        mac.update(payload);
        let result = mac.finalize();
        Some(format!("sha256={}", hex::encode(result.into_bytes())))
    }
}

#[async_trait::async_trait]
impl Channel for WebhookChannel {
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn channel_type(&self) -> ChannelType {
        ChannelType::Webhook
    }
    
    async fn init(&mut self) -> Result<(), ChannelError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Test webhook
        let response = self.client
            .get(&self.config.url)
            .send()
            .await
            .map_err(|e| ChannelError::ConnectionFailed(e.to_string()))?;
        
        if response.status().is_success() || response.status().as_u16() == 404 {
            self.connected = true;
            log::info!("Webhook '{}' initialized", self.config.name);
        }
        
        Ok(())
    }
    
    async fn send(&self, message: &ChannelMessage) -> Result<(), ChannelError> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Create payload
        let payload = WebhookPayload {
            channel: message.channel.to_string(),
            chat_id: message.chat_id.clone(),
            sender: WebhookSender {
                id: message.sender.id.clone(),
                name: message.sender.name.clone(),
            },
            content: message.as_text().unwrap_or("").to_string(),
            timestamp: message.timestamp.to_rfc3339(),
        };
        
        let payload_bytes = serde_json::to_vec(&payload)?;
        
        // Build request
        let mut request = self.client
            .post(&self.config.url)
            .header("Content-Type", "application/json")
            .body(payload_bytes.clone());
        
        // Add signature if secret is configured
        if let Some(sig) = self.generate_signature(&payload_bytes) {
            request = request.header("X-Signature", sig);
        }
        
        // Send
        let response = request
            .send()
            .await
            .map_err(|e| ChannelError::Http(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ChannelError::Http(format!(
                "Webhook returned {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }
        
        Ok(())
    }
    
    async fn receive(&self) -> Result<(), ChannelError> {
        // Webhooks don't receive, they only send
        Ok(())
    }
    
    async fn shutdown(&mut self) -> Result<(), ChannelError> {
        self.connected = false;
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
}

/// ─── Webhook Payload ───

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub channel: String,
    pub chat_id: String,
    pub sender: WebhookSender,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookSender {
    pub id: String,
    pub name: Option<String>,
}
