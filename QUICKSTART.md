# 🚀 SENTIENT OS - Hızlı Başlangıç

## 📋 Gereksinimler

- **Docker** (20.10+): https://docs.docker.com/get-docker/
- **Docker Compose** (v2): Docker ile birlikte gelir
- **Rust** (1.75+): https://rustup.rs/
- **Ollama** (Optional, Local LLM için): https://ollama.ai/

### Sistem Gereksinimleri

| Mod | RAM | VRAM | Açıklama |
|-----|-----|------|----------|
| Minimal | 8 GB | - | API-only, Cloud LLM |
| Standard | 16 GB | 8 GB | Local LLM (küçük modeller) |
| Full | 32 GB | 24 GB+ | Büyük modeller (27B-70B) |

---

## ⚡ Hızlı Başlangıç

### 1. .env Dosyasını Oluşturun

```bash
cd SENTIENT_CORE
cp .env.template .env
```

**Gerekli API Key'leri** (en az biri):
- `OPENROUTER_API_KEY` - Önerilen (42+ model)
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`

```bash
# .env dosyasını düzenleyin
nano .env
```

### 2. Sistemi Başlatın

```bash
# Tüm servisleri başlat
./scripts/start.sh

# veya sadece temel servisler
./scripts/start.sh --minimal
```

### 3. Health Check

```bash
./scripts/health-check.sh
```

### 4. Erişim

| Servis | URL |
|--------|-----|
| **Dashboard** | http://localhost:8080/dashboard |
| **API** | http://localhost:8080 |
| **GraphQL** | http://localhost:8080/graphql |
| **Grafana** | http://localhost:3001 |

---

## 🦙 Local LLM (Ollama)

GPU'nuz varsa, local LLM kullanarak $0 maliyetle çalıştırabilirsiniz:

```bash
# Ollama kurulumu (Linux/macOS)
curl -fsSL https://ollama.ai/install.sh | sh

# Modelleri indir
./scripts/setup-ollama.sh

# veya manuel
ollama pull gemma3:27b
ollama pull deepseek-r1:67b
```

---

## 📁 Proje Yapısı

```
SENTIENT_CORE/
├── crates/                 # Rust modülleri
│   ├── sentient_gateway/   # API Gateway
│   ├── sentient_llm/       # LLM entegrasyonu
│   ├── sentient_voice/     # Ses işleme
│   ├── sentient_channels/  # Mesaj platformları
│   ├── sentient_agents/    # Ajan sistemi
│   └── ...                 # 30+ crate
├── dashboard/              # Web dashboard
├── scripts/                # Yardımcı scriptler
├── config/                 # Konfigürasyon
├── docker-compose.yml      # Docker servisleri
├── .env.template           # Ortam değişkenleri
└── Cargo.toml              # Rust workspace
```

---

## 🔧 Komutlar

```bash
# Başlat
./scripts/start.sh

# Durdur
./scripts/stop.sh

# Health check
./scripts/health-check.sh

# Ollama kurulumu
./scripts/setup-ollama.sh

# Derleme
cargo build --release

# Test
cargo test

# Lint
cargo clippy
```

---

## 🌐 Desteklenen Platformlar

### Mesaj Kanalları (14)
Telegram, Discord, Slack, WhatsApp, Signal, Messenger, Instagram, Twitter/X, Teams, Google Chat, LINE, LinkedIn, WeChat, iMessage

### LLM Provider'lar (42+)
OpenAI, Anthropic, Google AI, Meta Llama, Mistral, DeepSeek, Groq, Together AI, Cohere, xAI, Amazon Bedrock, Azure OpenAI...

### Özellikler
- ✅ Multi-agent orchestration
- ✅ Voice control (STT/TTS)
- ✅ Desktop automation
- ✅ Smart home integration
- ✅ Calendar & Email
- ✅ Web scraping
- ✅ Code generation
- ✅ Image generation

---

## 🐛 Sorun Giderme

### PostgreSQL bağlanamıyor
```bash
docker logs sentient-postgres
docker-compose restart postgres
```

### Port kullanımda
```bash
# Port'u kontrol et
lsof -i :8080

# .env'de portu değiştir
GATEWAY_PORT=8081
```

### Ollama GPU hatası
```bash
# NVIDIA driver kontrol
nvidia-smi

# CUDA kurulumu
# https://developer.nvidia.com/cuda-downloads
```

---

## 📚 Dokümantasyon

- [Master Yapılacaklar](ARSIV_MASTER_YAPILACAKLAR.md)
- [Günlük Rapor](Arsiv/GUNLUK_RAPOR_2026-04-14.md)
- [Sistem Analizi](Arsiv/SENTIENT_CORE_GENEL_ANALIZ.md)
- [Kurulum Rehberi](Arsiv/SISTEMI_AYAGA_KALDIRMA_REHBERI.md)

---

## 📞 Destek

- GitHub Issues: https://github.com/nexsusagent-coder/SENTIENT_CORE/issues
- Discord: (Coming soon)

---

**🧠 SENTIENT OS** - Your AI Operating System
