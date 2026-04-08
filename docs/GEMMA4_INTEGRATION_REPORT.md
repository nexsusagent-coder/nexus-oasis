# 🧠 GEMMA 4 - SENTIENT OS ENTEGRASYON RAPORU

## 📅 Tarih: 2026-04-08
## 🎯 Amaç: Google Gemma 4 modellerinin SENTIENT OS'e kalıcı "öz parça" olarak entegrasyonu

---

## 1. GEMMA 4 MODEL AİLESİ

### 1.1 Mevcut Modeller

| Model | Parametreler | Aktif Param | Context | Multimodal | Durum |
|-------|-------------|-------------|---------|------------|-------|
| **Gemma 4 E2B** | 5.1B (2.3B effective) | 2.3B | 128K | Text + Image + Audio | ✅ Hafif |
| **Gemma 4 E4B** | 8B (4.5B effective) | 4.5B | 128K | Text + Image + Audio | ✅ Dengeli |
| **Gemma 4 26B A4B** | 25.2B (MoE) | 3.8B | 256K | Text + Image | ✅ Hızlı MoE |
| **Gemma 4 31B** | 30.7B | 30.7B | 256K | Text + Image | ✅ En güçlü |

### 1.2 Benchmark Performansı

| Test | Gemma 4 31B | Gemma 4 26B A4B | Gemma 4 E4B | Gemma 4 E2B |
|------|-------------|-----------------|-------------|-------------|
| MMLU Pro | 85.2% | 82.6% | 69.4% | 60.0% |
| AIME 2026 | 89.2% | 88.3% | 42.5% | 37.5% |
| LiveCodeBench | 80.0% | 77.1% | 52.0% | 44.0% |
| Codeforces ELO | 2150 | 1718 | 940 | 633 |
| GPQA Diamond | 84.3% | 82.3% | 58.6% | 43.4% |

---

## 2. TEMEL ÖZELLİKLER

### 2.1 Multimodal Destek
- ✅ **Text**: 140+ dil desteği
- ✅ **Image**: Değişken aspect ratio ve çözünürlük
- ✅ **Video**: Frame sequence analizi
- ✅ **Audio**: E2B/E4B modellerinde ASR + çeviri

### 2.2 Gelişmiş Özellikler
- ✅ **Thinking Mode**: Adım adım düşünme
- ✅ **Long Context**: 128K-256K token
- ✅ **Function Calling**: Native tool use
- ✅ **System Prompt**: Native system role desteği
- ✅ **Sliding Window Attention**: Hızlı inference

### 2.3 Mimari Yenilikler
- **Per-Layer Embeddings (PLE)**: E2B/E4B'de verimli parametre kullanımı
- **Mixture-of-Experts (MoE)**: 26B A4B'de 3.8B aktif parametre
- **Hybrid Attention**: Sliding + Global attention kombinasyonu
- **Proportional RoPE (p-RoPE)**: Long context optimizasyonu

---

## 3. ERİŞİM KANALLARI

### 3.1 OpenRouter (ÖNERİLEN)

```json
{
  "google/gemma-4-31b-it:free": {
    "context_length": 262144,
    "pricing": { "prompt": "0", "completion": "0" }
  },
  "google/gemma-4-26b-a4b-it:free": {
    "context_length": 262144,
    "pricing": { "prompt": "0", "completion": "0" }
  }
}
```

**Avantajlar:**
- 🆓 **ÜCRETSİZ** tier mevcut
- 256K context length
- V-GATE proxy ile güvenli erişim
- Mevcut OpenRouterProvider ile uyumlu

### 3.2 HuggingFace Transformers

```python
from transformers import AutoProcessor, AutoModelForCausalLM

MODEL_ID = "google/gemma-4-31B-it"

processor = AutoProcessor.from_pretrained(MODEL_ID)
model = AutoModelForCausalLM.from_pretrained(
    MODEL_ID,
    dtype="auto",
    device_map="auto"
)
```

### 3.3 Ollama (Yerel)

```bash
# Gemma 3 mevcut (Gemma 4 yakında)
ollama pull gemma3:27b
ollama run gemma3:27b
```

---

## 4. SENTIENT OS ENTEGRASYON PLANI

### 4.1 AŞAMA 1: OpenRouter Üzerinden Hızlı Entegrasyon

**Yer:** `crates/sentient_vgate/src/providers/`

**Değişiklik:** Mevcut `OpenRouterProvider` zaten uyumlu. Sadece model listesine eklemek yeterli:

