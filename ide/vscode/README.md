# SENTIENT OS VS Code Extension

AI-powered coding assistant with SENTIENT OS integration.

## Features

- 🤖 **Chat Interface** - Interactive chat with AI
- 📝 **Code Explanation** - Understand any code instantly
- 🔧 **Code Refactoring** - Improve code quality
- 🐛 **Bug Fixing** - Find and fix issues
- 🧪 **Test Generation** - Generate unit tests
- 📚 **Documentation** - Generate docs automatically
- 🌐 **Code Translation** - Convert between languages
- ⚡ **Optimization** - Improve performance
- 👀 **Code Review** - Get detailed feedback
- 💬 **Commit Messages** - Generate conventional commits

## Installation

### From VS Code Marketplace
1. Open VS Code
2. Press `Ctrl+Shift+X` (or `Cmd+Shift+X` on Mac)
3. Search for "SENTIENT OS"
4. Click Install

### From Source
```bash
cd ide/vscode
npm install
npm run compile
```

## Usage

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+S` | Open Chat |
| `Ctrl+Shift+E` | Explain Code |
| `Ctrl+Shift+R` | Refactor Code |
| `Ctrl+Shift+F` | Fix Code |
| `Ctrl+Shift+T` | Generate Tests |

### Context Menu

Select any code and right-click to see SENTIENT options:
- Explain Code
- Refactor Code
- Fix Code
- Generate Tests
- Generate Documentation
- Optimize Code
- Translate Code

## Configuration

Open Settings (`Ctrl+,`) and search for "SENTIENT":

| Setting | Description | Default |
|---------|-------------|---------|
| `sentient.apiUrl` | API endpoint URL | `http://localhost:8080` |
| `sentient.model` | LLM model to use | `gpt-4-turbo` |
| `sentient.maxTokens` | Max response tokens | `4096` |
| `sentient.temperature` | Response creativity | `0.7` |
| `sentient.streaming` | Enable streaming | `true` |
| `sentient.language` | Response language | `en` |

## Supported Models

- OpenAI: GPT-4 Turbo, GPT-4o, GPT-3.5 Turbo, O1-preview, O1-mini
- Anthropic: Claude 3 Opus, Claude 3 Sonnet, Claude 3 Haiku
- Google: Gemini 1.5 Pro, Gemini 1.5 Flash
- Meta: Llama 3.1 70B, Llama 3.1 405B
- Mistral: Mixtral 8x7B
- Alibaba: Qwen 2.5 72B
- Google: Gemma 4 27B
- And 600+ more models!

## License

MIT License
