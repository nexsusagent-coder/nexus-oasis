# ═══════════════════════════════════════════════════════════════════════════════════
#  🐺 SENTIENT - KAPSAMLI KURULUM KILAVUZU
# ═══════════════════════════════════════════════════════════════════════════════════
#
#  ⚠️  BU BELGE GİZLİDİR - ASLA GITHUB'A PUSH EDİLMEZ
#  ⚠️  Hassas API anahtarları ve yapılandırma bilgileri içerir
#
#  Bu dosya .gitignore ile korunmaktadır.
# ═══════════════════════════════════════════════════════════════════════════════════

## 📋 İÇİNDEKİLER

1. [Sistem Gereksinimleri](#1-sistem-gereksinimleri)
2. [Hızlı Kurulum](#2-hızlı-kurulum)
3. [Detaylı Yapılandırma](#3-detaylı-yapılandırma)
4. [Entegre Repoların Kullanımı](#4-entegre-repoların-kullanımı)
5. [Skill Sistemi](#5-skill-sistemi)
6. [V-GATE API Gateway](#6-v-gate-api-gateway)
7. [Troubleshooting](#7-troubleshooting)

---

# 1. SİSTEM GEREKSİNİMLERİ

## 1.1 Donanım Gereksinimleri

| Bileşen | Minimum | Önerilen |
|---------|---------|----------|
| CPU | 4 çekirdek | 8+ çekirdek |
| RAM | 8 GB | 16+ GB |
| Disk | 20 GB | 50+ GB SSD |
| GPU | - | CUDA destekli (opsiyonel) |

## 1.2 Yazılım Gereksinimleri

```bash
# İşletim Sistemi
- Ubuntu 22.04+ / Debian 12+
- macOS 12+ (Apple Silicon destekli)
- Windows 10/11 + WSL2

# Zorunlu Bağımlılıklar
- Rust 1.75+ (stable toolchain)
- Python 3.10+
- Node.js 18+ (npm 9+)
- Docker 24+ & Docker Compose v2

# Veritabanı (opsiyonel ama önerilen)
- SQLite 3.40+
- Qdrant (vektör veritabanı)
```

## 1.3 Rust Kurulumu

```bash
# Rust kur
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Stable toolchain
rustup default stable
rustup update

# Gerekli bileşenler
rustup component add clippy rustfmt rust-analyzer

# Hedef platformlar (cross-compile için)
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
```

## 1.4 Python Ortamı

```bash
# pyenv kur (önerilen)
curl https://pyenv.run | bash

# Python 3.11
pyenv install 3.11.7
pyenv global 3.11.7

# Virtual environment
python -m venv .venv
source .venv/bin/activate

# Gerekli paketler
pip install --upgrade pip
pip install maturin pyo3 numpy torch transformers
```

---

# 2. HIZLI KURULUM

## 2.1 Repoyu İndir

```bash
# SSH ile klonla (önerilen)
git clone git@github.com:YOUR_ORG/sentient-core.git
cd sentient-core

# Veya HTTPS ile
git clone https://github.com/YOUR_ORG/sentient-core.git
cd sentient-core
```

## 2.2 Ortam Değişkenlerini Ayarla

```bash
# .env dosyası oluştur (BU DOSYA GİTLİ OLMAMALI!)
cat > .env << 'EOF'
# ═════════════════════════════════════════════════════════════════
#  SENTIENT ORTAM DEĞİŞKENLERİ
# ═════════════════════════════════════════════════════════════════

# V-GATE API Anahtarları (Sunucu taraflı)
OPENROUTER_API_KEY=sk-or-xxxxxxxxxxxxxxxxxxxxxxxx
OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxxxxxx
ANTHROPIC_API_KEY=sk-ant-xxxxxxxxxxxxxxxxxxxxxxxx

# Modeller
SENTIENT_DEFAULT_MODEL=openrouter/qwen/qwen3-72b-instruct
SENTIENT_FALLBACK_MODEL=openrouter/qwen/qwen3-coder:free

# Veritabanı
DATABASE_URL=sqlite:./data/sentient.db
QDRANT_URL=http://localhost:6333
QDRANT_API_KEY=your-qdrant-key

# Memory Cube
MEMORY_DB_PATH=./data/memory/cube.db
MEMORY_VECTOR_DIM=1536

# Güvenlik
GUARDRAILS_ENABLED=true
GUARDRAILS_MAX_PROMPT_LENGTH=10000

# Tarayıcı (Browser-Use için)
BROWSER_HEADLESS=true
BROWSER_TIMEOUT_MS=30000

# Sandbox (OpenManus için)
SANDBOX_DOCKER_IMAGE=python:3.11-slim
SANDBOX_MEMORY_LIMIT=512m
SANDBOX_TIMEOUT_MS=60000

# Log seviyesi
RUST_LOG=info,sentient=debug
EOF

# Dosya izinlerini kısıtla
chmod 600 .env
```

## 2.3 Sistemi Derle

```bash
# Debug derleme (hızlı)
cargo build

# Release derleme (optimize)
cargo build --release

# Tüm workspace'i kontrol et
cargo check --workspace

# Testleri çalıştır
cargo test --workspace
```

## 2.4 Veritabanlarını Başlat

```bash
# SQLite veritabanı oluştur
mkdir -p data/memory data/sessions data/cache
sqlite3 data/sentient.db < migrations/init.sql

# Qdrant'ı Docker ile başlat
docker run -d \
  --name qdrant \
  -p 6333:6333 \
  -p 6334:6334 \
  -v $(pwd)/data/qdrant:/qdrant/storage \
  qdrant/qdrant:latest
```

## 2.5 SENTIENT'yı Başlat

```bash
# Geliştirici modu
cargo run --bin sentient-cli

# Web dashboard
cargo run --bin sentient-web

# Tüm servisler (Docker Compose)
docker-compose up -d
```

---

# 3. DETAYLI YAPILANDIRMA

## 3.1 SENTIENT Yapılandırma Dosyası

`config/sentient.toml`:

```toml
# ═════════════════════════════════════════════════════════════════
#  SENTIENT Ana Yapılandırması
# ═════════════════════════════════════════════════════════════════

[system]
name = "SENTIENT"
version = "0.1.0"
log_level = "info"
max_agents = 100
max_concurrent_tasks = 50

[memory]
# Memory Cube yapılandırması
cube_path = "./data/memory/cube.db"
vector_dim = 1536          # OpenAI embedding boyutu
max_entries = 1000000
cleanup_interval_secs = 3600
retention_days = 30

[vgate]
# V-GATE API Gateway
enabled = true
proxy_url = "http://localhost:8080"
rate_limit_rpm = 100       # Dakikada maksimum istek
timeout_secs = 30
retry_attempts = 3

[guardrails]
# Güvenlik sistemi
enabled = true
check_input = true
check_output = true
max_prompt_length = 10000
blocked_patterns = [
    "ignore previous instructions",
    "system prompt:",
    "you are now",
]

[agents]
# Ajan varsayılanları
default_model = "openrouter/qwen/qwen3-72b-instruct"
default_temperature = 0.7
max_tokens = 4096
verbose = true

[integrations]
# Python entegrasyonları
crewai_enabled = true
browser_enabled = true
memory_enabled = true
sandbox_enabled = true

[crewai]
agents_config = "./config/crewai/agents.yaml"
tasks_config = "./config/crewai/tasks.yaml"

[browser]
headless = true
user_agent = "Mozilla/5.0 (SENTIENT Bot)"
timeout_ms = 30000
max_pages = 10

[sandbox]
docker_image = "python:3.11-slim"
memory_limit = "512m"
cpu_limit = "1.0"
timeout_ms = 60000
network_enabled = false
```

## 3.2 Qdrant Koleksiyonları

```bash
# SENTIENT için koleksiyon oluştur
curl -X PUT http://localhost:6333/collections/sentient_memory \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": {
      "size": 1536,
      "distance": "Cosine"
    },
    "optimizers_config": {
      "indexing_threshold": 10000
    }
  }'

# Koleksiyon durumunu kontrol et
curl http://localhost:6333/collections/sentient_memory
```

---

# 4. ENTEGRE REPOLARIN KULLANIMI

## 4.1 CrewAI (Çoklu Ajan Koordinasyonu)

### Klasör Yapısı
```
integrations/crewai/
├── crewai/           # Ana CrewAI kütüphanesi
├── crewai-tools/     # CrewAI araçları
└── sentient_crew/       # SENTIENT-specific yapılandırma
    ├── agents.yaml   # Ajan tanımları
    ├── tasks.yaml    # Görev tanımları
    └── crews.yaml    # Ekip tanımları
```

### Rust'tan Kullanım

```rust
use sentient_python::wrappers::{CrewAgent, CrewTask};

// Ajan oluştur
let agent = CrewAgent::new("Araştırmacı", "Bilgi topla ve analiz et")
    .with_backstory("Deneyimli bir araştırmacısın");

// Görev oluştur ve ajana ata
let task = CrewTask::new("AI trendlerini ara", "2024 trendleri raporu")
    .assign_to(agent.id);

// PyO3 bridge üzerinden çalıştır
let result = python_bridge.run_crew("research_crew").await?;
```

### agents.yaml Örneği

```yaml
agents:
  researcher:
    role: "Kıdemli Araştırmacı"
    goal: "Derinlemesine araştırma yapmak ve bilgi toplamak"
    backstory: |
      10 yıllık deneyime sahip bir araştırmacısın.
      Teknoloji trendlerini takip etmek konusunda uzmansın.
    verbose: true
    tools:
      - web_search
      - file_reader
      
  writer:
    role: "İçerik Yazarı"
    goal: "Araştırma sonuçlarını okunabilir raporlara dönüştürmek"
    backstory: "Teknik konuları sade dille anlatan bir yazarsın."
    verbose: true
    tools:
      - file_writer
```

### tasks.yaml Örneği

```yaml
tasks:
  research_task:
    description: "{topic} hakkında kapsamlı araştırma yap"
    expected_output: "Kaynakça ile desteklenmiş detaylı araştırma raporu"
    agent: researcher
    tools:
      - web_search
      
  writing_task:
    description: "Araştırma sonuçlarını blog yazısına dönüştür"
    expected_output: "SEO uyumlu, 1500+ kelimelik blog yazısı"
    agent: writer
    context:
      - research_task
```

---

## 4.2 Browser-Use (Web Otomasyonu)

### Klasör Yapısı
```
integrations/browser-use/
├── browser_use/      # Ana kütüphane
└── sentient_browser/    # SENTIENT wrapper'ları
```

### Rust'tan Kullanım

```rust
use sentient_python::wrappers::{BrowserWrapper, TaskResult};

// Browser başlat
let browser = BrowserWrapper::new().await?;

// Görev tanımla
let task = "https://example.com sitesine git ve ürün fiyatlarını topla";
let result = browser.execute_task(task).await?;

// Sonuçları al
println!("Başarılı mı: {}", result.success);
println!("Çıktı: {}", result.output);
println!("Adım sayısı: {}", result.steps.len());
```

### Browser Yapılandırması

```rust
BrowserConfig {
    headless: true,
    user_agent: "Mozilla/5.0 (SENTIENT)".to_string(),
    viewport: Viewport { width: 1920, height: 1080 },
    timeout_ms: 30000,
    max_pages: 10,
    cookies_enabled: false,
    javascript_enabled: true,
}
```

---

## 4.3 Mem0 (Bellek Yönetimi)

### Klasör Yapısı
```
integrations/mem0/
├── mem0ai/           # Ana kütüphane
└── sentient_memory/     # SENTIENT integration
```

### Rust'tan Kullanım

```rust
use sentient_memory::{MemoryCube, MemoryType};

// Memory Cube başlat
let mut cube = MemoryCube::new("./data/memory/cube.db")?;

// Anı kaydet
let id = cube.create(
    "Kullanıcı Python hakkında soru sordu",
    MemoryType::Episodic,
    Some(json!({ "topic": "python", "user_id": "user_123" })),
    Some(3600), // 1 saat TTL
)?;

// Anı ara
let results = cube.search("python", 10)?;

// Benzer anıları bul (vektör araması)
let similar = cube.search_similar(embedding_vector, 5)?;
```

### Memory Cube API

```rust
impl MemoryCube {
    // CRUD operasyonları
    pub fn create(&mut self, content: &str, memory_type: MemoryType, 
                  metadata: Option<Value>, ttl: Option<i64>) -> Result<Uuid>;
    pub fn read(&self, id: Uuid) -> Result<Option<MemoryEntry>>;
    pub fn update(&mut self, id: Uuid, content: &str) -> Result<()>;
    pub fn delete(&mut self, id: Uuid) -> Result<bool>;
    
    // Arama
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>>;
    pub fn search_similar(&self, embedding: &[f32], limit: usize) 
        -> Result<Vec<(MemoryEntry, f32)>>;
    
    // Bakım
    pub fn cleanup_expired(&mut self) -> Result<usize>;
    pub fn count(&self) -> Result<usize>;
    pub fn stats(&self) -> Result<CubeStats>;
}
```

---

## 4.4 OpenManus (Kod Sandbox)

### Klasör Yapısı
```
integrations/openmanus/
├── manus/            # Ana framework
└── sentient_sandbox/    # SENTIENT integration
```

### Rust'tan Kullanım

```rust
use sentient_python::wrappers::SandboxResult;

// Sandbox ortamı oluştur
let sandbox = SandboxWrapper::new(config).await?;

// Python kodu çalıştır
let code = r#"
import numpy as np
data = np.random.randn(100)
print(f"Ortalama: {data.mean():.4f}")
print(f"Std Sapma: {data.std():.4f}")
"#;

let result = sandbox.execute(code).await?;

if result.is_ok() {
    println!("Çıktı: {}", result.output());
} else {
    println!("Hata: {}", result.stderr);
}
```

### Sandbox Yapılandırması

```rust
SandboxConfig {
    docker_image: "python:3.11-slim".to_string(),
    memory_limit: "512m".to_string(),
    cpu_limit: "1.0".to_string(),
    timeout_ms: 60000,
    network_enabled: false,
    volume_mounts: vec![
        "./sandbox/workspace:/workspace".to_string(),
    ],
    env_vars: HashMap::from([
        ("PYTHONUNBUFFERED", "1"),
    ]),
}
```

---

# 5. SKILL SİSTEMİ

## 5.1 Skill Yapısı

SENTIENT'da "skill"ler, belirli görevleri yerine getiren modüler yeteneklerdir. Her skill:

1. Rust tabanlı core implementasyon
2. Python wrapper (PyO3 ile)
3. Yapılandırma dosyası
4. Test suite

içerir.

### Skill Dizini

```
skills/
├── research/           # Web araştırması
│   ├── skill.toml
│   ├── src/lib.rs
│   └── tests/
├── codegen/           # Kod üretimi
│   ├── skill.toml
│   ├── src/lib.rs
│   └── tests/
├── analysis/          # Veri analizi
│   ├── skill.toml
│   ├── src/lib.rs
│   └── tests/
└── automation/        # Görev otomasyonu
    ├── skill.toml
    ├── src/lib.rs
    └── tests/
```

## 5.2 Skill Tanımı

`skills/research/skill.toml`:

```toml
[skill]
name = "research"
version = "0.1.0"
description = "Web araştırması ve bilgi toplama yeteneği"
author = "SENTIENT Team"

[dependencies]
sentient_core = { path = "../../crates/sentient_core" }
sentient_python = { path = "../../crates/sentient_python" }
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.18"

[config]
# Skill parametreleri
max_results = 10
timeout_secs = 30
sources = ["web", "academic", "news"]

[triggers]
# Ne zaman çalışır
keywords = ["araştır", "ara", "bul", "research", "search"]
patterns = ["^ Araştır: (.+)$"]
```

## 5.3 Skill Implementasyonu

`skills/research/src/lib.rs`:

```rust
use sentient_core::traits::*;
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};

/// Research Skill - Web araştırması yapar
pub struct ResearchSkill {
    id: uuid::Uuid,
    config: ResearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchConfig {
    pub max_results: usize,
    pub timeout_secs: u64,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub query: String,
    pub results: Vec<ResearchEntry>,
    pub summary: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchEntry {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub source: String,
}

impl ResearchSkill {
    pub fn new(config: ResearchConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            config,
        }
    }
    
    /// Araştırma gerçekleştir
    pub async fn research(&self, query: &str) -> SENTIENTResult<ResearchResult> {
        // 1. Tarayıcıyı başlat (Browser-Use)
        let browser = sentient_python::wrappers::BrowserWrapper::new().await?;
        
        // 2. Arama yap
        let search_results = browser.search(query, self.config.max_results).await?;
        
        // 3. Sonuçları işle
        let entries: Vec<ResearchEntry> = search_results
            .into_iter()
            .map(|r| ResearchEntry {
                title: r.title,
                url: r.url,
                snippet: r.snippet,
                source: "web".to_string(),
            })
            .collect();
        
        // 4. Özet oluştur (V-GATE üzerinden LLM)
        let summary = self.generate_summary(&entries).await?;
        
        Ok(ResearchResult {
            query: query.to_string(),
            results: entries,
            summary,
            sources: vec!["web".to_string()],
        })
    }
    
    async fn generate_summary(&self, entries: &[ResearchEntry]) -> SENTIENTResult<String> {
        // LLM ile özet oluştur
        // ...
        Ok("Araştırma özeti...".to_string())
    }
}

// SENTIENT bileşeni olarak kaydet
impl SENTIENTComponent for ResearchSkill {
    fn id(&self) -> uuid::Uuid { self.id }
    fn name(&self) -> &str { "research" }
    fn version(&self) -> &str { "0.1.0" }
    fn component_type(&self) -> ComponentType { ComponentType::Tool }
}
```

## 5.4 Skill Kaydı

```rust
// sentient_core/src/system.rs
impl SENTIENTSystem {
    /// Skill kaydet
    pub async fn register_skill(&self, skill: Box<dyn Skill>) -> SENTIENTResult<()> {
        let skill_name = skill.name().to_string();
        
        // Skill'i kaydet
        self.skills.write().await.insert(skill_name.clone(), skill);
        
        log::info!("🛠️  SKILL: '{}' yüklendi", skill_name);
        Ok(())
    }
    
    /// Skill çalıştır
    pub async fn execute_skill(&self, name: &str, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        let skills = self.skills.read().await;
        
        if let Some(skill) = skills.get(name) {
            skill.execute(input).await
        } else {
            Err(SENTIENTError::SkillNotFound(name.to_string()))
        }
    }
}
```

## 5.5 Built-in Skill'ler

| Skill | Açıklama | Entegrasyon |
|-------|----------|-------------|
| `research` | Web araştırması | Browser-Use |
| `codegen` | Kod üretimi | OpenManus Sandbox |
| `analysis` | Veri analizi | NumPy/Pandas |
| `automation` | Görev otomasyonu | CrewAI |
| `memory` | Bellek yönetimi | Mem0 |
| `web_browse` | Web etkileşimi | Browser-Use |
| `code_run` | Kod çalıştırma | OpenManus |

---

# 6. V-GATE API GATEWAY

## 6.1 V-GATE Mimarisi

```
┌─────────────────────────────────────────────────────────────────┐
│                         V-GATE PROXY                             │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐    │
│  │ Rate      │  │ Request    │  │ API Key   │  │ Response  │    │
│  │ Limiter   │  │ Logger     │  │ Manager   │  │ Cache     │    │
│  └───────────┘  └───────────┘  └───────────┘  └───────────┘    │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐      ┌─────────────────────┐           │
│  │ OpenRouter          │      │ Direct LLM APIs     │           │
│  │ (Multi-Model)       │      │ (OpenAI, Anthropic) │           │
│  └─────────────────────┘      └─────────────────────┘           │
└─────────────────────────────────────────────────────────────────┘
```

## 6.2 V-GATE Yapılandırması

`config/vgate.toml`:

```toml
[proxy]
listen = "0.0.0.0:8080"
upstream_timeout_secs = 30

[openrouter]
base_url = "https://openrouter.ai/api/v1"
api_key_env = "OPENROUTER_API_KEY"
default_headers = { "HTTP-Referer" = "https://sentient.ai", "X-Title" = "SENTIENT" }

[openai]
base_url = "https://api.openai.com/v1"
api_key_env = "OPENAI_API_KEY"

[anthropic]
base_url = "https://api.anthropic.com/v1"
api_key_env = "ANTHROPIC_API_KEY"

[cache]
enabled = true
ttl_secs = 3600
max_entries = 10000

[rate_limit]
requests_per_minute = 100
burst = 20
```

## 6.3 V-GATE API Kullanımı

```rust
use sentient_vgate::{VGateEngine, LlmRequest, Message};

// V-GATE başlat
let vgate = VGateEngine::new(config);

// İstek oluştur
let request = LlmRequest {
    model: "openrouter/qwen/qwen3-72b-instruct".into(),
    messages: vec![
        Message { role: "system".into(), content: "Yardımcı bir asistansın.".into() },
        Message { role: "user".into(), content: "Merhaba!".into() },
    ],
    max_tokens: Some(1024),
    temperature: Some(0.7),
    stream: Some(false),
};

// İstek gönder
let response = vgate.send_request(request).await?;

println!("Yanıt: {}", response.content);
```

---

# 7. TROUBLESHOOTING

## 7.1 Yaygın Hatalar

### Hata: "API Key Not Found"
```bash
# Çözüm: .env dosyasını kontrol et
cat .env | grep API_KEY

# Veya ortam değişkenini ayarla
export OPENROUTER_API_KEY="sk-or-xxxx"
```

### Hata: "Qdrant Connection Failed"
```bash
# Çözüm: Qdrant'ı yeniden başlat
docker restart qdrant

# Veya yeni konteyner oluştur
docker run -d --name qdrant -p 6333:6333 qdrant/qdrant
```

### Hata: "Python Module Not Found"
```bash
# Çözüm: Python sanal ortamını aktif et
source .venv/bin/activate

# Bağımlılıkları yeniden kur
pip install -r requirements.txt
maturin develop
```

### Hata: "Compilation Failed"
```bash
# Çözüm: Rust toolchain'i güncelle
rustup update stable
cargo clean
cargo build
```

## 7.2 Log Analizi

```bash
# SENTIENT loglarını görüntüle
tail -f logs/sentient.log

# Belirli bir modülün logları
grep "GUARDRAILS" logs/sentient.log

# Hata logları
grep -E "^ERROR|^WARN" logs/sentient.log
```

## 7.3 Sağlık Kontrolleri

```bash
# Sistem durumu
cargo run --bin sentient-cli status

# Veritabanı bağlantısı
sqlite3 data/sentient.db "SELECT COUNT(*) FROM memories;"

# Qdrant koleksiyonları
curl http://localhost:6333/collections

# V-GATE proxy
curl http://localhost:8080/health
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  📞 DESTEK
# ═══════════════════════════════════════════════════════════════════════════════

Sorularınız için:
- GitHub Issues: https://github.com/YOUR_ORG/sentient-core/issues
- Dokümantasyon: ./docs/
- E-posta: support@sentient.ai

---

⚠️  BU BELGE GİZLİDİR VE ASLA GITHUB'A PUSH EDİLMEMELİDİR!
⚠️  .gitignore tarafından korunmaktadır.