```rust
// sentient_vgate/src/providers/models.rs (YENİ DOSYA)

pub const GEMMA4_MODELS: &[(&str, &str, u64)] = &[
    // (model_id, display_name, context_length)
    ("google/gemma-4-31b-it:free", "Gemma 4 31B (Free)", 262144),
    ("google/gemma-4-31b-it", "Gemma 4 31B", 262144),
    ("google/gemma-4-26b-a4b-it:free", "Gemma 4 26B MoE (Free)", 262144),
    ("google/gemma-4-26b-a4b-it", "Gemma 4 26B MoE", 262144),
    ("google/gemma-4-e4b-it", "Gemma 4 E4B (Edge)", 131072),
    ("google/gemma-4-e2b-it", "Gemma 4 E2B (Mobile)", 131072),
];
```

### 4.2 AŞAMA 2: Native Gemma 4 Provider

**Yer:** `crates/sentient_vgate/src/providers/gemma4.rs` (YENİ)

```rust
//! ─── GEMMA 4 NATIVE SAĞLAYICISI ───
//! 
//! Gemma 4'e özel optimizasyonlar:
//! - Thinking mode desteği
//! - Multimodal input (text, image, audio)
//! - Function calling native tokens
//! - Long context optimization

use crate::providers::base::*;
use async_trait::async_trait;

pub struct Gemma4Provider {
    base_url: String,
    api_key: String,
    client: reqwest::Client,
    thinking_mode: bool,
}

impl Gemma4Provider {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: "https://openrouter.ai/api/v1".into(),
            api_key,
            client: reqwest::Client::new(),
            thinking_mode: false,
        }
    }
    
    /// Thinking mode aktifleştir
    pub fn with_thinking(mut self, enabled: bool) -> Self {
        self.thinking_mode = enabled;
        self
    }
    
    /// Multimodal mesaj oluştur
    pub fn create_multimodal_message(
        &self,
        text: &str,
        images: Option<Vec<&[u8]>>,
        audio: Option<&[u8]>,
    ) -> serde_json::Value {
        let mut content = vec![];
        
        if let Some(imgs) = images {
            for img in imgs {
                let base64 = base64::encode(img);
                content.push(serde_json::json!({
                    "type": "image",
                    "image_url": { "url": format!("data:image/jpeg;base64,{}", base64) }
                }));
            }
        }
        
        if let Some(aud) = audio {
            let base64 = base64::encode(aud);
            content.push(serde_json::json!({
                "type": "audio",
                "audio_url": { "url": format!("data:audio/wav;base64,{}", base64) }
            }));
        }
        
        content.push(serde_json::json!({
            "type": "text",
            "text": text
        }));
        
        serde_json::json!({ "role": "user", "content": content })
    }
}

#[async_trait]
impl LlmProvider for Gemma4Provider {
    async fn chat_completion(&self, request: LlmRequest) -> SENTIENTResult<LlmResponse> {
        // Gemma 4'e özel istek gövdesi
        let mut body = serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
        });
        
        // Thinking mode ekle
        if self.thinking_mode {
            body["thinking"] = serde_json::json!({
                "type": "enabled",
                "budget_tokens": 4096
            });
        }
        
        // ... implementation
        todo!()
    }
    
    async fn list_models(&self) -> SENTIENTResult<Vec<ModelInfo>> {
        Ok(GEMMA4_MODELS.iter().map(|(id, name, ctx)| ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: "gemma4".into(),
            context_length: Some(*ctx),
            pricing: None,
        }).collect())
    }
    
    fn name(&self) -> &str {
        "gemma4"
    }
}
```

### 4.3 AŞAMA 3: Thinking Mode Agent Entegrasyonu

**Yer:** `crates/sentient_orchestrator/src/agents/reasoning.rs` (YENİ)

```rust
//! ─── GEMMA 4 THINKING MODE ENTEGRASYONU ───
//!
//! Gemma 4'ün native thinking mode'unu SENTIENT agent sistemine entegre eder.

pub struct Gemma4ReasoningEngine {
    provider: Gemma4Provider,
    thinking_budget: u32,
}

impl Gemma4ReasoningEngine {
    /// Chain-of-thought ile görev çözümle
    pub async fn solve_with_thinking(
        &self,
        task: &str,
        context: &[String],
    ) -> SENTIENTResult<ReasoningResult> {
        let prompt = format!(
            "<thinking>\n\
            Görev: {}\n\
            Bağlam: {}\n\
            Adım adım düşün ve çözüm üret.\n\
            </thinking>",
            task,
            context.join("\n")
        );
        
        // ... implementation
    }
}
```

