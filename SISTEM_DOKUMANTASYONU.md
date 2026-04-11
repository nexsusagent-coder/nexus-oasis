# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - TAM SİSTEM DÖKÜMANTASYONU (KAPSAMLI TARAMA)
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-11
#  Versiyon: v4.0.0
#  Güncelleme: Tam Sistem Taraması
# ═══════════════════════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 1: PROJE GENEL BAKIŞ
# ═══════════════════════════════════════════════════════════════════════════════

## 1.1 Proje Kimliği

| Özellik | Değer |
|---------|-------|
| **Proje Adı** | SENTIENT OS |
| **Slogan** | "The Operating System That Thinks" |
| **GitHub** | https://github.com/nexsusagent-coder/SENTIENT_CORE |
| **Ko-fi** | https://ko-fi.com/sentientos |
| **Lisans** | AGPL v3 (Dual Licensing) |
| **Dil** | Rust + Python |
| **Versiyon** | 4.0.0 |
| **Edition** | 2021 |

## 1.2 Proje İstatistikleri (TAM TARAMA)

| Metrik | Değer |
|--------|-------|
| **Workspace Crate Sayısı** | 71 crate |
| **Crate Rust Dosyası** | 829 dosya |
| **Crate Rust Kodu** | 152,877 satır |
| **Integrations Python** | 42,143 dosya |
| **Integrations Rust** | 2,341 dosya |
| **Integrations Markdown** | 13,197 dosya |
| **Toplam Rust Kodu** | 1,161,910 satır |
| **Cargo.lock Satır** | 15,043 satır |
| **Skill Sayısı** | 5,587+ skill |
| **Örnek Sayısı** | 19 örnek proje |
| **Test Dizini** | 4 test türü |

## 1.3 Entegrasyon Boyutları

| Klasör | Boyut |
|--------|-------|
| agents/ | 2.9 GB |
| framework/ | 4.8 GB |
| memory/ | 191 MB |
| browser/ | 122 MB |
| sandbox/ | 180 MB |
| tools/ | 311 MB |
| skills/ | 99 MB |
| search/ | 12 MB |
| cli/ | 45 MB |
| security/ | 43 MB |
| execution/ | 19 MB |
| cevahir_ai/ | 46 MB |
| rakipler/ | 102 MB |
| **TOPLAM** | **~8.7 GB** |

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 2: TÜM CRATE'LER (71 ADET)
# ═══════════════════════════════════════════════════════════════════════════════

## 2.1 BÜYÜKLÜK SIRASINA GÖRE (İLK 30)

| Sıra | Crate | Satır | İşlev |
|------|-------|-------|-------|
| 🥇 | oasis_hands | 36,741 | Desktop Automation + Human Mimicry |
| 🥈 | sentient_orchestrator | 11,235 | Agent Loop & Routing |
| 🥉 | sentient_gateway | 10,058 | API Gateway + Channels |
| 4 | oasis_autonomous | 6,773 | Tam Otonom Agent |
| 5 | sentient_memory | 6,182 | Agent Bellek |
| 6 | oasis_browser | 5,311 | Browser Automation |
| 7 | sentient_cli | 5,020 | CLI |
| 8 | sentient_rag | 3,831 | RAG Engine |
| 9 | sentient_channels | 3,736 | 23 Platform |
| 10 | sentient_vgate | 3,525 | V-GATE Security |
| 11 | sentient_mcp | 3,003 | MCP Protocol |
| 12 | oasis_manus | 2,921 | Docker Execution |
| 13 | sentient_setup | 2,876 | Setup Wizard |
| 14 | sentient_plugin | 2,868 | Plugin System |
| 15 | sentient_scout | 2,763 | Scouting |
| 16 | sentient_settings | 2,726 | Settings |
| 17 | sentient_tee | 2,683 | TEE Support |
| 18 | sentient_voice | 2,634 | Voice STT/TTS |
| 19 | oasis_vault | 2,417 | Secrets Manager |
| 20 | sentient_enterprise | 2,461 | Enterprise Features |
| 21 | sentient_core | 2,326 | Core |
| 22 | sentient_benchmarks | 2,320 | Benchmarks |
| 23 | sentient_compliance | 2,226 | SOC 2 |
| 24 | sentient_vision | 2,201 | Vision/Multimodal |
| 25 | sentient_finetuning | 2,195 | Fine-tuning |
| 26 | sentient_skills | 2,136 | Skills System |
| 27 | sentient_zk_mcp | 2,062 | Zero-Knowledge |
| 28 | sentient_research | 2,011 | Research |
| 29 | sentient_ingestor | 2,000 | Skill Ingestor |
| 30 | sentient_marketplace | 1,680 | Marketplace |

