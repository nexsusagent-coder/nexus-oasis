# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - TEK YAPILACAKLAR LİSTESİ (Master Checklist)
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-13 | Kaynak: Arsiv klasöründeki 62 dosyanın tamamı
#  Her madde: NE yapılacak + NEDEN yapılacak + HANGİ DOSYADAN geldi
# ═══════════════════════════════════════════════════════════════════════════════

---

# 🔴 FAZ 1: SİSTEMİ AYAĞA KALDIR (Bugün - 3 Gün)

## 1.1 Altyapı Başlatma
- [ ] **Docker servisleri başlat** → NEDEN: Veritabanı, cache, vektör DB gerekli → KAYNAK: SISTEMI_AYAGA_KALDIRMA_REHBERI
  - postgres, redis, qdrant, minio, prometheus, grafana
  - `docker-compose up -d postgres redis qdrant minio prometheus grafana`
- [ ] **Ollama başlat + model indir** → NEDEN: Lokal LLM = $0 maliyet → KAYNAK: LOCAL_FIRST_KURULUM_PLANI
  - `systemctl start ollama`
  - `ollama pull gemma3:27b` ve `ollama pull deepseek-r1:67b`
- [ ] **.env yapılandır** → NEDEN: API key ve JWT olmadan sistem çalışmaz → KAYNAK: SISTEMI_AYAGA_KALDIRMA_REHBERI
  - API key (OpenRouter/OpenAI/Anthropic - en az biri)
  - JWT secret üret: `openssl rand -base64 64`
  - Gateway API key üret: `openssl rand -hex 32`
- [ ] **Gateway başlat (port 8080)** → NEDEN: Dashboard ve API erişimi için → KAYNAK: SISTEMI_AYAGA_KALDIRMA_REHBERI
  - `cargo run --bin sentient-gateway`
  - Health check: `curl http://localhost:8080/health`
- [ ] **Integration test çalıştır** → NEDEN: Runtime'da gerçekten çalışıyor mu? → KAYNAK: SISTEM_ANALIZ_VE_KURULUM_RAPORU
  - Henüz yapılmadı, Docker servisleri gerekli

## 1.2 Voice Entegrasyonu (JARVIS Hissi İçin KRİTİK)
- [ ] **Voice → Gateway bağlantısı** → NEDEN: Dashboard'dan sesli konuşma için → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_gateway/src/voice.rs` (~150 satır)
  - WebSocket voice stream handler
- [ ] **Voice → Channels bağlantısı** → NEDEN: Telegram/Discord'dan sesli komut → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_channels/src/voice_handler.rs` (~250 satır)
  - Telegram voice message → STT → LLM → TTS → voice response
- [ ] **Voice → Desktop bağlantısı** → NEDEN: Sesle bilgisayar kontrolü → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/oasis_autonomous/src/voice_control.rs` (~200 satır)
  - "Browser aç", "Ekranda ne var?", "Şuraya tıkla"
- [ ] **Dashboard Voice UI** → NEDEN: Mic button ile web'den konuşma → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI
  - `dashboard/assets/js/voice.js` + `voice.html` (~400 satır)
  - Mic button, waveform, transcription display

## 1.3 Dashboard Eksikleri
- [ ] **Setup Wizard UI** → NEDEN: Kullanıcılar terminal yerine web'den kurulum istiyor → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI, KURULUM_SISTEMI_ANALIZI
- [ ] **LLM Provider Yönetimi UI** → NEDEN: 42 provider'ı web'den yönetmek → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI
- [ ] **Channel Yönetimi UI** → NEDEN: 20+ kanalı aktif/pasif yapmak → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI
- [ ] **Agent Spawn UI** → NEDEN: Dashboard'dan ajan oluşturmak → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI
- [ ] **Permission Editor UI** → NEDEN: Güvenlik ayarlarını yönetmek → KAYNAK: DASHBOARD_KONTROL_PANELI_PLANI

---

# 🟠 FAZ 2: AKILLI ROUTING + PROAKTİF ENGINE (Hafta 2-4)

## 2.1 Akıllı LLM Router
- [ ] **DeBERTa classifier ile task routing** → NEDEN: %60-90 maliyet düşüşü → KAYNAK: Reddit (366 upvote), JARVIS_SEVIYESI_KAPSAMLI_ARASTIRMA
  - Basit task → ucuz model (Haiku, DeepSeek)
  - Karmaşık task → pahalı model (Opus, GPT-4)
  - `crates/sentient_llm/src/router.rs` (GENİŞLETME)

## 2.2 Proactive Engine
- [ ] **Time-based triggers** → NEDEN: "Saat 09:00 → Güne hazırlan" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN, TAM_OTONOM_VIZYON
  - `crates/sentient_proactive/` (YENİ CRATE, ~1,500 satır)
- [ ] **Event-based triggers** → NEDEN: "Email geldi → Acil mi?" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Pattern-based triggers** → NEDEN: "Her Cuma → Haftalık rapor" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Cron scheduling** → NEDEN: Zamanlanmış görevler → KAYNAK: crab-code CronCreate pattern
  - `crates/sentient_proactive/src/cron.rs`

## 2.3 Email Integration
- [ ] **Gmail API entegrasyonu (OAuth2)** → NEDEN: JARVIS gibi email okuma/yazma → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN, GAIA projesi
  - `crates/sentient_email/` (YENİ CRATE, ~800 satır)
- [ ] **IMAP/SMTP generic** → NEDEN: Gmail dışı sağlayıcılar → KAYNAK: INTEGRATION_LOG (Email ❌ SİLİNDİ)
- [ ] **Email özetleme + aksiyon önerme** → NEDEN: Proaktif davranış → KAYNAK: GAIA

## 2.4 Calendar Integration
- [ ] **Google Calendar API** → NEDEN: Toplantı hazırlığı, hatırlatma → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_calendar/` (YENİ CRATE, ~600 satır)
- [ ] **Outlook Calendar API** → NEDEN: Enterprise kullanıcılar → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Sesli hatırlatma** → NEDEN: "Toplantına 15 dk var" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN

