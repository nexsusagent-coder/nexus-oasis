//! Failover management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DRError, HealthStatus, Result};

/// Failover mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FailoverMode {
    /// Automatic failover
    Automatic,
    /// Manual failover only
    Manual,
    /// Disabled
    Disabled,
}

/// Failover state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverState {
    /// Current primary region
    pub primary_region: String,
    /// Current secondary region
    pub secondary_region: Option<String>,
    /// Failover mode
    pub mode: FailoverMode,
    /// Last failover time
    pub last_failover: Option<DateTime<Utc>>,
    /// Failover count
    pub failover_count: u64,
    /// Is currently failing over
    pub in_progress: bool,
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Enable automatic failover
    pub auto_failover: bool,
    /// Health check failures before failover
    pub failure_threshold: u32,
    /// Time window for failure counting (seconds)
    pub failure_window_secs: u64,
    /// Cooldown period after failover (seconds)
    pub cooldown_secs: u64,
    /// Maximum failovers per hour
    pub max_failovers_per_hour: u32,
    /// Regions in priority order
    pub region_priority: Vec<String>,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            auto_failover: true,
            failure_threshold: 3,
            failure_window_secs: 60,
            cooldown_secs: 300,
            max_failovers_per_hour: 2,
            region_priority: vec!["primary".to_string(), "secondary".to_string()],
        }
    }
}

/// Failover manager
pub struct FailoverManager {
    state: Arc<RwLock<FailoverState>>,
    config: FailoverConfig,
    failure_counts: Arc<RwLock<Vec<DateTime<Utc>>>>,
}

impl FailoverManager {
    /// Create new failover manager
    pub fn new(config: FailoverConfig) -> Self {
        let primary = config.region_priority.first()
            .cloned()
            .unwrap_or_else(|| "primary".to_string());

        Self {
            state: Arc::new(RwLock::new(FailoverState {
                primary_region: primary,
                secondary_region: config.region_priority.get(1).cloned(),
                mode: if config.auto_failover { FailoverMode::Automatic } else { FailoverMode::Manual },
                last_failover: None,
                failover_count: 0,
                in_progress: false,
            })),
            config,
            failure_counts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get current state
    pub async fn get_state(&self) -> FailoverState {
        self.state.read().await.clone()
    }

    /// Record health check failure
    pub async fn record_failure(&self) -> Result<()> {
        let now = Utc::now();
        let mut failures = self.failure_counts.write().await;
        
        // Remove old failures outside window
        let cutoff = now - chrono::Duration::seconds(self.config.failure_window_secs as i64);
        failures.retain(|&t| t > cutoff);
        
        failures.push(now);
        
        // Check if threshold reached
        if failures.len() >= self.config.failure_threshold as usize {
            tracing::warn!(
                failures = failures.len(),
                threshold = self.config.failure_threshold,
                "Failure threshold reached, initiating failover"
            );
            self.initiate_failover().await?;
        }

        Ok(())
    }

    /// Record successful health check
    pub async fn record_success(&self) {
        let mut failures = self.failure_counts.write().await;
        failures.clear();
    }

    /// Initiate failover
    pub async fn initiate_failover(&self) -> Result<()> {
        let mut state = self.state.write().await;

        // Check cooldown
        if let Some(last) = state.last_failover {
            let elapsed = (Utc::now() - last).num_seconds() as u64;
            if elapsed < self.config.cooldown_secs {
                tracing::warn!(
                    remaining = self.config.cooldown_secs - elapsed,
                    "Failover in cooldown period"
                );
                return Err(DRError::FailoverFailed("Cooldown period active".to_string()));
            }
        }

        // Check rate limit
        let cutoff = Utc::now() - chrono::Duration::hours(1);
        let recent_failovers = self.count_recent_failovers(cutoff).await;
        if recent_failovers >= self.config.max_failovers_per_hour {
            tracing::warn!(
                recent = recent_failovers,
                max = self.config.max_failovers_per_hour,
                "Failover rate limit exceeded"
            );
            return Err(DRError::FailoverFailed("Rate limit exceeded".to_string()));
        }

        // Check if already in progress
        if state.in_progress {
            return Err(DRError::FailoverFailed("Failover already in progress".to_string()));
        }

        // Find next region
        let current_idx = self.config.region_priority
            .iter()
            .position(|r| r == &state.primary_region)
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % self.config.region_priority.len();
        let next_region = self.config.region_priority.get(next_idx)
            .ok_or_else(|| DRError::NoHealthyRegions)?;

        tracing::info!(
            from = %state.primary_region,
            to = %next_region,
            "Initiating failover"
        );

        state.in_progress = true;
        let old_primary = state.primary_region.clone();
        state.primary_region = next_region.clone();
        state.secondary_region = Some(old_primary);
        state.last_failover = Some(Utc::now());
        state.failover_count += 1;
        state.in_progress = false;

        Ok(())
    }

    /// Manual failover to specific region
    pub async fn failover_to(&self, region: &str) -> Result<()> {
        let mut state = self.state.write().await;

        if !self.config.region_priority.contains(&region.to_string()) {
            return Err(DRError::RegionUnavailable(region.to_string()));
        }

        tracing::info!(
            from = %state.primary_region,
            to = %region,
            "Manual failover initiated"
        );

        let old_primary = state.primary_region.clone();
        state.primary_region = region.to_string();
        state.secondary_region = Some(old_primary);
        state.last_failover = Some(Utc::now());
        state.failover_count += 1;

        Ok(())
    }

    async fn count_recent_failovers(&self, _cutoff: DateTime<Utc>) -> u32 {
        // This would check a persistent log of failovers
        // For now, just return 0
        0
    }

    /// Get current primary region
    pub async fn get_primary(&self) -> String {
        self.state.read().await.primary_region.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_failover_manager() {
        let config = FailoverConfig::default();
        let manager = FailoverManager::new(config);
        
        let state = manager.get_state().await;
        assert_eq!(state.primary_region, "primary");
    }
}
