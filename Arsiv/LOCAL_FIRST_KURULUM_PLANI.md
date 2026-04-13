# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - LOCAL-FIRST KURULUM PLANI
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 13 Nisan 2026
#  Hedef: OpenClaw ve rakiplere karşı %100 Yerel, API Key'siz sistem
#  Durum: Planlama Aşaması
# ═══════════════════════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 1: MEVCUT DURUM ANALİZİ
# ═══════════════════════════════════════════════════════════════════════════════

## 1.1 Sistem İstatistikleri

| Metrik | Değer |
|--------|-------|
| Toplam Rust Dosyası | 3,103 adet |
| Toplam Kod Satırı | ~365,520 satır |
| Toplam Crate | 71 adet |
| Katman Sayısı | 17 adet |
| Derleme Durumu | ✅ 0 Error, 0 Warning |

## 1.2 API Key Gerektiren Bileşenler (KULLANILMAYACAK)

| Bileşen | Provider | Alternatif |
|---------|----------|------------|
| LLM | OpenAI, Anthropic, Google | Ollama (Yerel) |
| Embedding | OpenAI Embeddings | Local Embeddings |
| TTS/STT | OpenAI Whisper API | Local Whisper |
| Web Search | Tavily, SerpAPI | DuckDuckGo, SearXNG |
| Code Execution | E2B Cloud | Docker Sandbox |

## 1.3 Zaten Local-First Olan Bileşenler

| Bileşen | Teknoloji | Durum |
|---------|-----------|-------|
| Database | PostgreSQL | ✅ Docker'da |
| Cache | Redis | ✅ Docker'da |
| Vector DB | Qdrant | ✅ Docker'da |
| Object Storage | MinIO | ✅ Docker'da |
| Monitoring | Prometheus + Grafana | ✅ Docker'da |

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 2: KURULUM ADIMLARI (ÖNCELİK SIRASINA GÖRE)
# ═══════════════════════════════════════════════════════════════════════════════

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 1: TEMEL ALTYAPI (1-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 1.1 Docker Servislerini Başlat

**Dosya:** `docker-compose.yml`

```bash
# Servisleri başlat
docker-compose up -d postgres redis qdrant minio prometheus grafana

# Durum kontrolü
docker-compose ps
```

**Gerekli Portlar:**
| Servis | Port | URL |
|--------|------|-----|
| PostgreSQL | 5432 | localhost:5432 |
| Redis | 6379 | localhost:6379 |
| Qdrant | 6333 | http://localhost:6333 |
| MinIO | 9000/9001 | http://localhost:9000 |
| Prometheus | 9090 | http://localhost:9090 |
| Grafana | 3000 | http://localhost:3000 |

---

### ✅ 1.2 Ollama Kurulumu (LLM Kernel)

**Kurulum:**
```bash
# Linux/macOS
curl -fsSL https://ollama.com/install.sh | sh

# Servis başlat
ollama serve

# Modelleri indir (seçenekler)
ollama pull gemma3:27b          # 27B parametre, 256K context
ollama pull llama3.3:70b        # 70B parametre, 128K context
ollama pull qwen2.5:72b         # 72B parametre, 128K context
ollama pull deepseek-r1:67b     # 67B parametre, 164K context, reasoning
ollama pull mistral:24b         # 24B parametre, 128K context
ollama pull phi-4:14b           # 14B parametre, hızlı

# Vision destekli modeller
ollama pull llava:13b           # Vision + Language
ollama pull llama3.2-vision:11b # Vision destekli
```

**Donanım Gereksinimleri:**
| VRAM | Önerilen Model | Boyut |
|------|----------------|-------|
| 8GB | phi-4:14b, gemma3:12b | ~8GB |
| 16GB | mistral:24b, gemma3:27b | ~16GB |
| 24GB | gemma3:27b (quantized) | ~20GB |
| 48GB+ | llama3.3:70b, qwen2.5:72b | ~40GB |

**Test:**
```bash
# API testi
curl http://localhost:11434/api/generate -d '{
  "model": "gemma3:27b",
  "prompt": "Merhaba, nasıl yardımcı olabilirim?"
}'
```

---

### ✅ 1.3 Database Migration

**Dosya:** `scripts/init.sql` (yoksa oluştur)

```bash
# PostgreSQL'e bağlan
docker exec -it sentient-postgres psql -U sentient -d sentient

# Tabloları oluştur
\i /docker-entrypoint-initdb.d/init.sql
```

**Gerekli Tablolar:**
```sql
-- Kullanıcılar
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Oturumlar
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    title VARCHAR(500),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Mesajlar
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID REFERENCES sessions(id),
    role VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Skill'ler
CREATE TABLE IF NOT EXISTS skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    config JSONB,
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Plugin'ler
CREATE TABLE IF NOT EXISTS plugins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) UNIQUE NOT NULL,
    version VARCHAR(50),
    path TEXT,
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 2: LLM ENTEGRASYONU (2-4 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 2.1 Ollama Provider'ı Aktif Et

**Dosya:** `crates/sentient_llm/src/providers/ollama.rs`

Mevcut dosyayı kontrol et ve gerekirse düzelt:

```rust
// sentient_llm/src/providers/ollama.rs

use crate::{LlmError, LlmProvider, LlmResult};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OLLAMA_HOST: &str = "http://localhost:11434";

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    client: Client,
    host: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            host: OLLAMA_HOST.to_string(),
            model: model.into(),
        }
    }
    
    pub fn with_host(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            host: host.into(),
            model: model.into(),
        }
    }
    
    pub async fn list_models(&self) -> LlmResult<Vec<String>> {
        let url = format!("{}/api/tags", self.host);
        let response = self.client.get(&url).send().await?;
        let data: TagsResponse = response.json().await?;
        Ok(data.models.into_iter().map(|m| m.name).collect())
    }
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn complete(&self, prompt: &str) -> LlmResult<String> {
        let url = format!("{}/api/generate", self.host);
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };
        
        let response = self.client.post(&url)
            .json(&request)
            .send().await?;
        
        let data: GenerateResponse = response.json().await?;
        Ok(data.response)
    }
    
    async fn chat(&self, messages: Vec<crate::Message>) -> LlmResult<String> {
        let url = format!("{}/api/chat", self.host);
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.into_iter().map(|m| OllamaMessage {
                role: m.role.to_string(),
                content: m.content,
            }).collect(),
            stream: false,
        };
        
        let response = self.client.post(&url)
            .json(&request)
            .send().await?;
        
        let data: ChatResponse = response.json().await?;
        Ok(data.message.content)
    }
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: OllamaMessage,
}
```

---

### ✅ 2.2 Default Provider Olarak Ollama Ayarla

**Dosya:** `config/default.toml`

```toml
[llm]
provider = "ollama"
model = "gemma3:27b"
host = "http://localhost:11434"

