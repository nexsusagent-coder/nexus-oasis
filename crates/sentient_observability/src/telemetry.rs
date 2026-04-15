//! Telemetry Metrics for SENTIENT
//! 
//! GPU, Energy, and FLOPs tracking inspired by OpenJarvis Intelligence Per Watt research.

use serde::{Deserialize, Serialize};
use std::time::Instant;

/// GPU telemetry data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuTelemetry {
    /// GPU device name
    pub device_name: String,
    /// GPU index
    pub device_index: usize,
    /// GPU utilization percentage (0-100)
    pub utilization: f32,
    /// Memory used in MB
    pub memory_used_mb: u64,
    /// Total memory in MB
    pub memory_total_mb: u64,
    /// Temperature in Celsius
    pub temperature_c: f32,
    /// Power draw in Watts
    pub power_w: f32,
    /// Power limit in Watts
    pub power_limit_w: f32,
    /// Timestamp
    pub timestamp_ms: u64,
}

impl Default for GpuTelemetry {
    fn default() -> Self {
        Self {
            device_name: "Unknown".to_string(),
            device_index: 0,
            utilization: 0.0,
            memory_used_mb: 0,
            memory_total_mb: 0,
            temperature_c: 0.0,
            power_w: 0.0,
            power_limit_w: 0.0,
            timestamp_ms: 0,
        }
    }
}

impl GpuTelemetry {
    /// Memory utilization percentage
    pub fn memory_utilization(&self) -> f32 {
        if self.memory_total_mb == 0 {
            return 0.0;
        }
        (self.memory_used_mb as f32 / self.memory_total_mb as f32) * 100.0
    }

    /// Power utilization percentage
    pub fn power_utilization(&self) -> f32 {
        if self.power_limit_w == 0.0 {
            return 0.0;
        }
        (self.power_w / self.power_limit_w) * 100.0
    }
}

/// Energy telemetry data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnergyTelemetry {
    /// Energy consumed in Joules
    pub energy_j: f64,
    /// Average power in Watts
    pub avg_power_w: f32,
    /// Peak power in Watts
    pub peak_power_w: f32,
    /// Duration in seconds
    pub duration_s: f64,
    /// Carbon footprint estimate in gCO2 (if region data available)
    pub carbon_gco2: Option<f64>,
    /// Timestamp
    pub timestamp_ms: u64,
}

/// FLOPs telemetry data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlopsTelemetry {
    /// Total FLOPs (floating point operations)
    pub total_flops: u64,
    /// FLOPs per second
    pub flops_per_sec: f64,
    /// TFLOPs per second
    pub tflops_per_sec: f64,
    /// Operations breakdown by type
    pub operations: FlopOperations,
    /// Model FLOPs (if known)
    pub model_flops: Option<u64>,
    /// Timestamp
    pub timestamp_ms: u64,
}

/// Breakdown of FLOP operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlopOperations {
    /// Matrix multiplications
    pub matmul: u64,
    /// Attention operations
    pub attention: u64,
    /// Activation functions
    pub activations: u64,
    /// Normalization operations
    pub normalization: u64,
    /// Other operations
    pub other: u64,
}

impl FlopsTelemetry {
    /// Calculate efficiency: FLOPs per Watt
    pub fn efficiency(&self, power_w: f32) -> f64 {
        if power_w == 0.0 {
            return 0.0;
        }
        self.flops_per_sec / power_w as f64
    }

    /// Calculate GFLOPs per Watt
    pub fn gflops_per_watt(&self, power_w: f32) -> f64 {
        self.efficiency(power_w) / 1e9
    }
}

/// Model memory statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMemoryStats {
    /// Model name
    pub model_name: String,
    /// Parameters count
    pub parameters: u64,
    /// Memory footprint in MB
    pub memory_mb: u64,
    /// Quantization level (e.g., 4, 8, 16, 32)
    pub bits: u8,
    /// KV cache size in MB
    pub kv_cache_mb: u64,
    /// Activation memory in MB
    pub activation_mb: u64,
}

impl ModelMemoryStats {
    /// Calculate memory per parameter in bytes
    pub fn bytes_per_param(&self) -> f64 {
        if self.parameters == 0 {
            return 0.0;
        }
        (self.memory_mb as f64 * 1_048_576.0) / self.parameters as f64
    }

    /// Estimate memory for given sequence length
    pub fn estimate_memory_for_tokens(&self, tokens: usize) -> u64 {
        // Rough estimate: KV cache grows linearly with sequence length
        let kv_per_token = self.kv_cache_mb as f64 / 1024.0; // baseline per 1024 tokens
        let estimated_kv = (tokens as f64 / 1024.0) * kv_per_token;
        self.memory_mb + estimated_kv as u64
    }
}

