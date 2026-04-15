//! ─── Pattern Matching System ───
//!
//! Detects patterns for proactive behavior

use serde::{Deserialize, Serialize};

/// Pattern matcher for detecting behavioral patterns
pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

/// A pattern definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern ID
    pub id: String,
    
    /// Pattern name
    pub name: String,
    
    /// Pattern expression
    pub expression: String,
    
    /// Minimum occurrences to trigger
    pub min_occurrences: u32,
    
    /// Time window in seconds
    pub time_window_seconds: u64,
    
    /// Actions to take when matched
    pub actions: Vec<String>,
}

/// Result of pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternResult {
    /// Pattern that matched
    pub pattern_id: String,
    
    /// Match confidence
    pub confidence: f64,
    
    /// Occurrences found
    pub occurrences: u32,
    
    /// Time window analyzed
    pub window_seconds: u64,
    
    /// Recommended action
    pub recommended_action: String,
}

impl PatternMatcher {
    /// Create new pattern matcher
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }
    
    /// Add a pattern
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }
    
    /// Check for pattern matches in event history
    pub fn check_patterns(&self, events: &[crate::event::Event]) -> Vec<PatternResult> {
        let mut results = Vec::new();
        
        for pattern in &self.patterns {
            let matches = self.count_matches(pattern, events);
            
            if matches >= pattern.min_occurrences {
                results.push(PatternResult {
                    pattern_id: pattern.id.clone(),
                    confidence: (matches as f64 / pattern.min_occurrences as f64).min(1.0),
                    occurrences: matches,
                    window_seconds: pattern.time_window_seconds,
                    recommended_action: pattern.actions.first().cloned().unwrap_or_default(),
                });
            }
        }
        
        results
    }
    
    /// Count matches for a pattern
    fn count_matches(&self, pattern: &Pattern, events: &[crate::event::Event]) -> u32 {
        let now = chrono::Utc::now();
        let window_start = now - chrono::Duration::seconds(pattern.time_window_seconds as i64);
        
        events.iter()
            .filter(|e| e.timestamp > window_start)
            .filter(|e| self.matches_expression(pattern, e))
            .count() as u32
    }
    
    /// Check if event matches pattern expression
    fn matches_expression(&self, pattern: &Pattern, event: &crate::event::Event) -> bool {
        // Simple matching - could be enhanced with a proper DSL
        event.event_type.to_string().contains(&pattern.expression) ||
        event.payload.to_string().contains(&pattern.expression)
    }
    
    /// Get all patterns
    pub fn get_patterns(&self) -> &[Pattern] {
        &self.patterns
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_creation() {
        let pattern = Pattern {
            id: "friday-report".into(),
            name: "Friday Report Pattern".into(),
            expression: "calendar".into(),
            min_occurrences: 1,
            time_window_seconds: 86400,
            actions: vec!["generate_weekly_report".into()],
        };
        
        assert_eq!(pattern.id, "friday-report");
    }
}
