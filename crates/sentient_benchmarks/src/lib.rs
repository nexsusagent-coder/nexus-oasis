//! Performance benchmarks for SENTIENT AI OS
//!
//! Provides comprehensive benchmarking suite for:
//! - Memory operations
//! - Agent execution
//! - Channel processing
//! - Voice processing
//! - Latency measurements
//! - Throughput measurements

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

pub mod memory;
pub mod agent;
pub mod channel;
pub mod voice;
pub mod latency;
pub mod throughput;
pub mod report;
pub mod config;

pub use config::BenchmarkConfig;
pub use report::{BenchmarkReport, BenchmarkResult, ComparisonResult};

use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Benchmark metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    /// Operation name
    pub operation: String,

    /// Number of iterations
    pub iterations: u64,

    /// Total duration in microseconds
    pub total_duration_us: u64,

    /// Mean duration in microseconds
    pub mean_duration_us: f64,

    /// Median duration in microseconds
    pub median_duration_us: f64,

    /// Standard deviation in microseconds
    pub std_dev_us: f64,

    /// Min duration in microseconds
    pub min_duration_us: u64,

    /// Max duration in microseconds
    pub max_duration_us: u64,

    /// Operations per second
    pub ops_per_second: f64,

    /// Memory usage in bytes
    pub memory_bytes: Option<u64>,

    /// CPU usage percentage
    pub cpu_percent: Option<f64>,
}

impl Metrics {
    /// Calculate metrics from durations
    pub fn from_durations(operation: &str, durations: &[u64]) -> Self {
        use statrs::statistics::Statistics;

        let iterations = durations.len() as u64;
        let total: u64 = durations.iter().sum();
        let values: Vec<f64> = durations.iter().map(|&d| d as f64).collect();
        let mean = values.iter().map(|&d| d).mean();
        let median = {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).expect("operation failed"));
            if sorted.len() % 2 == 0 {
                (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
            } else {
                sorted[sorted.len() / 2]
            }
        };
        let std_dev = values.iter().map(|&d| d).std_dev();
        let min = durations.iter().min().copied().unwrap_or(0);
        let max = durations.iter().max().copied().unwrap_or(0);

        let ops_per_second = if total > 0 {
            (iterations as f64) / (total as f64 / 1_000_000.0)
        } else {
            0.0
        };

        Self {
            operation: operation.to_string(),
            iterations,
            total_duration_us: total,
            mean_duration_us: mean,
            median_duration_us: median,
            std_dev_us: std_dev,
            min_duration_us: min,
            max_duration_us: max,
            ops_per_second,
            memory_bytes: None,
            cpu_percent: None,
        }
    }
}

/// System information for benchmark context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// OS name
    pub os: String,

    /// CPU model
    pub cpu_model: String,

    /// CPU cores
    pub cpu_cores: usize,

    /// Total memory in bytes
    pub total_memory: u64,

    /// Rust version
    pub rust_version: String,

    /// SENTIENT version
    pub sentient_version: String,
}

impl SystemInfo {
    /// Collect system information
    pub fn collect() -> Self {
        use sysinfo::System;

        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: System::name().unwrap_or_else(|| "Unknown".to_string()),
            cpu_model: sys.cpus().first()
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            cpu_cores: sys.cpus().len(),
            total_memory: sys.total_memory(),
            rust_version: rustc_version_runtime::version().to_string(),
            sentient_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    results: Vec<Metrics>,
}

impl BenchmarkRunner {
    /// Create a new benchmark runner
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Run a benchmark
    pub async fn run<F, Fut>(&mut self, name: &str, mut f: F) -> Metrics
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            f().await;
        }

        // Measure
        let mut durations = Vec::with_capacity(self.config.iterations as usize);

        for _ in 0..self.config.iterations {
            let start = std::time::Instant::now();
            f().await;
            let elapsed = start.elapsed().as_micros() as u64;
            durations.push(elapsed);
        }

        let metrics = Metrics::from_durations(name, &durations);
        self.results.push(metrics.clone());
        metrics
    }

    /// Get all results
    pub fn results(&self) -> &[Metrics] {
        &self.results
    }

    /// Generate report
    pub fn report(&self) -> BenchmarkReport {
        BenchmarkReport {
            timestamp: Utc::now(),
            system_info: SystemInfo::collect(),
            config: self.config.clone(),
            results: self.results.clone(),
            comparison: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_runner() {
        let config = BenchmarkConfig::quick();
        let mut runner = BenchmarkRunner::new(config);

        let metrics = runner.run("test_op", || async {
            tokio::time::sleep(std::time::Duration::from_micros(100)).await;
        }).await;

        assert_eq!(metrics.operation, "test_op");
        assert_eq!(metrics.iterations, 100);
    }

    #[test]
    fn test_system_info() {
        let info = SystemInfo::collect();
        assert!(!info.os.is_empty());
        assert!(info.cpu_cores > 0);
    }
}
