// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Self-Reflection Pattern
// ═══════════════════════════════════════════════════════════════════════════════
//  Self-critique and improvement:
//  1. Generate initial response
//  2. Critique the response
//  3. Identify issues
//  4. Generate improved response
//  - Improves quality through iteration
//  - Catches errors before output
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{
    ReasoningStep, ReasoningTrace, PatternType,
    traits::{AgentContext, ReasoningPattern},
    Result,
};

/// Reflection result from self-critique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critique {
    /// Is the answer correct/satisfactory?
    pub is_satisfactory: bool,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Identified issues
    pub issues: Vec<String>,
    /// Suggested improvements
    pub improvements: Vec<String>,
}

impl Critique {
    pub fn satisfactory() -> Self {
        Self {
            is_satisfactory: true,
            confidence: 1.0,
            issues: Vec::new(),
            improvements: Vec::new(),
        }
    }

    pub fn with_issues(issues: Vec<String>) -> Self {
        Self {
            is_satisfactory: false,
            confidence: 0.5,
            issues,
            improvements: Vec::new(),
        }
    }
}

/// Self-Reflection pattern implementation
pub struct SelfReflectionPattern {
    max_iterations: usize,
    confidence_threshold: f32,
}

impl SelfReflectionPattern {
    pub fn new() -> Self {
        Self {
            max_iterations: 3,
            confidence_threshold: 0.8,
        }
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    async fn generate_initial(&self, query: &str, context: &dyn AgentContext) -> Result<String> {
        let prompt = format!(
            "Answer the following question:\n\n\
            Question: {}\n\n\
            Provide a clear and accurate answer.",
            query
        );

        context.call_llm(&prompt).await
    }

    async fn critique(
        &self,
        query: &str,
        answer: &str,
        context: &dyn AgentContext,
    ) -> Result<Critique> {
        let prompt = format!(
            "Critique this answer:\n\n\
            Question: {}\n\n\
            Answer: {}\n\n\
            Evaluate the answer for:\n\
            1. Accuracy - Is it factually correct?\n\
            2. Completeness - Does it fully answer the question?\n\
            3. Clarity - Is it easy to understand?\n\n\
            Respond in JSON format:\n\
            {{\n\
              \"is_satisfactory\": true/false,\n\
              \"confidence\": 0.0-1.0,\n\
              \"issues\": [\"issue1\", \"issue2\", ...],\n\
              \"improvements\": [\"improvement1\", \"improvement2\", ...]\n\
            }}",
            query, answer
        );

        let response = context.call_llm(&prompt).await?;

        // Try to parse as JSON
        if let Ok(critique) = serde_json::from_str::<Critique>(&response) {
            return Ok(critique);
        }

        // Fallback: parse from text
        let is_satisfactory = response.to_lowercase().contains("satisfactory") 
            || response.to_lowercase().contains("correct");
        
        Ok(Critique {
            is_satisfactory,
            confidence: if is_satisfactory { 0.8 } else { 0.4 },
            issues: Vec::new(),
            improvements: Vec::new(),
        })
    }

    async fn improve(
        &self,
        query: &str,
        answer: &str,
        critique: &Critique,
        context: &dyn AgentContext,
    ) -> Result<String> {
        let issues = critique.issues.join(", ");
        let improvements = critique.improvements.join(", ");

        let prompt = format!(
            "Improve this answer based on the critique:\n\n\
            Question: {}\n\n\
            Current answer: {}\n\n\
            Issues: {}\n\
            Suggested improvements: {}\n\n\
            Provide an improved answer that addresses these issues.",
            query, answer, issues, improvements
        );

        context.call_llm(&prompt).await
    }
}

impl Default for SelfReflectionPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReasoningPattern for SelfReflectionPattern {
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace> {
        let mut trace = ReasoningTrace::new(PatternType::SelfReflection);

        // Generate initial answer
        let mut current_answer = self.generate_initial(query, context).await?;
        trace.add_step(ReasoningStep::thought(1, format!("Initial answer: {}", 
            if current_answer.len() > 100 { 
                format!("{}...", &current_answer[..100]) 
            } else { 
                current_answer.clone() 
            }
        )));

        for i in 0..self.max_iterations {
            // Critique
            let critique = self.critique(query, &current_answer, context).await?;
            
            trace.add_step(ReasoningStep::thought(i * 2 + 2, format!(
                "Critique {}: confidence={:.2}, issues={}", 
                i + 1, critique.confidence, critique.issues.len()
            )));

            // Check if satisfactory
            if critique.is_satisfactory && critique.confidence >= self.confidence_threshold {
                trace.add_step(ReasoningStep::thought(i * 2 + 3, "Answer accepted after reflection"));
                trace.set_answer(current_answer);
                return Ok(trace);
            }

            // Improve
            let improved = self.improve(query, &current_answer, &critique, context).await?;
            trace.add_step(ReasoningStep::thought(i * 2 + 4, format!(
                "Improved answer (iteration {})", i + 1
            )));

            current_answer = improved;
        }

        // Return final answer even if not fully confident
        trace.set_answer(current_answer);
        Ok(trace)
    }

    fn name(&self) -> &'static str {
        "Self-Reflection"
    }

    fn description(&self) -> &'static str {
        "Generate, critique, and improve iteratively"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection_creation() {
        let pattern = SelfReflectionPattern::new()
            .with_max_iterations(5)
            .with_confidence_threshold(0.9);

        assert_eq!(pattern.max_iterations, 5);
    }

    #[test]
    fn test_critique_satisfactory() {
        let critique = Critique::satisfactory();
        assert!(critique.is_satisfactory);
        assert_eq!(critique.confidence, 1.0);
    }

    #[test]
    fn test_critique_with_issues() {
        let critique = Critique::with_issues(vec![
            "Missing detail".to_string(),
            "Unclear explanation".to_string(),
        ]);

        assert!(!critique.is_satisfactory);
        assert_eq!(critique.issues.len(), 2);
    }
}
