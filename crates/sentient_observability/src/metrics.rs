//! Prometheus metrics for SENTIENT

use prometheus::{
    register_counter, register_gauge, register_histogram, register_int_counter,
    register_int_gauge, Counter, Gauge, Histogram, IntCounter, IntGauge, Opts, HistogramOpts,
};
use lazy_static::lazy_static;
use std::time::Instant;

lazy_static! {
    // ============================================
    // Agent Metrics
    // ============================================
    
    /// Total number of agents
    pub static ref AGENTS_TOTAL: IntGauge = register_int_gauge!(
        "sentient_agents_total",
        "Total number of active agents"
    ).unwrap();
    
    /// Messages processed by agents
    pub static ref MESSAGES_TOTAL: IntCounter = register_int_counter!(
        "sentient_messages_total",
        "Total messages processed"
    ).unwrap();
    
    /// Message processing latency
    pub static ref MESSAGE_LATENCY: Histogram = register_histogram!(
        "sentient_message_latency_seconds",
        "Message processing latency in seconds",
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    ).unwrap();
    
    /// Active conversations
    pub static ref CONVERSATIONS_ACTIVE: IntGauge = register_int_gauge!(
        "sentient_conversations_active",
        "Number of active conversations"
    ).unwrap();

    // ============================================
    // LLM Metrics
    // ============================================
    
    /// LLM API calls
    pub static ref LLM_CALLS_TOTAL: IntCounter = register_int_counter!(
        "sentient_llm_calls_total",
        "Total LLM API calls"
    ).unwrap();
    
    /// LLM call latency
    pub static ref LLM_LATENCY: Histogram = register_histogram!(
        "sentient_llm_latency_seconds",
        "LLM API call latency in seconds",
        vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0]
    ).unwrap();
    
    /// Tokens used
    pub static ref TOKENS_USED: IntCounter = register_int_counter!(
        "sentient_tokens_used_total",
        "Total tokens used"
    ).unwrap();
    
    /// LLM errors
    pub static ref LLM_ERRORS: IntCounter = register_int_counter!(
        "sentient_llm_errors_total",
        "Total LLM API errors"
    ).unwrap();

    // ============================================
    // Channel Metrics
    // ============================================
    
    /// Messages received per channel
    pub static ref CHANNEL_MESSAGES_RECEIVED: IntCounter = register_int_counter!(
        "sentient_channel_messages_received_total",
        "Messages received per channel"
    ).unwrap();
    
    /// Messages sent per channel
    pub static ref CHANNEL_MESSAGES_SENT: IntCounter = register_int_counter!(
        "sentient_channel_messages_sent_total",
        "Messages sent per channel"
    ).unwrap();
    
    /// Channel connections
    pub static ref CHANNEL_CONNECTIONS: IntGauge = register_int_gauge!(
        "sentient_channel_connections",
        "Number of active channel connections"
    ).unwrap();
    
    /// Channel errors
    pub static ref CHANNEL_ERRORS: IntCounter = register_int_counter!(
        "sentient_channel_errors_total",
        "Total channel errors"
    ).unwrap();

    // ============================================
    // Memory Metrics
    // ============================================
    
    /// Memory entries count
    pub static ref MEMORY_ENTRIES: IntGauge = register_int_gauge!(
        "sentient_memory_entries",
        "Number of memory entries"
    ).unwrap();
    
    /// Memory size in bytes
    pub static ref MEMORY_SIZE_BYTES: Gauge = register_gauge!(
        "sentient_memory_size_bytes",
        "Memory storage size in bytes"
    ).unwrap();
    
    /// Memory search latency
    pub static ref MEMORY_SEARCH_LATENCY: Histogram = register_histogram!(
        "sentient_memory_search_latency_seconds",
        "Memory search latency in seconds",
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
    ).unwrap();

    // ============================================
    // Voice Metrics
    // ============================================
    
    /// STT operations
    pub static ref STT_OPERATIONS: IntCounter = register_int_counter!(
        "sentient_stt_operations_total",
        "Total speech-to-text operations"
    ).unwrap();
    
    /// TTS operations
    pub static ref TTS_OPERATIONS: IntCounter = register_int_counter!(
        "sentient_tts_operations_total",
        "Total text-to-speech operations"
    ).unwrap();
    
    /// Voice latency
    pub static ref VOICE_LATENCY: Histogram = register_histogram!(
        "sentient_voice_latency_seconds",
        "Voice operation latency in seconds",
        vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
    ).unwrap();

    // ============================================
    // System Metrics
    // ============================================
    
    /// HTTP requests
    pub static ref HTTP_REQUESTS_TOTAL: IntCounter = register_int_counter!(
        "sentient_http_requests_total",
        "Total HTTP requests"
    ).unwrap();
    
    /// HTTP request latency
    pub static ref HTTP_REQUEST_LATENCY: Histogram = register_histogram!(
        "sentient_http_request_latency_seconds",
        "HTTP request latency in seconds",
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]
    ).unwrap();
    
    /// WebSocket connections
    pub static ref WEBSOCKET_CONNECTIONS: IntGauge = register_int_gauge!(
        "sentient_websocket_connections",
        "Active WebSocket connections"
    ).unwrap();
    
    /// Queue size
    pub static ref QUEUE_SIZE: IntGauge = register_int_gauge!(
        "sentient_queue_size",
        "Message queue size"
    ).unwrap();
}

/// Timer for measuring latency
pub struct LatencyTimer {
    start: Instant,
    metric: &'static Histogram,
}

impl LatencyTimer {
    pub fn new(metric: &'static Histogram) -> Self {
        Self {
            start: Instant::now(),
            metric,
        }
    }
}

impl Drop for LatencyTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_secs_f64();
        self.metric.observe(elapsed);
    }
}

/// Increment a counter with optional labels
pub fn inc_counter(counter: &IntCounter) {
    counter.inc();
}

/// Set a gauge value
pub fn set_gauge(gauge: &IntGauge, value: i64) {
    gauge.set(value);
}

/// Observe a histogram value
pub fn observe_histogram(histogram: &Histogram, value: f64) {
    histogram.observe(value);
}

/// Start a latency timer
pub fn start_latency_timer(metric: &'static Histogram) -> LatencyTimer {
    LatencyTimer::new(metric)
}

/// Export metrics in Prometheus format
pub fn export_metrics() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registration() {
        // Metrics should be registered and accessible
        AGENTS_TOTAL.set(5);
        assert_eq!(AGENTS_TOTAL.get(), 5);
    }

    #[test]
    fn test_counter_increment() {
        let before = MESSAGES_TOTAL.get();
        inc_counter(&MESSAGES_TOTAL);
        assert_eq!(MESSAGES_TOTAL.get(), before + 1);
    }

    #[test]
    fn test_latency_timer() {
        {
            let _timer = start_latency_timer(&MESSAGE_LATENCY);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        // Timer should have recorded the latency
        // Can't easily assert the exact value, but it should not panic
    }

    #[test]
    fn test_export_metrics() {
        let exported = export_metrics();
        assert!(exported.contains("sentient_"));
    }
}
