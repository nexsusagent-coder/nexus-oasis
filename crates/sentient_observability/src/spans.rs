//! Tracing spans for common operations

use tracing::{info_span, Span};

/// Create a span for agent message processing
pub fn agent_message_span(agent_id: &str, message_type: &str) -> Span {
    info_span!(
        "process_message",
        agent_id = %agent_id,
        message_type = %message_type,
    )
}

/// Create a span for LLM API call
pub fn llm_call_span(provider: &str, model: &str) -> Span {
    info_span!(
        "llm_call",
        provider = %provider,
        model = %model,
    )
}

/// Create a span for channel operation
pub fn channel_span(channel_type: &str, operation: &str) -> Span {
    info_span!(
        "channel_operation",
        channel_type = %channel_type,
        operation = %operation,
    )
}

/// Create a span for memory operation
pub fn memory_span(operation: &str) -> Span {
    info_span!(
        "memory_operation",
        operation = %operation,
    )
}

/// Create a span for voice operation
pub fn voice_span(operation: &str) -> Span {
    info_span!(
        "voice_operation",
        operation = %operation,
    )
}

/// Create a span for skill execution
pub fn skill_span(skill_name: &str) -> Span {
    info_span!(
        "skill_execution",
        skill = %skill_name,
    )
}

/// Create a span for HTTP request
pub fn http_span(method: &str, path: &str) -> Span {
    info_span!(
        "http_request",
        method = %method,
        path = %path,
    )
}

/// Create a span for WebSocket connection
pub fn websocket_span(connection_id: &str) -> Span {
    info_span!(
        "websocket_connection",
        connection_id = %connection_id,
    )
}

/// Create a span for database query
pub fn db_span(query_type: &str, table: &str) -> Span {
    info_span!(
        "database_query",
        query_type = %query_type,
        table = %table,
    )
}

/// Create a span for enterprise operation
pub fn enterprise_span(operation: &str, tenant_id: &str) -> Span {
    info_span!(
        "enterprise_operation",
        operation = %operation,
        tenant_id = %tenant_id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[test]
    fn test_span_creation() {
        // Spans should be creatable
        let _span = agent_message_span("agent-123", "user");
        let _span = llm_call_span("openai", "gpt-4o");
        let _span = channel_span("telegram", "send");
        let _span = memory_span("search");
        let _span = skill_span("calculator");
    }

    #[tracing::instrument]
    #[test]
    fn test_span_in_function() {
        let _span = llm_call_span("anthropic", "claude-3");
        info!("Inside span");
    }
}