/// Aggregated telemetry snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    /// GPU data for all devices
    pub gpus: Vec<GpuTelemetry>,
    /// Energy consumption
    pub energy: EnergyTelemetry,
    /// FLOPs tracking
    pub flops: FlopsTelemetry,
    /// Model statistics
    pub model: Option<ModelMemoryStats>,
    /// Timestamp
    pub timestamp_ms: u64,
}

impl Default for TelemetrySnapshot {
    fn default() -> Self {
        Self {
            gpus: Vec::new(),
            energy: EnergyTelemetry::default(),
            flops: FlopsTelemetry::default(),
            model: None,
            timestamp_ms: 0,
        }
    }
}

/// Telemetry collector
pub struct TelemetryCollector {
    start_time: Instant,
    total_energy_j: f64,
    total_flops: u64,
    gpu_available: bool,
}

impl TelemetryCollector {
    /// Create new collector
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_energy_j: 0.0,
            total_flops: 0,
            gpu_available: false,
        }
    }

    /// Check if GPU monitoring is available
    pub fn gpu_available(&self) -> bool {
        self.gpu_available
    }

    /// Collect current telemetry snapshot
    pub fn collect(&mut self) -> TelemetrySnapshot {
        let timestamp_ms = self.start_time.elapsed().as_millis() as u64;

        TelemetrySnapshot {
            gpus: self.collect_gpu_data(timestamp_ms),
            energy: EnergyTelemetry {
                energy_j: self.total_energy_j,
                duration_s: self.start_time.elapsed().as_secs_f64(),
                timestamp_ms,
                ..Default::default()
            },
            flops: FlopsTelemetry {
                total_flops: self.total_flops,
                flops_per_sec: self.calculate_flops_per_sec(),
                tflops_per_sec: self.calculate_flops_per_sec() / 1e12,
                timestamp_ms,
                ..Default::default()
            },
            model: None,
            timestamp_ms,
        }
    }

    /// Record energy consumption
    pub fn record_energy(&mut self, joules: f64) {
        self.total_energy_j += joules;
    }

    /// Record FLOPs
    pub fn record_flops(&mut self, flops: u64) {
        self.total_flops += flops;
    }

    /// Record inference operation (estimates FLOPs based on model parameters)
    pub fn record_inference(&mut self, params: u64, input_tokens: usize, output_tokens: usize) {
        // Rough estimate: 2 * params * tokens for forward pass
        let estimated_flops = 2 * params * ((input_tokens + output_tokens) as u64);
        self.record_flops(estimated_flops);
    }

    fn collect_gpu_data(&self, timestamp_ms: u64) -> Vec<GpuTelemetry> {
        // Placeholder - in production, use nvidia-ml or similar
        // For now, return empty vec
        Vec::new()
    }

    fn calculate_flops_per_sec(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        self.total_flops as f64 / elapsed
    }
}

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// FLOPs estimator for transformer models
pub struct FlopsEstimator;

impl FlopsEstimator {
    /// Estimate FLOPs for a transformer forward pass
    pub fn transformer_forward(params: u64, seq_len: usize, vocab_size: usize) -> u64 {
        // Simplified estimate: 2 * params * seq_len
        // More accurate would account for embedding, attention, FFN separately
        2 * params * seq_len as u64
    }

    /// Estimate FLOPs for attention layer
    pub fn attention(hidden_dim: usize, seq_len: usize, num_heads: usize) -> u64 {
        // QKV projections: 3 * hidden_dim * hidden_dim
        let qkv = 3 * (hidden_dim * hidden_dim) as u64;
        
        // Attention scores: seq_len^2 * hidden_dim / num_heads * num_heads
        let attn_scores = 2 * (seq_len * seq_len * hidden_dim) as u64;
        
        // Output projection: hidden_dim * hidden_dim
        let output = (hidden_dim * hidden_dim) as u64;
        
        qkv + attn_scores + output
    }

    /// Estimate FLOPs for FFN layer (SwiGLU)
    pub fn ffn(hidden_dim: usize, intermediate_dim: usize) -> u64 {
        // Gate projection: hidden_dim * intermediate_dim
        // Up projection: hidden_dim * intermediate_dim
        // Down projection: intermediate_dim * hidden_dim
        3 * (hidden_dim * intermediate_dim) as u64
    }
}

/// Energy efficiency tracker
pub struct EnergyTracker {
    energy_j: f64,
    start_power_w: Option<f32>,
    start_time: Option<Instant>,
}

impl EnergyTracker {
    /// Create new tracker
    pub fn new() -> Self {
        Self {
            energy_j: 0.0,
            start_power_w: None,
            start_time: None,
        }
    }

    /// Start tracking with given power reading
    pub fn start(&mut self, power_w: f32) {
        self.start_power_w = Some(power_w);
        self.start_time = Some(Instant::now());
    }

