//! ─── Channel Message Types ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Channel type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelType {
    // Messaging Apps
    Telegram,
    Discord,
    WhatsApp,
    Slack,
    Signal,
    Messenger,      // Facebook Messenger
    Instagram,
    Twitter,
    LinkedIn,
    WeChat,
    Viber,
    Line,
    Snapchat,
    iMessage,
    
    // Enterprise
    Teams,          // Microsoft Teams
    GoogleChat,
    Chime,          // Amazon Chime
    Zoom,
    Webex,
    Mattermost,
    
    // Other
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
    /// Image with optional caption
    Image {
        url: String,
        caption: Option<String>,
    },
    /// File attachment
    File {
        name: String,
        url: String,
    },
    /// Audio message
    Audio {
        url: String,
    },
    /// Video
    Video {
        url: String,
    },
    /// Card/Embed
    Card {
        title: String,
        description: String,
        image: Option<String>,
        url: Option<String>,
    },
}

/// Message sender information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSender {
    pub id: String,
    pub name: Option<String>,
    pub username: Option<String>,
    pub is_bot: bool,
}

impl Default for MessageSender {
    fn default() -> Self {
        Self {
            id: "sentient".into(),
            name: Some("SENTIENT".into()),
            username: Some("sentient_ai".into()),
            is_bot: true,
        }
    }
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
}

impl ChannelMessage {
    /// Create new message
    pub fn new(channel: ChannelType, chat_id: impl Into<String>, content: MessageContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            channel,
            sender: MessageSender::default(),
            chat_id: chat_id.into(),
            content,
            reply_to: None,
            timestamp: Utc::now(),
        }
    }
    
    /// Get text content (if text message)
    pub fn as_text(&self) -> Option<&str> {
        match &self.content {
            MessageContent::Text(t) => Some(t),
            MessageContent::Markdown(t) => Some(t),
            _ => None,
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
            ChannelType::Messenger => write!(f, "messenger"),
            ChannelType::Instagram => write!(f, "instagram"),
            ChannelType::Twitter => write!(f, "twitter"),
            ChannelType::LinkedIn => write!(f, "linkedin"),
            ChannelType::WeChat => write!(f, "wechat"),
            ChannelType::Viber => write!(f, "viber"),
            ChannelType::Line => write!(f, "line"),
            ChannelType::Snapchat => write!(f, "snapchat"),
            ChannelType::iMessage => write!(f, "imessage"),
            ChannelType::Teams => write!(f, "teams"),
            ChannelType::GoogleChat => write!(f, "google_chat"),
            ChannelType::Chime => write!(f, "chime"),
            ChannelType::Zoom => write!(f, "zoom"),
            ChannelType::Webex => write!(f, "webex"),
            ChannelType::Mattermost => write!(f, "mattermost"),
            ChannelType::Matrix => write!(f, "matrix"),
            ChannelType::IRC => write!(f, "irc"),
            ChannelType::Webhook => write!(f, "webhook"),
        }
    }
}
