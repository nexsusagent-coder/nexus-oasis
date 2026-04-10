//! Health monitoring

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DRError, Result};

/// Health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Component name
    pub component: String,
    /// Health status
    pub status: HealthStatus,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Response time in ms
    pub response_time_ms: u64,
    /// Additional details
    pub details: HashMap<String, String>,
    /// Error message if unhealthy
    pub error: Option<String>,
}

/// Health check trait
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Name of the health check
    fn name(&self) -> &str;
    
    /// Run health check
    async fn check(&self) -> Result<HealthCheckResult>;
}

/// System health aggregator
pub struct HealthMonitor {
    checks: Arc<RwLock<Vec<Box<dyn HealthCheck>>>>,
    results: Arc<RwLock<HashMap<String, HealthCheckResult>>>,
    interval_secs: u64,
}

impl HealthMonitor {
    /// Create new health monitor
    pub fn new(interval_secs: u64) -> Self {
        Self {
            checks: Arc::new(RwLock::new(Vec::new())),
            results: Arc::new(RwLock::new(HashMap::new())),
            interval_secs,
        }
    }

    /// Add health check
    pub async fn add_check(&self, check: Box<dyn HealthCheck>) {
        let mut checks = self.checks.write().await;
        checks.push(check);
    }

    /// Run all health checks
    pub async fn run_checks(&self) -> Result<Vec<HealthCheckResult>> {
        let checks = self.checks.read().await;
        let mut results = Vec::new();

        for check in checks.iter() {
            let result = check.check().await?;
            results.push(result.clone());
            
            let mut stored = self.results.write().await;
            stored.insert(check.name().to_string(), result);
        }

        Ok(results)
    }

    /// Get overall health status
    pub async fn get_overall_status(&self) -> HealthStatus {
        let results = self.results.read().await;
        
        if results.is_empty() {
            return HealthStatus::Unknown;
        }

        let mut has_degraded = false;
        
        for result in results.values() {
            match result.status {
                HealthStatus::Unhealthy => return HealthStatus::Unhealthy,
                HealthStatus::Degraded => has_degraded = true,
                _ => {}
            }
        }

        if has_degraded {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Get health check result
    pub async fn get_result(&self, name: &str) -> Option<HealthCheckResult> {
        let results = self.results.read().await;
        results.get(name).cloned()
    }

    /// Get all results
    pub async fn get_all_results(&self) -> HashMap<String, HealthCheckResult> {
        let results = self.results.read().await;
        results.clone()
    }

    /// Start monitoring loop
    pub async fn start(&self) {
        let checks = self.checks.clone();
        let results = self.results.clone();
        let interval = self.interval_secs;

        tokio::spawn(async move {
            loop {
                let checks_guard = checks.read().await;
                
                for check in checks_guard.iter() {
                    match check.check().await {
                        Ok(result) => {
                            let mut stored = results.write().await;
                            stored.insert(check.name().to_string(), result);
                        }
                        Err(e) => {
                            tracing::error!(
                                check = %check.name(),
                                error = %e,
                                "Health check failed"
                            );
                        }
                    }
                }

                drop(checks_guard);
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
            }
        });
    }
}

/// HTTP health check
pub struct HttpHealthCheck {
    name: String,
    url: String,
    timeout_ms: u64,
    expected_status: u16,
}

impl HttpHealthCheck {
    pub fn new(name: String, url: String, timeout_ms: u64) -> Self {
        Self {
            name,
            url,
            timeout_ms,
            expected_status: 200,
        }
    }
}

#[async_trait]
impl HealthCheck for HttpHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> Result<HealthCheckResult> {
        let start = std::time::Instant::now();
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_millis(self.timeout_ms))
            .build()
            .map_err(|e| DRError::HealthCheckFailed(e.to_string()))?;

        let response = client.get(&self.url).send().await;
        
        let response_time_ms = start.elapsed().as_millis() as u64;
        
        match response {
            Ok(resp) => {
                let status = if resp.status().as_u16() == self.expected_status {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                };

                Ok(HealthCheckResult {
                    component: self.name.clone(),
                    status,
                    timestamp: Utc::now(),
                    response_time_ms,
                    details: HashMap::from([
                        ("url".to_string(), self.url.clone()),
                        ("status".to_string(), resp.status().to_string()),
                    ]),
                    error: None,
                })
            }
            Err(e) => Ok(HealthCheckResult {
                component: self.name.clone(),
                status: HealthStatus::Unhealthy,
                timestamp: Utc::now(),
                response_time_ms,
                details: HashMap::new(),
                error: Some(e.to_string()),
            }),
        }
    }
}

/// Database health check
pub struct DatabaseHealthCheck {
    name: String,
    connection_string: String,
}

impl DatabaseHealthCheck {
    pub fn new(name: String, connection_string: String) -> Self {
        Self { name, connection_string }
    }
}

#[async_trait]
impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }

    async fn check(&self) -> Result<HealthCheckResult> {
        let start = std::time::Instant::now();
        
        // Simple TCP connection test
        let parts: Vec<&str> = self.connection_string.split(':').collect();
        let host = parts.get(0).unwrap_or(&"localhost");
        let port: u16 = parts.get(1).unwrap_or(&"5432").parse().unwrap_or(5432);

        let addr = format!("{}:{}", host, port);
        let connected = tokio::net::TcpStream::connect(&addr).await.is_ok();
        
        let response_time_ms = start.elapsed().as_millis() as u64;
        
        Ok(HealthCheckResult {
            component: self.name.clone(),
            status: if connected { HealthStatus::Healthy } else { HealthStatus::Unhealthy },
            timestamp: Utc::now(),
            response_time_ms,
            details: HashMap::from([("address".to_string(), addr)]),
            error: if connected { None } else { Some("Connection failed".to_string()) },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor() {
        let monitor = HealthMonitor::new(30);
        let status = monitor.get_overall_status().await;
        assert_eq!(status, HealthStatus::Unknown);
    }
}
