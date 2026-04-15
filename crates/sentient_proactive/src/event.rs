//! ─── Event System ───
//!
//! Event bus for event-based triggers

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Timestamp;

/// Event bus for publishing and subscribing to events
pub struct EventBus {
    listeners: Arc<RwLock<HashMap<String, Vec<Box<dyn EventListener + Send + Sync>>>>>,
    event_log: Arc<RwLock<Vec<Event>>>,
}

/// An event in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID
    pub id: Uuid,
    
    /// Event type
    pub event_type: EventType,
    
    /// Event payload
    pub payload: serde_json::Value,
    
    /// Source of the event
    pub source: String,
    
    /// Timestamp
    pub timestamp: Timestamp,
    
    /// Priority (0-10, higher = more urgent)
    pub priority: u8,
    
    /// Whether this event requires acknowledgment
    pub requires_ack: bool,
}

impl Event {
    /// Create new event
    pub fn new(event_type: EventType, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            payload,
            source: "system".into(),
            timestamp: chrono::Utc::now(),
            priority: 5,
            requires_ack: false,
        }
    }
    
    /// Set source
    pub fn from_source(mut self, source: &str) -> Self {
        self.source = source.to_string();
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Types of events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // System events
    SystemStart,
    SystemShutdown,
    
    // Email events
    EmailReceived,
    EmailSent,
    
    // Calendar events
    CalendarEventCreated,
    CalendarReminder,
    
    // File events
    FileCreated,
    FileModified,
    FileDeleted,
    
    // Network events
    NetworkConnected,
    NetworkDisconnected,
    
    // App events
    AppLaunched,
    AppClosed,
    
    // Custom events
    Custom(String),
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemStart => write!(f, "system.start"),
            Self::SystemShutdown => write!(f, "system.shutdown"),
            Self::EmailReceived => write!(f, "email.received"),
            Self::EmailSent => write!(f, "email.sent"),
            Self::CalendarEventCreated => write!(f, "calendar.created"),
            Self::CalendarReminder => write!(f, "calendar.reminder"),
            Self::FileCreated => write!(f, "file.created"),
            Self::FileModified => write!(f, "file.modified"),
            Self::FileDeleted => write!(f, "file.deleted"),
            Self::NetworkConnected => write!(f, "network.connected"),
            Self::NetworkDisconnected => write!(f, "network.disconnected"),
            Self::AppLaunched => write!(f, "app.launched"),
            Self::AppClosed => write!(f, "app.closed"),
            Self::Custom(name) => write!(f, "custom.{}", name),
        }
    }
}

/// Event listener trait
#[async_trait::async_trait]
pub trait EventListener {
    /// Handle the event
    async fn on_event(&self, event: &Event);
    
    /// Get the event types this listener cares about
    fn event_types(&self) -> Vec<EventType>;
}

impl EventBus {
    /// Create new event bus
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
            event_log: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Subscribe to events
    pub async fn subscribe<L: EventListener + Send + Sync + Clone + 'static>(&self, listener: L) {
        let mut listeners = self.listeners.write().await;
        for event_type in listener.event_types() {
            let key = event_type.to_string();
            listeners.entry(key)
                .or_insert_with(Vec::new)
                .push(Box::new(listener.clone()));
        }
    }
    
    /// Publish an event
    pub async fn publish(&self, event: Event) {
        // Log the event
        let mut log = self.event_log.write().await;
        log.push(event.clone());
        if log.len() > 1000 {
            log.remove(0);
        }
        drop(log);
        
        // Notify listeners
        let listeners = self.listeners.read().await;
        let key = event.event_type.to_string();
        if let Some(listener_list) = listeners.get(&key) {
            for listener in listener_list {
                listener.on_event(&event).await;
            }
        }
    }
    
    /// Get recent events
    pub async fn get_recent_events(&self, count: usize) -> Vec<Event> {
        let log = self.event_log.read().await;
        log.iter().rev().take(count).cloned().collect()
    }
    
    /// Clear event log
    pub async fn clear_log(&self) {
        let mut log = self.event_log.write().await;
        log.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_event_bus() {
        let bus = EventBus::new();
        
        let event = Event::new(EventType::EmailReceived, serde_json::json!({
            "from": "test@example.com",
            "subject": "Test"
        }));
        
        bus.publish(event).await;
        
        let events = bus.get_recent_events(10).await;
        assert_eq!(events.len(), 1);
    }
    
    #[test]
    fn test_event_type_display() {
        assert_eq!(EventType::EmailReceived.to_string(), "email.received");
        assert_eq!(EventType::Custom("myevent".into()).to_string(), "custom.myevent");
    }
}
