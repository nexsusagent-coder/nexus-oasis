//! ─── Automation Engine ───

use serde::{Deserialize, Serialize};

/// Automation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub trigger: AutomationTrigger,
    pub condition: Option<AutomationCondition>,
    pub action: AutomationAction,
    pub enabled: bool,
}

impl AutomationRule {
    pub fn new(id: &str, name: &str, trigger: AutomationTrigger, action: AutomationAction) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            trigger,
            condition: None,
            action,
            enabled: true,
        }
    }
    
    pub fn with_condition(mut self, condition: AutomationCondition) -> Self {
        self.condition = Some(condition);
        self
    }
}

/// Automation trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationTrigger {
    // Time-based
    Time { time: String },
    Sunrise,
    Sunset,
    
    // State-based
    StateChanged {
        entity_id: String,
        from: Option<String>,
        to: Option<String>,
    },
    
    // Event-based
    Event {
        event_type: String,
        event_data: Option<serde_json::Value>,
    },
    
    // Location-based
    ZoneEntered {
        zone: String,
        person: String,
    },
    ZoneExited {
        zone: String,
        person: String,
    },
    
    // Numeric
    NumericState {
        entity_id: String,
        above: Option<f64>,
        below: Option<f64>,
    },
    
    // Webhook
    Webhook {
        webhook_id: String,
    },
}

/// Automation condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationCondition {
    State {
        entity_id: String,
        state: String,
    },
    Time {
        after: Option<String>,
        before: Option<String>,
    },
    Sun {
        after: Option<String>,
        before: Option<String>,
    },
    Zone {
        zone: String,
        person: String,
    },
    And(Vec<AutomationCondition>),
    Or(Vec<AutomationCondition>),
    Not(Box<AutomationCondition>),
}

/// Automation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationAction {
    CallService {
        entity_id: String,
        service: String,
        data: serde_json::Value,
    },
    ActivateScene {
        scene_id: String,
    },
    Delay {
        seconds: u64,
    },
    Notify {
        message: String,
        title: Option<String>,
    },
    Sequence(Vec<AutomationAction>),
    Parallel(Vec<AutomationAction>),
    Repeat {
        count: u32,
        actions: Vec<AutomationAction>,
    },
}

/// Automation engine
pub struct AutomationEngine {
    rules: Vec<AutomationRule>,
}

impl AutomationEngine {
    pub fn new() -> Self {
        Self {
            rules: default_automations(),
        }
    }
    
    /// Get all rules
    pub fn get_rules(&self) -> &[AutomationRule] {
        &self.rules
    }
    
    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&AutomationRule> {
        self.rules.iter().filter(|r| r.enabled).collect()
    }
    
    /// Add rule
    pub fn add_rule(&mut self, rule: AutomationRule) {
        self.rules.push(rule);
    }
    
    /// Enable/disable rule
    pub fn set_rule_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == id) {
            rule.enabled = enabled;
        }
    }
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Default automations
fn default_automations() -> Vec<AutomationRule> {
    vec![
        // Turn on lights when sun sets
        AutomationRule::new(
            "sunset_lights",
            "Turn on lights at sunset",
            AutomationTrigger::Sunset,
            AutomationAction::ActivateScene { scene_id: "evening".into() },
        ),
        
        // Turn off everything when leaving
        AutomationRule::new(
            "away_mode",
            "Activate away mode when everyone leaves",
            AutomationTrigger::ZoneExited {
                zone: "home".into(),
                person: "all".into(),
            },
            AutomationAction::ActivateScene { scene_id: "away".into() },
        ),
        
        // Night mode
        AutomationRule::new(
            "night_mode",
            "Activate night mode at 23:00",
            AutomationTrigger::Time { time: "23:00".into() },
            AutomationAction::ActivateScene { scene_id: "good_night".into() },
        ),
        
        // Morning routine
        AutomationRule::new(
            "morning_routine",
            "Morning routine at sunrise",
            AutomationTrigger::Sunrise,
            AutomationAction::ActivateScene { scene_id: "good_morning".into() },
        ),
        
        // Temperature alert
        AutomationRule::new(
            "temp_alert",
            "Alert if temperature too high",
            AutomationTrigger::NumericState {
                entity_id: "sensor.temperature".into(),
                above: Some(30.0),
                below: None,
            },
            AutomationAction::Notify {
                message: "Temperature is above 30°C!".into(),
                title: Some("Temperature Alert".into()),
            },
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_automation_creation() {
        let rule = AutomationRule::new(
            "test",
            "Test Rule",
            AutomationTrigger::Time { time: "12:00".into() },
            AutomationAction::ActivateScene { scene_id: "test".into() },
        );
        
        assert!(rule.enabled);
    }
    
    #[test]
    fn test_engine() {
        let engine = AutomationEngine::new();
        assert!(!engine.get_rules().is_empty());
    }
}
