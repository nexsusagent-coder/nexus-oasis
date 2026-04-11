// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Plan-and-Execute Pattern
// ═══════════════════════════════════════════════════════════════════════════════
//  Two-phase approach:
//  1. Planning: Decompose goal into steps
//  2. Execution: Execute steps sequentially
//  - Better for complex, multi-step tasks
//  - Allows for re-planning on failure
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use crate::{
    ReasoningStep, ReasoningTrace, Plan, PlanStep, StepStatus, PatternType,
    traits::{AgentContext, ReasoningPattern},
    Result, PatternError,
};

/// Plan-and-Execute pattern implementation
pub struct PlanAndExecutePattern {
    allow_replanning: bool,
    max_replans: usize,
}

impl PlanAndExecutePattern {
    pub fn new() -> Self {
        Self {
            allow_replanning: true,
            max_replans: 2,
        }
    }

    pub fn without_replanning(mut self) -> Self {
        self.allow_replanning = false;
        self
    }

    pub fn with_max_replans(mut self, max: usize) -> Self {
        self.max_replans = max;
        self
    }

    async fn create_plan(
        &self,
        goal: &str,
        context: &dyn AgentContext,
    ) -> Result<Plan> {
        let prompt = format!(
            "Create a step-by-step plan to achieve this goal:\n\n\
            Goal: {}\n\n\
            Break it down into specific, actionable steps.\n\
            Each step should be achievable with the available tools.\n\
            Format each step as:\n\
            1. [step description]\n\
            2. [step description]\n\
            etc.",
            goal
        );

        let response = context.call_llm(&prompt).await?;

        let mut steps = Vec::new();
        for (i, line) in response.lines().enumerate() {
            let line = line.trim();
            // Match numbered steps
            if line.starts_with(|c: char| c.is_numeric()) {
                let desc = line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == ' ');
                if !desc.is_empty() {
                    steps.push(PlanStep::new(i + 1, desc));
                }
            }
        }

        Ok(Plan::new(steps))
    }

    async fn execute_step(
        &self,
        step: &mut PlanStep,
        context: &dyn AgentContext,
        previous_results: &[String],
    ) -> Result<String> {
        step.status = StepStatus::InProgress;

        let prev_context = previous_results.join("\n");
        let prompt = format!(
            "Execute this step:\n\n\
            Step: {}\n\n\
            Previous results:\n{}\n\n\
            Describe what action to take and the result.",
            step.description, prev_context
        );

        let result = context.call_llm(&prompt).await?;
        step.status = StepStatus::Completed;

        Ok(result)
    }

    async fn replan(
        &self,
        goal: &str,
        failed_step: &PlanStep,
        reason: &str,
        context: &dyn AgentContext,
    ) -> Result<Plan> {
        let prompt = format!(
            "The plan to achieve '{}' failed at step '{}': {}\n\n\
            Create a revised plan that addresses this failure.\n\
            Format each step as:\n\
            1. [step description]\n\
            2. [step description]\n\
            etc.",
            goal, failed_step.description, reason
        );

        let response = context.call_llm(&prompt).await?;

        let mut steps = Vec::new();
        for (i, line) in response.lines().enumerate() {
            let line = line.trim();
            if line.starts_with(|c: char| c.is_numeric()) {
                let desc = line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == ' ');
                if !desc.is_empty() {
                    steps.push(PlanStep::new(i + 1, desc));
                }
            }
        }

        Ok(Plan::new(steps))
    }
}

impl Default for PlanAndExecutePattern {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReasoningPattern for PlanAndExecutePattern {
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace> {
        let mut trace = ReasoningTrace::new(PatternType::PlanAndExecute);
        let mut replan_count = 0;

        loop {
            // Create plan
            let mut plan = self.create_plan(query, context).await?;
            
            trace.add_step(ReasoningStep::thought(0, format!(
                "Created plan with {} steps", plan.steps.len()
            )));

            // Execute each step
            let mut results = Vec::new();
            for step in &mut plan.steps {
                match self.execute_step(step, context, &results).await {
                    Ok(result) => {
                        results.push(result.clone());
                        trace.add_step(ReasoningStep::thought(step.id, format!(
                            "Executed: {} -> {}", step.description, result
                        )));
                    }
                    Err(e) => {
                        step.status = StepStatus::Failed;
                        
                        if self.allow_replanning && replan_count < self.max_replans {
                            replan_count += 1;
                            trace.add_step(ReasoningStep::thought(99, format!(
                                "Step failed, replanning ({}/{})", replan_count, self.max_replans
                            )));
                            continue;
                        } else {
                            return Err(PatternError::PlanExecutionFailed {
                                step: step.id,
                                reason: e.to_string(),
                            });
                        }
                    }
                }
            }

            // All steps completed
            let answer = results.last().cloned().unwrap_or_default();
            trace.set_answer(answer);
            return Ok(trace);
        }
    }

    fn name(&self) -> &'static str {
        "Plan-and-Execute"
    }

    fn description(&self) -> &'static str {
        "First plan, then execute each step"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_execute_creation() {
        let pattern = PlanAndExecutePattern::new()
            .without_replanning()
            .with_max_replans(1);

        assert!(!pattern.allow_replanning);
    }

    #[test]
    fn test_plan_step_status() {
        let mut step = PlanStep::new(1, "Test step");
        assert_eq!(step.status, StepStatus::Pending);

        step.status = StepStatus::InProgress;
        assert_eq!(step.status, StepStatus::InProgress);
    }
}
