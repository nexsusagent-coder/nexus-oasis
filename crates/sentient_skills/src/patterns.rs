//! ─── Pattern Detection ───

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::watcher::UserAction;

/// Detected action pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPattern {
    pub id: String,
    pub name: String,
    pub actions: Vec<UserAction>,
    pub occurrences: u32,
    pub confidence: f64,
    pub average_duration_ms: u64,
    pub first_seen: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

impl ActionPattern {
    pub fn new(actions: Vec<UserAction>) -> Self {
        let total_duration: u64 = actions.iter().map(|a| a.duration_ms).sum();
        let avg_duration = if actions.is_empty() { 0 } else { total_duration / actions.len() as u64 };
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Unnamed Pattern".into(),
            actions,
            occurrences: 1,
            confidence: 0.5,
            average_duration_ms: avg_duration,
            first_seen: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
        }
    }
    
    pub fn update_occurrence(&mut self) {
        self.occurrences += 1;
        self.last_seen = chrono::Utc::now();
        self.confidence = (0.5 + 0.1 * (self.occurrences - 1) as f64).min(0.95);
    }
}

/// Pattern detector
pub struct PatternDetector {
    patterns: HashMap<String, ActionPattern>,
    min_sequence_length: usize,
    max_sequence_length: usize,
    similarity_threshold: f64,
}

impl PatternDetector {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            min_sequence_length: 2,
            max_sequence_length: 20,
            similarity_threshold: 0.8,
        }
    }
    
    /// Detect patterns in action buffer
    pub fn detect(&self, actions: &[UserAction]) -> Vec<ActionPattern> {
        if actions.len() < self.min_sequence_length * 2 {
            return vec![];
        }
        
        let mut detected: Vec<ActionPattern> = vec![];
        
        // Find repeating sequences
        for length in self.min_sequence_length..=self.max_sequence_length.min(actions.len() / 2) {
            for start in 0..actions.len() - length {
                let sequence = &actions[start..start + length];
                
                // Look for repetitions
                for check_start in (start + length)..actions.len() - length + 1 {
                    let check_sequence = &actions[check_start..check_start + length];
                    
                    if self.compare_sequences(sequence, check_sequence) >= self.similarity_threshold {
                        let pattern_key = self.sequence_key(sequence);
                        
                        if let Some(existing) = detected.iter_mut().find(|p| self.sequence_key(&p.actions) == pattern_key) {
                            existing.update_occurrence();
                        } else {
                            let mut pattern = ActionPattern::new(sequence.to_vec());
                            pattern.occurrences = 2;
                            pattern.confidence = 0.6;
                            pattern.name = self.generate_pattern_name(sequence);
                            detected.push(pattern);
                        }
                        break;
                    }
                }
            }
        }
        
        // Sort by confidence
        detected.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        detected
    }
    
    fn compare_sequences(&self, a: &[UserAction], b: &[UserAction]) -> f64 {
        if a.len() != b.len() { return 0.0; }
        let matches = a.iter().zip(b.iter()).filter(|(x, y)| self.actions_match(x, y)).count();
        matches as f64 / a.len() as f64
    }
    
    fn actions_match(&self, a: &UserAction, b: &UserAction) -> bool {
        std::mem::discriminant(&a.action_type) == std::mem::discriminant(&b.action_type)
    }
    
    fn sequence_key(&self, sequence: &[UserAction]) -> String {
        sequence.iter().map(|a| format!("{:?}", std::mem::discriminant(&a.action_type))).collect::<Vec<_>>().join("|")
    }
    
    fn generate_pattern_name(&self, sequence: &[UserAction]) -> String {
        if sequence.is_empty() { return "Empty Pattern".into(); }
        format!("{}-Action Pattern", sequence.len())
    }
    
    pub fn add_pattern(&mut self, pattern: ActionPattern) {
        let key = self.sequence_key(&pattern.actions);
        self.patterns.insert(key, pattern);
    }
    
    pub fn get_patterns(&self) -> Vec<&ActionPattern> {
        self.patterns.values().collect()
    }
}

impl Default for PatternDetector {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_creation() {
        let actions = vec![UserAction::new(crate::watcher::UserActionType::KeyboardShortcut("Ctrl+C".into()))];
        let pattern = ActionPattern::new(actions);
        assert_eq!(pattern.actions.len(), 1);
    }
}
