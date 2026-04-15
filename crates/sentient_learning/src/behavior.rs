//! ─── User Behavior Analysis ───

use crate::{InteractionEvent, EventType, LearningResult};
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Behavior analyzer
pub struct BehaviorAnalyzer {
    events: Vec<InteractionEvent>,
}

impl BehaviorAnalyzer {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    pub fn record(&mut self, event: InteractionEvent) {
        self.events.push(event);
    }
    
    pub fn analyze(&self) -> BehaviorProfile {
        let mut profile = BehaviorProfile::default();
        
        if self.events.is_empty() {
            return profile;
        }
        
        // Analyze peak activity hours
        let mut hour_counts: HashMap<u8, u32> = HashMap::new();
        for event in &self.events {
            let hour = event.timestamp.hour() as u8;
            *hour_counts.entry(hour).or_insert(0) += 1;
        }
        
        if let Some((peak_hour, _)) = hour_counts.iter().max_by_key(|(_, c)| *c) {
            profile.peak_activity_hour = Some(*peak_hour);
        }
        
        // Analyze preferred interaction types
        let mut type_counts: HashMap<String, u32> = HashMap::new();
        for event in &self.events {
            let type_str = match &event.event_type {
                EventType::Message => "message",
                EventType::VoiceCommand => "voice",
                EventType::FileOpen => "file_open",
                EventType::FileEdit => "file_edit",
                EventType::Search => "search",
                EventType::TaskComplete => "task",
                EventType::PreferenceChange => "pref_change",
                EventType::Feedback => "feedback",
            };
            *type_counts.entry(type_str.to_string()).or_insert(0) += 1;
        }
        
        if let Some((preferred, _)) = type_counts.iter().max_by_key(|(_, c)| *c) {
            profile.preferred_interaction = Some(preferred.clone());
        }
        
        // Calculate average session length (simplified)
        profile.average_session_minutes = 30; // Default estimate
        
        profile.total_interactions = self.events.len() as u32;
        profile.last_activity = self.events.last().map(|e| e.timestamp);
        
        profile
    }
}

impl Default for BehaviorAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Behavior profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub peak_activity_hour: Option<u8>,
    pub preferred_interaction: Option<String>,
    pub average_session_minutes: u32,
    pub total_interactions: u32,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
    pub common_tasks: Vec<String>,
    pub frequent_files: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_behavior_analyzer() {
        let mut analyzer = BehaviorAnalyzer::new();
        analyzer.record(InteractionEvent::new("user1", EventType::Message));
        let profile = analyzer.analyze();
        assert_eq!(profile.total_interactions, 1);
    }
}
