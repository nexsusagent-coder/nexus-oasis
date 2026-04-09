//! Multi-Agent Example
//!
//! Demonstrates multiple specialized agents working together.
//!
//! # Architecture
//! ```
//! ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
//! │  Research Agent │────▶│   Writer Agent  │────▶│  Reviewer Agent │
//! └─────────────────┘     └─────────────────┘     └─────────────────┘
//! ```
//!
//! # Usage
//! ```bash
//! cargo run --example multi-agent
//! ```

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use std::env;

/// Research Agent: Gathers information
async fn create_research_agent(api_key: &str) -> anyhow::Result<Agent> {
    let config = AgentConfig {
        name: "ResearchAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key: api_key.to_string(),
        system_prompt: Some(
            "You are a research specialist. Your job is to gather and synthesize \
             information on given topics. Be thorough and cite your sources when possible. \
             Focus on facts and data.".to_string()
        ),
        ..Default::default()
    };
    Agent::new(config).await
}

/// Writer Agent: Creates content
async fn create_writer_agent(api_key: &str) -> anyhow::Result<Agent> {
    let config = AgentConfig {
        name: "WriterAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key: api_key.to_string(),
        system_prompt: Some(
            "You are a skilled writer. Your job is to take research notes and \
             create engaging, well-structured content. Focus on clarity, flow, \
             and engaging narrative.".to_string()
        ),
        ..Default::default()
    };
    Agent::new(config).await
}

/// Reviewer Agent: Quality control
async fn create_reviewer_agent(api_key: &str) -> anyhow::Result<Agent> {
    let config = AgentConfig {
        name: "ReviewerAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key: api_key.to_string(),
        system_prompt: Some(
            "You are an editor and quality reviewer. Your job is to review content \
             for accuracy, clarity, and engagement. Provide specific feedback and \
             suggest improvements. Be constructive but thorough.".to_string()
        ),
        ..Default::default()
    };
    Agent::new(config).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🤖 SENTIENT Multi-Agent Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create specialized agents
    println!("📦 Creating specialized agents...");
    let mut research_agent = create_research_agent(&api_key).await?;
    let mut writer_agent = create_writer_agent(&api_key).await?;
    let mut reviewer_agent = create_reviewer_agent(&api_key).await?;
    println!("✅ Created 3 agents: Research, Writer, Reviewer\n");

    let topic = "The impact of AI on software development in 2024";
    println!("🎯 Topic: {}\n", topic);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Phase 1: Research
    println!("🔬 Phase 1: Research Agent");
    println!("─────────────────────────");
    let research_prompt = format!(
        "Research the following topic and provide key findings, statistics, \
         and notable developments: {}",
        topic
    );
    let research_result = research_agent
        .send(Message::user(&research_prompt))
        .await?;
    println!("📋 Research findings:\n{}\n", 
        research_result.content.chars().take(500).collect::<String>()
    );

    // Phase 2: Writing
    println!("✍️ Phase 2: Writer Agent");
    println!("───────────────────────");
    let writer_prompt = format!(
        "Based on these research notes, write a comprehensive article:\n\n{}\n\n\
         Write a well-structured, engaging article of about 500 words.",
        research_result.content
    );
    let article = writer_agent
        .send(Message::user(&writer_prompt))
        .await?;
    println!("📝 Draft article:\n{}\n", 
        article.content.chars().take(400).collect::<String>()
    );

    // Phase 3: Review
    println!("🔍 Phase 3: Reviewer Agent");
    println!("─────────────────────────");
    let review_prompt = format!(
        "Review this article for quality, accuracy, and engagement:\n\n{}\n\n\
         Provide specific feedback and suggestions for improvement.",
        article.content
    );
    let review = reviewer_agent
        .send(Message::user(&review_prompt))
        .await?;
    println!("📊 Review feedback:\n{}\n", 
        review.content.chars().take(400).collect::<String>()
    );

    // Phase 4: Final revision (Writer + Review feedback)
    println!("✨ Phase 4: Final Revision");
    println!("─────────────────────────");
    let final_prompt = format!(
        "Revise your article based on this feedback:\n\n{}\n\n\
         Original article:\n{}\n\n\
         Create the final polished version.",
        review.content,
        article.content
    );
    let final_article = writer_agent
        .send(Message::user(&final_prompt))
        .await?;
    
    println!("📄 FINAL ARTICLE:\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{}", final_article.content);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Summary
    println!("📊 Multi-Agent Pipeline Summary:");
    println!("┌─────────────────┬───────────┐");
    println!("│ Agent           │ Messages  │");
    println!("├─────────────────┼───────────┤");
    println!("│ Research Agent  │     {}     │", research_agent.history().len());
    println!("│ Writer Agent    │     {}     │", writer_agent.history().len());
    println!("│ Reviewer Agent  │     {}     │", reviewer_agent.history().len());
    println!("└─────────────────┴───────────┘\n");

    println!("✅ Multi-agent example completed!");
    Ok(())
}
