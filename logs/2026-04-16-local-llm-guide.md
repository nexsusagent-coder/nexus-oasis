# 🖥️ SENTIENT OS - Lokal LLM Çalıştırma Rehberi (2026)

**Tarih:** 2026-04-16  
**Hedef:** Ücretsiz, açık kaynak LLM'leri kendi bilgisayarında çalıştır

---

## 📊 2026 En Güncel Açık Kaynak Modeller (Nisan 2026)

### 🏆 Tier-1: En Yeni ve En Güçlü (2025-2026)

| Model | Parametre | Context | VRAM | Açık Kaynak | Ollama ID | Yetenek |
|-------|-----------|---------|------|-------------|-----------|---------|
| **Llama 4 Scout** | 109B MoE (16E) | 10M! | 48GB | ✅ Apache-2 | `llama4:scout` | Vision, Tools, 10M context |
| **Llama 4 Maverick** | 400B MoE (128E) | 1M | 96GB | ✅ Apache-2 | `llama4:maverick` | Vision, Tools, 1M context |
| **Qwen3 235B** | 235B MoE (22B aktif) | 128K | 24GB | ✅ Apache-2 | `qwen3:235b` | Reasoning, Tools |
| **Qwen3 32B** | 32B | 128K | 20GB | ✅ Apache-2 | `qwen3:32b` | Reasoning, Tools |
| **DeepSeek V4** | ~670B MoE | 256K | 96GB | ✅ MIT | `deepseek-v4` | Vision, Tools |
| **DeepSeek R2** | ~670B MoE | 256K | 96GB | ✅ MIT | `deepseek-r2` | Vision, Reasoning |
| **Gemma 3 27B** | 27B | 128K | 16GB | ✅ Gemma | `gemma3:27b` | Vision, Tools |
| **Phi-4** | 14B | 16K | 8GB | ✅ MIT | `phi4:14b` | Code, Reasoning |
| **Mistral Small 3.1** | 24B | 128K | 16GB | ✅ Apache-2 | `mistral-small3.1:24b` | Vision, Tools |

### 🥈 Tier-2: Orta Seviye (2024-2025)

| Model | Parametre | Context | VRAM | Açık Kaynak | Ollama ID | Yetenek |
|-------|-----------|---------|------|-------------|-----------|---------|
| **Qwen3 30B-A3B MoE** | 30B (3B aktif!) | 128K | 4GB | ✅ Apache-2 | `qwen3:30b-a3b` | Reasoning, çok hafif! |
| **Gemma 3 12B** | 12B | 128K | 8GB | ✅ Gemma | `gemma3:12b` | Vision |
| **Gemma 3 4B** | 4B | 128K | 4GB | ✅ Gemma | `gemma3:4b` | Vision |
| **Phi-4 Mini** | 5B | 128K | 4GB | ✅ MIT | `phi4-mini:5b` | Code |
| **Codestral 22B** | 22B | 32K | 14GB | ✅ (not OSS) | `codestral:22b` | Code specialist |
| **Qwen 2.5 Coder 14B** | 14B | 128K | 8GB | ✅ Apache-2 | `qwen2.5-coder:14b` | Code |
| **Qwen 2.5 Coder 7B** | 7B | 128K | 4GB | ✅ Apache-2 | `qwen2.5-coder:7b` | Code |
| **Command R 35B** | 35B | 128K | 20GB | ✅ CC-BY-NC | `command-r:35b` | Tools, RAG |
| **Granite 3.3 8B** | 8B | 128K | 4GB | ✅ Apache-2 | `granite3.3:8b` | Enterprise |
| **Pixtral 12B** | 12B | 128K | 8GB | ✅ Apache-2 | `pixtral:12b` | Vision |

### 🥉 Tier-3: Hafif / Düşük VRAM (4GB Altı)

| Model | Parametre | Context | VRAM | Açık Kaynak | Ollama ID | Yetenek |
|-------|-----------|---------|------|-------------|-----------|---------|
| **Llama 3.2 3B** | 3B | 128K | 2GB | ✅ Llama | `llama3.2:3b` | Genel |
| **Llama 3.2 1B** | 1B | 128K | 1GB | ✅ Llama | `llama3.2:1b` | En küçük |
| **Mistral 7B** | 7B | 32K | 4GB | ✅ Apache-2 | `mistral:7b` | Genel |
| **Llama 3.1 8B** | 8B | 128K | 4GB | ✅ Llama | `llama3.1:8b` | Genel |
| **DeepSeek R1 Distill 8B** | 8B | 128K | 4GB | ✅ MIT | `deepseek-r1:8b` | Reasoning |
| **DeepSeek R1 Distill 14B** | 14B | 128K | 8GB | ✅ MIT | `deepseek-r1:14b` | Reasoning |
| **StarCoder2 15B** | 15B | 16K | 8GB | ✅ BigCode | `starcoder2:15b` | Code |
| **InternLM 3 8B** | 8B | 32K | 4GB | ✅ Apache-2 | `internlm3:8b` | Chinese/English |
| **Yi Coder 9B** | 9B | 128K | 4GB | ✅ Apache-2 | `yi-coder:9b` | Code |
| **DBRX 132B MoE** | 132B (36B aktif) | 32K | 24GB | ✅ Databricks | `dbrx:132b` | MoE |

