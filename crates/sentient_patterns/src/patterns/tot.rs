// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Tree of Thoughts (ToT) Pattern
// ═══════════════════════════════════════════════════════════════════════════════
//  Explores multiple reasoning paths:
//  - Generates multiple thought branches
//  - Evaluates each path
//  - Backtracks from dead ends
//  - Finds optimal solution
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{
    ReasoningStep, ReasoningTrace, PatternType,
    traits::{AgentContext, ReasoningPattern},
    Result,
};

/// Tree node representing a thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtNode {
    pub id: usize,
    pub thought: String,
    pub evaluation: Option<f32>,
    pub children: Vec<ThoughtNode>,
    pub is_solution: bool,
}

impl ThoughtNode {
    pub fn new(id: usize, thought: impl Into<String>) -> Self {
        Self {
            id,
            thought: thought.into(),
            evaluation: None,
            children: Vec::new(),
            is_solution: false,
        }
    }

    pub fn with_evaluation(mut self, score: f32) -> Self {
        self.evaluation = Some(score);
        self
    }

    pub fn mark_solution(mut self) -> Self {
        self.is_solution = true;
        self
    }

    pub fn add_child(&mut self, child: ThoughtNode) {
        self.children.push(child);
    }
}

/// Tree of Thoughts pattern implementation
pub struct TreeOfThoughtsPattern {
    branching_factor: usize,
    max_depth: usize,
    evaluation_threshold: f32,
}

impl TreeOfThoughtsPattern {
    pub fn new() -> Self {
        Self {
            branching_factor: 3,
            max_depth: 4,
            evaluation_threshold: 0.7,
        }
    }

    pub fn with_branching(mut self, factor: usize) -> Self {
        self.branching_factor = factor;
        self
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.evaluation_threshold = threshold;
        self
    }

    async fn generate_thoughts(
        &self,
        query: &str,
        context: &dyn AgentContext,
        path: &[String],
        n: usize,
    ) -> Result<Vec<String>> {
        let path_str = path.iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {}", i + 1, t))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "Given the problem: '{}'\n\n\
            Current reasoning path:\n{}\n\n\
            Generate {} different possible next thoughts to continue reasoning.\n\
            Each thought should be a single step forward.\n\
            Format each thought on a new line starting with '- '.",
            query, path_str, n
        );

        let response = context.call_llm(&prompt).await?;
        
        let thoughts: Vec<String> = response.lines()
            .filter(|l| l.trim().starts_with('-'))
            .map(|l| l.trim().trim_start_matches('-').trim().to_string())
            .filter(|l| !l.is_empty())
            .take(n)
            .collect();

        Ok(thoughts)
    }

    async fn evaluate_thought(
        &self,
        query: &str,
        thought: &str,
        context: &dyn AgentContext,
    ) -> Result<f32> {
        let prompt = format!(
            "Problem: '{}'\n\n\
            Proposed thought: '{}'\n\n\
            Evaluate how promising this thought is for solving the problem.\n\
            Rate from 0.0 (useless/wrong) to 1.0 (exactly right).\n\
            Reply with just a number between 0 and 1.",
            query, thought
        );

        let response = context.call_llm(&prompt).await?;
        
        // Parse the score
        let score = response.trim()
            .parse::<f32>()
            .unwrap_or(0.5)
            .clamp(0.0, 1.0);

        Ok(score)
    }

    async fn search_tree(
        &self,
        query: &str,
        context: &dyn AgentContext,
        path: &mut Vec<String>,
        depth: usize,
        trace: &mut ReasoningTrace,
    ) -> Result<Option<String>> {
        Box::pin(self.search_tree_inner(query, context, path, depth, trace)).await
    }

    async fn search_tree_inner(
        &self,
        query: &str,
        context: &dyn AgentContext,
        path: &mut Vec<String>,
        depth: usize,
        trace: &mut ReasoningTrace,
    ) -> Result<Option<String>> {
        if depth >= self.max_depth {
            return Ok(None);
        }

        // Generate candidate thoughts
        let thoughts = self.generate_thoughts(query, context, path, self.branching_factor).await?;

        // Evaluate each thought
        let mut scored_thoughts = Vec::new();
        for thought in thoughts {
            let score = self.evaluate_thought(query, &thought, context).await?;
            if score >= self.evaluation_threshold {
                scored_thoughts.push((thought, score));
            }
        }

        // Sort by score descending
        scored_thoughts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Try each promising thought
        for (thought, score) in scored_thoughts {
            path.push(thought.clone());
            
            let step = ReasoningStep::thought(path.len(), format!("{} (score: {:.2})", thought, score));
            trace.add_step(step);

            // Check if this is a solution
            if score > 0.95 {
                return Ok(Some(thought));
            }

            // Recurse
            if let Some(solution) = self.search_tree(query, context, path, depth + 1, trace).await? {
                return Ok(Some(solution));
            }

            // Backtrack
            path.pop();
        }

        Ok(None)
    }
}

impl Default for TreeOfThoughtsPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReasoningPattern for TreeOfThoughtsPattern {
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace> {
        let mut trace = ReasoningTrace::new(PatternType::TreeOfThoughts);
        let mut path = Vec::new();

        let result = self.search_tree(query, context, &mut path, 0, &mut trace).await?;

        if let Some(solution) = result {
            trace.set_answer(solution);
        }

        Ok(trace)
    }

    fn name(&self) -> &'static str {
        "Tree of Thoughts"
    }

    fn description(&self) -> &'static str {
        "Explores multiple reasoning paths, evaluates, backtracks"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tot_creation() {
        let pattern = TreeOfThoughtsPattern::new()
            .with_branching(5)
            .with_max_depth(3)
            .with_threshold(0.8);

        assert_eq!(pattern.branching_factor, 5);
        assert_eq!(pattern.max_depth, 3);
    }

    #[test]
    fn test_thought_node() {
        let node = ThoughtNode::new(1, "First thought")
            .with_evaluation(0.8);

        assert_eq!(node.id, 1);
        assert_eq!(node.evaluation, Some(0.8));
        assert!(!node.is_solution);
    }
}
