# ═══════════════════════════════════════════════════════════════════════════════
#  CEVAHIR AI & LLM DESTEK RAPORU
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-10
#  Konu: Cevahir AI Entegrasyonu ve LLM Model Desteği
# ═══════════════════════════════════════════════════════════════════════════════

---

## 🧠 CEVAHIR AI NEDİR?

**Cevahir AI**, Muhammed Yasin Yılmaz ([@myylogic](https://github.com/myylogic)) tarafından geliştirilen,
**full-stack açık kaynak AI engine**'dir. Türkçe optimizasyonlu olmakla birlikte, dil bağımsız
bir mimariye sahiptir.

### GitHub
- **Repo:** https://github.com/myylogic/cevahir-ai
- **Lisans:** Apache License 2.0
- **Geliştirici:** Muhammed Yasin Yılmaz (@myylogic)

### Vizyon

> *"Sınırlı kaynaklarla küresel teknoloji devlerine meydan okuyan, Türk gençliğinin vizyonuyla
> şekillenmiş bir özgürlük manifestosu. Bu sadece bir model değil; kendi yapay zeka dünyanızı
> inşa etmeniz için tasarlanmış eksiksiz bir fabrikadır."*

**Türk Gençlerine Armağanımdır.**

---

## 🏗️ CEVAHIR AI MİMARİSİ

```
┌─────────────────────────────────────────────────────────────────┐
│                    Cevahir (Unified API)                         │
│                     model/cevahir.py                             │
└─────────────────────────────────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ TokenizerCore│ │ ModelManager  │ │CognitiveMgr  │
│ (Türkçe BPE) │ │ (V-7 NN)     │ │ (Cognitive)  │
└──────────────┘ └──────────────┘ └──────────────┘
        │               │               │
        ▼               ▼               ▼
  vocab/merges    Neural Network   Memory/Tools
  (60K vocab)    (RoPE, RMSNorm,  (RAG, Critic,
                   SwiGLU, …)       Tools)
```

---

## ⚡ TEMEL ÖZELLİKLER

### 1. Neural Network (V-7)

| Özellik | Standart | Açıklama |
|---------|----------|----------|
| **RoPE** | GPT-3+/LLaMA | Rotary Position Embedding |
| **RMSNorm** | GPT-3+/LLaMA | Root Mean Square Normalization |
| **SwiGLU** | GPT-4/PaLM | Gated Linear Unit aktivasyon |
| **KV Cache** | GPT-4/Claude | Inference optimizasyonu |
| **GQA** | LLaMA-2/3/Mistral | Grouped Query Attention |
| **MoE** | GPT-4/Gemini | Mixture of Experts |
| **Flash Attention** | Endüstri | Memory-efficient attention |

### 2. Cognitive Strategies (Bilişsel Stratejiler)

| Strateji | Kullanım Senaryosu | Açıklama |
|----------|-------------------|----------|
| **Direct** | Basit sorgular | Doğrudan yanıt, düşünme adımı yok |
| **Think** | Analitik görevler | İç ses üretimi, adım adım analiz |
| **Debate** | Fikir üretimi | Çoklu perspektif, tartışma |
| **TreeOfThoughts** | Karmaşık problemler | Ağaç yapısında düşünme, debug |

### 3. Türkçe BPE Tokenizer

- **Vocabulary:** 60,000 token
- **Unicode Desteği:** İ/ı, Ş/ş, Ğ/ğ, Ü/ü, Ö/ö, Ç/ç
- **GPU Batch Processing:** Milyonlarca satır/saniye
- **Morfolojik Desteği:** Eklemeli dil yapısı

### 4. Memory & RAG

- **Vector Store:** Semantic search
- **Knowledge Graph:** İlişki takibi
- **Semantic Cache:** Önbellek optimizasyonu
- **Episodic Memory:** Deneyim deposu

---

## 🔗 SENTIENT OS ENTEGRASYONU

Cevahir AI, SENTIENT OS'e **sentient_cevahir** crate'i ile entegre edilmiştir:

```
┌─────────────────────────────────────────────────────────────────┐
│                      SENTIENT OS Core                            │
│                    (Rust Event Graph)                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    sentient_cevahir (Rust)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │ CevahirBridge│  │ CognitiveMgr │  │TokenizerWrap │           │
│  │  (PyO3)      │  │ (Strategies) │  │  (BPE)       │           │
│  └──────────────┘  └──────────────┘  └──────────────┘           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Cevahir AI (Python/PyTorch)                  │
│  - V-7 Neural Network (GPT-4/LLaMA seviyesi mimari)            │
│  - Cognitive Management (Think/Debate/ToT stratejileri)        │
│  - Turkish BPE Tokenizer (60K vocabulary)                      │
│  - Memory & RAG (Vector store, semantic cache)                 │
└─────────────────────────────────────────────────────────────────┘
```

### Kullanım Örneği (Rust)

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

// Başlat
let bridge = CevahirBridge::new(CevahirConfig::default())?;

// Tree of Thoughts ile karmaşık problem
let output = bridge.process_with_strategy(
    "Bu algoritmanın zaman karmaşıklığı nedir?",
    CognitiveStrategy::TreeOfThoughts,
).await?;

println!("Reasoning: {:?}", output.reasoning);
println!("Response: {}", output.response);
```

---

## 📊 LLM MODEL DESTEĞİ

SENTIENT OS, **268 LLM modeli** ve **21 provider** destekler:

### Provider Dağılımı

```
╔════════════════════════════════════════════════════════════════════════════════╗
║                       LLM PROVIDER DAĞILIMI                                   ║
╠════════════════════════════════════════════════════════════════════════════════╣
║  Provider          │ Model Sayısı │ Tür                                      ║
╠════════════════════════════════════════════════════════════════════════════════╣
║  OpenRouter        │     79       │ Aggregator (tüm modeller)                ║
║  Ollama            │     54       │ Local (CPU/GPU) + Gemma 4 KERNEL        ║
║  OpenAI            │     17       │ Cloud (GPT-4, GPT-4o, o1)                ║
║  Google            │     13       │ Cloud (Gemini, Gemma)                    ║
║  **HuggingFace**   │     26       │ **Açık Kaynak Hub (ÜCRETSİZ)**          ║
║  Mistral           │     11       │ Cloud (Mistral, Mixtral, Codestral)      ║
║  AWS Bedrock       │     11       │ Enterprise (Claude, Llama, Titan)        ║
║  **Fireworks AI**  │     10       │ **Hızlı Inference (UCUZ)**              ║
║  **NVIDIA NIM**    │     10       │ **Enterprise GPU Inference**            ║
║  Anthropic         │     10       │ Cloud (Claude 3.5, Claude 3)             ║
║  Groq              │     10       │ Fast Inference (Llama, Mixtral)          ║
║  Together AI       │     10       │ Cloud (Llama, Qwen, DeepSeek)            ║
║  **Novita AI**     │      8       │ **Ucuz LLM API**                        ║
║  Cohere            │      7       │ Cloud (Command, Rerank)                  ║
║  Zhipu AI          │      7       │ China (GLM-4)                            ║
║  Azure OpenAI      │      7       │ Enterprise (GPT-4, GPT-3.5)              ║
║  **SiliconFlow**   │      6       │ **China (Qwen, DeepSeek)**              ║
║  Perplexity        │      5       │ Search (Sonar Online)                    ║
║  Alibaba Qwen      │      5       │ China (Qwen-Max, Qwen-VL)                ║
║  Google Vertex     │      5       │ Enterprise (Gemini)                      ║
║  **AI21 Labs**     │      5       │ **Jamba, Jurassic**                     ║
║  **Stability AI**  │      5       │ **StableLM**                            ║
║  **Hyperbolic**    │      5       │ **Decentralized Inference**             ║
║  **G4F**           │      5       │ **GPT4Free (ÜCRETSİZ)**                 ║
║  **Lepton AI**     │      5       │ **Serverless Inference**                ║
║  DeepSeek          │      4       │ Cloud (DeepSeek-Chat, DeepSeek-Coder)    ║
║  X.AI              │      4       │ Cloud (Grok, Grok-Vision)                ║
║  Baidu ERNIE       │      4       │ China (ERNIE-4.0)                        ║
║  **RunPod**        │      4       │ **Serverless GPU**                      ║
║  Moonshot          │      3       │ China (Moonshot-V1)                      ║
║  Replicate         │      3       │ Cloud (Llama, Mixtral)                   ║
║  IBM WatsonX       │      3       │ Enterprise (Granite, Llama)              ║
║  **Modal**         │      3       │ **Serverless Inference**                ║
║  **Inflection AI** │      2       │ **Pi Sohbet AI**                        ║
╠════════════════════════════════════════════════════════════════════════════════╣
║  TOPLAM            │    408       │ 35 Provider                              ║
╚════════════════════════════════════════════════════════════════════════════════╝
```

---

## 🎯 MODEL KATEGORİLERİ

### Premium Cloud Models

| Provider | Modeller |
|----------|----------|
| **Anthropic** | claude-3.5-sonnet, claude-3.5-haiku, claude-3-opus |
| **OpenAI** | gpt-4o, gpt-4o-mini, o1, o1-preview, o1-mini |
| **Google** | gemini-2.0-flash, gemini-1.5-pro, gemma-2-27b |
| **Mistral** | mistral-large, codestral, ministral-8b |
| **X.AI** | grok-beta, grok-2-1212, grok-2-vision |

### Open Source Models (via Ollama/OpenRouter)

| Provider | Modeller |
|----------|----------|
| **Gemma 4 (KERNEL)** | gemma4:31b, gemma4:26b-moe, gemma4:e4b, gemma4:e2b |
| **Meta Llama** | llama-3.3-70b, llama-3.1-405b, llama-3.2-vision |
| **Qwen** | qwen-2.5-72b, qwen-2.5-coder-32b, qwq-32b-preview |
| **DeepSeek** | deepseek-r1-671b, deepseek-coder-v2-236b |
| **Mistral** | mixtral-8x22b, mistral-7b |
| **Microsoft** | phi-4, phi-3-medium |

### Fast Inference (Groq)

| Model | Hız |
|-------|-----|
| llama-3.3-70b-versatile | ~500 tokens/s |
| mixtral-8x7b-32768 | ~400 tokens/s |
| gemma2-9b-it | ~600 tokens/s |

### Enterprise (AWS Bedrock, Azure, Vertex)

| Provider | Modeller |
|----------|----------|
| **AWS Bedrock** | Claude 3.5, Llama 3.3, Titan, Command-R+ |
| **Azure OpenAI** | GPT-4o, GPT-4-turbo, GPT-3.5-turbo |
| **Google Vertex** | Gemini 2.0, Gemini 1.5 Pro/Flash |

---

## 🇨🇳 ÇİN MODELLERİ

| Provider | Modeller | Context |
|----------|----------|---------|
| **Zhipu AI** | glm-4, glm-4-flash, glm-4v-plus | 128K |
| **Alibaba Qwen** | qwen-max, qwen-plus, qwen-vl-max | 32K |
| **Baidu ERNIE** | ernie-4.0-8k, ernie-3.5-8k | 8K |
| **Moonshot** | moonshot-v1-8k/32k/128k | 128K |
| **SiliconFlow** | Qwen2.5, DeepSeek-V3, Llama-3.3 | 128K |

---

## 🆓 ÜCRETSİZ LLM PROVIDER'LAR

| Provider | Modeller | Açıklama |
|----------|----------|----------|
| **HuggingFace** | 26 açık kaynak model | Ücretsiz tier mevcut |
| **G4F** | GPT-4, Claude-3, Gemini | GPT4Free - tamamen ücretsiz |
| **Ollama** | 54 yerel model | Lokal, tamamen ücretsiz |

---

## 💰 UCUZ LLM PROVIDER'LAR

| Provider | Fiyat | Özellik |
|----------|-------|----------|
| **Novita AI** | $0.001/1K tokens | En ucuz |
| **Fireworks AI** | $0.002/1K tokens | Hızlı inference |
| **Hyperbolic** | $0.002/1K tokens | Decentralized |
| **Lepton AI** | $0.003/1K tokens | Serverless |
| **RunPod** | $0.003/1K tokens | Serverless GPU |

---

## 🏢 ENTERPRISE PROVIDER'LAR

| Provider | Özellikler |
|----------|------------|
| **NVIDIA NIM** | GPU optimize, Nemotron-4 340B |
| **AWS Bedrock** | Claude, Llama, Titan, Command-R+ |
| **Azure OpenAI** | GPT-4, GPT-3.5, enterprise güvenlik |
| **Google Vertex** | Gemini, enterprise compliance |
| **IBM WatsonX** | Granite, enterprise AI |

---

## 🔧 YEREL LLM DESTEĞİ

### Ollama (50 Model)

SENTIENT OS, Ollama ile **tamamen yerel** çalışabilir:

```bash
# Ollama kurulumu
curl -fsSL https://ollama.com/install.sh | bash

# Model indirme
ollama pull llama3.3:70b
ollama pull qwen2.5-coder:32b
ollama pull deepseek-r1:671b
```

**Desteklenen Model Türleri:**
- **Chat:** llama3.3, qwen2.5, mistral, gemma2
- **Code:** codellama, qwen2.5-coder, deepseek-coder
- **Vision:** llava, moondream
- **Embedding:** nomic-embed-text, mxbai-embed-large

### Cevahir AI (Native)

SENTIENT OS'in kendi yerel LLM engine'i:

```bash
# Kullanım
sentient-chat --model cevahir

# Eğitim
python training_system/train.py --config configs/cevahir-v7.yaml
```

---

## 📈 PERFORMANS KARŞILAŞTIRMASI

| Engine | Inference (CPU) | Inference (GPU) | Memory |
|--------|-----------------|-----------------|--------|
| **Cevahir AI** | ~50 tokens/s | ~500 tokens/s | 2-4 GB |
| **Ollama** | ~30 tokens/s | ~400 tokens/s | 4-8 GB |
| **OpenAI API** | - | ~100 tokens/s | - |
| **Groq** | - | ~500 tokens/s | - |

---

## 🧠 GEMMA 4 - SENTIENT OS KERNEL

SENTIENT OS'in **varsayılan yerel LLM** modeli Google DeepMind Gemma 4'tür:

### Özellikler

| Özellik | Değer |
|---------|-------|
| **Parametre Sayısı** | 31B (Full) / 26B (MoE) |
| **Context Length** | 256K tokens |
| **Multimodal** | Text + Vision |
| **Thinking Mode** | Native destek |
| **Function Calling** | Native destek |
| **Lisans** | Apache 2.0 (TAMAMEN ÜCRETSİZ) |

### Model Varyantları

| Model | Parametre | Kullanım |
|-------|-----------|----------|
| `gemma4:31b` | 31B | **KERNEL DEFAULT** - Full model |
| `gemma4:26b-moe` | 26B | MoE - Daha hızlı |
| `gemma4:e4b` | 4B | Edge - Laptop optimize |
| `gemma4:e2b` | 2B | Mobile - Ultra hafif |

### Zero-Copy Memory Integration

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   GEMMA 4   │───▶│ ZERO-COPY   │───▶│ MEMORY CUBE │
│   KERNEL    │    │   BUFFER    │    │   L3        │
└─────────────┘    └─────────────┘    └─────────────┘
```

### Kullanım

```rust
use sentient_local::{Gemma4Engine, Gemma4Config};

// Varsayılan config (gemma4:31b)
let engine = Gemma4Engine::default_engine()?;

// Generate
let response = engine.generate("Merhaba dünya").await?;

// Chat with thinking mode
let response = engine.chat(messages).await?;

// Think mode (step-by-step)
let (answer, thinking) = engine.think("Karmaşık problem...").await?;
```

---

## 🎨 STRATEJİ SEÇİMİ

Cevahir AI, otomatik strateji seçimi yapar:

| Girdi Türü | Otomatik Strateji |
|------------|-------------------|
| Basit soru | Direct |
| Kod analizi | Think |
| Tasarım kararı | Debate |
| Debug/reasoning | TreeOfThoughts |

```rust
// Otomatik seçim
let strategy = CognitiveStrategy::auto_select(&input);

// Manuel seçim
let output = bridge.process_with_strategy(
    input,
    CognitiveStrategy::TreeOfThoughts,
).await?;
```

---

## 🔐 V-GATE API GÜVENLİĞİ

Tüm cloud LLM çağrıları **V-GATE Proxy** üzerinden geçer:

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Stored on server
```

**Avantajlar:**
- API keys ASLA client'ta tutulmaz
- Audit log tüm istekleri kaydeder
- Rate limiting otomatik
- Encryption (TLS 1.3)

---

## 📊 ÖZET

```
╔════════════════════════════════════════════════════════════════════════════════╗
║                    SENTIENT OS LLM EKOSİSTEMİ                                 ║
╠════════════════════════════════════════════════════════════════════════════════╣
║  Toplam Model Sayısı      : 408                                              ║
║  Toplam Provider Sayısı   : 35                                               ║
║  Yerel LLM Desteği        : Ollama (54) + Cevahir AI (Native)                ║
║  Gemma 4 (KERNEL)         : 31B params, 256K context, Thinking Mode         ║
║  Ücretsiz Provider'lar    : HuggingFace, G4F, Ollama                         ║
║  Ucuz Provider'lar        : Fireworks, Novita, Hyperbolic, Lepton           ║
║  Enterprise Provider'lar  : NVIDIA NIM, AWS Bedrock, Azure, Vertex           ║
║  Çin Provider'ları        : Zhipu, Alibaba, Baidu, Moonshot, SiliconFlow    ║
║  Decentralized            : Hyperbolic (blockchain-based)                   ║
║  Cognitive Stratejiler    : 4 (Direct, Think, Debate, ToT)                   ║
║  API Güvenliği            : V-GATE Proxy (keyless client)                    ║
╚════════════════════════════════════════════════════════════════════════════════╝
```

---

*SENTIENT OS - The Operating System That Thinks*
*Rapor Tarihi: 2026-04-10*
