//! ─── Workflow Builder ───

use crate::{Workflow, Node, Connection, NodeType, Trigger, WorkflowResult};

/// Fluent workflow builder
pub struct WorkflowBuilder {
    workflow: Workflow,
}

impl WorkflowBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            workflow: Workflow::new(name),
        }
    }
    
    pub fn description(mut self, desc: &str) -> Self {
        self.workflow.description = desc.to_string();
        self
    }
    
    pub fn node(mut self, name: &str, node_type: NodeType) -> NodeBuilder {
        let node = Node::new(name, node_type);
        let node_id = node.id.clone();
        self.workflow.nodes.push(node);
        NodeBuilder { builder: self, node_id }
    }
    
    pub fn trigger(mut self, trigger: Trigger) -> Self {
        self.workflow.triggers.push(trigger);
        self
    }
    
    pub fn connect(mut self, from: &str, to: &str) -> Self {
        let conn = Connection::new(from, "output", to, "input");
        self.workflow.connections.push(conn);
        self
    }
    
    pub fn variable(mut self, key: &str, value: serde_json::Value) -> Self {
        self.workflow.variables.insert(key.to_string(), value);
        self
    }
    
    pub fn build(self) -> WorkflowResult<Workflow> {
        self.workflow.validate()?;
        Ok(self.workflow)
    }
    
    pub fn build_unchecked(self) -> Workflow {
        self.workflow
    }
}

pub struct NodeBuilder {
    builder: WorkflowBuilder,
    node_id: String,
}

impl NodeBuilder {
    pub fn position(mut self, x: f64, y: f64) -> Self {
        if let Some(node) = self.builder.workflow.nodes.iter_mut().find(|n| n.id == self.node_id) {
            node.position.x = x;
            node.position.y = y;
        }
        self
    }
    
    pub fn config(mut self, config: serde_json::Value) -> Self {
        if let Some(node) = self.builder.workflow.nodes.iter_mut().find(|n| n.id == self.node_id) {
            node.config = config;
        }
        self
    }
    
    pub fn done(self) -> WorkflowBuilder {
        self.builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder() {
        let wf = WorkflowBuilder::new("Test")
            .description("Test workflow")
            .build_unchecked();
        
        assert_eq!(wf.name, "Test");
    }
}
