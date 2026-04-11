//! ─── LLM Types ───
//!
//! Core types for LLM operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  MESSAGES
// ═══════════════════════════════════════════════════════════════════════════════

/// Chat message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
    Function,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::Tool => write!(f, "tool"),
            Role::Function => write!(f, "function"),
        }
    }
}

/// Chat message content (text or multimodal)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    /// Simple text content
    Text(String),
    /// Multimodal content (text + images)
    Parts(Vec<ContentPart>),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text(String::new())
    }
}

impl Content {
    /// Create text content
    pub fn text<T: Into<String>>(text: T) -> Self {
        Content::Text(text.into())
    }

    /// Create multimodal content with image
    pub fn with_image<T: Into<String>>(text: T, image_url: &str) -> Self {
        Content::Parts(vec![
            ContentPart::Text { text: text.into() },
            ContentPart::Image {
                image_url: ImageUrl {
                    url: image_url.to_string(),
                    detail: None,
                },
            },
        ])
    }

    /// Get text content if available
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Content::Text(text) => Some(text),
            Content::Parts(parts) => {
                for part in parts {
                    if let ContentPart::Text { text } = part {
                        return Some(text);
                    }
                }
                None
            }
        }
    }
}

impl From<String> for Content {
    fn from(text: String) -> Self {
        Content::Text(text)
    }
}

impl From<&str> for Content {
    fn from(text: &str) -> Self {
        Content::Text(text.to_string())
    }
}

/// Content part for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentPart {
    Text { text: String },
    Image { image_url: ImageUrl },
}

/// Image URL with optional detail level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message role
    pub role: Role,
    /// Message content
    pub content: Content,
    /// Name (for tool/function calls)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tool calls (for assistant messages with tool calls)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Tool call ID (for tool response messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    /// Create a system message
    pub fn system<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::System,
            content: Content::text(content),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message
    pub fn user<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::User,
            content: Content::text(content),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create an assistant message
    pub fn assistant<T: Into<String>>(content: T) -> Self {
        Self {
            role: Role::Assistant,
            content: Content::text(content),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a user message with an image
    pub fn user_with_image<T: Into<String>>(text: T, image_url: &str) -> Self {
        Self {
            role: Role::User,
            content: Content::with_image(text, image_url),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create a tool response message
    pub fn tool_response(id: &str, content: &str) -> Self {
        Self {
            role: Role::Tool,
            content: Content::text(content),
            name: None,
            tool_calls: None,
            tool_call_id: Some(id.to_string()),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TOOL CALLING
// ═══════════════════════════════════════════════════════════════════════════════

/// Tool call from the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool call ID
    pub id: String,
    /// Tool type (usually "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function call details
    pub function: FunctionCall,
}

/// Function call details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name
    pub name: String,
    /// Function arguments as JSON string
    pub arguments: String,
}

impl FunctionCall {
    /// Parse arguments as a specific type
    pub fn parse_args<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.arguments)
    }
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool type (usually "function")
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition
    pub function: FunctionDefinition,
}

impl Tool {
    /// Create a function tool
    pub fn function(name: &str, description: &str, parameters: serde_json::Value) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: name.to_string(),
                description: description.to_string(),
                parameters,
            },
        }
    }
}

/// Function definition for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// Function parameters (JSON Schema)
    pub parameters: serde_json::Value,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAT REQUEST
// ═══════════════════════════════════════════════════════════════════════════════

/// Chat completion request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Model to use
    pub model: String,
    /// Messages in the conversation
    pub messages: Vec<Message>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Temperature (0.0 - 2.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-p sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Number of completions to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Stop sequences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Presence penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Frequency penalty
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// User ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Tools available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Tool choice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Response format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// Seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Stream response
    #[serde(default)]
    pub stream: bool,
    /// Log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Top log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
}

impl ChatRequest {
    /// Create a new chat request
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            user: None,
            tools: None,
            tool_choice: None,
            response_format: None,
            seed: None,
            stream: false,
            logprobs: None,
            top_logprobs: None,
        }
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    /// Add a message
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    /// Set tools
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Enable streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    /// Set response format
    pub fn with_json_response(mut self) -> Self {
        self.response_format = Some(ResponseFormat {
            type_: "json_object".to_string(),
        });
        self
    }
}

/// Tool choice options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// String choice
    String(String),
    /// Specific function choice
    Object { type_: String, function: FunctionChoice },
}

/// Function choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionChoice {
    pub name: String,
}

/// Response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAT RESPONSE
// ═══════════════════════════════════════════════════════════════════════════════

/// Chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Response ID
    pub id: String,
    /// Object type
    pub object: String,
    /// Creation timestamp
    pub created: i64,
    /// Model used
    pub model: String,
    /// Choices
    pub choices: Vec<Choice>,
    /// Usage statistics
    pub usage: Usage,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

/// Completion choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Choice index
    pub index: u32,
    /// Message
    pub message: Message,
    /// Finish reason
    pub finish_reason: Option<String>,
    /// Log probabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// Log probabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProbs {
    pub content: Option<Vec<TokenLogProb>>,
}

