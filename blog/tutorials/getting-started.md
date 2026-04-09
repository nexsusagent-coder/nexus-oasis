# Getting Started with SENTIENT

*Tutorial*

---

This tutorial will guide you through setting up SENTIENT and creating your first AI agent.

## Prerequisites

- Rust 1.75+ (for building from source)
- Or use pre-built binaries
- OpenAI API key (or other LLM provider)

## Installation

### Option 1: Quick Install (Recommended)

```bash
# Linux / macOS
curl -sSL https://get.sentient.ai | bash

# Restart your shell
source ~/.bashrc  # or ~/.zshrc
```

### Option 2: npm

```bash
npm install -g @sentient/ai
```

### Option 3: Docker

```bash
docker pull sentient/ai:latest
docker run -it -e OPENAI_API_KEY=sk-... sentient/ai:latest
```

### Option 4: Build from Source

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
cargo build --release
./target/release/sentient setup
```

## Initial Setup

```bash
# Run the setup wizard
sentient setup

# This will:
# 1. Create ~/.config/sentient/ directory
# 2. Ask for your API keys
# 3. Configure default model
# 4. Set up memory storage
```

### Configuration File

```toml
# ~/.config/sentient/config.toml

[llm]
provider = "openai"
model = "gpt-4o"
api_key = "sk-..."

[memory]
type = "lancedb"
path = "~/.config/sentient/memory"

[channels]
telegram = { enabled = false }
discord = { enabled = false }

[voice]
enabled = false
```

## Your First Agent

### Interactive REPL

```bash
# Start the REPL
sentient repl

# Chat with the agent
>>> Hello! I'm your first SENTIENT agent.
>>> What can you do?
>>> I can help you with tasks, answer questions, and execute tools!
>>> Exit with 'quit' or Ctrl+D
```

### Autonomous Agent

```bash
# Start an autonomous agent with a goal
sentient agent --goal "Research the latest developments in quantum computing and create a summary report"

# The agent will:
# 1. Plan the research steps
# 2. Search for information
# 3. Synthesize findings
# 4. Create a report
```

### Programmatic Usage

```rust
use sentient_core::{Agent, Config, Message};

#[tokio::main]
async fn main() -> Result<()> {
    // Create agent
    let config = Config::from_file("config.toml")?;
    let agent = Agent::new(config).await?;

    // Send message
    let response = agent
        .send(Message::user("What is the capital of Turkey?"))
        .await?;

    println!("{}", response.content);

    // Execute with tools
    let result = agent
        .execute("Search for recent AI news and summarize")
        .await?;

    println!("{}", result);

    Ok(())
}
```

## Adding Channels

### Telegram

```bash
# Configure Telegram
sentient channel add telegram --token "YOUR_BOT_TOKEN"

# Start the bot
sentient channel start telegram

# Your agent is now on Telegram!
```

### Discord

```bash
# Configure Discord
sentient channel add discord \
    --token "YOUR_BOT_TOKEN" \
    --client-id "YOUR_CLIENT_ID"

# Start the bot
sentient channel start discord
```

### Web API

```bash
# Start the REST API
sentient gateway --port 8080

# Make requests
curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello, SENTIENT!"}'
```

## Voice Integration

```bash
# Enable voice
sentient voice enable

# Configure STT
sentient voice stt set whisper --api-key "sk-..."

# Configure TTS
sentient voice tts set openai --api-key "sk-..."

# Start voice agent
sentient voice start --wake-word "Hey SENTIENT"
```

## Skills

### Install a Skill

```bash
# List available skills
sentient skill search

# Install a skill
sentient skill install web-scraper

# Use the skill
sentient repl
>>> Use web-scraper to get headlines from news.ycombinator.com
```

### Create a Custom Skill

```bash
# Create skill scaffold
sentient skill create my-skill

# Edit the skill
cd my-skill
# Edit skill.toml and src/lib.rs

# Test locally
sentient skill test

# Publish to marketplace
sentient skill publish
```

## Memory & Context

SENTIENT uses LanceDB for persistent memory:

```bash
# View memory stats
sentient memory stats

# Search memory
sentient memory search "previous conversations about AI"

# Clear memory
sentient memory clear
```

## Monitoring

### Status

```bash
# Check system status
sentient status

# Output:
# Agent: Running
# Memory: 1.2GB / 2GB
# Channels: Telegram (active), Discord (active)
# Last activity: 2 minutes ago
```

### Logs

```bash
# View logs
sentient logs --follow

# Filter logs
sentient logs --level error --since 1h
```

### Metrics

```bash
# Start with Prometheus metrics
sentient gateway --metrics --metrics-port 9090

# Metrics available at http://localhost:9090/metrics
```

## Next Steps

- [Create Your First AI Agent](./first-agent.md)
- [Build Custom Skills](./custom-skills.md)
- [Deploy to Production](./kubernetes-deployment.md)
- [Multi-Agent Workflows](./multi-agent.md)

## Troubleshooting

### "API key not found"

```bash
# Set environment variable
export OPENAI_API_KEY="sk-..."

# Or add to config
sentient config set openai.api_key "sk-..."
```

### "Memory allocation failed"

```bash
# Increase memory limit
sentient config set memory.limit 4GB
```

### "Channel connection failed"

```bash
# Check channel status
sentient channel status telegram

# Reconnect
sentient channel restart telegram
```

---

*Need help? [Open an issue](https://github.com/nexsusagent-coder/SENTIENT_CORE/issues) or [start a discussion](https://github.com/nexsusagent-coder/SENTIENT_CORE/discussions)!*
