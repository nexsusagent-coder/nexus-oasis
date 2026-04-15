# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - PERSONAL AI ROADMAP & GÜNLÜK RAPOR
# ═══════════════════════════════════════════════════════════════════════════════
#  Oluşturulma: 2026-04-15
#  Kaynak: OpenJarvis (https://github.com/open-jarvis/OpenJarvis) analizi
#  Amaç: SENTIENT OS'u "Kişisel AI Asistanı" platformuna dönüştürmek
# ═══════════════════════════════════════════════════════════════════════════════

## 📋 GÖRÜŞME ÖZETİ (2026-04-15)

### Konu: OpenJarvis İncelemesi ve SENTIENT Entegrasyon Stratejisi

**1. OpenJarvis Nedir?**
- Stanford SAIL (Hazy Research + Scaling Intelligence Lab) geliştirmesi
- "Local-first Personal AI" framework'ü — AI agent'ları yerel cihazda çalışsın
- Python + Rust (PyO3) — 116,828 Python satırı, 26,763 Rust satırı, 517 dosya
- Apache 2.0 lisanslı
- 7 Sütun: Intelligence, Agent, Tools, Engine, Learning, Channels, Connectors
- 30+ kanal, 25+ connector, 14 inference backend, 8 agent türü
- Intelligence Per Watt araştırmasından doğmuş — yerel modellerin %88.7'sini karşılayabildiği

**2. SENTIENT vs OpenJarvis Karşılaştırması**

| Modül | OpenJarvis | SENTIENT | Avantaj |
|-------|-----------|----------|---------|
| Engine | 14 backend | 42 LLM provider | SENTIENT 3x fazla |
| Channels | 30 | 21 | OpenJarvis +9 |
| Security | PII/Secret/SSRF | Guardrails+TEE+ZK | SENTIENT çok güçlü |
| i18n | ❌ YOK | ✅ 8 dil | SENTIENT'de var |
| Persona | Sabit "Jarvis" | OCEAN modeli | SENTIENT gelişmiş |
| Desktop | ❌ | ✅ | SENTIENT'de var |
| Vision | ❌ | ✅ | SENTIENT'de var |
| Image/Video | ❌ | ✅ | SENTIENT'de var |
| Quantize | ❌ | ✅ | SENTIENT'de var |
| Connectors | 25+ | ❌ | OpenJarvis avantajı |
| A2A | ✅ | ❌ | OpenJarvis avantajı |
| Telemetry | GPU/Energy/FLOPs | Benchmarks | OpenJarvis detaylı |

**3. Karar: Fork ETME — Kendi sistemimizi geliştir**
- SENTIENT zaten daha güçlü temel
- Eksik: Connectors, A2A, Detaylı Telemetry, Morning Digest
- OpenJarvis'ten ilham alıp Rust'ta kendi versiyonumuzu yazacağız

