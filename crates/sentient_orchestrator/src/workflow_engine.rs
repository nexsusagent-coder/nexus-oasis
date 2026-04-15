//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Workflow Engine - DAG-based Task Orchestration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Visual workflow builder & execution engine:
//!  - DAG (Directed Acyclic Graph) execution
//!  - Parallel task execution
//!  - Conditional branching
//!  - Loop & retry support
//!  - Human-in-the-loop approval
//!  - Workflow templates

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
//  WORKFLOW DEFINITION
// ═══════════════════════════════════════════════════════════════════════════════

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Description
    pub description: String,
    /// Version
    pub version: String,
    /// Nodes (tasks/steps)
    pub nodes: HashMap<String, WorkflowNode>,
    /// Edges (connections between nodes)
    pub edges: Vec<WorkflowEdge>,
    /// Input schema
    pub input_schema: Option<serde_json::Value>,
    /// Output schema
    pub output_schema: Option<serde_json::Value>,
    /// Variables
    pub variables: HashMap<String, serde_json::Value>,
    /// Timeout in seconds
    pub timeout_secs: u64,
    /// Retry policy
    pub retry_policy: RetryPolicy,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            version: "1.0.0".into(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            input_schema: None,
            output_schema: None,
            variables: HashMap::new(),
            timeout_secs: 3600,
            retry_policy: RetryPolicy::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    /// Add a node to the workflow
    pub fn add_node(&mut self, node: WorkflowNode) {
        self.nodes.insert(node.id.clone(), node);
        self.updated_at = Utc::now();
    }
    
    /// Add an edge between nodes
    pub fn add_edge(&mut self, from: String, to: String, condition: Option<Condition>) {
        self.edges.push(WorkflowEdge {
            id: Uuid::new_v4().to_string(),
            from_node: from,
            to_node: to,
            condition,
        });
        self.updated_at = Utc::now();
    }
    
    /// Get start nodes (nodes with no incoming edges)
    pub fn get_start_nodes(&self) -> Vec<&WorkflowNode> {
        let incoming: HashSet<&str> = self.edges.iter().map(|e| e.to_node.as_str()).collect();
        self.nodes.values()
            .filter(|n| !incoming.contains(n.id.as_str()))
            .collect()
    }
    
    /// Get successors of a node
    pub fn get_successors(&self, node_id: &str) -> Vec<&WorkflowNode> {
        self.edges.iter()
            .filter(|e| e.from_node == node_id)
            .filter_map(|e| self.nodes.get(&e.to_node))
            .collect()
    }
    
    /// Validate workflow (check for cycles, orphan nodes, etc.)
    pub fn validate(&self) -> Result<(), WorkflowError> {
        // Check for empty workflow
        if self.nodes.is_empty() {
            return Err(WorkflowError::ValidationFailed("Workflow has no nodes".into()));
        }
        
        // Check for cycles using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node_id in self.nodes.keys() {
            if self.has_cycle(node_id, &mut visited, &mut rec_stack) {
                return Err(WorkflowError::ValidationFailed("Workflow contains a cycle".into()));
            }
        }
        
        // Check for orphan nodes (unreachable)
        let reachable = self.get_reachable_nodes();
        for node_id in self.nodes.keys() {
            if !reachable.contains(node_id) {
                return Err(WorkflowError::ValidationFailed(
                    format!("Node {} is unreachable", node_id)
                ));
            }
        }
        
        Ok(())
    }
    
    fn has_cycle(&self, node_id: &str, visited: &mut HashSet<String>, rec_stack: &mut HashSet<String>) -> bool {
        if rec_stack.contains(node_id) {
            return true;
        }
        if visited.contains(node_id) {
            return false;
        }
        
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        
        for edge in self.edges.iter().filter(|e| e.from_node == node_id) {
            if self.has_cycle(&edge.to_node, visited, rec_stack) {
                return true;
            }
        }
        
        rec_stack.remove(node_id);
        false
    }
    
    fn get_reachable_nodes(&self) -> HashSet<String> {
        let mut reachable = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();
        
        // Start from nodes with no incoming edges
        for node in self.get_start_nodes() {
            queue.push_back(node.id.clone());
        }
        
        while let Some(node_id) = queue.pop_front() {
            if reachable.insert(node_id.clone()) {
                for edge in self.edges.iter().filter(|e| e.from_node == node_id) {
                    queue.push_back(edge.to_node.clone());
                }
            }
        }
        
        reachable
    }
}

