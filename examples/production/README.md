# Production Example

Complete production-ready SENTIENT application with all best practices.

## Features

| Feature | Implementation |
|---------|---------------|
| **Configuration** | Environment variables + defaults |
| **Logging** | Structured logging with `tracing` |
| **Error Handling** | Proper error types and propagation |
| **Content Filtering** | PII detection, prompt injection |
| **Memory** | Persistent SQLite storage |
| **Metrics** | Prometheus-compatible metrics |
| **Thread Safety** | Arc<RwLock<AppState>> |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application                                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐          │
│  │   Config     │    │   Logging    │    │   Metrics    │          │
│  │              │    │              │    │              │          │
│  │ Environment  │    │  Tracing     │    │  Prometheus  │          │
│  └──────────────┘    └──────────────┘    └──────────────┘          │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                      AppState (Arc<RwLock>)                   │  │
│  │                                                               │  │
│  │  Agent ┃ Memory ┃ Guardrails ┃ Metrics                        │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Prerequisites

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh
ollama serve &
ollama pull llama3.2:3b

# Optional: Prometheus for metrics
docker run -d -p 9090:9090 prom/prometheus
```

## Configuration

Create `.env` file or set environment variables:

```bash
# LLM Configuration
SENTIENT_PROVIDER=ollama       # or openai, anthropic
SENTIENT_MODEL=llama3.2:3b     # or gpt-4o, claude-3-5-sonnet
OLLAMA_URL=http://localhost:11434

# Cloud providers (optional)
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...

# Application
SENTIENT_DB_PATH=sentient.db
SENTIENT_MAX_HISTORY=100
SENTIENT_CONTENT_FILTER=true
SENTIENT_METRICS=true
SENTIENT_LOG_LEVEL=info
```

## Run

```bash
cd examples/production
cargo run
```

## Expected Output

```
╔════════════════════════════════════════════════════════════════╗
║             SENTIENT OS - Production Example                   ║
╚════════════════════════════════════════════════════════════════╝

⚙️  Configuration:
   Provider: ollama
   Model: llama3.2:3b
   Content Filter: true
   Metrics: true

🧠 Initializing SENTIENT Production App
📡 Provider: ollama
🤖 Model: llama3.2:3b
💾 Memory initialized: sentient.db
✅ Application initialized successfully

👤 User: Explain the benefits of Rust in 3 bullet points

💬 Chat request from user: demo-user
✅ Chat completed in 1234ms

🤖 SENTIENT: Here are the key benefits of Rust:

• **Memory Safety**: Rust prevents common bugs like null pointers and buffer overflows at compile time, without needing a garbage collector.

• **Zero-Cost Abstractions**: You can use high-level features like iterators and closures without runtime overhead.

• **Fearless Concurrency**: Rust's ownership system makes it easy to write correct concurrent code without data races.

⏱️  Processing time: 1234ms
🔒 Filtered: false

════════════════════════════════════════════════════════════════
✅ Production example complete!

Key features demonstrated:
  • Configuration management
  • Structured logging (tracing)
  • Error handling
  • Content filtering (guardrails)
  • Memory persistence
  • Metrics collection
  • Thread-safe state (Arc<RwLock>)
```

## Production Checklist

Before deploying to production:

- [ ] Set `SENTIENT_LOG_LEVEL=warn` or `error`
- [ ] Enable `SENTIENT_CONTENT_FILTER=true`
- [ ] Set up Prometheus metrics endpoint
- [ ] Configure persistent database path
- [ ] Add authentication/authorization
- [ ] Set up rate limiting
- [ ] Configure CORS for web clients
- [ ] Add health check endpoint
- [ ] Set up backup for database
- [ ] Configure SSL/TLS

## Extending

### Add REST API

```rust
use axum::{Router, Json, extract::State};

async fn chat_handler(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, Error> {
    let response = chat(state, request).await?;
    Ok(Json(response))
}

let app = Router::new()
    .route("/chat", post(chat_handler))
    .with_state(state);
```

### Add WebSocket

```rust
use axum::extract::ws::{WebSocket, WebSocketUpgrade};

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}
```

## License

Apache 2.0