## 2.2 OASIS Serisi Detay (7 Crate)

### 🥇 oasis_hands (36,741 satır) - EN BÜYÜK CRATE

**L6: EXECUTION - İnsan gibi bilgisayar kontrolü**

#### Ana Modüller:
- `input.rs` (21,271 satır) - Fare/klavye kontrolü
- `sovereign.rs` (19,358 satır) - L1 Sovereign Policy
- `tools.rs` (18,494 satır) - Araç kayıt
- `vision.rs` (15,818 satır) - Ekran görüntüsü + OCR
- `agent.rs` (18,454 satır) - Desktop Agent
- `screen.rs` (14,566 satır) - Ekran yakalama
- `session.rs` (10,920 satır) - Oturum yönetimi
- `skill_loader.rs` (10,230 satır) - Skill yükleme

#### Human Mimicry (6 dosya):
- `bumblebee.rs` - Doğal fare hareketi
- `bezier.rs` - Bezier eğrileri
- `typing_dynamics.rs` - Yazma dinamiği
- `mouse_dynamics.rs` - Fare dinamiği
- `behavior_model.rs` - Davranış modeli

#### Sentient Tools (30 araç):
agent_tool, ask_user_question, bash, brief, browser, calendar, config, email, file_edit, file_read, file_write, git, glob, grep, lsp, mcp, memory, n8n, notify, pdf, screenshot, sed, skill, task, todo_write, translate, web_fetch, web_search

#### Wrappers (239 dosya):
OpenClaw ve OpenHarness tam uyumluluk wrapper'ları

#### Sovereign Policy:
```
İZİN VERİLEN: libreoffice, firefox, vscode, gnome-terminal, nautilus
YASAKLI: rm -rf, format, dd, chmod 777, curl | bash (50+ komut)
```

---

### oasis_autonomous (6,773 satır) - TAM OTONOM AGENT

**Perception → Decision → Action → Learn**

#### Modüller:
- `planner.rs` (1,211) - Görev planlama
- `screen.rs` (1,029) - Ekran anlama
- `safety.rs` (875) - Güvenlik sistemi
- `agent_loop.rs` (729) - Agent döngüsü
- `memory.rs` (452) - Gelişmiş bellek
- `orchestrator.rs` (423) - Multi-agent
- `vision.rs` (461) - Gelişmiş görü
- `tools.rs` (526) - Araç zincirleri
- `healing.rs` (569) - Self-healing

#### Agent State'leri:
Idle, Initializing, Perceiving, Deciding, Acting, Learning, Error, Stopped, Paused

---

### oasis_browser (5,311 satır) - BROWSER AUTOMATION

**Browser-use + LightPanda**

- `proxy.rs` - Proxy yönetimi
- `observation.rs` - Sayfa gözlem
- `profile.rs` - Browser profili
- `stealth.rs` - Gizlilik modu
- `recap.rs` - Sayfa özeti
- `sovereign.rs` - Sovereign policy
- `actions.rs` - Browser aksiyonları

---

### oasis_manus (2,921 satır) - DOCKER EXECUTION

**OpenManus asimilasyonu**

