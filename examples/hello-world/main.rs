// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Hello World Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Minimal example showing basic SENTIENT usage
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_core::{Agent, AgentConfig, Message};
use sentient_gateway::{LlmClient, LlmConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 SENTIENT OS - Hello World");
    println!("═════════════════════════════\n");

    // 1. Create LLM client
    // Uses Ollama by default (free, local)
    let client = LlmClient::new(LlmConfig {
        provider: "ollama".into(),
        model: "llama3.2:3b".into(),
        api_key: None,
        base_url: Some("http://localhost:11434".into()),
    });

    println!("📡 Connecting to Ollama (localhost:11434)...");
    println!("🤖 Model: llama3.2:3b\n");

    // 2. Create agent
    let agent = Agent::new(AgentConfig {
        name: "hello-agent".into(),
        description: "A simple greeting agent".into(),
        llm: client,
        system_prompt: Some("You are a friendly AI assistant. Keep responses brief.".into()),
    });

    // 3. Send message
    println!("👤 You: Hello, SENTIENT!\n");
    
    let response = agent
        .chat(Message::user("Hello, SENTIENT!"))
        .await?;

    println!("🤖 Agent: {}\n", response.content);

    println!("═════════════════════════════");
    println!("✅ Hello World complete!");
    
    Ok(())
}
