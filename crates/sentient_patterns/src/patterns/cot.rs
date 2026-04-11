// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Chain of Thought (CoT) Pattern
// ═══════════════════════════════════════════════════════════════════════════════
//  Step-by-step reasoning:
//  "Let's think step by step..."
//  - Decomposes complex problems
//  - Shows intermediate reasoning
//  - Improves accuracy on reasoning tasks
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use crate::{
    ReasoningStep, ReasoningTrace, PatternType,
    traits::{AgentContext, ReasoningPattern},
    Result,
};

/// Chain of Thought pattern implementation
pub struct ChainOfThoughtPattern {
    include_examples: bool,
    max_steps: usize,
}

impl ChainOfThoughtPattern {
    pub fn new() -> Self {
        Self {
            include_examples: true,
            max_steps: 10,
        }
    }

    pub fn without_examples(mut self) -> Self {
        self.include_examples = false;
        self
    }

    pub fn with_max_steps(mut self, max: usize) -> Self {
        self.max_steps = max;
        self
    }

    fn build_prompt(&self, query: &str) -> String {
        let mut prompt = String::new();

        if self.include_examples {
            prompt.push_str(
                "Here are some examples of step-by-step reasoning:\n\n\
                Example 1:\n\
                Question: What is 15 + 27?\n\
                Let's think step by step:\n\
                Step 1: We need to add 15 and 27\n\
                Step 2: 15 + 20 = 35\n\
                Step 3: 35 + 7 = 42\n\
                Answer: 42\n\n\
                Example 2:\n\
                Question: Is 17 a prime number?\n\
                Let's think step by step:\n\
                Step 1: A prime number is only divisible by 1 and itself\n\
                Step 2: 17 is not divisible by 2 (17/2 = 8.5)\n\
                Step 3: 17 is not divisible by 3 (17/3 = 5.67)\n\
                Step 4: 17 is not divisible by 4, 5, or any number up to √17 ≈ 4.12\n\
                Answer: Yes, 17 is a prime number\n\n"
            );
        }

        prompt.push_str(&format!(
            "Now answer this question using step-by-step reasoning:\n\n\
            Question: {}\n\n\
            Let's think step by step:",
            query
        ));

        prompt
    }

    fn parse_steps(&self, response: &str) -> Vec<ReasoningStep> {
        let mut steps = Vec::new();
        
        for line in response.lines() {
            let line = line.trim();
            
            // Match "Step N: ..." format
            if line.starts_with("Step ") {
                // Find the colon and get content after it
                if let Some(colon_pos) = line.find(':') {
                    let content = line[colon_pos + 1..].trim();
                    steps.push(ReasoningStep::thought(steps.len() + 1, content));
                }
            }
        }

        // If no explicit steps found, treat each line as a step
        if steps.is_empty() {
            for (i, line) in response.lines().enumerate() {
                let line = line.trim();
                if !line.is_empty() && !line.starts_with("Answer:") {
                    steps.push(ReasoningStep::thought(i + 1, line));
                }
            }
        }

        steps
    }

    fn parse_answer(&self, response: &str) -> Option<String> {
        if let Some(answer_start) = response.find("Answer:") {
            let answer = response[answer_start + 7..].trim().to_string();
            Some(answer)
        } else {
            // Take the last non-empty line as answer
            response.lines()
                .filter(|l| !l.trim().is_empty())
                .last()
                .map(|l| l.trim().to_string())
        }
    }
}

impl Default for ChainOfThoughtPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReasoningPattern for ChainOfThoughtPattern {
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace> {
        let mut trace = ReasoningTrace::new(PatternType::ChainOfThought);

        let prompt = self.build_prompt(query);
        let response = context.call_llm(&prompt).await?;

        // Parse steps
        let steps = self.parse_steps(&response);
        for step in steps {
            trace.add_step(step);
        }

        // Parse answer
        if let Some(answer) = self.parse_answer(&response) {
            trace.set_answer(answer);
        }

        trace.add_tokens(response.len() / 4);

        Ok(trace)
    }

    fn name(&self) -> &'static str {
        "Chain of Thought"
    }

    fn description(&self) -> &'static str {
        "Step-by-step reasoning: 'Let's think step by step...'"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cot_creation() {
        let pattern = ChainOfThoughtPattern::new()
            .without_examples()
            .with_max_steps(5);
        
        assert!(!pattern.include_examples);
        assert_eq!(pattern.max_steps, 5);
    }

    #[test]
    fn test_parse_steps() {
        let pattern = ChainOfThoughtPattern::new();
        let response = "Step 1: First thing\nStep 2: Second thing\nAnswer: 42";
        
        let steps = pattern.parse_steps(response);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].thought, "First thing");
    }

    #[test]
    fn test_parse_answer() {
        let pattern = ChainOfThoughtPattern::new();
        let response = "Step 1: Something\nAnswer: The answer is 42";
        
        let answer = pattern.parse_answer(response);
        assert_eq!(answer, Some("The answer is 42".to_string()));
    }
}
