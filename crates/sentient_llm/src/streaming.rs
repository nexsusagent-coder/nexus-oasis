//! ═══════════════════════════════════════════════════════════════════════════════
//!  LLM Streaming Parser
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Parse and process LLM streaming responses:
//! - SSE (Server-Sent Events) parsing
//! - Incremental JSON parsing
//! - Tool call extraction
//! - Code block detection

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// ═══════════════════════════════════════════════════════════════════════════════
//  STREAM TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Streaming event from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Event ID
    pub id: Option<String>,
    /// Event type
    pub event: Option<String>,
    /// Event data
    pub data: String,
}

/// Parsed streaming chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    /// Delta content
    pub delta: Option<String>,
    /// Tool calls (if any)
    pub tool_calls: Vec<ToolCallDelta>,
    /// Is this the final chunk
    pub is_final: bool,
    /// Finish reason
    pub finish_reason: Option<String>,
    /// Token usage (in final chunk)
    pub usage: Option<TokenUsage>,
}

/// Incremental tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Tool call index
    pub index: usize,
    /// Tool call ID (set in first chunk)
    pub id: Option<String>,
    /// Function name (incremental)
    pub function_name: Option<String>,
    /// Function arguments (incremental JSON)
    pub function_arguments: Option<String>,
}

/// Token usage info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Complete streaming response
#[derive(Debug, Clone)]
pub struct StreamAccumulator {
    /// Accumulated content
    pub content: String,
    /// Accumulated tool calls
    pub tool_calls: Vec<AccumulatedToolCall>,
    /// Finish reason
    pub finish_reason: Option<String>,
    /// Token usage
    pub usage: Option<TokenUsage>,
}

/// Accumulated tool call
#[derive(Debug, Clone, Default)]
pub struct AccumulatedToolCall {
    pub id: String,
    pub function_name: String,
    pub arguments_json: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SSE PARSER
// ═══════════════════════════════════════════════════════════════════════════════

/// Parser error
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid SSE format")]
    InvalidSSE,
    
    #[error("Invalid JSON: {0}")]
    InvalidJSON(String),
    
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

/// SSE (Server-Sent Events) parser
pub struct SSEParser {
    buffer: String,
}

impl SSEParser {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
    
    /// Feed data and return complete events
    pub fn feed(&mut self, data: &str) -> Vec<StreamEvent> {
        self.buffer.push_str(data);
        self.parse_events()
    }
    
    fn parse_events(&mut self) -> Vec<StreamEvent> {
        let mut events = Vec::new();
        
        while let Some(event_end) = self.find_event_end() {
            let event_data = self.buffer[..event_end].to_string();
            self.buffer = self.buffer[event_end..].to_string();
            
            if let Some(event) = self.parse_single_event(&event_data) {
                events.push(event);
            }
        }
        
        events
    }
    
    fn find_event_end(&self) -> Option<usize> {
        // SSE events end with double newline
        if let Some(pos) = self.buffer.find("\n\n") {
            return Some(pos + 2);
        }
        if let Some(pos) = self.buffer.find("\r\n\r\n") {
            return Some(pos + 4);
        }
        None
    }
    
    fn parse_single_event(&self, data: &str) -> Option<StreamEvent> {
        let mut id = None;
        let mut event = None;
        let mut event_data = String::new();
        
        for line in data.lines() {
            if line.starts_with("id:") {
                id = Some(line[3..].trim().to_string());
            } else if line.starts_with("event:") {
                event = Some(line[6..].trim().to_string());
            } else if line.starts_with("data:") {
                event_data.push_str(line[5..].trim());
            } else if !line.is_empty() {
                // Continuation of data
                event_data.push('\n');
                event_data.push_str(line);
            }
        }
        
        if !event_data.is_empty() {
            Some(StreamEvent { id, event, data: event_data })
        } else {
            None
        }
    }
    
