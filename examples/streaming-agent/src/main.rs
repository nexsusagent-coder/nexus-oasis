//! Streaming Agent Example
//!
//! Demonstrates streaming responses from the AI agent.
//!
//! # Usage
//! ```bash
//! cargo run --example streaming-agent
//! ```

use sentient_core::{Agent, AgentConfig, Message, LlmProvider, StreamResponse};
use std::env;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🌊 SENTIENT Streaming Agent Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create agent
    let config = AgentConfig {
        name: "StreamingAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key,
        system_prompt: Some(
            "You are a helpful assistant. Provide detailed and thoughtful responses.".to_string()
        ),
        stream: true,
        ..Default::default()
    };

    let mut agent = Agent::new(config).await?;
    println!("✅ Agent ready with streaming enabled\n");

    // Example 1: Basic streaming
    println!("📝 Example 1: Basic Streaming");
    println!("─────────────────────────────\n");

    let prompt = "Explain how Rust's ownership model works in 3 paragraphs.";
    println!("👤 User: {}", prompt);
    println!("🤖 Agent: ");

    let mut stream = agent.stream(Message::user(prompt)).await?;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(StreamResponse::Token(token)) => {
                print!("{}", token);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            Ok(StreamResponse::Done) => {
                println!("\n✓ Stream completed");
            }
            Err(e) => {
                eprintln!("\n❌ Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 2: Streaming with callback
    println!("📝 Example 2: Streaming with Callback");
    println!("─────────────────────────────────────\n");

    let prompt = "Write a short poem about artificial intelligence.";
    println!("👤 User: {}", prompt);
    println!("🤖 Agent: ");

    let mut token_count = 0;
    let mut total_chars = 0;

    let mut stream = agent.stream(Message::user(prompt)).await?;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(StreamResponse::Token(token)) => {
                token_count += 1;
                total_chars += token.len();
                print!("{}", token);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            Ok(StreamResponse::Done) => {
                println!("\n\n📊 Stats: {} tokens, {} chars", token_count, total_chars);
            }
            _ => {}
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 3: Concurrent streams
    println!("📝 Example 3: Concurrent Streams");
    println!("────────────────────────────────\n");

    use futures::stream::{self, StreamExt};

    let prompts = vec![
        "What is 2 + 2?",
        "What is the capital of France?",
        "Who wrote Romeo and Juliet?",
    ];

    println!("👤 Sending {} concurrent requests...\n", prompts.len());

    let results: Vec<_> = stream::iter(prompts)
        .enumerate()
        .then(|(i, prompt)| async {
            // In production, each would have its own agent
            // Here we simulate concurrent processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100 * i as u64)).await;
            (i, prompt, format!("Response {} to: {}", i, prompt))
        })
        .collect()
        .await;

    for (i, prompt, response) in results {
        println!("📤 Stream {}:", i);
        println!("   Q: {}", prompt);
        println!("   A: {}\n", response);
    }

    // Show conversation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("📜 Conversation History:");
    for (i, msg) in agent.history().iter().enumerate() {
        let role = match msg.role {
            sentient_core::MessageRole::User => "👤",
            sentient_core::MessageRole::Assistant => "🤖",
            sentient_core::MessageRole::System => "⚙️",
        };
        println!("  [{}] {} {}", i + 1, role, 
            msg.content.chars().take(40).collect::<String>()
        );
    }

    println!("\n✅ Streaming example completed!");
    Ok(())
}
