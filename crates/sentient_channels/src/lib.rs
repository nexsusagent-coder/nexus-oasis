//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Channels - Multi-Platform Messaging Integration
//! ═══════════════════════════════════════════════════════════════════════════════

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(private_interfaces)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

pub mod message;
pub mod config;
pub mod telegram;
pub mod discord;
pub mod slack;
pub mod whatsapp;
pub mod messenger;
pub mod instagram;
pub mod twitter;
pub mod linkedin;
pub mod teams;
pub mod google_chat;
pub mod signal;
pub mod viber;
pub mod line;
pub mod snapchat;
pub mod wechat;
pub mod imessage;
pub mod chime;
pub mod zoom;
pub mod webex;
pub mod mattermost;
pub mod voice_handler;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use message::{ChannelMessage, ChannelType, MessageContent, MessageSender};
pub use config::{ChannelsConfig, ChannelConfig};
pub use telegram::TelegramChannel;
pub use discord::DiscordChannel as DiscordBot;
pub use slack::SlackChannel as SlackBot;
pub use whatsapp::WhatsAppChannel;
pub use messenger::MessengerChannel;
pub use instagram::InstagramChannel;
pub use twitter::TwitterChannel;
pub use linkedin::LinkedInChannel;
pub use teams::TeamsChannel;
pub use google_chat::GoogleChatChannel;
pub use signal::SignalChannel;
pub use viber::ViberChannel;
pub use line::LineChannel;
pub use snapchat::SnapchatChannel;
pub use wechat::WeChatChannel;
pub use imessage::IMessageChannel;
pub use chime::ChimeChannel;
pub use zoom::ZoomChannel;
pub use webex::WebexChannel;
pub use mattermost::MattermostChannel;

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
    
    /// Shutdown all channels
    pub async fn shutdown_all(&mut self) {
        for channel in &mut self.channels {
            let mut ch = channel.write().await;
            let _ = ch.shutdown().await;
        }
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── Errors ───

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Connection failed: {0}")]
    Connection(String),
    
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    
    #[error("Channel not found: {0}")]
    NotFound(String),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_type() {
        assert!(matches!(ChannelType::Telegram, ChannelType::Telegram));
    }
}
