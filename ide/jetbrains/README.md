# SENTIENT OS JetBrains Plugin

AI-powered coding assistant for JetBrains IDEs (IntelliJ IDEA, PyCharm, WebStorm, etc.)

## Features

- 🤖 **AI Chat** - Interactive conversation with AI
- 📝 **Explain Code** - Understand any code instantly
- 🔧 **Refactor** - Improve code quality
- 🐛 **Fix Bugs** - Find and fix issues
- 🧪 **Generate Tests** - Create unit tests
- 📚 **Generate Docs** - Auto-generate documentation
- 🌐 **Translate** - Convert between languages
- ⚡ **Optimize** - Improve performance
- 👀 **Code Review** - Get detailed feedback
- 💬 **Commit Messages** - Generate conventional commits

## Supported IDEs

- IntelliJ IDEA
- PyCharm
- WebStorm
- PhpStorm
- RubyMine
- CLion
- GoLand
- RustRover
- Android Studio
- DataGrip

## Installation

### From JetBrains Marketplace
1. Open Settings/Preferences → Plugins
2. Search for "SENTIENT OS"
3. Click Install
4. Restart IDE

### From Source
```bash
cd ide/jetbrains
./gradlew buildPlugin
```

Then install the plugin from `build/distributions/`

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

Select any code and right-click to see SENTIENT options.

### Intentions

Press `Alt+Enter` on selected code to see quick actions.

## Configuration

Open Settings → Tools → SENTIENT OS:

| Setting | Description | Default |
|---------|-------------|---------|
| API URL | API endpoint | `http://localhost:8080` |
| Model | LLM model | `gpt-4-turbo` |
| Max Tokens | Max response | `4096` |
| Temperature | Creativity | `0.7` |
| Streaming | Enable streaming | `true` |

## Supported Models

- OpenAI: GPT-4, GPT-4o, GPT-3.5, O1
- Anthropic: Claude 3 Opus/Sonnet/Haiku
- Google: Gemini 1.5 Pro/Flash
- Meta: Llama 3.1
- Mistral: Mixtral
- And 600+ more!

## License

MIT License