- `container.rs` - Docker container
- `planner.rs` - Execution planlama
- `executor.rs` - İcra motoru

---

### oasis_brain (1,203 satır) - COGNITIVE ENGINE

**Gemma 4 Kernel**

- `cognitive_loop.rs` - Bilişsel döngü
- `perception.rs` - Algılama
- `reasoning.rs` - Muhakeme
- `action.rs` - Aksiyon

---

### oasis_core (1,606 satır) - CORE RUNTIME

**Creusot Contracts - Matematiksel güvenlik**

- `contracts.rs` (23,466) - Creusot sözleşmeleri
- `runtime.rs` - Runtime
- `state.rs` - Durum yönetimi

---

### oasis_vault (2,417 satır) - SECURE SECRETS

- `vault.rs` - Vault implementasyonu
- `crypto.rs` - Kriptografi
- `secrets.rs` - Secrets yönetimi

---

## 2.3 SENTIENT Serisi Detay (Büyük Olanlar)

### sentient_orchestrator (11,235 satır)

**Agent Loop & Dynamic Routing**

- `agent.rs` - Agent implementasyonu
- `dynamic_router.rs` - Dinamik yönlendirme
- `execution.rs` - İcra motoru
- `goal.rs` - Hedef yönetimi
- `self_healing.rs` - Self-healing
- `swarm/` - Swarm modülü (coordinator, task_router, protocol, message, blackboard, collective)

---

### sentient_gateway (10,058 satır)

**API Gateway + Telegram + WebSocket**

- `dispatcher.rs` - İstek dağıtma
- `rate_limit.rs` - Hız sınırlama
- `task_manager.rs` - Görev yönetimi
- `claw3d.rs` - Claw3D entegrasyonu
- `api/` - REST API
- `auth/` - Kimlik doğrulama
- `telegram/` - Telegram bot
- `websocket/` - WebSocket

---

### sentient_memory (6,182 satır)

**Agent Bellek Sistemi**

- `consolidation.rs` - Bellek pekiştirme
- `cube.rs` - OLAP cube
- `decay.rs` - Bellek azalma
- `embeddings.rs` - Embedding yönetimi
- `fts.rs` - Full-text search
- `knowledge_graph.rs` - Bilgi grafiği
- `rag.rs` - RAG entegrasyonu
- `vector_index.rs` - Vektör indeksi

---

### sentient_channels (3,736 satır)

**23 Platform Desteği**

| Platform | Durum |
|----------|-------|
| Telegram | ✅ |
| Discord | ✅ |
| WhatsApp | ✅ |
| Slack | 🔄 |
| Teams | 🔄 |
| Signal | 🔄 |
| iMessage | 🔄 |
| Instagram | 🔄 |
| LinkedIn | 🔄 |
| Line | 🔄 |
| Mattermost | 🔄 |
| Messenger | 🔄 |
| Snapchat | 🔄 |
| Twitter | 🔄 |
| Viber | 🔄 |
| Webex | 🔄 |
| WeChat | 🔄 |
| Zoom | 🔄 |
| Chime | 🔄 |
| Google Chat | 🔄 |

---

### sentient_mcp (3,003 satır)

**Model Context Protocol**

- `client.rs` - MCP client
- `server.rs` - MCP server
- `protocol.rs` - Protokol
- `tool.rs` - Tool tanımlama
- `resource.rs` - Kaynak yönetimi
- `transport.rs` - Transport (HTTP, WS, Stdio)

---

## 2.4 Diğer Crate'ler (Kısa)

