# @sentient/ai

**SENTIENT** - AI Operating System with Autonomous Agents

## 🚀 Quick Start

```bash
# Install globally
npm install -g @sentient/ai

# Run setup wizard
sentient setup

# Start interactive REPL
sentient repl

# Run autonomous agent
sentient agent --goal "Build a REST API"

# Query LLM
sentient llm chat
```

## 📦 Installation

```bash
# npm
npm install -g @sentient/ai

# yarn
yarn global add @sentient/ai

# pnpm
pnpm add -g @sentient/ai
```

## 🖥️ Platform Support

| Platform | Architecture | Status |
|----------|-------------|--------|
| Linux | x86_64 | ✅ |
| Linux | ARM64 | ✅ |
| macOS | Intel | ✅ |
| macOS | Apple Silicon | ✅ |
| Windows | x86_64 | ✅ |

## 📚 API Usage

```javascript
const sentient = require('@sentient/ai');

// Check version
console.log(sentient.version);
console.log(sentient.getVersion());

// Check if installed
if (!sentient.isInstalled()) {
  console.log('Binary not found');
}

// Execute command
const output = await sentient.exec(['--help']);

// Run agent
const result = await sentient.agent('Build a REST API', {
  model: 'qwen/qwen3-1.7b:free',
  maxIterations: 50
});

// Query LLM
const response = await sentient.query('What is AI?');

// Run REPL
sentient.repl({ debug: true });
```

## 🔧 CLI Commands

```bash
sentient [command] [options]

Commands:
  repl          Start interactive REPL
  setup         Run setup wizard
  agent         Run autonomous agent
  status        Show system status
  llm           LLM operations
  gateway       Start API gateway
  serve         Start 24/7 service
  memory        Memory operations
  guardrails    Security policies
  sandbox       Sandbox operations
  swarm         Swarm operations

Options:
  --version     Show version
  --help        Show help
  --debug       Debug mode
  --quiet       Quiet mode
```

## 🌐 Features

- **Autonomous Agents** - Self-executing AI agents that complete tasks
- **Multi-LLM Support** - OpenRouter, OpenAI, Anthropic, local models
- **Safety Guardrails** - Built-in security and content filtering
- **Memory System** - Persistent storage and retrieval
- **Sandbox Execution** - Safe code execution environment
- **API Gateway** - REST API for integrations
- **Voice Support** - Speech-to-text and text-to-speech
- **Skills Marketplace** - Installable AI skills

## 📖 Documentation

- [Getting Started](https://github.com/nexsusagent-coder/SENTIENT_CORE#readme)
- [API Reference](https://docs.sentient.ai)
- [Examples](https://github.com/nexsusagent-coder/SENTIENT_CORE/tree/main/examples)

## 🤝 Contributing

Contributions welcome! See [Contributing Guide](CONTRIBUTING.md).

## 📄 License

MIT License - see [LICENSE](LICENSE) file.

---

Built with ❤️ by **NEXUS OASIS**