## 2.5 Smart Todo System
- [ ] **Self-researching tasks** → NEDEN: Todo → araştır → taslak yaz → onay iste → KAYNAK: GAIA projesi
  - `crates/sentient_todo/` (YENİ CRATE, ~500 satır)
- [ ] **Task decomposition** → NEDEN: Büyük task'ları alt görevlere böl → KAYNAK: TAM_OTONOM_VIZYON

---

# 🟡 FAZ 3: SMART HOME + SOCIAL MEDIA (Hafta 4-7)

## 3.1 Smart Home Integration
- [ ] **Home Assistant MCP Server** → NEDEN: Akıllı ev kontrolü = gerçek JARVIS → KAYNAK: home-assistant-vibecode-agent, JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_home/` (YENİ CRATE, ~1,000 satır)
  - Işık, klima, kilit, kamera kontrolü
- [ ] **Sesli komutla cihaz kontrolü** → NEDEN: "Işıkları aç" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Otomasyon tetikleme** → NEDEN: "Evden çıkınca ışıklar kapansın" → KAYNAK: Home Assistant

## 3.2 SearXNG Local Search
- [ ] **SearXNG Docker + API client** → NEDEN: Tamamen yerel web arama → KAYNAK: agenticSeek, LOCAL_FIRST_KURULUM_PLANI
  - `crates/sentient_search/src/searxng.rs` (~300 satır)
- [ ] **Rate limiting + result parsing** → NEDEN: Stabil arama deneyimi → KAYNAK: agenticSeek

## 3.3 Social Media Automation
- [ ] **Reddit automation skill** → NEDEN: Karma artırma, değerli yorum → KAYNAK: claude-skill-reddit (30⭐)
  - `crates/sentient_social/` (YENİ CRATE)
- [ ] **Instagram content creation** → NEDEN: AI ile görsel + metin üretimi → KAYNAK: Sosyal medya araştırması
- [ ] **Anti-bot bypass (oasis_hands)** → NEDEN: Gerçek browser = tespit edilemez → KAYNAK: claude-skill-reddit

---

# 🔵 FAZ 4: SPEAKER ID + EMOTION + SKILL WEAVER (Hafta 7-9)

## 4.1 Speaker Identification
- [ ] **pyannote-audio FFI bridge** → NEDEN: Kimin konuştuğunu tanı → KAYNAK: pyannote-audio (9,708⭐), JARVIS_SEVIYESI_ARASTIRMA
  - `crates/sentient_voice/src/speaker_id.rs` (~500 satır)
- [ ] **Voice biometrics registration** → NEDEN: Multi-user ses profilleri → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Access control based on voice** → NEDEN: Sadece yetkili kişinin komutlarını dinle → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN

## 4.2 Emotion Detection
- [ ] **Hume AI API entegrasyonu** → NEDEN: Sesteki duyguyu algıla → KAYNAK: Hume AI, JARVIS_SEVIYESI_ARASTIRMA
  - `crates/sentient_voice/src/emotion.rs` (~400 satır)
- [ ] **Mood-based response adaptation** → NEDEN: "Stresslisin, ara ver" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN

## 4.3 Skill Weaver (Otomatik Skill Üretici)
- [ ] **Screen watcher → skill generation** → NEDEN: Ne yaptığını izle → otomatik skill üret → KAYNAK: Reddit (335 upvote)
  - `crates/sentient_skills/src/weaver.rs`
- [ ] **Pattern extraction** → NEDEN: Tekrarlayan işleri tespit → KAYNAK: Sosyal medya araştırması

---

# ⚪ FAZ 5: MOBILE + DESKTOP APP + LSP (Hafta 9-12)

## 5.1 Desktop App (Tauri)
- [ ] **Tauri framework ile desktop app** → NEDEN: Electron değil, Rust native = hafif → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN, claw-code-rust
  - `apps/desktop/` (~3,000 satır)
  - System tray, global hotkey, voice widget
- [ ] **NOT: Electron KULLANMAYACAĞIZ** → NEDEN: Ağır ve yavaş, Tauri = Rust native → KAYNAK: GAIA (Electron kullanıyor, biz daha iyisini yaparız)

## 5.2 Mobile Remote Control
- [ ] **Web PWA interface** → NEDEN: Her cihazdan erişim → KAYNAK: hapi (3.4K⭐)
  - `crates/sentient_remote/` (YENİ CRATE)
- [ ] **Telegram Mini App** → NEDEN: Telegram'dan tam kontrol → KAYNAK: hapi, mrstack
- [ ] **Voice control (mikrofon)** → NEDEN: Telefondan sesli komut → KAYNAK: hapi
- [ ] **AFK modu** → NEDEN: Uzakta olsan bile onayla → KAYNAK: hapi

## 5.3 LSP Integration
- [ ] **Go-to-definition, references, hover** → NEDEN: Kod navigasyonu → KAYNAK: crab-code LSP, claw-code-rust LSP
  - `crates/sentient_devtools/src/lsp.rs`

---

# 🟣 FAZ 6: WORKFLOW + AGENT FARM + HEATMAP (Hafta 12-15)

## 6.1 Workflow Engine
- [ ] **Visual flow builder (Dashboard)** → NEDEN: n8n benzeri otomasyon → KAYNAK: GAIA, n8n, JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_workflow/` (YENİ CRATE, ~2,000 satır)
- [ ] **Pre-built workflow templates** → NEDEN: "Güne hazırlan", "Proje tamamla" → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN

