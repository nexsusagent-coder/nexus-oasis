//! ─── Trigger System ───
//!
//! Defines trigger types and configurations

use serde::{Deserialize, Serialize};

/// Trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// Unique identifier
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Type of trigger
    pub trigger_type: TriggerType,
    
    /// Action to perform when triggered
    pub action: String,
    
    /// Whether this trigger is enabled
    pub enabled: bool,
    
    /// Priority (higher = more important)
    pub priority: u8,
    
    /// Tags for organization
    pub tags: Vec<String>,
    
    /// Maximum executions (None = unlimited)
    pub max_executions: Option<u32>,
    
    /// Cooldown period in seconds
    pub cooldown_seconds: Option<u64>,
    
    /// Last execution time
    pub last_executed: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Execution count
    pub execution_count: u32,
}

impl Trigger {
    /// Create new trigger
    pub fn new(id: &str, name: &str, trigger_type: TriggerType) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: None,
            trigger_type,
            action: String::new(),
            enabled: true,
            priority: 5,
            tags: Vec::new(),
            max_executions: None,
            cooldown_seconds: None,
            last_executed: None,
            execution_count: 0,
        }
    }
    
    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    
    /// Set action
    pub fn with_action(mut self, action: &str) -> Self {
        self.action = action.to_string();
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
    
    /// Set cooldown
    pub fn with_cooldown(mut self, seconds: u64) -> Self {
        self.cooldown_seconds = Some(seconds);
        self
    }
    
    /// Check if trigger can fire (respects cooldown)
    pub fn can_fire(&self) -> bool {
        if !self.enabled {
            return false;
        }
        
        if let Some(max) = self.max_executions {
            if self.execution_count >= max {
                return false;
            }
        }
        
        if let Some(cooldown) = self.cooldown_seconds {
            if let Some(last) = self.last_executed {
                let elapsed = (chrono::Utc::now() - last).num_seconds() as u64;
                if elapsed < cooldown {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Mark as executed
    pub fn mark_executed(&mut self) {
        self.last_executed = Some(chrono::Utc::now());
        self.execution_count += 1;
    }
}

/// Types of triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum TriggerType {
    /// Time-based trigger (e.g., "09:00 every weekday")
    TimeBased {
        time: String,      // "HH:MM" format
        days: Vec<u8>,     // 0-6 (Sunday-Saturday)
    },
    
    /// Cron expression
    Cron {
        expression: String,
    },
    
    /// Event-based trigger (e.g., "email received")
    EventBased {
        event_type: String,
        conditions: serde_json::Value,
    },
    
    /// Pattern-based trigger (e.g., "every 5th occurrence")
    PatternBased {
        pattern: String,
        threshold: u32,
    },
    
    /// Interval-based trigger
    Interval {
        seconds: u64,
    },
    
    /// Composite trigger (AND/OR of multiple triggers)
    Composite {
        operator: CompositeOperator,
        triggers: Vec<TriggerType>,
    },
    
    /// External webhook trigger
    Webhook {
        endpoint: String,
        secret: Option<String>,
    },
}

impl TriggerType {
    /// Create a daily trigger
    pub fn daily(time: &str) -> Self {
        Self::TimeBased {
            time: time.to_string(),
            days: vec![0, 1, 2, 3, 4, 5, 6],
        }
    }
    
    /// Create a weekday trigger
    pub fn weekdays(time: &str) -> Self {
        Self::TimeBased {
            time: time.to_string(),
            days: vec![1, 2, 3, 4, 5],
        }
    }
    
    /// Create an interval trigger
    pub fn every_minutes(minutes: u64) -> Self {
        Self::Interval {
            seconds: minutes * 60,
        }
    }
    
    /// Create an hourly trigger
    pub fn hourly() -> Self {
        Self::Interval { seconds: 3600 }
    }
}

/// Composite operator for combining triggers
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompositeOperator {
    All,  // All triggers must fire
    Any,  // Any trigger can fire
}

/// Trigger configuration builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConfig {
    /// Maximum concurrent triggers
    pub max_concurrent: usize,
    
    /// Default priority
    pub default_priority: u8,
    
    /// Enable persistence
    pub persist: bool,
    
    /// Storage path (if persisting)
    pub storage_path: Option<String>,
}

impl Default for TriggerConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            default_priority: 5,
            persist: true,
            storage_path: Some("data/triggers.json".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_creation() {
        let trigger = Trigger::new("test", "Test", TriggerType::daily("09:00"))
            .with_action("generate_briefing")
            .with_priority(10);
        
        assert_eq!(trigger.id, "test");
        assert_eq!(trigger.priority, 10);
        assert!(trigger.can_fire());
    }
    
    #[test]
    fn test_cooldown() {
        let mut trigger = Trigger::new("test", "Test", TriggerType::hourly())
            .with_cooldown(10);
        
        assert!(trigger.can_fire());
        trigger.mark_executed();
        assert!(!trigger.can_fire()); // Should respect cooldown
    }
}