    /// Reset parser state
    pub fn reset(&mut self) {
        self.buffer.clear();
    }
}

impl Default for SSEParser {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHUNK PARSER
// ═══════════════════════════════════════════════════════════════════════════════

/// Parser for LLM streaming chunks (OpenAI format)
pub struct ChunkParser;

impl ChunkParser {
    /// Parse a streaming chunk from JSON
    pub fn parse(data: &str) -> Result<StreamChunk, ParseError> {
        // Handle [DONE] marker
        if data == "[DONE]" {
            return Ok(StreamChunk {
                delta: None,
                tool_calls: vec![],
                is_final: true,
                finish_reason: Some("stop".to_string()),
                usage: None,
            });
        }
        
        let json: serde_json::Value = serde_json::from_str(data)
            .map_err(|e| ParseError::InvalidJSON(e.to_string()))?;
        
        let choices = json.get("choices")
            .and_then(|c| c.as_array())
            .and_then(|a| a.first());
        
        let (delta, tool_calls, finish_reason) = if let Some(choice) = choices {
            let delta_obj = choice.get("delta");
            
            // Extract text delta
            let text = delta_obj.and_then(|d| d.get("content"))
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());
            
            // Extract tool calls
            let tools = delta_obj.and_then(|d| d.get("tool_calls"))
                .and_then(|t| t.as_array())
                .map(|arr| {
                    arr.iter().enumerate().map(|(idx, tc)| {
                        let function = tc.get("function");
                        ToolCallDelta {
                            index: tc.get("index").and_then(|i| i.as_u64()).unwrap_or(idx as u64) as usize,
                            id: tc.get("id").and_then(|i| i.as_str()).map(|s| s.to_string()),
                            function_name: function.and_then(|f| f.get("name"))
                                .and_then(|n| n.as_str()).map(|s| s.to_string()),
                            function_arguments: function.and_then(|f| f.get("arguments"))
                                .and_then(|a| a.as_str()).map(|s| s.to_string()),
                        }
                    }).collect()
                }).unwrap_or_default();
            
            let finish = choice.get("finish_reason")
                .and_then(|f| f.as_str())
                .map(|s| s.to_string());
            
            (text, tools, finish)
        } else {
            (None, vec![], None)
        };
        
