// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Production Example
// ═══════════════════════════════════════════════════════════════════════════════
//  Complete production-ready application with error handling, logging,
//  configuration, and monitoring
// ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;

use sentient_core::{Agent, AgentConfig, Message, Error};
use sentient_gateway::{LlmClient, LlmConfig};
use sentient_memory::{MemoryCube, MemoryConfig};
use sentient_guardrails::{Guardrails, GuardrailsConfig, FilterResult};
use sentient_observability::{Metrics, MetricsConfig};

// ═══════════════════════════════════════════════════════════════════════════════
//  Configuration
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConfig {
    /// LLM provider (ollama, openai, anthropic)
    provider: String,
    
    /// Model to use
    model: String,
    
    /// API key (if using cloud provider)
    api_key: Option<String>,
    
    /// Base URL (for Ollama)
    base_url: Option<String>,
    
    /// Database path for memory
    db_path: String,
    
    /// Maximum conversation history
    max_history: usize,
    
    /// Enable content filtering
    content_filter: bool,
    
    /// Enable metrics
    metrics: bool,
    
    /// Log level
    log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            provider: std::env::var("SENTIENT_PROVIDER")
                .unwrap_or_else(|_| "ollama".into()),
            model: std::env::var("SENTIENT_MODEL")
                .unwrap_or_else(|_| "llama3.2:3b".into()),
            api_key: std::env::var("OPENAI_API_KEY")
                .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
                .ok(),
            base_url: std::env::var("OLLAMA_URL")
                .ok(),
            db_path: "sentient.db".into(),
            max_history: 100,
            content_filter: true,
            metrics: true,
            log_level: "info".into(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  Application State
// ═══════════════════════════════════════════════════════════════════════════════

struct AppState {
    config: AppConfig,
    agent: Agent,
    memory: MemoryCube,
    guardrails: Option<Guardrails>,
    metrics: Option<Metrics>,
}

impl AppState {
    async fn new(config: AppConfig) -> Result<Arc<RwLock<Self>>, Error> {
        // Initialize logging
        let level = match config.log_level.as_str() {
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        };
        
        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .finish();
        
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

        info!("🧠 Initializing SENTIENT Production App");

        // Create LLM client
        let client = LlmClient::new(LlmConfig {
            provider: config.provider.clone(),
            model: config.model.clone(),
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
        });

        info!("📡 Provider: {}", config.provider);
        info!("🤖 Model: {}", config.model);

        // Create memory
        let memory = MemoryCube::new(MemoryConfig {
            db_path: config.db_path.clone(),
            max_messages: config.max_history,
        }).await.map_err(|e| Error::Memory(e.to_string()))?;

        info!("💾 Memory initialized: {}", config.db_path);

        // Create guardrails
        let guardrails = if config.content_filter {
            Some(Guardrails::new(GuardrailsConfig {
                enable_pii_detection: true,
                enable_prompt_injection_detection: true,
                max_input_length: 10000,
            }))
        } else {
            None
        };

        // Create metrics
        let metrics = if config.metrics {
            Some(Metrics::new(MetricsConfig {
                endpoint: Some("http://localhost:9090/metrics".into()),
                namespace: "sentient".into(),
            }))
        } else {
            None
        };

        // Create agent
        let agent = Agent::new(AgentConfig {
            name: "production-agent".into(),
            description: "Production-ready AI assistant".into(),
            llm: client,
            system_prompt: Some(r#"You are SENTIENT, a professional AI assistant.
Follow these guidelines:
1. Be helpful and accurate
2. Admit when you don't know something
3. Keep responses concise but complete
4. Format code and lists properly"#.into()),
        });

        info!("✅ Application initialized successfully");

        Ok(Arc::new(RwLock::new(Self {
            config,
            agent,
            memory,
            guardrails,
            metrics,
        })))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  Chat Function
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    message: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    response: String,
    filtered: bool,
    processing_time_ms: u64,
}

async fn chat(
    state: Arc<RwLock<AppState>>,
    request: ChatRequest,
) -> Result<ChatResponse, Error> {
    let start = std::time::Instant::now();
    
    info!("💬 Chat request from user: {}", request.user_id);

    // Read state
    let state_guard = state.read().await;

    // Content filtering
    let filtered = if let Some(ref guardrails) = state_guard.guardrails {
        match guardrails.filter(&request.message) {
            FilterResult::Safe => false,
            FilterResult::Filtered(reason) => {
                warn!("Content filtered: {}", reason);
                return Ok(ChatResponse {
                    response: format!("Your message was filtered: {}", reason),
                    filtered: true,
                    processing_time_ms: start.elapsed().as_millis() as u64,
                });
            }
            FilterResult::Blocked => {
                warn!("Content blocked");
                return Ok(ChatResponse {
                    response: "Your message was blocked due to policy violation.".into(),
                    filtered: true,
                    processing_time_ms: start.elapsed().as_millis() as u64,
                });
            }
        }
    } else {
        false
    };

    // Get response from agent
    let response = state_guard.agent
        .chat(Message::user(&request.message))
        .await?;

    // Record metrics
    if let Some(ref metrics) = state_guard.metrics {
        metrics.record("chat_request", 1);
        metrics.record("chat_latency_ms", start.elapsed().as_millis() as f64);
    }

    // Save to memory
    drop(state_guard);
    
    let mut state_guard = state.write().await;
    state_guard.memory.add_message("user", &request.message).await
        .map_err(|e| Error::Memory(e.to_string()))?;
    state_guard.memory.add_message("assistant", &response.content).await
        .map_err(|e| Error::Memory(e.to_string()))?;

    info!("✅ Chat completed in {}ms", start.elapsed().as_millis());

    Ok(ChatResponse {
        response: response.content,
        filtered,
        processing_time_ms: start.elapsed().as_millis() as u64,
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
//  Main
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(r#"
╔════════════════════════════════════════════════════════════════╗
║             SENTIENT OS - Production Example                   ║
╚════════════════════════════════════════════════════════════════╝
"#);

    // Load configuration
    let config = AppConfig::default();
    
    println!("⚙️  Configuration:");
    println!("   Provider: {}", config.provider);
    println!("   Model: {}", config.model);
    println!("   Content Filter: {}", config.content_filter);
    println!("   Metrics: {}", config.metrics);
    println!();

    // Initialize state
    let state = AppState::new(config).await?;

    // Example usage
    let request = ChatRequest {
        message: "Explain the benefits of Rust in 3 bullet points".into(),
        user_id: "demo-user".into(),
    };

    println!("👤 User: {}", request.message);
    println!();

    let response = chat(state.clone(), request).await?;

    println!("🤖 SENTIENT: {}", response.response);
    println!();
    println!("⏱️  Processing time: {}ms", response.processing_time_ms);
    println!("🔒 Filtered: {}", response.filtered);

    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!("✅ Production example complete!");
    println!();
    println!("Key features demonstrated:");
    println!("  • Configuration management");
    println!("  • Structured logging (tracing)");
    println!("  • Error handling");
    println!("  • Content filtering (guardrails)");
    println!("  • Memory persistence");
    println!("  • Metrics collection");
    println!("  • Thread-safe state (Arc<RwLock>)");

    Ok(())
}
