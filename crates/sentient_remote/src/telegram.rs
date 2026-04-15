//! ─── Telegram Mini App Integration ───

use serde::{Deserialize, Serialize};
use crate::{RemoteResult, RemoteError, RemoteCommand, CommandResult};

/// Telegram Mini App configuration
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub mini_app_url: String,
    pub allowed_users: Vec<i64>,
}

/// Telegram Mini App handler
pub struct TelegramMiniApp {
    config: TelegramConfig,
    http: reqwest::Client,
}

impl TelegramMiniApp {
    pub fn new(config: TelegramConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }
    
    /// Initialize mini app
    pub async fn init(&self) -> RemoteResult<()> {
        tracing::info!("Initializing Telegram Mini App");
        Ok(())
    }
    
    /// Send message
    pub async fn send_message(&self, chat_id: i64, text: &str) -> RemoteResult<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.config.bot_token
        );
        
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "parse_mode": "HTML"
        });
        
        let resp = self.http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| RemoteError::Http(e.to_string()))?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(RemoteError::Http(format!("Telegram API error: {}", resp.status())))
        }
    }
    
    /// Send inline keyboard
    pub async fn send_keyboard(&self, chat_id: i64, text: &str, buttons: Vec<Vec<KeyboardButton>>) -> RemoteResult<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.config.bot_token
        );
        
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": text,
            "reply_markup": {
                "inline_keyboard": buttons.iter().map(|row| 
                    row.iter().map(|btn| serde_json::json!({
                        "text": btn.text,
                        "callback_data": btn.callback_data
                    })).collect::<Vec<_>>()
                ).collect::<Vec<_>>()
            }
        });
        
        let resp = self.http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| RemoteError::Http(e.to_string()))?;
        
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(RemoteError::Http(format!("Telegram API error: {}", resp.status())))
        }
    }
    
    /// Handle callback query
    pub async fn handle_callback(&self, query_id: &str, data: &str) -> RemoteResult<CommandResult> {
        // Answer callback
        self.answer_callback(query_id).await?;
        
        // Parse command
        let command = RemoteCommand::from_callback_data(data)?;
        command.execute().await
    }
    
    async fn answer_callback(&self, query_id: &str) -> RemoteResult<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/answerCallbackQuery",
            self.config.bot_token
        );
        
        let body = serde_json::json!({
            "callback_query_id": query_id
        });
        
        self.http
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| RemoteError::Http(e.to_string()))?;
        
        Ok(())
    }
    
    /// Check user authorization
    pub fn is_authorized(&self, user_id: i64) -> bool {
        self.config.allowed_users.contains(&user_id)
    }
    
    /// Generate Mini App web app data
    pub fn generate_web_app_data(&self, user_id: i64) -> WebAppData {
        WebAppData {
            user_id,
            query_id: uuid::Uuid::new_v4().to_string(),
            auth_date: chrono::Utc::now().timestamp(),
            hash: "generated_hash".into(), // TODO: Proper HMAC
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,
    pub callback_data: String,
}

impl KeyboardButton {
    pub fn new(text: &str, callback_data: &str) -> Self {
        Self { text: text.into(), callback_data: callback_data.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppData {
    pub user_id: i64,
    pub query_id: String,
    pub auth_date: i64,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<TelegramMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<TelegramCallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub from: TelegramUser,
    pub chat: TelegramChat,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramChat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramCallback {
    pub id: String,
    pub from: TelegramUser,
    pub data: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyboard_button() {
        let btn = KeyboardButton::new("Approve", "approve_cmd");
        assert_eq!(btn.text, "Approve");
    }
}