## 6.2 Agent Farm
- [ ] **20+ paralel ajan yönetimi** → NEDEN: Büyük projelerde hız → KAYNAK: claude_code_agent_farm (781⭐)
  - `crates/sentient_agents/src/farm.rs` (GENİŞLETME)
  - Lock-based dosya koordinasyonu
  - Auto-recovery
  - Context window yönetimi

## 6.3 Heatmap Diff Viewer
- [ ] **Color-coded risk annotation** → NEDEN: Kod review'de odaklanma → KAYNAK: manaflow (1,004⭐)
  - `dashboard/assets/js/heatmap.js`

---

# 🟢 FAZ 7: CONTEXT ENGINEERING + LEARNING (Hafta 15-20)

## 7.1 Context Engineer
- [ ] **AGENTS.md standard desteği** → NEDEN: 4,727 kişi talep ediyor → KAYNAK: GitHub issue #6235
  - `AGENTS.md` (PROJE KÖKÜ)
- [ ] **PRP (Product Requirements Prompt) workflow** → NEDEN: Context engineering = 10x prompt engineering → KAYNAK: context-engineering-intro (13K⭐)
  - `crates/sentient_context/` (YENİ CRATE)

## 7.2 Continuous Learning
- [ ] **User behavior analysis** → NEDEN: Kullanıcıyı öğren → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
  - `crates/sentient_learning/` (YENİ CRATE, ~1,500 satır)
- [ ] **Preference learning** → NEDEN: Kişiselleştirme → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN
- [ ] **Adaptive personality** → NEDEN: Her kullanıcıya özel deneyim → KAYNAK: JARVIS_SEVIYESI_MASTER_PLAN

---

# 📋 KATMAN BAZLI EKSİKLİKLER (Arsiv'deki 17 Katman Analizinden)

