//! Mock implementations for testing

use sentient_core::{Message, MessageRole};

/// Mock LLM response generator
pub struct MockLlm {
    pub response_template: String,
    pub delay_ms: u64,
}

impl MockLlm {
    pub fn new() -> Self {
        Self {
            response_template: "This is a mock response to: {input}".to_string(),
            delay_ms: 0,
        }
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub async fn respond(&self, input: &str) -> String {
        if self.delay_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;
        }
        self.response_template.replace("{input}", input)
    }
}

impl Default for MockLlm {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock channel for testing
pub struct MockChannel {
    pub sent_messages: std::sync::Arc<tokio::sync::Mutex<Vec<Message>>>,
}

impl MockChannel {
    pub fn new() -> Self {
        Self {
            sent_messages: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn send(&self, message: Message) {
        self.sent_messages.lock().await.push(message);
    }

    pub async fn get_sent(&self) -> Vec<Message> {
        self.sent_messages.lock().await.clone()
    }

    pub async fn clear(&self) {
        self.sent_messages.lock().await.clear();
    }
}

impl Default for MockChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock skill for testing
pub struct MockSkill {
    pub name: String,
    pub description: String,
    pub responses: std::collections::HashMap<String, String>,
}

impl MockSkill {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: format!("Mock skill: {}", name),
            responses: std::collections::HashMap::new(),
        }
    }

    pub fn with_response(mut self, input: &str, output: &str) -> Self {
        self.responses.insert(input.to_string(), output.to_string());
        self
    }

    pub fn execute(&self, input: &str) -> Option<String> {
        self.responses.get(input).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_llm() {
        let llm = MockLlm::new().with_delay(10);
        let response = llm.respond("test input").await;
        
        assert!(response.contains("test input"));
    }

    #[tokio::test]
    async fn test_mock_channel() {
        let channel = MockChannel::new();
        
        channel.send(Message::user("Hello")).await;
        channel.send(Message::assistant("Hi")).await;
        
        let sent = channel.get_sent().await;
        assert_eq!(sent.len(), 2);
    }

    #[test]
    fn test_mock_skill() {
        let skill = MockSkill::new("calculator")
            .with_response("2+2", "4")
            .with_response("3*3", "9");
        
        assert_eq!(skill.execute("2+2"), Some("4".to_string()));
        assert_eq!(skill.execute("unknown"), None);
    }
}