| Crate | Satır | İşlev |
|-------|-------|-------|
| sentient_setup | 2,876 | Setup Wizard |
| sentient_plugin | 2,868 | Plugin System |
| sentient_scout | 2,763 | Scouting |
| sentient_settings | 2,726 | Settings |
| sentient_tee | 2,683 | TEE Support |
| sentient_voice | 2,634 | Voice STT/TTS |
| sentient_enterprise | 2,461 | Enterprise (RBAC, SSO) |
| sentient_core | 2,326 | Core |
| sentient_benchmarks | 2,320 | Benchmarks |
| sentient_compliance | 2,226 | SOC 2 Compliance |
| sentient_vision | 2,201 | Vision/Multimodal |
| sentient_finetuning | 2,195 | Fine-tuning (LoRA) |
| sentient_skills | 2,136 | Skills System |
| sentient_zk_mcp | 2,062 | Zero-Knowledge |
| sentient_research | 2,011 | MindSearch, AutoResearch |
| sentient_ingestor | 2,000 | Skill Ingestor |
| sentient_marketplace | 1,680 | Marketplace |
| sentient_cevahir | 1,630 | Turkish LLM |
| sentient_sync | 1,609 | Auto-Update |
| sentient_schema | 1,547 | Structured Output |
| sentient_patterns | 1,545 | Agentic Patterns |
| sentient_backup | 1,526 | Backup & Restore |
| sentient_sandbox | 1,405 | E2B Sandbox |
| sentient_web | 1,406 | Web Server |
| sentient_session | 1,364 | Session |
| sentient_persona | 1,396 | Persona Builder |
| sentient_selfcoder | 1,324 | Self-Coding |
| sentient_sla | 1,312 | SLA Monitoring |
| sentient_forge | 1,311 | Build Tools |
| sentient_dr | 1,408 | Disaster Recovery |
| sentient_image | 1,263 | Image Generation |
| sentient_storage | 1,237 | Persistence |
| sentient_groq | 1,233 | Groq LPU |
| sentient_local | 1,157 | Local LLM |
| sentient_anomaly | 1,160 | Anomaly Detection |
| sentient_i18n | 1,049 | i18n (8 dil) |
| sentient_desktop | 1,021 | Computer Use |
| sentient_observability | 986 | OpenTelemetry |
| sentient_skills_import | 980 | Skills Import |
| sentient_wake | 914 | Wake Word |
| sentient_python | 968 | Python Bridge |
| sentient_reporting | 886 | Reporting |
| sentient_devtools | 707 | DevTools |
| sentient_checkpoint | 701 | Checkpoint |
| sentient_execution | 697 | Execution |
| sentient_modes | 686 | Operation Modes |
| sentient_graph | 585 | Workflow Graph |
| sentient_agents | 545 | Multi-Agent |
| sentient_guardrails | 307 | Guardrails |
| sentient_cluster | 338 | Kubernetes |
| sentient_lancedb | 638 | LanceDB |
| sentient_vector | 190 | Vector DB |

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 3: ENTEGRASYONLAR (72+ PROJE)
# ═══════════════════════════════════════════════════════════════════════════════

## 3.1 Agent Framework'leri (17 Proje)

agency-agents, agent-s, agentgpt, auto-gpt, autogen, autogen-studio, autoresearch, babyagi, camel-ai, crewai, goose, gpt-engineer, metagpt, openhands, praisonai, swarm, taskweaver

## 3.2 AI Framework'leri (22 Proje)

aider, anthropic-cookbook, autogluon, continue-dev, dify, fastgpt, gpt4all, haystack, langchain, llama-index-full, llama-recipes, llama_index, lms, ollama, open-webui, phidata, pydantic-ai, semantic-kernel, smolagents, storm, tensorflow, text-generation-webui

## 3.3 Memory & Vector DB (4 Proje)

chromadb, letta, qdrant, weaviate

## 3.4 Browser Automation (5 Proje)

agent-browser, browser-use, bytebot, lightpanda, open-computer-use

## 3.5 Sandbox (3 Proje)

daytona, e2b-sdk, localstack

## 3.6 Tools (5 Proje)

crawl4ai, firecrawl, judge0, mem0, ragflow

## 3.7 Skills Libraries (6 Proje)