## 🔴 Kritik (Hemen Çözülmeli)

| # | Eksiklik | Katman | Kaynak Dosya |
|---|----------|--------|-------------|
| 1 | Desktop Platform Implementation YOK | Katman 10 | KATMAN_10_PRESENTATION_LAYER_ANALIZ |
| 2 | Web Frontend Dashboard eksik | Katman 10 | KATMAN_10_PRESENTATION_LAYER_ANALIZ |
| 3 | Creusot Binary YOK | Katman 11 | KATMAN_11_OASIS_LAYER_ANALIZ |
| 4 | Desktop Platform Impl YOK (oasis) | Katman 11 | KATMAN_11_OASIS_LAYER_ANALIZ |
| 5 | MFA Implementation YOK | Katman 8 | KATMAN_8_ENTERPRISE_LAYER_ANALIZ |
| 6 | Password Policy Enforcement YOK | Katman 8 | KATMAN_8_ENTERPRISE_LAYER_ANALIZ |
| 7 | Skill Versioning YOK | Katman 7 | KATMAN_7_SKILL_LAYER_ANALIZ |
| 8 | Skill Dependency YOK | Katman 7 | KATMAN_7_SKILL_LAYER_ANALIZ |
| 9 | Local Whisper Default YOK | Katman 9 | KATMAN_9_MEDIA_LAYER_ANALIZ |
| 10 | gRPC YOK | Katman 6 | KATMAN_6_INTEGRATION_LAYER_ANALIZ |
| 11 | GPU Support YOK (sandbox) | Katman 3 | KATMAN_3_TOOL_LAYER_ANALIZ |
| 12 | Quantization Binary YOK | Katman 12 | KATMAN_12_AI_ML_LAYER_ANALIZ |

## 🟠 Yüksek Öncelikli

| # | Eksiklik | Katman | Kaynak Dosya |
|---|----------|--------|-------------|
| 1 | WebSocket Implementation eksik | Katman 10 | KATMAN_10 |
| 2 | Vault Backend Storage YOK | Katman 11 | KATMAN_11 |
| 3 | Multi-Agent Coordination eksik | Katman 11 | KATMAN_11 |
| 4 | SCIM Provisioning YOK | Katman 8 | KATMAN_8 |
| 5 | GDPR/KVKK Consent Flow YOK | Katman 8 | KATMAN_8 |
| 6 | Intent Trigger eksik | Katman 7 | KATMAN_7 |
| 7 | Plugin Hot-Reload YOK | Katman 7 | KATMAN_7 |
| 8 | Speaker Diarization eksik | Katman 9 | KATMAN_9 |
| 9 | GraphQL YOK | Katman 6 | KATMAN_6 |
| 10 | PyO3 Python Bridge YOK (data) | Katman 14 | KATMAN_14 |
| 11 | Local Sandbox eksik | Katman 3 | KATMAN_3 |
| 12 | AI-Assisted Code Review YOK | Katman 3 | KATMAN_3 |
| 13 | RAG Vector Store YOK | Katman 12 | KATMAN_12 |
| 14 | Streaming Parser YOK (LLM) | LLM | SENTIENT_LLM_DETAYLI_ANALIZ |
| 15 | OAuth2 YOK (gateway) | Gateway | SENTIENT_GATEWAY_DETAYLI_ANALIZ |

## 🟡 Orta Öncelikli

| # | Eksiklik | Katman | Kaynak Dosya |
|---|----------|--------|-------------|
| 1 | CLI GUI Mode YOK | Katman 10 | KATMAN_10 |
| 2 | Rate Limiting Middleware eksik | Katman 10 | KATMAN_10 |
| 3 | Desktop OCR Entegrasyonu YOK | Katman 10 | KATMAN_10 |
| 4 | Captcha Solver Stub | Katman 11 | KATMAN_11 |
| 5 | Audit Storage Backend YOK | Katman 8 | KATMAN_8 |
| 6 | Webhook Notifications YOK | Katman 8 | KATMAN_8 |
| 7 | Skill Test Framework YOK | Katman 7 | KATMAN_7 |
| 8 | Payment Integration YOK | Katman 7 | KATMAN_7 |
| 9 | Video Template Library sınırlı | Katman 9 | KATMAN_9 |
| 10 | Image Edit API YOK | Katman 9 | KATMAN_9 |
| 11 | OpenAPI Docs YOK (gateway) | Gateway | SENTIENT_GATEWAY_DETAYLI_ANALIZ |
| 12 | Cost Tracker eksik (LLM) | LLM | SENTIENT_LLM_DETAYLI_ANALIZ |
| 13 | Persistent State YOK (orchestrator) | Orchestrator | SENTIENT_ORCHESTRATOR_DETAYLI_ANALIZ |
| 14 | Workflow Engine YOK (orchestrator) | Orchestrator | SENTIENT_ORCHESTRATOR_DETAYLI_ANALIZ |
| 15 | Multi-Browser YOK | Katman 3 | KATMAN_3 |
| 16 | Distributed Scraping YOK | Katman 3 | KATMAN_3 |

