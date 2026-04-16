# 🧠 SENTIENT OS - Detaylı Kullanım Rehberi & Senaryolar

> **Başlangıçtan ileri seviyeye — her senaryo, her mod, her entegrasyon**

---

## 📑 İçindekiler

1. [Kurulum Senaryoları](#1-kurulum-senaryoları)
2. [JARVIS Modu — Sesli Asistan](#2-jarvis-modu--sesli-asistan)
3. [Tam Otonom Kullanım — Desktop Agent](#3-tam-otonom-kullanım--desktop-agent)
4. [Multi-Agent Orkestrasyonu](#4-multi-agent-orkestrasyonu)
5. [Entegre Projeleri Kullanma](#5-entegre-projeleri-kullanma)
6. [AI Gateway/Router Kullanımı](#6-ai-gatewayrouter-kullanımı)
7. [Proactive Engine — Zamanlı Görevler](#7-proactive-engine--zamanlı-görevler)
8. [Daemon Modu — Arka Planda Asistan](#8-daemon-modu--arka-planda-asistan)
9. [Kanal Entegrasyonları — Telegram/Discord/WhatsApp](#9-kanal-entegrasyonları)
10. [Rust API ile Kullanım](#10-rust-api-ile-kullanım)
11. [Docker Production Deployment](#11-docker-production-deployment)
12. [Güvenlik — V-GATE & Guardrails](#12-güvenlik--v-gate--guardrails)

---

## 1. Kurulum Senaryoları

### Senaryo A: Yeni Başlayan — Hiçbir Şey Kurulu Değil

```bash
# Tek komutla her şeyi kur
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
```

**Ne olur?**
1. ⚠️ Yasal uyarı → `y` tuşla
2. Sistem tespiti → RAM: 16GB, GPU: yok
3. Kurulum modu → `1` (Quick)
4. LLM seçimi → `2` (Qwen3 30B-A3B MoE — 4GB VRAM, ücretsiz!)
5. Ollama otomatik kurulur → Model indirilir
6. .env oluşturulur → cargo build --release
7. ✅ `sentient chat` ile başla!

### Senaryo B: Geliştirici — API Key ile Hızlı Başlangıç

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# OpenRouter key al: https://openrouter.ai/keys ($5 ücretsiz kredi)
export OPENROUTER_API_KEY=sk-or-v1-xxxxx

# Hızlı kurulum
./install.sh --yes --quick

# Başlat
sentient chat --model openrouter/auto
```

### Senaryo C: Enterprise — Docker ile Tam Kurulum

```bash
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Tam kurulum (tüm servisler + Docker)
./install.sh --full

# Docker servisleri otomatik başlar:
# PostgreSQL, Redis, Qdrant, MinIO, Prometheus, Grafana, Ollama, SearXNG, RabbitMQ

# Gateway başlat
sentient gateway
# → http://localhost:8080 (OpenAI-compatible API)
```

### Senaryo D: Mevcut Ollama Kullanıcısı

```bash
# Ollama zaten kurulu ve modeller indirilmiş
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE

# Sadece derle ve kullan
./quick-start.sh

# Mevcut modelini kullan
sentient chat --model ollama:llama3.3:70b
```

---

## 2. JARVIS Modu — Sesli Asistan

### JARVIS Nasıl Çalışır?

```
┌─────────────────────────────────────────────────────────────────┐
│                      JARVIS MİMARİSİ                             │
│                                                                 │
│   🎤 Mikrofon                                                   │
│     │                                                           │
│     ▼                                                           │
│   ┌─────────────┐                                               │
│   │ Wake Word   │  "Hey Sentient" algılama                      │
│   │ (Porcupine) │  — Vosk veya Whisper ile de mümkün            │
│   └─────┬───────┘                                               │
│         │                                                       │
│         ▼                                                       │
│   ┌─────────────┐                                               │
│   │ STT        │  Ses → Metin çevirisi                          │
│   │ (Whisper)  │  — Lokal: whisper.cpp (ücretsiz)              │
│   └─────┬──────┘    — API: OpenAI Whisper (daha iyi)           │
│         │                                                       │
│         ▼                                                       │
│   ┌─────────────┐                                               │
│   │ LLM        │  Yanıt üretimi                                  │
│   │ (Herhangi) │  — Lokal: Ollama (ücretsiz)                   │
│   └─────┬──────┘    — API: GPT-4o, Claude, Gemini              │
│         │                                                       │
│         ▼                                                       │
│   ┌─────────────┐                                               │
│   │ TTS        │  Metin → Ses çevirisi                          │
│   │ (Piper)    │  — Lokal: Piper (Türkçe, ücretsiz)           │
│   └─────┬──────┘    — API: ElevenLabs (daha doğal)             │
│         │                                                       │
│         ▼                                                       │
│   🔊 Hoparlör                                                   │
│         │                                                       │
│         └──────→ Döngü devam eder (wake word dinliyor)          │
└─────────────────────────────────────────────────────────────────┘
```

### Lokal JARVIS (Tamamen Ücretsiz)

```bash
# 1. Ollama + model kur
ollama pull qwen3:30b-a3b

# 2. Whisper.cpp kur
git clone https://github.com/ggerganov/whisper.cpp ~/whisper.cpp
cd ~/whisper.cpp && make
bash ./models/download-ggml-model.sh medium

# 3. Piper TTS kur (Türkçe destekli)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xzf piper_1.2.0_amd64.tar.gz -C ~/.local/

# Türkçe ses modeli
mkdir -p ~/.local/share/piper/models
wget -O ~/.local/share/piper/models/tr_TR-medium.onnx \
  https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
wget -O ~/.local/share/piper/models/tr_TR-medium.onnx.json \
  https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json

# 4. .env yapılandır
cat >> .env << 'EOF'
VOICE_ENABLED=true
VOICE_STT=whisper_cpp
VOICE_TTS=piper
VOICE_LANGUAGE=tr
WHISPER_MODEL_PATH=~/whisper.cpp/models/ggml-medium.bin
PIPER_MODEL_PATH=~/.local/share/piper/models/tr_TR-medium.onnx
EOF

# 5. JARVIS başlat!
sentient voice
```

**Kullanım Senaryosu:**
```
Kullanıcı: "Hey Sentient"
SENTIENT: 🔊 "Evet, dinliyorum"
Kullanıcı: "Bugün hava nasıl?"
SENTIENT: 🔊 "İstanbul'da bugün hava 22 derece, güneşli"
Kullanıcı: "Bana müzik aç"
SENTIENT: 🔊 "YouTube'da müzik açıyorum..." → Browser otomatik açılır
```

### API JARVIS (Daha Doğal Ses)

```bash
# .env
VOICE_ENABLED=true
VOICE_STT=openai_whisper
VOICE_TTS=elevenlabs
OPENAI_API_KEY=sk-...
ELEVENLABS_API_KEY=...

# Başlat
sentient voice --wake-word "hey sentient"
```

### JARVIS Daemon Modu (7/24 Arka Planda)

```bash
# Daemon olarak başlat — sistem açıldığında otomatik çalışır
sentient daemon start

# Durdur
sentient daemon stop

# Durum
sentient daemon status
```

**Daemon ne yapar?**
- Wake word sürekli dinler
- Sesli komutları işler
- Proactive trigger'ları kontrol eder
- Arka planda email/rss takibi yapar
- Bildirimleri iletir

---

## 3. Tam Otonom Kullanım — Desktop Agent

### Desktop Agent Mimarisi

```
┌─────────────────────────────────────────────────────────────────────┐
│                     OASIS AUTONOMOUS AGENT                          │
│                                                                     │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐           │
│  │ PERCEIVE │ → │  DECIDE  │ → │   ACT   │ → │  LEARN  │           │
│  └────┬────┘   └────┬────┘   └────┬────┘   └────┬────┘           │
│       │              │              │              │               │
│  ┌────▼────┐   ┌─────▼───┐   ┌──────▼───┐   ┌─────▼───┐          │
│  │ SCREEN  │   │ PLANNER │   │  TOOLS   │   │ MEMORY  │          │
│  │ VISION  │   │ SAFETY  │   │ CHAINING │   │ HEALING │          │
│  └─────────┘   └─────────┘   └──────────┘   └─────────┘          │
│                                                                     │
│  SOVEREIGN CONSTITUTION L1:                                        │
│  ✗ rm -rf, format, dd, sudo, chmod 777...                        │
│  ✓ browser, editor, git, terminal, file manager                   │
└─────────────────────────────────────────────────────────────────────┘
```

### Senaryo 1: Web'de Araştırma Yap

```bash
sentient desktop --goal "Rust ile web framework karşılaştırması yap"
```

**Agent ne yapar?**
1. Browser açar → Google'da "Rust web framework comparison" arar
2. İlk 5 sonucu açar → Her sayfayı okur
3. Önemli bilgileri çıkarır → Belleğe kaydeder
4. Karşılaştırma tablosu oluşturur → Markdown rapor yazar

### Senaryo 2: Kod Yazma ve Test Etme

```bash
sentient desktop --goal "Bir Rust REST API projesi oluştur"
```

**Agent ne yapar?**
1. Terminal açar → `cargo new api_project`
2. VS Code açar → Kod yazar
3. Terminalde test eder → `cargo test`
4. Hata varsa düzeltir → Tekrar test eder
5. Çalışan projeyi raporlar

### Senaryo 3: Günlük Raporlama

```bash
sentient desktop --goal "Bugünkü GitHub commit'lerimi özetle"
```

**Agent ne yapar?**
1. Terminal açar → `git log --since today`
2. Commit mesajlarını okur
3. Özet rapor oluşturur
4. Markdown dosyasına yazar

### Güvenli Mod (Safe Mode)

```bash
# Kritik aksiyonlar için insan onayı iste
sentient desktop --safe-mode

# Sovereign policy — yasaklı komutlar asla çalışmaz
sentient desktop --sovereign
```

**Yasaklı Komutlar (50+):**
```
rm -rf /    format C:    dd if=/dev/zero    chmod 777 /
sudo rm     chown root    curl | bash      mkfs
shutdown    reboot        halt             poweroff
```

---

## 4. Multi-Agent Orkestrasyonu

### CrewAI Modu (Rol Bazlı)

```rust
use sentient_agents::{AgentOrchestrator, AgentRole, AgentFramework};

#[tokio::main]
async fn main() {
    let orchestrator = AgentOrchestrator::new(AgentFramework::CrewAI);

    // Araştırmacı
    let researcher = Agent::new("researcher")
        .role(AgentRole::Researcher)
        .model("deepseek-r1")
        .goal("Piyasa araştırması yap");

    // Yazar
    let writer = Agent::new("writer")
        .role(AgentRole::Writer)
        .model("gpt-4o")
        .goal("Rapor yaz");

    // Editör
    let editor = Agent::new("editor")
        .role(AgentRole::Reviewer)
        .model("claude-4-sonnet")
        .goal("Raporu düzenle");

    orchestrator.add_agent(researcher).await;
    orchestrator.add_agent(writer).await;
    orchestrator.add_agent(editor).await;

    let result = orchestrator.run("Yapay zeka pazar raporu yaz").await;
    println!("{}", result.output);
}
```

### CLI ile Multi-Agent

```bash
# Agent oluştur
sentient agent create researcher --model deepseek-r1
sentient agent create writer --model gpt-4o
sentient agent create editor --model claude-4-sonnet

# Crew oluştur
sentient crew create report-team --agents researcher,writer,editor

# Görev ver
sentient crew run report-team --goal "AI pazar analizi raporu yaz"
```

### AutoGen Modu (Konversation Bazlı)

```bash
# Microsoft AutoGen tarzı sohbet
sentient agent create coder --model gpt-4o --role "Kod yaz"
sentient agent create reviewer --model claude-4-sonnet --role "Kod incele"

# Agent'ları konuştur
sentient agent converse coder,reviewer --topic "Rust web API tasarımı"
```

### Swarm Modu (OpenAI)

```bash
# OpenAI Swarm tarzı hafif orkestrasyon
sentient swarm create support-team --agents 3

# Agent'lar arası handoff
sentient swarm run support-team --goal "Müşteri destek taleplerini çöz"
```

---

## 5. Entegre Projeleri Kullanma

SENTIENT OS, 72+ açık kaynak projeyi entegre eder. Her biri `integrations/` dizininde bulunur.

### CrewAI — Multi-Agent Framework

```bash
# CrewAI entegrasyonu
cd integrations/agents/crewai

# SENTIENT ile CrewAI kullan
python -c "
from crewai import Agent, Task, Crew
from sentient_bridge import SentientLLM

llm = SentientLLM(model='gpt-4o')

researcher = Agent(role='Researcher', llm=llm, goal='Araştırma yap')
writer = Agent(role='Writer', llm=llm, goal='Rapor yaz')

task1 = Task(description='AI pazarını araştır', agent=researcher)
task2 = Task(description='Rapor yaz', agent=writer)

crew = Crew(agents=[researcher, writer], tasks=[task1, task2])
result = crew.kickoff()
"
```

### LangChain — LLM Framework

```bash
cd integrations/framework/langchain

python -c "
from langchain.chat_models import ChatSentient
from langchain.schema import HumanMessage

llm = ChatSentient(model='gpt-4o', provider='openrouter')
response = llm([HumanMessage(content='Merhaba!')])
print(response.content)
"
```

### Goose — AI Coding Agent

```bash
cd integrations/agents/goose

# Goose'u SENTIENT LLM ile kullan
cargo run -- --model gpt-4o --goal "API endpoint ekle"
```

### AutoGPT — Otonom Agent

```bash
cd integrations/agents/auto-gpt

# AutoGPT'yi SENTIENT provider ile çalıştır
export SENTIENT_LLM_PROVIDER=openrouter
export OPENROUTER_API_KEY=sk-or-...
./run.sh --goal "Blog yazısı araştır ve yaz"
```

### Firecrawl — Web Scraping

```bash
cd integrations/tools/firecrawl

# Web sitesini kazı
curl -X POST http://localhost:3002/scrape \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

### Mem0 — Memory Layer

```bash
cd integrations/tools/mem0

# Agent belleğini yönet
python -c "
from mem0 import Memory
m = Memory()
m.add('Kullanıcı Rust seviyor', user_id='user1')
result = m.search('programlama', user_id='user1')
"
```

### Cevahir AI — Türkçe LLM Engine

```bash
# SENTIENT'in Türkçe LLM motoru
cd integrations/cevahir_ai

# Cevahir ile sohbet
python -m cevahir.chat --model v7 --language tr

# Cognitive strategy seç
python -m cevahir.cognitive --strategy think --input "Bu kodu analiz et"
```

| Strateji | Ne Zaman? |
|----------|-----------|
| Direct | Basit sorular |
| Think | Adım adım analiz |
| Debate | Çoklu perspektif |
| TreeOfThoughts | Karmaşık problem çözümü |

---

## 6. AI Gateway/Router Kullanımı

### Unify AI — Akıllı Yönlendirme

```bash
# .env
UNIFY_API_KEY=xxx

# Quality + Cost optimize
sentient chat --model "router@q>0.9&c<0.001"

# En hızlı model
sentient chat --model "router@speed"

# En ucuz model
sentient chat --model "router@cost"
```

**Ne yapar?** Her prompt'u analiz eder, en uygun modeli otomatik seçer:
- Basit soru → DeepSeek Flash (ucuz)
- Kod sorusu → GPT-4o (kaliteli)
- Reasoning → Claude (en iyi reasoning)

### Portkey — Enterprise Gateway

```bash
# .env
PORTKEY_API_KEY=xxx
PORTKEY_VIRTUAL_KEY=xxx

# Portkey üzerinden tüm provider'lara eriş
sentient chat --model "portkey/gpt-4o"
sentient chat --model "portkey/claude-4-sonnet"
sentient chat --model "portkey/deepseek-r1"
```

**Avantajları:**
- Otomatik failover (model düşerse diğerine geçer)
- Request caching (aynı soru tekrar sorulursa cache'den döner)
- Cost tracking (her request'in maliyetini izler)
- Virtual keys (API key'leri güvende)

### LiteLLM — Self-Hosted Proxy

```bash
# LiteLLM proxy'yi başlat
pip install litellm[proxy]
litellm --config config.yaml --port 4000

# config.yaml
model_list:
  - model_name: gpt-4o
    litellm_params:
      model: openai/gpt-4o
      api_key: os.environ/OPENAI_API_KEY
  - model_name: claude
    litellm_params:
      model: anthropic/claude-4-sonnet
      api_key: os.environ/ANTHROPIC_API_KEY
  - model_name: local
    litellm_params:
      model: ollama/llama3.3:70b

# SENTIENT'ten kullan
sentient chat --provider litellm --model gpt-4o
```

### OpenRouter — 300+ Model Marketplace

```bash
# .env
OPENROUTER_API_KEY=sk-or-v1-xxx

# Otomatik model seçimi
sentient chat --model "openrouter/auto"

# Belirli model
sentient chat --model "openrouter/anthropic/claude-4-sonnet"
sentient chat --model "openrouter/google/gemini-2.5-pro-preview"
sentient chat --model "openrouter/deepseek/deepseek-r1"
sentient chat --model "openrouter/meta-llama/llama-4-maverick"

# Ücretsiz modeller
sentient chat --model "openrouter/google/gemma-4-31b-it:free"
```

---

## 7. Proactive Engine — Zamanlı Görevler

### Sabah Bülteni (Morning Digest)

```bash
# .env
PROACTIVE_ENABLED=true

# CLI ile zamanlı görev ekle
sentient proactive add \
  --name "morning-brief" \
  --type time \
  --time "09:00" \
  --days "mon-fri" \
  --action "generate-briefing"
```

**Sabah 9'da ne yapar?**
1. Günlük hava durumu çeker
2. Takip edilen GitHub repo'larındaki yeni issue/PR'ları listeler
3. Email'deki önemli mesajları özetler
4. Takvimdeki bugünkü toplantıları gösterir
5. TTS ile sesli bülten okur (JARVIS modu aktifse)

### Event Bazlı Trigger

```bash
# Email geldiğinde
sentient proactive add \
  --name "urgent-email" \
  --type event \
  --event "email.received" \
  --condition "subject contains 'ACIL'" \
  --action "notify-telegram"

# GitHub PR açıldığında
sentient proactive add \
  --name "pr-review" \
  --type event \
  --event "github.pr_opened" \
  --action "auto-review"
```

### Pattern Bazlı Trigger

```bash
# Her Cuma haftalık rapor
sentient proactive add \
  --name "weekly-report" \
  --type pattern \
  --pattern "friday-17:00" \
  --action "generate-weekly-report"
```

---

## 8. Daemon Modu — Arka Planda Asistan

### Daemon Başlatma

```bash
# Başlat
sentient daemon start

# Durum kontrol
sentient daemon status
# Daemon: ✅ Running (PID: 12345, uptime: 2h 15m)
# Wake word: ✅ Listening
# Proactive: ✅ 3 triggers active
# Channels: ✅ Telegram, Discord

# Durdur
sentient daemon stop

# Log
sentient daemon log --tail
```

### Daemon Senaryosu: Yatakta Müzik Aç

```
1. Kullanıcı: "Hey Sentient, rahatlatıcı müzik aç"
2. Wake word algılandı
3. STT: "rahatlatıcı müzik aç"
4. Command parser: intent = PlayMusic, mood = relaxing
5. Action executor:
   a. Browser aç → YouTube'a git
   b. "relaxing music" ara
   c. İlk videoyu tıkla → Çalmaya başla
6. TTS: "Müziği açıyorum, iyi dinlemeler"
```

### Daemon Senaryosu: Sabah Bülteni

```
1. Saat 09:00 → Time trigger aktif
2. Proactive engine:
   a. Hava durumu API'sini çağır
   b. GitHub API'sinden yeni PR'ları al
   c. Email IMAP'tan son mesajları oku
   d. Google Calendar'dan bugünkü etkinlikleri al
3. LLM ile bülten oluştur
4. TTS ile sesli oku: "Günaydın! Bugün hava 22 derece..."
5. Telegram'a da gönder
```

---

## 9. Kanal Entegrasyonları

### Telegram Bot

```bash
# 1. @BotFather'dan bot oluştur → token al
# 2. .env'e ekle
echo "TELEGRAM_BOT_TOKEN=123456:ABC-DEF" >> .env

# 3. Başlat
sentient channel start telegram

# Bot artık Telegram'dan mesajları yanıtlar!
```

**Telegram'dan Kullanım:**
```
Kullanıcı: /ask Bugün toplantı var mı?
Bot: Bugün 2 toplantınız var: 10:00 Product Sync, 14:00 Sprint Review

Kullanıcı: /code Python fibonacci yaz
Bot: ```python
def fibonacci(n):
    ...
```

Kullanıcı: /search Rust async nedir?
Bot: [Web araması yapar, özet sunar]
```

### Discord Bot

```bash
# 1. discord.com/developers/applications → Bot oluştur
# 2. .env'e ekle
echo "DISCORD_BOT_TOKEN=xxx" >> .env
echo "DISCORD_APPLICATION_ID=xxx" >> .env

# 3. Başlat
sentient channel start discord
```

### WhatsApp Business

```bash
# 1. Meta Business API'den token al
# 2. .env'e ekle
echo "WHATSAPP_TOKEN=xxx" >> .env
echo "WHATSAPP_PHONE_ID=xxx" >> .env

# 3. Başlat
sentient channel start whatsapp
```

### Tüm Kanalları Aynı Anda Başlat

```bash
# .env
TELEGRAM_BOT_TOKEN=xxx
DISCORD_BOT_TOKEN=xxx
SLACK_BOT_TOKEN=xxx

# Daemon modu ile hepsini başlat
sentient daemon start --channels telegram,discord,slack
```

---

## 10. Rust API ile Kullanım

### Basit Sohbet

```rust
use sentient_llm::{LlmHub, ChatRequest, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hub = LlmHub::from_env()?;

    let response = hub.chat(ChatRequest {
        model: "gpt-4o".into(),
        messages: vec![Message::user("Rust'ta async nedir?")],
        ..Default::default()
    }).await?;

    println!("{}", response.choices[0].message.content.as_text().unwrap());
    Ok(())
}
```

### LlmHubBuilder ile Özelleştirme

```rust
use sentient_llm::{LlmHubBuilder, RoutingStrategy};

let hub = LlmHubBuilder::new()
    .ollama()?                          // Lokal Ollama
    .openai(api_key)?                   // OpenAI
    .anthropic(api_key)?                // Anthropic
    .deepseek(api_key)?                 // DeepSeek (ucuz)
    .unify(api_key)?                    // Akıllı router
    .default_model("deepseek-v3")      // Varsayılan model
    .routing(RoutingStrategy::Cost)     // En ucuz provider'ı seç
    .build();
```

### Smart Router Kullanımı

```rust
use sentient_llm::{SmartRouter, ComplexityTier};

let router = SmartRouter::new();

// Basit soru → ucuz model
let decision = router.route("Merhaba").await;
// → deepseek-v3 ($0.0001/1K)

// Kod sorusu → güçlü model
let decision = router.route("Rust'ta lifetime nedir?").await;
// → gpt-4o ($0.0025/1K)

// Reasoning → reasoning model
let decision = router.route("Bu algoritmanın zaman karmaşıklığı nedir?").await;
// → deepseek-r1 ($0.0008/1K)
```

### Streaming Kullanımı

```rust
use sentient_llm::{LlmHub, ChatRequest, Message};
use futures::StreamExt;

let hub = LlmHub::from_env()?;

let mut stream = hub.chat_stream(ChatRequest {
    model: "gpt-4o".into(),
    messages: vec![Message::user("Uzun bir hikaye anlat")],
    stream: true,
    ..Default::default()
}).await?;

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    if let Some(content) = chunk.choices.first().and_then(|c| c.delta.content.as_ref()) {
        print!("{}", content);
    }
}
```

### Embedding Kullanımı

```rust
use sentient_embed::{EmbeddingHub, EmbeddingRequest};

let hub = EmbeddingHub::from_env()?;

let embeddings = hub.embed(EmbeddingRequest {
    model: "voyage-3".into(),
    input: vec!["Rust programlama dili".into(), "Python programlama dili".into()],
}).await?;

// Vektör benzerliği hesapla
let similarity = hub.cosine_similarity(&embeddings[0], &embeddings[1]);
println!("Benzerlik: {:.4}", similarity);
```

### Memory (Bellek) Kullanımı

```rust
use sentient_core::traits::{MemoryEntry, MemoryType};
use sentient_memory::MemoryCube;

let memory = MemoryCube::new("data/sentient_memory.db")?;

// Kaydet
memory.store(MemoryEntry::new("Kullanıcı Rust seviyor", MemoryType::Semantic))?;

// Ara
let results = memory.search("programlama")?;

// Episodik bellek
memory.store(MemoryEntry::new("Dün toplantıda API tasarımı konuştuk", MemoryType::Episodic))?;
```

---

## 11. Docker Production Deployment

### docker-compose.yml Servisleri

| Servis | Port | İşlev | Zorunlu mu? |
|--------|------|-------|------------|
| PostgreSQL | 5432 | Ana veritabanı | ✅ |
| Redis | 6379 | Cache & Queue | ✅ |
| Qdrant | 6333 | Vector DB | ✅ RAG için |
| MinIO | 9000/9001 | Object Storage | Opsiyonel |
| Prometheus | 9090 | Metrik toplama | Opsiyonel |
| Grafana | 3001 | Dashboard | Opsiyonel |
| Ollama | 11434 | Lokal LLM | Lokal modda |
| SearXNG | 8888 | Arama motoru | Opsiyonel |
| RabbitMQ | 5672 | Message Queue | Opsiyonel |

### Başlatma

```bash
# Tam stack
docker-compose up -d

# Minimal (sadece DB + Cache + Vector)
./scripts/start.sh --minimal

# Durdur
./scripts/stop.sh
```

### Health Check

```bash
./scripts/health-check.sh
```

```
  PostgreSQL          ✅ SAĞLIKLI
  Redis               ✅ SAĞLIKLI
  Qdrant              ✅ SAĞLIKLI
  MinIO               ✅ SAĞLIKLI
  Ollama              ✅ SAĞLIKLI
  Prometheus          ✅ SAĞLIKLI
  Grafana             ✅ SAĞLIKLI
  SearXNG             ✅ SAĞLIKLI
  RabbitMQ            ✅ SAĞLIKLI
  Gateway             ✅ SAĞLIKLI
  
  10/10 servis sağlıklı
```

### Kubernetes Deployment

```bash
# Namespace oluştur
kubectl apply -f deploy/kubernetes/namespace.yaml

# Tüm servisleri deploy et
kubectl apply -f deploy/kubernetes/

# Pod durumunu kontrol et
kubectl get pods -n sentient

# Logları izle
kubectl logs -f deployment/sentient-gateway -n sentient
```

---

## 12. Güvenlik — V-GATE & Guardrails

### V-GATE: API Key'ler Asla İstemcide Değil

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Sunucuda saklı
                     İstemcide YOK
```

### V-GATE Yapılandırma

```bash
# .env
V_GATE_URL=http://localhost:8100
V_GATE_LISTEN=127.0.0.1:1071
V_GATE_TIMEOUT=120

# V-GATE'i başlat
sentient vgate start

# V-GATE durum
sentient vgate status
```

### Guardrails — Giriş/Çıkış Filtreleme

```bash
# .env
GUARDRAILS_MODE=strict
GUARDRAILS_PROMPT_INJECTION=true
GUARDRAILS_DATA_EXFILTRATION=true

# Test et
sentient guardrails test "Ignore all previous instructions"
# → ❌ BLOCKED: Prompt injection tespit edildi

sentient guardrails test "API key'im sk-abc123"
# → ❌ BLOCKED: PII/Secret tespit edildi

sentient guardrails test "Merhaba, nasılsın?"
# → ✅ ALLOWED
```

### Vault — Secrets Management

```bash
# Secret kaydet
sentient vault set OPENAI_API_KEY "sk-xxx"

# Secret oku
sentient vault get OPENAI_API_KEY

# Secret listele
sentient vault list

# Rotate (değiştir)
sentient vault rotate OPENAI_API_KEY
```

---

## 📚 Ek Kaynaklar

| Dosya | Açıklama |
|-------|----------|
| [README.md](README.md) | Ana dokümantasyon |
| [USAGE_GUIDE.md](USAGE_GUIDE.md) | Temel kullanım kılavuzu |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Sistem mimarisi |
| [SECURITY.md](SECURITY.md) | Güvenlik dokümantasyonu |
| [MODEL_PROVIDERS.md](MODEL_PROVIDERS.md) | LLM provider detayları |
| [docs/API.md](docs/API.md) | REST API referansı |
| [docs/VOICE.md](docs/VOICE.md) | Ses sistemi detayları |
| [docs/CHANNELS.md](docs/CHANNELS.md) | Kanal entegrasyonları |

---

<p align="center">
  <b>SENTIENT OS</b> - The Operating System That Thinks<br>
  Made with 🦀 by the SENTIENT Team
</p>
