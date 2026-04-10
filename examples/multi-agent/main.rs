// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Multi-Agent Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Multiple specialized agents working together
// ═══════════════════════════════════════════════════════════════════════════════

use sentient_core::{Agent, AgentConfig, Message, Task};
use sentient_gateway::{LlmClient, LlmConfig};
use sentient_orchestrator::{Orchestrator, OrchestratorConfig, AgentRole};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 SENTIENT OS - Multi-Agent Example");
    println!("═════════════════════════════════════\n");

    // ────────────────────────────────────────────────────────────────────────
    //  Create LLM Client
    // ────────────────────────────────────────────────────────────────────────
    
    let client = LlmClient::new(LlmConfig {
        provider: "ollama".into(),
        model: "llama3.2:3b".into(),
        api_key: None,
        base_url: Some("http://localhost:11434".into()),
    });

    println!("📡 Provider: Ollama");
    println!("🤖 Model: llama3.2:3b\n");

    // ────────────────────────────────────────────────────────────────────────
    //  Create Specialized Agents
    // ────────────────────────────────────────────────────────────────────────
    
    println!("👤 Creating specialized agents...\n");

    // Researcher Agent
    let researcher = Agent::new(AgentConfig {
        name: "researcher".into(),
        description: "Researches topics and gathers information".into(),
        llm: client.clone(),
        system_prompt: Some(r#"You are a research specialist.
Your job is to gather and summarize information on given topics.
Be thorough but concise. Present facts objectively."#.into()),
    });

    // Writer Agent
    let writer = Agent::new(AgentConfig {
        name: "writer".into(),
        description: "Writes clear and engaging content".into(),
        llm: client.clone(),
        system_prompt: Some(r#"You are a professional writer.
Your job is to take research and create well-structured, engaging content.
Use clear language and good formatting."#.into()),
    });

    // Editor Agent
    let editor = Agent::new(AgentConfig {
        name: "editor".into(),
        description: "Reviews and improves content".into(),
        llm: client.clone(),
        system_prompt: Some(r#"You are a professional editor.
Your job is to review content for:
- Clarity and coherence
- Grammar and style
- Accuracy
Provide constructive feedback and improvements."#.into()),
    });

    println!("✅ Created 3 agents:");
    println!("   📚 Researcher - Gathers information");
    println!("   ✍️  Writer - Creates content");
    println!("   🔍 Editor - Reviews and improves\n");

    // ────────────────────────────────────────────────────────────────────────
    //  Create Orchestrator
    // ────────────────────────────────────────────────────────────────────────
    
    let orchestrator = Orchestrator::new(OrchestratorConfig {
        name: "content-team".into(),
        agents: vec![
            (AgentRole::Worker, researcher),
            (AgentRole::Worker, writer),
            (AgentRole::Supervisor, editor),
        ],
    });

    // ────────────────────────────────────────────────────────────────────────
    //  Execute Multi-Agent Workflow
    // ────────────────────────────────────────────────────────────────────────
    
    let task = "Write a short article about the benefits of Rust programming language";

    println!("📋 Task: {}\n", task);
    println!("─────────────────────────────────────────────────────────────────────\n");

    // Step 1: Research
    println!("📚 Researcher: Gathering information...\n");
    let research = orchestrator
        .assign("researcher", Message::user(&format!("Research: {}", task)))
        .await?;
    
    println!("📊 Research Summary:");
    println!("{}\n", research.content);
    println!("─────────────────────────────────────────────────────────────────────\n");

    // Step 2: Write
    println!("✍️  Writer: Creating content...\n");
    let draft = orchestrator
        .assign("writer", Message::user(&format!(
            "Based on this research, write an article:\n\n{}\n\nTopic: {}",
            research.content, task
        )))
        .await?;
    
    println!("📝 Draft Article:");
    println!("{}\n", draft.content);
    println!("─────────────────────────────────────────────────────────────────────\n");

    // Step 3: Edit
    println!("🔍 Editor: Reviewing and improving...\n");
    let final_version = orchestrator
        .assign("editor", Message::user(&format!(
            "Review and improve this article:\n\n{}",
            draft.content
        )))
        .await?;
    
    println!("✨ Final Article:");
    println!("{}\n", final_version.content);
    println!("─────────────────────────────────────────────────────────────────────\n");

    println!("═════════════════════════════════════");
    println!("✅ Multi-Agent workflow complete!");
    println!("   Research → Write → Edit");

    Ok(())
}