[llm.options]
temperature = 0.7
top_p = 0.9
top_k = 40
num_ctx = 32768
num_predict = 4096

[llm.fallback]
enabled = true
provider = "ollama"
model = "phi-4:14b"

[embedding]
provider = "local"
model = "all-MiniLM-L6-v2"
dimension = 384

[vector_db]
provider = "qdrant"
host = "http://localhost:6333"
collection = "sentient_memory"

[database]
provider = "postgres"
url = "postgres://sentient:sentient@localhost:5432/sentient"

[cache]
provider = "redis"
url = "redis://localhost:6379"

[storage]
provider = "minio"
endpoint = "http://localhost:9000"
access_key = "minioadmin"
secret_key = "minioadmin"
bucket = "sentient"
```

---

### ✅ 2.3 Cevahir AI Entegrasyonu (Yerel LLM Engine)

**Mevcut Dosya:** `crates/sentient_cevahir/`

Cevahir AI, Türkçe optimize edilmiş yerel LLM motorudur.

**Özellikler:**
- RoPE (Rotary Position Embedding)
- RMSNorm
- SwiGLU
- KV Cache
- MoE (Mixture of Experts)
- GQA (Grouped Query Attention)
- Turkish BPE Tokenizer

**Aktivasyon:**
```bash
# Model dosyasını indir (.gguf format)
mkdir -p models
# GGUF model dosyasını buraya koy

