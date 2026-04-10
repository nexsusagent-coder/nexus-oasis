# 📚 SENTIENT OS Examples

Learn SENTIENT OS step by step with practical examples.

## Examples

| Example | Difficulty | Description |
|---------|------------|-------------|
| [Hello World](./hello-world/) | 🟢 Beginner | Minimal example, basic usage |
| [Chatbot](./chatbot/) | 🟡 Intermediate | Interactive chat with memory |
| [Multi-Agent](./multi-agent/) | 🟡 Intermediate | Multiple specialized agents |
| [Production](./production/) | 🔴 Advanced | Full production setup |

## Quick Start

### 1. Prerequisites

```bash
# Install Ollama (free, local LLM)
curl -fsSL https://ollama.com/install.sh | sh
ollama serve &

# Pull a model
ollama pull llama3.2:3b
```

### 2. Run Hello World

```bash
cd examples/hello-world
cargo run
```

### 3. Explore More

Each example has its own README with detailed explanations.

## Learning Path

```
hello-world ──▶ chatbot ──▶ multi-agent ──▶ production
    │              │              │              │
    │              │              │              │
    ▼              ▼              ▼              ▼
 Basic API    Memory &     Agent           Production
    usage      history     orchestration   features
```

## Environment Variables

All examples support these environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `SENTIENT_PROVIDER` | `ollama` | LLM provider |
| `SENTIENT_MODEL` | `llama3.2:3b` | Model to use |
| `OPENAI_API_KEY` | - | OpenAI API key |
| `ANTHROPIC_API_KEY` | - | Anthropic API key |
| `OLLAMA_URL` | `http://localhost:11434` | Ollama server URL |

## Provider Options

### Ollama (Free, Local)

```bash
ollama pull llama3.2:3b      # 3B params, fast
ollama pull llama3.2:1b      # 1B params, faster
ollama pull mistral:7b       # Mistral 7B
ollama pull codellama:7b     # Code-focused
```

### OpenAI (Cloud)

```bash
export SENTIENT_PROVIDER=openai
export SENTIENT_MODEL=gpt-4o
export OPENAI_API_KEY=sk-...
```

### Anthropic (Cloud)

```bash
export SENTIENT_PROVIDER=anthropic
export SENTIENT_MODEL=claude-3-5-sonnet-20241022
export ANTHROPIC_API_KEY=sk-ant-...
```

## Troubleshooting

### Ollama not responding

```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# Start Ollama
ollama serve
```

### Model not found

```bash
# List available models
ollama list

# Pull the model
ollama pull llama3.2:3b
```

### Build errors

```bash
# Update dependencies
cargo update

# Clean build
cargo clean && cargo build
```

## Contributing

Found an issue or have an example to share? Open a PR!

## License

Apache 2.0
