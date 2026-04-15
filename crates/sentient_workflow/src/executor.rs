//! ─── Workflow Executor ───

use crate::{Workflow, Node, WorkflowError, WorkflowResult, ExecutionStatus};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub workflow_id: String,
    pub execution_id: String,
    pub variables: HashMap<String, serde_json::Value>,
    pub node_outputs: HashMap<String, serde_json::Value>,
    pub current_node: Option<String>,
    pub status: ExecutionStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ExecutionContext {
    pub fn new(workflow_id: &str) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            execution_id: uuid::Uuid::new_v4().to_string(),
            variables: HashMap::new(),
            node_outputs: HashMap::new(),
            current_node: None,
            status: ExecutionStatus::Pending,
            started_at: chrono::Utc::now(),
            completed_at: None,
        }
    }
    
    pub fn set_variable(&mut self, key: &str, value: serde_json::Value) {
        self.variables.insert(key.to_string(), value);
    }
    
    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.variables.get(key)
    }
    
    pub fn set_output(&mut self, node_id: &str, output: serde_json::Value) {
        self.node_outputs.insert(node_id.to_string(), output);
    }
    
    pub fn get_output(&self, node_id: &str) -> Option<&serde_json::Value> {
        self.node_outputs.get(node_id)
    }
}

/// Workflow executor
pub struct WorkflowExecutor {
    max_parallel: usize,
    timeout_ms: u64,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        Self {
            max_parallel: 10,
            timeout_ms: 300_000, // 5 minutes
        }
    }
    
    pub async fn execute(&self, workflow: &Workflow) -> WorkflowResult<ExecutionContext> {
        workflow.validate()?;
        
        let mut ctx = ExecutionContext::new(&workflow.id);
        ctx.status = ExecutionStatus::Running;
        
        tracing::info!("Executing workflow: {} ({})", workflow.name, workflow.id);
        
        // Get start nodes (clone the node IDs to avoid borrow issues)
        let start_node_ids: Vec<String> = workflow.get_start_nodes()
            .iter()
            .map(|n| n.id.clone())
            .collect();
        
        for node_id in start_node_ids {
            if let Some(node) = workflow.get_node(&node_id) {
                self.execute_node_recursive(workflow, node, &mut ctx).await?;
            }
        }
        
        ctx.status = ExecutionStatus::Success;
        ctx.completed_at = Some(chrono::Utc::now());
        
        tracing::info!("Workflow completed: {}", workflow.id);
        Ok(ctx)
    }
    
    fn execute_node_recursive<'a>(
        &'a self,
        workflow: &'a Workflow,
        node: &'a Node,
        ctx: &'a mut ExecutionContext,
    ) -> Pin<Box<dyn Future<Output = WorkflowResult<()>> + 'a>> {
        Box::pin(async move {
            ctx.current_node = Some(node.id.clone());
            
            tracing::info!("Executing node: {} ({})", node.name, node.id);
            
            let result = self.run_node_action(node, ctx).await;
            
            match result {
                Ok(output) => {
                    ctx.set_output(&node.id, output);
                    
                    // Get connected node IDs
                    let next_node_ids: Vec<String> = workflow.connections.iter()
                        .filter(|c| c.source_node == node.id)
                        .map(|c| c.target_node.clone())
                        .collect();
                    
                    // Execute connected nodes
                    for next_id in next_node_ids {
                        if let Some(next_node) = workflow.get_node(&next_id) {
                            self.execute_node_recursive(workflow, next_node, ctx).await?;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Node {} failed: {}", node.id, e);
                    
                    match node.on_error {
                        crate::ErrorHandling::Stop => {
                            ctx.status = ExecutionStatus::Failed;
                            return Err(e);
                        }
                        crate::ErrorHandling::Continue => {
                            tracing::warn!("Continuing despite error");
                        }
                        crate::ErrorHandling::Retry { max_attempts } => {
                            for attempt in 0..max_attempts {
                                tracing::info!("Retry attempt {}/{}", attempt + 1, max_attempts);
                                if let Ok(output) = self.run_node_action(node, ctx).await {
                                    ctx.set_output(&node.id, output);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            ctx.current_node = None;
            Ok(())
        })
    }
    
    async fn run_node_action(
        &self,
        node: &Node,
        ctx: &ExecutionContext,
    ) -> WorkflowResult<serde_json::Value> {
        match &node.node_type {
            crate::NodeType::Http(config) => {
                tracing::info!("HTTP {} {}", config.method, config.url);
                Ok(serde_json::json!({ "status": "mock" }))
            }
            crate::NodeType::Email(config) => {
                tracing::info!("Email to: {:?}", config.to);
                Ok(serde_json::json!({ "sent": true }))
            }
            crate::NodeType::Script(config) => {
                tracing::info!("Running {} script", config.language);
                Ok(serde_json::json!({ "output": "script result" }))
            }
            crate::NodeType::Llm(config) => {
                tracing::info!("LLM prompt: {}", config.prompt);
                Ok(serde_json::json!({ "response": "AI response" }))
            }
            crate::NodeType::Condition(config) => {
                let result = self.evaluate_condition(&config.expression, ctx)?;
                Ok(serde_json::json!({ "result": result }))
            }
            crate::NodeType::Delay(config) => {
                tokio::time::sleep(std::time::Duration::from_millis(config.duration_ms.min(1000))).await;
                Ok(serde_json::json!({ "delayed": config.duration_ms }))
            }
            crate::NodeType::Loop(config) => {
                tracing::info!("Loop {} iterations", config.iterations);
                Ok(serde_json::json!({ "iterations": config.iterations }))
            }
            _ => {
                Ok(serde_json::json!({}))
            }
        }
    }
    
    fn evaluate_condition(&self, expr: &str, ctx: &ExecutionContext) -> WorkflowResult<bool> {
        if expr.contains("==") {
            let parts: Vec<&str> = expr.split("==").collect();
            if parts.len() == 2 {
                let left = parts[0].trim();
                let right = parts[1].trim();
                
                if let Some(val) = ctx.get_variable(left) {
                    return Ok(val.as_str().map(|s| s == right).unwrap_or(false));
                }
            }
        }
        Ok(true)
    }
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_execution_context() {
        let ctx = ExecutionContext::new("test-workflow");
        assert!(matches!(ctx.status, ExecutionStatus::Pending));
    }
}
