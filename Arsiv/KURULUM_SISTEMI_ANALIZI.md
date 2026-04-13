# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS KURULUM SİSTEMİ - OPENCLAW STANDARDI
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-13
#  Konu: OpenClaw benzeri interaktif kurulum sistemi analizi
# ═══════════════════════════════════════════════════════════════════════════════

---

## 🎯 SİSTEM ANLAŞILDI MI?

**EVET!** SENTIENT OS, OpenClaw standardında bir kurulum sistemine sahip:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    SENTIENT OS Setup Wizard                         │
│                  OpenClaw Standard - v7.0.0                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  1. Security Warning (Personal vs Lock-down mode)                  │
│  2. Setup Mode Selection (QuickStart vs Manual)                    │
│  3. LLM Provider Selection (100+ models, fuzzy search)             │
│  4. API Key Input (hidden, secure)                                 │
│  5. Communication Channels (20+ platforms, multi-select)           │
│  6. Tools (Web Search, SearXNG, DuckDuckGo)                        │
│  7. Permissions (Agent-S3 hardware access)                         │
│  8. Save & Start                                                   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 📋 KURULUM ADIMLARI (8 ADIM)

### ADIM 0: Security Warning ⚠️

```
╔════════════════════════════════════════════════════════════════════════════════╗
║   SECURITY WARNING                                                            ║
╚════════════════════════════════════════════════════════════════════════════════╝

This system is PERSONAL by default.
Multi-user access requires LOCK-DOWN mode.

   - Personal mode: Full access to all data
   - Lock-down mode: Restricted access, audit log enabled

? Do you want to continue? (Y/n)
```

**Kullanıcıdan İstenen:**
- Security warning'ı onaylaması

---

### ADIM 1: Setup Mode Selection

```
Select setup mode:

   QuickStart    - Fast setup
                   Port: 18789, Loopback, Token Auth
                   Recommended for first-time setup

   Manual        - Full control
                   Customize all settings
                   Recommended for experienced users

? Your selection: QuickStart
```

**Kullanıcıdan İstenen:**
- QuickStart veya Manual seçimi
- QuickStart = 4 adım, varsayılan ayarlar
- Manual = 6 adım, tam kontrol

---

### ADIM 2: LLM Provider Selection (100+ Modeller)

```
LLM Model Selection - OpenClaw Standard Format
   Format: provider/model_id | Fuzzy search enabled

? Select provider (type to search):
```

**Model Kategorileri:**

| Kategori | Provider Sayısı | Model Sayısı |
|----------|-----------------|--------------|
| **Premium** | 10 | 50+ |
| **Fast Inference** | 3 | 15+ |
| **Local (Ücretsiz)** | 1 | 54 |
| **Aggregator** | 2 | 100+ |
| **Chinese** | 5 | 20+ |
| **Enterprise** | 4 | 15+ |

**Premium Cloud Providers:**
- `anthropic/claude-3-5-sonnet-20241022`
- `anthropic/claude-3-opus-20240229`
- `openai/gpt-4o`
- `openai/gpt-4o-mini`
- `openai/o1`
- `openai/o1-preview`
- `google/gemini-2.0-flash`
- `google/gemini-1.5-pro`
- `mistral/mistral-large-latest`
- `mistral/codestral-latest`
- `xai/grok-beta`
- `xai/grok-2-1212`

**Local (API Key Gerektirmez):**
- `ollama/llama3.3:70b`
- `ollama/qwen2.5:72b`
- `ollama/deepseek-r1:70b`
- `ollama/gemma4:31b` ← **SENTIENT KERNEL DEFAULT**
- `ollama/mistral:7b`

**Aggregator (100+ Model):**
- `openrouter/auto` (otomatik seçim)
- `openrouter/anthropic/claude-3.5-sonnet`
- `openrouter/openai/gpt-4o`
- `openrouter/meta-llama/llama-3.3-70b-instruct`

**Chinese Providers:**
- `zhipu/glm-4`
- `alibaba/qwen-max`
- `baidu/ernie-4.0-8k`
- `moonshot/moonshot-v1-128k`
- `siliconflow/Qwen2.5-72B`

