//! ─── Channels Configuration ───

use serde::{Deserialize, Serialize};

/// Main channels configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    /// Telegram configuration
    pub telegram: Option<ChannelConfig>,
    
    /// Discord configuration
    pub discord: Option<ChannelConfig>,
    
    /// WhatsApp configuration
    pub whatsapp: Option<ChannelConfig>,
    
    /// Slack configuration
    pub slack: Option<ChannelConfig>,
    
    /// Webhook configurations
    pub webhooks: Vec<WebhookConfig>,
    
    /// Global settings
    pub global: GlobalConfig,
}

impl Default for ChannelsConfig {
    fn default() -> Self {
        Self {
            telegram: None,
            discord: None,
            whatsapp: None,
            slack: None,
            webhooks: Vec::new(),
            global: GlobalConfig::default(),
        }
    }
}

/// Individual channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Enable channel
    pub enabled: bool,
    
    /// Bot token / API key
    pub token: String,
    
    /// Bot username (optional)
    pub username: Option<String>,
    
    /// Allowed chat IDs (empty = all)
    pub allowed_chats: Vec<String>,
    
    /// Blocked user IDs
    pub blocked_users: Vec<String>,
    
    /// Admin user IDs
    pub admin_users: Vec<String>,
    
    /// Command prefix (default: /)
    pub command_prefix: String,
    
    /// Rate limit (messages per minute)
    pub rate_limit: u32,
    
    /// Enable natural language commands
    pub natural_language: bool,
    
    /// Welcome message
    pub welcome_message: Option<String>,
}

impl ChannelConfig {
    /// Create Telegram config
    pub fn telegram(token: impl Into<String>) -> Self {
        Self {
            enabled: true,
            token: token.into(),
            username: None,
            allowed_chats: Vec::new(),
            blocked_users: Vec::new(),
            admin_users: Vec::new(),
            command_prefix: "/".into(),
            rate_limit: 30,
            natural_language: true,
            welcome_message: Some("👋 Hello! I'm SENTIENT, your AI assistant. Type /help for commands.".into()),
        }
    }
    
    /// Create Discord config
    pub fn discord(token: impl Into<String>) -> Self {
        Self {
            enabled: true,
            token: token.into(),
            username: None,
            allowed_chats: Vec::new(),
            blocked_users: Vec::new(),
            admin_users: Vec::new(),
            command_prefix: "!".into(),
            rate_limit: 30,
            natural_language: true,
            welcome_message: Some("👋 Hello! I'm SENTIENT, your AI assistant. Type !help for commands.".into()),
        }
    }
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook name
    pub name: String,
    
    /// Webhook URL
    pub url: String,
    
    /// Secret for verification
    pub secret: Option<String>,
    
    /// Events to send
    pub events: Vec<String>,
    
    /// Enable webhook
    pub enabled: bool,
}

/// Global channel settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Enable message logging
    pub log_messages: bool,
    
    /// Max message length
    pub max_message_length: usize,
    
    /// Enable typing indicator
    pub typing_indicator: bool,
    
    /// Typing timeout (seconds)
    pub typing_timeout: u64,
    
    /// Enable read receipts
    pub read_receipts: bool,
    
    /// Auto-reply when processing
    pub auto_reply: bool,
    
    /// Auto-reply message
    pub auto_reply_message: String,
    
    /// Enable profanity filter
    pub profanity_filter: bool,
    
    /// Enable spam detection
    pub spam_detection: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            log_messages: true,
            max_message_length: 4096,
            typing_indicator: true,
            typing_timeout: 30,
            read_receipts: false,
            auto_reply: true,
            auto_reply_message: "Thinking...".into(),
            profanity_filter: true,
            spam_detection: true,
        }
    }
}
