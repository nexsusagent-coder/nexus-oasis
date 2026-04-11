// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq Streaming
// ═══════════════════════════════════════════════════════════════════════════════
//  Server-Sent Events streaming for real-time responses
// ═══════════════════════════════════════════════════════════════════════════════

use futures::Stream;
use serde::Deserialize;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Enable streaming
    pub enabled: bool,
    /// Include usage in final chunk
    pub include_usage: bool,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_usage: true,
        }
    }
}

/// Streaming event
#[derive(Debug, Clone, Deserialize)]
pub struct StreamEvent {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Vec<StreamChoice>,
    #[serde(default)]
    pub usage: Option<crate::chat::Usage>,
}

impl StreamEvent {
    /// Get delta content
    pub fn content(&self) -> Option<&str> {
        self.choices.first()?.delta.content.as_deref()
    }

    /// Check if stream is finished
    pub fn is_finished(&self) -> bool {
        self.choices.first()
            .map(|c| c.finish_reason.is_some())
            .unwrap_or(false)
    }

    /// Get finish reason
    pub fn finish_reason(&self) -> Option<&str> {
        self.choices.first()?.finish_reason.as_deref()
    }
}

/// Stream choice
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

/// Delta content
#[derive(Debug, Clone, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<crate::chat::ToolCall>>,
}

/// Stream wrapper for async iteration
pub struct ChatStream {
    inner: Pin<Box<dyn Stream<Item = Result<StreamEvent, crate::GroqError>> + Send>>,
}

impl ChatStream {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<StreamEvent, crate::GroqError>> + Send + 'static,
    {
        Self {
            inner: Box::pin(stream),
        }
    }

    /// Collect all content into a string
    pub async fn collect_content(mut self) -> crate::Result<String> {
        let mut content = String::new();
        
        while let Some(event) = futures::StreamExt::next(&mut self).await {
            let event = event?;
            if let Some(text) = event.content() {
                content.push_str(text);
            }
        }

        Ok(content)
    }
}

impl Stream for ChatStream {
    type Item = Result<StreamEvent, crate::GroqError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

/// SSE line parser
pub fn parse_sse_line(line: &str) -> Option<StreamEvent> {
    let line = line.trim();
    
    if line.is_empty() || line == "data: [DONE]" {
        return None;
    }

    if let Some(data) = line.strip_prefix("data: ") {
        serde_json::from_str(data).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_line() {
        let line = r#"data: {"id":"chatcmpl-123","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}"#;
        let event = parse_sse_line(line);
        
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.content(), Some("Hello"));
    }

    #[test]
    fn test_parse_sse_done() {
        let line = "data: [DONE]";
        assert!(parse_sse_line(line).is_none());
    }

    #[test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert!(config.enabled);
        assert!(config.include_usage);
    }
}
