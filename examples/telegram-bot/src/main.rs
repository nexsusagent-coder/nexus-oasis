//! Telegram Bot Example
//!
//! A Telegram bot powered by SENTIENT AI.
//!
//! # Usage
//! ```bash
//! TELEGRAM_BOT_TOKEN=your_token cargo run --example telegram-bot
//! ```
//!
//! # Commands
//! - /start - Start the bot
//! - /help - Show help
//! - /reset - Reset conversation
//! - Any text - Chat with AI

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use sentient_channels::{TelegramChannel, TelegramConfig, TelegramUpdate};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

/// User session manager
struct SessionManager {
    sessions: RwLock<HashMap<i64, Agent>>,
    api_key: String,
}

impl SessionManager {
    fn new(api_key: String) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            api_key,
        }
    }

    async fn get_or_create(&self, user_id: i64) -> anyhow::Result<Agent> {
        let sessions = self.sessions.read().await;
        if sessions.contains_key(&user_id) {
            // Return a clone - in production, you'd handle this differently
            drop(sessions);
            let mut sessions = self.sessions.write().await;
            return Ok(sessions.get(&user_id).cloned().unwrap());
        }
        drop(sessions);

        // Create new session
        let config = AgentConfig {
            name: format!("TelegramUser-{}", user_id),
            llm_provider: LlmProvider::OpenAI,
            llm_model: "gpt-4o".to_string(),
            api_key: self.api_key.clone(),
            system_prompt: Some(
                "You are a helpful AI assistant on Telegram. \
                 Be friendly, concise, and helpful. \
                 Use emojis occasionally for a friendly tone.".to_string()
            ),
            ..Default::default()
        };

        let agent = Agent::new(config).await?;

        let mut sessions = self.sessions.write().await;
        sessions.insert(user_id, agent.clone());

        Ok(agent)
    }

    async fn reset(&self, user_id: i64) -> anyhow::Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&user_id);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🤖 SENTIENT Telegram Bot Example\n");

    // Get configuration
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN")
        .expect("Please set TELEGRAM_BOT_TOKEN environment variable");
    let openai_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create session manager
    let sessions = Arc::new(SessionManager::new(openai_key));

    // Create Telegram channel
    let config = TelegramConfig {
        token: telegram_token,
        ..Default::default()
    };
    let mut telegram = TelegramChannel::new(config);

    println!("✅ Bot initialized");
    println!("📡 Starting polling...\n");

    // Start polling for updates
    telegram.start_polling(|update| {
        let sessions = sessions.clone();
        async move {
            handle_update(update, &sessions).await
        }
    }).await?;

    Ok(())
}

async fn handle_update(
    update: TelegramUpdate,
    sessions: &SessionManager,
) -> anyhow::Result<()> {
    let message = match update.message {
        Some(m) => m,
        None => return Ok(()),
    };

    let chat_id = message.chat.id;
    let user_id = message.from.id;
    let text = message.text.unwrap_or_default();

    // Handle commands
    if text.starts_with('/') {
        let command = text.split_whitespace().next().unwrap_or("");
        let args = text.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");

        match command {
            "/start" => {
                println!("📱 User {} started the bot", user_id);
                // Send welcome message via Telegram API
                return Ok(());
            }
            "/help" => {
                println!("❓ User {} requested help", user_id);
                // Send help message
                return Ok(());
            }
            "/reset" => {
                sessions.reset(user_id).await?;
                println!("🔄 Reset session for user {}", user_id);
                // Send confirmation
                return Ok(());
            }
            _ => {
                println!("❓ Unknown command: {}", command);
                return Ok(());
            }
        }
    }

    // Process message with AI
    println!("👤 User {}: {}", user_id, text);

    let mut agent = sessions.get_or_create(user_id).await?;
    let response = agent.send(Message::user(&text)).await?;

    println!("🤖 Agent: {}", response.content.chars().take(100).collect::<String>());

    // Send response via Telegram API
    // telegram.send_message(chat_id, &response.content).await?;

    Ok(())
}
