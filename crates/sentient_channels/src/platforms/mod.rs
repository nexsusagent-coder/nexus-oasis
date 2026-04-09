//! Platform-specific channel implementations
//!
//! Each platform has its own module with channel implementation.

// Phase 1: Core messaging platforms
pub mod whatsapp;
pub mod signal;
pub mod slack;
pub mod matrix_;
pub mod irc_;

// Phase 2: Enterprise and social platforms
pub mod teams;
pub mod wechat;
pub mod line_;
pub mod messenger;

// Re-exports
pub use whatsapp::{WhatsAppChannel, WhatsAppConfig, WhatsAppMessage};
pub use signal::{SignalChannel, SignalConfig, SignalMessage};
pub use slack::{SlackChannel, SlackConfig, SlackMessage, SlackBlock, SlackAction};
pub use matrix_::{MatrixChannel, MatrixConfig, MatrixMessage};
pub use irc_::{IRCChannel, IRCConfig, IRCMessage};

pub use teams::{TeamsChannel, TeamsConfig, TeamsMessage, AdaptiveCardBuilder};
pub use wechat::{WeChatChannel, WeChatConfig, WeChatMessage, WeChatArticle};
pub use line_::{LineChannel, LineConfig, LineMessage, FlexBuilder};
pub use messenger::{MessengerChannel, MessengerConfig, MessengerMessage, QuickReply, GenericElement};

/// Total number of supported channels
pub const CHANNEL_COUNT: usize = 15;

/// List of all supported channel names
pub const SUPPORTED_CHANNELS: &[&str] = &[
    // Phase 1
    "telegram",
    "discord",
    "whatsapp",
    "slack",
    "signal",
    "matrix",
    "irc",
    // Phase 2
    "teams",
    "wechat",
    "line",
    "messenger",
    // Phase 3 (planned)
    "twitter",
    "linkedin",
    "viber",
];
