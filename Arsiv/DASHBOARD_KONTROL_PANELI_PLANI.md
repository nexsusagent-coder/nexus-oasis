# ═══════════════════════════════════════════════════════════════════════════════
#  DASHBOARD KONTROL PANELİ - TAM ENTEGRASYON PLANI
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-13
#  Hedef: Terminal + Dashboard üzerinde TAM KONTROL
# ═══════════════════════════════════════════════════════════════════════════════

---

## 🎯 HEDEF

**OpenClaw Standard**: Sadece terminal değil, dashboard üzerinden de:
- ✅ Setup / Onboarding
- ✅ Tüm konfigürasyon değişiklikleri
- ✅ LLM Provider seçimi
- ✅ Channel yönetimi
- ✅ Tool aktifleştirme/pasifleştirme
- ✅ Permission ayarları
- ✅ Agent yönetimi
- ✅ Sistem durumu görüntüleme

---

## 📊 MEVCUT DURUM ANALİZİ

### ✅ Zaten Var Olanlar

| Özellik | Terminal | Dashboard | Durum |
|---------|----------|-----------|-------|
| Setup Wizard | ✅ `sentient_setup` | ❌ | Eksik |
| LLM Selection | ✅ TUI | ❌ | Eksik |
| Channel Config | ✅ TUI | ❌ | Eksik |
| Tool Status | ✅ CLI | ✅ Görüntüleme | Kısmen |
| Metrics | ❌ | ✅ | Var |
| Logs | ✅ | ✅ | Var |
| Agent Status | ✅ | ✅ Görüntüleme | Kısmen |
| Terminal | ✅ | ✅ WebSocket | Var |

### ❌ Dashboard'da Eksik Olanlar

1. **Setup/Onboarding Paneli**
2. **LLM Provider Yönetimi** (ekle/çıkar/değiştir)
3. **Channel Yönetimi** (aktifleştir/pasifleştir)
4. **Tool Kontrolü** (start/stop/config)
5. **Permission Editor**
6. **Config File Editor**
7. **Agent Spawn UI**
8. **Task Assignment UI**

---

## 🏗️ YENİ MİMARİ

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SENTIENT DASHBOARD v4.0                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        TOP NAVIGATION                                │   │
│  │  [🏠 Home] [⚙️ Setup] [🤖 Agents] [🔧 Tools] [📊 Monitor] [🔐 Security] │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌──────────────┬──────────────────────────────────────────────────────┐   │
│  │              │                                                      │   │
│  │   SIDEBAR    │                    MAIN CONTENT                      │   │
│  │              │                                                      │   │
│  │  📋 Overview │   ┌────────────────────────────────────────────┐    │   │
│  │  🤖 LLM      │   │                                            │    │   │
│  │  📡 Channels │   │           ACTIVE PANEL                      │    │   │
│  │  🔧 Tools    │   │                                            │    │   │
│  │  👥 Agents   │   │   (Setup / Config / Monitor / Control)     │    │   │
│  │  📊 Metrics  │   │                                            │    │   │
│  │  📜 Logs     │   │                                            │    │   │
│  │  🔐 Security │   └────────────────────────────────────────────┘    │   │
│  │  ⚙️ Settings │                                                      │   │
│  │              │                                                      │   │
│  └──────────────┴──────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        BOTTOM STATUS BAR                             │   │
│  │  [Terminal] [CPU: 12%] [MEM: 742MB] [Agents: 8] [V-GATE: ✅]         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 YENİ MODÜLLER

