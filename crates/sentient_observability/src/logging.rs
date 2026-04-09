//! Structured logging configuration

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use serde_json::json;
use chrono::Utc;

/// Log format configuration
#[derive(Debug, Clone)]
pub enum LogFormat {
    /// Human-readable pretty output
    Pretty,
    /// JSON structured logs
    Json,
    /// Compact single-line format
    Compact,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log format
    pub format: LogFormat,
    /// Log level
    pub level: String,
    /// Include file and line number
    pub include_location: bool,
    /// Include thread ID
    pub include_thread_id: bool,
    /// Include target
    pub include_target: bool,
    /// Output to file
    pub file_path: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            format: LogFormat::Pretty,
            level: "info".to_string(),
            include_location: true,
            include_thread_id: false,
            include_target: true,
            file_path: None,
        }
    }
}

/// Initialize structured logging
pub fn init_logging(config: LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    match config.format {
        LogFormat::Pretty => {
            let layer = fmt::layer()
                .with_target(config.include_target)
                .with_thread_ids(config.include_thread_id)
                .with_line_number(config.include_location)
                .with_span_events(FmtSpan::CLOSE)
                .pretty();

            tracing_subscriber::registry()
                .with(env_filter)
                .with(layer)
                .try_init()?;
        }
        LogFormat::Json => {
            let layer = fmt::layer()
                .json()
                .with_target(config.include_target)
                .with_thread_ids(config.include_thread_id)
                .with_line_number(config.include_location)
                .with_span_events(FmtSpan::CLOSE);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(layer)
                .try_init()?;
        }
        LogFormat::Compact => {
            let layer = fmt::layer()
                .with_target(config.include_target)
                .compact();

            tracing_subscriber::registry()
                .with(env_filter)
                .with(layer)
                .try_init()?;
        }
    }

    Ok(())
}

/// Log helper macros
#[macro_export]
macro_rules! log_agent_event {
    ($agent_id:expr, $event:expr, $($field:tt)*) => {
        tracing::info!(
            agent_id = %$agent_id,
            event = %$event,
            $($field)*
        );
    };
}

#[macro_export]
macro_rules! log_channel_event {
    ($channel:expr, $event:expr, $($field:tt)*) => {
        tracing::info!(
            channel = %$channel,
            event = %$event,
            $($field)*
        );
    };
}

#[macro_export]
macro_rules! log_llm_call {
    ($model:expr, $tokens:expr, $latency_ms:expr) => {
        tracing::debug!(
            model = %$model,
            tokens = $tokens,
            latency_ms = $latency_ms,
            "LLM call completed"
        );
    };
}

/// Structured log entry
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: serde_json::Value,
    pub span: Option<SpanInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpanInfo {
    pub name: String,
    pub id: u64,
    pub parent_id: Option<u64>,
}

impl LogEntry {
    pub fn new(level: &str, target: &str, message: &str) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            level: level.to_string(),
            target: target.to_string(),
            message: message.to_string(),
            fields: json!({}),
            span: None,
        }
    }

    pub fn with_field(mut self, key: &str, value: serde_json::Value) -> Self {
        if let serde_json::Value::Object(ref mut fields) = self.fields {
            fields.insert(key.to_string(), value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert!(matches!(config.format, LogFormat::Pretty));
        assert_eq!(config.level, "info");
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new("info", "sentient::test", "Test message")
            .with_field("user_id", json!("12345"))
            .with_field("count", json!(42));

        assert_eq!(entry.level, "info");
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.fields["user_id"], "12345");
        assert_eq!(entry.fields["count"], 42);
    }
}
