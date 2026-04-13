//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SETUP WIZARD v5.0.0 - Modern Interactive TUI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Arrow-key navigation, Space for multi-select, Enter to confirm
//!  Universal LLM Gateway + 20+ Messaging Channels
//!  Agent-S3 Hardware Permissions

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod wizard;
pub mod config;
pub mod integrations;
pub mod permissions;

pub use wizard::SetupWizard;
pub use config::{
    SetupConfig, ApiKeyConfig, IntegrationConfig, IntegrationConfigs,
    PermissionConfig, SetupStatus, SetupResult, RoutingModeConfig
};
pub use integrations::{TelegramSetup, DiscordSetup, SlackSetup, EmailSetup, GitHubSetup};
pub use permissions::{PermissionLevel, AuthManager};