/// Token log probability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Option<Vec<u8>>,
    pub top_logprobs: Vec<TopLogProb>,
}

/// Top log probability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Option<Vec<u8>>,
}

/// Token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    /// Prompt tokens
    pub prompt_tokens: u32,
    /// Completion tokens
    pub completion_tokens: u32,
    /// Total tokens
    pub total_tokens: u32,
}

impl Usage {
    /// Calculate cost based on model pricing
    pub fn calculate_cost(&self, model: &ModelInfo) -> f64 {
        let prompt_cost = (self.prompt_tokens as f64 / 1000.0) * model.input_cost_per_1k;
        let completion_cost = (self.completion_tokens as f64 / 1000.0) * model.output_cost_per_1k;
        prompt_cost + completion_cost
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MODEL INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Provider name
    pub provider: String,
    /// Context window size
    pub context_window: u32,
    /// Maximum output tokens
    pub max_output_tokens: u32,
    /// Input cost per 1K tokens
    pub input_cost_per_1k: f64,
    /// Output cost per 1K tokens
    pub output_cost_per_1k: f64,
    /// Supports vision
    pub supports_vision: bool,
    /// Supports function calling
    pub supports_tools: bool,
    /// Supports streaming
    pub supports_streaming: bool,
    /// Supports JSON mode
    pub supports_json: bool,
    /// Training cutoff date
    pub training_cutoff: Option<String>,
    /// Quality rating (1-5)
    pub quality_rating: u8,
    /// Speed rating (1-5)
    pub speed_rating: u8,
    /// Reasoning model
    pub is_reasoning: bool,
    /// Free tier available
    pub free_tier: bool,
}

impl ModelInfo {
    /// Get context window
    pub fn context_window(&self) -> u32 {
        self.context_window
    }

    /// Check if model supports feature
    pub fn supports(&self, feature: ModelFeature) -> bool {
        match feature {
            ModelFeature::Vision => self.supports_vision,
            ModelFeature::Tools => self.supports_tools,
            ModelFeature::Streaming => self.supports_streaming,
            ModelFeature::Json => self.supports_json,
            ModelFeature::Reasoning => self.is_reasoning,
        }
    }
}

/// Model features
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFeature {
    Vision,
    Tools,
    Streaming,
    Json,
    Reasoning,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STREAMING
// ═══════════════════════════════════════════════════════════════════════════════

/// Stream chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Response ID
    pub id: String,
    /// Object type
    pub object: String,
    /// Creation timestamp
    pub created: i64,
    /// Model used
    pub model: String,
    /// System fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    /// Choices
    pub choices: Vec<StreamChoice>,
}

/// Stream choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    /// Choice index
    pub index: u32,
    /// Delta content
    pub delta: Delta,
    /// Finish reason
    pub finish_reason: Option<String>,
}

/// Delta content in stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    /// Role (only in first chunk)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROVIDER INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider name
    pub name: String,
    /// Provider ID
    pub id: String,
    /// API base URL
    pub base_url: String,
    /// Documentation URL
    pub docs_url: String,
    /// Pricing URL
    pub pricing_url: String,
    /// Free tier available
    pub free_tier: bool,
    /// Free tier limits
    pub free_tier_limits: Option<String>,
    /// Available models
    pub models: Vec<String>,
    /// Supports streaming
    pub supports_streaming: bool,
    /// Supports function calling
    pub supports_tools: bool,
    /// Supports vision
    pub supports_vision: bool,
    /// API key env variable
    pub api_key_env: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let system = Message::system("You are helpful");
        assert_eq!(system.role, Role::System);

        let user = Message::user("Hello");
        assert_eq!(user.role, Role::User);

        let assistant = Message::assistant("Hi there!");
        assert_eq!(assistant.role, Role::Assistant);
    }

    #[test]
    fn test_chat_request() {
        let request = ChatRequest::new("gpt-4", vec![Message::user("Hello")])
            .with_max_tokens(100)
            .with_temperature(0.7);

        assert_eq!(request.model, "gpt-4");
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.temperature, Some(0.7));
    }

    #[test]
    fn test_content_as_text() {
        let content = Content::text("Hello");
        assert_eq!(content.as_text(), Some("Hello"));
    }

    #[test]
    fn test_usage_cost() {
        let model = ModelInfo {
            id: "gpt-4".into(),
            name: "GPT-4".into(),
            provider: "OpenAI".into(),
            context_window: 8192,
            max_output_tokens: 4096,
            input_cost_per_1k: 0.03,
            output_cost_per_1k: 0.06,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            supports_json: true,
            training_cutoff: None,
            quality_rating: 5,
            speed_rating: 3,
            is_reasoning: false,
            free_tier: false,
        };

        let usage = Usage {
            prompt_tokens: 1000,
            completion_tokens: 500,
            total_tokens: 1500,
        };

        let cost = usage.calculate_cost(&model);
        assert!((cost - 0.06).abs() < 0.001); // 0.03 + 0.03
    }
}
