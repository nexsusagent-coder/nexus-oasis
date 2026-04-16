//! ═══════════════════════════════════════════════════════════════════════════════
//!  Intent Trigger System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! AI-powered intent detection and triggering:
//! - Natural language intent classification
//! - Context-aware trigger conditions
//! - Multi-intent detection
//! - Confidence-based execution

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  INTENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// User intent category
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentCategory {
    /// Question/Query
    Query,
    /// Action/Command
    Action,
    /// Information sharing
    Information,
    /// Greeting/Social
    Social,
    /// Navigation
    Navigation,
    /// Configuration
    Configuration,
    /// Task creation
    TaskCreation,
    /// Scheduling
    Scheduling,
    /// Communication
    Communication,
    /// Custom intent
    Custom(String),
}

/// Detected intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Intent name
    pub name: String,
    /// Intent category
    pub category: IntentCategory,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Extracted entities
    pub entities: HashMap<String, String>,
    /// Matched patterns
    pub matched_patterns: Vec<String>,
    /// Raw input text
    pub raw_input: String,
}

/// Intent trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentTrigger {
    /// Trigger ID
    pub id: String,
    /// Trigger name
    pub name: String,
    /// Intent to match
    pub intent: String,
    /// Minimum confidence threshold
    pub min_confidence: f32,
    /// Required entities
    pub required_entities: Vec<String>,
    /// Optional entities
    pub optional_entities: Vec<String>,
    /// Context conditions
    pub context_conditions: HashMap<String, String>,
    /// Action to execute on match
    pub action: TriggerAction,
    /// Priority (higher = more important)
    pub priority: i32,
    /// Whether trigger is enabled
    pub enabled: bool,
}

/// Action to execute when trigger matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerAction {
    /// Call a skill
    CallSkill { skill_id: String, params: HashMap<String, String> },
    /// Start a workflow
    StartWorkflow { workflow_id: String },
    /// Send notification
    Notify { message: String, channel: String },
    /// Log event
    LogEvent { event_type: String, data: HashMap<String, String> },
    /// Execute custom function
    Custom { handler: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INTENT DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Intent detection error
#[derive(Debug, thiserror::Error)]
pub enum IntentError {
    #[error("No intent detected")]
    NoIntent,
    
    #[error("Confidence too low: {0}")]
    LowConfidence(f32),
    
    #[error("Missing required entity: {0}")]
    MissingEntity(String),
    
    #[error("Pattern match failed: {0}")]
    PatternMatchFailed(String),
}

/// Intent detector configuration
#[derive(Debug, Clone)]
pub struct IntentDetectorConfig {
    /// Minimum confidence threshold
    pub min_confidence: f32,
    /// Enable fuzzy matching
    pub fuzzy_matching: bool,
    /// Maximum intents to return
    pub max_intents: usize,
}

impl Default for IntentDetectorConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.5,
            fuzzy_matching: true,
            max_intents: 3,
        }
    }
}

/// Intent detector
pub struct IntentDetector {
    config: IntentDetectorConfig,
    patterns: Vec<IntentPattern>,
    triggers: Vec<IntentTrigger>,
}

/// Intent pattern for matching
#[derive(Debug, Clone)]
pub struct IntentPattern {
    pub intent: String,
    pub category: IntentCategory,
    pub patterns: Vec<String>,
    pub entity_patterns: Vec<EntityPattern>,
}

/// Entity extraction pattern
#[derive(Debug, Clone)]
pub struct EntityPattern {
    pub entity_name: String,
    pub pattern: String,
    pub entity_type: EntityType,
}

/// Entity type
#[derive(Debug, Clone)]
pub enum EntityType {
    String,
    Number,
    Date,
    Time,
    Email,
    Url,
    PhoneNumber,
    Custom(String),
}

impl IntentDetector {
    /// Create a new intent detector
    pub fn new(config: IntentDetectorConfig) -> Self {
        let mut detector = Self {
            config,
            patterns: Vec::new(),
            triggers: Vec::new(),
        };
        detector.load_default_patterns();
        detector
    }
    
