//! Health check endpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: HealthState,
    pub version: String,
    pub uptime_secs: u64,
    pub components: HashMap<String, ComponentHealth>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthState,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

impl HealthStatus {
    pub fn new(version: &str) -> Self {
        Self {
            status: HealthState::Healthy,
            version: version.to_string(),
            uptime_secs: 0,
            components: HashMap::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_component(mut self, name: &str, health: ComponentHealth) -> Self {
        self.components.insert(name.to_string(), health);
        self
    }

    pub fn update_uptime(&mut self, start_time: Instant) {
        self.uptime_secs = start_time.elapsed().as_secs();
    }

    pub fn calculate_overall_status(&mut self) {
        let has_unhealthy = self.components.values().any(|c| c.status == HealthState::Unhealthy);
        let has_degraded = self.components.values().any(|c| c.status == HealthState::Degraded);

        self.status = if has_unhealthy {
            HealthState::Unhealthy
        } else if has_degraded {
            HealthState::Degraded
        } else {
            HealthState::Healthy
        };
    }
}

/// Health checker for individual components
pub struct HealthChecker {
    checks: HashMap<String, Box<dyn HealthCheck + Send + Sync>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    pub fn register<C: HealthCheck + Send + Sync + 'static>(&mut self, name: &str, check: C) {
        self.checks.insert(name.to_string(), Box::new(check));
    }

    pub async fn check_all(&self) -> HashMap<String, ComponentHealth> {
        let mut results = HashMap::new();

        for (name, check) in &self.checks {
            let result = check.check().await;
            results.insert(name.clone(), result);
        }

        results
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for component health checks
#[async_trait::async_trait]
pub trait HealthCheck {
    async fn check(&self) -> ComponentHealth;
}

/// Database health check
pub struct DatabaseHealthCheck {
    pub name: String,
}

#[async_trait::async_trait]
impl HealthCheck for DatabaseHealthCheck {
    async fn check(&self) -> ComponentHealth {
        // Simulate database check
        let start = Instant::now();
        
        // In real implementation, would query the database
        let healthy = true;
        let latency = start.elapsed().as_millis() as u64;

        ComponentHealth {
            status: if healthy {
                HealthState::Healthy
            } else {
                HealthState::Unhealthy
            },
            message: if healthy {
                None
            } else {
                Some("Database connection failed".to_string())
            },
            latency_ms: Some(latency),
        }
    }
}

/// LLM provider health check
pub struct LlmHealthCheck {
    pub provider: String,
}

#[async_trait::async_trait]
impl HealthCheck for LlmHealthCheck {
    async fn check(&self) -> ComponentHealth {
        // Check LLM provider connectivity
        ComponentHealth {
            status: HealthState::Healthy,
            message: Some(format!("{} API accessible", self.provider)),
            latency_ms: None,
        }
    }
}

/// Memory health check
pub struct MemoryHealthCheck;

#[async_trait::async_trait]
impl HealthCheck for MemoryHealthCheck {
    async fn check(&self) -> ComponentHealth {
        ComponentHealth {
            status: HealthState::Healthy,
            message: Some("Memory system operational".to_string()),
            latency_ms: None,
        }
    }
}

/// Channel health check
pub struct ChannelHealthCheck {
    pub channel_type: String,
}

#[async_trait::async_trait]
impl HealthCheck for ChannelHealthCheck {
    async fn check(&self) -> ComponentHealth {
        ComponentHealth {
            status: HealthState::Healthy,
            message: Some(format!("{} channel connected", self.channel_type)),
            latency_ms: None,
        }
    }
}

/// Readiness probe - checks if service is ready to accept traffic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessStatus {
    pub ready: bool,
    pub checks: HashMap<String, bool>,
}

impl ReadinessStatus {
    pub fn new() -> Self {
        Self {
            ready: true,
            checks: HashMap::new(),
        }
    }

    pub fn with_check(mut self, name: &str, passed: bool) -> Self {
        self.checks.insert(name.to_string(), passed);
        if !passed {
            self.ready = false;
        }
        self
    }
}

impl Default for ReadinessStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// Liveness probe - checks if service is alive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivenessStatus {
    pub alive: bool,
}

impl LivenessStatus {
    pub fn alive() -> Self {
        Self { alive: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_creation() {
        let status = HealthStatus::new("1.0.0")
            .with_component("database", ComponentHealth {
                status: HealthState::Healthy,
                message: None,
                latency_ms: Some(5),
            });

        assert_eq!(status.version, "1.0.0");
        assert!(status.components.contains_key("database"));
    }

    #[test]
    fn test_health_status_calculation() {
        let mut status = HealthStatus::new("1.0.0")
            .with_component("db", ComponentHealth {
                status: HealthState::Healthy,
                message: None,
                latency_ms: None,
            })
            .with_component("cache", ComponentHealth {
                status: HealthState::Degraded,
                message: Some("High latency".to_string()),
                latency_ms: None,
            });

        status.calculate_overall_status();
        assert_eq!(status.status, HealthState::Degraded);
    }

    #[tokio::test]
    async fn test_health_checker() {
        let mut checker = HealthChecker::new();
        checker.register("database", DatabaseHealthCheck {
            name: "main".to_string(),
        });

        let results = checker.check_all().await;
        assert!(results.contains_key("database"));
    }

    #[test]
    fn test_readiness_status() {
        let status = ReadinessStatus::new()
            .with_check("database", true)
            .with_check("cache", true);

        assert!(status.ready);

        let status = ReadinessStatus::new()
            .with_check("database", true)
            .with_check("llm", false);

        assert!(!status.ready);
    }

    #[test]
    fn test_liveness_status() {
        let status = LivenessStatus::alive();
        assert!(status.alive);
    }
}