    /// Stop tracking and accumulate energy
    pub fn stop(&mut self, power_w: f32) -> f64 {
        if let (Some(start_power), Some(start_time)) = (self.start_power_w, self.start_time) {
            let duration = start_time.elapsed().as_secs_f64();
            let avg_power = (start_power + power_w) / 2.0;
            let energy = avg_power as f64 * duration;
            self.energy_j += energy;
            self.start_power_w = None;
            self.start_time = None;
            energy
        } else {
            0.0
        }
    }

    /// Get total energy consumed
    pub fn total_energy_j(&self) -> f64 {
        self.energy_j
    }

    /// Get total energy in kWh
    pub fn total_energy_kwh(&self) -> f64 {
        self.energy_j / 3_600_000.0
    }
}

impl Default for EnergyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Intelligence Per Watt calculator (inspired by OpenJarvis)
pub struct IntelligencePerWatt;

impl IntelligencePerWatt {
    /// Calculate intelligence efficiency metric
    /// Returns: FLOPs per Watt-second
    pub fn calculate(flops: u64, energy_j: f64) -> f64 {
        if energy_j == 0.0 {
            return 0.0;
        }
        flops as f64 / energy_j
    }

    /// Calculate GFLOPs per Watt
    pub fn gflops_per_watt(flops: u64, energy_j: f64) -> f64 {
        Self::calculate(flops, energy_j) / 1e9
    }

    /// Get efficiency rating based on GFLOPs/W
    pub fn efficiency_rating(gflops_per_watt: f64) -> &'static str {
        match gflops_per_watt {
            x if x >= 100.0 => "Excellent",
            x if x >= 50.0 => "Very Good",
            x if x >= 25.0 => "Good",
            x if x >= 10.0 => "Moderate",
            x if x > 0.0 => "Low",
            _ => "N/A",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_telemetry() {
        let gpu = GpuTelemetry {
            device_name: "RTX 4090".to_string(),
            device_index: 0,
            utilization: 85.0,
            memory_used_mb: 20000,
            memory_total_mb: 24000,
            temperature_c: 70.0,
            power_w: 350.0,
            power_limit_w: 450.0,
            timestamp_ms: 1000,
        };

        assert!((gpu.memory_utilization() - 83.33333).abs() < 0.01);
        assert!((gpu.power_utilization() - 77.77778).abs() < 0.01);
    }

    #[test]
    fn test_flops_telemetry() {
        let flops = FlopsTelemetry {
            total_flops: 1_000_000_000_000, // 1 TFLOP
            flops_per_sec: 1e12,
            tflops_per_sec: 1.0,
            timestamp_ms: 1000,
            ..Default::default()
        };

        assert!((flops.tflops_per_sec - 1.0).abs() < 0.01);
        assert!((flops.gflops_per_watt(300.0) - 3.33).abs() < 0.1);
    }

    #[test]
    fn test_model_memory_stats() {
        let model = ModelMemoryStats {
            model_name: "llama-7b".to_string(),
            parameters: 7_000_000_000,
            memory_mb: 14000,
            bits: 16,
            kv_cache_mb: 2000,
            activation_mb: 1000,
        };

        assert!(model.bytes_per_param() > 0.0);
    }

    #[test]
    fn test_telemetry_collector() {
        let mut collector = TelemetryCollector::new();
        
        collector.record_flops(1_000_000_000_000);
        collector.record_energy(1000.0);

        let snapshot = collector.collect();
        assert_eq!(snapshot.flops.total_flops, 1_000_000_000_000);
        assert_eq!(snapshot.energy.energy_j, 1000.0);
    }

    #[test]
    fn test_flops_estimator() {
        let flops = FlopsEstimator::transformer_forward(7_000_000_000, 1024, 32000);
        assert!(flops > 0);
    }

    #[test]
    fn test_energy_tracker() {
        let mut tracker = EnergyTracker::new();
        
        tracker.start(300.0);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let energy = tracker.stop(300.0);
        
        assert!(energy > 0.0);
        assert_eq!(tracker.total_energy_j(), energy);
    }

    #[test]
    fn test_intelligence_per_watt() {
        let efficiency = IntelligencePerWatt::gflops_per_watt(1_000_000_000_000, 10.0);
        assert_eq!(efficiency, 100.0);
        
        assert_eq!(IntelligencePerWatt::efficiency_rating(100.0), "Excellent");
        assert_eq!(IntelligencePerWatt::efficiency_rating(50.0), "Very Good");
        assert_eq!(IntelligencePerWatt::efficiency_rating(25.0), "Good");
        assert_eq!(IntelligencePerWatt::efficiency_rating(10.0), "Moderate");
        assert_eq!(IntelligencePerWatt::efficiency_rating(5.0), "Low");
    }
}
