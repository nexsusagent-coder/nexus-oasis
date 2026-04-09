//! ─── Slack Integration ───
//!
//! Supports:
//! - Slack Bolt SDK (via webhook)
//! - Slack Web API
//! - Socket Mode (real-time)
//! - Slash commands
//! - Interactive components

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{Channel, ChannelError, ChannelMessage, MessageContent, ChannelType};

/// Slack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    /// Bot User OAuth Token (xoxb-...)
    pub bot_token: String,
    
    /// App-Level Token (xapp-...)
    pub app_token: Option<String>,
    
    /// Signing Secret
    pub signing_secret: String,
    
    /// Default channel for notifications
    pub default_channel: Option<String>,
}

/// Slack channel
pub struct SlackChannel {
    config: SlackConfig,
    client: Client,
    base_url: String,
}

impl SlackChannel {
    pub fn new(config: SlackConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            base_url: "https://slack.com/api".into(),
        }
    }
    
    /// Post message to channel
    pub async fn post_message(&self, channel: &str, text: &str) -> Result<String, ChannelError> {
        let url = format!("{}/chat.postMessage", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "text": text
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if !result.ok {
            return Err(ChannelError::ApiError(result.error.unwrap_or_default()));
        }
        
        Ok(result.ts.unwrap_or_default())
    }
    
    /// Post message with blocks
    pub async fn post_blocks(&self, channel: &str, text: &str, blocks: Vec<SlackBlock>) -> Result<String, ChannelError> {
        let url = format!("{}/chat.postMessage", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "text": text,
            "blocks": blocks
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.ts.unwrap_or_default())
    }
    
    /// Post ephemeral message (only visible to user)
    pub async fn post_ephemeral(&self, channel: &str, user: &str, text: &str) -> Result<String, ChannelError> {
        let url = format!("{}/chat.postEphemeral", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "user": user,
            "text": text
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.message_ts.unwrap_or_default())
    }
    
    /// Update message
    pub async fn update_message(&self, channel: &str, ts: &str, text: &str) -> Result<(), ChannelError> {
        let url = format!("{}/chat.update", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "ts": ts,
            "text": text
        });
        
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Delete message
    pub async fn delete_message(&self, channel: &str, ts: &str) -> Result<(), ChannelError> {
        let url = format!("{}/chat.delete", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "ts": ts
        });
        
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Add reaction
    pub async fn add_reaction(&self, channel: &str, ts: &str, emoji: &str) -> Result<(), ChannelError> {
        let url = format!("{}/reactions.add", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel,
            "timestamp": ts,
            "name": emoji
        });
        
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Open a modal
    pub async fn open_modal(&self, trigger_id: &str, view: SlackView) -> Result<String, ChannelError> {
        let url = format!("{}/views.open", self.base_url);
        
        let body = serde_json::json!({
            "trigger_id": trigger_id,
            "view": view
        });
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.view.map(|v| v.id).unwrap_or_default())
    }
    
    /// Get user info
    pub async fn get_user_info(&self, user_id: &str) -> Result<SlackUser, ChannelError> {
        let url = format!("{}/users.info?user={}", self.base_url, user_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackUserResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.user)
    }
    
    /// List channels
    pub async fn list_channels(&self) -> Result<Vec<SlackChannelInfo>, ChannelError> {
        let url = format!("{}/conversations.list", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SlackChannelsResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.channels)
    }
    
    /// Join channel
    pub async fn join_channel(&self, channel: &str) -> Result<(), ChannelError> {
        let url = format!("{}/conversations.join", self.base_url);
        
        let body = serde_json::json!({
            "channel": channel
        });
        
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.bot_token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Verify request signature
    pub fn verify_signature(&self, timestamp: &str, body: &str, signature: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        type HmacSha256 = Hmac<Sha256>;
        
        let base = format!("v0:{}:{}", timestamp, body);
        
        let mut mac = match HmacSha256::new_from_slice(self.config.signing_secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        
        mac.update(base.as_bytes());
        let result = mac.finalize();
        let computed = format!("v0={}", hex::encode(result.into_bytes()));
        
        computed == signature
    }
}

#[async_trait]
impl Channel for SlackChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Slack
    }
    
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        match message.content {
            MessageContent::Text(text) => self.post_message(&message.recipient, &text).await,
            MessageContent::Blocks { text, blocks } => {
                let slack_blocks: Vec<SlackBlock> = blocks.into_iter().map(|b| SlackBlock {
                    block_type: b.block_type,
                    text: b.text.map(|t| BlockText {
                        text_type: "plain_text".into(),
                        text: t,
                    }),
                }).collect();
                self.post_blocks(&message.recipient, &text, slack_blocks).await
            }
            _ => Err(ChannelError::UnsupportedContentType),
        }
    }
    
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        // Slack uses webhooks/events API
        Ok(Vec::new())
    }
    
    fn is_connected(&self) -> bool {
        true
    }
}

/// Slack API response
#[derive(Debug, Deserialize)]
struct SlackResponse {
    ok: bool,
    error: Option<String>,
    ts: Option<String>,
    message_ts: Option<String>,
    view: Option<SlackView>,
}

/// Slack user response
#[derive(Debug, Deserialize)]
struct SlackUserResponse {
    user: SlackUser,
}

/// Slack user info
#[derive(Debug, Deserialize)]
pub struct SlackUser {
    pub id: String,
    pub name: String,
    pub real_name: Option<String>,
    pub profile: Option<UserProfile>,
}

#[derive(Debug, Deserialize)]
pub struct UserProfile {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub image_48: Option<String>,
}

/// Slack channels response
#[derive(Debug, Deserialize)]
struct SlackChannelsResponse {
    channels: Vec<SlackChannelInfo>,
}

/// Slack channel info
#[derive(Debug, Deserialize)]
pub struct SlackChannelInfo {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub is_private: bool,
    pub num_members: Option<i32>,
}

/// Slack block
#[derive(Debug, Serialize)]
pub struct SlackBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: Option<BlockText>,
}

/// Block text
#[derive(Debug, Serialize)]
pub struct BlockText {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: String,
}

/// Slack view (for modals)
#[derive(Debug, Serialize)]
pub struct SlackView {
    #[serde(rename = "type")]
    pub view_type: String,
    pub title: BlockText,
    pub blocks: Vec<SlackBlock>,
    pub submit: Option<BlockText>,
    #[serde(rename = "private_metadata")]
    pub private_metadata: Option<String>,
    #[serde(rename = "callback_id")]
    pub callback_id: Option<String>,
}

/// Slack event (from Events API)
#[derive(Debug, Deserialize)]
pub struct SlackEvent {
    pub event_type: String,
    pub user: Option<String>,
    pub channel: Option<String>,
    pub text: Option<String>,
    pub ts: Option<String>,
    pub thread_ts: Option<String>,
}

impl From<SlackEvent> for ChannelMessage {
    fn from(event: SlackEvent) -> Self {
        Self {
            id: event.ts.unwrap_or_default(),
            channel: ChannelType::Slack,
            sender: event.user.unwrap_or_default(),
            recipient: event.channel.unwrap_or_default(),
            content: MessageContent::Text(event.text.unwrap_or_default()),
            timestamp: chrono::Utc::now(),
            metadata: None,
        }
    }
}
