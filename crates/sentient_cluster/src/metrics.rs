//! ─── Prometheus Metrics ───

use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, Opts, Registry,
    TextEncoder, Encoder,
};
use lazy_static::lazy_static;

lazy_static! {
    /// Metrics registry
    pub static ref REGISTRY: Registry = Registry::new();
    
    /// Total tasks processed
    pub static ref TASKS_TOTAL: Counter = Counter::with_opts(
        Opts::new("sentient_tasks_total", "Total number of tasks processed")
    ).expect("Failed to create TASKS_TOTAL counter");
    
    /// Active agents
    pub static ref AGENTS_ACTIVE: Gauge = Gauge::with_opts(
        Opts::new("sentient_agents_active", "Number of active agents")
    ).expect("Failed to create AGENTS_ACTIVE gauge");
    
    /// Task duration histogram
    pub static ref TASK_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("sentient_task_duration_seconds", "Task execution duration")
            .buckets(vec![0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0, 300.0])
    ).expect("Failed to create TASK_DURATION histogram");
    
    /// Messages processed by channel
    pub static ref MESSAGES_BY_CHANNEL: Counter = Counter::with_opts(
        Opts::new("sentient_messages_by_channel_total", "Messages processed by channel")
    ).expect("Failed to create MESSAGES_BY_CHANNEL counter");
    
    /// Voice recognition latency
    pub static ref VOICE_LATENCY: Histogram = Histogram::with_opts(
        HistogramOpts::new("sentient_voice_latency_seconds", "Voice recognition latency")
            .buckets(vec![0.05, 0.1, 0.2, 0.5, 1.0, 2.0])
    ).expect("Failed to create VOICE_LATENCY histogram");
    
    /// API requests
    pub static ref API_REQUESTS: Counter = Counter::with_opts(
        Opts::new("sentient_api_requests_total", "Total API requests")
    ).expect("Failed to create API_REQUESTS counter");
    
    /// Errors
    pub static ref ERRORS_TOTAL: Counter = Counter::with_opts(
        Opts::new("sentient_errors_total", "Total errors")
    ).expect("Failed to create ERRORS_TOTAL counter");
    
    /// Memory usage
    pub static ref MEMORY_USAGE: Gauge = Gauge::with_opts(
        Opts::new("sentient_memory_bytes", "Memory usage in bytes")
    ).expect("Failed to create MEMORY_USAGE gauge");
    
    /// GPU usage
    pub static ref GPU_USAGE: Gauge = Gauge::with_opts(
        Opts::new("sentient_gpu_usage_percent", "GPU usage percentage")
    ).expect("Failed to create GPU_USAGE gauge");
}

/// Initialize metrics
pub fn init() {
    let _ = REGISTRY.register(Box::new(TASKS_TOTAL.clone()));
    let _ = REGISTRY.register(Box::new(AGENTS_ACTIVE.clone()));
    let _ = REGISTRY.register(Box::new(TASK_DURATION.clone()));
    let _ = REGISTRY.register(Box::new(MESSAGES_BY_CHANNEL.clone()));
    let _ = REGISTRY.register(Box::new(VOICE_LATENCY.clone()));
    let _ = REGISTRY.register(Box::new(API_REQUESTS.clone()));
    let _ = REGISTRY.register(Box::new(ERRORS_TOTAL.clone()));
    let _ = REGISTRY.register(Box::new(MEMORY_USAGE.clone()));
    let _ = REGISTRY.register(Box::new(GPU_USAGE.clone()));
}

/// Export metrics in Prometheus format
pub fn export() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    
    let metric_families = REGISTRY.gather();
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        log::error!("Failed to encode metrics: {}", e);
        return String::new();
    }
    
    String::from_utf8(buffer).expect("Metrics should be valid UTF-8")
}

/// Record task execution
pub fn record_task(duration_secs: f64, success: bool) {
    TASK_DURATION.observe(duration_secs);
    TASKS_TOTAL.inc();
    
    if !success {
        ERRORS_TOTAL.inc();
    }
}

/// Record voice processing
pub fn record_voice(latency_secs: f64) {
    VOICE_LATENCY.observe(latency_secs);
}

/// Update active agents count
pub fn set_active_agents(count: i64) {
    AGENTS_ACTIVE.set(count as f64);
}

/// Update memory usage
pub fn set_memory_usage(bytes: i64) {
    MEMORY_USAGE.set(bytes as f64);
}

/// Increment message counter
pub fn record_message(_channel: &str) {
    MESSAGES_BY_CHANNEL.inc_by(1.0);
    // Could add label for channel
}
