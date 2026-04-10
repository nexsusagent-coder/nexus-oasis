//! ═══════════════════════════════════════════════════════════════════════════════
//!  DISCORD CHANNEL - Bot API Integration
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
//  DISCORD API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordMessage {
    id: String,
    channel_id: String,
    author: DiscordUser,
    content: Option<String>,
    timestamp: String,
    embeds: Vec<DiscordEmbed>,
    attachments: Vec<DiscordAttachment>,
    message_reference: Option<DiscordMessageReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    bot: Option<bool>,
    avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    image: Option<DiscordEmbedImage>,
    thumbnail: Option<DiscordEmbedImage>,
    fields: Option<Vec<DiscordEmbedField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordEmbedImage {
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordEmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordAttachment {
    id: String,
    filename: String,
    url: String,
    content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordMessageReference {
    message_id: String,
    channel_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscordChannelInfo {
    id: String,
    #[serde(rename = "type")]
    channel_type: u8,
    name: Option<String>,
    guild_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateMessageRequest {
    content: Option<String>,
    embed: Option<DiscordEmbed>,
    embeds: Option<Vec<DiscordEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_reference: Option<DiscordMessageReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GatewayHello {
    heartbeat_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GatewayPayload {
    op: u8,
    d: Option<serde_json::Value>,
    s: Option<u64>,
    t: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DISCORD CHANNEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Discord channel implementation
pub struct DiscordChannel {
    config: ChannelConfig,
    client: reqwest::Client,
    bot_token: String,
    base_url: String,
    is_connected: Arc<RwLock<bool>>,
    application_id: Option<String>,
    message_rx: Option<mpsc::Receiver<ChannelMessage>>,
    message_tx: mpsc::Sender<ChannelMessage>,
}

impl DiscordChannel {
    pub fn new(config: ChannelConfig) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            bot_token: config.token.clone(),
            base_url: "https://discord.com/api/v10".to_string(),
            client: reqwest::Client::new(),
            config,
            is_connected: Arc::new(RwLock::new(false)),
            application_id: None,
            message_rx: Some(rx),
            message_tx: tx,
        }
    }

    /// Get current bot user
    pub async fn get_current_user(&self) -> Result<DiscordUser, ChannelError> {
        let url = format!("{}/users/@me", self.base_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ChannelError::AuthFailed("Invalid bot token".into()));
        }

        response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))
    }

    /// Send text message to channel
    pub async fn send_text(&self, channel_id: &str, content: &str) -> Result<String, ChannelError> {
        // Discord has 2000 character limit
        let content = if content.len() > 2000 {
            &content[..2000]
        } else {
            content
        };

        let request = CreateMessageRequest {
            content: Some(content.to_string()),
            embed: None,
            embeds: None,
            message_reference: None,
        };

        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ChannelError::ApiError(format!("Discord API error: {}", error_text)));
        }

        let msg: DiscordMessage = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;

        Ok(msg.id)
    }

    /// Send embed message
    pub async fn send_embed(
        &self, 
        channel_id: &str, 
        title: &str, 
        description: &str,
        image_url: Option<&str>,
        url: Option<&str>
    ) -> Result<String, ChannelError> {
        let embed = DiscordEmbed {
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            url: url.map(String::from),
            image: image_url.map(|u| DiscordEmbedImage { url: u.to_string() }),
            thumbnail: None,
            fields: None,
        };

        let request = CreateMessageRequest {
            content: None,
            embed: Some(embed),
            embeds: None,
            message_reference: None,
        };

        let url_endpoint = format!("{}/channels/{}/messages", self.base_url, channel_id);
        
        let response = self.client
            .post(&url_endpoint)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ChannelError::ApiError("Failed to send embed".into()));
        }

        let msg: DiscordMessage = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;

        Ok(msg.id)
    }

    /// Reply to a message
    pub async fn reply(&self, channel_id: &str, message_id: &str, content: &str) -> Result<String, ChannelError> {
        let request = CreateMessageRequest {
            content: Some(content.to_string()),
            embed: None,
            embeds: None,
            message_reference: Some(DiscordMessageReference {
                message_id: message_id.to_string(),
                channel_id: channel_id.to_string(),
            }),
        };

        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ChannelError::ApiError("Reply failed".into()));
        }

        let msg: DiscordMessage = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;

        Ok(msg.id)
    }

    /// Get channel info
    pub async fn get_channel(&self, channel_id: &str) -> Result<DiscordChannelInfo, ChannelError> {
        let url = format!("{}/channels/{}", self.base_url, channel_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(ChannelError::NotFound(format!("Channel {}", channel_id)));
        }

        response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))
    }

    /// Delete message
    pub async fn delete_message(&self, channel_id: &str, message_id: &str) -> Result<(), ChannelError> {
        let url = format!("{}/channels/{}/messages/{}", self.base_url, channel_id, message_id);
        
        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(ChannelError::ApiError("Delete failed".into()))
        }
    }

    /// Add reaction to message
    pub async fn add_reaction(&self, channel_id: &str, message_id: &str, emoji: &str) -> Result<(), ChannelError> {
        let url = format!(
            "{}/channels/{}/messages/{}/reactions/{}/@me",
            self.base_url, channel_id, message_id, 
            urlencoding::encode(emoji)
        );
        
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(ChannelError::ApiError("Add reaction failed".into()))
        }
    }

    /// Convert Discord message to ChannelMessage
    fn convert_message(&self, msg: DiscordMessage) -> ChannelMessage {
        let content = if let Some(text) = &msg.content {
            if !text.is_empty() {
                MessageContent::Text(text.clone())
            } else if !msg.embeds.is_empty() {
                let embed = &msg.embeds[0];
                MessageContent::Card {
                    title: embed.title.clone().unwrap_or_default(),
                    description: embed.description.clone().unwrap_or_default(),
                    image: embed.image.as_ref().map(|i| i.url.clone()),
                    url: embed.url.clone(),
                }
            } else if !msg.attachments.is_empty() {
                let att = &msg.attachments[0];
                MessageContent::File {
                    name: att.filename.clone(),
                    url: att.url.clone(),
                }
            } else {
                MessageContent::Text("[Empty message]".into())
            }
        } else {
            MessageContent::Text("[Empty message]".into())
        };

        let sender = MessageSender {
            id: msg.author.id,
            name: Some(msg.author.username.clone()),
            username: Some(format!("{}#{}", msg.author.username, msg.author.discriminator)),
            is_bot: msg.author.bot.unwrap_or(false),
        };

        ChannelMessage {
            id: uuid::Uuid::new_v4(),
            channel: ChannelType::Discord,
            sender,
            chat_id: msg.channel_id.clone(),
            content,
            reply_to: msg.message_reference.map(|r| {
                uuid::Uuid::parse_str(&r.message_id).unwrap_or(uuid::Uuid::nil())
            }),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Start gateway connection (WebSocket)
    pub async fn start_gateway(&self) -> Result<(), ChannelError> {
        // Mark as connected
        *self.is_connected.write().await = true;

        let connected = self.is_connected.clone();
        let tx = self.message_tx.clone();
        let token = self.bot_token.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            log::info!("Discord gateway starting...");

            // Use WebSocket to connect to Discord Gateway
            let gateway_url = "wss://gateway.discord.gg/?v=10&encoding=json";
            
            loop {
                if !*connected.read().await {
                    break;
                }

                // Try to connect to gateway
                match tokio_tungstenite::connect_async(gateway_url).await {
                    Ok((ws_stream, _)) => {
                        log::info!("Discord gateway connected");
                        
                        use futures::StreamExt;
                        let (_, mut read) = ws_stream.split();
                        
                        // Handle hello and send identify
                        // This is simplified - real implementation needs proper gateway handling
                        
                        while let Some(msg) = read.next().await {
                            if !*connected.read().await {
                                break;
                            }
                            
                            match msg {
                                Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                                    if let Ok(payload) = serde_json::from_str::<GatewayPayload>(&text) {
                                        match payload.op {
                                            0 => { // Dispatch
                                                if payload.t.as_deref() == Some("MESSAGE_CREATE") {
                                                    if let Some(data) = payload.d {
                                                        if let Ok(discord_msg) = serde_json::from_value::<DiscordMessage>(data) {
                                                            // Check permissions
                                                            let chat_id = discord_msg.channel_id.clone();
                                                            if !config.allowed_chats.is_empty() && !config.allowed_chats.contains(&chat_id) {
                                                                continue;
                                                            }

                                                            let channel_msg = ChannelMessage {
                                                                id: uuid::Uuid::new_v4(),
                                                                channel: ChannelType::Discord,
                                                                sender: MessageSender {
                                                                    id: discord_msg.author.id,
                                                                    name: Some(discord_msg.author.username),
                                                                    username: Some(discord_msg.author.discriminator),
                                                                    is_bot: discord_msg.author.bot.unwrap_or(false),
                                                                },
                                                                chat_id,
                                                                content: MessageContent::Text(discord_msg.content.unwrap_or_default()),
                                                                reply_to: None,
                                                                timestamp: chrono::Utc::now(),
                                                            };
                                                            
                                                            let _ = tx.send(channel_msg).await;
                                                        }
                                                    }
                                                }
                                            }
                                            10 => { // Hello
                                                // Start heartbeat loop
                                                log::debug!("Discord gateway hello received");
                                            }
                                            11 => { // Heartbeat ACK
                                                log::trace!("Discord heartbeat ACK");
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                                    log::warn!("Discord gateway closed, reconnecting...");
                                    break;
                                }
                                Err(e) => {
                                    log::error!("Discord gateway error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to connect to Discord gateway: {}", e);
                    }
                }

                // Wait before reconnecting
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }

            log::info!("Discord gateway stopped");
        });

        Ok(())
    }
}

#[async_trait]
impl Channel for DiscordChannel {
    fn name(&self) -> &str {
        "discord"
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Discord
    }

    async fn init(&mut self) -> Result<(), ChannelError> {
        // Verify bot token
        let bot_user = self.get_current_user().await?;
        log::info!("Discord bot initialized: {}#{}", bot_user.username, bot_user.discriminator);
        
        // Start gateway connection
        self.start_gateway().await?;
        
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let channel_id = &message.chat_id;

        match message.content {
            MessageContent::Text(text) => {
                self.send_text(channel_id, &text).await
            }
            MessageContent::Markdown(text) => {
                self.send_text(channel_id, &text).await
            }
            MessageContent::Card { title, description, image, url } => {
                self.send_embed(
                    channel_id, 
                    &title, 
                    &description, 
                    image.as_deref(),
                    url.as_deref()
                ).await
            }
            MessageContent::Image { url: image_url, caption } => {
                self.send_embed(
                    channel_id,
                    caption.as_deref().unwrap_or("Image"),
                    "",
                    Some(&image_url),
                    None
                ).await
            }
            MessageContent::File { name, url } => {
                self.send_text(channel_id, &format!("File: {} [{}](", name, url)).await
            }
            _ => {
                Err(ChannelError::InvalidMessage("Unsupported content type for Discord".into()))
            }
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        Ok(Vec::new())
    }

    async fn shutdown(&mut self) -> Result<(), ChannelError> {
        *self.is_connected.write().await = false;
        log::info!("Discord channel shutdown");
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
        let config = ChannelConfig::discord("test_token");
        let channel = DiscordChannel::new(config);
        assert_eq!(channel.name(), "discord");
        assert_eq!(channel.channel_type(), ChannelType::Discord);
    }

    #[test]
    fn test_embed_creation() {
        let embed = DiscordEmbed {
            title: Some("Test".into()),
            description: Some("Description".into()),
            url: None,
            image: Some(DiscordEmbedImage { url: "https://example.com/image.png".into() }),
            thumbnail: None,
            fields: None,
        };

        assert_eq!(embed.title, Some("Test".into()));
    }
}
