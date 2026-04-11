// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Pattern Traits
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use crate::{Action, ReasoningTrace, Plan, Result};

/// Reasoning pattern trait
#[async_trait]
pub trait ReasoningPattern: Send + Sync {
    /// Run reasoning on a query
    async fn reason(&self, query: &str, context: &dyn AgentContext) -> Result<ReasoningTrace>;

    /// Get pattern name
    fn name(&self) -> &'static str;

    /// Get pattern description
    fn description(&self) -> &'static str;
}

/// Agent context trait
#[async_trait]
pub trait AgentContext: Send + Sync {
    /// Call LLM with prompt
    async fn call_llm(&self, prompt: &str) -> Result<String>;

    /// Execute a tool/action
    async fn execute_tool(&self, action: &Action) -> Result<String>;

    /// Get available tools
    fn available_tools(&self) -> Vec<&str>;

    /// Check if tool is available
    fn has_tool(&self, name: &str) -> bool;
}

/// Tool executor trait
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Execute a tool
    async fn execute(&self, tool: &str, input: &serde_json::Value) -> Result<String>;

    /// Get tool names
    fn tool_names(&self) -> Vec<&str>;

    /// Get tool description
    fn tool_description(&self, name: &str) -> Option<&str>;
}

/// Planner trait
#[async_trait]
pub trait Planner: Send + Sync {
    /// Create a plan for a goal
    async fn plan(&self, goal: &str, context: &dyn AgentContext) -> Result<Plan>;

    /// Revise plan based on results
    async fn revise(&self, plan: &Plan, feedback: &str) -> Result<Plan>;
}

/// Reflector trait
#[async_trait]
pub trait Reflector: Send + Sync {
    /// Reflect on a result
    async fn reflect(&self, query: &str, result: &str) -> Result<ReflectionResult>;
}

/// Reflection result
#[derive(Debug, Clone)]
pub struct ReflectionResult {
    pub is_correct: bool,
    pub confidence: f32,
    pub issues: Vec<String>,
    pub improvements: Vec<String>,
}

impl ReflectionResult {
    pub fn correct() -> Self {
        Self {
            is_correct: true,
            confidence: 1.0,
            issues: Vec::new(),
            improvements: Vec::new(),
        }
    }

    pub fn with_issues(issues: Vec<String>) -> Self {
        Self {
            is_correct: false,
            confidence: 0.5,
            issues,
            improvements: Vec::new(),
        }
    }
}
