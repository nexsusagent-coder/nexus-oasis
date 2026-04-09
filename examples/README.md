# 📚 Examples

This directory contains runnable examples demonstrating SENTIENT features.

## 📋 Table of Contents

| Example | Difficulty | Description |
|---------|------------|-------------|
| [basic-agent](./basic-agent/) | 🟢 Beginner | Minimal agent with chat |
| [multi-agent](./multi-agent/) | 🟡 Intermediate | Multiple specialized agents |
| [voice-agent](./voice-agent/) | 🟡 Intermediate | Voice-enabled agent |
| [telegram-bot](./telegram-bot/) | 🟡 Intermediate | Telegram bot |
| [discord-bot](./discord-bot/) | 🟡 Intermediate | Discord bot with slash commands |
| [custom-skill](./custom-skill/) | 🟡 Intermediate | Create custom skills |
| [web-api](./web-api/) | 🟡 Intermediate | REST API server |
| [streaming-agent](./streaming-agent/) | 🟡 Intermediate | Streaming responses |
| [enterprise-sso](./enterprise-sso/) | 🔴 Advanced | Enterprise features |
| [kubernetes-deployment](./kubernetes-deployment/) | 🔴 Advanced | K8s deployment |

## 🚀 Quick Start

### Prerequisites

```bash
# Set your API key
export OPENAI_API_KEY="sk-..."
```

### Run an Example

```bash
# Navigate to example
cd examples/basic-agent

# Run
cargo run
```

## 📖 Examples Overview

### 1. Basic Agent 🟢

The simplest way to use SENTIENT:

```rust
use sentient_core::{Agent, AgentConfig, Message};

let mut agent = Agent::new(config).await?;
let response = agent.send(Message::user("Hello!")).await?;
```

**Learn:** Agent creation, configuration, basic chat

---

### 2. Multi-Agent 🟡

Multiple agents working together:

```
Research Agent → Writer Agent → Reviewer Agent
```

**Learn:** Agent orchestration, pipeline patterns, specialized agents

---

### 3. Voice Agent 🟡

Voice-enabled agent with wake word:

```rust
// Wait for "Hey SENTIENT"
wake_detector.wait_for_wake_word().await?;

// Listen and transcribe
let text = voice_engine.listen_and_transcribe().await?;

// Speak response
voice_engine.speak(&response).await?;
```

**Learn:** STT/TTS, wake word detection, audio processing

---

### 4. Telegram Bot 🟡

Deploy to Telegram:

```rust
let mut telegram = TelegramChannel::new(config);
telegram.start_polling(|update| async {
    // Handle message
}).await?;
```

**Learn:** Channel integration, webhooks, session management

---

### 5. Discord Bot 🟡

Discord bot with slash commands:

```rust
// Register /chat command
command.name("chat")
    .description("Chat with SENTIENT AI")
    .create_option(|opt| opt.name("message"))
```

**Learn:** Discord API, slash commands, interactions

---

### 6. Custom Skill 🟡

Create your own tools:

```rust
#[async_trait]
impl Skill for WeatherSkill {
    async fn execute(&self, ctx: &SkillContext, args: Value) -> Result<SkillResult> {
        // Your logic here
    }
}
```

**Learn:** Skill development, tool calling, parameter handling

---

### 7. Web API 🟡

REST API server:

```
POST /chat        - Chat with AI
POST /stream      - Stream responses
GET  /health      - Health check
GET  /models      - List models
```

**Learn:** Axum, routing, middleware, streaming

---

### 8. Streaming Agent 🟡

Real-time token streaming:

```rust
let mut stream = agent.stream(Message::user("...")).await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(StreamResponse::Token(token)) => print!("{}", token),
        _ => {}
    }
}
```

**Learn:** Async streams, SSE, real-time updates

---

### 9. Enterprise SSO 🔴

Enterprise features:

- **SSO**: Okta, Auth0, Azure AD, Google, Keycloak
- **RBAC**: Role-based access control
- **Audit**: Compliance logging
- **Multi-tenancy**: Tenant isolation

**Learn:** Enterprise patterns, authentication, authorization

---

### 10. Kubernetes Deployment 🔴

Production deployment:

- Deployment (3 replicas)
- HPA (auto-scaling)
- Custom Resource Definitions
- Ingress with TLS

**Learn:** Kubernetes, operators, CRDs, scaling

## 🛠️ Common Patterns

### Error Handling

```rust
match agent.send(message).await {
    Ok(response) => println!("{}", response.content),
    Err(SentientError::RateLimitExceeded) => {
        // Wait and retry
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Configuration

```rust
let config = AgentConfig {
    name: "MyAgent".to_string(),
    llm_provider: LlmProvider::OpenAI,
    llm_model: "gpt-4o".to_string(),
    api_key: env::var("OPENAI_API_KEY")?,
    system_prompt: Some("You are helpful.".to_string()),
    temperature: 0.7,
    max_tokens: 2000,
    ..Default::default()
};
```

### Session Management

```rust
struct SessionManager {
    sessions: RwLock<HashMap<String, Agent>>,
}

impl SessionManager {
    async fn get_or_create(&self, user_id: &str) -> Agent {
        // Get or create session
    }
}
```

## 🧪 Testing

```bash
# Run all examples
cargo run --all

# Run specific example
cargo run -p basic-agent

# Run with logging
RUST_LOG=debug cargo run -p basic-agent
```

## 📦 Dependencies

Most examples require:

```toml
[dependencies]
sentient_core = { path = "../../crates/sentient_core" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

## 🔗 Related

- [Documentation](../docs/)
- [Tutorials](../blog/tutorials/)
- [API Reference](../docs/API.md)

---

**Questions?** [Open an issue](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues) or [start a discussion](https://github.com/nexsusagent-coder/SENTIENT_CORE/discussions)!
