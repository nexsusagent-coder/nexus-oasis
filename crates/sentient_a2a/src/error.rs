//! A2A error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum A2AError {
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Message delivery failed: {0}")]
    DeliveryFailed(String),

    #[error("Transport error: {0}")]
    TransportError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Registration failed: {0}")]
    RegistrationFailed(String),

    #[error("Capability not supported: {0}")]
    CapabilityNotSupported(String),

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Connection refused: {0}")]
    ConnectionRefused(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type A2AResult<T> = Result<T, A2AError>;
