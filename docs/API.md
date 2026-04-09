# SENTIENT AI - API Reference

Complete API documentation for SENTIENT AI Operating System.

---

## Table of Contents

1. [Core API](#core-api)
2. [Channels API](#channels-api)
3. [Voice API](#voice-api)
4. [Memory API](#memory-api)
5. [Agent API](#agent-api)
6. [Skills API](#skills-api)
7. [Kubernetes API](#kubernetes-api)

---

## Core API

### LLM Provider

```rust
use sentient_core::LlmClient;

// Create client
let client = LlmClient::new("openai/gpt-4o")?;

// Chat completion
let response = client.chat()
    .system("You are a helpful assistant.")
    .user("Hello!")
    .send()
    .await?;

println!("{}", response.content);
```

### Streaming

```rust
let mut stream = client.chat()
    .user("Tell me a story")
    .stream()
    .await?;

while let Some(chunk) = stream.next().await {
    print!("{}", chunk.delta);
}
```

### Model Selection

```rust
// Supported formats:
// - openai/gpt-4o
// - anthropic/claude-3-5-sonnet
// - google/gemini-2.0-flash
// - ollama/llama3.3
// - openrouter/anthropic/claude-3-opus

let client = LlmClient::new("anthropic/claude-3-5-sonnet")?;
```

---

## Channels API

### Telegram

```rust
use sentient_channels::{TelegramBot, ChannelConfig};

let bot = TelegramBot::new("BOT_TOKEN").await?;

// Send message
bot.send_message(chat_id, "Hello!").await?;

// Handle updates
bot.on_message(|msg| async move {
    println!("Received: {}", msg.text);
    Ok(())
}).await?;
```

### Discord

```rust
use sentient_channels::{DiscordBot, DiscordConfig};

let bot = DiscordBot::new("BOT_TOKEN").await?;

// Send to channel
bot.send(channel_id, "Hello Discord!").await?;

// Slash commands
bot.register_command(
    SlashCommand::new("ping", "Ping pong")
        .handler(|ctx| async move {
            ctx.reply("Pong!").await
        })
).await?;
```

### WhatsApp Business

```rust
use sentient_channels::{WhatsAppClient, WhatsAppConfig};

let client = WhatsAppClient::new(
    "PHONE_ID",
    "ACCESS_TOKEN"
).await?;

// Send message
client.send_text("+1234567890", "Hello WhatsApp!").await?;

// Send template
client.send_template(
    "+1234567890",
    "welcome",
    &[("name", "John")]
).await?;
```

### Slack

```rust
use sentient_channels::{SlackBot, SlackConfig};

let bot = SlackBot::new("xoxb-...").await?;

// Send message
bot.send_message("#general", "Hello Slack!").await?;

// Send blocks
bot.send_blocks("#general", vec![
    Block::section("Hello *Slack*!"),
    Block::divider(),
    Block::actions(vec![
        Button::new("approve", "Approve").primary(),
        Button::new("deny", "Deny").danger(),
    ]),
]).await?;
```

### Matrix

```rust
use sentient_channels::{MatrixClient, MatrixConfig};

let client = MatrixClient::new(
    "https://matrix.org",
    "@sentient:matrix.org",
    "PASSWORD"
).await?;

// Join room
client.join_room("#sentient:matrix.org").await?;

// Send message
client.send_message(room_id, "Hello Matrix!").await?;

// Encrypted message (E2EE)
client.send_encrypted(room_id, "Secret message").await?;
```

---

## Voice API

### Speech-to-Text (Whisper)

```rust
use sentient_voice::{WhisperSTT, SttConfig};

let stt = WhisperSTT::new(SttConfig {
    model: "whisper-1",  // or local path
    language: Some("tr"),
    ..Default::default()
}).await?;

// Transcribe file
let text = stt.transcribe("audio.mp3").await?;

// Stream transcription
let mut stream = stt.transcribe_stream(mic_stream).await?;
while let Some(segment) = stream.next().await {
    println!("{}", segment.text);
}
```

### Text-to-Speech

```rust
use sentient_voice::{TtsEngine, TtsConfig};

let tts = TtsEngine::new(TtsConfig {
    provider: "openai",  // or "elevenlabs", "system"
    voice: "alloy",
    ..Default::default()
}).await?;

// Synthesize
let audio = tts.synthesize("Hello world!").await?;

// Stream synthesis
let mut stream = tts.synthesize_stream(long_text).await?;
while let Some(chunk) = stream.next().await {
    speaker.play(chunk).await?;
}
```

### Wake Word Detection

```rust
use sentient_wake::{WakeWordDetector, WakeConfig, Engine};

let detector = WakeWordDetector::new(WakeConfig {
    engine: Engine::Porcupine,  // or Vosk, Whisper
    keyword: "hey sentient",
    sensitivity: 0.5,
    ..Default::default()
}).await?;

// Start listening
detector.start(|detection| async move {
    println!("Wake word detected!");
    // Start voice interaction
    start_voice_session().await;
}).await?;
```

---

## Memory API

### Short-term Memory

```rust
use sentient_memory::{MemoryStore, MemoryEntry};

let store = MemoryStore::new()?;

// Store
store.store(MemoryEntry {
    key: "user_preference".into(),
    value: json!({"language": "tr"}),
    ttl: Some(Duration::hours(24)),
}).await?;

// Retrieve
let entry = store.get("user_preference").await?;
```

### Long-term Memory (LanceDB)

```rust
use sentient_memory::{LanceMemory, MemoryVector};

let memory = LanceMemory::new("./memory.db").await?;

// Store with embedding
memory.store_vector(MemoryVector {
    id: "conv-1".into(),
    content: "User asked about Rust programming".into(),
    embedding: embedding,
    metadata: json!({"user": "john", "topic": "programming"}),
}).await?;

// Semantic search
let results = memory.search(
    "programming languages",
    10  // top-k
).await?;
```

---

## Agent API

### Autonomous Agent

```rust
use sentient_orchestrator::{Agent, AgentConfig, Goal};

let agent = Agent::new(AgentConfig {
    name: "code-assistant",
    model: "anthropic/claude-3-5-sonnet",
    tools: vec!["code", "web", "file"],
    ..Default::default()
}).await?;

// Run with goal
let result = agent.run(Goal {
    description: "Build a REST API in Rust".into(),
    max_steps: 50,
    success_criteria: "API compiles and runs".into(),
}).await?;

println!("Result: {:?}", result);
```

### Multi-Agent Orchestration

```rust
use sentient_orchestrator::{Orchestrator, AgentPool};

let pool = AgentPool::new()
    .add("researcher", "openai/gpt-4o")?
    .add("coder", "anthropic/claude-3-5-sonnet")?
    .add("reviewer", "google/gemini-2.0-flash")?;

let orchestrator = Orchestrator::new(pool);

// Parallel execution
let results = orchestrator.parallel(vec![
    Task::new("research", "Find best practices"),
    Task::new("code", "Implement API"),
]).await?;

// Sequential pipeline
let result = orchestrator.pipeline(vec![
    Step::new("research", "Analyze requirements"),
    Step::new("code", "Write code"),
    Step::new("review", "Review and fix"),
]).await?;
```

---

## Skills API

### Install Skills

```rust
use sentient_skills_import::{SkillsImporter, Skill};

let importer = SkillsImporter::new();

// Search ClawHub
let skills = importer.search("translator").await?;

// Install skill
importer.install("translator-pro").await?;

// List installed
let installed = importer.list_installed().await?;
```

### Create Custom Skill

```yaml
# skill.yaml
name: "custom-skill"
version: "1.0.0"
description: "My custom skill"
author: "developer"
main: "index.js"
dependencies:
  - "axios"
config:
  properties:
    apiKey:
      type: string
      secret: true
  required:
    - apiKey
```

```javascript
// index.js
module.exports = {
  name: "custom-skill",
  
  async execute(context, ...args) {
    const { config, memory, llm } = context;
    
    // Use LLM
    const response = await llm.chat(args.join(" "));
    
    // Store in memory
    await memory.store("last_response", response);
    
    return { success: true, response };
  }
};
```

---

## Kubernetes API

### Deploy Agent

```yaml
# sentient-agent.yaml
apiVersion: sentient.ai/v1
kind: SentientAgent
metadata:
  name: production-agent
spec:
  replicas: 3
  agentType: Worker
  channels:
    - telegram
    - discord
  model:
    provider: openai
    model: gpt-4o
    apiKeySecret: openai-api-key
  resources:
    memory: 2Gi
    cpu: "1"
  voiceEnabled: true
  skills:
    - translator
    - code-review
```

### Task Dispatch

```yaml
# sentient-task.yaml
apiVersion: sentient.ai/v1
kind: SentientTask
metadata:
  name: process-message
spec:
  taskType: Channel
  input:
    channel: telegram
    message: "Hello!"
  priority: 5
  timeout: 300
  retries: 3
```

### CRD Operations

```bash
# Apply CRDs
kubectl apply -f https://sentient.ai/crds.yaml

# List agents
kubectl get sentientagents

# Scale agent
kubectl scale sentientagent production-agent --replicas=5

# View logs
kubectl logs -l app=sentient-agent
```

---

## HTTP API (Gateway)

### Start Gateway

```bash
sentient gateway --port 8080
```

### Endpoints

```bash
# Chat
POST /v1/chat
{
  "messages": [
    {"role": "user", "content": "Hello!"}
  ],
  "model": "openai/gpt-4o"
}

# Stream chat
POST /v1/chat/stream
{
  "messages": [...],
  "stream": true
}

# Voice transcription
POST /v1/voice/transcribe
Content-Type: multipart/form-data
file: audio.mp3

# Voice synthesis
POST /v1/voice/synthesize
{
  "text": "Hello world!",
  "voice": "alloy"
}

# Skills
GET /v1/skills
POST /v1/skills/install
{
  "skill_id": "translator"
}

# Agents
POST /v1/agents/run
{
  "goal": "Build a REST API",
  "model": "anthropic/claude-3-5-sonnet"
}
```

### WebSocket

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

// Send message
ws.send(JSON.stringify({
  type: 'chat',
  content: 'Hello!'
}));

// Receive stream
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data.content);
};
```

---

## Error Handling

All APIs return standard error responses:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Missing required field: model",
    "details": {
      "field": "model",
      "required": true
    }
  }
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `INVALID_REQUEST` | Invalid request parameters |
| `UNAUTHORIZED` | Missing or invalid API key |
| `RATE_LIMITED` | Too many requests |
| `MODEL_NOT_FOUND` | Requested model not available |
| `PROVIDER_ERROR` | Upstream provider error |
| `INTERNAL_ERROR` | Internal server error |

---

## Rate Limits

| Tier | Requests/min | Tokens/min |
|------|--------------|------------|
| Free | 20 | 40,000 |
| Pro | 100 | 200,000 |
| Enterprise | Unlimited | Unlimited |

---

## SDK Languages

- **Rust**: `sentient` crate
- **TypeScript/JavaScript**: `@sentient/ai`
- **Python**: `sentient-ai` (coming soon)

---

**Documentation Version: 4.0.0**