# Config'de ayarla
[cevahir]
enabled = true
model_path = "models/cevahir-7b.gguf"
context_size = 8192
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 3: EMBEDDING & VECTOR DB (2-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 3.1 Local Embedding Modeli

**Kullanılacak Modeller:**

| Model | Boyut | Hız | Kalite |
|-------|-------|-----|--------|
| all-MiniLM-L6-v2 | 384d | ⚡⚡⚡ | ⭐⭐ |
| all-mpnet-base-v2 | 768d | ⚡⚡ | ⭐⭐⭐ |
| bge-large-en-v1.5 | 1024d | ⚡ | ⭐⭐⭐⭐ |
| e5-large-v2 | 1024d | ⚡ | ⭐⭐⭐⭐ |

**Rust Implementation (candle-transformers):**

**Dosya:** `crates/sentient_embedding/src/local.rs`

```rust
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::Tokenizer;

pub struct LocalEmbedding {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl LocalEmbedding {
    pub fn new(model_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let device = Device::Cpu; // veya Device::Cuda(0)
        let config = Config::from_file(&format!("{}/config.json", model_path));
        let weights = candle_core::VarBuilder::from_pth(
            &format!("{}/model.safetensors", model_path),
            candle_core::DType::F32,
            &device,
        )?;
        let model = BertModel::load(vb, &config)?;
        let tokenizer = Tokenizer::from_file(&format!("{}/tokenizer.json", model_path))?;
        
        Ok(Self { model, tokenizer, device })
    }
    
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let tokens = self.tokenizer.encode(text, true)?;
        let input_ids = Tensor::new(tokens.get_ids(), &self.device)?;
        let embeddings = self.model.forward(&input_ids)?;
        // Mean pooling
        let pooled = embeddings.mean(1)?;
        Ok(pooled.to_vec1()?)
    }
}
```

**Kurulum:**
```bash
# Model dosyalarını indir
mkdir -p models/embedding
cd models/embedding

# HuggingFace'den indir (all-MiniLM-L6-v2)
wget https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/config.json
wget https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/model.safetensors
wget https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/tokenizer.json
wget https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/vocab.txt
```

---

### ✅ 3.2 Qdrant Vector DB Yapılandırması

**Qdrant Dashboard:** http://localhost:6333/dashboard

**Collection Oluşturma:**
```bash
curl -X PUT http://localhost:6333/collections/sentient_memory \
  -H 'Content-Type: application/json' \
  -d '{
    "vectors": {
      "size": 384,
      "distance": "Cosine"
    }
  }'
```

**Rust Client:**
```rust
use qdrant_client::prelude::*;

pub async fn setup_qdrant() -> Result<(), Box<dyn std::error::Error>> {
    let config = QdrantClientConfig::from_url("http://localhost:6333");
    let client = QdrantClient::new(Some(config))?;
    
    // Collection oluştur
    client.create_collection(&CreateCollection {
        collection_name: "sentient_memory".into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 384,
                distance: Distance::Cosine as i32,
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;
    
    Ok(())
}
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 4: SES İŞLEME (STT/TTS) (2-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 4.1 Whisper STT (Speech-to-Text)

**Kurulum:**
```bash
# Whisper.cpp (hızlı, CPU)
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make

# Model indir
bash ./models/download-ggml-model.sh medium

# Test
./main -m models/ggml-medium.bin -f test.wav
```

**Rust Binding:**
```rust
// crates/sentient_voice/src/whisper_local.rs

use whisper_rs::{FullParams, WhisperContext, WhisperContextParameters};

pub struct LocalWhisper {
    ctx: WhisperContext,
}

impl LocalWhisper {
    pub fn new(model_path: &str) -> Result<Self, WhisperError> {
        let ctx_params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(model_path, ctx_params)?;
        Ok(Self { ctx })
    }
    
    pub fn transcribe(&self, audio_path: &str) -> Result<String, WhisperError> {
        let mut params = FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        params.set_translate(false);
        params.set_language(Some("tr"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        
        let state = self.ctx.create_state()?;
        state.full_parallel(params, audio_path, 1)?;
        
        let num_segments = state.full_n_segments()?;
        let mut result = String::new();
        
        for i in 0..num_segments {
            let text = state.full_get_segment_text(i)?;
            result.push_str(&text);
            result.push(' ');
        }
        
        Ok(result.trim().to_string())
    }
}
```

**Model Boyutları:**
| Model | Boyut | RAM | Hız |
|-------|-------|-----|-----|
| tiny | 75MB | ~1GB | ⚡⚡⚡ |
| base | 150MB | ~1GB | ⚡⚡⚡ |
| small | 500MB | ~2GB | ⚡⚡ |
| medium | 1.5GB | ~5GB | ⚡ |
| large | 3GB | ~10GB | 🐢 |

---

### ✅ 4.2 Piper TTS (Text-to-Speech)

**Kurulum:**
```bash
# Piper indir
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xzf piper_1.2.0_amd64.tar.gz

# Türkçe model indir
mkdir -p piper/models
cd piper/models
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr-female-low.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr-female-low.onnx.json

# Test
./piper/piper --model piper/models/tr-female-low.onnx --output_file test.wav < text.txt
```

**Rust Integration:**
```rust
// crates/sentient_voice/src/piper.rs

use std::process::Command;

pub struct PiperTTS {
    piper_path: String,
    model_path: String,
}

impl PiperTTS {
    pub fn new(piper_path: &str, model_path: &str) -> Self {
        Self {
            piper_path: piper_path.to_string(),
            model_path: model_path.to_string(),
        }
    }
    
    pub fn synthesize(&self, text: &str, output_path: &str) -> Result<(), std::io::Error> {
        let mut child = Command::new(&self.piper_path)
            .arg("--model")
            .arg(&self.model_path)
            .arg("--output_file")
            .arg(output_path)
            .stdin(std::process::Stdio::piped())
            .spawn()?;
        
        use std::io::Write;
        child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
        child.wait()?;
        
        Ok(())
    }
}
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 5: WEB SEARCH (API Key'siz) (1-2 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 5.1 DuckDuckGo Search (Ücretsiz, No API Key)

**Dosya:** `crates/sentient_search/src/duckduckgo.rs`

```rust
use reqwest::Client;
use scraper::{Html, Selector};

pub struct DuckDuckGoSearch {
    client: Client,
}

impl DuckDuckGoSearch {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
                .build()
                .unwrap(),
        }
    }
    
    pub async fn search(&self, query: &str, max_results: usize) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(query));
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        
        let document = Html::parse_document(&html);
        let result_selector = Selector::parse(".result").unwrap();
        let title_selector = Selector::parse(".result__a").unwrap();
        let snippet_selector = Selector::parse(".result__snippet").unwrap();
        let link_selector = Selector::parse(".result__url").unwrap();
        
        let mut results = Vec::new();
        
        for result in document.select(&result_selector).take(max_results) {
            let title = result.select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            
            let snippet = result.select(&snippet_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            
            let link = result.select(&link_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            
            results.push(SearchResult {
                title: title.trim().to_string(),
                snippet: snippet.trim().to_string(),
                link: link.trim().to_string(),
            });
        }
        
        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub snippet: String,
    pub link: String,
}
```

---

### ✅ 5.2 SearXNG (Self-Hosted Meta Search)

**Docker Compose'a Ekle:**
```yaml
  # SearXNG - Self-hosted search engine
  searxng:
    image: searxng/searxng:latest
    container_name: sentient-searxng
    ports:
      - "8888:8080"
    environment:
      - BASE_URL=http://localhost:8888/
      - INSTANCE_NAME=SENTIENT Search
    volumes:
      - searxng-data:/etc/searxng
    networks:
      - sentient-network
```

**Rust Client:**
```rust
pub struct SearXNG {
    base_url: String,
    client: Client,
}

impl SearXNG {
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, Error> {
        let url = format!("{}/search?format=json&q={}", self.base_url, query);
        let response = self.client.get(&url).send().await?;
        let data: SearXNGResponse = response.json().await?;
        
        Ok(data.results.into_iter().map(|r| SearchResult {
            title: r.title,
            snippet: r.content,
            link: r.url,
        }).collect())
    }
}
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 6: CODE EXECUTION (Docker Sandbox) (2-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 6.1 Docker-based Code Execution

**Dosya:** `crates/sentient_sandbox/src/docker.rs`

```rust
use bollard::Docker;
use bollard::container::{CreateContainerOptions, StartContainerOptions, WaitContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::exec::{CreateExecOptions, StartExecResults};

pub struct DockerSandbox {
    docker: Docker,
    timeout: u64,
}

impl DockerSandbox {
    pub async fn new() -> Result<Self, Error> {
        let docker = Docker::connect_with_socket_defaults()?;
        Ok(Self { docker, timeout: 30 })
    }
    
    pub async fn execute(&self, code: &str, language: Language) -> Result<ExecutionResult, Error> {
        let image = language.docker_image();
        let command = language.execute_command(code);
        
        // Container oluştur
        let container = self.docker.create_container(
            Some(CreateContainerOptions { name: "" }),
            ContainerConfig {
                image: Some(image),
                cmd: Some(command),
                env: Some(vec!["CODE=".to_string() + code]),
                ..Default::default()
            },
        ).await?;
        
        // Başlat
        self.docker.start_container(&container.id, None).await?;
        
        // Bekle
        let wait = self.docker.wait_container(&container.id, None).await?;
        
        // Sonucu al
        let logs = self.docker.logs(&container.id, Some(LogsOptions {
            stdout: true,
            stderr: true,
            ..Default::default()
        })).await?;
        
        // Temizle
        self.docker.remove_container(&container.id, None).await?;
        
        Ok(ExecutionResult {
            output: logs,
            exit_code: wait.status_code,
        })
    }
}

pub enum Language {
    Python,
    JavaScript,
    Rust,
    Go,
    Bash,
}

impl Language {
    fn docker_image(&self) -> &'static str {
        match self {
            Self::Python => "python:3.11-slim",
            Self::JavaScript => "node:20-slim",
            Self::Rust => "rust:1.75-slim",
            Self::Go => "golang:1.21-alpine",
            Self::Bash => "bash:5",
        }
    }
}
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 7: MULTIMODAL (Vision) (2-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 7.1 LLaVA (Vision-Language Model)

**Ollama ile:**
```bash
# LLaVA modeli indir
ollama pull llava:13b

# Test
ollama run llava:13b
>>> [upload image] Bu resimde ne var?
```

**Rust Integration:**
```rust
pub struct LlavaVision {
    ollama: OllamaProvider,
}

impl LlavaVision {
    pub async fn analyze_image(&self, image_path: &str, prompt: &str) -> Result<String, Error> {
        let image_data = std::fs::read(image_path)?;
        let image_base64 = base64::encode(&image_data);
        
        let request = VisionRequest {
            model: "llava:13b".into(),
            prompt: prompt.into(),
            images: vec![image_base64],
        };
        
        self.ollama.vision_request(request).await
    }
}
```

---

## ═══════════════════════════════════════════════════════════════════════════════
##  FAZ 8: MEMORY & RAG (2-3 Gün)
## ═══════════════════════════════════════════════════════════════════════════════

### ✅ 8.1 RAG Pipeline

```rust
pub struct LocalRAG {
    embedding: LocalEmbedding,
    qdrant: QdrantClient,
    llm: OllamaProvider,
}

impl LocalRAG {
    pub async fn index_document(&self, doc: &Document) -> Result<(), Error> {
        // Chunk'lara böl
        let chunks = self.chunk_text(&doc.content, 500, 50);
        
        for (i, chunk) in chunks.iter().enumerate() {
            // Embed
            let embedding = self.embedding.embed(chunk)?;
            
            // Vector DB'ye kaydet
            self.qdrant.upsert_points(UpsertPoints {
                collection_name: "documents".into(),
                points: vec![PointStruct {
                    id: Some(Uuid::new_v4().to_string()),
                    vector: Some(embedding.into()),
                    payload: Some(json!({
                        "content": chunk,
                        "doc_id": doc.id,
                        "chunk_index": i,
                    })),
                }],
            }).await?;
        }
        
        Ok(())
    }
    
    pub async fn query(&self, question: &str, top_k: usize) -> Result<String, Error> {
        // Soruyu embed et
        let query_embedding = self.embedding.embed(question)?;
        
        // En yakın chunk'ları bul
        let results = self.qdrant.search_points(SearchPoints {
            collection_name: "documents".into(),
            vector: query_embedding,
            limit: top_k as u64,
            ..Default::default()
        }).await?;
        
        // Context oluştur
        let context = results.iter()
            .map(|r| r.payload["content"].as_str().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        // LLM'e sor
        let prompt = format!(
            "Aşağıdaki bilgiyi kullanarak soruyu cevapla:\n\n{}\n\nSoru: {}",
            context, question
        );
        
        self.llm.complete(&prompt).await
    }
}
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 3: KONFİGÜRASYON DOSYALARI
# ═══════════════════════════════════════════════════════════════════════════════

## 3.1 .env Dosyası (API Key'siz)

```env
# ═════════════════════════════════════════════════════════════════
#  SENTIENT OS - LOCAL-FIRST CONFIGURATION
#  NO API KEYS REQUIRED
# ═════════════════════════════════════════════════════════════════

# LLM (Ollama - Local)
OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=gemma3:27b
OLLAMA_FALLBACK_MODEL=phi-4:14b

# Database (PostgreSQL)
DATABASE_URL=postgres://sentient:sentient@localhost:5432/sentient

# Cache (Redis)
REDIS_URL=redis://localhost:6379

# Vector DB (Qdrant)
QDRANT_URL=http://localhost:6333
QDRANT_COLLECTION=sentient_memory

# Object Storage (MinIO)
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=sentient

# Embedding (Local)
EMBEDDING_MODEL=all-MiniLM-L6-v2
EMBEDDING_DIMENSION=384

# Whisper (Local STT)
WHISPER_MODEL=medium
WHISPER_LANGUAGE=tr

# Piper (Local TTS)
PIPER_MODEL=tr-female-low

# Search (DuckDuckGo - No API Key)
SEARCH_PROVIDER=duckduckgo

# Sandbox (Docker)
SANDBOX_TYPE=docker
SANDBOX_TIMEOUT=30

# Logging
RUST_LOG=info
```

---

## 3.2 config/local.toml

```toml
# ═════════════════════════════════════════════════════════════════
#  SENTIENT OS - LOCAL CONFIGURATION
# ═════════════════════════════════════════════════════════════════

[system]
name = "SENTIENT OS"
version = "4.0.0"
mode = "local-first"

[llm]
provider = "ollama"
model = "gemma3:27b"
host = "http://localhost:11434"

[llm.options]
temperature = 0.7
top_p = 0.9
top_k = 40
num_ctx = 32768
num_predict = 4096
repeat_penalty = 1.1

[llm.fallback]
enabled = true
model = "phi-4:14b"

[embedding]
provider = "local"
model = "all-MiniLM-L6-v2"
dimension = 384
batch_size = 32

[vector_db]
provider = "qdrant"
host = "http://localhost:6333"
collection = "sentient_memory"
distance = "Cosine"

[database]
provider = "postgres"
url = "postgres://sentient:sentient@localhost:5432/sentient"
pool_size = 10

[cache]
provider = "redis"
url = "redis://localhost:6379"
ttl = 3600

[storage]
provider = "minio"
endpoint = "http://localhost:9000"
access_key = "minioadmin"
secret_key = "minioadmin"
bucket = "sentient"

[voice.stt]
provider = "whisper_local"
model = "medium"
language = "tr"

[voice.tts]
provider = "piper"
model = "tr-female-low"

[search]
provider = "duckduckgo"
max_results = 10

[sandbox]
type = "docker"
timeout = 30
memory_limit = "512m"

[memory]
type = "hybrid"  # short-term + long-term + vector
max_short_term = 100
max_long_term = 10000

[rag]
chunk_size = 500
chunk_overlap = 50
top_k = 5

[monitoring]
enabled = true
prometheus_port = 9090
grafana_port = 3000

[plugins]
enabled = true
directory = "./plugins"
auto_load = true

[skills]
enabled = true
directory = "./skills"
auto_load = true

[security]
encryption = true
audit_log = true
rate_limit = 100
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 4: KURULUM KOMUTLARI
# ═══════════════════════════════════════════════════════════════════════════════

## 4.1 Tek Komutla Tam Kurulum

```bash
#!/bin/bash
# ═════════════════════════════════════════════════════════════════
#  SENTIENT OS - LOCAL-FIRST SETUP SCRIPT
# ═════════════════════════════════════════════════════════════════

set -e

echo "🚀 SENTIENT OS Local-First Kurulumu Başlıyor..."

# 1. Docker servislerini başlat
echo "📦 Docker servisleri başlatılıyor..."
docker-compose up -d postgres redis qdrant minio prometheus grafana

# 2. Ollama kur ve model indir
echo "🧠 Ollama kuruluyor..."
if ! command -v ollama &> /dev/null; then
    curl -fsSL https://ollama.com/install.sh | sh
fi

echo "📥 LLM modeli indiriliyor (gemma3:27b)..."
ollama pull gemma3:27b

# 3. Embedding modeli indir
echo "📊 Embedding modeli indiriliyor..."
mkdir -p models/embedding
cd models/embedding
if [ ! -f "model.safetensors" ]; then
    wget -q https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/config.json
    wget -q https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/model.safetensors
    wget -q https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/main/tokenizer.json
fi
cd ../..

# 4. Whisper modeli indir
echo "🎤 Whisper modeli indiriliyor..."
mkdir -p models/whisper
cd models/whisper
if [ ! -f "ggml-medium.bin" ]; then
    wget -q https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin
fi
cd ../..

# 5. Rust build
echo "🔨 Rust projesi derleniyor..."
cargo build --release

# 6. Database migration
echo "🗄️ Database migration yapılıyor..."
cargo run --bin sentient-migrate

echo "✅ Kurulum tamamlandı!"
echo ""
echo "📋 Sonraki adımlar:"
echo "   1. SENTIENT'i başlat: cargo run --release"
echo "   2. Dashboard: http://localhost:8080"
echo "   3. Grafana: http://localhost:3000 (admin/sentient)"
echo "   4. Qdrant: http://localhost:6333/dashboard"
echo "   5. MinIO: http://localhost:9001 (minioadmin/minioadmin)"
```

---

## 4.2 Sistem Başlatma

```bash
# Geliştirme modu
cargo run --release

# Veya Docker ile
docker-compose up -d sentient

# Logları izle
docker-compose logs -f sentient
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 5: TEST LİSTESİ
# ═══════════════════════════════════════════════════════════════════════════════

## 5.1 Birim Testleri

```bash
# Tüm testleri çalıştır
cargo test --all

# Belirli crate
cargo test -p sentient_llm
cargo test -p sentient_memory
cargo test -p sentient_gateway
```

## 5.2 Entegrasyon Testleri

```bash
# Ollama bağlantısı
curl http://localhost:11434/api/tags

# Qdrant bağlantısı
curl http://localhost:6333/collections

# PostgreSQL bağlantısı
psql -h localhost -U sentient -d sentient -c "SELECT 1"

# Redis bağlantısı
redis-cli ping

# MinIO bağlantısı
curl http://localhost:9000/minio/health/live
```

## 5.3 Uçtan Uca Test

```bash
# API testi
curl -X POST http://localhost:8080/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Merhaba, nasılsın?"}'

# RAG testi
curl -X POST http://localhost:8080/api/rag/query \
  -H "Content-Type: application/json" \
  -d '{"query": "SENTIENT nedir?"}'

# Search testi
curl -X POST http://localhost:8080/api/search \
  -H "Content-Type: application/json" \
  -d '{"query": "Rust programlama dili"}'
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 6: SORUN GİDERME
# ═══════════════════════════════════════════════════════════════════════════════

## 6.1 Yaygın Sorunlar

| Sorun | Çözüm |
|-------|-------|
| Ollama connection refused | `ollama serve` |
| Qdrant collection not found | Collection oluştur |
| PostgreSQL connection failed | Docker'ı yeniden başlat |
| GPU out of memory | Daha küçük model kullan |
| Whisper too slow | tiny veya base model kullan |

## 6.2 Logları İnceleme

```bash
# SENTIENT logları
tail -f logs/sentient.log

# Docker logları
docker-compose logs -f sentient

# Ollama logları
journalctl -u ollama -f
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 7: DONANIM ÖNERİLERİ
# ═══════════════════════════════════════════════════════════════════════════════

## 7.1 Minimum Sistem

| Bileşen | Minimum |
|---------|---------|
| CPU | 4 çekirdek |
| RAM | 16 GB |
| GPU | - |
| Disk | 50 GB SSD |

**Çalıştırılabilir modeller:** phi-4:14b, gemma3:12b

## 7.2 Önerilen Sistem

| Bileşen | Önerilen |
|---------|----------|
| CPU | 8+ çekirdek |
| RAM | 32 GB |
| GPU | RTX 3090/4090 (24GB VRAM) |
| Disk | 500 GB NVMe SSD |

**Çalıştırılabilir modeller:** gemma3:27b, mistral:24b, llama3.1:70b (quantized)

## 7.3 Sunucu Sistemi

| Bileşen | Sunucu |
|---------|--------|
| CPU | 16+ çekirdek |
| RAM | 64+ GB |
| GPU | 2x A100 40GB veya H100 |
| Disk | 2 TB NVMe SSD |

**Çalıştırılabilir modeller:** Tüm modeller (full precision)

---

*📅 Tarih: 13 Nisan 2026*
*🔧 Durum: Plan Hazır*
*🚀 Sonraki Adım: Faz 1 başlatılacak*