/// Workflow node (task/step)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    /// Node ID
    pub id: String,
    /// Node name
    pub name: String,
    /// Node type
    pub node_type: NodeType,
    /// Input transformation (JQ-like expression)
    pub input_transform: Option<String>,
    /// Output transformation
    pub output_transform: Option<String>,
    /// Timeout in seconds
    pub timeout_secs: Option<u64>,
    /// Retry policy override
    pub retry_policy: Option<RetryPolicy>,
    /// Position (for visual editor)
    pub position: Option<(f64, f64)>,
    /// Description
    pub description: Option<String>,
}

impl WorkflowNode {
    pub fn new(id: String, name: String, node_type: NodeType) -> Self {
        Self {
            id,
            name,
            node_type,
            input_transform: None,
            output_transform: None,
            timeout_secs: None,
            retry_policy: None,
            position: None,
            description: None,
        }
    }
    
    pub fn task(name: String, action: TaskAction) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::Task { action },
        )
    }
    
    pub fn parallel(name: String, branches: Vec<String>) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::Parallel { branches },
        )
    }
    
    pub fn condition(name: String, branches: Vec<ConditionalBranch>) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::Condition { branches },
        )
    }
    
    pub fn loop_node(name: String, max_iterations: u32) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::Loop { max_iterations },
        )
    }
    
    pub fn approval(name: String, approvers: Vec<String>, timeout_secs: u64) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::HumanApproval { approvers, timeout_secs },
        )
    }
    
    pub fn subworkflow(name: String, workflow_id: String) -> Self {
        Self::new(
            Uuid::new_v4().to_string(),
            name,
            NodeType::SubWorkflow { workflow_id },
        )
    }
}

/// Node type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// Start node
    Start,
    /// End node
    End,
    /// Task node
    Task { action: TaskAction },
    /// Parallel execution
    Parallel { branches: Vec<String> },
    /// Conditional branching
    Condition { branches: Vec<ConditionalBranch> },
    /// Loop
    Loop { max_iterations: u32 },
    /// Human approval required
    HumanApproval { approvers: Vec<String>, timeout_secs: u64 },
    /// Sub-workflow
    SubWorkflow { workflow_id: String },
    /// Delay
    Delay { seconds: u64 },
    /// Webhook trigger
    Webhook { url: String, method: String },
    /// Email notification
    Email { recipients: Vec<String>, template: String },
    /// Custom action
    Custom { handler: String, config: HashMap<String, serde_json::Value> },
}

/// Task action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAction {
    /// Action type
    pub action_type: String,
    /// Handler function name
    pub handler: String,
    /// Parameters
    pub params: HashMap<String, serde_json::Value>,
    /// Timeout
    pub timeout_secs: Option<u64>,
}

/// Conditional branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalBranch {
    pub name: String,
    pub condition: Condition,
    pub target_node: String,
}

/// Condition expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub expression: String, // Simple expression language
}

impl Condition {
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }
    
    /// Evaluate condition against context
    pub fn evaluate(&self, context: &HashMap<String, serde_json::Value>) -> bool {
        // Simple expression evaluation
        // Supports: $var == "value", $var != "value", $var > 10, etc.
        let expr = self.expression.trim();
        
        // Check for equality
        if let Some((left, right)) = expr.split_once("==") {
            let left = left.trim().strip_prefix('$').unwrap_or(left.trim());
            let right = right.trim().trim_matches('"');
            
            if let Some(value) = context.get(left) {
                return value.as_str().map(|s| s == right).unwrap_or(false);
            }
        }
        
        // Check for inequality
        if let Some((left, right)) = expr.split_once("!=") {
            let left = left.trim().strip_prefix('$').unwrap_or(left.trim());
            let right = right.trim().trim_matches('"');
            
            if let Some(value) = context.get(left) {
                return value.as_str().map(|s| s != right).unwrap_or(true);
            }
        }
        
        // Check for greater than
        if let Some((left, right)) = expr.split_once('>') {
            let left = left.trim().strip_prefix('$').unwrap_or(left.trim());
            let right: f64 = right.trim().parse().unwrap_or(0.0);
            
            if let Some(value) = context.get(left) {
                return value.as_f64().map(|v| v > right).unwrap_or(false);
            }
        }
        
        // Default: check if variable is truthy
        let var = expr.strip_prefix('$').unwrap_or(expr);
        context.get(var).map(|v| !v.is_null()).unwrap_or(false)
    }
}

/// Workflow edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    pub condition: Option<Condition>,
}

