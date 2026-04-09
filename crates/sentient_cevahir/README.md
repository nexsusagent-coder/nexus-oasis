# Sentient Cevahir - LLM Cognitive Engine

[![Crates.io](https://img.shields.io/crates/v/sentient_cevahir.svg)](https://crates.io/crates/sentient_cevahir)
[![Documentation](https://docs.rs/sentient_cevahir/badge.svg)](https://docs.rs/sentient_cevahir)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📖 Genel Bakış

**Sentient Cevahir**, [Cevahir AI](https://github.com/myylogic/cevahir-ai) projesinin SENTIENT OS ile entegrasyonunu sağlayan Rust crate'idir. Bu modül, Türkçe optimizasyonlu full-stack LLM engine'i SENTIENT ekosistemine dahil eder.

### 🎯 Temel Özellikler

| Özellik | Açıklama |
|---------|----------|
| **Neural Network (V-7)** | RoPE, RMSNorm, SwiGLU, KV Cache, MoE, GQA |
| **Cognitive Strategies** | Direct, Think, Debate, Tree of Thoughts |
| **Turkish BPE Tokenizer** | Native Türkçe tokenizer, GPU batch processing |
| **Memory & RAG** | Vector store, semantic cache, knowledge graph |
| **Tool Execution** | Dynamic tool registration ve execution |
| **Middleware Pipeline** | Cache, tracing, metrics, validation |

## 🏗️ Mimari

```
┌─────────────────────────────────────────────────────────────────┐
│                      SENTIENT OS Core                            │
│                    (Rust Event Graph)                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    sentient_cevahir                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │   Cognitive  │  │   Tokenizer  │  ┌──────────────┐ │
│  │ CevahirBridge│  │CognitiveManager│ │  │TokenizerCore │ │
│  │  (PyO3)      │  │  (Strategies)  │ │  │  (BPE)       │ │
│  └──────────────┘  └────────────────┘ │  └──────────────┘ │
│                              │        │                     │
│                              ▼        │                     │
│                    ┌──────────────┐  │                     │
│                    │ ModelManager │  │                     │
│                    │ (V-7 NN)     │  │                     │
│                    └──────────────┘  │                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Python Layer                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │ Cevahir AI Engine (PyTorch)                                 ││
│  │ - src/neural_network.py (V-7 Architecture)                 ││
│  │ - cognitive_management/cognitive_manager.py                 ││
│  │ - tokenizer_management/core/tokenizer_core.py               ││
│  │ - model/cevahir.py (Unified API)                           ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## 🚀 Kullanım

### Temel Başlatma

```rust
use sentient_cevahir::{CevahirBridge, CevahirConfig, CognitiveStrategy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Varsayılan yapılandırma ile başlat
    let config = CevahirConfig::default();
    let bridge = CevahirBridge::new(config)?;
    
    // Basit metin üretimi
    let response = bridge.generate("Merhaba dünya", 128).await?;
    println!("Response: {}", response);
    
    Ok(())
}
```

### Cognitive Strateji ile İşlem

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

let bridge = CevahirBridge::new(CevahirConfig::default())?;

// Tree of Thoughts ile karmaşık problem çözümü
let output = bridge.process_with_strategy(
    "Bu algoritmanın zaman karmaşıklığı nedir?",
    CognitiveStrategy::TreeOfThoughts,
).await?;

println!("Reasoning: {:?}", output.reasoning);
println!("Response: {}", output.response);
```

### Tokenizer Kullanımı

```rust
use sentient_cevahir::TokenizerWrapper;

let tokenizer = TokenizerWrapper::new(
    "data/vocab_lib/vocab.json",
    "data/merges_lib/merges.txt",
)?;

// Encode
let (tokens, ids) = tokenizer.encode("Merhaba dünya")?;
println!("Tokens: {:?}", tokens);
println!("IDs: {:?}", ids);

// Decode
let text = tokenizer.decode(&ids)?;
println!("Decoded: {}", text);
```

### Tool Kayıt Etme

```rust
use sentient_cevahir::{CevahirBridge, ToolDefinition};

let bridge = CevahirBridge::new(CevahirConfig::default())?;

//SENTIENT araçlarını kaydet
bridge.register_tool(ToolDefinition {
    name: "browser".to_string(),
    description: "Web sayfalarını analiz et".to_string(),
    parameters: vec!["url".to_string()],
    executor: |args| {
        // Browser tool implementation
        Ok("Page analyzed".to_string())
    },
})?;

bridge.register_tool(ToolDefinition {
    name: "sandbox".to_string(),
    description: "Kod çalıştır".to_string(),
    parameters: vec!["code".to_string()],
    executor: |args| {
        // Sandbox implementation
        Ok("Code executed".to_string())
    },
})?;
```

## 🧠 Cognitive Stratejiler

| Strateji | Kullanım Senaryosu | Açıklama |
|----------|-------------------|----------|
| **Direct** | Basit sorgular | Doğrudan yanıt, düşünme adımı yok |
| **Think** | Analitik görevler | İç ses üretimi, adım adım analiz |
| **Debate** | Fikir üretimi | Çoklu perspektif, tartışma |
| **TreeOfThoughts** | Karmaşık problemler | Ağaç yapısında düşünme, debug |

### Strateji Seçim Mantığı

```rust
// Otomatik strateji seçimi
let strategy = CognitiveStrategy::auto_select(&input);

// Manual seçim
let strategy = CognitiveStrategy::Think;
```

## ⚙️ Yapılandırma

```rust
use sentient_cevahir::CevahirConfig;

let config = CevahirConfig {
    // Device
    device: "cuda".to_string(),  // veya "cpu"
    seed: Some(42),
    log_level: "INFO".to_string(),
    
    // Model
    vocab_size: 60000,
    embed_dim: 512,
    num_heads: 8,
    num_layers: 8,
    dropout: 0.15,
    
    // V-4 Features
    use_rope: true,
    use_rmsnorm: true,
    use_swiglu: true,
    use_kv_cache: true,
    use_moe: false,
    
    // Cognitive
    default_strategy: CognitiveStrategy::Think,
    max_thinking_steps: 5,
    enable_memory: true,
    
    // Paths
    vocab_path: "data/vocab_lib/vocab.json".to_string(),
    merges_path: "data/merges_lib/merges.txt".to_string(),
    model_path: None,  // Eğitilmiş model yolu
    
    ..Default::default()
};
```

## 🔗 SENTIENT Entegrasyonu

### Memory Entegrasyonu

```rust
use sentient_cevahir::CevahirBridge;
use sentient_memory::MemoryStore;

let memory = MemoryStore::new()?;
let bridge = CevahirBridge::with_memory(config, memory)?;

// Bellek ile birlikte işlem
let output = bridge.process_with_memory(
    "Önceki hataları hatırla",
    CognitiveStrategy::Think,
).await?;
```

### Event Graph Entegrasyonu

```rust
use sentient_graph::EventGraph;
use sentient_cevahir::CevahirBridge;

let graph = EventGraph::new()?;
let bridge = CevahirBridge::new(config)?;

// Event emit et
bridge.on_response(|output| {
    graph.emit(Event::CognitiveResponse {
        strategy: output.strategy,
        tokens: output.token_count,
    });
});
```

## 📊 Performans

| Metrik | Değer |
|--------|-------|
| **Inference (CPU)** | ~50 tokens/s |
| **Inference (GPU)** | ~500 tokens/s |
| **Tokenizer** | ~100K tokens/s |
| **Memory Usage** | ~2-4 GB (model boyutuna göre) |

## 🔧 Gereksinimler

- Rust 1.70+
- Python 3.8+
- PyTorch 2.0+
- CUDA (opsiyonel, GPU için)

## 📜 Lisans

MIT License - SENTIENT OS ile uyumlu

## 🙏 Teşekkürler

Bu crate, [Cevahir AI](https://github.com/myylogic/cevahir-ai) projesi üzerine inşa edilmiştir. 
Orijinal proje Muhammed Yasin Yılmaz ([@myylogic](https://github.com/myylogic)) tarafından geliştirilmiştir.

---

**SENTIENT OS - The Operating System That Thinks**