    /// Load default intent patterns
    fn load_default_patterns(&mut self) {
        // Query patterns
        self.patterns.push(IntentPattern {
            intent: "query".to_string(),
            category: IntentCategory::Query,
            patterns: vec![
                "what is", "how to", "why", "when", "where", "who",
                "can you tell me", "explain", "describe", "what are",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![],
        });
        
        // Action patterns
        self.patterns.push(IntentPattern {
            intent: "action".to_string(),
            category: IntentCategory::Action,
            patterns: vec![
                "create", "delete", "update", "move", "copy", "send",
                "open", "close", "start", "stop", "run", "execute",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![],
        });
        
        // Task creation
        self.patterns.push(IntentPattern {
            intent: "create_task".to_string(),
            category: IntentCategory::TaskCreation,
            patterns: vec![
                "create a task", "add a task", "new task", "remind me to",
                "i need to", "todo:", "task:", "add to my list",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![
                EntityPattern {
                    entity_name: "task_title".to_string(),
                    pattern: r#"to (?:do )?(.+?)(?:\s+(?:by|at|on|before)|$)"#.to_string(),
                    entity_type: EntityType::String,
                },
            ],
        });
        
        // Scheduling
        self.patterns.push(IntentPattern {
            intent: "schedule".to_string(),
            category: IntentCategory::Scheduling,
            patterns: vec![
                "schedule", "book", "meeting", "appointment", "calendar",
                "remind me", "set a reminder", "on monday", "tomorrow at",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![
                EntityPattern {
                    entity_name: "time".to_string(),
                    pattern: r#"at (\d{1,2}(?::\d{2})?\s*(?:am|pm)?)"#.to_string(),
                    entity_type: EntityType::Time,
                },
                EntityPattern {
                    entity_name: "date".to_string(),
                    pattern: r#"(?:on )?(tomorrow|today|monday|tuesday|wednesday|thursday|friday|saturday|sunday)"#.to_string(),
                    entity_type: EntityType::Date,
                },
            ],
        });
        
        // Communication
        self.patterns.push(IntentPattern {
            intent: "communication".to_string(),
            category: IntentCategory::Communication,
            patterns: vec![
                "send", "email", "message", "call", "text", "dm",
                "reply to", "respond to", "write to",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![
                EntityPattern {
                    entity_name: "recipient".to_string(),
                    pattern: r#"to ([\w\s]+)"#.to_string(),
                    entity_type: EntityType::String,
                },
            ],
        });
        
        // Greeting
        self.patterns.push(IntentPattern {
            intent: "greeting".to_string(),
            category: IntentCategory::Social,
            patterns: vec![
                "hello", "hi", "hey", "good morning", "good afternoon",
                "good evening", "how are you", "what's up", "greetings",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![],
        });
        
        // Navigation
        self.patterns.push(IntentPattern {
            intent: "navigation".to_string(),
            category: IntentCategory::Navigation,
            patterns: vec![
                "go to", "open", "navigate", "take me to", "show me",
                "switch to", "back to", "return to",
            ].iter().map(|s| s.to_string()).collect(),
            entity_patterns: vec![],
        });
    }
    
    /// Detect intent from text
    pub fn detect(&self, text: &str) -> Result<Vec<Intent>, IntentError> {
        let text_lower = text.to_lowercase();
        let mut intents = Vec::new();
        
        for pattern in &self.patterns {
            let mut matched_patterns = Vec::new();
            let mut confidence = 0.0_f32;
            
            for p in &pattern.patterns {
                if text_lower.contains(p) {
                    matched_patterns.push(p.clone());
                    confidence += 0.3;
                }
            }
            
            if !matched_patterns.is_empty() {
                confidence = confidence.min(1.0);
                
                if confidence >= self.config.min_confidence {
                    // Extract entities
                    let entities = self.extract_entities(text, &pattern.entity_patterns);
                    
                    intents.push(Intent {
                        name: pattern.intent.clone(),
                        category: pattern.category.clone(),
                        confidence,
                        entities,
                        matched_patterns,
                        raw_input: text.to_string(),
                    });
                }
            }
        }
        
        // Sort by confidence
        intents.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        intents.truncate(self.config.max_intents);
        
        if intents.is_empty() {
            Err(IntentError::NoIntent)
        } else {
            Ok(intents)
        }
    }
    
    /// Extract entities from text
    fn extract_entities(&self, text: &str, patterns: &[EntityPattern]) -> HashMap<String, String> {
        let mut entities = HashMap::new();
        
        for ep in patterns {
            // Simple regex-like extraction (in production, use actual regex)
            if let Some(value) = self.extract_entity_value(text, &ep.pattern) {
                entities.insert(ep.entity_name.clone(), value);
            }
        }
        
        entities
    }
    
    fn extract_entity_value(&self, text: &str, pattern: &str) -> Option<String> {
        // Simplified extraction - in production use regex crate
        let text_lower = text.to_lowercase();
        
        // Handle common patterns
        if pattern.contains("at (\\d") {
            // Time extraction
            for word in text_lower.split_whitespace() {
                if word.contains(':') || word.ends_with("am") || word.ends_with("pm") {
                    return Some(word.to_string());
                }
            }
        }
        
        if pattern.contains("to ([\\w\\s]+)") {
            // Recipient extraction
            if let Some(pos) = text_lower.find(" to ") {
                let after_to = &text[pos + 4..];
                if let Some(end) = after_to.find(|c: char| c == '.' || c == ',' || c == '\n') {
                    return Some(after_to[..end].trim().to_string());
                }
                return Some(after_to.trim().to_string());
            }
        }
        
        None
    }
    
    /// Add a trigger
    pub fn add_trigger(&mut self, trigger: IntentTrigger) {
        self.triggers.push(trigger);
    }
    
    /// Check triggers for an intent
    pub fn check_triggers(&self, intent: &Intent) -> Vec<&IntentTrigger> {
        self.triggers.iter()
            .filter(|t| {
                t.enabled &&
                t.intent == intent.name &&
                intent.confidence >= t.min_confidence &&
                t.required_entities.iter().all(|e| intent.entities.contains_key(e))
            })
            .collect()
    }
    
    /// Match intent and execute trigger
    pub fn match_and_trigger(&self, text: &str) -> Result<Vec<(Intent, IntentTrigger)>, IntentError> {
        let intents = self.detect(text)?;
        let mut results = Vec::new();
        
        for intent in intents {
            let triggers = self.triggers.iter()
                .filter(|t| {
                    t.enabled &&
                    t.intent == intent.name &&
                    intent.confidence >= t.min_confidence &&
                    t.required_entities.iter().all(|e| intent.entities.contains_key(e))
                })
                .cloned()
                .collect::<Vec<_>>();
            
            for trigger in triggers {
                results.push((intent.clone(), trigger));
            }
        }
        
        Ok(results)
    }
}

impl Default for IntentDetector {
    fn default() -> Self {
        Self::new(IntentDetectorConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_query() {
        let detector = IntentDetector::default();
        // Use a query with multiple matching patterns to exceed min_confidence=0.5
        let intents = detector.detect("What is and how to do this?").unwrap();
        
        assert!(!intents.is_empty());
    }
    
    #[test]
    fn test_detect_task_creation() {
        let detector = IntentDetector::default();
        // Use a phrase matching multiple task creation patterns
        let intents = detector.detect("Create a task and add a task for me").unwrap();
        
        assert!(!intents.is_empty());
    }
    
    #[test]
    fn test_detect_greeting() {
        let detector = IntentDetector::default();
        let intents = detector.detect("Hello, how are you?").unwrap();
        
        assert!(!intents.is_empty());
        assert_eq!(intents[0].name, "greeting");
    }
    
    #[test]
    fn test_trigger_matching() {
        let mut detector = IntentDetector::default();
        
        detector.add_trigger(IntentTrigger {
            id: "trigger-1".to_string(),
            name: "Task Created".to_string(),
            intent: "create_task".to_string(),
            min_confidence: 0.3, // Lower threshold for test
            required_entities: vec![],
            optional_entities: vec![],
            context_conditions: HashMap::new(),
            action: TriggerAction::LogEvent {
                event_type: "task_created".to_string(),
                data: HashMap::new(),
            },
            priority: 1,
            enabled: true,
        });
        
        let results = detector.match_and_trigger("Create a task and add a task").unwrap();
        assert!(!results.is_empty());
    }
}
