//! ─── REASONING ENGINE ───
//!
//! Core reasoning capabilities powered by Gemma 4

use crate::{BrainConfig, BrainError, KERNEL_MODEL};
use sentient_local::LocalEngine;
use serde::{Deserialize, Serialize};
use log::info;

/// Reasoning Engine
pub struct ReasoningEngine {
    config: BrainConfig,
}

impl ReasoningEngine {
    pub fn new(config: BrainConfig) -> Self {
        Self { config }
    }
    
    /// Execute reasoning process
    pub async fn reason(&self, input: &str, gemma4: &LocalEngine) -> Result<ReasoningResult, BrainError> {
        let mut steps = Vec::new();
        let mut current_thought = input.to_string();
        
        for step_num in 0..self.config.max_reasoning_steps {
            // Create thinking prompt
            let prompt = if self.config.thinking_mode {
                let prev_thoughts: Vec<&str> = steps.iter().map(|s: &ThinkingStep| s.thought.as_str()).collect();
                format!(
                    "Step {} of reasoning. Think about: {}\n\nPrevious thoughts: {}\n\nProvide your next reasoning step and conclusion if ready.",
                    step_num + 1,
                    input,
                    prev_thoughts.join("\n")
                )
            } else {
                format!("Analyze: {}", current_thought)
            };
            
            // Generate with Gemma 4
            let response = gemma4.generate(&prompt).await
                .map_err(|e| BrainError::Gemma4(e.to_string()))?;
            
            let step = ThinkingStep {
                step_number: step_num + 1,
                thought: response.clone(),
                is_conclusion: response.contains("therefore") || response.contains("conclusion") || step_num >= self.config.max_reasoning_steps - 1,
            };
            
            let is_conclusion = step.is_conclusion;
            steps.push(step);
            
            if is_conclusion {
                break;
            }
            
            current_thought = response;
        }
        
        // Generate final conclusion
        let conclusion = steps.iter()
            .filter(|s| s.is_conclusion)
            .last()
            .map(|s| s.thought.clone())
            .unwrap_or_else(|| steps.last().map(|s| s.thought.clone()).unwrap_or_default());
        
        Ok(ReasoningResult {
            input: input.to_string(),
            steps,
            conclusion,
            model: KERNEL_MODEL.to_string(),
        })
    }
}

/// Thinking step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingStep {
    pub step_number: u32,
    pub thought: String,
    pub is_conclusion: bool,
}

/// Reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub input: String,
    pub steps: Vec<ThinkingStep>,
    pub conclusion: String,
    pub model: String,
}

impl ReasoningResult {
    /// Get all thoughts as single string
    pub fn full_reasoning(&self) -> String {
        self.steps.iter()
            .map(|s| format!("Step {}: {}", s.step_number, s.thought))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
