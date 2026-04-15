//! ─── Scheduler System ───
//!
//! Handles scheduling and execution of triggers

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use chrono::{Datelike, Timelike};

use crate::trigger::{Trigger, TriggerType};
use crate::action::ActionExecutor;
use crate::ProactiveResult;

/// Scheduler for managing trigger execution
pub struct Scheduler {
    triggers: Arc<RwLock<HashMap<String, Trigger>>>,
    executor: ActionExecutor,
    tx: mpsc::Sender<ScheduledTask>,
}

/// A scheduled task ready for execution
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub trigger_id: String,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub priority: u8,
    pub action: String,
}

/// Status of a scheduled task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl Scheduler {
    /// Create new scheduler
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::channel(100);
        Self {
            triggers: Arc::new(RwLock::new(HashMap::new())),
            executor: ActionExecutor::new(),
            tx,
        }
    }
    
    /// Add a trigger
    pub async fn add_trigger(&self, trigger: Trigger) {
        let mut triggers = self.triggers.write().await;
        triggers.insert(trigger.id.clone(), trigger);
    }
    
    /// Remove a trigger
    pub async fn remove_trigger(&self, id: &str) -> ProactiveResult<()> {
        let mut triggers = self.triggers.write().await;
        triggers.remove(id)
            .ok_or_else(|| crate::ProactiveError::TriggerNotFound(id.to_string()))?;
        Ok(())
    }
    
    /// Enable/disable a trigger
    pub async fn set_trigger_enabled(&self, id: &str, enabled: bool) -> ProactiveResult<()> {
        let mut triggers = self.triggers.write().await;
        let trigger = triggers.get_mut(id)
            .ok_or_else(|| crate::ProactiveError::TriggerNotFound(id.to_string()))?;
        trigger.enabled = enabled;
        Ok(())
    }
    
    /// Get all triggers
    pub async fn get_triggers(&self) -> Vec<Trigger> {
        let triggers = self.triggers.read().await;
        triggers.values().cloned().collect()
    }
    
    /// Check which triggers should fire now
    pub async fn check_triggers(&self) -> Vec<ScheduledTask> {
        let triggers = self.triggers.read().await;
        let now = chrono::Utc::now();
        
        triggers.values()
            .filter(|t| t.can_fire() && self.should_fire(t, &now))
            .map(|t| ScheduledTask {
                trigger_id: t.id.clone(),
                scheduled_time: now,
                priority: t.priority,
                action: t.action.clone(),
            })
            .collect()
    }
    
    /// Check if a specific trigger should fire
    fn should_fire(&self, trigger: &Trigger, now: &chrono::DateTime<chrono::Utc>) -> bool {
        match &trigger.trigger_type {
            TriggerType::TimeBased { time, days } => {
                let parts: Vec<&str> = time.split(':').collect();
                if parts.len() != 2 { return false; }
                
                let hour: u32 = parts[0].parse().unwrap_or(0);
                let minute: u32 = parts[1].parse().unwrap_or(0);
                
                now.hour() == hour 
                    && now.minute() == minute
                    && days.contains(&(now.weekday().num_days_from_sunday() as u8))
            }
            
            TriggerType::Cron { expression } => {
                // Simplified cron check
                self.check_cron(expression, now)
            }
            
            TriggerType::Interval { seconds } => {
                if let Some(last) = trigger.last_executed {
                    let elapsed = (*now - last).num_seconds() as u64;
                    return elapsed >= *seconds;
                }
                true // First execution
            }
            
            _ => false, // Event/pattern triggers handled elsewhere
        }
    }
    
    /// Simplified cron check
    fn check_cron(&self, expression: &str, now: &chrono::DateTime<chrono::Utc>) -> bool {
        // Parse simplified cron: minute hour day month weekday
        let parts: Vec<&str> = expression.split_whitespace().collect();
        if parts.len() != 5 { return false; }
        
        let check_part = |part: &str, value: u32| -> bool {
            part == "*" || part.parse::<u32>().map(|v| v == value).unwrap_or(false)
        };
        
        check_part(parts[0], now.minute())
            && check_part(parts[1], now.hour())
            && check_part(parts[2], now.day())
            && check_part(parts[3], now.month())
            && check_part(parts[4], now.weekday().num_days_from_monday())
    }
    
    /// Execute a scheduled task
    pub async fn execute_task(&self, task: ScheduledTask) -> crate::action::ActionResult {
        self.executor.execute(&task.action).await
    }
    
    /// Start the scheduler loop
    pub async fn start(&self) {
        let triggers = self.triggers.clone();
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            loop {
                let mut triggers_guard = triggers.write().await;
                
                for (id, trigger) in triggers_guard.iter_mut() {
                    if trigger.can_fire() {
                        // Check and potentially execute
                        trigger.mark_executed();
                    }
                }
                
                drop(triggers_guard);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_add_remove_trigger() {
        let scheduler = Scheduler::new();
        let trigger = Trigger::new("test", "Test", TriggerType::hourly());
        
        scheduler.add_trigger(trigger).await;
        let triggers = scheduler.get_triggers().await;
        assert_eq!(triggers.len(), 1);
        
        scheduler.remove_trigger("test").await.unwrap();
        let triggers = scheduler.get_triggers().await;
        assert!(triggers.is_empty());
    }
}
