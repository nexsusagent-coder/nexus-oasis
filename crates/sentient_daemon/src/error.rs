//! Daemon error types

use std::fmt;

/// Daemon errors
#[derive(Debug)]
pub enum DaemonError {
    /// Voice error
    Voice(String),
    /// Browser error
    Browser(String),
    /// Config error
    Config(String),
    /// IO error
    Io(String),
    /// Command parse error
    CommandParse(String),
    /// Action execution error
    Action(String),
    /// Shutdown requested
    Shutdown,
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonError::Voice(msg) => write!(f, "Voice error: {}", msg),
            DaemonError::Browser(msg) => write!(f, "Browser error: {}", msg),
            DaemonError::Config(msg) => write!(f, "Config error: {}", msg),
            DaemonError::Io(msg) => write!(f, "IO error: {}", msg),
            DaemonError::CommandParse(msg) => write!(f, "Command parse error: {}", msg),
            DaemonError::Action(msg) => write!(f, "Action error: {}", msg),
            DaemonError::Shutdown => write!(f, "Shutdown requested"),
        }
    }
}

impl std::error::Error for DaemonError {}

/// Result type alias
pub type DaemonResult<T> = Result<T, DaemonError>;

// ═══════════════════════════════════════════════════════════════════════════════
// FROM IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl From<std::io::Error> for DaemonError {
    fn from(e: std::io::Error) -> Self {
        DaemonError::Io(e.to_string())
    }
}

impl From<config::ConfigError> for DaemonError {
    fn from(e: config::ConfigError) -> Self {
        DaemonError::Config(e.to_string())
    }
}