**Enterprise:**
- `azure/gpt-4o`
- `bedrock/anthropic.claude-3-5-sonnet`
- `vertex/gemini-2.0-flash`
- `watsonx/ibm/granite-13b-chat-v2`

**Kullanıcıdan İstenen:**
- Model seçimi (fuzzy search ile)
- Seçilen model `provider/model_id` formatında

---

### ADIM 3: API Key Input (Hidden)

```
API Key Input (hidden)

? OpenAI API Key (sk-...): ********
[OK] API Key saved (hidden)
```

**API Key Prompt'ları:**

| Provider | Prompt Format | Örnek |
|----------|---------------|-------|
| Anthropic | `sk-ant-...` | `sk-ant-api03-...` |
| OpenAI | `sk-...` | `sk-proj-...` |
| Google | `AIza...` | `AIzaSy...` |
| OpenRouter | `sk-or-...` | `sk-or-v1-...` |
| Groq | `gsk_...` | `gsk_abc...` |
| Mistral | Mistral API Key | `...` |
| DeepSeek | DeepSeek API Key | `...` |
| Together | Together AI API Key | `...` |
| Perplexity | `pplx-...` | `pplx-...` |
| HuggingFace | `hf_...` | `hf_abc...` |
| NVIDIA | `nvapi-...` | `nvapi-...` |

**ÖNEMLI: API Key Gerektirmeyen Provider'lar:**
- `ollama` - Yerel, tamamen ücretsiz
- `g4f` - GPT4Free, ücretsiz

**Ollama için farklı prompt:**
```
Note: Ollama installation required:
   curl -fsSL https://ollama.com/install.sh | sh
   ollama pull llama3.3:70b
```

**Kullanıcıdan İstenen:**
- API key (password hidden input)
- Boş bırakılabilir → sonra eklenebilir

---

### ADIM 4: Communication Channels (20+ Platform)

```
Communication Channels - 20+ Platforms
   Space: Select/Remove    Enter: Confirm

? Select channels (Space to select, Enter to confirm):
```

**Kanal Kategorileri:**

**Mobile Messengers:**
- Telegram Bot
- WhatsApp Business
- Signal
- iMessage (macOS)
- WeChat
- LINE
- Viber
- KakaoTalk

**Enterprise Platforms:**
- Discord
- Slack
- Microsoft Teams
- Google Chat
- Webex
- Zoom Chat
- Mattermost
- Rocket.Chat

**Decentralized / Privacy:**
- Matrix/Element
- XMPP/Jabber
- Session
- Wire
- Threema
- Nostr

**Social Platforms:**
- Twitter/X DM
- Instagram DM
- Facebook Messenger
- LinkedIn
- Reddit

**Email & SMS:**
- Email (SMTP/IMAP)
- SMS (Twilio)
- RCS

**Web/API:**
- Web Dashboard
- REST API

**Kullanıcıdan İstenen:**
- Multi-select (Space ile seçim)
- Her seçilen kanal için token/key

**Telegram Setup Örneği:**
```
  Telegram Bot Setup
     1. Find @BotFather on Telegram
     2. Send /newbot and follow instructions
     3. Copy the token

  ? Bot Token (hidden): ********

  Security Policy:
     - Allow all users
     - Allow specific users only
     - Private (password protected)
  
  ? Select policy: Allow all users
```

---

### ADIM 5: Tools (Web Search)

```
Tools - Web Search & More

? Select tools:
```

