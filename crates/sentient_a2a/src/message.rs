//! A2A Message types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A2A Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: String,
    /// Message type
    pub message_type: MessageType,
    /// Sender agent ID
    pub from: String,
    /// Receiver agent ID (or "broadcast")
    pub to: String,
    /// Payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Correlation ID (for request/response)
    pub correlation_id: Option<String>,
    /// TTL in seconds
    pub ttl: Option<u32>,
    /// Priority (1=highest)
    pub priority: u8,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Message {
    pub fn new(from: &str, to: &str, message_type: MessageType, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message_type,
            from: from.to_string(),
            to: to.to_string(),
            payload,
            timestamp: Utc::now(),
            correlation_id: None,
            ttl: None,
            priority: 5,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_correlation_id(mut self, id: &str) -> Self {
        self.correlation_id = Some(id.to_string());
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let elapsed = (Utc::now() - self.timestamp).num_seconds() as u32;
            elapsed > ttl
        } else {
            false
        }
    }

    pub fn is_broadcast(&self) -> bool {
        self.to == "broadcast" || self.to == "*"
    }
}

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    // Discovery
    Ping,
    Pong,
    Discover,
    DiscoverResponse,

    // Registration
    Register,
    RegisterAck,
    Unregister,

    // Communication
    Request,
    Response,
    Notification,
    Event,

    // Task
    TaskAssign,
    TaskStatus,
    TaskResult,
    TaskCancel,

    // Capability
    CapabilityQuery,
    CapabilityResponse,

    // System
    Heartbeat,
    Error,
    Custom(String),
}

/// Message builder
pub struct MessageBuilder {
    from: String,
    to: String,
    message_type: MessageType,
    payload: serde_json::Value,
    correlation_id: Option<String>,
    ttl: Option<u32>,
    priority: u8,
}

impl MessageBuilder {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            message_type: MessageType::Request,
            payload: serde_json::json!(null),
            correlation_id: None,
            ttl: None,
            priority: 5,
        }
    }

    pub fn message_type(mut self, mt: MessageType) -> Self {
        self.message_type = mt;
        self
    }

    pub fn payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = payload;
        self
    }

    pub fn correlation_id(mut self, id: &str) -> Self {
        self.correlation_id = Some(id.to_string());
        self
    }

    pub fn ttl(mut self, ttl: u32) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn build(self) -> Message {
        Message {
            id: Uuid::new_v4().to_string(),
            message_type: self.message_type,
            from: self.from,
            to: self.to,
            payload: self.payload,
            timestamp: Utc::now(),
            correlation_id: self.correlation_id,
            ttl: self.ttl,
            priority: self.priority,
            metadata: std::collections::HashMap::new(),
        }
    }
}
