//! Multi-region management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DRError, HealthStatus, Result};

/// Region configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionConfig {
    /// Region identifier
    pub id: String,
    /// Region name
    pub name: String,
    /// Region endpoint
    pub endpoint: String,
    /// Priority (lower = higher priority)
    pub priority: u32,
    /// Is region active
    pub active: bool,
    /// Latency in ms
    pub latency_ms: Option<u64>,
    /// Region-specific settings
    pub settings: HashMap<String, String>,
}

/// Region status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionStatus {
    /// Region ID
    pub id: String,
    /// Health status
    pub health: HealthStatus,
    /// Last health check
    pub last_check: Option<DateTime<Utc>>,
    /// Current latency
    pub latency_ms: Option<u64>,
    /// Is primary region
    pub is_primary: bool,
    /// Is accepting traffic
    pub accepting_traffic: bool,
    /// Error count
    pub error_count: u64,
}

/// Region manager
pub struct RegionManager {
    regions: Arc<RwLock<HashMap<String, RegionConfig>>>,
    statuses: Arc<RwLock<HashMap<String, RegionStatus>>>,
    primary: Arc<RwLock<Option<String>>>,
}

impl RegionManager {
    /// Create new region manager
    pub fn new() -> Self {
        Self {
            regions: Arc::new(RwLock::new(HashMap::new())),
            statuses: Arc::new(RwLock::new(HashMap::new())),
            primary: Arc::new(RwLock::new(None)),
        }
    }

    /// Add region
    pub async fn add_region(&self, config: RegionConfig) -> Result<()> {
        let id = config.id.clone();
        let is_first = {
            let regions = self.regions.read().await;
            regions.is_empty()
        };

        let mut regions = self.regions.write().await;
        regions.insert(id.clone(), config.clone());

        let mut statuses = self.statuses.write().await;
        statuses.insert(id.clone(), RegionStatus {
            id: id.clone(),
            health: HealthStatus::Unknown,
            last_check: None,
            latency_ms: None,
            is_primary: is_first,
            accepting_traffic: is_first,
            error_count: 0,
        });

        if is_first {
            let mut primary = self.primary.write().await;
            *primary = Some(id.clone());
        }

        tracing::info!(region_id = %id, "Region added");
        Ok(())
    }

    /// Remove region
    pub async fn remove_region(&self, id: &str) -> Result<()> {
        let mut regions = self.regions.write().await;
        let mut statuses = self.statuses.write().await;

        regions.remove(id);
        statuses.remove(id);

        // Update primary if removed
        let mut primary = self.primary.write().await;
        if primary.as_deref() == Some(id) {
            *primary = regions.keys().next().cloned();
        }

        tracing::info!(region_id = %id, "Region removed");
        Ok(())
    }

    /// Update region status
    pub async fn update_status(&self, id: &str, health: HealthStatus, latency_ms: Option<u64>) {
        let mut statuses = self.statuses.write().await;
        if let Some(status) = statuses.get_mut(id) {
            status.health = health;
            status.last_check = Some(Utc::now());
            status.latency_ms = latency_ms;
        }
    }

    /// Get region by ID
    pub async fn get_region(&self, id: &str) -> Option<RegionConfig> {
        let regions = self.regions.read().await;
        regions.get(id).cloned()
    }

    /// Get region status
    pub async fn get_status(&self, id: &str) -> Option<RegionStatus> {
        let statuses = self.statuses.read().await;
        statuses.get(id).cloned()
    }

    /// Get primary region
    pub async fn get_primary(&self) -> Option<(RegionConfig, RegionStatus)> {
        let primary_id = self.primary.read().await.clone();
        
        if let Some(id) = primary_id {
            let regions = self.regions.read().await;
            let statuses = self.statuses.read().await;
            
            let config = regions.get(&id).cloned();
            let status = statuses.get(&id).cloned();
            
            match (config, status) {
                (Some(c), Some(s)) => Some((c, s)),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Get all regions
    pub async fn list_regions(&self) -> Vec<RegionConfig> {
        let regions = self.regions.read().await;
        regions.values().cloned().collect()
    }

    /// Get healthy regions
    pub async fn get_healthy_regions(&self) -> Vec<RegionConfig> {
        let regions = self.regions.read().await;
        let statuses = self.statuses.read().await;

        regions.values()
            .filter(|r| {
                statuses.get(&r.id)
                    .map(|s| s.health == HealthStatus::Healthy)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }

    /// Get next failover target
    pub async fn get_failover_target(&self) -> Option<RegionConfig> {
        let primary = self.primary.read().await.clone();
        let regions = self.regions.read().await;
        let statuses = self.statuses.read().await;

        // Find healthy region with lowest priority (excluding primary)
        regions.values()
            .filter(|r| Some(&r.id) != primary.as_ref())
            .filter(|r| r.active)
            .filter(|r| {
                statuses.get(&r.id)
                    .map(|s| s.health == HealthStatus::Healthy)
                    .unwrap_or(false)
            })
            .min_by_key(|r| r.priority)
            .cloned()
    }

    /// Set primary region
    pub async fn set_primary(&self, id: &str) -> Result<()> {
        let regions = self.regions.read().await;
        
        if !regions.contains_key(id) {
            return Err(DRError::RegionUnavailable(id.to_string()));
        }

        let mut primary = self.primary.write().await;
        let old_primary = primary.clone();
        *primary = Some(id.to_string());

        let mut statuses = self.statuses.write().await;
        if let Some(old_id) = old_primary {
            if let Some(status) = statuses.get_mut(&old_id) {
                status.is_primary = false;
            }
        }
        if let Some(status) = statuses.get_mut(id) {
            status.is_primary = true;
        }

        tracing::info!(region_id = %id, "Primary region changed");
        Ok(())
    }
}

impl Default for RegionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_region_manager() {
        let manager = RegionManager::new();
        
        let config = RegionConfig {
            id: "us-east-1".to_string(),
            name: "US East".to_string(),
            endpoint: "https://us-east.sentient-os.ai".to_string(),
            priority: 1,
            active: true,
            latency_ms: None,
            settings: HashMap::new(),
        };

        manager.add_region(config).await.unwrap();
        
        let regions = manager.list_regions().await;
        assert_eq!(regions.len(), 1);
        
        let primary = manager.get_primary().await;
        assert!(primary.is_some());
    }
}
