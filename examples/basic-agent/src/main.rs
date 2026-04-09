//! Basic Agent Example
//!
//! A minimal example showing how to create and interact with a SENTIENT agent.
//!
//! # Usage
//! ```bash
//! cargo run --example basic-agent
//! ```
//!
//! # Environment Variables
//! - `OPENAI_API_KEY`: Your OpenAI API key (required)
//! - `ANTHROPIC_API_KEY`: Your Anthropic API key (optional)

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🤖 SENTIENT Basic Agent Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create agent configuration
    let config = AgentConfig {
        name: "BasicAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key,
        system_prompt: Some(
            "You are a helpful AI assistant. Be concise and accurate.".to_string()
        ),
        ..Default::default()
    };

    // Create the agent
    println!("📦 Creating agent with GPT-4o...\n");
    let mut agent = Agent::new(config).await?;

    // Example 1: Simple question
    println!("👤 User: What is the capital of Turkey?");
    let response = agent
        .send(Message::user("What is the capital of Turkey?"))
        .await?;
    println!("🤖 Agent: {}\n", response.content);

    // Example 2: Follow-up with context
    println!("👤 User: What is its population?");
    let response = agent
        .send(Message::user("What is its population?"))
        .await?;
    println!("🤖 Agent: {}\n", response.content);

    // Example 3: Code generation
    println!("👤 User: Write a Rust function to calculate fibonacci");
    let response = agent
        .send(Message::user("Write a Rust function to calculate fibonacci"))
        .await?;
    println!("🤖 Agent:\n{}\n", response.content);

    // Show conversation history
    println!("📜 Conversation History:");
    for (i, msg) in agent.history().iter().enumerate() {
        let role = match msg.role {
            sentient_core::MessageRole::User => "👤 User",
            sentient_core::MessageRole::Assistant => "🤖 Agent",
            sentient_core::MessageRole::System => "⚙️ System",
        };
        println!("  [{}] {}: {}", i + 1, role, 
            msg.content.chars().take(50).collect::<String>()
        );
    }

    println!("\n✅ Example completed successfully!");
    Ok(())
}