### 1. Setup Panel (`/setup`)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ⚙️ SETUP & CONFIGURATION                                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  STEP 1: LLM Provider                                        │    │
│  │  ─────────────────────────────────────────────────────────── │    │
│  │  Current: openai/gpt-4o                                      │    │
│  │                                                              │    │
│  │  [🔍 Search models...]                                       │    │
│  │                                                              │    │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │    │
│  │  │ Anthropic   │ │ OpenAI      │ │ Google      │            │    │
│  │  │ ─────────── │ │ ─────────── │ │ ─────────── │            │    │
│  │  │ claude-3.5  │ │ gpt-4o      │ │ gemini-2.0  │            │    │
│  │  │ sonnet      │ │ gpt-4-turbo │ │ flash       │            │    │
│  │  │             │ │ o1-preview  │ │ gemini-1.5  │            │    │
│  │  └─────────────┘ └─────────────┘ └─────────────┘            │    │
│  │                                                              │    │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐            │    │
│  │  │ Ollama      │ │ Groq        │ │ OpenRouter  │            │    │
│  │  │ ─────────── │ │ ─────────── │ │ ─────────── │            │    │
│  │  │ llama3.3:70b│ │ llama-3.3   │ │ 100+ models │            │    │
│  │  │ gemma2:27b  │ │ mixtral     │ │             │            │    │
│  │  └─────────────┘ └─────────────┘ └─────────────┘            │    │
│  │                                                              │    │
│  │  API Key: [••••••••••••••••] [Show] [Test]                  │    │
│  │                                                              │    │
│  │                                        [Previous] [Next →]   │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 2. Channels Panel (`/channels`)

```
┌─────────────────────────────────────────────────────────────────────┐
│  📡 COMMUNICATION CHANNELS                                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ 🔍 Search channels...                    [+ Add Channel]      │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ ✅ Telegram                                    [Edit] [Test]  │   │
│  │ ────────────────────────────────────────────────────────────  │   │
│  │ Bot Token: ••••••••••••                                        │   │
│  │ Chat ID: -1001234567890                                        │   │
│  │ Status: Connected | Messages: 1,547 | Last: 2 mins ago        │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ ✅ Discord                                     [Edit] [Test]  │   │
│  │ ────────────────────────────────────────────────────────────  │   │
│  │ Bot Token: ••••••••••••                                        │   │
│  │ Guild ID: 123456789                                            │   │
│  │ Status: Connected | Messages: 892 | Last: 5 mins ago          │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ ⏸️ Slack                                      [Edit] [Enable] │   │
│  │ ────────────────────────────────────────────────────────────  │   │
│  │ Webhook URL: Not configured                                    │   │
│  │ Status: Disabled                                               │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ ⏸️ WhatsApp                                   [Edit] [Enable] │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  Available: Email, Slack, Discord, Telegram, WhatsApp, Matrix,     │
│             Teams, IRC, Mastodon, Twitter, LinkedIn...              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 3. Agents Panel (`/agents`)

```
┌─────────────────────────────────────────────────────────────────────┐
│  🤖 MULTI-AGENT ORCHESTRATOR                                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Framework: [CrewAI ▼]    Max Agents: [5]    [+ Spawn Agent]  │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │  ACTIVE AGENTS (3/5)                                        │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │                                                             │     │
│  │  🅰️ Alpha - Research Agent                     [●] Active   │     │
│  │     Task: "Python kütüphaneleri araştır"                   │     │
│  │     Workload: 45% | Completed: 127 tasks                   │     │
│  │     [View Logs] [Pause] [Assign Task]                      │     │
│  │                                                             │     │
│  │  🅱️ Beta - Code Agent                         [●] Active   │     │
│  │     Task: "API endpoint yaz"                               │     │
│  │     Workload: 78% | Completed: 89 tasks                    │     │
│  │     [View Logs] [Pause] [Assign Task]                      │     │
│  │                                                             │     │
│  │  🇬 Gamma - Data Agent                        [○] Idle     │     │
│  │     Task: None                                             │     │
│  │     Workload: 0% | Completed: 234 tasks                    │     │
│  │     [View Logs] [Start] [Assign Task]                      │     │
│  │                                                             │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  TASK QUEUE                                                   │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │  #1 [Pending] "Veritabanı şeması tasarla" - Priority: 8      │   │
│  │  #2 [Pending] "Test yaz" - Priority: 5                       │   │
│  │  #3 [Pending] "Dokümantasyon güncelle" - Priority: 3         │   │
│  │                                                              │   │
│  │  [+ Add Task] [Assign All] [Clear Queue]                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  MESSAGE BUS                                   [📡 Live Feed] │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │  12:45:32 Alpha → Beta: "Research complete, handing off"     │   │
│  │  12:45:30 Gamma → All: "Data processing started"             │   │
│  │  12:45:28 Beta → Alpha: "Ready for code generation"          │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 4. Tools Panel (`/tools`)

