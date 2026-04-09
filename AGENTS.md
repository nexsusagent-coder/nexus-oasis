# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - PROJECT MISSION AND ARCHITECTURE
# ═══════════════════════════════════════════════════════════════════════════════

## MISSION AND IDENTITY

**Project Name:** SENTIENT OS  
**Developer:** Pi  
**Purpose:** Build an autonomous, secure, and high-performance AI Operating System with a Rust core.

This project is designed as an **integration platform** that enables different open-source projects to work harmoniously within the SENTIENT ecosystem. All architecture is based on modularity and extensibility principles.

---

## CORE ARCHITECTURE COMPONENTS

### 1. CENTRAL NERVOUS SYSTEM (Core)

The system's main orchestration is built on a Rust-based event-graph framework:

- Low memory consumption
- Lock-free concurrency
- High-performance event processing

**Location:** `crates/sentient_core/`, `crates/sentient_graph/`

### 2. PyO3 Bridge (Integration Layer)

Python-based tools are integrated into the Rust core as **native modules** using PyO3 library:

- Zero-copy data flow
- Type-safe bridges
- Automatic error conversion

**Location:** `crates/sentient_python/`

### 3. COGNITIVE MEMORY (Hippocampus)

All agent history, short/long-term memory, and knowledge graphs are managed in a memory system running on local SQLite:

- Episodic memory (experiences)
- Semantic memory (information)
- Procedural memory (methods)

**Location:** `crates/sentient_memory/`

### 4. WEB INTERACTION MODULE

Internet interactions via browser-use and Lightpanda integrations:

- Headless browser control
- Web scraping
- Form filling and automation

**Location:** `crates/oasis_browser/`

### 5. SANDBOX EXECUTION MODULE

Isolated code execution in Docker:

- Secure code execution
- Resource isolation
- Timeout management

**Location:** `crates/oasis_manus/`, `crates/sentient_sandbox/`

### 6. SECURITY SYSTEM (Guardrails)

Security filters for all input/output layers:

- Prompt injection detection
- Data leak prevention
- SQL injection protection

**Location:** `crates/sentient_guardrails/`

### 7. V-GATE ARCHITECTURE (API Security)

**API keys are NEVER in client code.**

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Stored on server
```

### 8. CEVAHIR AI ENGINE (Local LLM & Cognitive Reasoning)

**Full-stack LLM engine with cognitive reasoning capabilities.**

Cevahir AI,SENTIENT OS'inyerel LLM motoru olarak entegre edilmiştir:

- **Neural Network (V-7):** RoPE, RMSNorm, SwiGLU, KV Cache, MoE, GQA
- **Cognitive Strategies:** Direct, Think, Debate, Tree of Thoughts
- **Turkish BPE Tokenizer:** Native Türkçe tokenizer, GPU batch processing
- **Memory & RAG:** Vector store, semantic cache, knowledge graph
- **Tool Execution:** Dynamic tool registration ve execution
- **Middleware Pipeline:** Cache, tracing, metrics, validation

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
│  │CevahirBridge │  │ CognitiveMgr │  │TokenizerWrap │           │
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

**Kullanım Alanları:**

| Senaryo | Strateji | Açıklama |
|---------|----------|----------|
| Basit sorgular | Direct | Doğrudan yanıt |
| Kod analizi | Think | Adım adım analiz |
| Tasarım kararları | Debate | Çoklu perspektif |
| Debug/reasoning | TreeOfThoughts | Ağaç yapısında düşünme |

**Location:** `crates/sentient_cevahir/`

---

## ERROR MANAGEMENT AND IDENTITY

Raw errors from external systems (TypeError, Traceback, etc.) are not shown directly to the user. All outputs are converted to SENTIENT's professional format.

**Example:**
```
❌ Python: TypeError: 'NoneType' object is not subscriptable
✅ SENTIENT: DATA_MODULE: Requested data not found. Please check parameters.
```

---

## DEVELOPMENT METHODOLOGY

### Coding Standards

1. **Language:** Rust (core), Python (integrations)
2. **Testing:** Unit tests mandatory for each module
3. **Documentation:** Explanatory comments in code

### Models Used

- `qwen/qwen3.6-plus:free` - General tasks
- `qwen/qwen3-coder:free` - Code writing

### Workflow

1. Develop modules hierarchically
2. Compile and test at each step (`cargo build`, `cargo test`)
3. Analyze and fix errors
4. Verify stability

---

## INTEGRATION TABLE

| Category | Projects | License |
|----------|----------|---------|
| **LLM Engine** | **Cevahir AI** | **Apache-2** |
| Agent Framework | CrewAI, AutoGen, OpenHands | MIT |
| Browser | Browser-Use, Lightpanda | MIT |
| Memory | Mem0, MemGPT, ChromaDB | MIT/Apache-2 |
| Security | NeMo-Guardrails | Apache-2 |
| Sandbox | OpenManus, Open Interpreter | MIT/AGPL-3 |

**Detailed license information:** `THIRD_PARTY_NOTICES.md`

---

## PROJECT STATUS

| Metric | Value |
|--------|-------|
| Rust Crate | 38 |
| Rust Files | 561 |
| Tests | 560+ ✅ |
| Integrated Projects | 72 |

---

## CEVAHIR AI INTEGRATION DETAILS

### Mimari Özellikler

| Özellik | Standart | Açıklama |
|---------|----------|----------|
| **RoPE** | GPT-3+/LLaMA | Rotary Position Embedding |
| **RMSNorm** | GPT-3+/LLaMA | Root Mean Square Normalization |
| **SwiGLU** | GPT-4/PaLM | Gated Linear Unit aktivasyon |
| **KV Cache** | GPT-4/Claude | Inference optimizasyonu |
| **GQA** | LLaMA-2/3/Mistral | Grouped Query Attention |
| **MoE** | GPT-4/Gemini | Mixture of Experts |
| **YaRN** | LLaMA-3.1 | Uzun context desteği |
| **Flash Attention** | Endüstri | Memory-efficient attention |

### Cognitive Stratejiler

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

// Otomatik strateji seçimi
let output = bridge.process_with_strategy(
    "Bu kodu analiz et",
    CognitiveStrategy::Think,
).await?;

// Tree of Thoughts ile karmaşık problem
let output = bridge.process_with_strategy(
    "Bu hatanın kök nedenini bul",
    CognitiveStrategy::TreeOfThoughts,
).await?;
```

### Entegrasyon Noktaları

|SENTIENT Modül | Cevahir Bileşen | İşlev |
|----------------|-----------------|-------|
| sentient_memory | MemoryAdapter | Epizodik/semantik bellek |
| sentient_vector | VectorStore | Semantic search |
| sentient_graph | EventBus | Event emit |
| sentient_guardrails | ValidationMiddleware | Güvenlik kontrolü |
| oasis_browser | Tool (browser) | Web etkileşimi |
| sentient_sandbox | Tool (sandbox) | Kod çalıştırma |

### Kaynak

- **GitHub:** https://github.com/myylogic/cevahir-ai
- **Geliştirici:** Muhammed Yasin Yılmaz ([@myylogic](https://github.com/myylogic))
- **Lisans:** Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
