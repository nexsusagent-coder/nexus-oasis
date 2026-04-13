//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT ANOMALY - Real-time Anomaly Detection Engine (Enterprise Grade 2026)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! AI ajanlarının güvenli çalışması için gerçek zamanlı anomali tespiti.
//!
//! ## Tespit Edilen Anomaliler:
//! - Sonsuz döngü (infinite loop)
//! - Davranış sapması (behavioral deviation)
//! - Kaynak anomalileri (CPU, memory, latency)

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - Çıktı anomalileri (repeated outputs)

pub mod detector;
pub mod metrics;
pub mod alert;
pub mod profile;
pub mod timeseries;

pub use detector::*;
pub use metrics::*;
pub use alert::*;
pub use profile::*;
pub use timeseries::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  ANOMALY TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Anomaly severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum AnomalySeverity {
    Info,
    #[default]
    Warning,
    High,
    Critical,
}

impl AnomalySeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnomalySeverity::Info => "INFO",
            AnomalySeverity::Warning => "WARNING",
            AnomalySeverity::High => "HIGH",
            AnomalySeverity::Critical => "CRITICAL",
        }
    }
}

/// Anomaly type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnomalyType {
    /// Infinite loop or repeated action pattern
    LoopPattern,
    /// Behavior deviates from learned profile
    BehaviorDeviation,
    /// Resource usage anomaly (CPU, memory)
    ResourceAnomaly,
    /// Response time anomaly
    LatencyAnomaly,
    /// Output pattern anomaly
    OutputAnomaly,
    /// Memory leak suspected
    MemoryLeak,
    /// CPU spike detected
    CpuSpike,
    /// Network communication anomaly
    NetworkAnomaly,
    /// Custom anomaly type
    Custom(String),
}

impl std::fmt::Display for AnomalyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnomalyType::LoopPattern => write!(f, "Loop Pattern"),
            AnomalyType::BehaviorDeviation => write!(f, "Behavior Deviation"),
            AnomalyType::ResourceAnomaly => write!(f, "Resource Anomaly"),
            AnomalyType::LatencyAnomaly => write!(f, "Latency Anomaly"),
            AnomalyType::OutputAnomaly => write!(f, "Output Anomaly"),
            AnomalyType::MemoryLeak => write!(f, "Memory Leak"),
            AnomalyType::CpuSpike => write!(f, "CPU Spike"),
            AnomalyType::NetworkAnomaly => write!(f, "Network Anomaly"),
            AnomalyType::Custom(name) => write!(f, "Custom: {}", name),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANOMALY STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

/// Detected anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Unique anomaly ID
    pub id: uuid::Uuid,
    
    /// Type of anomaly
    pub anomaly_type: AnomalyType,
    
    /// Severity level
    pub severity: AnomalySeverity,
    
    /// Associated agent ID
    pub agent_id: Option<String>,
    
    /// Metric name
    pub metric: String,
    
    /// Observed value
    pub observed_value: f64,
    
    /// Expected range (min, max)
    pub expected_range: (f64, f64),
    
    /// Z-score (number of standard deviations)
    pub z_score: f64,
    
    /// Detection timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Human-readable description
    pub description: String,
    
    /// Recommended remediation action
    pub recommended_action: Option<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Anomaly {
    pub fn new(
        anomaly_type: AnomalyType,
        severity: AnomalySeverity,
        metric: impl Into<String>,
        observed: f64,
        expected_range: (f64, f64),
        z_score: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            anomaly_type,
            severity,
            agent_id: None,
            metric: metric.into(),
            observed_value: observed,
            expected_range,
            z_score,
            timestamp: chrono::Utc::now(),
            description: String::new(),
            recommended_action: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Check if this is a critical anomaly
    pub fn is_critical(&self) -> bool {
        self.severity >= AnomalySeverity::High
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Anomaly detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// Z-score threshold for anomaly detection
    pub z_threshold: f64,
    
    /// Window size for time series analysis
    pub window_size: usize,
    
    /// Minimum samples before detection
    pub min_samples: usize,
    
    /// Detection interval in seconds
    pub detection_interval_secs: u64,
    
    /// Enable loop detection
    pub loop_detection: bool,
    
    /// Maximum action repetition before loop alert
    pub max_repetition: u32,
    
    /// Enable resource monitoring
    pub resource_monitoring: bool,
    
    /// Maximum CPU percentage
    pub max_cpu_percent: f64,
    
    /// Maximum memory percentage
    pub max_memory_percent: f64,
    
    /// Maximum response time in ms
    pub max_response_time_ms: u64,
    
    /// Enable behavior profiling
    pub behavior_profiling: bool,
    
    /// Auto-update behavior profiles
    pub auto_update_profile: bool,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            z_threshold: 3.0,
            window_size: 100,
            min_samples: 10,
            detection_interval_secs: 5,
            loop_detection: true,
            max_repetition: 5,
            resource_monitoring: true,
            max_cpu_percent: 90.0,
            max_memory_percent: 85.0,
            max_response_time_ms: 30000,
            behavior_profiling: true,
            auto_update_profile: true,
        }
    }
}

impl AnomalyConfig {
    /// Production configuration
    pub fn production() -> Self {
        Self {
            z_threshold: 2.5,
            window_size: 500,
            min_samples: 50,
            detection_interval_secs: 1,
            loop_detection: true,
            max_repetition: 3,
            resource_monitoring: true,
            max_cpu_percent: 80.0,
            max_memory_percent: 75.0,
            max_response_time_ms: 10000,
            behavior_profiling: true,
            auto_update_profile: false,
        }
    }
    
    /// Development configuration (more lenient)
    pub fn development() -> Self {
        Self {
            z_threshold: 4.0,
            window_size: 50,
            min_samples: 5,
            detection_interval_secs: 10,
            loop_detection: true,
            max_repetition: 10,
            resource_monitoring: true,
            max_cpu_percent: 95.0,
            max_memory_percent: 90.0,
            max_response_time_ms: 60000,
            behavior_profiling: false,
            auto_update_profile: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anomaly_creation() {
        let anomaly = Anomaly::new(
            AnomalyType::LoopPattern,
            AnomalySeverity::High,
            "test_metric",
            100.0,
            (0.0, 50.0),
            3.5,
        );
        
        assert!(anomaly.is_critical());
        assert!(anomaly.agent_id.is_none());
    }

    #[test]
    fn test_config_presets() {
        let prod = AnomalyConfig::production();
        assert_eq!(prod.z_threshold, 2.5);
        
        let dev = AnomalyConfig::development();
        assert_eq!(dev.z_threshold, 4.0);
    }
    
    #[test]
    fn test_severity_ordering() {
        assert!(AnomalySeverity::Critical > AnomalySeverity::High);
        assert!(AnomalySeverity::High > AnomalySeverity::Warning);
        assert!(AnomalySeverity::Warning > AnomalySeverity::Info);
    }
}
