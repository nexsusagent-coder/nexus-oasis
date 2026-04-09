//! Web API Example
//!
//! A REST API server for SENTIENT AI.
//!
//! # Endpoints
//! - POST /chat - Chat with AI
//! - POST /stream - Stream responses
//! - GET /health - Health check
//! - GET /models - List available models
//!
//! # Usage
//! ```bash
//! cargo run --example web-api
//! curl -X POST http://localhost:3000/chat -d '{"message": "Hello!"}'
//! ```

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, sse::Event},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

/// Application state
#[derive(Clone)]
struct AppState {
    api_key: String,
    sessions: Arc<RwLock<HashMap<String, Vec<Message>>>>,
}

/// Chat request
#[derive(Debug, Deserialize)]
struct ChatRequest {
    message: String,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default = "default_model")]
    model: String,
}

fn default_model() -> String {
    "gpt-4o".to_string()
}

/// Chat response
#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
    session_id: String,
    model: String,
    usage: Usage,
}

#[derive(Debug, Serialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

/// Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// Health response
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    uptime_secs: u64,
}

/// Models response
#[derive(Debug, Serialize)]
struct ModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize)]
struct ModelInfo {
    id: String,
    name: String,
    provider: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🌐 SENTIENT Web API Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY");

    let state = AppState {
        api_key,
        sessions: Arc::new(RwLock::new(HashMap::new())),
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/models", get(list_models))
        .route("/chat", post(chat))
        .route("/chat/:session_id", get(get_session))
        .route("/chat/:session_id", delete(delete_session))
        .route("/stream", post(stream_chat))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    println!("📡 Server starting on {}", addr);
    println!("\n📚 Endpoints:");
    println!("  GET  /health          - Health check");
    println!("  GET  /models          - List models");
    println!("  POST /chat            - Chat with AI");
    println!("  GET  /chat/:id        - Get session history");
    println!("  DELETE /chat/:id      - Delete session");
    println!("  POST /stream          - Stream responses");
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
        uptime_secs: 0, // In production, track actual uptime
    })
}

/// List available models
async fn list_models() -> Json<ModelsResponse> {
    Json(ModelsResponse {
        models: vec![
            ModelInfo {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                provider: "openai".to_string(),
            },
            ModelInfo {
                id: "gpt-4-turbo".to_string(),
                name: "GPT-4 Turbo".to_string(),
                provider: "openai".to_string(),
            },
            ModelInfo {
                id: "gpt-3.5-turbo".to_string(),
                name: "GPT-3.5 Turbo".to_string(),
                provider: "openai".to_string(),
            },
            ModelInfo {
                id: "claude-3-opus".to_string(),
                name: "Claude 3 Opus".to_string(),
                provider: "anthropic".to_string(),
            },
            ModelInfo {
                id: "claude-3-sonnet".to_string(),
                name: "Claude 3 Sonnet".to_string(),
                provider: "anthropic".to_string(),
            },
        ],
    })
}

/// Chat endpoint
async fn chat(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    let session_id = request.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    println!("💬 Chat request: session={}, msg={}", 
        session_id, 
        request.message.chars().take(50).collect::<String>()
    );

    // In production, this would call the actual LLM API
    let response = simulate_llm_response(&request.message);

    // Store in session
    let mut sessions = state.sessions.write().await;
    let session = sessions.entry(session_id.clone()).or_insert_with(Vec::new);
    session.push(Message {
        role: "user".to_string(),
        content: request.message.clone(),
    });
    session.push(Message {
        role: "assistant".to_string(),
        content: response.clone(),
    });

    Ok(Json(ChatResponse {
        response,
        session_id,
        model: request.model,
        usage: Usage {
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
        },
    }))
}

/// Get session history
async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let sessions = state.sessions.read().await;
    match sessions.get(&session_id) {
        Some(history) => Ok(Json(history.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete session
async fn delete_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> StatusCode {
    let mut sessions = state.sessions.write().await;
    if sessions.remove(&session_id).is_some() {
        println!("🗑️ Deleted session: {}", session_id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

/// Stream chat (SSE)
async fn stream_chat(
    State(_state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<axum::response::Sse<impl futures::Stream<Item = Result<Event, anyhow::Error>>>, StatusCode> {
    use futures::stream::{self, StreamExt};

    println!("🌊 Stream request: {}", request.message.chars().take(50).collect::<String>());

    let words: Vec<&str> = "This is a simulated streaming response from SENTIENT AI. In production, this would stream actual tokens from the LLM API.".split_whitespace().collect();

    let stream = futures::stream::iter(words)
        .enumerate()
        .map(|(i, word)| {
            Ok(Event::default()
                .data(format!("{{\"token\": \"{}\", \"index\": {}}}", word, i)))
        })
        .throttle(Duration::from_millis(100));

    Ok(axum::response::Sse::new(stream))
}

/// Simulate LLM response
fn simulate_llm_response(message: &str) -> String {
    format!(
        "I received your message: \"{}\". In production, this would be a real AI response.",
        message.chars().take(100).collect::<String>()
    )
}

// Add uuid dependency inline for simplicity
mod uuid {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> Self {
            Uuid
        }
        
        pub fn to_string(&self) -> String {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            format!("{:016x}", timestamp)
        }
    }
}
