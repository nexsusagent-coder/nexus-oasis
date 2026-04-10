//! Recovery plan management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::Result;

/// Recovery plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPlan {
    /// Plan ID
    pub id: Uuid,
    /// Plan name
    pub name: String,
    /// Plan description
    pub description: String,
    /// RTO (Recovery Time Objective) in seconds
    pub rto_secs: u64,
    /// RPO (Recovery Point Objective) in seconds
    pub rpo_secs: u64,
    /// Recovery steps
    pub steps: Vec<RecoveryStep>,
    /// Last updated
    pub last_updated: DateTime<Utc>,
    /// Last tested
    pub last_tested: Option<DateTime<Utc>>,
    /// Is plan active
    pub active: bool,
}

/// Recovery step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryStep {
    /// Step ID
    pub id: Uuid,
    /// Step name
    pub name: String,
    /// Step description
    pub description: String,
    /// Order in sequence
    pub order: u32,
    /// Timeout in seconds
    pub timeout_secs: u64,
    /// Required steps (must complete first)
    pub depends_on: Vec<Uuid>,
    /// Step type
    pub step_type: RecoveryStepType,
    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Types of recovery steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStepType {
    /// Restore from backup
    RestoreBackup,
    /// Failover to secondary
    Failover,
    /// Restart services
    RestartServices,
    /// Update DNS
    UpdateDns,
    /// Notify team
    NotifyTeam,
    /// Run health checks
    HealthCheck,
    /// Custom script
    CustomScript,
    /// Wait for condition
    WaitForCondition,
}

/// Recovery execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryExecution {
    /// Execution ID
    pub id: Uuid,
    /// Plan ID
    pub plan_id: Uuid,
    /// Started at
    pub started_at: DateTime<Utc>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Current step
    pub current_step: Option<Uuid>,
    /// Step results
    pub step_results: HashMap<Uuid, StepResult>,
    /// Overall status
    pub status: RecoveryStatus,
    /// Error message
    pub error: Option<String>,
}

/// Step execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step ID
    pub step_id: Uuid,
    /// Started at
    pub started_at: DateTime<Utc>,
    /// Completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Status
    pub status: StepStatus,
    /// Output
    pub output: String,
    /// Error
    pub error: Option<String>,
}

/// Recovery status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecoveryStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Step status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

impl RecoveryPlan {
    /// Create new recovery plan
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            rto_secs: 14400, // 4 hours
            rpo_secs: 3600,  // 1 hour
            steps: Vec::new(),
            last_updated: Utc::now(),
            last_tested: None,
            active: true,
        }
    }

    /// Add recovery step
    pub fn add_step(&mut self, step: RecoveryStep) {
        self.steps.push(step);
        self.steps.sort_by_key(|s| s.order);
        self.last_updated = Utc::now();
    }

    /// Remove step
    pub fn remove_step(&mut self, step_id: Uuid) {
        self.steps.retain(|s| s.id != step_id);
        self.last_updated = Utc::now();
    }

    /// Validate plan
    pub fn validate(&self) -> Result<()> {
        // Check all dependencies exist
        let step_ids: Vec<Uuid> = self.steps.iter().map(|s| s.id).collect();
        
        for step in &self.steps {
            for dep_id in &step.depends_on {
                if !step_ids.contains(dep_id) {
                    return Err(crate::DRError::InvalidConfig(
                        format!("Step {} has invalid dependency", step.id)
                    ));
                }
            }
        }

        // Check for circular dependencies
        // (simplified check - full implementation would need proper cycle detection)
        
        Ok(())
    }

    /// Get execution order
    pub fn get_execution_order(&self) -> Vec<&RecoveryStep> {
        // Topological sort based on dependencies
        // Simplified version - just use order field
        self.steps.iter().collect()
    }
}

impl RecoveryStep {
    /// Create new recovery step
    pub fn new(name: String, step_type: RecoveryStepType, order: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            order,
            timeout_secs: 300,
            depends_on: Vec::new(),
            step_type,
            parameters: HashMap::new(),
        }
    }

    /// Add dependency
    pub fn depends_on(mut self, step_id: Uuid) -> Self {
        self.depends_on.push(step_id);
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Add parameter
    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_plan() {
        let mut plan = RecoveryPlan::new(
            "Test Plan".to_string(),
            "Test recovery plan".to_string(),
        );

        let step1 = RecoveryStep::new(
            "Health Check".to_string(),
            RecoveryStepType::HealthCheck,
            1,
        );

        let step2 = RecoveryStep::new(
            "Failover".to_string(),
            RecoveryStepType::Failover,
            2,
        ).depends_on(step1.id);

        plan.add_step(step1);
        plan.add_step(step2);

        assert_eq!(plan.steps.len(), 2);
        assert!(plan.validate().is_ok());
    }
}