/// Retry policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            multiplier: 2.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WORKFLOW EXECUTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Execution status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Paused,
    WaitingApproval,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// Workflow execution instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Execution ID
    pub id: String,
    /// Workflow ID
    pub workflow_id: String,
    /// Execution status
    pub status: ExecutionStatus,
    /// Input data
    pub input: HashMap<String, serde_json::Value>,
    /// Output data
    pub output: HashMap<String, serde_json::Value>,
    /// Current context
    pub context: HashMap<String, serde_json::Value>,
    /// Node execution states
    pub node_states: HashMap<String, NodeExecutionState>,
    /// Completed nodes
    pub completed_nodes: HashSet<String>,
    /// Execution history
    pub history: Vec<ExecutionEvent>,
    /// Started at
    pub started_at: Option<DateTime<Utc>>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Error message
    pub error: Option<String>,
}

impl WorkflowExecution {
    pub fn new(workflow_id: String, input: HashMap<String, serde_json::Value>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            workflow_id,
            status: ExecutionStatus::Pending,
            input: input.clone(),
            output: HashMap::new(),
            context: input,
            node_states: HashMap::new(),
            completed_nodes: HashSet::new(),
            history: Vec::new(),
            started_at: None,
            completed_at: None,
            error: None,
        }
    }
    
    pub fn duration_ms(&self) -> u64 {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => (end - start).num_milliseconds() as u64,
            (Some(start), None) => (Utc::now() - start).num_milliseconds() as u64,
            _ => 0,
        }
    }
}

/// Node execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecutionState {
    pub node_id: String,
    pub status: NodeStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub retry_count: u32,
}

impl NodeExecutionState {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            status: NodeStatus::Pending,
            started_at: None,
            completed_at: None,
            output: None,
            error: None,
            retry_count: 0,
        }
    }
}

/// Node status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Skipped,
    WaitingApproval,
}

/// Execution event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub node_id: Option<String>,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WORKFLOW ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Workflow engine
pub struct WorkflowEngine {
    /// Workflow registry
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    /// Active executions
    executions: Arc<RwLock<HashMap<String, WorkflowExecution>>>,
    /// Task handlers
    handlers: Arc<RwLock<HashMap<String, Box<dyn TaskHandler + Send + Sync>>>>,
    /// Event sender
    events: mpsc::Sender<ExecutionEvent>,
}

