// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - ReAct Pattern (Reason + Act)
// ═══════════════════════════════════════════════════════════════════════════════
//  Interleaves reasoning and action:
//  1. Thought: Reason about the current state
//  2. Action: Choose and execute a tool
//  3. Observation: See the result
//  4. Repeat until answer found
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use crate::{
    Action, ReasoningStep, ReasoningTrace, PatternType,
    traits::{AgentContext, ReasoningPattern},
    Result, PatternError,
};

/// ReAct pattern implementation
pub struct ReActPattern {
    max_iterations: usize,
}

impl ReActPattern {
    pub fn new() -> Self {
        Self { max_iterations: 10 }
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    fn parse_action(&self, response: &str) -> Option<Action> {
        // Parse "Action: tool_name[input]" format
        if let Some(action_start) = response.find("Action:") {
            let action_part = &response[action_start + 7..];
            
            if let Some(bracket_start) = action_part.find('[') {
                if let Some(bracket_end) = action_part.find(']') {
                    let tool = action_part[..bracket_start].trim().to_string();
                    let input_str = &action_part[bracket_start + 1..bracket_end];
                    
                    // Try to parse as JSON, otherwise use as string
                    let input = if input_str.starts_with('{') || input_str.starts_with('[') {
                        serde_json::from_str(input_str).unwrap_or(serde_json::json!(input_str))
                    } else {
                        serde_json::json!(input_str)
                    };

                    return Some(Action::new(tool, input));
                }
            }
        }
        None
    }

    fn parse_thought(&self, response: &str) -> Option<String> {
        if let Some(thought_start) = response.find("Thought:") {
            let thought_part = &response[thought_start + 8..];
            let thought = if let Some(action_start) = thought_part.find("Action:") {
                thought_part[..action_start].trim().to_string()
            } else if let Some(answer_start) = thought_part.find("Answer:") {
                thought_part[..answer_start].trim().to_string()
            } else {
                thought_part.trim().to_string()
            };
            Some(thought)
        } else {
            None
        }
    }

    fn parse_answer(&self, response: &str) -> Option<String> {
        if let Some(answer_start) = response.find("Answer:") {
            let answer = response[answer_start + 7..].trim().to_string();
            Some(answer)
        } else {
            None
        }
    }

    fn build_prompt(&self, query: &str, trace: &ReasoningTrace) -> String {
        let mut prompt = format!(
            "Answer the following question using the ReAct pattern.\n\
            Available tools: search, calculate, lookup\n\n\
            Use this format:\n\
            Thought: your reasoning\n\
            Action: tool_name[input]\n\n\
            After receiving an observation:\n\
            Thought: reasoning about observation\n\
            ... (continue until you know the answer)\n\
            Answer: final answer\n\n\
            Question: {}\n\n",
            query
        );

        for step in &trace.steps {
            prompt.push_str(&format!("Thought: {}\n", step.thought));
            if let Some(action) = &step.action {
                prompt.push_str(&format!("Action: {}[{}]\n", action.tool, action.input));
            }
            if let Some(obs) = &step.observation {
                prompt.push_str(&format!("Observation: {}\n", obs));
            }
        }

        prompt.push_str("Thought:");
        prompt
    }
}

impl Default for ReActPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReasoningPattern for ReActPattern {
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace> {
        let mut trace = ReasoningTrace::new(PatternType::ReAct);

        for i in 0..self.max_iterations {
            let prompt = self.build_prompt(query, &trace);
            let response = context.call_llm(&prompt).await?;

            // Parse thought
            if let Some(thought) = self.parse_thought(&response) {
                let mut step = ReasoningStep::thought(i + 1, thought);

                // Parse action
                if let Some(action) = self.parse_action(&response) {
                    step = step.with_action(action.clone());
                    
                    // Execute action
                    let observation = context.execute_tool(&action).await?;
                    step = step.with_observation(&observation);
                }

                // Check for answer
                if let Some(answer) = self.parse_answer(&response) {
                    step = step.final_answer();
                    trace.add_step(step);
                    trace.set_answer(answer);
                    return Ok(trace);
                }

                trace.add_step(step);
            }

            trace.add_tokens(response.len() / 4); // Rough estimate
        }

        Err(PatternError::MaxIterationsExceeded(self.max_iterations))
    }

    fn name(&self) -> &'static str {
        "ReAct"
    }

    fn description(&self) -> &'static str {
        "Reason + Act: Interleaves thinking and tool use"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react_creation() {
        let pattern = ReActPattern::new().with_max_iterations(5);
        assert_eq!(pattern.max_iterations, 5);
    }

    #[test]
    fn test_parse_action() {
        let pattern = ReActPattern::new();
        let response = "Thought: I need to search\nAction: search[Rust programming]";
        
        let action = pattern.parse_action(response);
        assert!(action.is_some());
        let action = action.unwrap();
        assert_eq!(action.tool, "search");
    }

    #[test]
    fn test_parse_thought() {
        let pattern = ReActPattern::new();
        let response = "Thought: I should search for this\nAction: search[test]";
        
        let thought = pattern.parse_thought(response);
        assert_eq!(thought, Some("I should search for this".to_string()));
    }

    #[test]
    fn test_parse_answer() {
        let pattern = ReActPattern::new();
        let response = "Answer: The answer is 42";
        
        let answer = pattern.parse_answer(response);
        assert_eq!(answer, Some("The answer is 42".to_string()));
    }
}
