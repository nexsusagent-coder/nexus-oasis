//! Discord Bot Example
//!
//! A Discord bot powered by SENTIENT AI with slash commands.
//!
//! # Usage
//! ```bash
//! DISCORD_TOKEN=your_token DISCORD_APP_ID=your_app_id cargo run --example discord-bot
//! ```
//!
//! # Slash Commands
//! - /chat <message> - Chat with AI
//! - /reset - Reset conversation
//! - /help - Show help

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use serenity::{
    async_trait,
    model::{
        application::{Interaction, InteractionResponseType},
        gateway::Ready,
        id::ChannelId,
    },
    prelude::*,
};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

struct Bot {
    api_key: String,
    sessions: Arc<RwLock<HashMap<u64, Agent>>>,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🤖 {} is connected!", ready.user.name);

        // Register slash commands
        let commands = vec![
            serenity::model::application::Command::create_global_application_command(
                &ctx.http,
                |command| {
                    command
                        .name("chat")
                        .description("Chat with SENTIENT AI")
                        .create_option(|option| {
                            option
                                .name("message")
                                .description("Your message to the AI")
                                .kind(serenity::model::application::CommandOptionType::String)
                                .required(true)
                        })
                },
            ),
            serenity::model::application::Command::create_global_application_command(
                &ctx.http,
                |command| {
                    command.name("reset").description("Reset conversation history")
                },
            ),
        ];

        for cmd in commands {
            println!("📝 Registered command: {}", cmd.name);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "chat" => {
                    let message = command
                        .data
                        .options
                        .first()
                        .and_then(|opt| opt.value.as_ref())
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    println!("💬 {}: {}", command.user.name, message);

                    // Get or create agent session
                    let agent = self.get_or_create_session(command.user.id.0).await;

                    match agent {
                        Ok(mut agent) => {
                            match agent.send(Message::user(message)).await {
                                Ok(response) => response.content,
                                Err(e) => format!("Error: {}", e),
                            }
                        }
                        Err(e) => format!("Failed to create session: {}", e),
                    }
                }
                "reset" => {
                    let mut sessions = self.sessions.write().await;
                    sessions.remove(&command.user.id.0);
                    "🔄 Conversation reset!".to_string()
                }
                _ => "Unknown command".to_string(),
            };

            // Send response
            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| data.content(content))
                })
                .await
            {
                println!("❌ Failed to send response: {}", e);
            }
        }
    }
}

impl Bot {
    async fn get_or_create_session(&self, user_id: u64) -> anyhow::Result<Agent> {
        let sessions = self.sessions.read().await;
        if let Some(agent) = sessions.get(&user_id) {
            return Ok(agent.clone());
        }
        drop(sessions);

        let config = AgentConfig {
            name: format!("DiscordUser-{}", user_id),
            llm_provider: LlmProvider::OpenAI,
            llm_model: "gpt-4o".to_string(),
            api_key: self.api_key.clone(),
            system_prompt: Some(
                "You are SENTIENT, a helpful AI assistant on Discord. \
                 Be friendly and helpful. Use Discord markdown formatting \
                 when appropriate (code blocks, bold, etc.).".to_string()
            ),
            ..Default::default()
        };

        let agent = Agent::new(config).await?;

        let mut sessions = self.sessions.write().await;
        sessions.insert(user_id, agent.clone());

        Ok(agent)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🤖 SENTIENT Discord Bot Example\n");

    let discord_token = env::var("DISCORD_TOKEN")
        .expect("Please set DISCORD_TOKEN environment variable");
    let openai_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    let bot = Bot {
        api_key: openai_key,
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };

    let mut client = Client::builder(discord_token, GatewayIntents::empty())
        .event_handler(bot)
        .await?;

    println!("✅ Bot initialized");
    println!("📡 Connecting to Discord...\n");

    client.start().await?;

    Ok(())
}
