# SENTIENT AI - Getting Started Guide

Welcome to SENTIENT AI Operating System! This guide will help you get up and running quickly.

---

## Prerequisites

- **OS**: Linux, macOS, or Windows
- **RAM**: 8GB minimum (16GB+ recommended)
- **Disk**: 5GB+ free space
- **API Key**: OpenAI, Anthropic, or other LLM provider

---

## Installation

### Option 1: Quick Install (Recommended)

**Linux / macOS:**
```bash
curl -sSL https://get.sentient.ai | bash
```

**Windows (PowerShell):**
```powershell
irm https://get.sentient.ai/ps | iex
```

### Option 2: npm

```bash
npm install -g @sentient/ai
```

### Option 3: Binary Download

Download from [GitHub Releases](https://github.com/nexsusagent-coder/SENTIENT_CORE/releases):

- `sentient-linux-x86_64.tar.gz`
- `sentient-macos-arm64.tar.gz`
- `sentient-windows-x86_64.zip`

### Option 4: Build from Source

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE
cd SENTIENT_CORE
cargo build --release
./target/release/sentient-cli setup
```

---

## First-Time Setup

Run the interactive setup wizard:

```bash
sentient setup
```

This will guide you through:

1. **API Key Configuration**
   ```
   Enter your OpenAI API key: sk-...
   Enter your Anthropic API key (optional): sk-ant-...
   ```

2. **Model Selection**
   ```
   Select default model:
   1. openai/gpt-4o
   2. anthropic/claude-3-5-sonnet
   3. google/gemini-2.0-flash
   4. ollama/llama3.3 (local)
   
   Choice [1-4]: 1
   ```

3. **Channel Setup** (optional)
   ```
   Configure Telegram? [y/N]: y
   Enter Telegram Bot Token: 123456:ABC...
   
   Configure Discord? [y/N]: n
   ```

4. **Voice Setup** (optional)
   ```
   Enable voice? [y/N]: y
   Wake word: hey sentient
   ```

---

## Basic Usage

### Interactive REPL

Start an interactive chat session:

```bash
sentient repl
```

```
SENTIENT v4.0.0 | Model: openai/gpt-4o
Type /help for commands

You: Hello, who are you?
AI: I'm SENTIENT, an AI assistant powered by GPT-4o. How can I help you today?

You: Write a haiku about coding
AI: Keyboard clicks softly,
    Logic flows through fingertips,
    Bugs become features.

You: /exit
Goodbye!
```

### Single Query

Ask a single question:

```bash
sentient ask "What is the capital of Turkey?"
```

### File Analysis

Analyze files:

```bash
sentient analyze README.md
sentient analyze src/ --summary
```

### Code Generation

Generate code:

```bash
sentient code "Create a REST API in Rust with Axum"
sentient code "Write a Python script to scrape a website" --output scraper.py
```

---

## Autonomous Agents

### Run with Goal

Let SENTIENT work autonomously:

```bash
sentient agent --goal "Build a REST API with user authentication"
```

### With Constraints

Add constraints:

```bash
sentient agent \
  --goal "Build a web scraper" \
  --constraints "Use Python, respect robots.txt, add rate limiting" \
  --max-steps 20
```

### Multi-Step Projects

```bash
sentient agent \
  --goal "Create a complete web application" \
  --plan "research,design,implement,test,document" \
  --output ./my-app
```

---

## Channels

### Telegram Bot

```bash
# Configure
sentient channel add telegram --token "123456:ABC..."

# Start bot
sentient channel start telegram

# Send test message
sentient channel send telegram @mychat "Hello from SENTIENT!"
```

### Discord Bot

```bash
# Configure
sentient channel add discord --token "Bot token"

# Register slash commands
sentient channel discord register

# Start bot
sentient channel start discord
```

### WhatsApp Business

```bash
# Configure
sentient channel add whatsapp \
  --phone-id "123456789" \
  --token "EAAB..."

# Send message
sentient channel send whatsapp +1234567890 "Hello!"
```

---

## Voice Commands

### Enable Voice Mode

```bash
sentient voice enable
sentient voice start
```

### Wake Word

Say "Hey SENTIENT" to activate voice interaction.

### Voice Chat

```bash
# Start voice session
sentient voice chat

# Transcribe file
sentient voice transcribe recording.mp3

# Synthesize speech
sentient voice speak "Hello world!" --output hello.mp3
```

---

## Skills

### Search Skills

```bash
sentient skill search translator
sentient skill search --category productivity
```

### Install Skills

```bash
sentient skill install translator-pro
sentient skill install code-reviewer
```

### List Installed

```bash
sentient skill list
```

### Create Custom Skill

```bash
sentient skill create my-skill
cd my-skill
# Edit skill.yaml and index.js
sentient skill publish
```

---

## API Server

### Start Gateway

```bash
sentient gateway --port 8080
```

### HTTP Requests

```bash
# Chat
curl http://localhost:8080/v1/chat \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Hello!"}]}'

# Stream
curl http://localhost:8080/v1/chat/stream \
  -H "Content-Type: application/json" \
  -d '{"messages": [{"role": "user", "content": "Tell me a story"}], "stream": true}'
```

---

## Configuration

### Config File Location

- Linux/macOS: `~/.config/sentient/config.toml`
- Windows: `%APPDATA%\sentient\config.toml`

### Example Config

```toml
[llm]
default_provider = "openai"
default_model = "gpt-4o"
api_key = "sk-..."

[llm.providers.openai]
api_key = "sk-..."

[llm.providers.anthropic]
api_key = "sk-ant-..."

[channels.telegram]
enabled = true
token = "123456:ABC..."

[voice]
enabled = true
wake_word = "hey sentient"
stt_provider = "whisper"
tts_provider = "openai"

[agent]
max_steps = 50
timeout = 300
```

### Environment Variables

```bash
export SENTIENT_OPENAI_KEY="sk-..."
export SENTIENT_ANTHROPIC_KEY="sk-ant-..."
export SENTIENT_DEFAULT_MODEL="anthropic/claude-3-5-sonnet"
```

---

## Tips & Tricks

### Use Local Models

```bash
# Install Ollama first
ollama pull llama3.3

# Use with SENTIENT
sentient repl --model ollama/llama3.3
```

### Custom System Prompt

```bash
sentient repl --system "You are a Rust expert. Be concise."
```

### Output Formats

```bash
# JSON output
sentient ask "List planets" --format json

# Markdown output
sentient ask "Explain async/await" --format markdown

# Code output
sentient code "Sort array" --language python
```

### Debug Mode

```bash
SENTIENT_DEBUG=1 sentient repl
```

---

## Next Steps

- [API Reference](API.md)
- [Channels Guide](CHANNELS.md)
- [Voice Guide](VOICE.md)
- [Skills Development](SKILLS.md)
- [Kubernetes Deployment](KUBERNETES.md)

---

## Getting Help

- **Documentation**: https://docs.sentient.ai
- **GitHub Issues**: https://github.com/nexsusagent-coder/SENTIENT_CORE/issues
- **Discord**: https://discord.gg/sentient

---

**Happy coding with SENTIENT! 🚀**
