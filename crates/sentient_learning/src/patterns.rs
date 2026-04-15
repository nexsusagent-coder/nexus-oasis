//! ─── Pattern Recognition ───

use crate::{InteractionEvent, EventType, LearningResult};
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pattern recognizer
pub struct PatternRecognizer {
    patterns: Vec<DetectedPattern>,
    sequences: Vec<Vec<String>>,
    min_occurrences: u32,
}

impl PatternRecognizer {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            sequences: Vec::new(),
            min_occurrences: 3,
        }
    }
    
    pub fn analyze_events(&mut self, events: &[InteractionEvent]) -> Vec<DetectedPattern> {
        if events.len() < self.min_occurrences as usize {
            return Vec::new();
        }
        
        // Build sequence of event types
        let sequence: Vec<String> = events
            .iter()
            .map(|e| format!("{:?}", e.event_type))
            .collect();
        
        self.sequences.push(sequence);
        
        // Detect patterns
        self.detect_sequential_patterns();
        self.detect_time_patterns(events);
        self.detect_frequency_patterns(events);
        
        self.patterns.clone()
    }
    
    fn detect_sequential_patterns(&mut self) {
        let mut pattern_counts: HashMap<String, u32> = HashMap::new();
        
        for sequence in &self.sequences {
            // Look for 2-3 event patterns
            for window in 2..=3 {
                for i in 0..sequence.len().saturating_sub(window - 1) {
                    let pattern = sequence[i..i + window].join(" -> ");
                    *pattern_counts.entry(pattern).or_insert(0) += 1;
                }
            }
        }
        
        // Add detected patterns
        for (pattern, count) in pattern_counts {
            if count >= self.min_occurrences {
                if !self.patterns.iter().any(|p| p.pattern == pattern) {
                    self.patterns.push(DetectedPattern {
                        pattern,
                        pattern_type: PatternType::Sequential,
                        occurrences: count,
                        confidence: (count as f32 / 10.0).min(1.0),
                    });
                }
            }
        }
    }
    
    fn detect_time_patterns(&mut self, events: &[InteractionEvent]) {
        let mut hour_counts: HashMap<u8, u32> = HashMap::new();
        let mut day_counts: HashMap<u8, u32> = HashMap::new();
        
        for event in events {
            *hour_counts.entry(event.timestamp.hour() as u8).or_insert(0) += 1;
            *day_counts.entry(event.timestamp.weekday().number_from_monday() as u8).or_insert(0) += 1;
        }
        
        // Peak hours
        if let Some((&hour, &count)) = hour_counts.iter().max_by_key(|(_, c)| *c) {
            if count >= self.min_occurrences {
                self.patterns.push(DetectedPattern {
                    pattern: format!("Active at {}:00", hour),
                    pattern_type: PatternType::TimeBased,
                    occurrences: count,
                    confidence: (count as f32 / 20.0).min(1.0),
                });
            }
        }
        
        // Peak days
        if let Some((&day, &count)) = day_counts.iter().max_by_key(|(_, c)| *c) {
            let day_name = match day {
                1 => "Monday",
                2 => "Tuesday",
                3 => "Wednesday",
                4 => "Thursday",
                5 => "Friday",
                6 => "Saturday",
                7 => "Sunday",
                _ => "Unknown",
            };
            
            if count >= self.min_occurrences {
                self.patterns.push(DetectedPattern {
                    pattern: format!("Most active on {}", day_name),
                    pattern_type: PatternType::TimeBased,
                    occurrences: count,
                    confidence: (count as f32 / 10.0).min(1.0),
                });
            }
        }
    }
    
    fn detect_frequency_patterns(&mut self, events: &[InteractionEvent]) {
        let mut type_counts: HashMap<String, u32> = HashMap::new();
        
        for event in events {
            let type_str = format!("{:?}", event.event_type);
            *type_counts.entry(type_str).or_insert(0) += 1;
        }
        
        for (event_type, count) in type_counts {
            if count >= self.min_occurrences {
                self.patterns.push(DetectedPattern {
                    pattern: format!("Frequent: {}", event_type),
                    pattern_type: PatternType::Frequency,
                    occurrences: count,
                    confidence: (count as f32 / events.len() as f32).min(1.0),
                });
            }
        }
    }
    
    pub fn get_patterns(&self) -> &[DetectedPattern] {
        &self.patterns
    }
    
    pub fn get_top_patterns(&self, n: usize) -> Vec<&DetectedPattern> {
        let mut patterns: Vec<_> = self.patterns.iter().collect();
        patterns.sort_by(|a, b| b.occurrences.cmp(&a.occurrences));
        patterns.into_iter().take(n).collect()
    }
}

impl Default for PatternRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPattern {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub occurrences: u32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PatternType {
    Sequential,
    TimeBased,
    Frequency,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_recognizer() {
        let mut recognizer = PatternRecognizer::new();
        let events = vec![
            InteractionEvent::new("user1", EventType::Message),
            InteractionEvent::new("user1", EventType::Search),
            InteractionEvent::new("user1", EventType::Message),
            InteractionEvent::new("user1", EventType::Search),
            InteractionEvent::new("user1", EventType::Message),
        ];
        
        let patterns = recognizer.analyze_events(&events);
        assert!(!patterns.is_empty());
    }
}
