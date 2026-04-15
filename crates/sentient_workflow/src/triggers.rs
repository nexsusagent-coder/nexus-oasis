//! ─── Trigger Manager ───

use crate::{Trigger, TriggerType, Workflow, WorkflowResult};
use std::collections::HashMap;

/// Trigger manager
pub struct TriggerManager {
    triggers: HashMap<String, TriggerEntry>,
    running: bool,
}

struct TriggerEntry {
    trigger: Trigger,
    workflow_id: String,
}

impl TriggerManager {
    pub fn new() -> Self {
        Self {
            triggers: HashMap::new(),
            running: false,
        }
    }
    
    pub fn register(&mut self, workflow_id: &str, trigger: Trigger) -> String {
        let id = trigger.id.clone();
        self.triggers.insert(id.clone(), TriggerEntry {
            trigger,
            workflow_id: workflow_id.to_string(),
        });
        id
    }
    
    pub fn unregister(&mut self, trigger_id: &str) {
        self.triggers.remove(trigger_id);
    }
    
    pub async fn start(&mut self) -> WorkflowResult<()> {
        if self.running { return Ok(()); }
        
        tracing::info!("Starting trigger manager with {} triggers", self.triggers.len());
        
        // Start listening for triggers
        for (id, entry) in &self.triggers {
            match &entry.trigger.trigger_type {
                TriggerType::Schedule { cron } => {
                    tracing::info!("Cron trigger: {} -> {}", id, cron);
                }
                TriggerType::Webhook { path, method } => {
                    tracing::info!("Webhook trigger: {} -> {} {}", id, method, path);
                }
                TriggerType::Event { event_type } => {
                    tracing::info!("Event trigger: {} -> {}", id, event_type);
                }
                TriggerType::Voice { phrase } => {
                    tracing::info!("Voice trigger: {} -> '{}'", id, phrase);
                }
                TriggerType::FileWatch { path, pattern } => {
                    tracing::info!("File watch trigger: {} -> {} ({})", id, path, pattern);
                }
                TriggerType::Manual => {
                    tracing::info!("Manual trigger: {}", id);
                }
            }
        }
        
        self.running = true;
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    pub fn check_trigger(&self, trigger_type: &TriggerType) -> Option<&str> {
        for (id, entry) in &self.triggers {
            if self.matches_trigger(&entry.trigger.trigger_type, trigger_type) {
                return Some(&entry.workflow_id);
            }
        }
        None
    }
    
    fn matches_trigger(&self, a: &TriggerType, b: &TriggerType) -> bool {
        match (a, b) {
            (TriggerType::Webhook { path: p1, .. }, TriggerType::Webhook { path: p2, .. }) => p1 == p2,
            (TriggerType::Event { event_type: e1 }, TriggerType::Event { event_type: e2 }) => e1 == e2,
            (TriggerType::Voice { phrase: p1 }, TriggerType::Voice { phrase: p2 }) => p1 == p2,
            _ => false,
        }
    }
}

impl Default for TriggerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_registration() {
        let mut mgr = TriggerManager::new();
        let trigger = Trigger::new(TriggerType::Manual);
        mgr.register("wf-1", trigger);
        assert_eq!(mgr.triggers.len(), 1);
    }
}
