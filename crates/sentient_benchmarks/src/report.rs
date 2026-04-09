//! Benchmark reporting

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::{Metrics, SystemInfo};
use crate::config::BenchmarkConfig;

/// Complete benchmark report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// System information
    pub system_info: SystemInfo,

    /// Configuration used
    pub config: BenchmarkConfig,

    /// Benchmark results
    pub results: Vec<Metrics>,

    /// Comparison with baseline (if any)
    pub comparison: Option<ComparisonResult>,
}

impl BenchmarkReport {
    /// Save report to file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load report from file
    pub fn load(path: &Path) -> std::io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let report = serde_json::from_str(&json)?;
        Ok(report)
    }

    /// Generate markdown summary
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# SENTIENT Performance Benchmark Report\n\n");
        md.push_str(&format!("**Date**: {}\n\n", self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        md.push_str(&format!("**Version**: {}\n\n", self.system_info.sentient_version));

        md.push_str("## System Information\n\n");
        md.push_str(&format!("- **OS**: {}\n", self.system_info.os));
        md.push_str(&format!("- **CPU**: {} ({} cores)\n", self.system_info.cpu_model, self.system_info.cpu_cores));
        md.push_str(&format!("- **Memory**: {:.2} GB\n", self.system_info.total_memory as f64 / 1e9));
        md.push_str(&format!("- **Rust**: {}\n\n", self.system_info.rust_version));

        md.push_str("## Results\n\n");
        md.push_str("| Operation | Iterations | Mean (μs) | Median (μs) | Std Dev (μs) | Ops/sec |\n");
        md.push_str("|-----------|------------|-----------|-------------|--------------|--------|\n");

        for result in &self.results {
            md.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {:.2} | {:.0} |\n",
                result.operation,
                result.iterations,
                result.mean_duration_us,
                result.median_duration_us,
                result.std_dev_us,
                result.ops_per_second,
            ));
        }

        if let Some(ref comparison) = self.comparison {
            md.push_str("\n## Comparison with Baseline\n\n");
            md.push_str("| Operation | Baseline (μs) | Current (μs) | Change |\n");
            md.push_str("|-----------|---------------|--------------|--------|\n");

            for (op, change) in &comparison.changes {
                md.push_str(&format!(
                    "| {} | {:.2} | {:.2} | {}{:.1}% |\n",
                    op,
                    change.baseline_us,
                    change.current_us,
                    if change.percent_change >= 0.0 { "+" } else { "" },
                    change.percent_change,
                ));
            }
        }

        md
    }

    /// Generate summary
    pub fn summary(&self) -> String {
        let total_ops: f64 = self.results.iter().map(|r| r.ops_per_second).sum();
        let avg_latency: f64 = self.results.iter()
            .map(|r| r.mean_duration_us)
            .sum::<f64>() / self.results.len() as f64;

        format!(
            "Benchmark Summary:\n\
             - Total operations tested: {}\n\
             - Total ops/sec: {:.0}\n\
             - Average latency: {:.2} μs\n\
             - System: {} ({} cores)",
            self.results.len(),
            total_ops,
            avg_latency,
            self.system_info.os,
            self.system_info.cpu_cores,
        )
    }
}

/// Result change between baseline and current
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeResult {
    /// Operation name
    pub operation: String,

    /// Baseline duration in microseconds
    pub baseline_us: f64,

    /// Current duration in microseconds
    pub current_us: f64,

    /// Percent change (positive = slower, negative = faster)
    pub percent_change: f64,

    /// Is this a regression?
    pub is_regression: bool,
}

/// Comparison result between two benchmark runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Baseline timestamp
    pub baseline_timestamp: DateTime<Utc>,

    /// Changes for each operation
    pub changes: HashMap<String, ChangeResult>,

    /// Summary
    pub summary: ComparisonSummary,
}

/// Summary of comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    /// Number of improvements
    pub improvements: usize,

    /// Number of regressions
    pub regressions: usize,

    /// Number of neutral changes
    pub neutral: usize,

    /// Average change percentage
    pub avg_change_percent: f64,

    /// Overall verdict
    pub verdict: Verdict,
}

/// Comparison verdict
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Verdict {
    Improved,
    Regressed,
    Neutral,
    Mixed,
}

impl BenchmarkReport {
    /// Compare with baseline report
    pub fn compare(&self, baseline: &BenchmarkReport) -> ComparisonResult {
        let mut changes = HashMap::new();
        let mut improvements = 0;
        let mut regressions = 0;
        let mut neutral = 0;
        let mut total_change = 0.0;

        for current in &self.results {
            if let Some(base) = baseline.results.iter().find(|r| r.operation == current.operation) {
                let percent_change = ((current.mean_duration_us - base.mean_duration_us)
                    / base.mean_duration_us) * 100.0;

                let is_regression = percent_change > 10.0; // >10% slower is regression

                if percent_change < -5.0 {
                    improvements += 1;
                } else if percent_change > 5.0 {
                    regressions += 1;
                } else {
                    neutral += 1;
                }

                total_change += percent_change;

                changes.insert(current.operation.clone(), ChangeResult {
                    operation: current.operation.clone(),
                    baseline_us: base.mean_duration_us,
                    current_us: current.mean_duration_us,
                    percent_change,
                    is_regression,
                });
            }
        }

        let avg_change = if changes.len() > 0 {
            total_change / changes.len() as f64
        } else {
            0.0
        };

        let verdict = if improvements > regressions && regressions == 0 {
            Verdict::Improved
        } else if regressions > improvements && improvements == 0 {
            Verdict::Regressed
        } else if improvements == 0 && regressions == 0 {
            Verdict::Neutral
        } else {
            Verdict::Mixed
        };

        ComparisonResult {
            baseline_timestamp: baseline.timestamp,
            changes,
            summary: ComparisonSummary {
                improvements,
                regressions,
                neutral,
                avg_change_percent: avg_change,
                verdict,
            },
        }
    }
}

/// Benchmark result for a single operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Operation name
    pub name: String,

    /// Duration
    pub duration_us: u64,

    /// Success
    pub success: bool,

    /// Error message (if failed)
    pub error: Option<String>,
}
