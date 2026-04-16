# 🚀 SENTIENT OS - Hızlı Başlangıç (5 Dakika)

> **Sıfırdan çalışan bir AI asistan — en hızlı yol**

---

## ⚡ 3 Adımda Başla

### Adım 1: Kur

```bash
curl -fsSL https://raw.githubusercontent.com/nexsusagent-coder/SENTIENT_CORE/main/install.sh | bash
```

### Adım 2: LLM Seç

```bash
# Seçenek A: Ücretsiz (Lokal)
curl -fsSL https://ollama.com/install.sh | sh
ollama pull gemma3:27b

# Seçenek B: API Key ile (Daha iyi)
export OPENROUTER_API_KEY=sk-or-v1-xxx  # https://openrouter.ai/keys
```

### Adım 3: Başlat

```bash
sentient chat
```

**Bitti!** 🎉

---

## 🧠 Ne Yapabilirsin?

| Özellik | Komut | Açıklama |
|---------|-------|----------|
| **Sohbet** | `sentient chat` | İnteraktif AI sohbeti |
| **Tek soru** | `sentient ask "Soru?"` | Hızlı soru-cevap |
| **Kod yaz** | `sentient code "REST API yaz"` | Kod üretimi |
| **Sesli** | `sentient voice` | JARVIS modu |
| **Otonom** | `sentient desktop --goal "..."` | Tam otonom agent |
| **Daemon** | `sentient daemon start` | 7/24 arka plan |
| **Kanal** | `sentient channel start telegram` | Telegram bot |
| **Gateway** | `sentient gateway` | REST API server |

---

## 🔑 LLM Seçenekleri

| Seçenek | Maliyet | Kalite | Hız |
|---------|---------|--------|-----|
| **Ollama (lokal)** | Free | ⭐⭐⭐⭐ | Donanıma bağlı |
| **OpenRouter Free** | Free | ⭐⭐⭐⭐ | Hızlı |
| **DeepSeek** | $0.27/1M | ⭐⭐⭐⭐⭐ | Hızlı |
| **GPT-4o** | $30/1M | ⭐⭐⭐⭐⭐ | Orta |
| **Claude 4** | $15/1M | ⭐⭐⭐⭐⭐ | Orta |
| **Groq** | Free tier | ⭐⭐⭐⭐ | **EN HIZLI** |

---

## 📁 Proje Yapısı

```
SENTIENT_CORE/
├── crates/              # 93 Rust crate
│   ├── sentient_llm/    # 57+ provider, 245+ model
│   ├── sentient_voice/  # STT, TTS, Wake Word
│   ├── sentient_channels/ # 20+ platform
│   ├── sentient_orchestrator/ # Multi-agent
│   ├── sentient_daemon/  # 7/24 arka plan
│   ├── sentient_proactive/ # Zamanlı görevler
│   ├── sentient_home/    # Akıllı ev
│   ├── sentient_cevahir/ # Türkçe LLM
│   ├── oasis_autonomous/ # Otonom agent
│   ├── oasis_hands/     # 43+ tool
│   └── ...              # +80 crate daha
├── integrations/        # 72+ entegre proje
├── skills/             # 5,587+ skill
└── docs/               # Dokümantasyon
```

---

## 🛠️ Sistem Gereksinimleri

| Mod | RAM | VRAM | Disk |
|-----|-----|------|------|
| **API-only** | 8 GB | - | 20 GB |
| **Lokal küçük** | 16 GB | 8 GB | 50 GB |
| **Lokal büyük** | 32 GB | 24 GB | 100 GB |

---

## 📚 Detaylı Dokümantasyon

| Dosya | Açıklama |
|-------|----------|
| [INSTALL.md](INSTALL.md) | Kapsamlı kurulum |
| [INSTALL_GUIDE.md](INSTALL_GUIDE.md) | Universal kurulum (tüm platformlar) |
| [USAGE_GUIDE.md](USAGE_GUIDE.md) | Detaylı kullanım kılavuzu |
| [docs/USAGE_SCENARIOS.md](docs/USAGE_SCENARIOS.md) | Gerçek dünya senaryoları |
| [MODEL_PROVIDERS.md](MODEL_PROVIDERS.md) | 57+ provider rehberi |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Sistem mimarisi |

---

*🧠 SENTIENT OS - The Operating System That Thinks*
