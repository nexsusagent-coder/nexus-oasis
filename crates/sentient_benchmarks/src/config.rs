//! Benchmark configuration

use serde::{Deserialize, Serialize};

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of iterations for each benchmark
    pub iterations: u64,

    /// Number of warmup iterations
    pub warmup_iterations: u64,

    /// Output format
    pub output_format: OutputFormat,

    /// Compare against baseline
    pub baseline: Option<String>,

    /// Save results
    pub save_results: bool,

    /// Results directory
    pub results_dir: String,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            output_format: OutputFormat::Json,
            baseline: None,
            save_results: true,
            results_dir: "target/benchmarks".to_string(),
        }
    }
}

/// Output format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Html,
    Markdown,
    Csv,
}

impl BenchmarkConfig {
    /// Quick benchmark (few iterations)
    pub fn quick() -> Self {
        Self {
            iterations: 100,
            warmup_iterations: 10,
            ..Default::default()
        }
    }

    /// Standard benchmark
    pub fn standard() -> Self {
        Self::default()
    }

    /// Thorough benchmark (many iterations)
    pub fn thorough() -> Self {
        Self {
            iterations: 10000,
            warmup_iterations: 1000,
            ..Default::default()
        }
    }

    /// CI benchmark (deterministic)
    pub fn ci() -> Self {
        Self {
            iterations: 500,
            warmup_iterations: 50,
            output_format: OutputFormat::Json,
            save_results: true,
            ..Default::default()
        }
    }
}
