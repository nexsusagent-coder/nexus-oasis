//! ═══════════════════════════════════════════════════════════════════════════════
//!  TELEGRAM CHANNEL - Bot API Integration
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{
    Channel, ChannelError, ChannelType, ChannelMessage, MessageContent, 
    MessageSender, config::ChannelConfig
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use futures::StreamExt;

// ═══════════════════════════════════════════════════════════════════════════════
//  TELEGRAM API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramUpdate {
    update_id: u64,
    message: Option<TelegramMessage>,
    edited_message: Option<TelegramMessage>,
    callback_query: Option<TelegramCallbackQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramMessage {
    message_id: u64,
    from: Option<TelegramUser>,
    chat: TelegramChat,
    text: Option<String>,
    caption: Option<String>,
    photo: Option<Vec<TelegramPhotoSize>>,
    document: Option<TelegramDocument>,
    audio: Option<TelegramAudio>,
    video: Option<TelegramVideo>,
    reply_to_message: Option<Box<TelegramMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramUser {
    id: u64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramChat {
    id: i64,
    #[serde(rename = "type")]
    chat_type: String,
    title: Option<String>,
    username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramPhotoSize {
    file_id: String,
    file_unique_id: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramDocument {
    file_id: String,
    file_unique_id: String,
    file_name: Option<String>,
    mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramAudio {
    file_id: String,
    file_unique_id: String,
    duration: u32,
    title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramVideo {
    file_id: String,
    file_unique_id: String,
    width: u32,
    height: u32,
    duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramCallbackQuery {
    id: String,
    from: TelegramUser,
    message: Option<TelegramMessage>,
    data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SendMessageRequest {
    chat_id: i64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SendMessageResponse {
    ok: bool,
    result: Option<TelegramMessage>,
    description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TELEGRAM CHANNEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Telegram channel implementation
pub struct TelegramChannel {
    config: ChannelConfig,
    client: reqwest::Client,
    bot_token: String,
    base_url: String,
    is_connected: Arc<RwLock<bool>>,
    last_update_id: Arc<RwLock<u64>>,
    message_rx: Option<mpsc::Receiver<ChannelMessage>>,
    message_tx: mpsc::Sender<ChannelMessage>,
}

impl TelegramChannel {
    pub fn new(config: ChannelConfig) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            bot_token: config.token.clone(),
            base_url: format!("https://api.telegram.org/bot{}", config.token),
            client: reqwest::Client::new(),
            config,
            is_connected: Arc::new(RwLock::new(false)),
            last_update_id: Arc::new(RwLock::new(0)),
            message_rx: Some(rx),
            message_tx: tx,
        }
    }

    /// Get bot info
    pub async fn get_me(&self) -> Result<TelegramUser, ChannelError> {
        let url = format!("{}/getMe", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        #[derive(Deserialize)]
        struct Response {
            ok: bool,
            result: Option<TelegramUser>,
            description: Option<String>,
        }
        
        let data: Response = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        data.result.ok_or_else(|| {
            ChannelError::ApiError(data.description.unwrap_or_else(|| "Unknown error".into()))
        })
    }

    /// Send text message
    pub async fn send_text(&self, chat_id: i64, text: &str) -> Result<u64, ChannelError> {
        self.send_text_with_options(chat_id, text, None, None).await
    }

    /// Send text message with options
    pub async fn send_text_with_options(
        &self, 
        chat_id: i64, 
        text: &str, 
        parse_mode: Option<&str>,
        reply_to: Option<u64>
    ) -> Result<u64, ChannelError> {
        // Split long messages
        let max_len = 4096;
        let text = if text.len() > max_len {
            &text[..max_len-3]
        } else {
            text
        };

        let request = SendMessageRequest {
            chat_id,
            text: text.to_string(),
            parse_mode: parse_mode.map(String::from),
            reply_to_message_id: reply_to,
        };

        let url = format!("{}/sendMessage", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SendMessageResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(data.result.map(|m| m.message_id).unwrap_or(0))
        } else {
            Err(ChannelError::ApiError(
                data.description.unwrap_or_else(|| "Send failed".into())
            ))
        }
    }

    /// Send photo
    pub async fn send_photo(&self, chat_id: i64, photo_url: &str, caption: Option<&str>) -> Result<u64, ChannelError> {
        #[derive(Serialize)]
        struct SendPhotoRequest {
            chat_id: i64,
            photo: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            caption: Option<String>,
        }

        let request = SendPhotoRequest {
            chat_id,
            photo: photo_url.to_string(),
            caption: caption.map(String::from),
        };

        let url = format!("{}/sendPhoto", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SendMessageResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(data.result.map(|m| m.message_id).unwrap_or(0))
        } else {
            Err(ChannelError::ApiError(
                data.description.unwrap_or_else(|| "Send photo failed".into())
            ))
        }
    }

    /// Send document
    pub async fn send_document(&self, chat_id: i64, document_url: &str, filename: Option<&str>) -> Result<u64, ChannelError> {
        #[derive(Serialize)]
        struct SendDocumentRequest {
            chat_id: i64,
            document: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            filename: Option<String>,
        }

        let request = SendDocumentRequest {
            chat_id,
            document: document_url.to_string(),
            filename: filename.map(String::from),
        };

        let url = format!("{}/sendDocument", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let data: SendMessageResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(data.result.map(|m| m.message_id).unwrap_or(0))
        } else {
            Err(ChannelError::ApiError(
                data.description.unwrap_or_else(|| "Send document failed".into())
            ))
        }
    }

    /// Get updates (polling)
    async fn get_updates(&self, offset: u64, timeout: u32) -> Result<Vec<TelegramUpdate>, ChannelError> {
        #[derive(Serialize)]
        struct GetUpdatesRequest {
            offset: u64,
            timeout: u32,
            allowed_updates: Vec<String>,
        }

        let request = GetUpdatesRequest {
            offset,
            timeout,
            allowed_updates: vec!["message".into(), "edited_message".into(), "callback_query".into()],
        };

        let url = format!("{}/getUpdates", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&request)
            .timeout(std::time::Duration::from_secs(timeout as u64 + 10))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        #[derive(Deserialize)]
        struct Response {
            ok: bool,
            result: Option<Vec<TelegramUpdate>>,
            description: Option<String>,
        }
        
        let data: Response = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        if data.ok {
            Ok(data.result.unwrap_or_default())
        } else {
            Err(ChannelError::ApiError(
                data.description.unwrap_or_else(|| "Get updates failed".into())
            ))
        }
    }

    /// Convert Telegram message to ChannelMessage
    fn convert_message(&self, msg: TelegramMessage) -> ChannelMessage {
        let content = if let Some(text) = &msg.text {
            MessageContent::Text(text.clone())
        } else if let Some(caption) = &msg.caption {
            if let Some(photos) = &msg.photo {
                // Get largest photo
                let photo = photos.last().map(|p| p.file_id.clone()).unwrap_or_default();
                MessageContent::Image {
                    url: photo,
                    caption: Some(caption.clone()),
                }
            } else if let Some(doc) = &msg.document {
                MessageContent::File {
                    name: doc.file_name.clone().unwrap_or_default(),
                    url: doc.file_id.clone(),
                }
            } else {
                MessageContent::Text(caption.clone())
            }
        } else if let Some(photos) = &msg.photo {
            let photo = photos.last().map(|p| p.file_id.clone()).unwrap_or_default();
            MessageContent::Image {
                url: photo,
                caption: None,
            }
        } else if let Some(doc) = &msg.document {
            MessageContent::File {
                name: doc.file_name.clone().unwrap_or_default(),
                url: doc.file_id.clone(),
            }
        } else {
            MessageContent::Text("[Non-text message]".into())
        };

        let sender = msg.from.map(|u| MessageSender {
            id: u.id.to_string(),
            name: Some(format!("{} {}", u.first_name, u.last_name.unwrap_or_default()).trim().to_string()),
            username: u.username,
            is_bot: u.is_bot,
        }).unwrap_or_default();

        ChannelMessage {
            id: uuid::Uuid::new_v4(),
            channel: ChannelType::Telegram,
            sender,
            chat_id: msg.chat.id.to_string(),
            content,
            reply_to: msg.reply_to_message.map(|_| uuid::Uuid::nil()), // Simplified
            timestamp: chrono::Utc::now(),
        }
    }

    /// Start polling for updates
    pub async fn start_polling(&self) -> Result<(), ChannelError> {
        let connected = self.is_connected.clone();
        let tx = self.message_tx.clone();
        let token = self.bot_token.clone();
        let base_url = self.base_url.clone();
        let last_id = self.last_update_id.clone();
        let config = self.config.clone();

        // Mark as connected
        *connected.write().await = true;

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let mut offset = *last_id.read().await;

            log::info!("Telegram polling started for bot");

            loop {
                if !*connected.read().await {
                    break;
                }

                // Get updates
                let url = format!("{}/getUpdates", base_url);
                #[derive(Serialize)]
                struct Req { offset: u64, timeout: u32, allowed_updates: Vec<String> }
                
                let request = Req {
                    offset,
                    timeout: 30,
                    allowed_updates: vec!["message".into(), "edited_message".into()],
                };

                let response = match client
                    .post(&url)
                    .json(&request)
                    .timeout(std::time::Duration::from_secs(45))
                    .send()
                    .await
                {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("Telegram polling error: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                        continue;
                    }
                };

                #[derive(Deserialize)]
                struct Resp { ok: bool, result: Option<Vec<TelegramUpdate>> }
                
                let data: Resp = match response.json().await {
                    Ok(d) => d,
                    Err(e) => {
                        log::error!("Telegram parse error: {}", e);
                        continue;
                    }
                };

                if let Some(updates) = data.result {
                    for update in updates {
                        offset = update.update_id + 1;

                        if let Some(msg) = update.message {
                            // Check if chat is allowed
                            if !config.allowed_chats.is_empty() {
                                let chat_id = msg.chat.id.to_string();
                                if !config.allowed_chats.contains(&chat_id) {
                                    continue;
                                }
                            }

                            // Check if user is blocked
                            if let Some(user) = &msg.from {
                                if config.blocked_users.contains(&user.id.to_string()) {
                                    continue;
                                }
                            }

                            let channel_msg = TelegramChannel::convert_message_static(msg);
                            
                            if let Err(e) = tx.send(channel_msg).await {
                                log::error!("Failed to send message to channel: {}", e);
                            }
                        }
                    }
                }

                *last_id.write().await = offset;
            }

            log::info!("Telegram polling stopped");
        });

        Ok(())
    }

    fn convert_message_static(msg: TelegramMessage) -> ChannelMessage {
        let content = if let Some(text) = &msg.text {
            MessageContent::Text(text.clone())
        } else {
            MessageContent::Text("[Non-text message]".into())
        };

        let sender = msg.from.map(|u| MessageSender {
            id: u.id.to_string(),
            name: Some(format!("{} {}", u.first_name, u.last_name.unwrap_or_default()).trim().to_string()),
            username: u.username,
            is_bot: u.is_bot,
        }).unwrap_or_default();

        ChannelMessage {
            id: uuid::Uuid::new_v4(),
            channel: ChannelType::Telegram,
            sender,
            chat_id: msg.chat.id.to_string(),
            content,
            reply_to: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[async_trait]
impl Channel for TelegramChannel {
    fn name(&self) -> &str {
        "telegram"
    }

    fn channel_type(&self) -> ChannelType {
        ChannelType::Telegram
    }

    async fn init(&mut self) -> Result<(), ChannelError> {
        // Verify bot token
        let bot_info = self.get_me().await?;
        log::info!("Telegram bot initialized: @{}", bot_info.username.unwrap_or_default());
        
        // Start polling
        self.start_polling().await?;
        
        Ok(())
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let chat_id: i64 = message.chat_id.parse()
            .map_err(|_| ChannelError::InvalidMessage("Invalid chat_id".into()))?;

        let msg_id = match message.content {
            MessageContent::Text(text) => {
                self.send_text(chat_id, &text).await?
            }
            MessageContent::Markdown(text) => {
                self.send_text_with_options(chat_id, &text, Some("Markdown"), None).await?
            }
            MessageContent::Image { url, caption } => {
                self.send_photo(chat_id, &url, caption.as_deref()).await?
            }
            MessageContent::File { name, url } => {
                self.send_document(chat_id, &url, Some(&name)).await?
            }
            MessageContent::Card { title, description, image, url: card_url } => {
                let text = if let Some(img) = image {
                    format!("**{}**\n\n{}\n\n[Link]({})", title, description, card_url.unwrap_or_default())
                } else {
                    format!("**{}**\n\n{}", title, description)
                };
                self.send_text_with_options(chat_id, &text, Some("Markdown"), None).await?
            }
            _ => {
                return Err(ChannelError::InvalidMessage("Unsupported content type".into()));
            }
        };

        Ok(msg_id.to_string())
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        // This would need to be implemented with proper async stream
        Ok(Vec::new())
    }

    async fn shutdown(&mut self) -> Result<(), ChannelError> {
        *self.is_connected.write().await = false;
        log::info!("Telegram channel shutdown");
        Ok(())
    }

    fn is_connected(&self) -> bool {
        // Synchronous check - this is a limitation
        false // Would need to use try_read in real implementation
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
        let config = ChannelConfig::telegram("test_token");
        let channel = TelegramChannel::new(config);
        assert_eq!(channel.name(), "telegram");
        assert_eq!(channel.channel_type(), ChannelType::Telegram);
    }

    #[test]
    fn test_message_conversion() {
        let msg = TelegramMessage {
            message_id: 123,
            from: Some(TelegramUser {
                id: 456,
                is_bot: false,
                first_name: "Test".into(),
                last_name: Some("User".into()),
                username: Some("testuser".into()),
            }),
            chat: TelegramChat {
                id: 789,
                chat_type: "private".into(),
                title: None,
                username: Some("testuser".into()),
            },
            text: Some("Hello".into()),
            caption: None,
            photo: None,
            document: None,
            audio: None,
            video: None,
            reply_to_message: None,
        };

        let converted = TelegramChannel::convert_message_static(msg);
        assert_eq!(converted.chat_id, "789");
        
        if let MessageContent::Text(t) = &converted.content {
            assert_eq!(t, "Hello");
        } else {
            panic!("Expected text content");
        }
    }
}