        // Extract usage
        let usage = json.get("usage").map(|u| TokenUsage {
            prompt_tokens: u.get("prompt_tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32,
            completion_tokens: u.get("completion_tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32,
            total_tokens: u.get("total_tokens").and_then(|t| t.as_u64()).unwrap_or(0) as u32,
        });
        
        Ok(StreamChunk {
            delta,
            tool_calls,
            is_final: finish_reason.is_some(),
            finish_reason,
            usage,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STREAM ACCUMULATOR
// ═══════════════════════════════════════════════════════════════════════════════

impl StreamAccumulator {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Accumulate a chunk
    pub fn accumulate(&mut self, chunk: &StreamChunk) {
        // Accumulate content
        if let Some(ref delta) = chunk.delta {
            self.content.push_str(delta);
        }
        
        // Accumulate tool calls
        for tc in &chunk.tool_calls {
            // Ensure we have enough slots
            while self.tool_calls.len() <= tc.index {
                self.tool_calls.push(AccumulatedToolCall::default());
            }
            
            let accumulated = &mut self.tool_calls[tc.index];
            
            if let Some(ref id) = tc.id {
                accumulated.id = id.clone();
            }
            if let Some(ref name) = tc.function_name {
                accumulated.function_name.push_str(name);
            }
            if let Some(ref args) = tc.function_arguments {
                accumulated.arguments_json.push_str(args);
            }
        }
        
        // Store finish reason and usage
        if let Some(ref reason) = chunk.finish_reason {
            self.finish_reason = Some(reason.clone());
        }
        if let Some(ref usage) = chunk.usage {
            self.usage = Some(usage.clone());
        }
    }
    
    /// Check if stream is complete
    pub fn is_complete(&self) -> bool {
        self.finish_reason.is_some()
    }
    
    /// Get complete response
    pub fn into_response(self) -> StreamResponse {
        StreamResponse {
            content: self.content,
            tool_calls: self.tool_calls.into_iter().map(|tc| CompleteToolCall {
                id: tc.id,
                function: FunctionCall {
                    name: tc.function_name,
                    arguments: tc.arguments_json,
                },
            }).collect(),
            finish_reason: self.finish_reason,
            usage: self.usage,
        }
    }
}

impl Default for StreamAccumulator {
    fn default() -> Self {
        Self {
            content: String::new(),
            tool_calls: Vec::new(),
            finish_reason: None,
            usage: None,
        }
    }
}

/// Complete streaming response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamResponse {
    pub content: String,
    pub tool_calls: Vec<CompleteToolCall>,
    pub finish_reason: Option<String>,
    pub usage: Option<TokenUsage>,
}

/// Complete tool call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteToolCall {
    pub id: String,
    pub function: FunctionCall,
}

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CODE BLOCK DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Code block detector for streaming content
pub struct CodeBlockDetector {
    in_code_block: bool,
    current_language: Option<String>,
    buffer: String,
}

impl CodeBlockDetector {
    pub fn new() -> Self {
        Self {
            in_code_block: false,
            current_language: None,
            buffer: String::new(),
        }
    }
    
    /// Process content and detect code blocks
    pub fn process(&mut self, content: &str) -> Vec<CodeBlockEvent> {
        let mut events = Vec::new();
        let mut remaining = content;
        
        while !remaining.is_empty() {
            if self.in_code_block {
                // Look for closing ```
                if let Some(pos) = remaining.find("```") {
                    // Add content before closing
                    let before = &remaining[..pos];
                    events.push(CodeBlockEvent::CodeContent(before.to_string()));
                    
                    // Close block
                    events.push(CodeBlockEvent::CodeBlockEnd);
                    self.in_code_block = false;
                    self.current_language = None;
                    
                    remaining = &remaining[pos + 3..];
                } else {
                    // All content is code
                    events.push(CodeBlockEvent::CodeContent(remaining.to_string()));
                    break;
                }
            } else {
                // Look for opening ```
                if let Some(pos) = remaining.find("```") {
                    // Add text before
                    if pos > 0 {
                        events.push(CodeBlockEvent::TextContent(remaining[..pos].to_string()));
                    }
                    
                    // Extract language
                    let after_ticks = &remaining[pos + 3..];
                    let lang_end = after_ticks.find(|c: char| c == '\n' || c == ' ').unwrap_or(0);
                    let language = if lang_end > 0 {
                        Some(after_ticks[..lang_end].to_string())
                    } else {
                        None
                    };
                    
                    events.push(CodeBlockEvent::CodeBlockStart {
                        language: language.clone(),
                    });
                    self.in_code_block = true;
                    self.current_language = language;
                    
                    // Skip past language identifier
                    remaining = after_ticks;
                    if let Some(newline_pos) = remaining.find('\n') {
                        remaining = &remaining[newline_pos + 1..];
                    }
                } else {
                    // All content is text
                    events.push(CodeBlockEvent::TextContent(remaining.to_string()));
                    break;
                }
            }
        }
        
        events
    }
    
    /// Check if currently in code block
    pub fn is_in_code_block(&self) -> bool {
        self.in_code_block
    }
}

impl Default for CodeBlockDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Code block event
#[derive(Debug, Clone)]
pub enum CodeBlockEvent {
    TextContent(String),
    CodeBlockStart { language: Option<String> },
    CodeContent(String),
    CodeBlockEnd,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sse_parser() {
        let mut parser = SSEParser::new();
        
        let events = parser.feed("data: {\"test\": 1}\n\ndata: {\"test\": 2}\n\n");
        
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].data, "{\"test\": 1}");
        assert_eq!(events[1].data, "{\"test\": 2}");
    }
    
    #[test]
    fn test_chunk_parser() {
        let data = r#"{"choices":[{"delta":{"content":"Hello"},"finish_reason":null}]}"#;
        
        let chunk = ChunkParser::parse(data).unwrap();
        
        assert_eq!(chunk.delta, Some("Hello".to_string()));
        assert!(!chunk.is_final);
    }
    
    #[test]
    fn test_chunk_parser_done() {
        let chunk = ChunkParser::parse("[DONE]").unwrap();
        
        assert!(chunk.is_final);
        assert_eq!(chunk.finish_reason, Some("stop".to_string()));
    }
    
    #[test]
    fn test_stream_accumulator() {
        let mut acc = StreamAccumulator::new();
        
        acc.accumulate(&StreamChunk {
            delta: Some("Hello ".to_string()),
            tool_calls: vec![],
            is_final: false,
            finish_reason: None,
            usage: None,
        });
        
        acc.accumulate(&StreamChunk {
            delta: Some("World!".to_string()),
            tool_calls: vec![],
            is_final: true,
            finish_reason: Some("stop".to_string()),
            usage: Some(TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
            }),
        });
        
        assert_eq!(acc.content, "Hello World!");
        assert!(acc.is_complete());
    }
    
    #[test]
    fn test_code_block_detector() {
        let mut detector = CodeBlockDetector::new();
        
        let events = detector.process("Here is code:\n```rust\nfn main() {}\n```\nDone");
        
        assert!(matches!(events[0], CodeBlockEvent::TextContent(_)));
        assert!(matches!(events[1], CodeBlockEvent::CodeBlockStart { .. }));
        assert!(matches!(events[2], CodeBlockEvent::CodeContent(_)));
        assert!(matches!(events[3], CodeBlockEvent::CodeBlockEnd));
    }
}
