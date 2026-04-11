// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq Chat API
// ═══════════════════════════════════════════════════════════════════════════════
//  OpenAI-compatible chat completions
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::System,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a tool response message
    pub fn tool_response(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Tool,
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    /// Add name to message
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Chat completion request
#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

impl ChatRequest {
    /// Create a new chat request
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature: None,
            max_tokens: None,
            top_p: None,
            stream: None,
            stop: None,
            tools: None,
            tool_choice: None,
            response_format: None,
        }
    }

    /// Set temperature (0.0 - 2.0)
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max: usize) -> Self {
        self.max_tokens = Some(max);
        self
    }

    /// Set top_p
    pub fn with_top_p(mut self, p: f32) -> Self {
        self.top_p = Some(p);
        self
    }

    /// Enable streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Add stop sequences
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Add tools (function calling)
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set tool choice
    pub fn with_tool_choice(mut self, choice: ToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Set JSON response format
    pub fn with_json_response(mut self) -> Self {
        self.response_format = Some(ResponseFormat::JsonObject);
        self
    }
}

/// Chat completion response
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Usage,
    #[serde(default)]
    pub system_fingerprint: Option<String>,
}

impl ChatResponse {
    /// Get the message content
    pub fn content(&self) -> Option<&str> {
        self.choices.first()?.message.content.as_deref()
    }

    /// Get all tool calls
    pub fn tool_calls(&self) -> Option<&Vec<ToolCall>> {
        self.choices.first()?.message.tool_calls.as_ref()
    }

    /// Get usage statistics
    pub fn usage(&self) -> &Usage {
        &self.usage
    }

    /// Check if response was streamed
    pub fn is_streamed(&self) -> bool {
        self.object == "chat.completion.chunk"
    }
}

/// Chat choice
#[derive(Debug, Clone, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: Option<String>,
}

/// Response message
#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub role: MessageRole,
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Token usage
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    #[serde(default)]
    pub prompt_time_ms: Option<f64>,
    #[serde(default)]
    pub completion_time_ms: Option<f64>,
    #[serde(default)]
    pub total_time_ms: Option<f64>,
}

impl Usage {
    /// Calculate tokens per second for completion
    pub fn completion_tps(&self) -> Option<f64> {
        let time = self.completion_time_ms?;
        if time > 0.0 {
            Some(self.completion_tokens as f64 / (time / 1000.0))
        } else {
            None
        }
    }
}

/// Tool definition for function calling
#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDef,
}

impl Tool {
    pub fn function(name: impl Into<String>, description: impl Into<String>, parameters: serde_json::Value) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}

/// Function definition
#[derive(Debug, Clone, Serialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

impl FunctionCall {
    /// Parse arguments as JSON
    pub fn parse_args<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}

/// Tool choice
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ToolChoice {
    String(String), // "auto", "none", "required"
    Object { #[serde(rename = "type")] tool_type: String, function: ToolChoiceFunction },
}

impl ToolChoice {
    pub fn auto() -> Self {
        Self::String("auto".to_string())
    }

    pub fn none() -> Self {
        Self::String("none".to_string())
    }

    pub fn required() -> Self {
        Self::String("required".to_string())
    }

    pub fn function(name: impl Into<String>) -> Self {
        Self::Object {
            tool_type: "function".to_string(),
            function: ToolChoiceFunction { name: name.into() },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolChoiceFunction {
    pub name: String,
}

/// Response format
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Text,
    JsonObject,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage::user("Hello");
        assert_eq!(msg.role, MessageRole::User);
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_chat_request_builder() {
        let req = ChatRequest::new("llama-3.3-70b", vec![ChatMessage::user("Hi")])
            .with_temperature(0.7)
            .with_max_tokens(100);

        assert_eq!(req.temperature, Some(0.7));
        assert_eq!(req.max_tokens, Some(100));
    }

    #[test]
    fn test_tool_creation() {
        let tool = Tool::function(
            "get_weather",
            "Get weather for a location",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                }
            })
        );

        assert_eq!(tool.function.name, "get_weather");
    }

    #[test]
    fn test_usage_tps() {
        let usage = Usage {
            prompt_tokens: 100,
            completion_tokens: 500,
            total_tokens: 600,
            prompt_time_ms: Some(50.0),
            completion_time_ms: Some(1000.0),
            total_time_ms: Some(1050.0),
        };

        let tps = usage.completion_tps().unwrap();
        // 500 tokens in 1 second = 500 tps
        assert_eq!(tps, 500.0);
    }
}
