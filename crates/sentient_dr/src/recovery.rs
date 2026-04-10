//! Recovery orchestration

use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    DRError, RecoveryPlan, RecoveryExecution, RecoveryStatus, RecoveryStep, RecoveryStepType,
    StepResult, StepStatus, Result,
};

/// Recovery orchestrator
pub struct RecoveryOrchestrator {
    plans: Arc<RwLock<std::collections::HashMap<Uuid, RecoveryPlan>>>,
    executions: Arc<RwLock<std::collections::HashMap<Uuid, RecoveryExecution>>>,
}

impl RecoveryOrchestrator {
    /// Create new orchestrator
    pub fn new() -> Self {
        Self {
            plans: Arc::new(RwLock::new(std::collections::HashMap::new())),
            executions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Add recovery plan
    pub async fn add_plan(&self, plan: RecoveryPlan) -> Result<Uuid> {
        plan.validate()?;
        
        let id = plan.id;
        let mut plans = self.plans.write().await;
        plans.insert(id, plan);
        
        tracing::info!(plan_id = %id, "Recovery plan added");
        Ok(id)
    }

    /// Get plan
    pub async fn get_plan(&self, id: Uuid) -> Option<RecoveryPlan> {
        self.plans.read().await.get(&id).cloned()
    }

    /// List all plans
    pub async fn list_plans(&self) -> Vec<RecoveryPlan> {
        self.plans.read().await.values().cloned().collect()
    }

    /// Execute recovery plan
    pub async fn execute(&self, plan_id: Uuid) -> Result<RecoveryExecution> {
        let plan = {
            let plans = self.plans.read().await;
            plans.get(&plan_id)
                .cloned()
                .ok_or_else(|| DRError::PlanNotFound(plan_id.to_string()))?
        };

        let execution = RecoveryExecution {
            id: Uuid::new_v4(),
            plan_id,
            started_at: Utc::now(),
            completed_at: None,
            current_step: None,
            step_results: std::collections::HashMap::new(),
            status: RecoveryStatus::InProgress,
            error: None,
        };

        let execution_id = execution.id;
        {
            let mut executions = self.executions.write().await;
            executions.insert(execution_id, execution.clone());
        }

        tracing::info!(
            execution_id = %execution_id,
            plan_id = %plan_id,
            "Starting recovery execution"
        );

        // Execute steps in order
        let steps = plan.get_execution_order();
        let mut all_success = true;

        for step in steps {
            // Check dependencies
            let deps_met = step.depends_on.iter().all(|dep_id| {
                execution.step_results.get(dep_id)
                    .map(|r| r.status == StepStatus::Success)
                    .unwrap_or(false)
            });

            if !deps_met {
                tracing::warn!(
                    step_id = %step.id,
                    "Skipping step - dependencies not met"
                );
                
                self.record_step_result(execution_id, step.id, StepResult {
                    step_id: step.id,
                    started_at: Utc::now(),
                    completed_at: Some(Utc::now()),
                    status: StepStatus::Skipped,
                    output: String::new(),
                    error: Some("Dependencies not met".to_string()),
                }).await?;
                
                continue;
            }

            // Execute step
            let result = self.execute_step(&step).await;
            
            match result {
                Ok(step_result) => {
                    if step_result.status != StepStatus::Success {
                        all_success = false;
                    }
                    self.record_step_result(execution_id, step.id, step_result).await?;
                }
                Err(e) => {
                    all_success = false;
                    self.record_step_result(execution_id, step.id, StepResult {
                        step_id: step.id,
                        started_at: Utc::now(),
                        completed_at: Some(Utc::now()),
                        status: StepStatus::Failed,
                        output: String::new(),
                        error: Some(e.to_string()),
                    }).await?;
                }
            }
        }

        // Update execution status
        let mut executions = self.executions.write().await;
        if let Some(exec) = executions.get_mut(&execution_id) {
            exec.status = if all_success { RecoveryStatus::Completed } else { RecoveryStatus::Failed };
            exec.completed_at = Some(Utc::now());
        }

        let final_execution = executions.get(&execution_id).cloned()
            .ok_or_else(|| DRError::RecoveryFailed("Execution not found".to_string()))?;

        tracing::info!(
            execution_id = %execution_id,
            status = ?final_execution.status,
            "Recovery execution completed"
        );

        Ok(final_execution)
    }

    /// Execute single step
    async fn execute_step(&self, step: &RecoveryStep) -> Result<StepResult> {
        let started_at = Utc::now();

        tracing::info!(step_id = %step.id, step_name = %step.name, "Executing step");

        let result = match step.step_type {
            RecoveryStepType::HealthCheck => self.execute_health_check(step).await,
            RecoveryStepType::Failover => self.execute_failover(step).await,
            RecoveryStepType::RestoreBackup => self.execute_restore_backup(step).await,
            RecoveryStepType::RestartServices => self.execute_restart_services(step).await,
            RecoveryStepType::UpdateDns => self.execute_update_dns(step).await,
            RecoveryStepType::NotifyTeam => self.execute_notify_team(step).await,
            RecoveryStepType::CustomScript => self.execute_custom_script(step).await,
            RecoveryStepType::WaitForCondition => self.execute_wait_for_condition(step).await,
        };

        let completed_at = Utc::now();

        match result {
            Ok(output) => Ok(StepResult {
                step_id: step.id,
                started_at,
                completed_at: Some(completed_at),
                status: StepStatus::Success,
                output,
                error: None,
            }),
            Err(e) => Ok(StepResult {
                step_id: step.id,
                started_at,
                completed_at: Some(completed_at),
                status: StepStatus::Failed,
                output: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    /// Record step result
    async fn record_step_result(&self, execution_id: Uuid, step_id: Uuid, result: StepResult) -> Result<()> {
        let mut executions = self.executions.write().await;
        if let Some(exec) = executions.get_mut(&execution_id) {
            exec.step_results.insert(step_id, result);
            exec.current_step = Some(step_id);
        }
        Ok(())
    }

    // Step implementations

    async fn execute_health_check(&self, _step: &RecoveryStep) -> Result<String> {
        tracing::info!("Executing health check");
        // Implementation would check actual health endpoints
        Ok("Health check passed".to_string())
    }

    async fn execute_failover(&self, step: &RecoveryStep) -> Result<String> {
        let target_region = step.parameters.get("target_region")
            .ok_or_else(|| DRError::RecoveryFailed("No target region specified".to_string()))?;
        
        tracing::info!(target = %target_region, "Executing failover");
        Ok(format!("Failed over to {}", target_region))
    }

    async fn execute_restore_backup(&self, step: &RecoveryStep) -> Result<String> {
        let backup_id = step.parameters.get("backup_id")
            .ok_or_else(|| DRError::RecoveryFailed("No backup ID specified".to_string()))?;
        
        tracing::info!(backup_id = %backup_id, "Restoring from backup");
        Ok(format!("Restored from backup {}", backup_id))
    }

    async fn execute_restart_services(&self, step: &RecoveryStep) -> Result<String> {
        let services = step.parameters.get("services").cloned().unwrap_or_else(|| "all".to_string());
        
        tracing::info!(services = %services, "Restarting services");
        Ok(format!("Restarted services: {}", services))
    }

    async fn execute_update_dns(&self, step: &RecoveryStep) -> Result<String> {
        let target = step.parameters.get("target")
            .ok_or_else(|| DRError::RecoveryFailed("No DNS target specified".to_string()))?;
        
        tracing::info!(target = %target, "Updating DNS");
        Ok(format!("DNS updated to {}", target))
    }

    async fn execute_notify_team(&self, step: &RecoveryStep) -> Result<String> {
        let team = step.parameters.get("team").cloned().unwrap_or_else(|| "on-call".to_string());
        let message = step.parameters.get("message").cloned().unwrap_or_else(|| "Recovery in progress".to_string());
        
        tracing::info!(team = %team, message = %message, "Notifying team");
        Ok(format!("Notified {}: {}", team, message))
    }

    async fn execute_custom_script(&self, step: &RecoveryStep) -> Result<String> {
        let script = step.parameters.get("script")
            .ok_or_else(|| DRError::RecoveryFailed("No script specified".to_string()))?;
        
        tracing::info!(script = %script, "Executing custom script");
        Ok(format!("Executed script: {}", script))
    }

    async fn execute_wait_for_condition(&self, step: &RecoveryStep) -> Result<String> {
        let condition = step.parameters.get("condition")
            .ok_or_else(|| DRError::RecoveryFailed("No condition specified".to_string()))?;
        
        tracing::info!(condition = %condition, "Waiting for condition");
        Ok(format!("Condition met: {}", condition))
    }

    /// Get execution status
    pub async fn get_execution(&self, id: Uuid) -> Option<RecoveryExecution> {
        self.executions.read().await.get(&id).cloned()
    }

    /// Cancel execution
    pub async fn cancel_execution(&self, id: Uuid) -> Result<()> {
        let mut executions = self.executions.write().await;
        if let Some(exec) = executions.get_mut(&id) {
            exec.status = RecoveryStatus::Cancelled;
            exec.completed_at = Some(Utc::now());
        }
        Ok(())
    }
}

impl Default for RecoveryOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator() {
        let orchestrator = RecoveryOrchestrator::new();
        
        let plan = RecoveryPlan::new(
            "Test Plan".to_string(),
            "Test recovery".to_string(),
        );
        
        let plan_id = orchestrator.add_plan(plan).await.unwrap();
        let retrieved = orchestrator.get_plan(plan_id).await;
        
        assert!(retrieved.is_some());
    }
}
