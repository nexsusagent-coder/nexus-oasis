# 🤖 MODEL & PROVIDER REHBERİ

SENTIENT OS, **10+ provider** ve **100+ model** desteği sunar.

---

## 📊 Hızlı Başlangıç

### Ücretsiz Seçenekler

| Seçenek | Nasıl | Maliyet |
|---------|-------|---------|
| **Yerel Model** | Ollama kur + model indir | $0 |
| **OpenRouter Free** | OPENROUTER_API_KEY | $0 (limitli) |
| **Gemma 4 Free** | OpenRouter free tier | $0 |

### API Key Gerektirenler

| Provider | Key Env | En İyi Model |
|----------|---------|--------------|
| OpenAI | `OPENAI_API_KEY` | GPT-4o |
| Anthropic | `ANTHROPIC_API_KEY` | Claude 3.7 |
| Google | `GOOGLE_API_KEY` | Gemini 2.0 |
| Groq | `GROQ_API_KEY` | Llama 3.3 |

---

## 🏠 YEREL MODELLER (Key Gerektirmez)

### Ollama Kurulumu

```bash
# Linux/macOS
curl -fsSL https://ollama.ai/install.sh | sh

# Model indir
ollama pull gemma4:31b
ollama pull llama3.3:70b
ollama pull qwen2.5:72b

# Çalıştır
ollama run gemma4:31b
```

### Önerilen Yerel Modeller

| Model | Parametre | Context | VRAM | İndirme |
|-------|-----------|---------|------|---------|
| **Gemma 4 31B** | 31B | 256K | 24GB | `ollama pull gemma4:31b` |
| **Llama 3.3 70B** | 70B | 128K | 48GB | `ollama pull llama3.3:70b` |
| **Qwen 2.5 72B** | 72B | 128K | 48GB | `ollama pull qwen2.5:72b` |
| **DeepSeek R1 67B** | 67B | 128K | 40GB | `ollama pull deepseek-r1:67b` |
| **Mistral 24B** | 24B | 128K | 16GB | `ollama pull mistral:24b` |

### Donanım Gereksinimleri

| VRAM | Önerilen Model |
|------|----------------|
| 8GB | Gemma 4 E2B, Phi-4 |
| 16GB | Mistral 24B, Gemma 4 E4B |
| 24GB | Gemma 4 31B |
| 40GB+ | DeepSeek R1 67B |
| 48GB+ | Llama 3.3 70B, Qwen 2.5 72B |

---

## 🔑 API PROVIDER'LAR

### OpenRouter (Önerilen)

**Tek API key ile 100+ model erişimi.**

```bash
# .env
OPENROUTER_API_KEY=sk-or-...
```

| Model | Context | Ücretsiz? | Özellik |
|-------|---------|-----------|---------|
| `google/gemma-4-31b-it:free` | 256K | ✅ | Varsayılan |
| `google/gemma-4-26b-a4b-it:free` | 256K | ✅ | Hızlı |
| `meta-llama/llama-3.3-70b-instruct:free` | 128K | ✅ | Genel |
| `qwen/qwen3-235b-a22b-instruct:free` | 128K | ✅ | Advanced |
| `deepseek/deepseek-r1:free` | 164K | ✅ | Reasoning |
| `openai/gpt-4o` | 128K | ❌ | Multimodal |
| `anthropic/claude-3.7-sonnet` | 200K | ❌ | Coding |

---

### OpenAI

```bash
# .env
OPENAI_API_KEY=sk-...
```

| Model | Context | Fiyat (1K tokens) | Özellik |
|-------|---------|-------------------|---------|
| `gpt-4o` | 128K | $0.03-0.06 | Multimodal |
| `gpt-4o-mini` | 128K | $0.00015-0.0006 | Hızlı |
| `o3-mini` | 200K | Değişken | Reasoning |

---

### Anthropic (Claude)

```bash
# .env
ANTHROPIC_API_KEY=sk-ant-...
```

| Model | Context | Fiyat (1K tokens) | Özellik |
|-------|---------|-------------------|---------|
| `claude-3.7-sonnet` | 200K | $0.003-0.015 | Coding, Reasoning |
| `claude-3.5-sonnet` | 200K | $0.003-0.015 | Genel |
| `claude-3.5-haiku` | 200K | $0.00025-0.00125 | Hızlı |

---

### Google (Gemini)

```bash
# .env
GOOGLE_API_KEY=...
```

| Model | Context | Özellik |
|-------|---------|---------|
| `gemini-2.0-flash-exp` | **1M** | Çok uzun context |
| `gemini-exp-1206` | **2M** | Extreme context |

---

### Groq (Hızlı Inference)

```bash
# .env
GROQ_API_KEY=...
```

| Model | Context | Özellik |
|-------|---------|---------|
| `llama-3.3-70b-versatile` | 128K | En hızlı |
| `mixtral-8x7b-32768` | 32K | Çok hızlı |

---

### DeepSeek (Ucuz)

```bash
# .env
DEEPSEEK_API_KEY=...
```

| Model | Context | Fiyat | Özellik |
|-------|---------|-------|---------|
| `deepseek-r1` | 164K | ~$0.001/1K | Reasoning |
| `deepseek-chat` | 64K | ~$0.0005/1K | Genel |

---

### Mistral

```bash
# .env
MISTRAL_API_KEY=...
```

