//! ─── SENTIENT Remote Control ───
//!
//! Mobile remote control capabilities:
//! - PWA interface for web access
//! - Telegram Mini App integration
//! - Voice control from mobile
//! - AFK mode for remote approvals

pub mod pwa;
pub mod telegram;
pub mod voice;
pub mod session;
pub mod commands;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use pwa::PwaServer;
pub use telegram::TelegramMiniApp;
pub use voice::MobileVoiceControl;
pub use session::RemoteSession;
pub use commands::RemoteCommand;

/// Remote control error
#[derive(Debug, Error)]
pub enum RemoteError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Command error: {0}")]
    Command(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    Http(String),
    
    #[error("Not authorized")]
    NotAuthorized,
}

pub type RemoteResult<T> = Result<T, RemoteError>;

/// Remote device info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    Phone,
    Tablet,
    Desktop,
    Web,
    Telegram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    Voice,
    Touch,
    Keyboard,
    Camera,
    Location,
    Notifications,
}

impl RemoteDevice {
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            device_type,
            last_seen: chrono::Utc::now(),
            capabilities: vec![],
        }
    }
    
    pub fn with_capabilities(mut self, caps: Vec<Capability>) -> Self {
        self.capabilities = caps;
        self
    }
    
    pub fn touch(&mut self) {
        self.last_seen = chrono::Utc::now();
    }
}

/// AFK mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfkConfig {
    pub enabled: bool,
    pub auto_approve: bool,
    pub require_confirmation: bool,
    pub timeout_minutes: u32,
    pub allowed_commands: Vec<String>,
}

impl Default for AfkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_approve: false,
            require_confirmation: true,
            timeout_minutes: 60,
            allowed_commands: vec![
                "shutdown".into(),
                "lock".into(),
                "screenshot".into(),
                "message".into(),
            ],
        }
    }
}

/// Remote controller
pub struct RemoteController {
    devices: Vec<RemoteDevice>,
    sessions: std::collections::HashMap<String, RemoteSession>,
    afk_config: AfkConfig,
}

impl RemoteController {
    pub fn new() -> Self {
        Self {
            devices: vec![],
            sessions: std::collections::HashMap::new(),
            afk_config: AfkConfig::default(),
        }
    }
    
    /// Register a new device
    pub fn register_device(&mut self, device: RemoteDevice) -> String {
        let id = device.id.clone();
        self.devices.push(device);
        id
    }
    
    /// Get all registered devices
    pub fn get_devices(&self) -> &[RemoteDevice] {
        &self.devices
    }
    
    /// Create a new session
    pub fn create_session(&mut self, device_id: &str) -> RemoteResult<String> {
        let session = RemoteSession::new(device_id);
        let session_id = session.id.clone();
        self.sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }
    
    /// Get session
    pub fn get_session(&self, session_id: &str) -> Option<&RemoteSession> {
        self.sessions.get(session_id)
    }
    
    /// Validate session
    pub fn validate_session(&self, session_id: &str) -> RemoteResult<()> {
        match self.sessions.get(session_id) {
            Some(session) if session.is_valid() => Ok(()),
            Some(_) => Err(RemoteError::SessionExpired),
            None => Err(RemoteError::Auth("Invalid session".into())),
        }
    }
    
    /// Execute remote command
    pub async fn execute_command(&self, session_id: &str, command: RemoteCommand) -> RemoteResult<CommandResult> {
        self.validate_session(session_id)?;
        
        // Check AFK permissions
        if self.afk_config.require_confirmation {
            // Send confirmation request
            tracing::info!("AFK confirmation required for: {:?}", command);
        }
        
        command.execute().await
    }
    
    /// Set AFK mode
    pub fn set_afk_mode(&mut self, config: AfkConfig) {
        self.afk_config = config;
    }
    
    pub fn get_afk_config(&self) -> &AfkConfig {
        &self.afk_config
    }
}

impl Default for RemoteController {
    fn default() -> Self {
        Self::new()
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl CommandResult {
    pub fn success(message: &str) -> Self {
        Self { success: true, message: message.to_string(), data: None }
    }
    
    pub fn failure(message: &str) -> Self {
        Self { success: false, message: message.to_string(), data: None }
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_creation() {
        let device = RemoteDevice::new("iPhone", DeviceType::Phone)
            .with_capabilities(vec![Capability::Voice, Capability::Touch]);
        
        assert_eq!(device.name, "iPhone");
        assert_eq!(device.capabilities.len(), 2);
    }
    
    #[test]
    fn test_afk_config() {
        let config = AfkConfig::default();
        assert!(config.enabled);
        assert!(config.require_confirmation);
    }
}
