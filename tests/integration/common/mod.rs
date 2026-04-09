//! Common test utilities

pub mod fixtures;
pub mod mocks;
pub mod assertions;

/// Create a test agent configuration
pub fn test_config() -> sentient_core::AgentConfig {
    sentient_core::AgentConfig {
        name: "TestAgent".to_string(),
        api_key: "test-api-key".to_string(),
        llm_model: "gpt-4o-mini".to_string(),
        temperature: 0.7,
        max_tokens: 100,
        ..Default::default()
    }
}

/// Create a test message
pub fn test_message(content: &str) -> sentient_core::Message {
    sentient_core::Message::user(content)
}

/// Wait for async condition
pub async fn wait_for<F, Fut>(condition: F, timeout_ms: u64)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);
    
    while start.elapsed() < timeout {
        if condition().await {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    
    panic!("Timeout waiting for condition");
}