| Model | Context | Özellik |
|-------|---------|---------|
| `mistral-large-2412` | 128K | En güçlü |
| `mixtral-8x22b-instruct` | 64K | MoE |
| `pixtral-large-2411` | 128K | Vision |

---

### Cohere

```bash
# .env
COHERE_API_KEY=...
```

| Model | Context | Özellik |
|-------|---------|---------|
| `command-r-plus` | 128K | RAG optimizasyonu |
| `command-r` | 128K | RAG |

---

### Together AI

```bash
# .env
TOGETHER_API_KEY=...
```

Açık modeller için optimize edilmiş platform.

---

### Replicate

```bash
# .env
REPLICATE_API_KEY=...
```

Her modeli API ile çalıştırma imkanı.

---

## 🆓 ÜCRETSİZ MODELLER (OpenRouter)

Bu modeller OpenRouter free tier ile **$0** maliyetle kullanılabilir:

| Model | Context | Thinking | Vision |
|-------|---------|----------|--------|
| Gemma 4 31B | 256K | ✅ | ✅ |
| Gemma 4 26B MoE | 256K | ✅ | ✅ |
| Llama 3.3 70B | 128K | ❌ | ❌ |
| Qwen3 235B MoE | 128K | ✅ | ❌ |
| DeepSeek R1 | 164K | ✅ | ❌ |
| Mistral Small 24B | 128K | ❌ | ❌ |

---

## 📊 MODEL KARŞILAŞTIRMA

### En İyi Coding

| Model | Provider | Ücret |
|-------|----------|-------|
| Claude 3.7 Sonnet | Anthropic | $ |
| GPT-4o | OpenAI | $ |
| Qwen 2.5 72B | Local | Free |
| Gemma 4 31B | Local | Free |

### En İyi Reasoning

| Model | Provider | Ücret |
|-------|----------|-------|
| DeepSeek R1 | DeepSeek/Local | $/Free |
| o3-mini | OpenAI | $ |
| Claude 3.7 Sonnet | Anthropic | $ |
| Gemma 4 31B | Local | Free |

### En İyi Multimodal (Vision)

| Model | Provider | Ücret |
|-------|----------|-------|
| GPT-4o | OpenAI | $ |
| Claude 3.7 Sonnet | Anthropic | $ |
| Gemini 2.0 Flash | Google | $ |
| Gemma 4 31B | Local | Free |

### En Uzun Context

| Model | Context | Provider |
|-------|---------|----------|
| Gemini Exp | **2M** | Google |
| Gemini 2.0 Flash | **1M** | Google |
| Gemma 4 31B | **256K** | Local/Free |
| Claude 3.7 | **200K** | Anthropic |

### En Hızlı

| Model | Provider | Hız |
|-------|----------|-----|
| Llama 3.3 70B | Groq | En hızlı |
| Mixtral 8x7B | Groq | Çok hızlı |
| GPT-4o-mini | OpenAI | Hızlı |
| Claude 3.5 Haiku | Anthropic | Hızlı |

---

## ⚙️ YAPILANDIRMA

### .env Dosyası

```bash
# Provider seçimi (öncelik sırası)
OPENROUTER_API_KEY=sk-or-...      # Önerilen
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=...
GROQ_API_KEY=...
COHERE_API_KEY=...
MISTRAL_API_KEY=...
DEEPSEEK_API_KEY=...
TOGETHER_API_KEY=...
REPLICATE_API_KEY=...

# Yerel model (Ollama)
OLLAMA_HOST=http://localhost:11434

# Varsayılan model
SENTIENT_MODEL=google/gemma-4-31b-it:free
```

### Config Dosyası (config.toml)

```toml
[llm]
provider = "openrouter"
model = "google/gemma-4-31b-it:free"
temperature = 0.7
max_tokens = 16384

[llm.fallback]
provider = "local"
model = "gemma4:31b"
```

---

## 🔄 MODEL DEĞİŞTİRME

### CLI ile

```bash
# Model listesi
sentient models list

# Model değiştir
sentient config set model openai/gpt-4o

# Provider değiştir
sentient config set provider openai
```

### API ile

```bash
curl -X POST http://localhost:8080/api/config \
  -H "Content-Type: application/json" \
  -d '{"model": "anthropic/claude-3.7-sonnet"}'
```

---

## 💡 İPUÇLARI

### 1. Ücretsiz Başlayın
```bash
# OpenRouter free tier ile
export OPENROUTER_API_KEY=sk-or-...
sentient --model google/gemma-4-31b-it:free
```

### 2. Yerel Model Kullanın
```bash
# Ollama kur
ollama pull gemma4:31b

# SENTIENT'i yerel modele ayarla
sentient --provider local --model gemma4:31b
```

### 3. Fallback Ayarlayın
```bash
# API limit'e takılırsa yerel modele geç
sentient config set fallback.provider local
sentient config set fallback.model gemma4:31b
```

### 4. Görev Bazlı Model Seçin
```bash
# Coding için Claude
sentient --task coding --model anthropic/claude-3.7-sonnet

# Reasoning için DeepSeek R1
sentient --task reasoning --model deepseek/deepseek-r1
```

---

## 📚 İLGİLİ DOKÜMANLAR

- [WHY_SENTIENT.md](./WHY_SENTIENT.md) - Neden SENTIENT?
- [INSTALL.md](./INSTALL.md) - Kurulum rehberi
- [USER_MANUAL.md](./USER_MANUAL.md) - Kullanım kılavuzu

---

*🧠 SENTIENT OS - The Operating System That Thinks*
