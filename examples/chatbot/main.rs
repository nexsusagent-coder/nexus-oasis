// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Chatbot Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Interactive chatbot with memory and conversation history
// ═══════════════════════════════════════════════════════════════════════════════

use std::io::{self, Write};
use sentient_core::{Agent, AgentConfig, Message};
use sentient_gateway::{LlmClient, LlmConfig, LlmProvider};
use sentient_memory::{MemoryCube, MemoryConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(r#"
╔════════════════════════════════════════════════════════════════╗
║                                                                ║
║     ███████╗███████╗███╗   ██╗██╗   ██╗███████╗████████╗███████╗██████╗     ║
║     ██╔════╝██╔════╝████╗  ██║██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗    ║
║     ███████╗█████╗  ██╔██╗ ██║██║   ██║███████╗   ██║   █████╗  ██████╔╝    ║
║     ╚════██║██╔══╝  ██║╚██╗██║╚██╗ ██╔╝╚════██║   ██║   ██╔══╝  ██╔══██╗    ║
║     ███████║███████╗██║ ╚████║ ╚████╔╝ ███████║   ██║   ███████╗██║  ██║    ║
║     ╚══════╝╚══════╝╚═╝  ╚═══╝  ╚═══╝  ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝    ║
║                                                                ║
║              The Operating System That Thinks                   ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
"#);

    // ────────────────────────────────────────────────────────────────────────
    //  Configuration
    // ────────────────────────────────────────────────────────────────────────
    
    println!("⚙️  Configuring...\n");

    // Choose provider (Ollama for free local, or use API key for cloud)
    let provider = std::env::var("SENTIENT_PROVIDER")
        .unwrap_or_else(|_| "ollama".into());
    
    let model = std::env::var("SENTIENT_MODEL")
        .unwrap_or_else(|_| "llama3.2:3b".into());
    
    let api_key = std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
        .ok();

    println!("📡 Provider: {}", provider);
    println!("🤖 Model: {}", model);
    println!("💾 Memory: Enabled\n");

    // ────────────────────────────────────────────────────────────────────────
    //  Create LLM Client
    // ────────────────────────────────────────────────────────────────────────
    
    let client = match provider.as_str() {
        "openai" => LlmClient::new(LlmConfig {
            provider: "openai".into(),
            model: model.clone(),
            api_key: api_key.clone(),
            base_url: None,
        }),
        "claude" | "anthropic" => LlmClient::new(LlmConfig {
            provider: "anthropic".into(),
            model: model.clone(),
            api_key: api_key.clone(),
            base_url: None,
        }),
        _ => LlmClient::new(LlmConfig {
            provider: "ollama".into(),
            model: model.clone(),
            api_key: None,
            base_url: Some("http://localhost:11434".into()),
        }),
    };

    // ────────────────────────────────────────────────────────────────────────
    //  Create Memory
    // ────────────────────────────────────────────────────────────────────────
    
    let memory = MemoryCube::new(MemoryConfig {
        db_path: ":memory:".into(),
        max_messages: 100,
    }).await?;

    // ────────────────────────────────────────────────────────────────────────
    //  Create Agent
    // ────────────────────────────────────────────────────────────────────────
    
    let agent = Agent::new(AgentConfig {
        name: "chatbot".into(),
        description: "A helpful AI assistant with memory".into(),
        llm: client,
        system_prompt: Some(r#"You are SENTIENT, a helpful AI assistant.
You have access to the conversation history.
Be helpful, accurate, and concise.
If you don't know something, say so."#.into()),
    });

    // ────────────────────────────────────────────────────────────────────────
    //  Chat Loop
    // ────────────────────────────────────────────────────────────────────────
    
    println!("💬 Chat started! Type 'exit' to quit, 'clear' to clear history.\n");
    println!("─────────────────────────────────────────────────────────────────────\n");

    loop {
        // Read input
        print!("👤 You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Handle commands
        match input {
            "exit" | "quit" | "bye" => {
                println!("\n👋 Goodbye! Thanks for chatting with SENTIENT!");
                break;
            }
            "clear" | "reset" => {
                memory.clear().await?;
                println!("🧹 Memory cleared!\n");
                continue;
            }
            "help" => {
                println!("\n📖 Commands:");
                println!("  exit/quit/bye - Exit the chat");
                println!("  clear/reset   - Clear conversation history");
                println!("  help          - Show this help");
                println!("  history       - Show conversation history\n");
                continue;
            }
            "history" => {
                let history = memory.get_history().await?;
                println!("\n📜 Conversation History:");
                for msg in history {
                    println!("  [{}] {}", msg.role, msg.content);
                }
                println!();
                continue;
            }
            "" => continue,
            _ => {}
        }

        // Save user message
        memory.add_message("user", input).await?;

        // Get response
        print!("🤖 SENTIENT: ");
        io::stdout().flush()?;

        let response = agent
            .chat(Message::user(input))
            .await?;

        println!("{}\n", response.content);

        // Save assistant response
        memory.add_message("assistant", &response.content).await?;

        println!("─────────────────────────────────────────────────────────────────────\n");
    }

    Ok(())
}