```
┌─────────────────────────────────────────────────────────────────────┐
│  🔧 TOOLS & INTEGRATIONS                                            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Category: [All ▼]  Status: [All ▼]       [🔍 Search...]      │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ SEARCH & RESEARCH (3 tools)                                 │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │ 🧠 MindSearch                         [●] Active            │     │
│  │    AI-powered deep research                                  │     │
│  │    [Configure] [View Logs] [Disable]                         │     │
│  │                                                              │     │
│  │ 🔐 Google CLI                         [●] Active            │     │
│  │    Command-line Google access                                │     │
│  │    [Configure] [View Logs] [Disable]                         │     │
│  │                                                              │     │
│  │ 🌐 SearXNG                            [●] Active            │     │
│  │    Privacy-focused meta search                               │     │
│  │    [Configure] [View Logs] [Disable]                         │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐     │
│  │ BROWSER AUTOMATION (2 tools)                                │     │
│  ├────────────────────────────────────────────────────────────┤     │
│  │ 🌍 Browser-Use                        [●] Active            │     │
│  │    AI agent browser automation                               │     │
│  │    [Configure] [View Sessions] [Disable]                     │     │
│  │                                                              │     │
│  │ 🐼 Lightpanda                         [●] Active            │     │
│  │    Lightweight browser engine                                │     │
│  │    [Configure] [View Logs] [Disable]                         │     │
│  └────────────────────────────────────────────────────────────┘     │
│                                                                      │
│  Total: 43 tools | Active: 38 | Ready: 5 | Disabled: 0             │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 5. Settings Panel (`/settings`)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ⚙️ SYSTEM SETTINGS                                                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ GENERAL                                                       │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │ Language:          [English ▼]                                │   │
│  │ Dashboard Port:    [18789]                                    │   │
│  │ Dashboard Host:    [127.0.0.1]                                │   │
│  │ Theme:             [Dark ▼]                                   │   │
│  │ Log Level:         [Info ▼]                                   │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ SECURITY                                                      │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │ Mode:              [●] Personal  [○] Lock-down                │   │
│  │ Require Confirmation: [✓] For destructive actions             │   │
│  │ Audit Log:         [✓] Enabled                                │   │
│  │ Session Timeout:   [3600] seconds                             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ PERMISSIONS                                                   │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │ File System:       [●] Full Access                            │   │
│  │ Network:           [●] Full Access                            │   │
│  │ System Commands:   [●] With Confirmation                      │   │
│  │ Browser:           [●] Full Access                            │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ CONFIG FILES                                                  │   │
│  ├──────────────────────────────────────────────────────────────┤   │
│  │ 📄 ~/.sentient/config.toml        [Edit] [Download]           │   │
│  │ 📄 ~/.sentient/.env               [Edit] [Download]           │   │
│  │ 📄 ~/.sentient/permissions.json   [Edit] [Download]           │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│                                    [Reset to Defaults] [Save All]   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔌 YENİ API ENDPOINTS

### Setup API

```
POST   /api/setup/start           # Setup başlat
GET    /api/setup/status          # Setup durumu
POST   /api/setup/llm             # LLM config kaydet
POST   /api/setup/channels        # Channels config kaydet
POST   /api/setup/tools           # Tools config kaydet
POST   /api/setup/permissions     # Permissions config kaydet
POST   /api/setup/complete        # Setup tamamla
```

### Config API

```
GET    /api/config                # Tüm config'i getir
PUT    /api/config                # Config güncelle
GET    /api/config/llm            # LLM config
PUT    /api/config/llm            # LLM config güncelle
GET    /api/config/channels       # Channels config
PUT    /api/config/channels       # Channels config güncelle
DELETE /api/config/channels/:id   # Channel sil
GET    /api/config/tools          # Tools config
PUT    /api/config/tools/:id      # Tool config güncelle
```

### Agents API

```
GET    /api/agents                # Agent listesi
POST   /api/agents                # Yeni agent spawn
GET    /api/agents/:id            # Agent detayı
PUT    /api/agents/:id            # Agent güncelle
DELETE /api/agents/:id            # Agent sil
POST   /api/agents/:id/task       # Agent'a task ata
GET    /api/agents/:id/logs       # Agent logları
GET    /api/agents/message-bus    # Message bus feed (WebSocket)
```

### Tools API

```
GET    /api/tools                 # Tool listesi
GET    /api/tools/:id             # Tool detayı
PUT    /api/tools/:id             # Tool config güncelle
POST   /api/tools/:id/start       # Tool başlat
POST   /api/tools/:id/stop        # Tool durdur
POST   /api/tools/:id/execute     # Tool çalıştır
GET    /api/tools/:id/logs        # Tool logları
```

---

## 📁 DOSYA YAPISI

```
dashboard/
├── src/
│   ├── main.rs              # Ana server
│   ├── api.rs               # API handlers
│   ├── ws.rs                # WebSocket handlers
│   │
│   ├── routes/              # YENİ: Route modülleri
│   │   ├── mod.rs
│   │   ├── setup.rs         # Setup API
│   │   ├── config.rs        # Config API
│   │   ├── agents.rs        # Agents API
│   │   ├── tools.rs         # Tools API
│   │   └── channels.rs      # Channels API
│   │
│   └── state/               # YENİ: State management
│       ├── mod.rs
│       ├── config_state.rs
│       ├── agent_state.rs
│       └── tool_state.rs
│
├── assets/
│   ├── index.html           # Ana sayfa (mevcut)
│   ├── setup.html           # YENİ: Setup wizard
│   ├── agents.html          # YENİ: Agent yönetimi
│   ├── channels.html        # YENİ: Channel yönetimi
│   ├── tools.html           # YENİ: Tool yönetimi
│   ├── settings.html        # YENİ: Settings
│   │
│   ├── css/
│   │   └── style.css        # Ana stiller
│   │
│   └── js/
│       ├── app.js           # Ana uygulama
│       ├── setup.js         # Setup wizard
│       ├── agents.js        # Agent yönetimi
│       ├── channels.js      # Channel yönetimi
│       └── tools.js         # Tool yönetimi
│
└── Cargo.toml
```

---

## 🚀 UYGULAMA SIRASI

### Faz 1: Temel Altyapı (1-2 gün)

1. **Route Modülleri Oluştur**
   - `routes/mod.rs`
   - `routes/setup.rs`
   - `routes/config.rs`

2. **State Management**
   - `state/mod.rs`
   - `state/config_state.rs`

3. **API Endpoints**
   - Setup API
   - Config API

### Faz 2: Setup Paneli (2-3 gün)

1. **HTML Template**
   - `assets/setup.html`
   - Step-by-step wizard UI

2. **JavaScript**
   - `assets/js/setup.js`
   - Model selection
   - API key input

3. **Backend Integration**
   - Config kaydetme
   - .env dosyası güncelleme

### Faz 3: Agents Paneli (2-3 gün)

1. **HTML Template**
   - `assets/agents.html`
   - Agent cards
   - Task queue

2. **JavaScript**
   - `assets/js/agents.js`
   - Agent spawn
   - Task assignment

3. **Backend Integration**
   - MultiAgentOrchestrator entegrasyonu
   - Message Bus WebSocket

### Faz 4: Tools & Channels (2-3 gün)

1. **Tools Panel**
   - `assets/tools.html`
   - Tool configuration
   - Enable/disable

2. **Channels Panel**
   - `assets/channels.html`
   - Channel configuration
   - Test connections

### Faz 5: Settings & Polish (1-2 gün)

1. **Settings Panel**
   - `assets/settings.html`
   - Config editor
   - File management

2. **UI Polish**
   - Animations
   - Error handling
   - Loading states

---

## ✅ SONUÇ

Bu plan tamamlandığında:

| İşlem | Terminal | Dashboard |
|-------|----------|-----------|
| Setup / Onboarding | ✅ | ✅ |
| LLM Provider Seçimi | ✅ | ✅ |
| Channel Yönetimi | ✅ | ✅ |
| Tool Kontrolü | ✅ | ✅ |
| Agent Spawn | ✅ | ✅ |
| Task Assignment | ✅ | ✅ |
| Permission Ayarları | ✅ | ✅ |
| Config Edit | ✅ | ✅ |
| Sistem İzleme | ✅ | ✅ |

**TAM KONTROL HER İKİ YERDEN!** 🎯

---

*Rapor Tarihi: 2026-04-13*
*Durum: PLAN HAZIR - UYGULAMA BEKLENİYOR*