---

# 📋 CHANNEL EKSİKLİKLERİ (INTEGRATION_LOG'dan)

| # | Platform | Durum | Neden |
|---|----------|-------|-------|
| 1 | Telegram | ❌ SİLİNDİ | teloxide API değişiklikleri |
| 2 | Discord | ❌ SİLİNDİ | serenity API değişiklikleri |
| 3 | Slack | ❌ SİLİNDİ | slack-api değişiklikleri |
| 4 | WhatsApp | ❌ SİLİNDİ | Business API gerekli |
| 5 | IRC | ❌ SİLİNDİ | irc crate desteği yok |
| 6 | Matrix | ❌ SİLİNDİ | matrix-sdk API değişiklikleri |
| 7 | Signal | ❌ YOK | Hiç implementasyon yok |
| 8 | Email (SMTP) | ❌ YOK | Hiç implementasyon yok |
| 9 | SMS (Twilio) | ❌ YOK | Hiç implementasyon yok |
| 10 | Facebook Messenger | ❌ YOK | Hiç implementasyon yok |
| 11 | Instagram DM | ❌ YOK | Hiç implementasyon yok |
| 12 | Twitter DM | ❌ YOK | Hiç implementasyon yok |
| 13 | Microsoft Teams | ❌ YOK | Hiç implementasyon yok |
| 14 | Google Chat | ❌ YOK | Hiç implementasyon yok |
| 15 | LINE | ⚠️ STUB | Sadece tanımlar var |

**NOT:** Telegram ve Discord örnekleri `/examples/` dizininde çalışır durumda. Sadece `crates/sentient_channels` içindeki implementasyonlar silinmiş. Örneklerden yararlanılabilir.

---

# 📋 DİĞER YAPILACAKLAR (GUNCEL_DURUM_VE_YAPILACAKLAR'dan)

## Topluluk & Pazarlama
- [ ] Discord sunucu oluştur
- [ ] 5 dakikalık "What is SENTIENT OS?" videosu
- [ ] Quick start demonstration videosu
- [ ] Multi-agent showcase videosu
- [ ] Patreon hesabı aç
- [ ] GitHub Sponsors başvurusu yap

## İlk Çalıştırma
- [ ] Hello World örneği çalıştır
- [ ] Chatbot örneği çalıştır
- [ ] Multi-agent örneği çalıştır
- [ ] CI/CD workflow'larını kontrol et

## Güvenlik (Katman 8'den)
- [ ] MFA implementasyonu (sadece flag var, gerçek impl yok)
- [ ] Password policy enforcement
- [ ] GDPR/KVKK consent flow
- [ ] Audit storage backend (sadece in-memory)
- [ ] SOC 2 compliance şablonları

---

# 📊 ÖZET İSTATİSTİKLER

| Kategori | Sayı |
|----------|------|
| 🔴 Bugün yapılacak | 9 |
| 🟠 Faz 2 (Hafta 2-4) | 11 |
| 🟡 Faz 3 (Hafta 4-7) | 7 |
| 🔵 Faz 4 (Hafta 7-9) | 7 |
| ⚪ Faz 5 (Hafta 9-12) | 7 |
| 🟣 Faz 6 (Hafta 12-15) | 5 |
| 🟢 Faz 7 (Hafta 15-20) | 5 |
| Katman kritik eksiklikleri | 12 |
| Katman yüksek eksiklikleri | 15 |
| Katman orta eksiklikleri | 16 |
| Channel eksiklikleri | 15 |
| Toplam yapılacak madde | ~109 |

---

*Bu belge Arsiv klasöründeki 62 dosyanın tamamı taranarak oluşturulmuştur.*
*Tarih: 2026-04-13*
*Sonraki adım: Faz 1 maddelerini sırayla uygulamaya başla*
