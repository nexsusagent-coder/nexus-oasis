# 🧠 SENTIENT OS - Gerçek Dünya Kullanım Senaryoları

> **Her senaryo, gerçek crate kaynak kodundan türetilmiştir**

---

## 📑 İçindekiler

1. [JARVIS: Yatakta Müzik Aç](#1-jarvis-yatakta-müzik-aç)
2. [JARVIS: Akıllı Ev Kontrolü](#2-jarvis-akıllı-ev-kontrolü)
3. [JARVIS: Sabah Bülteni](#3-jarvis-sabah-bülteni)
4. [Otonom: Web Araştırması](#4-otonom-web-araştırması)
5. [Otonom: Kod Yazma ve Test](#5-otonom-kod-yazma-ve-test)
6. [Multi-Agent: Araştırma Ekibi](#6-multi-agent-arştırma-ekibi)
7. [Multi-Agent: Yazılım Şirketi](#7-multi-agent-yazılım-şirketi)
8. [AI Gateway: Akıllı Yönlendirme](#8-ai-gateway-akıllı-yönlendirme)
9. [Cevahir AI: Türkçe Reasoning](#9-cevahir-ai-türkçe-reasoning)
10. [Proactive: GitHub PR İnceleme](#10-proactive-github-pr-i̇nceleme)
11. [Kanal: 7/24 Telegram Destek Botu](#11-kanal-724-telegram-destek-botu)
12. [MCP: Claude Desktop Entegrasyonu](#12-mcp-claude-desktop-entegrasyonu)
13. [Persona: Dinamik Kişilik Değişimi](#13-persona-dinamik-kişilik-değişimi)
14. [Workflow: Otomatik Rapor Pipeline](#14-workflow-otomatik-rapor-pipeline)
15. [Enterprise: SOC2 Uyumlu Dağıtım](#15-enterprise-soc2-uyumlu-dağıtım)

---

## 1. JARVIS: Yatakta Müzik Aç

**Kullanılan Crate'ler:** `sentient_daemon`, `sentient_voice`, `sentient_wake`, `oasis_browser`

### Senaryo Akışı

```
Kullanıcı yatakta yatıyor → "Hey Sentient, rahatlatıcı müzik aç"
```

### Detaylı Akış

```
1. sentient_wake: "Hey Sentient" algılandı
   │
2. sentient_voice::stt: Ses kaydı → "rahatlatıcı müzik aç"
   │  (Whisper.cpp — lokal, ücretsiz)
   │
3. sentient_daemon::commands::CommandParser::parse()
   │  → intent: PlayMusic
   │  → entities: { "platform": "youtube", "query": "rahatlatıcı" }
   │  → confidence: 0.9
   │  → original: "rahatlatıcı müzik aç"
   │  (Türkçe anahtar kelimeler: müzik, şarkı, melodi, playlist)
   │  (İngilizce: music, song, play, playlist)
   │
4. sentient_daemon::actions::VoiceActionExecutor::execute()
   │  → StealthEngine::human_delay() — insan gibi gecikme
   │  → oasis_browser::BrowserAction::Navigate → YouTube arama URL
   │  → oasis_browser::BrowserAction::WaitForPageLoad
   │  → oasis_browser::BrowserAction::Click("a#video-title", index=0)
   │
5. sentient_voice::tts: "Rahatlatıcı müziği açıyorum."
   │  (Piper — lokal, Türkçe, ücretsiz)
   │
6. Daemon → State::Listening (tekrar wake word dinliyor)
```

### Komut

```bash
# Daemon başlat
sentient daemon start

# Veya doğrudan voice mod
sentient voice --wake-word "hey sentient" --language tr
```

### Desteklenen Müzik Komutları

| Türkçe | İngilizce | Aksiyon |
|--------|-----------|---------|
| "müzik aç" | "play music" | YouTube'da arar |
| "şarkı aç" | "play song" | YouTube'da arar |
| "sezen aksu aç" | "play sezen aksu" | YouTube'da arar |
| "rahatlatıcı müzik" | "relaxing music" | YouTube arama + tıklama |
| "durdur" | "pause" | YouTube pause butonu |
| "devam et" | "resume" | YouTube play butonu |
| "kapat" | "close" | Tarayıcı sekmesi kapat |

---

## 2. JARVIS: Akıllı Ev Kontrolü

**Kullanılan Crate'ler:** `sentient_daemon`, `sentient_home`, `oasis_browser`

### Senaryo Akışı

```
Kullanıcı: "Salon ışığını kapat"
```

### Detaylı Akış

```
1. CommandParser::parse("salon ışığını kapat")
   → matches_home_keywords() = true
   → intent: ControlHome
   → entities: { "action": "turn_off", "room": "living_room", "device_type": "light" }

2. VoiceActionExecutor::control_home()
   → HomeClient::execute_command(DeviceCommand::TurnOff("light.living_room"))
   → Home Assistant API'ye REST istek

3. TTS: "Salon ışığını kapatıyorum."
```

### Desteklenen Ev Komutları

| Türkçe Komut | Algılanan Aksiyon | Entity |
|---|---|---|
| "salon ışığını aç" | turn_on | light.living_room |
| "yatak odası lambasını kapat" | turn_off | light.bedroom |
| "mutfak ışığını kıs" | dim | light.kitchen (30%) |
| "ofis lambasını parlat" | brighten | light.office (100%) |
| "klimayı 22 derece yap" | set_temperature | climate |
| "film modu" | activate_scene | movie |
| "uyku modu" | activate_scene | good_night |
| "sabah modu" | activate_scene | good_morning |
| "parti modu" | activate_scene | party |

### Yapılandırma

```bash
# .env
HOME_ASSISTANT_URL=http://homeassistant.local:8123
HOME_ASSISTANT_TOKEN=eyJ0eXAi...

sentient home connect
```

---

## 3. JARVIS: Sabah Bülteni

**Kullanılan Crate'ler:** `sentient_proactive`, `sentient_digest`, `sentient_connectors`, `sentient_voice`, `sentient_channels`

### Senaryo

Her sabah 9'da otomatik bülten oluşturur ve sesli okur.

### Akış

```
1. sentient_proactive: TimeBased trigger saat 09:00 tetiklenir
   │  (days: [1,2,3,4,5] — sadece hafta içi)
   │
2. sentient_digest: Günlük bülten oluştur
   │  ├→ sentient_connectors::github → Yeni PR/issue'lar
   │  ├→ sentient_connectors::gmail → Önemli emailler
   │  ├→ sentient_calendar → Bugünkü toplantılar
   │  ├→ sentient_search → Hava durumu
   │  └→ sentient_llm → Özet oluştur
   │
3. sentient_voice::tts: Sesli oku
   │  "Günaydın! Bugün hava 22 derece, güneşli.
   │   2 toplantınız var: 10:00 Product Sync, 14:00 Sprint Review.
   │   GitHub'da 3 yeni PR var."
   │
4. sentient_channels::telegram: Metin olarak gönder
```

### Komut

```bash
sentient proactive add \
  --name "morning-brief" \
  --type time \
  --time "09:00" \
  --days "mon-fri" \
  --action "generate-briefing"
```

---

## 4. Otonom: Web Araştırması

**Kullanılan Crate'ler:** `oasis_autonomous`, `oasis_hands`, `oasis_browser`, `sentient_memory`

### Senaryo

```bash
sentient desktop --goal "Rust ile web framework karşılaştırması yap"
```

### Akış

```
1. Agent State: Initializing
   → AgentConfig::default() yüklenir

2. State: Perceiving
   → screen.rs: Ekran görüntüsü al
   → vision.rs: UI elementleri tespit et

3. State: Deciding
   → planner.rs: Görevi adımlara böl
   → safety.rs: "rm -rf" gibi komutlar engelli
   → Plan: [Browser aç → Google'da ara → İlk 5 sonuca tıkla → Oku → Özetle]

4. State: Acting
   → Action::BrowserNavigate("https://google.com")
   → Action::TypeText("Rust web framework comparison", human_like=true)
   → Action::KeyPress(Key::Enter)
   → Action::MouseClick(button=Left, x=300, y=400)  // İlk sonuç
   → Action::BrowserClick(selector="article")
   → ... her sayfa için tekrarla

5. State: Learning
   → memory.rs: Önemli bilgileri belleğe kaydet
   → Episode: "Rust framework karşılaştırması: Actix-web en hızlı"
   → healing.rs: Hata olduysa自我修复

6. State: Idle → Rapor yazıldı
```

### Sovereign Constitution

```
✗ YASAKLI (50+ komut):
  rm -rf /, format, dd if=/dev/zero, chmod 777 /
  sudo, su, chown root, curl | bash, mkfs
  shutdown, reboot, halt, poweroff

✓ İZİN VERİLEN:
  libreoffice, firefox, vscode, gnome-terminal
  nautilus, git, cargo, npm, python3
```

---

## 5. Otonom: Kod Yazma ve Test

**Kullanılan Crate'ler:** `oasis_autonomous`, `oasis_hands`, `sentient_sandbox`

### Senaryo

```bash
sentient desktop --goal "Bir Rust REST API projesi oluştur"
```

### Akış

```
1. Planner: Görevi parçala
   → Adım 1: cargo new api_project
   → Adım 2: Cargo.toml'a bağımlılık ekle (axum, serde, tokio)
   → Adım 3: src/main.rs yaz
   → Adım 4: cargo test çalıştır
   → Adım 5: Hata varsa düzelt (self-healing)
   → Adım 6: cargo run ile doğrula

2. Acting:
   → Action::TypeText("cargo new api_project", human_like=true)
   → Action::KeyPress(Key::Enter)
   → Action::TypeText("cd api_project && code .")
   → ... VS Code'da kod yaz
   → Action::KeyShortcut(modifiers=[Ctrl, S], key=S)  // Kaydet
   → Action::TypeText("cargo test")

3. Self-Healing (hata olursa):
   → ErrorPattern analizi
   → Kod düzeltme stratejisi
   → Yeniden deneme (exponential backoff)
   → Sonuç belleğe kaydet
```

---

## 6. Multi-Agent: Araştırma Ekibi

**Kullanılan Crate'ler:** `sentient_agents`, `sentient_orchestrator`

### Senaryo

```bash
# Crew oluştur
sentient crew create research-team \
  --agents "researcher:deepseek-r1,writer:gpt-4o,editor:claude-4-sonnet"

# Görev ver
sentient crew run research-team --goal "Yapay zeka pazar analizi raporu yaz"
```

### Akış (CrewAI modu)

```
1. AgentOrchestrator: AgentFramework::CrewAI seçildi
   │
2. Crew yapısı:
   │  ├── Researcher (deepseek-r1) — Bilgi toplar
   │  ├── Writer (gpt-4o) — Rapor yazar
   │  └── Editor (claude-4-sonnet) — Düzenler
   │
3. SwarmCoordinator: Görev dağıtımı
   │  → Task: "AI pazar büyüklüğünü araştır" → Researcher
   │  → Task: "Bulguları raporla" → Writer
   │  → Task: "Raporu düzenle" → Editor
   │
4. Blackboard: Paylaşılan bellek
   │  → Researcher bulgularını yazar
   │  → Writer bulguları okur
   │  → Editor yazıyı okur ve düzeltir
   │
5. Sonuç: Kapsamlı AI pazar analizi raporu
```

---

## 7. Multi-Agent: Yazılım Şirketi

**Kullanılan Crate'ler:** `sentient_agents::metagpt`, `sentient_orchestrator`

### Senaryo

MetaGPT şirketi modeli ile 4 farklı role sahip ekip.

```bash
sentient crew run software-team --framework metagpt \
  --goal "Sosyal medya uygulaması geliştir"
```

### Akış

```
1. Product Manager → Gereksinim analizi + PRD yazımı
2. Architect → Sistem tasarımı + mimari kararlar
3. Engineer → Kod implementasyonu
4. QA Engineer → Test + doğrulama
```

---

## 8. AI Gateway: Akıllı Yönlendirme

**Kullanılan Crate'ler:** `sentient_llm` (Unify, Portkey, LiteLLM, OpenRouter)

### Senaryo: Unify AI ile Maliyet Optimizasyonu

```bash
# Unify: ML bazlı routing
sentient chat --model "unify/router@q>0.9&c<0.001"
```

**Ne yapar?** Her prompt'u analiz eder, en uygun modeli seçer:
- "Merhaba" → DeepSeek Flash (ucuz, basit)
- "Rust'ta lifetime nedir?" → GPT-4o (kod kalitesi yüksek)
- "Bu algoritmanın O() analizi" → DeepSeek R1 (reasoning)

### Senaryo: Portkey ile Enterprise Gateway

```bash
# Portkey: Failover + Cache + Cost tracking
sentient chat --model "portkey/gpt-4o"

# Model düşerse otomatik diğerine geçer (failover)
# Aynı soru tekrar sorulursa cache'den döner
# Her request'in maliyetini izler
```

### Senaryo: LiteLLM ile Self-Hosted Proxy

```bash
# LiteLLM proxy başlat
pip install litellm[proxy]
litellm --config config.yaml --port 4000

# config.yaml: 100+ provider, model mapping, rate limits
# SENTIENT'ten kullan:
sentient chat --provider litellm --model gpt-4o
```

---

## 9. Cevahir AI: Türkçe Reasoning

**Kullanılan Crate'ler:** `sentient_cevahir`

### Senaryo: Tree of Thoughts ile Problem Çözme

```rust
use sentient_cevahir::{CevahirBridge, CognitiveStrategy};

let bridge = CevahirBridge::new()?;

// Karmaşık hata analizi
let result = bridge.process_with_strategy(
    "Bu Rust kodunda deadlock oluyor, neden?",
    CognitiveStrategy::TreeOfThoughts,
).await?;

// Debate ile çoklu perspektif
let result = bridge.process_with_strategy(
    "Microservices vs Monolith — hangisi?",
    CognitiveStrategy::Debate,
).await?;
```

| Strateji | Ne Zaman | Açıklama |
|----------|----------|----------|
| Direct | Basit soru | Doğrudan yanıt |
| Think | Kod analizi | Adım adım düşünme |
| Debate | Tasarım kararı | Çoklu perspektif |
| TreeOfThoughts | Debug/reasoning | Ağaç yapısında düşünme |

---

## 10. Proactive: GitHub PR İnceleme

**Kullanılan Crate'ler:** `sentient_proactive`, `sentient_connectors::github`

### Senaryo

GitHub'da yeni PR açıldığında otomatik inceleme.

```bash
sentient proactive add \
  --name "pr-auto-review" \
  --type event \
  --event "github.pr_opened" \
  --action "auto-review"
```

### Akış

```
1. GitHub webhook → sentient_proactive event tetiklenir
2. sentient_connectors::github → PR diff'ini alır
3. sentient_llm → Kod analizi (güvenlik, stil, performans)
4. sentient_channels::github → Review yorumu gönderir
```

---

## 11. Kanal: 7/24 Telegram Destek Botu

**Kullanılan Crate'ler:** `sentient_channels`, `sentient_gateway`, `sentient_llm`

### Senaryo

Telegram botu müşteri sorularını 7/24 yanıtlar.

```bash
# Bot oluştur: @BotFather → /newbot
TELEGRAM_BOT_TOKEN=123456:ABC-DEF...

# Başlat
sentient channel start telegram
```

### Telegram Komutları

```
/ask <soru>      — AI'ya sor
/code <istek>    — Kod yaz
/search <konu>   — Web ara
/status          — Sistem durumu
/help            — Yardım
```

---

## 12. MCP: Claude Desktop Entegrasyonu

**Kullanılan Crate'ler:** `sentient_mcp`

### Senaryo

SENTIENT araçlarını Claude Desktop'tan kullanma.

```rust
use sentient_mcp::{Server, ServerConfig};

let mut server = Server::new(ServerConfig::default());

// SENTIENT araçlarını MCP server olarak kaydet
server.register_tool(EchoTool);       // Echo
server.register_tool(FileReadTool);   // Dosya okuma
server.register_tool(WebSearchTool);   // Web arama
server.register_tool(CodeReviewTool); // Kod inceleme

// stdio transport — Claude Desktop iletişimi
server.run().await?;
```

**Claude Desktop configuration:**
```json
{
  "mcpServers": {
    "sentient": {
      "command": "sentient-mcp-server",
      "args": ["--transport", "stdio"]
    }
  }
}
```

---

## 13. Persona: Dinamik Kişilik Değişimi

**Kullanılan Crate'ler:** `sentient_persona`

### Senaryo

Kullanıcıya göre otomatik persona değişimi.

```bash
# Rust uzmanı persona (kod sorularında)
sentient persona create "Rust Uzmanı" --traits "concise,technical,formal"

# Samimi asistan (genel sohbet)
sentient persona create "Arkadaş" --traits "friendly,casual,helpful"

# Aktif et
sentient persona set "Rust Uzmanı"
```

**DynamicAdaptationEngine** kullanıcının tercihlerini öğrenir ve zamanla persona'yı otomatik ayarlar.

---

## 14. Workflow: Otomatik Rapor Pipeline

**Kullanılan Crate'ler:** `sentient_workflow`

### Senaryo

n8n tarzı node-based workflow ile otomatik rapor.

```
[Timer: Cuma 17:00] → [GitHub: PR listesi] → [LLM: Özetle] → [Email: Rapor gönder]
```

### Workflow States

| Status | Açıklama |
|--------|----------|
| Draft | Taslak oluşturuldu |
| Active | Aktif ve çalışıyor |
| Paused | Geçici duraklatıldı |
| Completed | Başarıyla tamamlandı |
| Failed | Hata oluştu |

---

## 15. Enterprise: SOC2 Uyumlu Dağıtım

**Kullanılan Crate'ler:** `sentient_compliance`, `sentient_vgate`, `oasis_vault`, `sentient_tee`, `sentient_zk_mcp`

### Senaryo

SOC2 uyumlu, şifreli API key yönetimi, audit log, TEE execution.

```bash
# V-GATE: API key'ler sunucuda, istemcide yok
sentient vgate start --config ~/.config/sentient/vgate.toml

# Vault: Secret'lar şifreli
sentient vault set OPENAI_API_KEY "sk-xxx"

# Guardrails: Strict mode
sentient config set guardrails.mode strict

# Audit log
sentient audit list --from 2026-01-01 --to 2026-04-16

# RBAC
sentient role create admin --permissions "all"
sentient user assign admin user@company.com

# SSO (Okta/Auth0/Azure)
sentient config set sso.provider okta
```

---

## 📊 Senaryo Özet Tablosu

| # | Senaryo | Crate'ler | Mod |
|---|---------|-----------|-----|
| 1 | Yatakta müzik aç | daemon, voice, wake, browser | JARVIS |
| 2 | Akıllı ev kontrolü | daemon, home | JARVIS |
| 3 | Sabah bülteni | proactive, digest, connectors, voice | Proactive |
| 4 | Web araştırması | autonomous, hands, browser, memory | Otonom |
| 5 | Kod yazma | autonomous, hands, sandbox | Otonom |
| 6 | Araştırma ekibi | agents, orchestrator | Multi-Agent |
| 7 | Yazılım şirketi | agents::metagpt, orchestrator | Multi-Agent |
| 8 | Akıllı yönlendirme | llm (unify, portkey, litellm) | Gateway |
| 9 | Türkçe reasoning | cevahir | Cognitive |
| 10 | GitHub PR inceleme | proactive, connectors::github | Proactive |
| 11 | Telegram bot | channels, gateway, llm | Kanal |
| 12 | Claude Desktop MCP | mcp | MCP |
| 13 | Dinamik persona | persona | Persona |
| 14 | Otomatik rapor | workflow | Workflow |
| 15 | SOC2 dağıtım | compliance, vgate, vault, tee, zk_mcp | Enterprise |

---

<p align="center">
  <b>SENTIENT OS</b> - The Operating System That Thinks<br>
  Made with 🦀 by the SENTIENT Team
</p>
