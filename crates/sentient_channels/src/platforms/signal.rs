//! ─── Signal Integration ───
//!
//! Supports:
//! - signal-cli REST API
//! - signal-cli D-Bus interface
//! - Direct libsignal (experimental)

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{Channel, ChannelError, ChannelMessage, MessageContent, ChannelType};

/// Signal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    /// Signal phone number (with country code)
    pub phone_number: String,
    
    /// signal-cli REST API URL
    pub api_url: String,
    
    /// Attachment storage path
    pub attachment_path: Option<String>,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            phone_number: String::new(),
            api_url: "http://localhost:8080".into(),
            attachment_path: None,
        }
    }
}

/// Signal channel via signal-cli REST API
pub struct SignalChannel {
    config: SignalConfig,
    client: Client,
}

impl SignalChannel {
    pub fn new(config: SignalConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
    
    /// Send text message
    pub async fn send_text(&self, to: &str, message: &str) -> Result<String, ChannelError> {
        let url = format!("{}/v2/send", self.config.api_url);
        
        let body = serde_json::json!({
            "number": self.config.phone_number,
            "recipients": [to],
            "message": message
        });
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(ChannelError::ApiError(error));
        }
        
        let result: SignalSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.timestamp.to_string())
    }
    
    /// Send message with attachments
    pub async fn send_attachment(&self, to: &str, message: &str, attachments: &[&str]) -> Result<String, ChannelError> {
        let url = format!("{}/v2/send", self.config.api_url);
        
        let body = serde_json::json!({
            "number": self.config.phone_number,
            "recipients": [to],
            "message": message,
            "base64_attachments": attachments
        });
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SignalSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.timestamp.to_string())
    }
    
    /// Send reaction
    pub async fn send_reaction(&self, to: &str, message_timestamp: u64, emoji: &str) -> Result<(), ChannelError> {
        let url = format!("{}/v2/reactions", self.config.api_url);
        
        let body = serde_json::json!({
            "number": self.config.phone_number,
            "recipients": [to],
            "reaction": emoji,
            "target_author": to,
            "timestamp": message_timestamp
        });
        
        self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Get messages
    pub async fn get_messages(&self) -> Result<Vec<SignalMessage>, ChannelError> {
        let url = format!("{}/v1/messages/{}", self.config.api_url, self.config.phone_number);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let messages: Vec<SignalMessage> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(messages)
    }
    
    /// Register phone number
    pub async fn register(&self, captcha: Option<&str>, voice: bool) -> Result<(), ChannelError> {
        let url = format!("{}/v1/register/{}", self.config.api_url, self.config.phone_number);
        
        let mut body = serde_json::json!({
            "voice": voice
        });
        
        if let Some(c) = captcha {
            body["captcha"] = serde_json::json!(c);
        }
        
        self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Verify registration
    pub async fn verify(&self, code: &str) -> Result<(), ChannelError> {
        let url = format!("{}/v1/register/{}/verify/{}", 
            self.config.api_url, self.config.phone_number, code);
        
        self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Check if number is registered on Signal
    pub async fn is_registered(&self, number: &str) -> Result<bool, ChannelError> {
        let url = format!("{}/v1/registered/{}", self.config.api_url, number);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(response.status().is_success())
    }
    
    /// Create group
    pub async fn create_group(&self, name: &str, members: &[&str]) -> Result<String, ChannelError> {
        let url = format!("{}/v1/groups/{}", self.config.api_url, self.config.phone_number);
        
        let body = serde_json::json!({
            "name": name,
            "members": members
        });
        
        let response = self.client
            .put(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: GroupResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.id)
    }
}

#[async_trait]
impl Channel for SignalChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Signal
    }
    
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        match message.content {
            MessageContent::Text(text) => self.send_text(&message.recipient, &text).await,
            MessageContent::Image { url, caption } => {
                let msg = caption.unwrap_or_default();
                self.send_attachment(&message.recipient, &msg, &[&url]).await
            }
            _ => Err(ChannelError::UnsupportedContentType),
        }
    }
    
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        let messages = self.get_messages().await?;
        
        Ok(messages.into_iter().map(|m| ChannelMessage {
            id: m.timestamp.to_string(),
            channel: ChannelType::Signal,
            sender: m.source,
            recipient: self.config.phone_number.clone(),
            content: MessageContent::Text(m.message),
            timestamp: chrono::DateTime::from_timestamp(m.timestamp as i64, 0)
                .unwrap_or_else(|| chrono::Utc::now()),
            metadata: None,
        }).collect())
    }
    
    fn is_connected(&self) -> bool {
        true // REST API is always available
    }
}

/// Signal send result
#[derive(Debug, Deserialize)]
struct SignalSendResult {
    timestamp: u64,
}

/// Signal message
#[derive(Debug, Deserialize)]
pub struct SignalMessage {
    pub timestamp: u64,
    pub source: String,
    pub message: String,
    #[serde(default)]
    pub attachments: Vec<String>,
    #[serde(default)]
    pub group: Option<String>,
}

/// Group creation result
#[derive(Debug, Deserialize)]
struct GroupResult {
    id: String,
}
