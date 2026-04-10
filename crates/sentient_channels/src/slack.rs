//! ═══════════════════════════════════════════════════════════════════════════════
//!  SLACK CHANNEL - Bot API Integration
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{
    Channel, ChannelError, ChannelType, ChannelMessage, MessageContent, 
    MessageSender, config::ChannelConfig
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
//  SLACK API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackMessage {
    ts: String,
    channel: String,
    user: Option<String>,
    text: Option<String>,
    bot_id: Option<String>,
    subtype: Option<String>,
    thread_ts: Option<String>,
    files: Option<Vec<SlackFile>>,
    attachments: Option<Vec<SlackAttachment>>,
    blocks: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackFile {
    id: String,
    name: String,
    url_private: Option<String>,
    permalink: Option<String>,
    mimetype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackAttachment {
    text: Option<String>,
    title: Option<String>,
    title_link: Option<String>,
    image_url: Option<String>,
    fallback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackUser {
    id: String,
    name: String,
    real_name: Option<String>,
    profile: Option<SlackProfile>,
    is_bot: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackProfile {
    display_name: Option<String>,
    real_name: Option<String>,
    image_24: Option<String>,
    image_48: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackChannelInfo {
    id: String,
    name: String,
    is_channel: Option<bool>,
    is_group: Option<bool>,
    is_im: Option<bool>,
    is_private: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PostMessageRequest {
    channel: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    blocks: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<SlackAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_broadcast: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackResponse<T> {
    ok: bool,
    #[serde(flatten)]
    data: Option<T>,
    error: Option<String>,
    response_metadata: Option<SlackResponseMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackResponseMetadata {
    next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RtmConnectResponse {
    url: String,
    team: SlackTeam,
    self_user: SlackUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SlackTeam {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventPayload {
    #[serde(rename = "type")]
    event_type: String,
    #[serde(flatten)]
    data: serde_json::Value,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SLACK CHANNEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Slack channel implementation
pub struct SlackChannel {
    config: ChannelConfig,
    client: reqwest::Client,
    bot_token: String,
    app_token: Option<String>,
    base_url: String,
    is_connected: Arc<RwLock<bool>>,
    bot_user_id: Option<String>,
    message_rx: Option<mpsc::Receiver<ChannelMessage>>,
    message_tx: mpsc::Sender<ChannelMessage>,
    users_cache: Arc<RwLock<std::collections::HashMap<String, SlackUser>>>,
}

impl SlackChannel {
    pub fn new(config: ChannelConfig) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            bot_token: config.token.clone(),
            app_token: None,
            base_url: "https://slack.com/api".to_string(),
            client: reqwest::Client::new(),
            config,
            is_connected: Arc::new(RwLock::new(false)),
            bot_user_id: None,
            message_rx: Some(rx),
            message_tx: tx,
            users_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Create with app-level token (for Socket Mode)
    pub fn with_app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = Some(app_token.into());
        self
    }

    /// Test authentication
    pub async fn auth_test(&self) -> Result<SlackUser, ChannelError> {
        let url = format!("{}/auth.test", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<SlackUser> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            data.data.ok_or_else(|| ChannelError::ApiError("No user data".into()))
        } else {
            Err(ChannelError::AuthFailed(data.error.unwrap_or_else(|| "Auth failed".into())))
        }
    }

    /// Send text message
    pub async fn send_text(&self, channel: &str, text: &str) -> Result<String, ChannelError> {
        // Slack has 40000 character limit per message
        let text = if text.len() > 40000 {
            &text[..40000]
        } else {
            text
        };

        let request = PostMessageRequest {
            channel: channel.to_string(),
            text: text.to_string(),
            thread_ts: None,
            blocks: None,
            attachments: None,
            reply_broadcast: None,
        };

        self.post_message(request).await
    }

    /// Send message with blocks (formatted)
    pub async fn send_blocks(
        &self, 
        channel: &str, 
        text: &str,
        blocks: Vec<serde_json::Value>
    ) -> Result<String, ChannelError> {
        let request = PostMessageRequest {
            channel: channel.to_string(),
            text: text.to_string(),
            thread_ts: None,
            blocks: Some(blocks),
            attachments: None,
            reply_broadcast: None,
        };

        self.post_message(request).await
    }

    /// Send reply in thread
    pub async fn reply_in_thread(
        &self, 
        channel: &str, 
        thread_ts: &str, 
        text: &str,
        broadcast: bool
    ) -> Result<String, ChannelError> {
        let request = PostMessageRequest {
            channel: channel.to_string(),
            text: text.to_string(),
            thread_ts: Some(thread_ts.to_string()),
            blocks: None,
            attachments: None,
            reply_broadcast: if broadcast { Some(true) } else { None },
        };

        self.post_message(request).await
    }

    /// Post message helper
    async fn post_message(&self, request: PostMessageRequest) -> Result<String, ChannelError> {
        let url = format!("{}/chat.postMessage", self.base_url);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<SlackMessage> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            data.data.map(|m| m.ts).ok_or_else(|| ChannelError::ApiError("No message ts".into()))
        } else {
            Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "Post failed".into())))
        }
    }

    /// Get user info
    pub async fn get_user(&self, user_id: &str) -> Result<SlackUser, ChannelError> {
        // Check cache first
        {
            let cache = self.users_cache.read().await;
            if let Some(user) = cache.get(user_id) {
                return Ok(user.clone());
            }
        }

        let url = format!("{}/users.info?user={}", self.base_url, user_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<SlackUser> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            if let Some(user) = &data.data {
                // Cache user
                self.users_cache.write().await.insert(user_id.to_string(), user.clone());
            }
            data.data.ok_or_else(|| ChannelError::ApiError("No user data".into()))
        } else {
            Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "User not found".into())))
        }
    }

    /// Get channel info
    pub async fn get_channel(&self, channel_id: &str) -> Result<SlackChannelInfo, ChannelError> {
        let url = format!("{}/conversations.info?channel={}", self.base_url, channel_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<SlackChannelInfo> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            data.data.ok_or_else(|| ChannelError::ApiError("No channel data".into()))
        } else {
            Err(ChannelError::NotFound(format!("Channel {}", channel_id)))
        }
    }

    /// List channels
    pub async fn list_channels(&self) -> Result<Vec<SlackChannelInfo>, ChannelError> {
        let url = format!("{}/conversations.list?types=public_channel,private_channel", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        #[derive(Deserialize)]
        struct ChannelsResponse {
            channels: Vec<SlackChannelInfo>,
        }
        
        let data: SlackResponse<ChannelsResponse> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(data.data.map(|d| d.channels).unwrap_or_default())
        } else {
            Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "List failed".into())))
        }
    }

    /// Delete message
    pub async fn delete_message(&self, channel: &str, ts: &str) -> Result<(), ChannelError> {
        let url = format!("{}/chat.delete", self.base_url);
        
        #[derive(Serialize)]
        struct DeleteRequest {
            channel: String,
            ts: String,
        }

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&DeleteRequest {
                channel: channel.to_string(),
                ts: ts.to_string(),
            })
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<()> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(())
        } else {
            Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "Delete failed".into())))
        }
    }

    /// Add reaction
    pub async fn add_reaction(&self, channel: &str, ts: &str, emoji: &str) -> Result<(), ChannelError> {
        let url = format!("{}/reactions.add", self.base_url);
        
        #[derive(Serialize)]
        struct ReactionRequest {
            channel: String,
            timestamp: String,
            name: String,
        }

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&ReactionRequest {
                channel: channel.to_string(),
                timestamp: ts.to_string(),
                name: emoji.to_string(),
            })
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<()> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(())
        } else {
            Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "Reaction failed".into())))
        }
    }

    /// Convert Slack message to ChannelMessage
    async fn convert_message(&self, msg: SlackMessage) -> ChannelMessage {
        let sender = if let Some(user_id) = &msg.user {
            match self.get_user(user_id).await {
                Ok(user) => MessageSender {
                    id: user.id,
                    name: user.real_name,
                    username: user.name.into(),
                    is_bot: user.is_bot.unwrap_or(false),
                },
                Err(_) => MessageSender {
                    id: user_id.clone(),
                    name: None,
                    username: Some(user_id.clone()),
                    is_bot: false,
                },
            }
        } else {
            MessageSender::default()
        };

        let content = if let Some(text) = &msg.text {
            MessageContent::Text(text.clone())
        } else if let Some(files) = &msg.files {
            if !files.is_empty() {
                let file = &files[0];
                MessageContent::File {
                    name: file.name.clone(),
                    url: file.permalink.clone().unwrap_or_default(),
                }
            } else {
                MessageContent::Text("[Empty message]".into())
            }
        } else if let Some(attachments) = &msg.attachments {
            if !attachments.is_empty() {
                let att = &attachments[0];
                MessageContent::Card {
                    title: att.title.clone().unwrap_or_default(),
                    description: att.text.clone().unwrap_or_default(),
                    image: att.image_url.clone(),
                    url: att.title_link.clone(),
                }
            } else {
                MessageContent::Text("[Empty message]".into())
            }
        } else {
            MessageContent::Text("[Empty message]".into())
        };

        ChannelMessage {
            id: uuid::Uuid::new_v4(),
            channel: ChannelType::Slack,
            sender,
            chat_id: msg.channel.clone(),
            content,
            reply_to: msg.thread_ts.and_then(|ts| {
                // Parse ts to uuid (simplified)
                Some(uuid::Uuid::new_v4())
            }),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Start Socket Mode (recommended for production)
    pub async fn start_socket_mode(&self) -> Result<(), ChannelError> {
        let app_token = self.app_token.as_ref()
            .ok_or_else(|| ChannelError::AuthFailed("App-level token required for Socket Mode".into()))?;

        *self.is_connected.write().await = true;

        let connected = self.is_connected.clone();
        let tx = self.message_tx.clone();
        let bot_token = self.bot_token.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            log::info!("Slack Socket Mode starting...");

            // Connect to Socket Mode WebSocket
            let socket_url = "wss://wss-primary.slack.com/socket";

            loop {
                if !*connected.read().await {
                    break;
                }

                match tokio_tungstenite::connect_async(socket_url).await {
                    Ok((ws_stream, _)) => {
                        log::info!("Slack Socket Mode connected");

                        use futures::StreamExt;
                        use futures::SinkExt;
                        let (mut sink, mut read) = ws_stream.split();

                        // Send hello
                        let hello = serde_json::json!({
                            "type": "hello"
                        });
                        let _ = sink.send(tokio_tungstenite::tungstenite::Message::Text(
                            hello.to_string()
                        )).await;

                        while let Some(msg) = read.next().await {
                            if !*connected.read().await {
                                break;
                            }

                            match msg {
                                Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&text) {
                                        if let Some(event_type) = payload.get("type").and_then(|v| v.as_str()) {
                                            match event_type {
                                                "events_api" => {
                                                    // Acknowledge
                                                    if let Some(envelope_id) = payload.get("envelope_id") {
                                                        let ack = serde_json::json!({
                                                            "envelope_id": envelope_id
                                                        });
                                                        let _ = sink.send(
                                                            tokio_tungstenite::tungstenite::Message::Text(ack.to_string())
                                                        ).await;
                                                    }

                                                    // Parse event
                                                    if let Some(event) = payload.get("payload").and_then(|p| p.get("event")) {
                                                        if event.get("type").and_then(|v| v.as_str()) == Some("message") {
                                                            // Skip bot messages
                                                            if event.get("bot_id").is_some() {
                                                                continue;
                                                            }

                                                            let channel = event.get("channel").and_then(|v| v.as_str()).unwrap_or("");
                                                            
                                                            // Check permissions
                                                            if !config.allowed_chats.is_empty() && !config.allowed_chats.contains(&channel.to_string()) {
                                                                continue;
                                                            }

                                                            let channel_msg = ChannelMessage {
                                                                id: uuid::Uuid::new_v4(),
                                                                channel: ChannelType::Slack,
                                                                sender: MessageSender {
                                                                    id: event.get("user").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                                                                    name: None,
                                                                    username: None,
                                                                    is_bot: false,
                                                                },
                                                                chat_id: channel.to_string(),
                                                                content: MessageContent::Text(
                                                                    event.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string()
                                                                ),
                                                                reply_to: event.get("thread_ts").map(|_| uuid::Uuid::new_v4()),
                                                                timestamp: chrono::Utc::now(),
                                                            };

                                                            let _ = tx.send(channel_msg).await;
                                                        }
                                                    }
                                                }
                                                "hello" => {
                                                    log::debug!("Slack Socket Mode hello received");
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                                    log::warn!("Slack Socket Mode closed, reconnecting...");
                                    break;
                                }
                                Err(e) => {
                                    log::error!("Slack Socket Mode error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to connect to Slack Socket Mode: {}", e);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }

            log::info!("Slack Socket Mode stopped");
        });

        Ok(())
    }

    /// Start RTM (legacy, simpler)
    pub async fn start_rtm(&self) -> Result<(), ChannelError> {
        let url = format!("{}/rtm.connect", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SlackResponse<RtmConnectResponse> = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if !data.ok {
            return Err(ChannelError::ApiError(data.error.unwrap_or_else(|| "RTM connect failed".into())));
        }

        let rtm_data = data.data.ok_or_else(|| ChannelError::ApiError("No RTM data".into()))?;
        
        log::info!("Slack RTM connected to team: {}", rtm_data.team.name);

        *self.is_connected.write().await = true;

        let connected = self.is_connected.clone();
        let tx = self.message_tx.clone();
        let ws_url = rtm_data.url;

        tokio::spawn(async move {
            log::info!("Slack RTM WebSocket connecting...");

            if let Ok((ws_stream, _)) = tokio_tungstenite::connect_async(&ws_url).await {
                log::info!("Slack RTM connected");
                
                use futures::StreamExt;
                let (_, mut read) = ws_stream.split();

                while let Some(msg) = read.next().await {
                    if !*connected.read().await {
                        break;
                    }

                    if let Ok(tokio_tungstenite::tungstenite::Message::Text(text)) = msg {
                        if let Ok(event) = serde_json::from_str::<EventPayload>(&text) {
                            if event.event_type == "message" {
                                if let Ok(slack_msg) = serde_json::from_value::<SlackMessage>(event.data) {
                                    // Skip bot messages
                                    if slack_msg.bot_id.is_some() {
                                        continue;
                                    }

                                    let channel_msg = ChannelMessage {
                                        id: uuid::Uuid::new_v4(),
                                        channel: ChannelType::Slack,
                                        sender: MessageSender {
                                            id: slack_msg.user.clone().unwrap_or_default(),
                                            name: None,
                                            username: None,
                                            is_bot: false,
                                        },
                                        chat_id: slack_msg.channel.clone(),
                                        content: MessageContent::Text(slack_msg.text.unwrap_or_default()),
                                        reply_to: None,
                                        timestamp: chrono::Utc::now(),
                                    };

                                    let _ = tx.send(channel_msg).await;
                                }
                            }
                        }
                    }
                }
            }

            log::info!("Slack RTM stopped");
        });

        Ok(())
    }
}

#[async_trait]
impl Channel for SlackChannel {
    fn name(&self) -> &str {
        "slack"
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Slack
    }

    async fn init(&mut self) -> Result<(), ChannelError> {
        // Verify auth
        let user = self.auth_test().await?;
        log::info!("Slack bot initialized: {}", user.name);
        self.bot_user_id = Some(user.id);

        // Start Socket Mode if app token available, otherwise RTM
        if self.app_token.is_some() {
            self.start_socket_mode().await?;
        } else {
            self.start_rtm().await?;
        }
        
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let channel = &message.chat_id;

        match message.content {
            MessageContent::Text(text) => {
                self.send_text(channel, &text).await
            }
            MessageContent::Markdown(text) => {
                // Convert markdown to Slack mrkdwn
                let text = text
                    .replace("**", "*")
                    .replace("__", "_")
                    .replace("~~", "~");
                self.send_text(channel, &text).await
            }
            MessageContent::Card { title, description, image, url } => {
                let blocks = vec![
                    serde_json::json!({
                        "type": "section",
                        "text": {
                            "type": "mrkdwn",
                            "text": format!("*{}*\n{}", title, description)
                        }
                    })
                ];
                self.send_blocks(channel, &format!("{}: {}", title, description), blocks).await
            }
            MessageContent::Image { url: image_url, caption } => {
                let blocks = vec![
                    serde_json::json!({
                        "type": "image",
                        "image_url": image_url,
                        "alt_text": caption.as_deref().unwrap_or("Image")
                    })
                ];
                self.send_blocks(channel, caption.as_deref().unwrap_or("Image"), blocks).await
            }
            MessageContent::File { name, url } => {
                self.send_text(channel, &format!("File: {} <{}|Download>", name, url)).await
            }
            _ => {
                Err(ChannelError::InvalidMessage("Unsupported content type for Slack".into()))
            }
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        Ok(Vec::new())
    }

    async fn shutdown(&mut self) -> Result<(), ChannelError> {
        *self.is_connected.write().await = false;
        log::info!("Slack channel shutdown");
        Ok(())
    }

    fn is_connected(&self) -> bool {
        false
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let config = ChannelConfig {
            enabled: true,
            token: "xoxb-test".into(),
            username: None,
            allowed_chats: Vec::new(),
            blocked_users: Vec::new(),
            admin_users: Vec::new(),
            command_prefix: "/".into(),
            rate_limit: 30,
            natural_language: true,
            welcome_message: None,
        };
        let channel = SlackChannel::new(config);
        assert_eq!(channel.name(), "slack");
        assert_eq!(channel.channel_type(), ChannelType::Slack);
    }

    #[test]
    fn test_slack_message_types() {
        let msg = SlackMessage {
            ts: "1234567890.123456".into(),
            channel: "C12345".into(),
            user: Some("U12345".into()),
            text: Some("Hello".into()),
            bot_id: None,
            subtype: None,
            thread_ts: None,
            files: None,
            attachments: None,
            blocks: None,
        };

        assert_eq!(msg.text, Some("Hello".into()));
    }
}
