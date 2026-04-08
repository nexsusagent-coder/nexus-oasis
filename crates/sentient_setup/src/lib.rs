//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SETUP WIZARD v4.0.0 - Universal Omni-Gateway
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//!  İlk açılışta çalışan kurulum sihirbazı.
//!  Universal LLM Gateway + 20+ Messaging Channels
//!  Agent-S3 Hardware Permissions

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
