# Chatbot Example

Interactive chatbot with memory and conversation history.

## Features

- 💬 Interactive REPL chat
- 💾 Conversation memory (SQLite)
- 📜 History viewing
- 🔄 Multi-provider support (Ollama, OpenAI, Claude)
- 🧹 Clear/reset commands

## Prerequisites

### Option 1: Ollama (Free, Local)

```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Start Ollama
ollama serve

# Pull model
ollama pull llama3.2:3b
```

### Option 2: OpenAI (Cloud)

```bash
export OPENAI_API_KEY=your-api-key
export SENTIENT_PROVIDER=openai
export SENTIENT_MODEL=gpt-4o
```

### Option 3: Claude (Cloud)

```bash
export ANTHROPIC_API_KEY=your-api-key
export SENTIENT_PROVIDER=anthropic
export SENTIENT_MODEL=claude-3-5-sonnet-20241022
```

## Run

```bash
cd examples/chatbot
cargo run
```

## Usage

```
💬 Chat started! Type 'exit' to quit, 'clear' to clear history.

👤 You: Hello!
🤖 SENTIENT: Hello! I'm SENTIENT, your AI assistant. How can I help you today?

👤 You: What's the capital of Turkey?
🤖 SENTIENT: The capital of Turkey is Ankara.

👤 You: What did I just ask?
🤖 SENTIENT: You asked about the capital of Turkey, which is Ankara.

👤 You: history
📜 Conversation History:
  [user] Hello!
  [assistant] Hello! I'm SENTIENT...
  [user] What's the capital of Turkey?
  [assistant] The capital of Turkey is Ankara.
  [user] What did I just ask?
  [assistant] You asked about the capital of Turkey...

👤 You: exit
👋 Goodbye!
```

## Commands

| Command | Description |
|---------|-------------|
| `exit`, `quit`, `bye` | Exit the chat |
| `clear`, `reset` | Clear conversation history |
| `help` | Show available commands |
| `history` | Show conversation history |

## Key Concepts

1. **MemoryCube** - Persistent conversation storage
2. **Agent** - AI assistant with context awareness
3. **Provider Selection** - Switch between LLM providers

## Next Steps

- [Multi-Agent Example](../multi-agent/) - Multiple agents working together
