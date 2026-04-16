# 🤖 SENTIENT OS - Model & Provider Rehberi

> **57+ Provider, 245+ Native Model, 200K+ Aggregator Erişimi**

---

## 📑 İçindekiler

1. [Hızlı Başlangıç](#1-hızlı-başlangıç)
2. [Lokal Modeller (Ücretsiz)](#2-lokal-modeller-ücretsiz)
3. [Direct Provider'lar](#3-direct-providerlar)
4. [Aggregator Provider'lar](#4-aggregator-providerlar)
5. [Enterprise Cloud Provider'lar](#5-enterprise-cloud-providerlar)
6. [Çin AI Provider'ları](#6-çin-ai-providerları)
7. [AI Gateway/Router Provider'ları](#7-ai-gatewayrouter-providerları)
8. [Lokal Inference Provider'ları](#8-lokal-inference-providerları)
9. [Model Seçim Rehberi](#9-model-seçim-rehberi)
10. [Yapılandırma](#10-yapılandırma)

---

## 1. Hızlı Başlangıç

### Ücretsiz Seçenekler

| Seçenek | Nasıl | Maliyet |
|---------|-------|---------|
| **Ollama Lokal** | `ollama pull gemma3:27b` | $0 |
| **OpenRouter Free** | API key ile free modeller | $0 |
| **Gemini Flash** | Google AI free tier | $0 |
| **Groq Free** | Groq free tier | $0 |

### API Key ile

| Provider | Key Env | En İyi Model | Fiyat |
|----------|---------|-------------|-------|
| OpenRouter | `OPENROUTER_API_KEY` | 200+ model | Değişken |
| OpenAI | `OPENAI_API_KEY` | GPT-4o | $0.03/1K |
| Anthropic | `ANTHROPIC_API_KEY` | Claude 4 Sonnet | $0.003/1K |
| DeepSeek | `DEEPSEEK_API_KEY` | DeepSeek V3 | **EN UCUZ** |
| Groq | `GROQ_API_KEY` | Llama 3.3 70B | **EN HIZLI** |

---

## 2. Lokal Modeller (Ücretsiz)

### Ollama ile Kurulum

```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull gemma3:27b    # ÖNERİLEN
```

### Önerilen Modeller (VRAM'a Göre)

| VRAM | Model | Parametre | Context | İndirme |
|------|-------|-----------|---------|---------|
| 4 GB | Qwen3 30B MoE | 30B (3B aktif) | 128K | `ollama pull qwen3:30b-a3b` |
| 4 GB | Phi-4 Mini | 3.8B | 128K | `ollama pull phi4-mini` |
| 4 GB | Llama 3.2 1B | 1.2B | 128K | `ollama pull llama3.2:1b` |
| 8 GB | DeepSeek R1 Distill 8B | 8B | 128K | `ollama pull deepseek-r1:8b` |
| 8 GB | Mistral Small 3.1 | 24B | 128K | `ollama pull mistral-small3.1` |
| 8 GB | Qwen 2.5 Coder 7B | 7B | 128K | `ollama pull qwen2.5-coder:7b` |
| 16 GB | Gemma 3 27B | 27B | 256K | `ollama pull gemma3:27b` |
| 16 GB | Gemma 3 12B | 12B | 256K | `ollama pull gemma3:12b` |
| 16 GB | Pixtral 12B | 12B | 128K | `ollama pull pixtral:12b` |
| 24 GB | Llama 4 Scout | 109B (17B aktif) | 10M | `ollama pull llama4:scout` |
| 48 GB | Llama 3.3 70B | 70B | 128K | `ollama pull llama3.3:70b` |
| 48 GB | DeepSeek R1 67B | 67B | 128K | `ollama pull deepseek-r1:67b` |

### Diğer Lokal Yöntemler

| Yöntem | Açıklama | Kurulum |
|--------|----------|---------|
| vLLM | Yüksek performans server | `pip install vllm` |
| LM Studio | GUI ile model yönetimi | [lmstudio.ai](https://lmstudio.ai) |
| Llamafile | Tek dosya çalıştırma | [llamafile.com](https://llamafile.com) |

---

## 3. Direct Provider'lar

| # | Provider | Modeller | Key Env | Özellik |
|---|----------|----------|---------|---------|
| 1 | **OpenAI** | GPT-4o, o1, o3, o4-mini | `OPENAI_API_KEY` | Multimodal, reasoning |
| 2 | **Anthropic** | Claude 4, 3.5 Sonnet, Opus 4.1 | `ANTHROPIC_API_KEY` | Coding, 200K context |
| 3 | **Google AI** | Gemini 2.5 Pro/Flash, 2.0 | `GOOGLE_AI_API_KEY` | 1M+ context, free tier |
| 4 | **Mistral** | Mistral Large 3, Codestral | `MISTRAL_API_KEY` | Avrupa, open weights |
| 5 | **DeepSeek** | V3, R1, V4, R2 | `DEEPSEEK_API_KEY` | **EN UCUZ!** |
| 6 | **xAI** | Grok 2, Grok 4 | `XAI_API_KEY` | X entegrasyonu |
| 7 | **Cohere** | Command R+, Command R2 | `COHERE_API_KEY` | RAG optimize |
| 8 | **Perplexity** | Sonar, Sonar Deep Research v2 | `PERPLEXITY_API_KEY` | Web search built-in |
| 9 | **Groq** | Llama 3.3 70B, Gemma 2 | `GROQ_API_KEY` | **EN HIZLI!** LPU |
| 10 | **AI21 Labs** | Jamba 1.5 | `AI21_API_KEY` | SSM mimari |
| 11 | **Reka** | Reka Core, Edge, Flash | `REKA_API_KEY` | Multimodal |
| 12 | **Cerebras** | Llama 3.3 70B | `CEREBRAS_API_KEY` | Wafer-scale chip |
| 13 | **Fireworks** | Llama, Mixtral | `FIREWORKS_API_KEY` | Hızlı inference |
| 14 | **Replicate** | Her model | `REPLICATE_API_KEY` | Cloud run |
| 15 | **StepFun** | Step-2 16K | `STEPFUN_API_KEY` | Çin |
| 16 | **Aleph Alpha** | Luminous | `ALEPH_ALPHA_API_KEY` | Almanya, GDPR |
| 17 | **Sarvam** | Sarvam-2 | `SARVAM_API_KEY` | Hint dilleri |
| 18 | **Voyage** | Voyage-3 | `VOYAGE_API_KEY` | Embedding |
| 19 | **Upstage** | Solar Mini | `UPSTAGE_API_KEY` | Kore |
| 20 | **GigaChat** | GigaChat Max | `GIGACHAT_API_KEY` | Rusya |

---

## 4. Aggregator Provider'lar

| # | Provider | Model Sayısı | Key Env | Özellik |
|---|----------|-------------|---------|---------|
| 1 | **OpenRouter** | 200+ | `OPENROUTER_API_KEY` | **EN BÜYÜK!** Free modeller |
| 2 | **Together AI** | 100+ | `TOGETHER_API_KEY` | Açık model optimize |
| 3 | **Hugging Face** | 200K+ | `HF_API_KEY` | Model hub |
| 4 | **DeepInfra** | 50+ | `DEEPINFRA_API_KEY` | Ucuz inference |
| 5 | **GLHF** | 13+ | `GLHF_API_KEY` | Gaming |
| 6 | **Novita** | 12+ | `NOVITA_API_KEY` | Ucuz |
| 7 | **Hyperbolic** | 13+ | `HYPERBOLIC_API_KEY` | Merkeziyetsiz |
| 8 | **SiliconFlow** | 17+ | `SILICONFLOW_API_KEY` | Çin |
| 9 | **Lepton AI** | 5+ | `LEPTON_API_KEY` | Serverless |
| 10 | **Chutes** | 10+ | `CHUTES_API_KEY` | Ücretsiz modeller |

---

## 5. Enterprise Cloud Provider'lar

| # | Provider | Modeller | Key Env | Özellik |
|---|----------|----------|---------|---------|
| 1 | **Azure OpenAI** | GPT-4o, o1 | `AZURE_OPENAI_KEY` | Enterprise SLA |
| 2 | **AWS Bedrock** | Claude, Llama, Titan | `AWS_ACCESS_KEY_ID` | AWS native |
| 3 | **Vertex AI** | Gemini, Claude, Llama | `GOOGLE_CLOUD_KEY` | GCP native |
| 4 | **NVIDIA NIM** | Llama, Mistral | `NVIDIA_API_KEY` | Enterprise AI |
| 5 | **SambaNova** | Llama, Mistral | `SAMBANOVA_API_KEY` | Reconfigurable |
| 6 | **IBM WatsonX** | Granite 4.0, Llama | `IBM_API_KEY` | IBM Cloud |

---

## 6. Çin AI Provider'ları

| # | Provider | Modeller | Key Env | Özellik |
|---|----------|----------|---------|---------|
| 1 | **Zhipu AI** | GLM-4 | `ZHIPU_API_KEY` | Akademik |
| 2 | **Moonshot** | Kimi (128K) | `MOONSHOT_API_KEY` | Uzun context |
| 3 | **Yi (01.AI)** | Yi Lightning | `YI_API_KEY` | Hızlı |
| 4 | **Baidu ERNIE** | ERNIE 4.0 | `BAIDU_API_KEY` | Arama entegrasyonu |
| 5 | **MiniMax** | abab 6.5 | `MINIMAX_API_KEY` | Sosyal |
| 6 | **Qwen Direct** | Qwen4 Max | `QWEN_API_KEY` | Alibaba |
| 7 | **Mod** | DeepSeek uyumlu | `MOD_API_KEY` | Ucuz |

---

## 7. AI Gateway/Router Provider'ları

| # | Provider | Özellik | Key Env |
|---|----------|---------|---------|
| 1 | **Unify AI** | ML bazlı routing: `router@q>0.9&c<0.001` | `UNIFY_API_KEY` |
| 2 | **Portkey** | Failover, cache, cost tracking | `PORTKEY_API_KEY` |
| 3 | **Helicone** | Observability proxy, analytics | `HELICONE_API_KEY` |
| 4 | **NotDiamond** | ML model router, auto-optimize | `NOTDIAMOND_API_KEY` |
| 5 | **AI/ML API** | %80 indirimli API | `AIMLAPI_API_KEY` |
| 6 | **Glama** | MCP gateway, multi-provider | `GLLML_API_KEY` |
| 7 | **Requesty** | A/B testing router | `REQUESTY_API_KEY` |
| 8 | **LiteLLM** | Self-hosted, 100+ provider | `LITELLM_API_KEY` |
| 9 | **Cloudflare Workers AI** | Edge inference | `CLOUDFLARE_API_KEY` |

---

## 8. Lokal Inference Provider'ları

| # | Provider | Tür | Açıklama |
|---|----------|-----|----------|
| 1 | **Ollama** | Server | En popüler, 50K+ model |
| 2 | **vLLM** | Server | Yüksek performans, PagedAttention |
| 3 | **LM Studio** | GUI | Drag & drop model yönetimi |
| 4 | **Llamafile** | Single-file | `./model.llamafile` ile çalışır |

---

## 9. Model Seçim Rehberi

### En İyi Coding

| Model | Provider | Fiyat | Context |
|-------|----------|-------|---------|
| Claude 4 Sonnet | Anthropic | $$$ | 200K |
| GPT-4o | OpenAI | $$$ | 128K |
| DeepSeek V3 | DeepSeek | $ | 64K |
| Qwen 2.5 Coder | Lokal | Free | 128K |

### En İyi Reasoning

| Model | Provider | Fiyat | Context |
|-------|----------|-------|---------|
| DeepSeek R1 | DeepSeek/Lokal | $/Free | 164K |
| o4-mini | OpenAI | $$ | 200K |
| Claude 4 Sonnet | Anthropic | $$$ | 200K |
| Gemma 3 27B | Lokal | Free | 256K |

### En Uzun Context

| Model | Context | Provider |
|-------|---------|----------|
| Gemini Exp | **2M** | Google |
| Gemini 2.0 Flash | **1M** | Google |
| Llama 4 Scout | **10M** | Lokal/Ollama |
| Gemma 3 27B | **256K** | Lokal |

### En Hızlı

| Model | Provider | Latency |
|-------|----------|---------|
| Llama 3.3 70B | Groq | ~50ms |
| Gemma 2 9B | Groq | ~30ms |
| GPT-4o-mini | OpenAI | ~200ms |
| Gemini 2.0 Flash | Google | ~100ms |

### En Ucuz

| Model | Provider | Fiyat/1M tokens |
|-------|----------|-----------------|
| DeepSeek V3 | DeepSeek | $0.27/$1.10 |
| DeepSeek R1 | DeepSeek | $0.55/$2.19 |
| Gemini 2.0 Flash | Google | Free tier |
| Qwen3 30B MoE | Ollama | Free |

---

## 10. Yapılandırma

### .env Dosyası

```bash
# ═══ ÖNERİLEN BAŞLANGIÇ ═══
OPENROUTER_API_KEY=sk-or-v1-xxx     # 200+ model, $5 ücretsiz kredi
OLLAMA_HOST=http://localhost:11434   # Lokal, ücretsiz

# ═══ DİĞER PROVIDER'LAR (Opsiyonel) ═══
OPENAI_API_KEY=sk-xxx
ANTHROPIC_API_KEY=sk-ant-xxx
GOOGLE_AI_API_KEY=xxx
DEEPSEEK_API_KEY=xxx
GROQ_API_KEY=gsk_xxx
MISTRAL_API_KEY=xxx
COHERE_API_KEY=xxx
PERPLEXITY_API_KEY=xxx
XAI_API_KEY=xxx

# ═══ GATEWAY ═══
UNIFY_API_KEY=xxx          # Akıllı routing
PORTKEY_API_KEY=xxx        # Enterprise gateway
```

### CLI ile Model Değiştirme

```bash
# Model listesi
sentient model list

# Model değiştir
sentient model set openai/gpt-4o
sentient model set ollama/gemma3:27b
sentient model set deepseek/deepseek-r1

# Akıllı routing
sentient model set "unify/router@q>0.9&c<0.001"
sentient model set "openrouter/auto"
```

---

*🧠 SENTIENT OS - The Operating System That Thinks*  
*93 Crate | 57+ Provider | 245+ Model | 200K+ Aggregator Erişimi*
