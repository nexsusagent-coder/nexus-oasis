//! Distributed tracing setup using OpenTelemetry

use opentelemetry::{
    global,
    trace::{Tracer, TracerProvider},
    sdk::trace::{self, BatchSpanProcessor, RandomIdGenerator, Sampler},
    sdk::Resource,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

/// Tracing configuration
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// Service name
    pub service_name: String,
    /// OTLP endpoint (e.g., "http://localhost:4317")
    pub otlp_endpoint: Option<String>,
    /// Enable console output
    pub console_output: bool,
    /// Sampling ratio (0.0 to 1.0)
    pub sampling_ratio: f64,
    /// Enable Jaeger export
    pub jaeger_enabled: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            service_name: "sentient".to_string(),
            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok(),
            console_output: true,
            sampling_ratio: 1.0,
            jaeger_enabled: false,
        }
    }
}

/// Initialize distributed tracing
pub fn init_tracing(config: TracingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let resource = Resource::new(vec![KeyValue::new(
        SERVICE_NAME,
        config.service_name.clone(),
    )]);

    let mut layers = Vec::new();

    // OTLP layer (for Jaeger, Tempo, etc.)
    if let Some(endpoint) = &config.otlp_endpoint {
        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint);

        let tracer_provider = trace::TracerProvider::builder()
            .with_resource(resource.clone())
            .with_sampler(Sampler::TraceIdRatioBased(config.sampling_ratio))
            .with_id_generator(RandomIdGenerator::default())
            .with_span_processor(BatchSpanProcessor::builder(exporter.build_span_exporter()?)
                .build())
            .build();

        let tracer = tracer_provider.tracer("sentient");
        global::set_tracer_provider(tracer_provider);

        let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        layers.push(telemetry_layer.boxed());
    }

    // Console output layer
    if config.console_output {
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true);
        layers.push(fmt_layer.boxed());
    }

    // Environment filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sentient=debug"));

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(layers)
        .try_init()?;

    Ok(())
}

/// Shutdown tracing
pub fn shutdown_tracing() {
    global::shutdown_tracer_provider();
}

/// Create a tracing span for agent operations
#[macro_export]
macro_rules! agent_span {
    ($name:expr) => {
        tracing::info_span!("agent", name = $name)
    };
}

/// Create a tracing span for channel operations
#[macro_export]
macro_rules! channel_span {
    ($channel_type:expr, $action:expr) => {
        tracing::info_span!("channel", type = $channel_type, action = $action)
    };
}

/// Create a tracing span for LLM operations
#[macro_export]
macro_rules! llm_span {
    ($model:expr, $operation:expr) => {
        tracing::info_span!("llm", model = $model, operation = $operation)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_default() {
        let config = TracingConfig::default();
        assert_eq!(config.service_name, "sentient");
        assert!(config.console_output);
        assert_eq!(config.sampling_ratio, 1.0);
    }

    #[test]
    fn test_tracing_config_from_env() {
        std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4317");
        let config = TracingConfig::default();
        assert!(config.otlp_endpoint.is_some());
        std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
    }
}
