//! ─── SENTIENT SMART HOME SYSTEM ───
//!
//! Home Assistant integration for JARVIS-like smart home control
//!
//! # Features
//! - **Device Control**: Lights, climate, locks, cameras, media
//! - **Voice Commands**: "Turn on the lights", "Set temperature to 22"
//! - **Automation Triggers**: "When I leave, turn off everything"
//! - **Scene Management**: Predefined and dynamic scenes
//!
//! # Example
//! ```rust,ignore
//! use sentient_home::{HomeClient, DeviceCommand};
//!
//! #[tokio::main]
//! async fn main() {
//!     let home = HomeClient::connect("http://homeassistant.local:8123", "YOUR_TOKEN").await;
//!     
//!     // Voice command: "Turn on living room lights"
//!     home.execute_command(DeviceCommand::LightOn("light.living_room")).await;
//!     
//!     // Automation: "Goodnight mode"
//!     home.activate_scene("goodnight").await;
//! }
//! ```

pub mod models;
pub mod client;
pub mod devices;
pub mod scenes;
pub mod automation;
pub mod voice_commands;

pub use models::{Device, DeviceType, EntityState, Area};
pub use client::{HomeClient, HomeConfig};
pub use devices::{DeviceController, DeviceCommand};
pub use scenes::{SceneManager, Scene};
pub use automation::{AutomationEngine, AutomationRule};
pub use voice_commands::{VoiceCommandParser, ParsedCommand};

pub mod prelude {
    pub use crate::{HomeClient, Device, DeviceCommand, Scene};
}

/// Result type for home operations
pub type HomeResult<T> = Result<T, HomeError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum HomeError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Command failed: {0}")]
    CommandFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = HomeError::DeviceNotFound("light.living_room".into());
        assert!(err.to_string().contains("light.living_room"));
    }
}
