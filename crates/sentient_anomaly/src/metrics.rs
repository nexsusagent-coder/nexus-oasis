//! Metrics Collector

use serde::{Deserialize, Serialize};

/// Metrics collector
pub struct MetricsCollector {
    metrics: std::collections::HashMap<String, f64>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: std::collections::HashMap::new(),
        }
    }

    pub fn record(&mut self, name: &str, value: f64) {
        self.metrics.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).copied()
    }

    pub fn gather(&self) -> String {
        let mut output = String::new();
        for (name, value) in &self.metrics {
            output.push_str(&format!("{} {}\n", name, value));
        }
        output
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics() {
        let mut collector = MetricsCollector::new();
        collector.record("cpu", 45.5);
        assert_eq!(collector.get("cpu"), Some(45.5));
    }
}