| Proje | Skill Sayısı |
|-------|--------------|
| Claw3D | 5,143 |
| awesome-n8n-templates | 500+ |
| awesome-openclaw-skills | 200+ |
| deerflow-skills | 100+ |
| everything-claude-code | 181 |
| gstack | 37 |

## 3.8 Diğer (8 Proje)

- Search: MindSearch, searxng
- CLI: gemini-cli, google-workspace-cli
- Security: nemo-guardrails
- Execution: open-interpreter
- Turkish LLM: cevahir_ai (19 modül)
- Rakipler: OpenHarness, oh-my-claudecode, oh-my-codex, pi-mono

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 4: SKILL SİSTEMİ (5,587+ SKILL)
# ═══════════════════════════════════════════════════════════════════════════════

## 4.1 Skill Kategorileri

| Kategori | Skill Sayısı | Alt Kategoriler |
|----------|--------------|-----------------|
| Dev | 2,965+ | Coding-Agents (1,374), Web-Frontend (901), DevOps-Cloud (375), Git-GitHub (155), CLI-Tools (170), iOS-macOS (29) |
| OSINT | 1,050+ | Search-Research (339), Browser-Automation (336), Data-Analytics (35) |
| Social | 238+ | Communication (141), Marketing-Sales (97) |
| Automation | 306+ | Productivity (202), Calendar (64), Smart-Home (40) |
| Media | 246+ | Image-Video-Gen (164), Streaming (84), Speech (42) |
| Productivity | 214+ | Notes-PKM (69), PDF-Documents (102), Apple-Apps (43) |
| Security | 52+ | Security-Passwords (52) |
| Mobile | 233+ | Transportation (108), Health-Fitness (81), Shopping (45) |
| Gaming | 108+ | Gaming (25), Personal-Dev (48), Moltbook (35) |

## 4.2 Native Skills

code-review, web-researcher, debug-helper, git-workflow, competitor-analyzer, codegen, research, automation, analysis

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 5: ÖRNEK PROJELER (19 ADET)
# ═══════════════════════════════════════════════════════════════════════════════

agentic-patterns, basic-agent, chatbot, code-sandbox, custom-skill, discord-bot, enterprise-sso, groq-chat, hello-world, image-gen, kubernetes-deployment, multi-agent, production, streaming-agent, structured-output, telegram-bot, voice-agent, web-api, web-search

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 6: DİĞER KLASÖRLER
# ═══════════════════════════════════════════════════════════════════════════════

## 6.1 apps/
- desktop/ - Tauri desktop app
- mobile/ - React Native (Android + iOS)

## 6.2 data/
- asena.db - Ana veritabanı
- asena_graph.db - Bilgi grafiği
- asena_memory.db - Bellek DB
- asena_skills.db - Skill DB
- asena_vectors.db - Vektör DB

## 6.3 tests/
- e2e/ - End-to-end testler
- integration/ - İntegrasyon testleri
- property/ - Property testleri
- unit/ - Unit testler

## 6.4 docs/
API.md, API_REFERENCE.md, CHANNELS.md, DEPLOYMENT.md, GEMMA4_INTEGRATION_REPORT.md, GETTING_STARTED.md, KUBERNETES.md, OZ_PARCA_RAPORU.md, README.md, TESTING.md, USER_MANUAL.md, VOICE.md, openapi.yaml

## 6.5 config/
alerts.yml, grafana/, nginx.conf, prometheus.yml, ssl/

## 6.6 scripts/
init.sql, run_tests.sh, src/

## 6.7 deploy/
docker-compose.prod.yml, docker-deploy.sh, healthcheck.sh, install-production.sh, monitoring/, nginx/, sentient*.service

## 6.8 dashboard/
Tauri dashboard: main.rs, api.rs, skill_loader.rs, skills_hub.rs, tool_monitor.rs, vgate_panel.rs, ws.rs

## 6.9 benchmarks/
results/, scripts/

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 7: DOKÜMANTASYON DOSYALARI
# ═══════════════════════════════════════════════════════════════════════════════