---

## 🚀 Hızlı Başlangıç: Ollama ile Lokal Çalıştırma

### 1. Ollama Kurulumu
```bash
# Linux/Mac
curl -fsSL https://ollama.ai/install.sh | sh

# Docker
docker run -d -v ollama:/root/.ollama -p 11434:11434 ollama/ollama
```

### 2. Model İndirme ve Çalıştırma
```bash
# En hafif reasoning model (4GB VRAM)
ollama pull qwen3:30b-a3b

# En iyi kod modeli (8GB VRAM) 
ollama pull qwen2.5-coder:14b

# En güncel MoE (24GB VRAM)
ollama pull qwen3:235b

# Vision destekli (8GB VRAM)
ollama pull pixtral:12b

# En yeni Llama 4 Scout (48GB VRAM)
ollama pull llama4:scout
```

### 3. SENTIENT OS ile Entegrasyon
```rust
use sentient_llm::{LlmHub, ChatRequest, Message};

#[tokio::main]
async fn main() {
    // Ollama otomatik algılanır (localhost:11434)
    let hub = LlmHub::from_env().unwrap();
    
    // Lokal Qwen3 ile reasoning
    let response = hub.chat(ChatRequest {
        model: "qwen3:30b-a3b".into(),
        messages: vec![Message::user("Rust ile fibonacci hesapla")],
        ..Default::default()
    }).await.unwrap();
    
    println!("{}", response.choices[0].message.content.as_text().unwrap());
}
```

---

## 💡 VRAM Rehberi: Hangi Ekran Kartı Hangi Modeli Çalıştırır?

| VRAM | Önerilen Modeller |
|------|-------------------|
| **2-4GB** | Llama 3.2 1B/3B, Qwen3 30B-A3B MoE, Phi-4 Mini |
| **4-8GB** | Llama 3.1 8B, Mistral 7B, Gemma 3 4B, Phi-4 14B, Qwen 2.5 Coder 7B |
| **8-16GB** | Gemma 3 12B/27B, Qwen 2.5 Coder 14B, DeepSeek R1 Distill 14B, Pixtral 12B |
| **16-24GB** | Qwen3 32B, Mistral Small 3.1, Command R 35B, Codestral 22B |
| **24-48GB** | Llama 3.3 70B, DeepSeek R1 70B, Qwen3 235B MoE, DBRX 132B |
| **48-96GB** | Llama 4 Scout, Llama 4 Maverick, DeepSeek V4/R2 |

---

## 🔓 Tamamen Ücretsiz Çalıştırma Yöntemleri

### Yöntem 1: Ollama (En Kolay)
```bash
ollama serve          # Sunucuyu başlat
ollama run llama3.1:8b # Modeli çalıştır
```

### Yöntem 2: vLLM (Yüksek Performans)
```bash
pip install vllm
python -m vllm.entrypoints.openai.api_server --model meta-llama/Llama-3.3-70B-Instruct
```

### Yöntem 3: LM Studio (GUI)
- [lmstudio.ai](https://lmstudio.ai) indir
- Modeli GUI'den seç, tek tıkla çalıştır
- OpenAI uyumlu API otomatik localhost:1234'te

### Yöntem 4: Llamafile (Tek Dosya)
```bash
# Tek dosya indir, çalıştır - başka hiçbir şey gerekmez
wget https://huggingface.co/Mozilla/llamafile/...
chmod +x model.llamafile
./model.llamafile
```

### Yöntem 5: Chutes (Bulutta Ücretsiz!)
```bash
# API key gerekmez, sınırsız ücretsiz inference
curl https://llm.chutes.ai/v1/chat/completions \
  -H "Authorization: Bearer $CHUTES_API_KEY" \
  -d '{"model":"chutesai/Llama-4-Maverick-17B-128E-Instruct","messages":[{"role":"user","content":"Merhaba"}]}'
```

---

## 🇹🇷 Türkçe Desteği Güçlü Modeller

| Model | Türkçe Performans | Açık Kaynak |
|-------|-------------------|-------------|
| **Qwen3 235B** | ⭐⭐⭐⭐⭐ | ✅ |
| **Cevahir V-7** (SENTIENT) | ⭐⭐⭐⭐⭐ (Yerli!) | ✅ |
| **DeepSeek V4** | ⭐⭐⭐⭐ | ✅ |
| **Command R** | ⭐⭐⭐⭐ (Multilingual) | ✅ |
| **Aya Exa 32B** | ⭐⭐⭐⭐⭐ (101 dil) | ✅ |
| **Grok 3** | ⭐⭐⭐⭐ | ❌ |

---

*SENTIENT OS - The Operating System That Thinks*