**Mevcut Tools:**
- SearXNG (self-hosted search)
- DuckDuckGo (API key'siz)
- Tavily (API key gerekli)
- Brave Search (API key gerekli)
- Ollama Embeddings

**Kullanıcıdan İstenen:**
- Tool seçimi
- Gerekli API key'ler

---

### ADIM 6: Permissions (Agent-S3)

```
Permissions - Hardware Access

? Allow agent to control:
   [ ] Mouse
   [ ] Keyboard
   [ ] Screen capture
   [ ] File system
   [ ] Network
```

**Kullanıcıdan İstenen:**
- Hardware permission'ları onaylaması

---

### ADIM 7: Save & Show Success

```
Saving configuration...

[OK] Configuration saved to ~/.sentient/config.json
[OK] API keys saved to ~/.sentient/.env

───────────────────────────────────────────────────────────────────────
Setup Complete!
───────────────────────────────────────────────────────────────────────

Start SENTIENT:
   sentient start

Open dashboard:
   http://127.0.0.1:18789

View logs:
   sentient logs

Stop:
   sentient stop
```

---

## 🔄 QUICKSTART vs MANUAL

### QuickStart (4 Adım)
```
1. LLM Provider Selection
2. Communication Channels
3. Tools
4. Save
```

**Varsayılanlar:**
- Port: 18789
- Host: 127.0.0.1 (loopback)
- Auth: Token
- Language: English

### Manual (6 Adım)
```
1. Language Selection
2. LLM Provider Selection
3. Communication Channels
4. Tools
5. Permissions
6. Save
```

---

## 🔐 API KEY AKIŞI

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Setup     │     │   Config    │     │   Runtime   │
│   Wizard    │────▶│   Files     │────▶│   Usage     │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    │             │
              ~/.sentient/    ~/.sentient/
              config.json      .env
                    │             │
              Model/Provider   API Keys
              Settings         (hidden)
```

**config.json Örneği:**
```json
{
  "llm": {
    "provider": "openai",
    "model": "gpt-4o"
  },
  "dashboard": {
    "port": 18789,
    "host": "127.0.0.1"
  },
  "integrations": {
    "telegram": { "enabled": true },
    "discord": { "enabled": false }
  }
}
```

**.env Örneği:**
```bash
OPENAI_API_KEY=sk-proj-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=AIza...
TELEGRAM_BOT_TOKEN=...
```

---

## 🆚 OPENCLAW vs SENTIENT KARŞILAŞTIRMA

| Özellik | OpenClaw | SENTIENT OS |
|---------|----------|-------------|
| **Setup Wizard** | ✅ Interaktif | ✅ Interaktif (aynı) |
| **Security Warning** | ✅ Personal/Lock-down | ✅ Personal/Lock-down (aynı) |
| **QuickStart Mode** | ✅ | ✅ (aynı) |
| **Manual Mode** | ✅ | ✅ (aynı) |
| **LLM Models** | ~50 | **100+** (daha fazla) |
| **Format** | provider/model_id | provider/model_id (aynı) |
| **Fuzzy Search** | ✅ | ✅ (aynı) |
| **API Key Hidden** | ✅ | ✅ (aynı) |
| **Channels** | ~10 | **20+** (daha fazla) |
| **Tools** | ~5 | ~10 (daha fazla) |
| **Permissions** | ✅ | ✅ Agent-S3 (aynı) |
| **Local Models** | Ollama | Ollama + **Gemma 4 KERNEL** |
| **Desktop App** | ❌ | ✅ **Tauri** |
| **Dashboard** | ✅ Web | ✅ Enterprise War Room |
| **Voice** | ❌ | ✅ Whisper STT + TTS |
| **Video Gen** | ❌ | ✅ Runway, Pika, Luma... |

---

## ✅ SONUÇ

**SENTIENT OS'in kurulum sistemi OpenClaw standardında:**

1. ✅ Interaktif TUI setup wizard
2. ✅ Security warning (Personal vs Lock-down)
3. ✅ QuickStart vs Manual mode
4. ✅ 100+ LLM model desteği (provider/model_id format)
5. ✅ Fuzzy search ile model seçimi
6. ✅ Hidden API key input (password)
7. ✅ 20+ communication channel
8. ✅ Tools configuration
9. ✅ Permissions (Agent-S3)
10. ✅ Config save to ~/.sentient/

**Ek Özellikler (OpenClaw'da yok):**
- Gemma 4 KERNEL (varsayılan yerel model)
- Desktop App (Tauri)
- Enterprise Dashboard
- Voice (Whisper STT + TTS)
- Video Generation
- 42 LLM Provider (OpenClaw'dan daha fazla)

---

## 🚀 KURULUM KOMUTLARI

```bash
# Kurulumu başlat
sentient setup

# Veya direkt başlat
sentient start

# Dashboard
sentient dashboard

# Durum kontrol
sentient status

# Log'lar
sentient logs

# Durdur
sentient stop
```

---

*Rapor Tarihi: 2026-04-13*
*Sistem: OpenClaw Standardında Kurulum Wizard*
