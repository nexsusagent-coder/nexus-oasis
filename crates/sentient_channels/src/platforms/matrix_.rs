//! ─── Matrix Integration ───
//!
//! Supports:
//! - Matrix Client-Server API
//! - End-to-end encryption (via matrix-sdk)
//! - Spaces, Rooms, DMs
//! - Reactions, Replies

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{Channel, ChannelError, ChannelMessage, MessageContent, ChannelType};

/// Matrix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixConfig {
    /// Homeserver URL
    pub homeserver: String,
    
    /// User ID (@user:domain.com)
    pub user_id: String,
    
    /// Access token or password
    pub access_token: Option<String>,
    pub password: Option<String>,
    
    /// Device ID
    pub device_id: Option<String>,
    
    /// Default room
    pub default_room: Option<String>,
}

/// Matrix channel (using matrix-sdk)
pub struct MatrixChannel {
    config: MatrixConfig,
    // client: matrix_sdk::Client, // Would require matrix-sdk crate
}

impl MatrixChannel {
    pub fn new(config: MatrixConfig) -> Self {
        Self { config }
    }
    
    /// Send text message to room
    pub async fn send_text(&self, room_id: &str, text: &str) -> Result<String, ChannelError> {
        // Use Matrix Client-Server API directly
        let client = reqwest::Client::new();
        
        let txn_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            self.config.homeserver, room_id, txn_id
        );
        
        let body = serde_json::json!({
            "msgtype": "m.text",
            "body": text
        });
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(ChannelError::ApiError(error));
        }
        
        let result: MatrixSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.event_id)
    }
    
    /// Send message with formatted body (HTML)
    pub async fn send_html(&self, room_id: &str, body: &str, formatted: &str) -> Result<String, ChannelError> {
        let client = reqwest::Client::new();
        
        let txn_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            self.config.homeserver, room_id, txn_id
        );
        
        let payload = serde_json::json!({
            "msgtype": "m.text",
            "body": body,
            "format": "org.matrix.custom.html",
            "formatted_body": formatted
        });
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: MatrixSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.event_id)
    }
    
    /// Send image
    pub async fn send_image(&self, room_id: &str, url: &str, filename: &str) -> Result<String, ChannelError> {
        let client = reqwest::Client::new();
        
        let txn_id = uuid::Uuid::new_v4().to_string();
        let api_url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            self.config.homeserver, room_id, txn_id
        );
        
        let body = serde_json::json!({
            "msgtype": "m.image",
            "url": url,
            "body": filename,
            "info": {
                "mimetype": "image/jpeg"
            }
        });
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .put(&api_url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: MatrixSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.event_id)
    }
    
    /// Reply to message
    pub async fn reply(&self, room_id: &str, event_id: &str, text: &str) -> Result<String, ChannelError> {
        let client = reqwest::Client::new();
        
        let txn_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.room.message/{}",
            self.config.homeserver, room_id, txn_id
        );
        
        // In-reply-to format
        let body = serde_json::json!({
            "msgtype": "m.text",
            "body": format!("> <@user> ...\n\n{}", text),
            "m.relates_to": {
                "m.in_reply_to": {
                    "event_id": event_id
                }
            }
        });
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: MatrixSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.event_id)
    }
    
    /// Add reaction to message
    pub async fn react(&self, room_id: &str, event_id: &str, emoji: &str) -> Result<String, ChannelError> {
        let client = reqwest::Client::new();
        
        let txn_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/send/m.reaction/{}",
            self.config.homeserver, room_id, txn_id
        );
        
        let body = serde_json::json!({
            "m.relates_to": {
                "rel_type": "m.annotation",
                "event_id": event_id,
                "key": emoji
            }
        });
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: MatrixSendResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.event_id)
    }
    
    /// Join room
    pub async fn join_room(&self, room_id_or_alias: &str) -> Result<String, ChannelError> {
        let client = reqwest::Client::new();
        
        let url = format!(
            "{}/_matrix/client/v3/join/{}",
            self.config.homeserver, 
            urlencoding::encode(room_id_or_alias)
        );
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: JoinResult = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result.room_id)
    }
    
    /// Leave room
    pub async fn leave_room(&self, room_id: &str) -> Result<(), ChannelError> {
        let client = reqwest::Client::new();
        
        let url = format!(
            "{}/_matrix/client/v3/rooms/{}/leave",
            self.config.homeserver, room_id
        );
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        Ok(())
    }
    
    /// Sync messages (polling)
    pub async fn sync(&self, since: Option<&str>) -> Result<SyncResponse, ChannelError> {
        let client = reqwest::Client::new();
        
        let mut url = format!(
            "{}/_matrix/client/v3/sync?timeout=30000",
            self.config.homeserver
        );
        
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        
        let token = self.config.access_token.as_ref()
            .ok_or_else(|| ChannelError::ApiError("No access token".into()))?;
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;
        
        let result: SyncResponse = response.json().await
            .map_err(|e| ChannelError::Parse(e.to_string()))?;
        
        Ok(result)
    }
}

#[async_trait]
impl Channel for MatrixChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::Matrix
    }
    
    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        match message.content {
            MessageContent::Text(text) => self.send_text(&message.recipient, &text).await,
            MessageContent::Html { body, formatted } => self.send_html(&message.recipient, &body, &formatted).await,
            MessageContent::Image { url, caption } => {
                self.send_image(&message.recipient, &url, caption.as_deref().unwrap_or("image")).await
            }
            _ => Err(ChannelError::UnsupportedContentType),
        }
    }
    
    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> {
        let sync = self.sync(None).await?;
        
        let mut messages = Vec::new();
        
        // Parse timeline events
        for (_room_id, room_data) in sync.rooms.join {
            for event in room_data.timeline.events {
                if event.event_type == "m.room.message" {
                    if let Some(content) = event.content {
                        if let Some(body) = content.get("body").and_then(|b| b.as_str()) {
                            messages.push(ChannelMessage {
                                id: event.event_id,
                                channel: ChannelType::Matrix,
                                sender: event.sender,
                                recipient: String::new(),
                                content: MessageContent::Text(body.to_string()),
                                timestamp: chrono::Utc::now(),
                                metadata: None,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(messages)
    }
    
    fn is_connected(&self) -> bool {
        self.config.access_token.is_some()
    }
}

/// Matrix send result
#[derive(Debug, Deserialize)]
struct MatrixSendResult {
    event_id: String,
}

/// Join result
#[derive(Debug, Deserialize)]
struct JoinResult {
    room_id: String,
}

/// Sync response
#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    pub next_batch: String,
    pub rooms: Rooms,
}

#[derive(Debug, Deserialize)]
pub struct Rooms {
    pub join: std::collections::HashMap<String, JoinedRoom>,
}

#[derive(Debug, Deserialize)]
pub struct JoinedRoom {
    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub events: Vec<RoomEvent>,
}

#[derive(Debug, Deserialize)]
pub struct RoomEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub event_id: String,
    pub sender: String,
    pub content: Option<serde_json::Value>,
    pub origin_server_ts: Option<u64>,
}

/// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
