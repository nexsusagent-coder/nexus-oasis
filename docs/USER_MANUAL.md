# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT NEXUS OS v1.1.0 - KULLANICI KILAVUZU
#  Universal Omni-Gateway & Full Channel Support
# ═══════════════════════════════════════════════════════════════════════════════

```
    🐺 SENTIENT NEXUS OS - Universal AI Operations Platform
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
    
    🌐 Universal LLM Gateway - HERHANGİ bir AI modeline bağlanın
    💬 20+ Mesajlaşma Kanalı - WhatsApp, Signal, iMessage, Matrix ve dahası
    🤖 5.587 Otonom Skill - Tam otomatik AI işlemleri
    🔐 Agent-S3 Hardware Permissions - Klavye/Mouse kontrolü
    
    ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
```

---

## 📋 İÇİNDEKİLER

1. [Giriş](#1-giriş)
2. [Kurulum Sihirbazı](#2-kurulum-sihirbazı)
3. [Universal LLM Gateway](#3-universal-llm-gateway)
4. [20+ Mesajlaşma Kanalı](#4-20-mesajlaşma-kanalı)
5. [Agent-S3 Hardware Permissions](#5-agent-s3-hardware-permissions)
6. [5.587 Otonom Skill](#6-5587-otonom-skill)
7. [Tam Otonom Mod](#7-tam-otonom-mod)
8. [API Entegrasyonları](#8-api-entegrasyonları)
9. [Kullanım Örnekleri](#9-kullanım-örnekleri)
10. [Sıkça Sorulan Sorular](#10-sıkça-sorulan-sorular)

---

## 1. GİRİŞ

### 1.1 SENTIENT Nedir?

**SENTIENT NEXUS OS**, Rust çekirdeğine sahip, otonom, güvenli ve yüksek performanslı bir Yapay Zeka İşletim Sistemi'dir. Açık kaynak projelerin uyumlu çalışmasını sağlayan bir **entegrasyon platformu** olarak tasarlanmıştır.

### 1.2 Temel Özellikler

| Özellik | Açıklama |
|---------|----------|
| 🌐 **Universal LLM Gateway** | Herhangi bir OpenAI/Anthropic uyumlu API'ye bağlanın |
| 💬 **20+ Mesajlaşma Kanalı** | WhatsApp, Signal, iMessage, Matrix, MS Teams ve dahası |
| 🤖 **5.587 Otonom Skill** | Dosya, web, kod, veri, sistem işlemleri |
| 🔐 **Agent-S3** | Klavye/Mouse kontrolü ile desktop automation |
| 🧠 **Bilişsel Bellek** | Episodik, semantik, prosedürel bellek |
| 🛡️ **V-GATE** | API anahtarları için güvenli proxy |
| 🔒 **Guardrails** | Prompt injection ve veri sızıntısı koruması |

### 1.3 Sistem Gereksinimleri

```
┌─────────────────────────────────────────────────────────┐
│  Minimum                        │  Önerilen             │
├─────────────────────────────────┼───────────────────────┤
│  CPU: 4 çekirdek               │  CPU: 8+ çekirdek     │
│  RAM: 8 GB                      │  RAM: 16+ GB          │
│  Disk: 20 GB                    │  Disk: 50+ GB SSD     │
│  OS: Linux/macOS/Windows        │  OS: Ubuntu 22.04+    │
│  Rust: 1.75+                    │  Rust: 1.80+          │
└─────────────────────────────────────────────────────────┘
```

### 1.4 Hızlı Başlangıç

```bash
# 1. Repoyu klonla
git clone https://github.com/sentient-ai/sentient-nexus-os.git
cd sentient-nexus-os

# 2. Otomatik kurulum
./setup.sh

# 3. SENTIENT'yı başlat
make run

# Dashboard: http://localhost:8080
```

---

## 2. KURULUM SİHİRBAZI

### 2.1 İlk Başlatma

SENTIENT'yı ilk kez başlattığınızda, kurulum sihirbazı otomatik olarak çalışır:

```bash
# SENTIENT'yı başlat
make run

# veya
cargo run --release
```

### 2.2 Kurulum Adımları

```
╔════════════════════════════════════════════════════════════════════════════════╗
║  🐺 SENTIENT NEXUS OS - UNIVERSAL OMNI-GATEWAY v1.1.0                            ║
╚════════════════════════════════════════════════════════════════════════════════╝

→ [1/8] Dil Seçimi
→ [2/8] LLM Provider Yapılandırması
→ [3/8] 🎯 Custom Provider (Universal Gateway)
→ [4/8] API Anahtarları
→ [5/8] 💬 Mesajlaşma Kanalları (20+ Platform)
→ [6/8] 🏢 Enterprise Entegrasyonları
→ [7/8] 🔐 AGENT-S3 Hardware Permissions
→ [8/8] Kurulum Tamamlanıyor
```

### 2.3 Hızlı Kurulum

Mevcut ayarları kullanarak hızlı kurulum:

```bash
# Mevcut config ile başlat
sentient run --config /path/to/config.toml

# Varsayılan ayarlarla başlat
sentient run --default
```

---

## 3. UNIVERSAL LLM GATEWAY

### 3.1 Konsept

SENTIENT, **herhangi bir OpenAI veya Anthropic uyumlu API'ye** bağlanabilir. Bu sayede:

- ✅ OpenAI (GPT-4, GPT-3.5)
- ✅ Anthropic (Claude 3 Opus/Sonnet/Haiku)
- ✅ Google AI (Gemini)
- ✅ Groq (Ultra hızlı inferencing)
- ✅ Mistral AI (Mixtral, Mistral)
- ✅ Together AI (Open source models)
- ✅ DeepSeek (Code specialized)
- ✅ Ollama (Yerel, ücretsiz)
- ✅ LM Studio (Yerel)
- ✅ vLLM (Yerel high-performance)
- ✅ AWS Bedrock
- ✅ Azure OpenAI
- ✅ Alibaba Qwen
- ✅ Baidu Ernie
- ✅ Moonshot AI
- ✅ **VE HERHANGİ BİR OpenAI-compatible API!**

### 3.2 Önceden Tanımlı Provider'lar

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  Provider          │ Base URL                              │ Format         │
├────────────────────┼───────────────────────────────────────┼────────────────┤
│  Together AI       │ https://api.together.xyz/v1          │ OpenAI         │
│  Groq              │ https://api.groq.com/openai/v1       │ OpenAI         │
│  Fireworks AI      │ https://api.fireworks.ai/inference/v1│ OpenAI         │
│  Perplexity AI     │ https://api.perplexity.ai            │ OpenAI         │
│  DeepSeek          │ https://api.deepseek.com/v1          │ OpenAI         │
│  Mistral AI        │ https://api.mistral.ai/v1            │ OpenAI         │
│  OpenRouter        │ https://openrouter.ai/api/v1         │ OpenAI         │
│  Ollama (Local)    │ http://localhost:11434/v1            │ OpenAI         │
│  LM Studio (Local) │ http://localhost:1234/v1             │ OpenAI         │
│  vLLM (Local)      │ http://localhost:8000/v1             │ OpenAI         │
│  Anthropic         │ https://api.anthropic.com            │ Anthropic      │
│  Alibaba Qwen      │ https://dashscope.aliyuncs.com/api/v1│ OpenAI         │
│  Moonshot          │ https://api.moonshot.cn/v1           │ OpenAI         │
│  Replicate         │ https://api.replicate.com/v1         │ OpenAI         │
│  Anyscale          │ https://api.anyscale.com/v1          │ OpenAI         │
│  Aleph Alpha       │ https://api.aleph-alpha.com/v1       │ Custom         │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 3.3 Custom Provider Ekleme

**Adım 1: Kurulum sihirbazında "Custom Provider" seçin**

```
→ [2/8] LLM Provider Yapılandırması
  🎯 Custom Provider (HERHANGİ Bir API!)  ← Bu seçeneği seçin
```

**Adım 2: API formatını belirleyin**

```
API Formatı seçin:
  1. OpenAI Compatible (/chat/completions)    ← Çoğu provider için
  2. Anthropic Compatible (/messages)
  3. Custom Endpoint
```

**Adım 3: Base URL girin**

```
📌 Base URL örnekleri:
   OpenAI:    https://api.openai.com/v1
   Together:  https://api.together.xyz/v1
   Groq:      https://api.groq.com/openai/v1
   DeepSeek:  https://api.deepseek.com/v1
   Ollama:    http://localhost:11434/v1

Base URL: https://api.your-provider.com/v1
```

**Adım 4: API Key girin**

```
API Key (boş bırakabilirsiniz): sk-xxxxxxxxxxxxxxxx
```

**Adım 5: Model seçin**

```
Model adı (ör: gpt-4, claude-3-opus, mixtral-8x7b): llama-3-70b
```

### 3.4 Programatik Kullanım

```rust
use sentient_settings::{CustomProviderConfig, ApiFormat, ModelInfo};

// Custom provider oluştur
let provider = CustomProviderConfig::new("MyProvider", "https://api.custom.ai/v1")
    .with_api_key("sk-xxx")
    .with_format(ApiFormat::OpenAI)
    .with_model(ModelInfo {
        id: "custom-model-70b".to_string(),
        name: "Custom Model 70B".to_string(),
        context_length: 32768,
        pricing: None,
        capabilities: vec!["chat".to_string(), "code".to_string()],
    });

// Chat URL'ini al
let url = provider.chat_url();
// => "https://api.custom.ai/v1/chat/completions"
```

### 3.5 Config Dosyasından

```toml
# ~/.config/sentient/config.toml

[llm]
provider = "custom"
model = "llama-3-70b"
base_url = "https://api.custom.ai/v1"
api_format = "openai"

[[custom_providers]]
name = "CustomAI"
base_url = "https://api.custom.ai/v1"
api_format = "openai"
default_model = "llama-3-70b"
enabled = true

[api_keys.extra]
CUSTOMAI_API_KEY = "sk-xxxxxxxx"
```

### 3.6 Yerel LLM Kurulumu

#### Ollama (Önerilen)

```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh

# Model indir
ollama pull qwen2.5-coder:7b    # Kod için
ollama pull llama3.1:8b         # Genel için
ollama pull mistral-nemo:12b    # Dengeli

# SENTIENT'da Ollama seç
# Base URL: http://localhost:11434/v1
# Model: qwen2.5-coder:7b
```

#### LM Studio

```bash
# 1. LM Studio'yu indir: https://lmstudio.ai
# 2. Bir model yükle (ör: Qwen2.5-Coder-7B)
# 3. Local Server başlat (port 1234)
# 4. SENTIENT'da:
#    Base URL: http://localhost:1234/v1
#    Model: (model adı)
```

#### vLLM (High-Performance)

```bash
# vLLM kur
pip install vllm

# Server başlat
python -m vllm.entrypoints.openai.api_server \
    --model meta-llama/Llama-3.1-8B-Instruct \
    --host 0.0.0.0 \
    --port 8000

# SENTIENT'da:
# Base URL: http://localhost:8000/v1
```

---

## 4. 20+ MESAJLAŞMA KANALI

### 4.1 Desteklenen Platformlar

SENTIENT, **dünyanın en kapsamlı mesajlaşma entegrasyonuna** sahiptir:

#### 📱 MOBİL MESSENGER'LAR

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **WhatsApp Business API** | Metin, Resim, Video, Audio, Dosya, Location, Buttons | Meta Business Manager |
| **Signal Messenger** | E2E Şifreli, Metin, Media | signal-cli veya GUI Automation |
| **Telegram Bot** | Metin, Media, Buttons, Inline, Groups | @BotFather |
| **iMessage** (macOS) | Metin, Media, Groups | GUI Automation (Agent-S3) |
| **WeChat / 企业微信** | Metin, Media, Enterprise | WeChat Work Portal |
| **LINE Messenger** | Metin, Media, Flex Messages | LINE Developers |
| **Viber** | Metin, Media | Viber Admin Panel |
| **KakaoTalk** | Metin, Media | Kakao Developers |

#### 🏢 ENTERPRISE PLATFORM'LAR

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **Microsoft Teams** | Metin, Cards, Threads, Calls | Azure Bot veya Webhook |
| **Slack** | Metin, Blocks, Threads, Files | Slack App |
| **Google Chat** | Metin, Cards, Threads | Google Chat API |
| **Discord** | Metin, Embeds, Threads, Voice | Discord Developer Portal |
| **Cisco Webex** | Metin, Cards, Video | Webex Developer Portal |
| **Zoom Chat** | Metin, Files | Zoom Marketplace |
| **Mattermost** | Metin, Attachments | Bot Token |
| **RocketChat** | Metin, Attachments | Personal Access Token |

#### 🔐 DECENTRALIZED / FEDERATED

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **Matrix (Element)** | Metin, Media, E2E, Federated | Homeserver + Access Token |
| **XMPP/Jabber** | Metin, OMEMO Encryption | JID + Password |
| **Session** | Anonim, E2E | GUI Automation |
| **Wire** | E2E Şifreli | Email + Password |
| **Threema** | E2E Şifreli | Threema Work API |

#### 📱 SOSYAL PLATFORM'LAR

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **Twitter/X DM** | Direct Messages | Twitter Developer Portal |
| **Instagram DM** | Direct Messages | Meta for Developers |
| **Facebook Messenger** | Metin, Media, Templates | Meta Page Token |
| **LinkedIn Messaging** | Direct Messages | LinkedIn Developers |
| **Reddit Chat** | Direct Messages | Reddit Apps |

#### 📧 EMAIL & SMS

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **Email (SMTP)** | HTML, Attachments | SMTP Server |
| **SMS (Twilio)** | Metin, Media | Twilio Console |
| **RCS Messaging** | Rich Cards, Suggestions | Google Business Messages |

#### 🐙 DEVELOPER TOOLS

| Platform | Özellikler | Kurulum |
|----------|------------|---------|
| **GitHub** | Issues, PRs, Comments | Personal Access Token |
| **GitLab** | Issues, MRs | Personal Access Token |
| **Jira** | Issues, Comments | API Token |
| **PagerDuty** | Alerts, Incidents | API Token |

### 4.2 Kanal Kurulum Örnekleri

#### WhatsApp Business API

```bash
# 1. Meta Business Manager'da WhatsApp hesabı oluşturun
# https://business.facebook.com

# 2. Kurulum sihirbazında:
→ [5/8] 💬 Mesajlaşma Kanalları
  📱 Mobile Messengers
    WhatsApp Business API

Phone Number ID: 123456789012345
Permanent Access Token: EAAxxxxxxxx...

✅ WhatsApp bağlandı!
```

**Örnek Kullanım:**

```rust
use sentient_channels::whatsapp::WhatsAppClient;

let client = WhatsAppClient::new(phone_id, access_token);

// Mesaj gönder
client.send_text(
    "+1234567890",
    "Merhaba! Bu mesaj SENTIENT tarafından gönderildi."
).await?;

// Resim gönder
client.send_image(
    "+1234567890",
    "https://example.com/image.jpg",
    Some("Resim açıklaması")
).await?;
```

#### Telegram Bot

```bash
# 1. Telegram'da @BotFather'ı açın
# 2. /newbot komutunu gönderin
# 3. Bot için isim ve username girin
# 4. Token'ı kopyalayın

→ [5/8] 💬 Mesajlaşma Kanalları
  📱 Mobile Messengers
    Telegram Bot

Telegram Bot Token: 1234567890:ABCdefGHIjklMNOpqrsTUVwxyz

✅ Telegram bağlandı!
```

**Örnek Kullanım:**

```rust
use sentient_channels::telegram::TelegramClient;

let client = TelegramClient::new(bot_token);

// Mesaj gönder
client.send_message(
    chat_id,
    "Merhaba! SENTIENT size selamlarını iletiyor."
).await?;

// Inline keyboard ile
client.send_message_with_keyboard(
    chat_id,
    "Bir seçenek seçin:",
    vec![
        vec![
            InlineButton::callback("Seçenek A", "option_a"),
            InlineButton::callback("Seçenek B", "option_b"),
        ]
    ]
).await?;
```

#### Matrix (Element)

```bash
# 1. Matrix hesabı oluşturun (matrix.org veya kendi homeserver)
# 2. Access Token alın: Element → Settings → Help → Access Token

→ [5/8] 💬 Mesajlaşma Kanalları
  🔐 Decentralized
    Matrix (Element)

Homeserver URL [https://matrix-client.matrix.org]: 
Access Token: syt_xxxxx...
User ID (@user:matrix.org): @sentient-bot:matrix.org

✅ Matrix bağlandı!
```

**Örnek Kullanım:**

```rust
use sentient_channels::matrix::MatrixClient;

let client = MatrixClient::new(homeserver, access_token, user_id);

// Odaya mesaj gönder
client.send_room_message(
    room_id,
    "Merhaba Matrix!"
).await?;

// DM gönder
client.send_direct_message(
    "@user:matrix.org",
    "Bu gizli bir mesaj!"
).await?;
```

#### iMessage (macOS Only)

```bash
# iMessage, GUI Automation ile kontrol edilir

→ [5/8] 💬 Mesajlaşma Kanalları
  📱 Mobile Messengers
    iMessage (macOS)

⚠️  iMessage sadece macOS'ta kullanılabilir.

iMessage entegrasyonunu aktif et? (y/n): y
💡 GUI Automation seçildi!
   SENTIENT, Agent-S3 ile iMessage'ı otomatik kontrol edecek.
   Bu, Level 3+ yetkilendirme gerektirir.

✅ iMessage yapılandırıldı!
```

**Örnek Kullanım (GUI Automation):**

```rust
use sentient_agent::gui::{GuiController, Key};

// iMessage ile mesaj gönder (macOS)
async fn send_imessage(phone: &str, message: &str) -> Result<()> {
    let gui = GuiController::new();
    
    // Messages.app aç
    gui.open_application("Messages")?;
    gui.wait(1500);
    
    // Yeni mesaj
    gui.press_key(Key::Cmd, Key::N)?;
    gui.wait(500);
    
    // Alıcı
    gui.type_text(phone)?;
    gui.press_key(Key::Tab)?;
    
    // Mesaj
    gui.type_text(message)?;
    gui.press_key(Key::Enter)?;
    
    Ok(())
}
```

#### Signal (GUI Automation)

```rust
use sentient_agent::gui::{GuiController, Key};

// Signal Desktop ile mesaj gönder
async fn send_signal_message(contact: &str, message: &str) -> Result<()> {
    let gui = GuiController::new();
    
    // Signal Desktop'ı aç
    gui.open_application("Signal")?;
    gui.wait(2000);
    
    // Arama yap
    gui.press_key(Key::Ctrl, Key::F)?;
    gui.type_text(contact)?;
    gui.press_key(Key::Enter)?;
    
    // Mesaj yaz
    gui.type_text(message)?;
    gui.press_key(Key::Enter)?;
    
    Ok(())
}
```

### 4.3 Kanal Özellikleri Matrisi

```
┌─────────────────┬──────┬────────┬────────┬───────┬─────────┬───────────┬───────────┐
│ Platform        │ Text │ Images │ Video  │ Files │ Buttons │ E2E Enc   │ Groups    │
├─────────────────┼──────┼────────┼────────┼───────┼─────────┼───────────┼───────────┤
│ WhatsApp        │  ✅  │   ✅   │   ✅   │  ✅   │   ✅    │    ✅     │    ✅     │
│ Signal          │  ✅  │   ✅   │   ✅   │  ✅   │   ❌    │    ✅     │    ✅     │
│ Telegram        │  ✅  │   ✅   │   ✅   │  ✅   │   ✅    │    ❌     │    ✅     │
│ iMessage        │  ✅  │   ✅   │   ✅   │  ✅   │   ❌    │    ✅     │    ✅     │
│ Discord         │  ✅  │   ✅   │   ✅   │  ✅   │   ✅    │    ❌     │    ✅     │
│ Slack           │  ✅  │   ✅   │   ✅   │  ✅   │   ✅    │    ❌     │    ✅     │
│ MS Teams        │  ✅  │   ✅   │   ✅   │  ✅   │   ✅    │    ❌     │    ✅     │
│ Matrix          │  ✅  │   ✅   │   ✅   │  ✅   │   ❌    │    ✅     │    ✅     │
│ Twitter DM      │  ✅  │   ✅   │   ❌   │  ❌   │   ❌    │    ❌     │    ❌     │
│ Email           │  ✅  │   ✅   │   ❌   │  ✅   │   ❌    │    ❌     │    ✅     │
│ SMS             │  ✅  │   ❌   │   ❌   │  ❌   │   ❌    │    ❌     │    ❌     │
└─────────────────┴──────┴────────┴────────┴───────┴─────────┴───────────┴───────────┘
```

### 4.4 Çok Kanallı Mesajlaşma

```rust
use sentient_channels::{ChannelRouter, Message};

let router = ChannelRouter::new()
    .add_channel("whatsapp", whatsapp_client)
    .add_channel("telegram", telegram_client)
    .add_channel("discord", discord_client)
    .add_channel("email", email_client);

// Aynı mesajı birden fazla kanala gönder
let message = Message::text("Toplantı yarın 14:00!");

router.broadcast(
    &["whatsapp", "telegram", "discord"],
    message
).await?;
```

---

## 5. AGENT-S3 HARDWARE PERMISSIONS

### 5.1 Konsept

**Agent-S3**, SENTIENT'nın klavye ve fareyi kontrol ederek masaüstü otomasyonu sağlayan modülüdür. Bu, şu işlemleri mümkün kılar:

- 🖱️ Otomatik tıklama ve kaydırma
- ⌨️ Otomatik yazma ve klavye kısayolları
- 👁️ Ekran okuma ve görsel AI analizi
- 📱 GUI tabanlı mesajlaşma (Signal Desktop, iMessage, Session)
- 🌐 Tarayıcı otomasyonu

### 5.2 Yetki Seviyeleri

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  LEVEL  │  YETKİLER                                                          │
├─────────┼────────────────────────────────────────────────────────────────────┤
│    1    │  📖 Sadece okuma                                                   │
│         │  • Dosya okuma                                                     │
│         │  • Web tarama                                                      │
│         │  • Log analizi                                                     │
├─────────┼────────────────────────────────────────────────────────────────────┤
│    2    │  📝 Dosya işlemleri                                                │
│         │  • Level 1 +                                                       │
│         │  • Dosya oluşturma, düzenleme, silme                               │
│         │  • Dizin yönetimi                                                  │
├─────────┼────────────────────────────────────────────────────────────────────┤
│    3    │  🖱️ Klavye/Mouse kontrolü (Agent-S3)                               │
│         │  • Level 2 +                                                       │
│         │  • GUI automation                                                  │
│         │  • Desktop control                                                 │
│         │  • Signal, iMessage, Session (GUI mode)                            │
├─────────┼────────────────────────────────────────────────────────────────────┤
│    4    │  🚀 Tam otonom                                                     │
│         │  • Level 3 +                                                       │
│         │  • Ağ erişimi (SSH, FTP, Docker)                                   │
│         │  • API çağrıları                                                   │
├─────────┼────────────────────────────────────────────────────────────────────┤
│    5    │  ⚡ Sistem yönetimi                                                 │
│         │  • Level 4 +                                                       │
│         │  • Root erişimi                                                    │
│         │  • Service management                                              │
│         │  • Kernel modules                                                  │
└────────────────┴────────────────────────────────────────────────────────────────────┘
```

### 5.3 GUI Automation Kurulumu

```bash
→ [7/8] 🔐 AGENT-S3 Hardware Permissions

🤖 OTONOM MOD SEVİYELERİ:

  Level 3: 🖱️ Klavye/Mouse kontrolü (Agent-S3)
           → GUI automation, desktop control
           → Signal, iMessage, Session (GUI mode)

Varsayılan yetki seviyesi: Level 3 - Klavye/Mouse (Agent-S3)

🖱️ AGENT-S3: KLAVYE/MOUSE KONTROLÜ

Bu modda SENTIENT şunları yapabilir:
  ✓ Masaüstü uygulamalarını kontrol etme
  ✓ Signal Desktop, iMessage, Session kullanma
  ✓ Otomatik form doldurma
  ✓ Ekran okuma ve analiz
  ✓ Otomatik tıklama ve yazma

SENTIENT klavye ve fareyi kontrol edebilsin mi? (y/n): y

⚠️  DİKKAT:
SENTIENT klavye ve fareyi tam kontrol edebilecek.
Bu, potansiyel olarak güçlü bir özelliktir.

AGENT-S3 aktivasyonunu onaylıyor musunuz? (y/n): y

✅ AGENT-S3 aktif! Klavye/Mouse kontrolü etkin.

Ekran kaydı için izin verilsin mi? (Görsel AI analiz) (y/n): y
```

### 5.4 GUI Automation API

```rust
use sentient_agent::gui::{GuiController, Key, Mouse, MouseButton};

let gui = GuiController::new();

// Uygulama aç
gui.open_application("Firefox")?;
gui.wait(2000);

// URL yaz
gui.press_key(Key::Ctrl, Key::L)?;  // Adres çubuğu
gui.type_text("https://github.com")?;
gui.press_key(Key::Enter)?;

// Tıkla
gui.move_mouse(100, 200)?;
gui.click(MouseButton::Left)?;

// Screenshot al
let screenshot = gui.capture_screen()?;

// OCR ile metin oku
let text = gui.ocr_region(0, 0, 800, 600)?;
```

### 5.5 Güvenlik Önlemleri

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  ⚠️  AGENT-S3 GÜVENLİK UYARILARI                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  1. GUI kontrolü aktif iken DİKKATLİ olun                                    │
│     • SENTIENT klavye/mouse hareketlerini simüle edebilir                       │
│     • Acil durumda: Ctrl+Alt+Del veya fareyi hareket ettirin                 │
│                                                                              │
│  2. Güvenmediğiniz skill'leri çalıştırmayın                                  │
│     • Skill modunu "auto_safe" veya "manual" olarak tutun                    │
│     • "full_auto" sadece güvenilir ortamlarda kullanın                       │
│                                                                              │
│  3. Ekran kaydı verilerine dikkat edin                                       │
│     • Ekran görüntüleri hassas veri içerebilir                               │
│     • Hassas uygulamaları (şifre yöneticisi vb.) koruyun                     │
│                                                                              │
│  4. Erişim sınırlamaları kullanın                                            │
│     • blocked_skills listesine tehlikeli skill'ler ekleyin                   │
│     • allowed_skills ile whitelist kullanın                                  │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 5.587 OTONOM SKILL

### 6.1 Skill Kategorileri

SENTIENT, **5.587 farklı otonom skill'e** sahiptir:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  KATEGORİ                 │  SKILL SAYISI  │  ÖRNEKLER                      │
├───────────────────────────┼────────────────┼────────────────────────────────┤
│  📁 Dosya Yönetimi        │      847       │ read, write, move, archive     │
│  🌐 Web Etkileşimi        │    1.234       │ scrape, crawl, fill, submit    │
│  💻 Kod Geliştirme        │      956       │ generate, refactor, test       │
│  📊 Veri Analizi          │      678       │ parse, visualize, transform    │
│  🔧 Sistem Yönetimi       │      423       │ install, configure, monitor    │
│  🎨 Medya İşleme          │      312       │ convert, resize, transcribe    │
│  📧 İletişim              │      534       │ send, schedule, template       │
│  🔐 Güvenlik              │      603       │ scan, audit, encrypt           │
├───────────────────────────┼────────────────┼────────────────────────────────┤
│  TOPLAM                   │    5.587       │                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Skill Çalıştırma Modları

```bash
→ [7/8] 🔐 AGENT-S3 Hardware Permissions

🎯 5.587 OTONOM SKILL

Skill çalıştırma modu:
  1. 🔒 Manual Onay (Her skill için onay iste)
  2. ⚡ Auto-Safe (Güvenli skill'ler otomatik)  ← Önerilen
  3. 🚀 Full Auto (Tüm skill'ler otomatik)
```

### 6.3 Örnek Skill'ler

#### Dosya İşlemleri

```yaml
# skill: file.batch_rename
description: Bir dizindeki dosyaları yeniden adlandır
params:
  directory: "/path/to/files"
  pattern: "*.jpg"
  rename_template: "{date}_{index}"
output:
  renamed_files: 47
  errors: []
```

#### Web Scraping

```yaml
# skill: web.extract_structured
description: Web sayfasından yapılandırılmış veri çıkar
params:
  url: "https://example.com/products"
  selectors:
    title: "h1.product-title"
    price: ".price"
    image: "img.product-image@src"
output:
  products:
    - title: "Product 1"
      price: "$99.99"
      image: "https://..."
```

#### Kod Üretimi

```yaml
# skill: code.generate_api
description: REST API endpoint'leri oluştur
params:
  language: "rust"
  framework: "axum"
  endpoints:
    - method: GET
      path: /users
      response: User[]
    - method: POST
      path: /users
      body: CreateUser
output:
  files:
    - src/handlers/users.rs
    - src/models/user.rs
```

#### Veri Analizi

```yaml
# skill: data.analyze_csv
description: CSV dosyasını analiz et
params:
  file: "/data/sales.csv"
  operations:
    - summary_stats
    - correlation_matrix
    - outlier_detection
output:
  rows: 10000
  columns: 15
  insights:
    - "Strong correlation (0.89) between price and sales"
    - "23 outliers detected in revenue column"
```

### 6.4 Skill Kataloğu

Tüm skill'leri görüntüleme:

```bash
# Tüm skill'leri listele
sentient skills list

# Kategoriye göre filtrele
sentient skills list --category web

# Skill detaylarını görüntüle
sentient skills show file.batch_rename

# Skill ara
sentient skills search "csv"

# Skill çalıştır
sentient skills run file.batch_rename --directory /path/to/files
```

### 6.5 Özel Skill Oluşturma

```yaml
# ~/.sentient/skills/my_custom_skill.yaml

id: custom.my_skill
name: My Custom Skill
description: Özel skill tanımı
version: 1.0.0
category: custom

# Parametreler
params:
  input:
    type: string
    required: true
    description: Giriş parametresi
  
  options:
    type: object
    properties:
      verbose:
        type: boolean
        default: false

# Çalıştırma
execute:
  type: shell
  command: "echo '{{input}}' | process"
  
# Veya Python
# execute:
#   type: python
#   script: |
#     def run(input, options):
#         return {"result": input.upper()}

# İzinler
permissions:
  level: 2
  safe: true
```

---

## 7. TAM OTONOM MOD

### 7.1 Konsept

SENTIENT, **tam otonom modda** çalışarak kullanıcı müdahalesi olmadan karmaşık görevleri yerine getirebilir:

```bash
# Tam otonom modda başlat
sentient run --autonomous --level 4

# veya config'den
sentient run --config ~/.sentient/autonomous.toml
```

### 7.2 Otonom Mod Yapılandırması

```toml
# ~/.sentient/autonomous.toml

[autonomous]
enabled = true
level = 4  # 1-5

# Güvenlik
safe_mode = true
require_confirmation_for = ["delete", "shutdown", "format"]

# Sınırlamalar
max_actions_per_hour = 100
max_file_size_mb = 100
allowed_directories = ["/home/user/projects", "/home/user/documents"]
blocked_directories = ["/etc", "/root", "/var"]

# Kaynak sınırları
max_memory_mb = 4096
max_cpu_percent = 80

# Zamanlama
working_hours_start = "09:00"
working_hours_end = "18:00"
timezone = "Europe/Istanbul"
```

### 7.3 Otonom Görev Örnekleri

```bash
# Proje analizi ve rapor
sentient run "Bu projeyi analiz et, mimari rapor yaz, README.md güncelle"

# Veri işleme
sentient run "data/*.csv dosyalarını işle, görselleştirmeler oluştur, rapor yaz"

# Kod üretimi
sentient run "Bu API spec'ten Rust backend oluştur, test yaz, deploy et"

# Web scraping
sentient run "Bu siteden ürün bilgilerini çek, veritabanına kaydet"

# Sistem yönetimi
sentient run "Log dosyalarını analiz et, hata raporu oluştur, email gönder"
```

---

## 8. API ENTTEGRASYONLARI

### 8.1 REST API

SENTIENT, tam teşekküllü bir REST API sunar:

```bash
# Server başlat
sentient server --port 8080

# veya
make run
```

**Endpoint'ler:**

```
GET    /api/skills              # Skill listesi
GET    /api/skills/{id}         # Skill detayı
POST   /api/skills/{id}/run     # Skill çalıştır

GET    /api/channels            # Kanal listesi
POST   /api/channels/{id}/send  # Mesaj gönder

GET    /api/agents              # Agent listesi
POST   /api/agents/{id}/task    # Görev ata

GET    /api/status              # Sistem durumu
GET    /api/logs                # Log akışı

POST   /api/chat                # Chat endpoint
POST   /api/execute             # Kod çalıştır
```

### 8.2 WebSocket

```javascript
// WebSocket bağlantısı
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
    ws.send(JSON.stringify({
        type: 'subscribe',
        channels: ['logs', 'status', 'events']
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Event:', data);
};
```

### 8.3 CLI Kullanımı

```bash
# Chat
sentient chat "Merhaba, nasılsın?"

# Skill çalıştır
sentient skill run file.search --pattern "*.rs"

# Kanal mesajı
sentient channel send telegram "@user" "Merhaba!"

# Agent görevi
sentient agent task researcher "AI trendlerini ara"

# Sistem durumu
sentient status

# Logları izle
sentient logs --follow
```

---

## 9. KULLANIM ÖRNEKLERİ

### 9.1 Geliştirici İş Akışı

```bash
# 1. GitHub'dan proje klonla ve analiz et
sentient run "github'dan microsoft/vscode projesini klonla, mimari yapısını analiz et"

# 2. API oluştur
sentient run "Bu proje için REST API endpoint'leri oluştur, OpenAPI dokümantasyonu yaz"

# 3. Test yaz
sentient run "Her endpoint için birim testleri yaz, coverage > %80 olsun"

# 4. Deploy et
sentient run "Docker image oluştur, Kubernetes manifest yaz ve test ortamına deploy et"
```

### 9.2 Veri Analizi İş Akışı

```bash
# 1. Veriyi çek
sentient run "https://data.gov/dataset.csv dosyasını indir, ön işleme yap"

# 2. Analiz et
sentient run "Veriyi analiz et, görselleştirmeler oluştur, rapor yaz"

# 3. Tahmin modeli
sentient run "Makine öğrenmesi modeli eğit, cross-validation yap, modeli kaydet"

# 4. Paylaş
sentient run "Sonuçları Slack ve Email üzerinden ekiple paylaş"
```

### 9.3 Masaüstü Otomasyonu

```bash
# 1. Signal Desktop ile toplu mesaj
sentient run "Signal Desktop'ı aç, contacts.csv'deki herkese 'Toplantı yarın 14:00' mesajı gönder"

# 2. Web formu doldur
sentient run "Chrome'da https://form.example.com adresini aç, form data.json ile doldur"

# 3. Screenshot ve rapor
sentient run "Her 5 dakikada bir https://dashboard.example.com screenshot al, PDF rapor oluştur"
```

### 9.4 Çok Kanallı İletişim

```bash
# Aynı mesajı birden fazla kanala gönder
sentient run "Toplantı hatırlatması: Yarın 14:00 sprint review. Bu mesajı WhatsApp, Telegram, Slack ve Email olarak tüm ekibe gönder"

# Sosyal medya yönetimi
sentient run "Bu haftanın blog yazısını Twitter, LinkedIn ve Facebook'ta paylaş"

# Müşteri desteği
sentient run "Son 24 saatte gelen tüm destek taleplerini kontrol et, önceliklendir, yanıt taslakları hazırla"
```

### 9.5 DevOps İşlemleri

```bash
# Sunucu monitoring
sentient run "Tüm sunucuların durumunu kontrol et, CPU/RAM kullanımı raporla, anormal durumları bildir"

# Log analizi
sentient run "Son 7 günlük logları analiz et, error paternlerini bul, root cause analizi yap"

# Backup kontrolü
sentient run "Backup'ların son durumunu kontrol et, eksik olanları belirle, rapor gönder"
```

---

## 10. SIKÇA SORULAN SORULAR

### S1: Hangi LLM provider'larını kullanabilirim?

**C:** OpenAI veya Anthropic API formatını destekleyen HERHANGİ BİR provider'ı kullanabilirsiniz. Önceden tanımlı provider'lar arasında Together AI, Groq, Mistral, DeepSeek, Ollama ve daha fazlası var. Custom Provider seçeneği ile dünyadaki tüm modellere bağlanabilirsiniz.

### S2: WhatsApp kişisel hesabımı bağlayabilir miyim?

**C:** WhatsApp Business API gereklidir. Kişisel WhatsApp hesapları resmi API ile desteklenmez. Ancak, Signal veya Telegram gibi alternatifleri kullanabilirsiniz.

### S3: Agent-S3 güvenli mi?

**C:** Agent-S3 güçlü bir özelliktir ve dikkatli kullanılmalıdır:
- Skill modunu "auto_safe" olarak tutun
- Güvenmediğiniz skill'leri çalıştırmayın
- Acil durumda fareyi hareket ettirerek otomasyonu durdurabilirsiniz

### S4: Yerel LLM kullanabilir miyim?

**C:** Evet! Ollama, LM Studio, vLLM veya LocalAI gibi yerel LLM çözümlerini kullanabilirsiniz:
```bash
# Ollama kur
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5-coder:7b

# SENTIENT'da Ollama seç
# Base URL: http://localhost:11434/v1
```

### S5: API anahtarlarım güvende mi?

**C:** Evet, API anahtarlarınız V-GATE proxy tarafından korunur:
- Anahtarlar asla istemci kodunda tutulmaz
- Tüm API istekleri V-GATE üzerinden geçer
- Şifreli depolama kullanılır

### S6: Kendi skill'lerimi oluşturabilir miyim?

**C:** Evet! YAML formatında özel skill'ler oluşturabilirsiniz:
```yaml
# ~/.sentient/skills/my_skill.yaml
id: custom.my_skill
name: My Custom Skill
execute:
  type: shell
  command: "your-command"
```

### S7: Birden fazla mesajlaşma kanalını aynı anda kullanabilir miyim?

**C:** Evet! SENTIENT aynı anda 20+ kanalı destekler. Örneğin:
- WhatsApp + Telegram + Discord + Slack
- Matrix + Signal + Email
- Tümünü aynı anda

### S8: iMessage Linux'ta çalışır mı?

**C:** Hayır, iMessage sadece macOS'ta çalışır. Linux'ta Signal, Telegram veya Matrix kullanabilirsiniz.

### S9: Skill'leri sınırlayabilir miyim?

**C:** Evet, config dosyasında:
```toml
[permissions]
skill_mode = "auto_safe"
allowed_skills = ["file.*", "web.scrape"]
blocked_skills = ["system.shutdown", "network.scan"]
```

### S10: SENTIENT'yı bulutta çalıştırabilir miyim?

**C:** Evet, SENTIENT Docker container olarak çalışabilir:
```bash
docker run -d \
  -v ~/.sentient:/root/.sentient \
  -p 8080:8080 \
  sentient/nexus-os:latest
```

### S11: Telegram entegrasyonu nasıl yapılır?

**C:**
1. Telegram'da @BotFather'ı bulun
2. `/newbot` komutunu gönderin
3. Bot ismi ve username belirleyin
4. Token'ı SENTIENT'ya girin
5. Chat ID için: `https://api.telegram.org/bot<TOKEN>/getUpdates`

### S12: DeepSeek kullanabilir miyim?

**C:** Evet! DeepSeek, OpenAI-compatible API sunar:
```
Provider: Custom
Base URL: https://api.deepseek.com/v1
API Key: sk-xxxx
Model: deepseek-coder-33b-instruct
```

### S13: V-GATE nedir?

**C:** V-GATE, API anahtarlarını güvenli bir şekilde saklayan ve yöneten bir proxy sistemidir. API anahtarlarınız asla istemci kodunda tutulmaz, tüm istekler V-GATE üzerinden geçer.

### S14: 5.587 skill'in tamamını kullanmak zorunda mıyım?

**C:** Hayır, ihtiyacınız olan skill'leri seçersiniz. Auto-safe modda, SENTIENT güvenli skill'leri otomatik çalıştırır, tehlikeli olanlar için onay ister.

### S15: Enterprise lisans var mı?

**C:** SENTIENT MIT lisansı ile açık kaynaklıdır. Enterprise destek için: enterprise@sentient.ai

---

## 📞 DESTEK

- **Dokümantasyon:** https://docs.sentient.ai
- **GitHub:** https://github.com/sentient-ai/sentient-nexus-os
- **Discord:** https://discord.gg/sentient-ai
- **Email:** support@sentient.ai

---

```
    ╔══════════════════════════════════════════════════════════════════════════╗
    ║                                                                          ║
    ║   🐺 SENTIENT NEXUS OS v1.1.0 - Universal Omni-Gateway & Full Channel       ║
    ║                                                                          ║
    ║   Made with ❤️  by Pi                                                    ║
    ║   https://github.com/sentient-ai/sentient-nexus-os                            ║
    ║                                                                          ║
    ╚══════════════════════════════════════════════════════════════════════════╝
```