### 4.4 AŞAMA 4: Function Calling Native Desteği

**Yer:** `crates/sentient_orchestrator/src/tools/gemma4_tools.rs` (YENİ)

```rust
//! ─── GEMMA 4 NATIVE FUNCTION CALLING ───
//!
//! Gemma 4'ün native <|tool_call|> token'larını kullanarak
//! SENTIENT araçlarını çağırma.

pub struct Gemma4ToolCaller {
    tools: Vec<ToolDefinition>,
}

#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

impl Gemma4ToolCaller {
    /// Tool çağrısını parse et
    pub fn parse_tool_call(&self, response: &str) -> Option<ToolCallRequest> {
        // <|tool_call>call:function_name{...args}<tool_call|>
        let re = regex::Regex::new(
            r"<\|tool_call>call:(\w+)(\{.*?\})<tool_call\|>"
        ).ok()?;
        
        re.captures(response).map(|caps| {
            ToolCallRequest {
                name: caps[1].to_string(),
                arguments: serde_json::from_str(&caps[2]).unwrap_or_default(),
            }
        })
    }
}
```

---

## 5. ÖNERİLEN ENTEGRASYON YOL HARİTASI

### 🟢 ÖNCELİK 1: Hızlı Entegrasyon (1-2 Gün)

| İşlem | Dosya | Durum |
|-------|-------|-------|
| Model listesine ekleme | `providers/models.rs` | Yeni |
| Varsayılan model olarak ayarla | `config/default.rs` | Güncelleme |
| CLI'da model seçimi | `sentient_cli/src/repl/` | Güncelleme |

### 🟡 ÖNCELİK 2: Native Provider (3-5 Gün)

| İşlem | Dosya | Durum |
|-------|-------|-------|
| Gemma4Provider oluştur | `providers/gemma4.rs` | Yeni |
| Multimodal destek | `providers/gemma4.rs` | Yeni |
| Thinking mode | `providers/gemma4.rs` | Yeni |
| Provider factory güncelle | `providers/mod.rs` | Güncelleme |

### 🔴 ÖNCELİK 3: Derin Entegrasyon (1-2 Hafta)

| İşlem | Dosya | Durum |
|-------|-------|-------|
| Reasoning engine | `orchestrator/reasoning.rs` | Yeni |
| Native function calling | `orchestrator/tools/` | Yeni |
| Vision entegrasyonu | `oasis_hands/vision/` | Güncelleme |
| Audio entegrasyonu | `oasis_hands/audio/` | Yeni |

---

## 6. GÜVENLİK VE LİSANS

### 6.1 Lisans
- **Apache 2.0** - Tamamen açık kaynak
- Ticari kullanım serbest
- Değiştirme ve dağıtma özgürlüğü

### 6.2 V-GATE Entegrasyonu
```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │  OpenRouter │
│   Client    │─────▶│   Proxy     │─────▶│   Gemma 4   │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Güvenli)
                     Server-side
```

**Avantajlar:**
- ✅ API anahtarı asla istemcide
- ✅ Guardrails ile istek denetimi
- ✅ Rate limiting
- ✅ Audit log

---

## 7. SONUÇ VE TAVSİYELER

### ✅ ENTEGRASYON ÖNERİLEN

**Neden Gemma 4?**
1. **Ücretsiz Tier**: OpenRouter'da ücretsiz erişim
2. **Yerel Çalışma**: E2B/E4B mobil/laptop'ta çalışabilir
3. **Multimodal**: Text + Image + Audio desteği
4. **Long Context**: 256K token context
5. **Native Function Calling**: Agent sistemleri için ideal
6. **Thinking Mode**: Reasoning görevleri için optimize
7. **Apache 2.0**: Tamamen açık kaynak

### 📋 HEMEN BAŞLANACAK İŞLER

1. **Model listesine Gemma 4 ekle** → `sentient_vgate/src/providers/models.rs`
2. **Varsayılan model olarak ayarla** → `google/gemma-4-26b-a4b-it:free`
3. **CLI'da model seçimi göster** → `sentient-shell`

---

**Raporu Hazırlayan:** SENTIENT Core  
**Tarih:** 2026-04-08  
**Durum:** Entegrasyon için onay bekleniyor
