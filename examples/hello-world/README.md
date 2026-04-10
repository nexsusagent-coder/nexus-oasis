# Hello World Example

Minimal SENTIENT OS example showing basic usage.

## Prerequisites

1. **Ollama** installed and running:
   ```bash
   # Install Ollama
   curl -fsSL https://ollama.com/install.sh | sh
   
   # Start Ollama
   ollama serve
   
   # Pull model
   ollama pull llama3.2:3b
   ```

## Run

```bash
cd examples/hello-world
cargo run
```

## Expected Output

```
🧠 SENTIENT OS - Hello World
═════════════════════════════

📡 Connecting to Ollama (localhost:11434)...
🤖 Model: llama3.2:3b

👤 You: Hello, SENTIENT!

🤖 Agent: Hello! I'm SENTIENT, your AI assistant. How can I help you today?

═════════════════════════════
✅ Hello World complete!
```

## What This Shows

1. **LlmClient** - Connecting to an LLM provider
2. **Agent** - Creating a basic agent
3. **Chat** - Sending a message and getting a response

## Next Steps

- Try [Chatbot Example](../chatbot/) for a more complete example
- See [Multi-Agent Example](../multi-agent/) for agent orchestration