| Dosya | Açıklama |
|-------|----------|
| README.md | Ana dokümantasyon (79KB) |
| ARCHITECTURE.md | Mimari dokümanı (21KB) |
| INSTALL.md | Kurulum rehberi (28KB) |
| SETUP.md | Kurulum adımları (23KB) |
| DEPLOYMENT.md | Deployment rehberi (11KB) |
| SECURITY.md | Güvenlik dokümanı (4KB) |
| SECURITY_DETAILED.md | Detaylı güvenlik (9KB) |
| USER_MANUAL.md | Kullanım kılavuzu (25KB) |
| ROADMAP.md | Yol haritası (6KB) |
| CHANGELOG.md | Değişiklik günlüğü (5KB) |
| CONTRIBUTING.md | Katkı rehberi (5KB) |
| CODE_OF_CONDUCT.md | Davranış kuralları (5KB) |
| GOVERNANCE.md | Yönetişim (5KB) |
| ENTERPRISE.md | Enterprise dokümanı (9KB) |
| SPONSORS.md | Sponsor programı (3KB) |
| AGENTS.md | Agent dokümanı (10KB) |
| MODEL_PROVIDERS.md | Provider dokümanı (8KB) |
| WHY_SENTIENT.md | Neden SENTIENT (17KB) |
| CEVAHIR_AI_LLM_RAPORU.md | Cevahir AI raporu (21KB) |
| RAKIP_ANALIZ_RAPORU.md | Rakip analizi (28KB) |
| SISTEM_DOKUMANTASYONU.md | Bu dosya |
| ENTEGRASTON_HEDEFLERI.md | Entegrasyon hedefleri (29KB) |

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 8: KURULUM DOSYALARI
# ═══════════════════════════════════════════════════════════════════════════════

| Dosya | Açıklama |
|-------|----------|
| Cargo.toml | Workspace yapılandırması |
| Cargo.lock | Bağımlılık kilidi (15,043 satır) |
| Dockerfile | Production Docker |
| Dockerfile.dev | Development Docker |
| Dockerfile.minimal | Minimal Docker |
| docker-compose.yml | Docker Compose |
| docker-compose.dev.yml | Dev Docker Compose |
| install.sh | Linux kurulum scripti |
| install.ps1 | Windows kurulum scripti |
| setup.sh | Linux setup |
| setup.ps1 | Windows setup |
| quick-install.sh | Hızlı kurulum |
| quick-install.ps1 | Hızlı kurulum (Windows) |
| sentient | Linux binary |
| sentient.bat | Windows batch |
| sentient.ps1 | Windows PowerShell |
| Makefile | Build komutları (16KB) |

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 9: ÖZET TABLOSU
# ═══════════════════════════════════════════════════════════════════════════════

| Metrik | Değer |
|--------|-------|
| **Crate Sayısı** | 71 |
| **Rust Dosya** | 829 |
| **Crate Rust Kodu** | 152,877 satır |
| **Integrations Python** | 42,143 dosya |
| **Integrations Rust** | 2,341 dosya |
| **Integrations Markdown** | 13,197 dosya |
| **Toplam Entegrasyon** | 72+ proje |
| **Skill Sayısı** | 5,587+ |
| **Örnek Proje** | 19 |
| **Kanal Desteği** | 23 platform |
| **LLM Provider** | 40+ |
| **LLM Model** | 600+ |

---

## En Büyük 5 Crate

| Sıra | Crate | Satır |
|------|-------|-------|
| 🥇 | oasis_hands | 36,741 |
| 🥈 | sentient_orchestrator | 11,235 |
| 🥉 | sentient_gateway | 10,058 |
| 4 | oasis_autonomous | 6,773 |
| 5 | sentient_memory | 6,182 |

---

**Tarih:** 2026-04-11
**Versiyon:** 4.0.0
**GitHub:** https://github.com/nexsusagent-coder/SENTIENT_CORE
**Ko-fi:** https://ko-fi.com/sentientos
