# SENTIENT AI - Getting Started Guide

> **5 dakikada çalışan AI asistan — tüm platformlar**

---

## Hızlı Başlangıç

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.ps1 | iex
```

### Build from Source

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE
cd SENTIENT_CORE
cargo build --release
./target/release/sentient setup
```

---

## LLM Yapılandırması

### Seçenek A: Lokal (Ücretsiz, API Key Gerekmez)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Model indir
ollama pull gemma3:27b       # 16GB VRAM (önerilen)
ollama pull qwen3:30b-a3b    # 4GB VRAM (hafif)
ollama pull deepseek-r1:7b   # 8GB VRAM (reasoning)

# .env
LLM_PROVIDER=ollama
OLLAMA_MODEL=gemma3:27b

# Başlat
sentient chat
```

### Seçenek B: API Key ile

```bash
# OpenRouter (önerilen — 200+ model, $5 ücretsiz kredi)
# https://openrouter.ai/keys
export OPENROUTER_API_KEY=sk-or-v1-xxxxx

# Diğer provider'lar
export OPENAI_API_KEY=sk-xxx
export ANTHROPIC_API_KEY=sk-ant-xxx
export DEEPSEEK_API_KEY=xxx
export GROQ_API_KEY=gsk_xxx

# Başlat
sentient chat
```

---

## Temel Komutlar

```bash
sentient chat "Merhaba"                        # Sohbet
sentient ask "Rust'ta ownership nedir?"         # Tek soru
sentient code "Python REST API yaz"            # Kod üret
sentient voice                                 # Sesli asistan (JARVIS)
sentient desktop --goal "YouTube'da müzik aç"  # Otonom agent
sentient daemon start                          # 7/24 arka plan asistan
sentient channel start telegram                # Telegram bot
sentient gateway                               # REST API server (port 8080)
sentient doctor                                # Sistem kontrolü
```

---

## JARVIS Modu

```bash
# Lokal (ücretsiz)
sentient voice --wake-word "hey sentient" --language tr

# Kullanım:
# "Hey Sentient, rahatlatıcı müzik aç"  → YouTube'da arar
# "Salon ışığını kapat"                 → Home Assistant'a komut
# "Google'da rust ara"                  → Web arama
# "Saat kaç"                            → Saati söyler
```

Desteklenen 17 sesli komut intent'i: PlayMusic, PlayVideo, WebSearch, Pause, Resume, Close, WhatTime, Weather, ControlHome, SetReminder, GitHubTrending, ProjectAssign...

---

## Multi-Agent Orkestrasyonu

```bash
# CrewAI: Araştırmacı + Yazar + Editör
sentient crew create report-team \
  --agents "researcher:deepseek-r1,writer:gpt-4o,editor:claude-4-sonnet"
sentient crew run report-team --goal "AI pazar analizi raporu yaz"

# MetaGPT: Şirket modeli (PM + Architect + Engineer + QA)
sentient crew run software-team --framework metagpt \
  --goal "Sosyal medya uygulaması geliştir"
```

6 framework desteklenir: CrewAI, AutoGen, Swarm, MetaGPT, Agent-S, SENTIENT Native

---

## Kanal Entegrasyonları

```bash
# Telegram bot
sentient channel add telegram --token "123456:ABC-..."
sentient channel start telegram

# Discord bot
sentient channel add discord --token "Bot YOUR_TOKEN"
sentient channel start discord

# Çoklu kanal
sentient daemon start --channels telegram,discord,slack
```

20+ platform: Telegram, Discord, WhatsApp, Slack, Email, Teams, Signal, Matrix, iMessage, Instagram, LinkedIn, Twitter/X, Line, WeChat, Messenger, Mattermost, Google Chat, Webex, Zoom, Chime

---

## 57+ LLM Provider

| Tür | Provider'lar |
|-----|-------------|
| **Direct** | OpenAI, Anthropic, Google, Mistral, DeepSeek, xAI, Cohere, Groq, Perplexity, AI21, Reka, Cerebras, Fireworks, Replicate, StepFun, Aleph Alpha, Sarvam, Voyage, Upstage, GigaChat |
| **Aggregator** | OpenRouter (200+), Together, HuggingFace (200K+), DeepInfra, GLHF, Novita, Hyperbolic, SiliconFlow, Lepton, Chutes |
| **Enterprise** | Azure OpenAI, AWS Bedrock, Vertex AI, NVIDIA NIM, SambaNova, IBM WatsonX |
| **Chinese** | Zhipu AI, Moonshot, Yi, Baidu ERNIE, MiniMax, Qwen Direct, Mod |
| **Gateway** | Unify, Portkey, Helicone, NotDiamond, AI/ML API, Glama, Requesty, LiteLLM, Cloudflare |
| **Local** | Ollama, vLLM, LM Studio, Llamafile |

Detaylı liste: [MODEL_PROVIDERS.md](../MODEL_PROVIDERS.md)

---

## Akıllı Ev (Home Assistant)

```bash
# .env
HOME_ASSISTANT_URL=http://homeassistant.local:8123
HOME_ASSISTANT_TOKEN=eyJ0eXAi...

# Sesli komutlar
# "Salon ışığını kapat" → light.living_room turn_off
# "Film modu"           → movie scene activate
# "Klimayı 22 yap"      → climate set_temperature
```

---

## Güvenlik

```bash
# V-GATE: API key'ler sunucuda, istemcide YOK
sentient vgate start

# Guardrails: Prompt injection engelleme
sentient guardrails test "Ignore all previous instructions"
# → ❌ BLOCKED

# Vault: Şifreli secret yönetimi
sentient vault set OPENAI_API_KEY "sk-xxx"
```

6 güvenlik crate'i: guardrails, vault, tee, zk_mcp, compliance, anomaly

---

## Sistem Gereksinimleri

| Mod | RAM | VRAM | Disk |
|-----|-----|------|------|
| API-only | 8 GB | - | 20 GB |
| Lokal küçük | 16 GB | 8 GB | 50 GB |
| Lokal büyük | 32 GB | 24 GB | 100 GB |

---

## Detaylı Dokümantasyon

| Dosya | Açıklama |
|-------|----------|
| [QUICKSTART.md](../QUICKSTART.md) | 5 dakika başlangıç |
| [INSTALL.md](../INSTALL.md) | Kapsamlı kurulum (20 bölüm) |
| [INSTALL_GUIDE.md](../INSTALL_GUIDE.md) | Universal kurulum (tüm platformlar) |
| [USAGE_GUIDE.md](../USAGE_GUIDE.md) | Kapsamlı kullanım kılavuzu (20 bölüm) |
| [USAGE_SCENARIOS.md](USAGE_SCENARIOS.md) | 15 gerçek dünya senaryosu |
| [MODEL_PROVIDERS.md](../MODEL_PROVIDERS.md) | 57+ provider detayları |
| [ARCHITECTURE.md](../ARCHITECTURE.md) | Sistem mimarisi (A1-A12) |
| [SECURITY.md](../SECURITY.md) | Güvenlik politikası |
| [API.md](API.md) | REST API referansı |
| [CHANNELS.md](CHANNELS.md) | Kanal entegrasyonları |
| [VOICE.md](VOICE.md) | Ses sistemi rehberi |

---

## Destek

- **GitHub Issues**: https://github.com/nexsusagent-coder/SENTIENT_CORE/issues
- **Email**: sentient@sentient-os.ai

---

**Happy coding with SENTIENT! 🚀**