impl WorkflowEngine {
    pub fn new(events_tx: mpsc::Sender<ExecutionEvent>) -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
            executions: Arc::new(RwLock::new(HashMap::new())),
            handlers: Arc::new(RwLock::new(HashMap::new())),
            events: events_tx,
        }
    }
    
    /// Register a workflow
    pub async fn register(&self, workflow: Workflow) -> Result<String, WorkflowError> {
        workflow.validate()?;
        
        let id = workflow.id.clone();
        let mut workflows = self.workflows.write().await;
        workflows.insert(id.clone(), workflow);
        
        log::info!("📋 Workflow registered: {}", id);
        Ok(id)
    }
    
    /// Register a task handler
    pub async fn register_handler<H>(&self, name: String, handler: H)
    where
        H: TaskHandler + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().await;
        handlers.insert(name, Box::new(handler));
    }
    
    /// Start workflow execution
    pub async fn execute(
        &self,
        workflow_id: &str,
        input: HashMap<String, serde_json::Value>,
    ) -> Result<String, WorkflowError> {
        let workflows = self.workflows.read().await;
        let workflow = workflows.get(workflow_id)
            .ok_or_else(|| WorkflowError::WorkflowNotFound(workflow_id.to_string()))?;
        
        // Create execution instance
        let mut execution = WorkflowExecution::new(workflow_id.to_string(), input);
        execution.status = ExecutionStatus::Running;
        execution.started_at = Some(Utc::now());
        
        // Initialize node states
        for node_id in workflow.nodes.keys() {
            execution.node_states.insert(node_id.clone(), NodeExecutionState::new(node_id.clone()));
        }
        
        let execution_id = execution.id.clone();
        
        // Store execution
        let mut executions = self.executions.write().await;
        executions.insert(execution_id.clone(), execution);
        
        drop(workflows);
        drop(executions);
        
        // Start execution
        self.run_execution(workflow_id, &execution_id).await?;
        
        Ok(execution_id)
    }
    
    /// Run execution loop
    async fn run_execution(&self, workflow_id: &str, execution_id: &str) -> Result<(), WorkflowError> {
        let workflow = {
            let workflows = self.workflows.read().await;
            workflows.get(workflow_id).cloned()
                .ok_or_else(|| WorkflowError::WorkflowNotFound(workflow_id.to_string()))?
        };
        
        // Get ready nodes (nodes whose dependencies are satisfied)
        let ready_nodes = self.get_ready_nodes(&workflow, execution_id).await?;
        
        for node_id in ready_nodes {
            Box::pin(self.execute_node(&workflow, execution_id, &node_id)).await?;
        }
        
        Ok(())
    }
    
    /// Get nodes ready for execution
    async fn get_ready_nodes(&self, workflow: &Workflow, execution_id: &str) -> Result<Vec<String>, WorkflowError> {
        let executions = self.executions.read().await;
        let execution = executions.get(execution_id)
            .ok_or_else(|| WorkflowError::ExecutionNotFound(execution_id.to_string()))?;
        
        let mut ready = Vec::new();
        
        for (node_id, node_state) in &execution.node_states {
            if node_state.status != NodeStatus::Pending {
                continue;
            }
            
            // Check if all dependencies are completed
            let dependencies: Vec<&str> = workflow.edges.iter()
                .filter(|e| e.to_node == *node_id)
                .map(|e| e.from_node.as_str())
                .collect();
            
            let all_deps_complete = dependencies.iter()
                .all(|dep| execution.completed_nodes.contains(*dep));
            
            if all_deps_complete || dependencies.is_empty() {
                // Check edge conditions
                let conditions_met = workflow.edges.iter()
                    .filter(|e| e.to_node == *node_id)
                    .all(|e| {
                        e.condition.as_ref()
                            .map(|c| c.evaluate(&execution.context))
                            .unwrap_or(true)
                    });
                
                if conditions_met {
                    ready.push(node_id.clone());
                }
            }
        }
        
        Ok(ready)
    }
    
    /// Execute a single node
    async fn execute_node(&self, workflow: &Workflow, execution_id: &str, node_id: &str) -> Result<(), WorkflowError> {
        let node = workflow.nodes.get(node_id)
            .ok_or_else(|| WorkflowError::NodeNotFound(node_id.to_string()))?;
        
        // Update node state to running
        {
            let mut executions = self.executions.write().await;
            if let Some(execution) = executions.get_mut(execution_id) {
                if let Some(state) = execution.node_states.get_mut(node_id) {
                    state.status = NodeStatus::Running;
                    state.started_at = Some(Utc::now());
                }
            }
        }
        
        log::info!("▶️ Executing node: {} ({})", node.name, node_id);
        
        // Execute based on node type
        let result = match &node.node_type {
            NodeType::Start => Ok(None),
            NodeType::End => Ok(None),
            NodeType::Task { action } => self.execute_task(action, execution_id).await,
            NodeType::Parallel { branches } => {
                // Mark all branch nodes as ready
                Ok(None)
            }
            NodeType::Condition { branches } => {
                let executions = self.executions.read().await;
                let execution = executions.get(execution_id);
                if let Some(exec) = execution {
                    for branch in branches {
                        if branch.condition.evaluate(&exec.context) {
                            // Queue the target node
                            break;
                        }
                    }
                }
                Ok(None)
            }
            NodeType::Loop { max_iterations } => {
                // Handle loop logic
                Ok(Some(serde_json::json!({ "iterations": 0, "max": max_iterations })))
            }
            NodeType::HumanApproval { approvers, timeout_secs } => {
                // Mark as waiting for approval
                let mut executions = self.executions.write().await;
                if let Some(execution) = executions.get_mut(execution_id) {
                    execution.status = ExecutionStatus::WaitingApproval;
                    if let Some(state) = execution.node_states.get_mut(node_id) {
                        state.status = NodeStatus::WaitingApproval;
                    }
                }
                Ok(None)
            }
            NodeType::SubWorkflow { workflow_id: sub_id } => {
                Box::pin(self.execute(sub_id, HashMap::new())).await?;
                Ok(None)
            }
            NodeType::Delay { seconds } => {
                tokio::time::sleep(tokio::time::Duration::from_secs(*seconds)).await;
                Ok(None)
            }
            _ => Ok(None),
        };
        
        // Update node state based on result
        {
            let mut executions = self.executions.write().await;
            if let Some(execution) = executions.get_mut(execution_id) {
                if let Some(state) = execution.node_states.get_mut(node_id) {
                    match result {
                        Ok(output) => {
                            state.status = NodeStatus::Completed;
                            state.completed_at = Some(Utc::now());
                            state.output = output;
                            execution.completed_nodes.insert(node_id.to_string());
                        }
                        Err(e) => {
                            state.status = NodeStatus::Failed;
                            state.completed_at = Some(Utc::now());
                            state.error = Some(e.to_string());
                        }
                    }
                }
            }
        }
        
        // Continue execution
        Box::pin(self.run_execution(&workflow.id, execution_id)).await?;
        
        Ok(())
    }
    
    /// Execute a task action
    async fn execute_task(
        &self,
        action: &TaskAction,
        execution_id: &str,
    ) -> Result<Option<serde_json::Value>, WorkflowError> {
        let handlers = self.handlers.read().await;
        
        if let Some(handler) = handlers.get(&action.handler) {
            let context = {
                let executions = self.executions.read().await;
                executions.get(execution_id)
                    .map(|e| e.context.clone())
                    .unwrap_or_default()
            };
            
            handler.execute(action, context).await
        } else {
            Err(WorkflowError::HandlerNotFound(action.handler.clone()))
        }
    }
    
    /// Get execution status
    pub async fn get_execution(&self, execution_id: &str) -> Option<WorkflowExecution> {
        let executions = self.executions.read().await;
        executions.get(execution_id).cloned()
    }
    
    /// Cancel execution
    pub async fn cancel(&self, execution_id: &str) -> Result<(), WorkflowError> {
        let mut executions = self.executions.write().await;
        if let Some(execution) = executions.get_mut(execution_id) {
            execution.status = ExecutionStatus::Cancelled;
            execution.completed_at = Some(Utc::now());
        }
        Ok(())
    }
    
    /// Approve a node (for HumanApproval nodes)
    pub async fn approve(&self, execution_id: &str, node_id: &str, approved: bool) -> Result<(), WorkflowError> {
        let mut executions = self.executions.write().await;
        if let Some(execution) = executions.get_mut(execution_id) {
            if let Some(state) = execution.node_states.get_mut(node_id) {
                if state.status == NodeStatus::WaitingApproval {
                    state.status = if approved { NodeStatus::Completed } else { NodeStatus::Failed };
                    state.completed_at = Some(Utc::now());
                    execution.completed_nodes.insert(node_id.to_string());
                    execution.status = ExecutionStatus::Running;
                }
            }
        }
        Ok(())
    }
}

