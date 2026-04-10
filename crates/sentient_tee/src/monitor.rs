//! TEE Monitor - Real-time Security Monitoring
//!
//! Provides continuous monitoring of TEE health and security status.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

/// Security event type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEvent {
    TeeInitialized,
    AttestationPassed,
    AttestationFailed,
    SecurityViolation,
    MemoryThresholdExceeded,
    HeartbeatMissed,
    PlatformDegraded,
    SealingFailure,
    UnsealingFailure,
}

/// Security log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLogEntry {
    pub event: SecurityEvent,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub severity: Severity,
}

/// Event severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// Monitoring metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeeMetrics {
    pub total_attestations: u64,
    pub failed_attestations: u64,
    pub security_violations: u64,
    pub memory_operations: u64,
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    pub uptime_seconds: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Enable periodic health checks
    pub health_check_enabled: bool,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Enable anomaly detection
    pub anomaly_detection_enabled: bool,
    /// Max allowed failed attestations before alert
    pub max_failed_attestations: u32,
    /// Heartbeat timeout in seconds
    pub heartbeat_timeout_secs: u64,
    /// Enable metrics collection
    pub metrics_enabled: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            health_check_enabled: true,
            health_check_interval_secs: 30,
            anomaly_detection_enabled: true,
            max_failed_attestations: 3,
            heartbeat_timeout_secs: 60,
            metrics_enabled: true,
        }
    }
}

/// Alert callback type
pub type AlertCallback = Box<dyn Fn(SecurityEvent, &str) + Send + Sync>;

/// TEE Monitor with real-time monitoring capabilities
pub struct TeeMonitor {
    log: Arc<RwLock<Vec<SecurityLogEntry>>>,
    metrics: Arc<RwLock<TeeMetrics>>,
    config: MonitorConfig,
    start_time: Instant,
    alert_callbacks: Arc<RwLock<Vec<AlertCallback>>>,
    last_health_check: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl TeeMonitor {
    pub fn new() -> Self {
        Self::with_config(MonitorConfig::default())
    }

    pub fn with_config(config: MonitorConfig) -> Self {
        Self {
            log: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(TeeMetrics::default())),
            config,
            start_time: Instant::now(),
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
            last_health_check: Arc::new(RwLock::new(None)),
        }
    }

    /// Log a security event
    pub async fn log_event(&self, event: SecurityEvent, message: impl Into<String>) {
        self.log_event_with_severity(event, message, Severity::Info).await;
    }

    /// Log event with specific severity
    pub async fn log_event_with_severity(
        &self,
        event: SecurityEvent,
        message: impl Into<String>,
        severity: Severity,
    ) {
        let entry = SecurityLogEntry {
            event,
            timestamp: chrono::Utc::now(),
            message: message.into(),
            severity,
        };

        // Update metrics
        self.update_metrics(&entry).await;

        // Log the entry
        let mut log = self.log.write().await;
        log.push(entry.clone());

        // Trigger alerts for critical events
        if matches!(severity, Severity::Critical | Severity::Warning) {
            self.trigger_alert(event, &entry.message).await;
        }

        // Log to system
        match severity {
            Severity::Info => log::info!("TEE Monitor: {:?} - {}", event, entry.message),
            Severity::Warning => log::warn!("TEE Monitor: {:?} - {}", event, entry.message),
            Severity::Critical => log::error!("TEE Monitor: {:?} - {}", event, entry.message),
        }
    }

    /// Update metrics based on event
    async fn update_metrics(&self, entry: &SecurityLogEntry) {
        if !self.config.metrics_enabled {
            return;
        }

        let mut metrics = self.metrics.write().await;
        metrics.uptime_seconds = self.start_time.elapsed().as_secs();

        match entry.event {
            SecurityEvent::AttestationPassed => metrics.total_attestations += 1,
            SecurityEvent::AttestationFailed => {
                metrics.total_attestations += 1;
                metrics.failed_attestations += 1;
            }
            SecurityEvent::SecurityViolation => metrics.security_violations += 1,
            SecurityEvent::SealingFailure | SecurityEvent::UnsealingFailure => {
                metrics.memory_operations += 1;
            }
            _ => {}
        }

        metrics.last_heartbeat = Some(chrono::Utc::now());
    }

    /// Register an alert callback
    pub async fn register_alert_callback(&self, callback: AlertCallback) {
        let mut callbacks = self.alert_callbacks.write().await;
        callbacks.push(callback);
    }

    /// Trigger alert callbacks
    async fn trigger_alert(&self, event: SecurityEvent, message: &str) {
        let callbacks = self.alert_callbacks.read().await;
        for callback in callbacks.iter() {
            callback(event, message);
        }
    }

    /// Perform health check
    pub async fn health_check(&self) -> HealthStatus {
        let metrics = self.metrics.read().await;
        let last_check = self.last_health_check.read().await;

        let mut status = HealthStatus::new();

        // Check heartbeat
        if let Some(last_heartbeat) = metrics.last_heartbeat {
            let elapsed = (chrono::Utc::now() - last_heartbeat).num_seconds() as u64;
            if elapsed > self.config.heartbeat_timeout_secs {
                status.issues.push(format!("Heartbeat timeout: {}s", elapsed));
                status.healthy = false;
            }
        }

        // Check attestation failure rate
        if self.config.anomaly_detection_enabled && metrics.total_attestations > 0 {
            let failure_rate = metrics.failed_attestations as f64 / metrics.total_attestations as f64;
            if failure_rate > 0.1 {
                status.issues.push(format!("High attestation failure rate: {:.1}%", failure_rate * 100.0));
                status.healthy = false;
            }
        }

        // Check security violations
        if metrics.security_violations > 0 {
            status.issues.push(format!("{} security violations detected", metrics.security_violations));
            status.warnings += 1;
        }

        status.metrics = Some(metrics.clone());
        status
    }