**4. Kullanıcı Gereksinimleri**
- ✅ Çoklu dil desteği (özellikle Türkçe)
- ✅ Kullanıcı asistana kendi ismini koyabilmeli ("Jarvis" zorunlu değil)
- ✅ Kişisel AI deneyimi (günlük bülten, connector'lar, sesli asistan)

---

## 🏗️ 4 AŞAMALI PLAN

### SPRINT 1 (Hafta 1-2): Özelleştirilebilir Kimlik
| Görev | Dosya | Durum |
|-------|-------|-------|
| sentient init (ilk kurulum sihirbazı) | `crates/sentient_setup/src/wizard.rs` | ✅ Güncellendi |
| assistant_name config (tüm sistemde) | `crates/sentient_setup/src/config.rs` | ✅ Eklendi |
| Wake word özelleştirme ("Hey {name}") | `crates/sentient_setup/src/wizard.rs` | ✅ configure_voice() |
| i18n'de persona ismi entegrasyonu | `crates/sentient_i18n/src/translations.rs` | ✅ assistant.* anahtarları |
| CLI: sentient ask / sentient chat | `crates/sentient_cli/src/main.rs` | ✅ run_ask() + run_chat() |
| CLI: sentient doctor | `crates/sentient_cli/src/main.rs` | ✅ run_doctor() |
| Persona template sistemi | `crates/sentient_persona/src/templates.rs` | ✅ 6 yeni kişilik |
| SetupConfig: .env export | `crates/sentient_setup/src/config.rs` | ✅ ASSISTANT_NAME vb. |
| CLI: sentient init | `crates/sentient_cli/src/main.rs` | ✅ run_init() |
| CLI: Banner asistan ismini gösteriyor | `crates/sentient_cli/src/main.rs` | ✅ print_banner() |
| CLI: sentient-setup dependency | `crates/sentient_cli/Cargo.toml` | ✅ Eklendi |

### SPRINT 2 (Hafta 3-4): Connector'lar ✅ TAMAMLANDI
| Görev | Dosya | Durum |
|-------|-------|-------|
| sentient_connectors crate oluştur | `crates/sentient_connectors/` | ✅ |
| Connector trait + registry | `crates/sentient_connectors/src/connector.rs` | ✅ |
| Gmail connector (OAuth2) | `crates/sentient_connectors/src/gmail.rs` | ✅ |
| Google Calendar connector | `crates/sentient_connectors/src/calendar.rs` | ✅ |
| Weather connector (OpenWeatherMap) | `crates/sentient_connectors/src/weather.rs` | ✅ |
| RSS/Atom connector (feed-rs) | `crates/sentient_connectors/src/rss.rs` | ✅ |
| GitHub Notifications connector | `crates/sentient_connectors/src/github.rs` | ✅ |
| OAuth2 flow manager (device flow dahil) | `crates/sentient_connectors/src/oauth.rs` | ✅ |
| Types: Document, Email, CalendarEvent, WeatherData | `crates/sentient_connectors/src/types.rs` | ✅ |
| Error handling | `crates/sentient_connectors/src/error.rs` | ✅ |
| CLI: sentient connect / sentient sync | `crates/sentient_cli/` | ⬜ Sprint 3 |
| Obsidian connector (local) | `crates/sentient_connectors/src/obsidian.rs` | ⬜ Opsiyonel |

### SPRINT 5 (Hafta 9-10): Voice Assistant ✅ TAMAMLANDI
| Görev | Dosya | Durum |
|-------|-------|-------|
| sentient_voice crate oluştur | `crates/sentient_voice/` | ✅ |
| TTS engine (Text-to-Speech) | `crates/sentient_voice/src/tts.rs` | ✅ OpenAI, ElevenLabs |
| STT engine (Speech-to-Text) | `crates/sentient_voice/src/stt.rs` | ✅ OpenAI Whisper |
| Wake word detection | `crates/sentient_voice/src/wake.rs` | ✅ |
| Audio capture/playback | `crates/sentient_voice/src/audio.rs` | ✅ |
| Voice assistant integration | `crates/sentient_voice/src/assistant.rs` | ✅ |
| VAD (Voice Activity Detection) | `crates/sentient_voice/src/vad.rs` | ✅ |
| Gateway compatibility | `crates/sentient_voice/src/lib.rs` | ✅ StreamConfig, etc |

### SPRINT 6 (Hafta 11-12): CLI Voice Komutları ✅ TAMAMLANDI
| Görev | Dosya | Durum |
|-------|-------|-------|
| CLI: sentient voice komutu | `crates/sentient_cli/src/main.rs` | ✅ |
| --speak: TTS test | `crates/sentient_cli/src/main.rs` | ✅ |
| --listen: STT test | `crates/sentient_cli/src/main.rs` | ✅ |
| --wake: Wake word modu | `crates/sentient_cli/src/main.rs` | ✅ |

### SPRINT 4 (Hafta 7-8): Gelişmiş Özellikler ✅ TAMAMLANDI
| Görev | Dosya | Durum |
|-------|-------|-------|
| A2A Protocol (Google Agent-to-Agent) | `crates/sentient_a2a/` | ✅ |
| Telemetry (GPU/Energy/FLOPs) | `crates/sentient_observability/src/telemetry.rs` | ✅ |
| Deep Research Agent | `crates/sentient_research/` | ✅ |
| Arapça, Korece, Portekizce i18n | `crates/sentient_i18n/src/translations.rs` | ✅ |

### SPRINT 8 (Hafta 13-14): Background Daemon ✅ TAMAMLANDI
| Görev | Dosya | Durum |
|-------|-------|-------|
| sentient_daemon crate | `crates/sentient_daemon/` | ✅ |
| CommandParser (TR/EN) | `crates/sentient_daemon/src/commands.rs` | ✅ |
| VoiceActionExecutor | `crates/sentient_daemon/src/actions.rs` | ✅ |
| SentientDaemon (ana loop) | `crates/sentient_daemon/src/daemon.rs` | ✅ |
| YouTube music/video | `crates/sentient_daemon/src/actions.rs` | ✅ |

---

## 📝 İŞLEM GÜNLÜĞÜ

| Tarih | İşlem | Dosya | Durum |
|-------|-------|-------|-------|
| 2026-04-15 | AKTIF_GOREVLER.md güncellendi (15/15 tamam) | `archive/dev_reports/AKTIF_GOREVLER.md` | ✅ |
| 2026-04-15 | OpenJarvis repo klonlandı ve incelendi | `/tmp/OpenJarvis/` | ✅ |
| 2026-04-15 | PERSONAL_AI_ROADMAP.md oluşturuldu | `PERSONAL_AI_ROADMAP.md` | ✅ |
| 2026-04-15 | SetupConfig: assistant_name + personality + voice_enabled eklendi | `crates/sentient_setup/src/config.rs` | ✅ |
| 2026-04-15 | SetupWizard: asistan ismi seçimi (10 preset + custom) | `crates/sentient_setup/src/wizard.rs` | ✅ |
| 2026-04-15 | SetupWizard: kişilik tarzı seçimi (6 seçenek) | `crates/sentient_setup/src/wizard.rs` | ✅ |
| 2026-04-15 | SetupWizard: sesli asistan yapılandırması | `crates/sentient_setup/src/wizard.rs` | ✅ |
| 2026-04-15 | SetupWizard: başarı/goodbye mesajları asistan ismini kullanıyor | `crates/sentient_setup/src/wizard.rs` | ✅ |
| 2026-04-15 | SetupConfig: .env'e ASSISTANT_NAME/PERSONALITY/LANGUAGE/VOICE eklendi | `crates/sentient_setup/src/config.rs` | ✅ |
| 2026-04-15 | Persona templates: 6 yeni kişilik (friendly/professional/technical/casual/creative/mentor) | `crates/sentient_persona/src/templates.rs` | ✅ |
| 2026-04-15 | i18n: assistant.* anahtarları eklendi (TR + EN) | `crates/sentient_i18n/src/translations.rs` | ✅ |
| 2026-04-15 | CLI: sentient ask/chat/doctor komutları geliştiriliyor | `crates/sentient_cli/src/` | 🔄 |
| 2026-04-15 | CLI: sentient ask — tek soru sor, cevap al | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient chat — interaktif sohbet | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient init — setup wizard entegrasyonu | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient doctor — sistem kontrolü (6 check) | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: Banner asistan ismini gösteriyor | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient-setup dependency eklendi | `crates/sentient_cli/Cargo.toml` | ✅ |
| 2026-04-15 | CLI: reqwest dependency eklendi (health check) | `crates/sentient_cli/Cargo.toml` | ✅ |
| 2026-04-15 | Sprint 1 TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 2 başlıyor — sentient_connectors crate | — | 🔄 |
| 2026-04-15 | sentient_connectors crate oluşturuldu | `crates/sentient_connectors/` | ✅ |
| 2026-04-15 | Connector trait + Registry | `crates/sentient_connectors/src/connector.rs` | ✅ |
| 2026-04-15 | OAuth2 Manager (device flow dahil) | `crates/sentient_connectors/src/oauth.rs` | ✅ |
| 2026-04-15 | Types: Document, Credentials, Email, Calendar, Weather, Feed | `crates/sentient_connectors/src/types.rs` | ✅ |
| 2026-04-15 | Weather connector (OpenWeatherMap) | `crates/sentient_connectors/src/weather.rs` | ✅ |
| 2026-04-15 | RSS connector (feed-rs) | `crates/sentient_connectors/src/rss.rs` | ✅ |
| 2026-04-15 | GitHub connector (notifications, issues) | `crates/sentient_connectors/src/github.rs` | ✅ |
| 2026-04-15 | Gmail connector (Google Gmail API) | `crates/sentient_connectors/src/gmail.rs` | ✅ |
| 2026-04-15 | Calendar connector (Google Calendar API) | `crates/sentient_connectors/src/calendar.rs` | ✅ |
| 2026-04-15 | Workspace'e eklendi | `Cargo.toml` | ✅ |
| 2026-04-15 | Sprint 2 — Connectors TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 3 başlıyor — sentient_digest crate | — | 🔄 |
| 2026-04-15 | sentient_digest crate oluşturuldu | `crates/sentient_digest/` | ✅ |
| 2026-04-15 | DigestEngine — ana orkestratör | `crates/sentient_digest/src/engine.rs` | ✅ |
| 2026-04-15 | Collector trait + Registry | `crates/sentient_digest/src/collector.rs` | ✅ |
| 2026-04-15 | 6 built-in collector (Weather, Calendar, Email, News, Tasks, Greeting) | `crates/sentient_digest/src/collector.rs` | ✅ |
| 2026-04-15 | Composer — text + HTML output | `crates/sentient_digest/src/composer.rs` | ✅ |
| 2026-04-15 | Scheduler — cron-based zamanlama | `crates/sentient_digest/src/scheduler.rs` | ✅ |
| 2026-04-15 | Section builders | `crates/sentient_digest/src/sections.rs` | ✅ |
| 2026-04-15 | Template Registry — tr/en/de şablonları | `crates/sentient_digest/src/templates.rs` | ✅ |
| 2026-04-15 | Types: Digest, DigestSection, DigestItem, DigestConfig | `crates/sentient_digest/src/types.rs` | ✅ |
| 2026-04-15 | Sprint 3 — Morning Digest TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 4 başlıyor — CLI + A2A | — | 🔄 |
| 2026-04-15 | CLI: sentient digest komutu | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient connect komutu | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient sync komutu | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | sentient_a2a crate oluşturuldu | `crates/sentient_a2a/` | ✅ |
| 2026-04-15 | A2A Message types (15 message type) | `crates/sentient_a2a/src/message.rs` | ✅ |
| 2026-04-15 | A2A Agent types (AgentId, Capability, Metadata) | `crates/sentient_a2a/src/agent.rs` | ✅ |
| 2026-04-15 | A2A Registry (register, discover, capability index) | `crates/sentient_a2a/src/registry.rs` | ✅ |
| 2026-04-15 | A2A Transport (HTTP, WebSocket, InProcess) | `crates/sentient_a2a/src/transport.rs` | ✅ |
| 2026-04-15 | A2A Protocol impl (send, request, broadcast) | `crates/sentient_a2a/src/protocol.rs` | ✅ |
| 2026-04-15 | Sprint 4 — CLI + A2A TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 5 başlıyor — Voice Assistant | — | 🔄 |
| 2026-04-15 | sentient_voice crate oluşturuldu | `crates/sentient_voice/` | ✅ |
| 2026-04-15 | TTS engine (OpenAI, ElevenLabs) | `crates/sentient_voice/src/tts.rs` | ✅ |
| 2026-04-15 | STT engine (OpenAI Whisper) | `crates/sentient_voice/src/stt.rs` | ✅ |
| 2026-04-15 | Wake word detection | `crates/sentient_voice/src/wake.rs` | ✅ |
| 2026-04-15 | Audio capture/playback | `crates/sentient_voice/src/audio.rs` | ✅ |
| 2026-04-15 | Voice assistant orchestration | `crates/sentient_voice/src/assistant.rs` | ✅ |
| 2026-04-15 | VAD (Voice Activity Detection) | `crates/sentient_voice/src/vad.rs` | ✅ |
| 2026-04-15 | Gateway compatibility types | `crates/sentient_voice/src/lib.rs` | ✅ |
| 2026-04-15 | Workspace'e eklendi | `Cargo.toml` | ✅ |
| 2026-04-15 | Sprint 5 — Voice Assistant TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 6 başlıyor — CLI voice komutu | — | 🔄 |
| 2026-04-15 | CLI: sentient voice komutu eklendi | `crates/sentient_cli/src/main.rs` | ✅ |
| 2026-04-15 | CLI: sentient_voice dependency eklendi | `crates/sentient_cli/Cargo.toml` | ✅ |
| 2026-04-15 | Sprint 6 — CLI Voice TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 7 başlıyor — Gelişmiş Özellikler | — | 🔄 |
| 2026-04-15 | Telemetry: GPU/Energy/FLOPs tracking | `crates/sentient_observability/src/telemetry.rs` | ✅ |
| 2026-04-15 | Research Agent: Source search, analysis, report | `crates/sentient_research/` | ✅ |
| 2026-04-15 | i18n: Arapça, Korece, Portekizce (11 dil) | `crates/sentient_i18n/` | ✅ |
| 2026-04-15 | Sprint 7 — Gelişmiş Özellikler TAMAMLANDI ✅ | — | ✅ |
| 2026-04-15 | Sprint 8 başlıyor — Background Daemon | — | 🔄 |
| 2026-04-15 | sentient_daemon crate oluşturuldu | `crates/sentient_daemon/` | ✅ |
| 2026-04-15 | CommandParser: TR/EN komut ayrıştırma | `crates/sentient_daemon/src/commands.rs` | ✅ |
| 2026-04-15 | VoiceActionExecutor: Browser aksiyonları | `crates/sentient_daemon/src/actions.rs` | ✅ |
| 2026-04-15 | SentientDaemon: Ana koordinasyon | `crates/sentient_daemon/src/daemon.rs` | ✅ |
| 2026-04-15 | YouTube müzik/video açma desteği | `crates/sentient_daemon/src/actions.rs` | ✅ |
| 2026-04-15 | Sprint 8 — Background Daemon TAMAMLANDI ✅ | — | ✅ |

---

## 📂 REFERANS DOSYALARI

### SENTIENT DOSYALARI (DEĞİŞİKLİK YAPILACAK)

| Dosya | Yol | Sprint |
|-------|-----|--------|
| Persona tanımı | `crates/sentient_persona/src/persona.rs` | S1 |
| Persona builder | `crates/sentient_persona/src/builder.rs` | S1 |
| Persona loader | `crates/sentient_persona/src/loader.rs` | S1 |
| Persona templates | `crates/sentient_persona/src/templates.rs` | S1 |
| i18n çeviriler | `crates/sentient_i18n/src/translations.rs` | S1 |
| i18n formatter | `crates/sentient_i18n/src/formatter.rs` | S1 |
| i18n locale | `crates/sentient_i18n/src/locale.rs` | S1 |
| CLI main | `crates/sentient_cli/src/main.rs` | S1 |
| CLI commands | `crates/sentient_cli/src/commands/mod.rs` | S1 |
| Setup wizard | `crates/sentient_setup/src/` | S1 |
| Wake word | `crates/sentient_wake/src/lib.rs` | S1 |
| Core config | `crates/sentient_core/src/` (varsa) | S1 |
| Calendar | `crates/sentient_calendar/src/` | S3 |
| Email | `crates/sentient_email/src/` | S3 |
| Channels | `crates/sentient_channels/src/lib.rs` | S3 |

### OPENJARVIS REFERANS DOSYALARI (İLHAM)

| Dosya | Yol | Ne İçin |
|-------|-----|---------|
| Config sistemi | `/tmp/OpenJarvis/src/openjarvis/core/config.py` | Config yapısı |
| Persona sabiti | `/tmp/OpenJarvis/src/openjarvis/core/config.py:1318` | assistant_name |
| Connector trait | `/tmp/OpenJarvis/src/openjarvis/connectors/_stubs.py` | Connector ABC |
| Connector list | `/tmp/OpenJarvis/src/openjarvis/connectors/__init__.py` | 25+ connector |
| Morning Digest | `/tmp/OpenJarvis/src/openjarvis/agents/morning_digest.py` | Digest agent |
| CLI init | `/tmp/OpenJarvis/src/openjarvis/cli/init_cmd.py` | init komutu |
| CLI connect | `/tmp/OpenJarvis/src/openjarvis/cli/connect_cmd.py` | connect komutu |
| CLI digest | `/tmp/OpenJarvis/src/openjarvis/cli/digest_cmd.py` | digest komutu |
| Engine discovery | `/tmp/OpenJarvis/src/openjarvis/engine/_discovery.py` | Auto-discovery |
| Registry | `/tmp/OpenJarvis/src/openjarvis/core/registry.py` | Decorator registry |
| Event Bus | `/tmp/OpenJarvis/src/openjarvis/core/events.py` | Event system |
| Channel stubs | `/tmp/OpenJarvis/src/openjarvis/channels/_stubs.py` | Channel ABC |
| Telemetry | `/tmp/OpenJarvis/src/openjarvis/telemetry/__init__.py` | GPU/Energy/FLOPs |
| A2A Protocol | `/tmp/OpenJarvis/src/openjarvis/a2a/__init__.py` | Google A2A |
| Skill system | `/tmp/OpenJarvis/src/openjarvis/skills/__init__.py` | Skill import |
| Security | `/tmp/OpenJarvis/src/openjarvis/security/__init__.py` | Guardrails |

---

*Bu dosya her işlem sonrası güncellenecektir.*
*SENTIENT OS - The Operating System That Thinks*
