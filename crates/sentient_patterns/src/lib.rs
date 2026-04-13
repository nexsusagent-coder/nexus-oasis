// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Agentic Patterns
// ═══════════════════════════════════════════════════════════════════════════════
//  Reasoning and decision-making patterns for AI agents
//  - ReAct (Reason + Act)
//  - Chain of Thought (CoT)
//  - Tree of Thoughts (ToT)
//  - Plan-and-Execute
//  - Self-Reflection
// ═══════════════════════════════════════════════════════════════════════════════

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod patterns;
pub mod error;
pub mod traits;

pub use patterns::{react, cot, tot, plan_execute, reflection};
pub use error::{PatternError, Result};
pub use traits::{ReasoningPattern, AgentContext, ToolExecutor};

use serde::{Deserialize, Serialize};

/// Reasoning step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number
    pub step: usize,
    /// Thought/reasoning
    pub thought: String,
    /// Action to take (optional)
    pub action: Option<Action>,
    /// Observation from action (optional)
    pub observation: Option<String>,
    /// Is this the final answer?
    pub is_final: bool,
}

impl ReasoningStep {
    pub fn thought(step: usize, thought: impl Into<String>) -> Self {
        Self {
            step,
            thought: thought.into(),
            action: None,
            observation: None,
            is_final: false,
        }
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_observation(mut self, obs: impl Into<String>) -> Self {
        self.observation = Some(obs.into());
        self
    }

    pub fn final_answer(mut self) -> Self {
        self.is_final = true;
        self
    }
}

/// Action to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Tool name
    pub tool: String,
    /// Tool input
    pub input: serde_json::Value,
}

impl Action {
    pub fn new(tool: impl Into<String>, input: serde_json::Value) -> Self {
        Self {
            tool: tool.into(),
            input,
        }
    }

    pub fn search(query: impl Into<String>) -> Self {
        Self::new("search", serde_json::json!({ "query": query.into() }))
    }

    pub fn calculate(expression: impl Into<String>) -> Self {
        Self::new("calculate", serde_json::json!({ "expression": expression.into() }))
    }

    pub fn lookup(key: impl Into<String>) -> Self {
        Self::new("lookup", serde_json::json!({ "key": key.into() }))
    }
}

/// Reasoning trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTrace {
    /// Pattern used
    pub pattern: PatternType,
    /// All steps
    pub steps: Vec<ReasoningStep>,
    /// Final answer
    pub answer: Option<String>,
    /// Total tokens used (approximate)
    pub tokens_used: usize,
}

impl ReasoningTrace {
    pub fn new(pattern: PatternType) -> Self {
        Self {
            pattern,
            steps: Vec::new(),
            answer: None,
            tokens_used: 0,
        }
    }

    pub fn add_step(&mut self, step: ReasoningStep) {
        self.steps.push(step);
    }

    pub fn set_answer(&mut self, answer: impl Into<String>) {
        self.answer = Some(answer.into());
    }

    pub fn add_tokens(&mut self, tokens: usize) {
        self.tokens_used += tokens;
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Pattern type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    #[serde(rename = "react")]
    ReAct,
    #[serde(rename = "cot")]
    ChainOfThought,
    #[serde(rename = "tot")]
    TreeOfThoughts,
    #[serde(rename = "plan_execute")]
    PlanAndExecute,
    #[serde(rename = "reflection")]
    SelfReflection,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReAct => write!(f, "ReAct"),
            Self::ChainOfThought => write!(f, "Chain of Thought"),
            Self::TreeOfThoughts => write!(f, "Tree of Thoughts"),
            Self::PlanAndExecute => write!(f, "Plan-and-Execute"),
            Self::SelfReflection => write!(f, "Self-Reflection"),
        }
    }
}

/// Planning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Plan steps
    pub steps: Vec<PlanStep>,
    /// Estimated complexity
    pub complexity: Complexity,
}

impl Plan {
    pub fn new(steps: Vec<PlanStep>) -> Self {
        let complexity = Self::estimate_complexity(&steps);
        Self { steps, complexity }
    }

    fn estimate_complexity(steps: &[PlanStep]) -> Complexity {
        match steps.len() {
            0..=2 => Complexity::Simple,
            3..=5 => Complexity::Medium,
            _ => Complexity::Complex,
        }
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// Plan step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: usize,
    pub description: String,
    pub dependencies: Vec<usize>,
    pub status: StepStatus,
}

impl PlanStep {
    pub fn new(id: usize, description: impl Into<String>) -> Self {
        Self {
            id,
            description: description.into(),
            dependencies: Vec::new(),
            status: StepStatus::Pending,
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<usize>) -> Self {
        self.dependencies = deps;
        self
    }
}

/// Step status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Complexity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Complexity {
    Simple,
    Medium,
    Complex,
}

// Re-export for convenience
pub mod prelude {
    pub use crate::{PatternType, ReasoningStep, ReasoningTrace, Action, Plan, PlanStep};
    pub use crate::traits::{ReasoningPattern, AgentContext};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_step() {
        let step = ReasoningStep::thought(1, "I need to search for information")
            .with_action(Action::search("Rust programming"));

        assert_eq!(step.step, 1);
        assert!(step.action.is_some());
    }

    #[test]
    fn test_action_creation() {
        let action = Action::search("test query");
        assert_eq!(action.tool, "search");
    }

    #[test]
    fn test_reasoning_trace() {
        let mut trace = ReasoningTrace::new(PatternType::ReAct);
        trace.add_step(ReasoningStep::thought(1, "First thought"));
        trace.set_answer("Final answer");

        assert_eq!(trace.step_count(), 1);
        assert_eq!(trace.answer, Some("Final answer".to_string()));
    }

    #[test]
    fn test_plan() {
        let plan = Plan::new(vec![
            PlanStep::new(1, "Step 1"),
            PlanStep::new(2, "Step 2"),
        ]);

        assert_eq!(plan.step_count(), 2);
        assert_eq!(plan.complexity, Complexity::Simple);
    }
}
