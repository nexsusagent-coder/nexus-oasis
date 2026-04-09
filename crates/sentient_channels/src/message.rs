//! ─── Channel Message Types ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Channel type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelType {
    Telegram,
    Discord,
    WhatsApp,
    Slack,
    Signal,
    Matrix,
    IRC,
    Webhook,
}

/// Message content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// Plain text
    Text(String),
    /// Markdown formatted text
    Markdown(String),
    /// HTML formatted text
    Html { body: String, formatted: String },
    /// Image with optional caption
    Image {
        url: String,
        caption: Option<String>,
    },
    /// File attachment
    File {
        name: String,
        url: String,
        size: u64,
    },
    /// Audio message
    Audio {
        url: String,
        duration_secs: u32,
    },
    /// Video
    Video {
        url: String,
        duration_secs: u32,
    },
    /// Location
    Location {
        latitude: f64,
        longitude: f64,
        title: Option<String>,
    },
    /// Contact
    Contact {
        name: String,
        phone: String,
    },
    /// Interactive buttons
    Buttons {
        text: String,
        buttons: Vec<Button>,
    },
    /// Card/Embed
    Card {
        title: String,
        description: String,
        image: Option<String>,
        url: Option<String>,
        color: Option<u32>,
    },
    /// Slack blocks
    Blocks {
        text: String,
        blocks: Vec<Block>,
    },
    /// Unknown/Unsupported
    Unknown,
}

/// Slack-style block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_type: String,
    pub text: Option<String>,
}

/// Button for interactive messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub text: String,
    pub action: ButtonAction,
    pub style: ButtonStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonAction {
    Postback(String),
    Url(String),
    Callback(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Success,
}

/// Message sender information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSender {
    pub id: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub is_bot: bool,
}

/// Channel message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    /// Unique message ID
    pub id: Uuid,
    
    /// Source channel
    pub channel: ChannelType,
    
    /// Sender information
    pub sender: MessageSender,
    
    /// Chat/Channel ID
    pub chat_id: String,
    
    /// Message content
    pub content: MessageContent,
    
    /// Reply to message ID (if replying)
    pub reply_to: Option<Uuid>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Extra metadata
    pub metadata: serde_json::Value,
}

impl ChannelMessage {
    /// Create new message
    pub fn new(channel: ChannelType, chat_id: impl Into<String>, content: MessageContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            channel,
            sender: MessageSender {
                id: "sentient".into(),
                name: Some("SENTIENT".into()),
                username: Some("sentient_ai".into()),
                avatar_url: None,
                is_bot: true,
            },
            chat_id: chat_id.into(),
            content,
            reply_to: None,
            timestamp: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }
    
    /// Set sender
    pub fn with_sender(mut self, sender: MessageSender) -> Self {
        self.sender = sender;
        self
    }
    
    /// Reply to another message
    pub fn reply_to(mut self, message_id: Uuid) -> Self {
        self.reply_to = Some(message_id);
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        if self.metadata.is_null() {
            self.metadata = serde_json::json!({});
        }
        if let Some(obj) = self.metadata.as_object_mut() {
            obj.insert(key.into(), value);
        }
        self
    }
    
    /// Get text content (if text message)
    pub fn as_text(&self) -> Option<&str> {
        match &self.content {
            MessageContent::Text(t) => Some(t),
            MessageContent::Markdown(t) => Some(t),
            _ => None,
        }
    }
    
    /// Check if message mentions someone
    pub fn mentions(&self, user_id: &str) -> bool {
        if let Some(text) = self.as_text() {
            text.contains(&format!("@{}", user_id)) || text.contains(&format!("<@{}>", user_id))
        } else {
            false
        }
    }
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::Telegram => write!(f, "telegram"),
            ChannelType::Discord => write!(f, "discord"),
            ChannelType::WhatsApp => write!(f, "whatsapp"),
            ChannelType::Slack => write!(f, "slack"),
            ChannelType::Signal => write!(f, "signal"),
            ChannelType::Matrix => write!(f, "matrix"),
            ChannelType::IRC => write!(f, "irc"),
            ChannelType::Webhook => write!(f, "webhook"),
        }
    }
}
