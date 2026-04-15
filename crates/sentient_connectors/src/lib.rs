//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Connectors - Data Source Integration Layer
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Sprint 2: Personal AI - Veri kaynağı konektörleri
//!  - Gmail, Google Calendar, Weather, RSS, GitHub, vb.
//!
//!  Inspired by: OpenJarvis connectors system

pub mod types;
pub mod error;
pub mod oauth;
pub mod connector;

#[cfg(feature = "gmail")]
pub mod gmail;

#[cfg(feature = "calendar")]
pub mod calendar;

#[cfg(feature = "weather")]
pub mod weather;

#[cfg(feature = "rss")]
pub mod rss;

#[cfg(feature = "github")]
pub mod github;

// Re-export key types unconditionally (github always available as core connector)
#[cfg(feature = "github")]
pub use github::{GitHubConnector, GitHubIssue, TrendingRepo};

pub use types::*;
pub use error::{ConnectorError, ConnectorResult};
pub use connector::{Connector, ConnectorRegistry, ConnectorStatus, SyncResult};
pub use oauth::{OAuthConfig, OAuthManager, OAuthToken};
