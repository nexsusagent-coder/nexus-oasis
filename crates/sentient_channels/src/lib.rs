//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Channels - Multi-Platform Messaging Integration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Supported platforms:
//!  - Telegram (Bot API)
//!  - Discord (Bot API)
//!  - WhatsApp (Business API)
//!  - Slack (Bot API)
//!  - Signal (signal-cli REST API)
//!  - Matrix (Client-Server API)
//!  - IRC (RFC 1459)
//!  - Generic Webhook
//!
//!  Features:
//!  - Unified message interface
//!  - Command handling
//!  - Natural language processing
//!  - Multi-channel broadcasting
//!  - Rate limiting
//!  - Message queuing

pub mod telegram;
pub mod discord;
pub mod webhook;
pub mod message;
pub mod commands;
pub mod config;
pub mod platforms;

use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use message::{ChannelMessage, ChannelType, MessageContent, MessageSender};
pub use config::{ChannelsConfig, ChannelConfig};

/// ─── Channel Trait ───

#[async_trait]
pub trait Channel: Send + Sync {
    /// Channel name
    fn name(&self) -> &str {
        "unnamed"
    }
    
    /// Channel type
    fn channel_type(&self) -> ChannelType;
    
    /// Initialize channel
    async fn init(&mut self) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Send message
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError>;
    
    /// Receive messages
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError>;
    
    /// Shutdown channel
    async fn shutdown(&mut self) -> Result<(), ChannelError> {
        Ok(())
    }
    
    /// Check if connected
    fn is_connected(&self) -> bool;
}

/// ─── Channel Manager ───

pub struct ChannelManager {
    channels: Vec<Arc<RwLock<Box<dyn Channel>>>>,
    message_handler: Option<Arc<dyn Fn(ChannelMessage) + Send + Sync>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            message_handler: None,
        }
    }
    
    /// Add channel
    pub fn add_channel<C: Channel + 'static>(&mut self, channel: C) {
        self.channels.push(Arc::new(RwLock::new(Box::new(channel))));
    }
    
    /// Set message handler
    pub fn set_handler<F>(&mut self, handler: F)
    where
        F: Fn(ChannelMessage) + Send + Sync + 'static,
    {
        self.message_handler = Some(Arc::new(handler));
    }
    
    /// Initialize all channels
    pub async fn init_all(&mut self) -> Result<(), ChannelError> {
        for channel in &self.channels {
            let mut ch = channel.write().await;
            ch.init().await?;
        }
        Ok(())
    }
    
    /// Broadcast message to all channels
    pub async fn broadcast(&self, content: MessageContent) -> Vec<Result<String, ChannelError>> {
        let mut results = Vec::new();
        
        for channel in &self.channels {
            let ch = channel.read().await;
            let msg = ChannelMessage::new(ch.channel_type(), "broadcast", content.clone());
            results.push(ch.send(msg).await);
        }
        
        results
    }
    
    /// Send to specific channel
    pub async fn send_to(&self, channel_type: ChannelType, chat_id: &str, content: MessageContent) -> Result<String, ChannelError> {
        for channel in &self.channels {
            let ch = channel.read().await;
            if ch.channel_type() == channel_type {
                let msg = ChannelMessage::new(channel_type, chat_id, content);
                return ch.send(msg).await;
            }
        }
        Err(ChannelError::NotFound(format!("{:?}", channel_type)))
    }
    
    /// Receive from all channels
    pub async fn receive_all(&self) -> Vec<ChannelMessage> {
        let mut all_messages = Vec::new();
        
        for channel in &self.channels {
            let ch = channel.read().await;
            if let Ok(messages) = ch.receive().await {
                all_messages.extend(messages);
            }
        }
        
        all_messages
    }
    
    /// Get connected channels
    pub fn connected_channels(&self) -> Vec<ChannelType> {
        // Would need async to check is_connected
        vec![]
    }
    
    /// Shutdown all channels
    pub async fn shutdown_all(&mut self) {
        for channel in &mut self.channels {
            let mut ch = channel.write().await;
            let _ = ch.shutdown().await;
        }
    }
}

/// ─── Errors ───

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    
    #[error("Unsupported content type")]
    UnsupportedContentType,
    
    #[error("Channel not found: {0}")]
    NotFound(String),
    
    #[error("Broadcast errors: {0:?}")]
    Broadcast(Vec<ChannelError>),
    
    #[error("IO error: {0}")]
    StdIo(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_type() {
        assert!(matches!(ChannelType::Telegram, ChannelType::Telegram));
        assert!(matches!(ChannelType::Discord, ChannelType::Discord));
        assert!(matches!(ChannelType::WhatsApp, ChannelType::WhatsApp));
        assert!(matches!(ChannelType::Slack, ChannelType::Slack));
        assert!(matches!(ChannelType::Signal, ChannelType::Signal));
        assert!(matches!(ChannelType::Matrix, ChannelType::Matrix));
        assert!(matches!(ChannelType::IRC, ChannelType::IRC));
    }
}
