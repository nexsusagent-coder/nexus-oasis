//! Backup scheduler

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{BackupConfig, BackupEngine, BackupHandle, Result, BackupError};

/// Schedule frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    /// Run every N minutes
    Minutes(u32),
    /// Run every N hours
    Hours(u32),
    /// Run every N days
    Days(u32),
    /// Run every N weeks
    Weeks(u32),
    /// Run at specific times
    Cron(String),
    /// Run once at specific time
    Once(DateTime<Utc>),
}

/// Backup schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    /// Schedule ID
    pub id: uuid::Uuid,
    /// Schedule name
    pub name: String,
    /// Backup configuration
    pub config: BackupConfig,
    /// Frequency
    pub frequency: ScheduleFrequency,
    /// Enabled
    pub enabled: bool,
    /// Last run
    pub last_run: Option<DateTime<Utc>>,
    /// Next run
    pub next_run: Option<DateTime<Utc>>,
    /// Run count
    pub run_count: u64,
    /// Last result
    pub last_result: Option<String>,
}

impl BackupSchedule {
    /// Create new schedule
    pub fn new(name: String, config: BackupConfig, frequency: ScheduleFrequency) -> Self {
        let next_run = Self::calculate_next_run(&frequency, None);
        
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            config,
            frequency,
            enabled: true,
            last_run: None,
            next_run,
            run_count: 0,
            last_result: None,
        }
    }

    /// Calculate next run time
    fn calculate_next_run(frequency: &ScheduleFrequency, last: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
        let base = last.unwrap_or_else(Utc::now);
        
        Some(match frequency {
            ScheduleFrequency::Minutes(m) => base + Duration::minutes(*m as i64),
            ScheduleFrequency::Hours(h) => base + Duration::hours(*h as i64),
            ScheduleFrequency::Days(d) => base + Duration::days(*d as i64),
            ScheduleFrequency::Weeks(w) => base + Duration::weeks(*w as i64),
            ScheduleFrequency::Cron(_) => {
                // TODO: Implement cron parsing
                base + Duration::hours(24)
            }
            ScheduleFrequency::Once(time) => *time,
        })
    }

    /// Update next run time
    pub fn update_next_run(&mut self) {
        self.next_run = Self::calculate_next_run(&self.frequency, self.last_run);
    }
}

/// Backup scheduler
pub struct BackupScheduler {
    schedules: Arc<RwLock<HashMap<uuid::Uuid, BackupSchedule>>>,
    running: Arc<RwLock<bool>>,
}

impl BackupScheduler {
    /// Create new scheduler
    pub fn new() -> Self {
        Self {
            schedules: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Add schedule
    pub async fn add_schedule(&self, schedule: BackupSchedule) -> Result<uuid::Uuid> {
        let id = schedule.id;
        let mut schedules = self.schedules.write().await;
        schedules.insert(id, schedule);
        
        tracing::info!(schedule_id = %id, "Schedule added");
        Ok(id)
    }

    /// Remove schedule
    pub async fn remove_schedule(&self, id: uuid::Uuid) -> Result<()> {
        let mut schedules = self.schedules.write().await;
        schedules.remove(&id);
        
        tracing::info!(schedule_id = %id, "Schedule removed");
        Ok(())
    }

    /// Get schedule
    pub async fn get_schedule(&self, id: uuid::Uuid) -> Option<BackupSchedule> {
        let schedules = self.schedules.read().await;
        schedules.get(&id).cloned()
    }

    /// List all schedules
    pub async fn list_schedules(&self) -> Vec<BackupSchedule> {
        let schedules = self.schedules.read().await;
        schedules.values().cloned().collect()
    }

    /// Enable schedule
    pub async fn enable_schedule(&self, id: uuid::Uuid) -> Result<()> {
        let mut schedules = self.schedules.write().await;
        if let Some(schedule) = schedules.get_mut(&id) {
            schedule.enabled = true;
            tracing::info!(schedule_id = %id, "Schedule enabled");
        }
        Ok(())
    }

    /// Disable schedule
    pub async fn disable_schedule(&self, id: uuid::Uuid) -> Result<()> {
        let mut schedules = self.schedules.write().await;
        if let Some(schedule) = schedules.get_mut(&id) {
            schedule.enabled = false;
            tracing::info!(schedule_id = %id, "Schedule disabled");
        }
        Ok(())
    }

    /// Start scheduler
    pub async fn start(&self) {
        let mut running = self.running.write().await;
        *running = true;
        tracing::info!("Backup scheduler started");
    }

    /// Stop scheduler
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Backup scheduler stopped");
    }

    /// Run pending backups
    pub async fn run_pending(&self) -> Result<Vec<BackupHandle>> {
        let now = Utc::now();
        let mut handles = Vec::new();

        let schedules = self.schedules.read().await;
        
        for schedule in schedules.values() {
            if !schedule.enabled {
                continue;
            }

            if let Some(next_run) = schedule.next_run {
                if next_run <= now {
                    tracing::info!(schedule_id = %schedule.id, "Running scheduled backup");
                    
                    let engine = BackupEngine::new(schedule.config.clone());
                    match engine.execute().await {
                        Ok(handle) => {
                            handles.push(handle);
                        }
                        Err(e) => {
                            tracing::error!(schedule_id = %schedule.id, error = %e, "Scheduled backup failed");
                        }
                    }
                }
            }
        }

        // Update schedules
        drop(schedules);
        let mut schedules = self.schedules.write().await;
        for schedule in schedules.values_mut() {
            schedule.last_run = Some(now);
            schedule.update_next_run();
            schedule.run_count += 1;
        }

        Ok(handles)
    }

    /// Run specific schedule manually
    pub async fn run_now(&self, id: uuid::Uuid) -> Result<BackupHandle> {
        let schedules = self.schedules.read().await;
        
        let schedule = schedules.get(&id)
            .ok_or_else(|| BackupError::NotFound(format!("Schedule not found: {}", id)))?;

        let engine = BackupEngine::new(schedule.config.clone());
        let handle = engine.execute().await?;

        // Update schedule
        drop(schedules);
        let mut schedules = self.schedules.write().await;
        if let Some(schedule) = schedules.get_mut(&id) {
            schedule.last_run = Some(Utc::now());
            schedule.run_count += 1;
        }

        Ok(handle)
    }
}

impl Default for BackupScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scheduler_creation() {
        let scheduler = BackupScheduler::new();
        let schedules = scheduler.list_schedules().await;
        assert!(schedules.is_empty());
    }

    #[tokio::test]
    async fn test_add_remove_schedule() {
        let scheduler = BackupScheduler::new();
        let config = BackupConfig::default();
        let schedule = BackupSchedule::new(
            "test".to_string(),
            config,
            ScheduleFrequency::Days(1),
        );
        let id = schedule.id;

        scheduler.add_schedule(schedule).await.unwrap();
        assert!(scheduler.get_schedule(id).await.is_some());

        scheduler.remove_schedule(id).await.unwrap();
        assert!(scheduler.get_schedule(id).await.is_none());
    }
}
