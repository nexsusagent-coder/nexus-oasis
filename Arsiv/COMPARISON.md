# SENTIENT vs OpenClaw Karşılaştırması

## Hızlı Özet

| Özellik | OpenClaw | SENTIENT | Durum |
|---------|----------|----------|-------|
| **Kurulum** | `npm install -g openclaw` | `cargo build` (5-10 dk) | 🔴 Gereksinim |
| **Binary Releases** | Var (npm, dmg, apk) | Yok | 🔴 Kritik |
| **Onboarding** | Interactive wizard | TUI wizard | 🟡 Geliştirilecek |
| **Channels** | 50+ | Gateway var | 🟡 Entegrasyonlar eksik |
| **Voice** | Voice Wake + Talk Mode | Yok | 🔴 Planlanmalı |
| **Mobile Apps** | iOS/Android | Yok | 🔴 Gelecek |
| **Skills Marketplace** | ClawHub | Yok | 🟡 Planlanmalı |
| **Docs** | Kapsamlı | Temel | 🟡 Geliştirilecek |

---

## OpenClaw'dan Alınacak Dersler

### 1. Kurulum Basitliği

**OpenClaw:**
```bash
npm install -g openclaw@latest
openclaw onboard --install-daemon
# 30 saniye, hazır!
```

**SENTIENT (Hedef):**
```bash
# Seçenek 1: Binary indir (önerilen)
curl -sSL https://get.sentient.ai | bash

# Seçenek 2: npm (Node.js olanlar için)
npm install -g @sentient/ai

# Seçenek 3: Cargo (Rust geliştiricileri için)
cargo install sentient
```

### 2. Onboarding Wizard

**OpenClaw Onboarding:**
- Model seçimi (OAuth/API key)
- Channel konfigürasyonu
- Skills kurulumu
- Daemon kurulumu
- Security ayarları

**SENTIENT Gereksinimleri:**
- [ ] Model seçimi (var, iyileştirilecek)
- [ ] Channel konfigürasyonu (yok)
- [ ] API key yönetimi (var)
- [ ] Security policy ayarları (var)
- [ ] First-run experience (yok)

### 3. Channel Entegrasyonları

**OpenClaw Channels:**
- WhatsApp (Baileys)
- Telegram (grammY)
- Discord (discord.js)
- Slack (Bolt)
- Signal (signal-cli)
- iMessage / BlueBubbles
- Matrix, IRC, Teams, vb.

**SENTIENT Hedefleri (Öncelik Sırası):**
1. **Telegram** - Bot API ile kolay
2. **Discord** - Webhook ile hızlı
3. **WhatsApp** - Baileys kullanılabilir
4. **Slack** - Bolt benzeri
5. **Signal** - signal-cli entegrasyonu

### 4. Voice & Audio

**OpenClaw:**
- Voice Wake (wake word detection)
- Talk Mode (continuous voice)
- ElevenLabs + System TTS fallback

**SENTIENT Hedefleri:**
- [ ] Whisper entegrasyonu (STT)
- [ ] TTS modülü (System/ElevenLabs)
- [ ] Wake word detection (Porcupine/snowboy)

### 5. Skills Platform

**OpenClaw ClawHub:**
- Community skills marketplace
- Skill install gating
- Workspace skills
- Bundled skills

**SENTIENT Hedefleri:**
- [ ] Skills marketplace
- [ ] Skill discovery
- [ ] Skill rating/reviews
- [ ] Community contributions

---

## Aksiyon Planı

### Faz 1: Kurulum (1 Hafta)
- [ ] GitHub Releases workflow (binary dağıtımı)
- [ ] `get.sentient.ai` domain ve redirect
- [ ] npm package (@sentient/ai)
- [ ] Homebrew formula (macOS)
- [ ] AUR package (Arch Linux)

### Faz 2: Onboarding (1 Hafta)
- [ ] İyileştirilmiş TUI wizard
- [ ] Channel konfigürasyon adımları
- [ ] First-run experience
- [ ] Daemon/service kurulumu

### Faz 3: Channels (2 Hafta)
- [ ] Telegram bot entegrasyonu
- [ ] Discord webhook entegrasyonu
- [ ] WhatsApp Business API
- [ ] Generic webhook sistemi

### Faz 4: Voice (1 Hafta)
- [ ] Whisper STT entegrasyonu
- [ ] TTS modülü
- [ ] Voice command sistemi

### Faz 5: Marketplace (2 Hafta)
- [ ] Skills registry
- [ ] Skill discovery UI
- [ ] Community contributions

---

## Metrikler

### OpenClaw
- ⭐ 352,974 stars
- 🍴 71,210 forks
- 📦 npm weekly downloads: ~50K

### SENTIENT (Hedefler)
- ⭐ 1,000+ stars (İlk ay)
- 📦 npm weekly downloads: 100+ (İlk ay)
- 🚀 Binary downloads: 500+ (İlk ay)

---

## Sonuç

SENTIENT'in Rust tabanı güçlü ama kullanıcı deneyimi OpenClaw kadar kolay değil. Öncelik:

1. **Binary dağıtımı** - Kritik
2. **Basit kurulum** - `curl | bash` ile 30 saniye
3. **İyi onboarding** - Adım adım rehber
4. **Channel entegrasyonları** - Telegram/Discord öncelikli
5. **Docs** - Kapsamlı dokümantasyon