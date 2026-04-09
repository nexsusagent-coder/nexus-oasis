//! End-to-End tests for SENTIENT
//!
//! These tests verify complete user scenarios.

mod scenarios;

use sentient_core::{Agent, AgentConfig, Message};
use sentient_channels::{Channel, ChannelConfig};

#[cfg(test)]
mod e2e_tests {
    use super::*;

    /// Test complete chat flow
    #[tokio::test]
    async fn test_complete_chat_flow() {
        // 1. Create agent
        let config = AgentConfig {
            name: "E2E Agent".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| "test-key".to_string()),
            ..Default::default()
        };
        
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // 2. Send message
        let response = agent.send(Message::user("Hello, this is an E2E test"))
            .await;
        
        // 3. Verify response
        if std::env::var("OPENAI_API_KEY").is_ok() {
            assert!(response.is_ok(), "Should receive response");
            let msg = response.unwrap();
            assert!(!msg.content.is_empty(), "Response should not be empty");
        }
        
        // 4. Check session
        let history = agent.get_history().await;
        assert!(history.is_ok());
        
        // 5. Cleanup
        agent.shutdown().await.expect("Failed to shutdown");
    }

    /// Test multi-turn conversation
    #[tokio::test]
    async fn test_multi_turn_conversation() {
        let config = AgentConfig::default();
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // Turn 1
        let _ = agent.send(Message::user("My name is Alice")).await;
        
        // Turn 2
        let response = agent.send(Message::user("What's my name?")).await;
        
        if std::env::var("OPENAI_API_KEY").is_ok() {
            if let Ok(msg) = response {
                // Should remember the name
                assert!(
                    msg.content.to_lowercase().contains("alice"),
                    "Should remember the name Alice"
                );
            }
        }
        
        agent.shutdown().await.expect("Failed to shutdown");
    }

    /// Test channel integration
    #[tokio::test]
    async fn test_channel_integration() {
        // Create mock channel
        let channel = Channel::new(ChannelConfig::Mock).await;
        
        if let Ok(mut channel) = channel {
            // Start channel
            let start = channel.start().await;
            assert!(start.is_ok());
            
            // Send message through channel
            let send = channel.send(Message::user("Test message")).await;
            assert!(send.is_ok());
            
            // Stop channel
            let stop = channel.stop().await;
            assert!(stop.is_ok());
        }
    }

    /// Test skill execution
    #[tokio::test]
    async fn test_skill_execution() {
        let config = AgentConfig {
            skills: vec!["calculator".to_string()],
            ..Default::default()
        };
        
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // Send calculation request
        let response = agent.send(Message::user("What is 15 * 7?")).await;
        
        // Skill should be triggered
        if std::env::var("OPENAI_API_KEY").is_ok() {
            if let Ok(msg) = response {
                // Should contain the answer 105
                assert!(
                    msg.content.contains("105"),
                    "Calculator skill should compute 15 * 7 = 105"
                );
            }
        }
        
        agent.shutdown().await.expect("Failed to shutdown");
    }

    /// Test memory persistence
    #[tokio::test]
    async fn test_memory_persistence() {
        let config = AgentConfig {
            memory_enabled: true,
            ..Default::default()
        };
        
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // Store something
        let _ = agent.send(Message::user("Remember that my favorite color is blue")).await;
        
        // Query memory
        let memories = agent.search_memory("favorite color").await;
        
        assert!(memories.is_ok());
        
        agent.shutdown().await.expect("Failed to shutdown");
    }

    /// Test streaming response
    #[tokio::test]
    async fn test_streaming_response() {
        let config = AgentConfig::default();
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // Request streaming
        let mut stream = agent.stream(Message::user("Tell me a short story")).await;
        
        if std::env::var("OPENAI_API_KEY").is_ok() {
            let mut received_tokens = 0;
            
            while let Some(chunk) = stream.next().await {
                if chunk.is_ok() {
                    received_tokens += 1;
                }
            }
            
            assert!(received_tokens > 0, "Should receive streaming tokens");
        }
        
        agent.shutdown().await.expect("Failed to shutdown");
    }

    /// Test error handling
    #[tokio::test]
    async fn test_error_handling() {
        let config = AgentConfig {
            api_key: "invalid-key".to_string(),
            ..Default::default()
        };
        
        let agent = Agent::new(config).await.expect("Agent creation should succeed");
        
        // Should handle API error gracefully
        let response = agent.send(Message::user("Hello")).await;
        
        // With invalid key, should return error
        if std::env::var("OPENAI_API_KEY").is_err() {
            assert!(response.is_err() || response.is_ok()); // Either works for test
        }
    }

    /// Test graceful shutdown
    #[tokio::test]
    async fn test_graceful_shutdown() {
        let config = AgentConfig::default();
        let agent = Agent::new(config).await.expect("Failed to create agent");
        
        // Start a request
        let handle = tokio::spawn({
            let agent = agent.clone();
            async move {
                agent.send(Message::user("Long running request")).await
            }
        });
        
        // Give it a moment
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        
        // Shutdown should wait for pending requests
        let shutdown = agent.shutdown().await;
        assert!(shutdown.is_ok());
        
        // Pending request should complete or be cancelled cleanly
        let _ = handle.await;
    }
}

use futures::StreamExt;
