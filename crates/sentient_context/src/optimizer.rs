//! ─── Context Optimizer ───
//!
//! Optimize context for better AI responses:
//! - Remove redundancy
//! - Compress similar sections
//! - Smart truncation

use crate::{ContextSection, ContextConfig};
use serde::{Deserialize, Serialize};

/// Context optimizer
pub struct ContextOptimizer {
    config: ContextConfig,
}

impl ContextOptimizer {
    pub fn new() -> Self {
        Self {
            config: ContextConfig::default(),
        }
    }
    
    pub fn with_config(mut self, config: ContextConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Optimize sections
    pub fn optimize(&self, sections: Vec<ContextSection>) -> Vec<ContextSection> {
        let mut optimized = sections;
        
        // Remove duplicates
        optimized = self.remove_duplicates(optimized);
        
        // Merge similar sections
        optimized = self.merge_similar(optimized);
        
        // Apply priority ordering
        optimized.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        optimized
    }
    
    /// Remove duplicate sections
    fn remove_duplicates(&self, sections: Vec<ContextSection>) -> Vec<ContextSection> {
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        
        for section in sections {
            let key = format!("{}:{}", section.name, section.content);
            if !seen.contains(&key) {
                seen.insert(key);
                result.push(section);
            }
        }
        
        result
    }
    
    /// Merge similar sections
    fn merge_similar(&self, sections: Vec<ContextSection>) -> Vec<ContextSection> {
        let mut merged: Vec<ContextSection> = Vec::new();
        
        for section in sections {
            // Check if there's a similar section already merged
            if let Some(existing) = merged.iter_mut().find(|s| s.name == section.name) {
                // Append content
                if !existing.content.contains(&section.content) {
                    existing.content.push_str("\n\n");
                    existing.content.push_str(&section.content);
                    existing.token_count += section.token_count;
                }
            } else {
                merged.push(section);
            }
        }
        
        merged
    }
    
    /// Calculate optimization score
    pub fn calculate_score(&self, original: &[ContextSection], optimized: &[ContextSection]) -> OptimizationScore {
        let original_tokens: u32 = original.iter().map(|s| s.token_count).sum();
        let optimized_tokens: u32 = optimized.iter().map(|s| s.token_count).sum();
        
        let reduction = if original_tokens > 0 {
            (1.0 - (optimized_tokens as f64 / original_tokens as f64)) * 100.0
        } else {
            0.0
        };
        
        OptimizationScore {
            original_tokens,
            optimized_tokens,
            reduction_percent: reduction,
            original_sections: original.len(),
            optimized_sections: optimized.len(),
        }
    }
}

impl Default for ContextOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimization score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationScore {
    pub original_tokens: u32,
    pub optimized_tokens: u32,
    pub reduction_percent: f64,
    pub original_sections: usize,
    pub optimized_sections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimizer() {
        let optimizer = ContextOptimizer::new();
        let sections = vec![
            ContextSection::new("Test", "Content"),
            ContextSection::new("Test", "Content"), // Duplicate
        ];
        
        let optimized = optimizer.optimize(sections);
        assert_eq!(optimized.len(), 1);
    }
}
