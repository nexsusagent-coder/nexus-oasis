//! ─── Preference Learning ───

use crate::{InteractionEvent, EventType, LearningResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Preference learner
pub struct PreferenceLearner {
    preferences: UserPreferences,
    feedback_scores: HashMap<String, f64>,
}

impl PreferenceLearner {
    pub fn new() -> Self {
        Self {
            preferences: UserPreferences::default(),
            feedback_scores: HashMap::new(),
        }
    }
    
    pub fn update_from_event(&mut self, event: &InteractionEvent) {
        match &event.event_type {
            EventType::PreferenceChange => {
                if let Ok(change) = serde_json::from_value::<PreferenceChange>(event.metadata.clone()) {
                    self.apply_change(&change);
                }
            }
            EventType::Feedback => {
                if let Ok(feedback) = serde_json::from_value::<Feedback>(event.metadata.clone()) {
                    self.process_feedback(&feedback);
                }
            }
            _ => {}
        }
    }
    
    fn apply_change(&mut self, change: &PreferenceChange) {
        match change.category.as_str() {
            "language" => self.preferences.language = change.value.clone(),
            "response_length" => {
                self.preferences.response_length = match change.value.as_str() {
                    "short" => ResponseLength::Short,
                    "medium" => ResponseLength::Medium,
                    "detailed" => ResponseLength::Detailed,
                    _ => ResponseLength::Medium,
                };
            }
            "tone" => {
                self.preferences.tone = match change.value.as_str() {
                    "formal" => Tone::Formal,
                    "casual" => Tone::Casual,
                    "friendly" => Tone::Friendly,
                    _ => Tone::Friendly,
                };
            }
            "theme" => self.preferences.theme = change.value.clone(),
            _ => {}
        }
    }
    
    fn process_feedback(&mut self, feedback: &Feedback) {
        // Update preference confidence based on feedback
        let key = &feedback.target;
        let current = self.feedback_scores.get(key).copied().unwrap_or(0.5);
        
        let new_score = if feedback.positive {
            (current + 0.1).min(1.0)
        } else {
            (current - 0.1).max(0.0)
        };
        
        self.feedback_scores.insert(key.clone(), new_score);
    }
    
    pub fn get_preferences(&self) -> &UserPreferences {
        &self.preferences
    }
    
    pub fn get_confidence(&self, preference: &str) -> f64 {
        self.feedback_scores.get(preference).copied().unwrap_or(0.5)
    }
}

impl Default for PreferenceLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String,
    pub response_length: ResponseLength,
    pub tone: Tone,
    pub theme: String,
    pub notifications_enabled: bool,
    pub voice_speed: f32,
    pub custom_prompts: HashMap<String, String>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            language: "tr".to_string(),
            response_length: ResponseLength::Medium,
            tone: Tone::Friendly,
            theme: "dark".to_string(),
            notifications_enabled: true,
            voice_speed: 1.0,
            custom_prompts: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ResponseLength {
    Short,
    Medium,
    Detailed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Tone {
    Formal,
    Casual,
    Friendly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PreferenceChange {
    category: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Feedback {
    target: String,
    positive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preference_learner() {
        let learner = PreferenceLearner::new();
        let prefs = learner.get_preferences();
        assert_eq!(prefs.language, "tr");
    }
}
