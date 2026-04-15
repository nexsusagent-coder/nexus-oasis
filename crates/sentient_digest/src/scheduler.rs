//! Digest scheduler - timed delivery

use chrono::{DateTime, Utc, TimeZone, Timelike};
use cron::Schedule;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{DigestError, DigestResult};

/// Schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConfig {
    /// Cron expression for scheduling
    pub cron_expression: String,
    /// Timezone offset in hours
    pub timezone_offset: i32,
    /// Enabled
    pub enabled: bool,
    /// Channels to deliver to
    pub channels: Vec<String>,
}

impl Default for ScheduleConfig {
    fn default() -> Self {
        Self {
            cron_expression: "0 8 * * * *".to_string(),
            timezone_offset: 3, // Europe/Istanbul
            enabled: true,
            channels: vec!["telegram".to_string()],
        }
    }
}

impl ScheduleConfig {
    pub fn morning() -> Self {
        Self {
            cron_expression: "0 8 * * * *".to_string(),
            ..Default::default()
        }
    }

    pub fn evening() -> Self {
        Self {
            cron_expression: "0 20 * * * *".to_string(),
            ..Default::default()
        }
    }

    pub fn daily_at(hour: u32, minute: u32) -> Self {
        Self {
            cron_expression: format!("{} {} * * * *", minute, hour),
            ..Default::default()
        }
    }

    /// Get next scheduled time (simplified)
    pub fn next_run(&self) -> Option<DateTime<Utc>> {
        let schedule = Schedule::from_str(&self.cron_expression).ok()?;
        
        // Use UTC for simplicity
        schedule.upcoming(Utc).next()
    }

    /// Get all upcoming runs
    pub fn upcoming(&self, count: usize) -> Vec<DateTime<Utc>> {
        if let Ok(schedule) = Schedule::from_str(&self.cron_expression) {
            schedule.upcoming(Utc).take(count).collect()
        } else {
            Vec::new()
        }
    }
}

/// Digest scheduler
pub struct DigestScheduler {
    config: ScheduleConfig,
    last_run: Option<DateTime<Utc>>,
    next_run: Option<DateTime<Utc>>,
}

impl DigestScheduler {
    pub fn new(config: ScheduleConfig) -> Self {
        let next_run = config.next_run();
        Self {
            config,
            last_run: None,
            next_run,
        }
    }

    /// Check if it's time to generate digest
    pub fn should_run(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        if let Some(next) = self.next_run {
            Utc::now() >= next
        } else {
            false
        }
    }

    /// Mark digest as generated
    pub fn mark_completed(&mut self) {
        self.last_run = Some(Utc::now());
        self.next_run = self.config.next_run();
    }

    /// Get time until next digest
    pub fn time_until_next(&self) -> Option<chrono::Duration> {
        self.next_run.map(|next| next - Utc::now())
    }

    /// Get config
    pub fn config(&self) -> &ScheduleConfig {
        &self.config
    }

    /// Get last run time
    pub fn last_run(&self) -> Option<DateTime<Utc>> {
        self.last_run
    }

    /// Get next run time
    pub fn next_run(&self) -> Option<DateTime<Utc>> {
        self.next_run
    }
}
