//! ─── Performance Metrics ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Metrics collector
pub struct MetricsCollector {
    metrics: HashMap<String, Metric>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Self::initialize_metrics(),
        }
    }
    
    fn initialize_metrics() -> HashMap<String, Metric> {
        let mut metrics = HashMap::new();
        
        // API metrics
        metrics.insert("api.latency_p50".into(), Metric::new(
            "API Latency P50",
            MetricType::Latency,
            "ms",
        ));
        
        metrics.insert("api.latency_p99".into(), Metric::new(
            "API Latency P99",
            MetricType::Latency,
            "ms",
        ));
        
        metrics.insert("api.requests_per_second".into(), Metric::new(
            "API Requests/Second",
            MetricType::Throughput,
            "req/s",
        ));
        
        metrics.insert("api.error_rate".into(), Metric::new(
            "API Error Rate",
            MetricType::ErrorRate,
            "%",
        ));
        
        // System metrics
        metrics.insert("system.cpu_usage".into(), Metric::new(
            "CPU Usage",
            MetricType::Resource,
            "%",
        ));
        
        metrics.insert("system.memory_usage".into(), Metric::new(
            "Memory Usage",
            MetricType::Resource,
            "%",
        ));
        
        metrics.insert("system.disk_usage".into(), Metric::new(
            "Disk Usage",
            MetricType::Resource,
            "%",
        ));
        
        // SLA metrics
        metrics.insert("sla.uptime".into(), Metric::new(
            "Uptime",
            MetricType::Sla,
            "%",
        ));
        
        metrics.insert("sla.mttr".into(), Metric::new(
            "Mean Time To Resolution",
            MetricType::Sla,
            "hours",
        ));
        
        metrics.insert("sla.incident_count".into(), Metric::new(
            "Incident Count",
            MetricType::Sla,
            "count",
        ));
        
        metrics
    }
    
    /// Record metric value
    pub fn record(&mut self, name: &str, value: f64) {
        if let Some(metric) = self.metrics.get_mut(name) {
            metric.record(value);
        }
    }
    
    /// Get metric
    pub fn get(&self, name: &str) -> Option<&Metric> {
        self.metrics.get(name)
    }
    
    /// Get all metrics
    pub fn all(&self) -> &HashMap<String, Metric> {
        &self.metrics
    }
    
    /// Get metrics by type
    pub fn by_type(&self, metric_type: MetricType) -> Vec<(&String, &Metric)> {
        self.metrics.iter()
            .filter(|(_, m)| m.metric_type == metric_type)
            .collect()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Single metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub unit: String,
    pub current: f64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub count: u64,
    pub samples: Vec<MetricSample>,
    pub last_updated: Option<DateTime<Utc>>,
}

impl Metric {
    pub fn new(name: &str, metric_type: MetricType, unit: &str) -> Self {
        Self {
            name: name.into(),
            metric_type,
            unit: unit.into(),
            current: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            avg: 0.0,
            count: 0,
            samples: Vec::with_capacity(1000),
            last_updated: None,
        }
    }
    
    /// Record new value
    pub fn record(&mut self, value: f64) {
        self.current = value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.count += 1;
        
        // Running average
        self.avg = self.avg + (value - self.avg) / self.count as f64;
        
        // Store sample
        self.samples.push(MetricSample {
            timestamp: Utc::now(),
            value,
        });
        
        // Keep only last 1000 samples
        if self.samples.len() > 1000 {
            self.samples.remove(0);
        }
        
        self.last_updated = Some(Utc::now());
    }
}

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Latency,
    Throughput,
    ErrorRate,
    Resource,
    Sla,
    Custom,
}

/// Metric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSample {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        collector.record("api.latency_p50", 50.0);
        collector.record("api.latency_p50", 60.0);
        
        let metric = collector.get("api.latency_p50").unwrap();
        assert_eq!(metric.current, 60.0);
    }
}