/// Task handler trait
#[async_trait::async_trait]
pub trait TaskHandler {
    async fn execute(
        &self,
        action: &TaskAction,
        context: HashMap<String, serde_json::Value>,
    ) -> Result<Option<serde_json::Value>, WorkflowError>;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum WorkflowError {
    WorkflowNotFound(String),
    ExecutionNotFound(String),
    NodeNotFound(String),
    HandlerNotFound(String),
    ValidationFailed(String),
    ExecutionFailed(String),
    Timeout(String),
    ApprovalRequired(String),
}

impl std::fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WorkflowNotFound(id) => write!(f, "Workflow not found: {}", id),
            Self::ExecutionNotFound(id) => write!(f, "Execution not found: {}", id),
            Self::NodeNotFound(id) => write!(f, "Node not found: {}", id),
            Self::HandlerNotFound(name) => write!(f, "Handler not found: {}", name),
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            Self::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            Self::Timeout(msg) => write!(f, "Timeout: {}", msg),
            Self::ApprovalRequired(node) => write!(f, "Approval required for node: {}", node),
        }
    }
}

impl std::error::Error for WorkflowError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("Test Workflow".into());
        assert!(!workflow.id.is_empty());
        assert_eq!(workflow.name, "Test Workflow");
    }
    
    #[test]
    fn test_workflow_validation_empty() {
        let workflow = Workflow::new("Empty".into());
        assert!(workflow.validate().is_err());
    }
    
    #[test]
    fn test_node_creation() {
        let node = WorkflowNode::task("Test Task".into(), TaskAction {
            action_type: "test".into(),
            handler: "test_handler".into(),
            params: HashMap::new(),
            timeout_secs: None,
        });
        
        assert_eq!(node.name, "Test Task");
    }
    
    #[test]
    fn test_condition_evaluation() {
        let cond = Condition::new("$status == \"success\"");
        
        let mut ctx = HashMap::new();
        ctx.insert("status".into(), serde_json::json!("success"));
        
        assert!(cond.evaluate(&ctx));
        
        ctx.insert("status".into(), serde_json::json!("failed"));
        assert!(!cond.evaluate(&ctx));
    }
    
    #[tokio::test]
    async fn test_workflow_engine() {
        let (tx, _rx) = mpsc::channel(100);
        let engine = WorkflowEngine::new(tx);
        
        let mut workflow = Workflow::new("Test".into());
        let start = WorkflowNode::new("start".into(), "Start".into(), NodeType::Start);
        let end = WorkflowNode::new("end".into(), "End".into(), NodeType::End);
        
        workflow.add_node(start);
        workflow.add_node(end);
        workflow.add_edge("start".into(), "end".into(), None);
        
        let id = engine.register(workflow).await.unwrap();
        assert!(!id.is_empty());
    }
}