    /// Run periodic health check (call this in a background task)
    pub async fn run_health_check_loop(&self) {
        if !self.config.health_check_enabled {
            return;
        }

        let interval = Duration::from_secs(self.config.health_check_interval_secs);
        
        loop {
            tokio::time::sleep(interval).await;

            let status = self.health_check().await;

            if !status.healthy {
                self.log_event_with_severity(
                    SecurityEvent::PlatformDegraded,
                    format!("Health check failed: {:?}", status.issues),
                    Severity::Warning,
                ).await;
            }

            let mut last_check = self.last_health_check.write().await;
            *last_check = Some(chrono::Utc::now());
        }
    }

    /// Get all log entries
    pub async fn entries(&self) -> Vec<SecurityLogEntry> {
        self.log.read().await.clone()
    }

    /// Get recent entries (last N)
    pub async fn recent_entries(&self, n: usize) -> Vec<SecurityLogEntry> {
        let log = self.log.read().await;
        let start = log.len().saturating_sub(n);
        log[start..].to_vec()
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> TeeMetrics {
        self.metrics.read().await.clone()
    }

    /// Clear old entries (keep last N)
    pub async fn trim_log(&self, keep: usize) {
        let mut log = self.log.write().await;
        if log.len() > keep {
            let start = log.len() - keep;
            *log = log.split_off(start);
        }
    }

    /// Check for anomalies
    pub async fn detect_anomalies(&self) -> Vec<Anomaly> {
        if !self.config.anomaly_detection_enabled {
            return Vec::new();
        }

        let metrics = self.metrics.read().await;
        let mut anomalies = Vec::new();

        // Check for high failure rate
        if metrics.total_attestations > 10 {
            let failure_rate = metrics.failed_attestations as f64 / metrics.total_attestations as f64;
            if failure_rate > 0.5 {
                anomalies.push(Anomaly {
                    kind: AnomalyKind::HighAttestationFailure,
                    severity: Severity::Critical,
                    description: format!("Attestation failure rate: {:.1}%", failure_rate * 100.0),
                });
            }
        }

        // Check for security violations
        if metrics.security_violations > 5 {
            anomalies.push(Anomaly {
                kind: AnomalyKind::SecurityViolationSpike,
                severity: Severity::Warning,
                description: format!("{} security violations detected", metrics.security_violations),
            });
        }

        // Check heartbeat
        if let Some(last_heartbeat) = metrics.last_heartbeat {
            let elapsed = (chrono::Utc::now() - last_heartbeat).num_seconds() as u64;
            if elapsed > self.config.heartbeat_timeout_secs / 2 {
                anomalies.push(Anomaly {
                    kind: AnomalyKind::StaleHeartbeat,
                    severity: Severity::Warning,
                    description: format!("Heartbeat {}s old", elapsed),
                });
            }
        }

        anomalies
    }
}

impl Default for TeeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthStatus {
    #[serde(default = "default_healthy")]
    pub healthy: bool,
    pub warnings: u32,
    pub issues: Vec<String>,
    pub metrics: Option<TeeMetrics>,
}

fn default_healthy() -> bool { true }

impl HealthStatus {
    /// Create a new health status (default healthy)
    pub fn new() -> Self {
        Self {
            healthy: true,
            warnings: 0,
            issues: Vec::new(),
            metrics: None,
        }
    }
}

/// Detected anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub kind: AnomalyKind,
    pub severity: Severity,
    pub description: String,
}

/// Anomaly types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnomalyKind {
    HighAttestationFailure,
    SecurityViolationSpike,
    StaleHeartbeat,
    MemoryThresholdExceeded,
    PlatformDegraded,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor() {
        let monitor = TeeMonitor::new();
        monitor.log_event(SecurityEvent::TeeInitialized, "TEE started").await;
        
        let entries = monitor.entries().await;
        assert_eq!(entries.len(), 1);
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let monitor = TeeMonitor::new();
        
        monitor.log_event(SecurityEvent::AttestationPassed, "OK").await;
        monitor.log_event(SecurityEvent::AttestationFailed, "Failed").await;
        monitor.log_event(SecurityEvent::AttestationFailed, "Failed again").await;

        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.total_attestations, 3);
        assert_eq!(metrics.failed_attestations, 2);
    }

    #[tokio::test]
    async fn test_health_check() {
        let monitor = TeeMonitor::new();
        
        let status = monitor.health_check().await;
        assert!(status.healthy);
    }

    #[tokio::test]
    async fn test_anomaly_detection() {
        let monitor = TeeMonitor::new();
        
        // Generate some failures
        for _ in 0..15 {
            monitor.log_event(SecurityEvent::AttestationFailed, "fail").await;
        }

        let anomalies = monitor.detect_anomalies().await;
        assert!(!anomalies.is_empty());
    }
}
