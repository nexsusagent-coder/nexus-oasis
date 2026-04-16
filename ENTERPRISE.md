# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - ENTERPRISE DATASHEET
# ═══════════════════════════════════════════════════════════════════════════════
#  Enterprise-Grade AI Agent Framework
#  Contact: enterprise@sentient.ai
# ═══════════════════════════════════════════════════════════════════════════════

# 📋 ÜRÜN ÖZETİ

SENTIENT OS, kurumsal yapay zeka agent'larını çalıştırmak, yönetmek ve 
ölçeklendirmek için tasarlanmış Rust-tabanlı bir AI işletim sistemidir.

**Tek Metrik:**
- 303,490 satır Rust kodu (93 crate)
- 560+ test
- 245+ native LLM model + 200K+ aggregator erişimi (57+ provider)
- 20+ kanal entegrasyonu
- 5,587+ skill
- 72+ entegre proje


# ═══════════════════════════════════════════════════════════════════════════════
#  ENTERPRISE ÖZELLİKLERİ
# ═══════════════════════════════════════════════════════════════════════════════

## 🔐 Güvenlik & Compliance (6 Güvenlik Crate'i)

| Özellik | Crate | Açıklama |
|---------|-------|----------|
| **SOC 2 Type II** | sentient_compliance (2,226 satır) | Compliance framework hazır |
| **RBAC** | sentient_enterprise (2,461 satır) | Role-based access control |
| **SSO** | sentient_enterprise | SAML 2.0, OIDC, LDAP |
| **Secret Management** | oasis_vault (2,417 satır) | Şifreli secret yönetimi, crypto |
| **Audit Log** | sentient_compliance | Tüm işlemler kayıt altında |
| **Data Encryption** | oasis_vault | AES-256-GCM, rest ve transit |
| **TEE** | sentient_tee (2,683 satır) | AMD SEV-SNP, Intel TDX |
| **ZK Proofs** | sentient_zk_mcp (2,062 satır) | Zero-knowledge MCP |
| **Anomaly Detection** | sentient_anomaly (1,160 satır) | İzinsiz giriş tespiti |
| **Guardrails** | sentient_guardrails (307 satır) | Prompt injection, data exfiltration |

## 🏗️ Infrastructure

| Özellik | Açıklama |
|---------|----------|
| **Multi-Tenant** | Tenant izolasyonu |
| **High Availability** | Cluster mode |
| **Disaster Recovery** | Backup & restore |
| **Observability** | Prometheus, Grafana, Jaeger |
| **Kubernetes Ready** | Helm charts, operators |

## 🤖 AI Capabilities

| Özellik | Crate | Açıklama |
|---------|-------|----------|
| **245+ Native Models** | sentient_llm (14,445 satır) | 57+ provider, 200K+ aggregator erişimi |
| **Smart Router** | sentient_llm::router | Zorluk bazlı otomatik model seçimi |
| **RAG Engine** | sentient_rag (3,831 satır) | Vector search, document processing |
| **Multi-Agent** | sentient_orchestrator (11,235 satır) | 6 framework: CrewAI, AutoGen, Swarm, MetaGPT |
| **Desktop Automation** | oasis_hands (36,741 satır) | 43+ tool, human mimicry |
| **Otonom Agent** | oasis_autonomous (6,773 satır) | Perceive→Decide→Act→Learn |
| **Skills** | sentient_skills (2,136 satır) | 5,587+ hazır yetenek |
| **Voice** | sentient_voice (2,634 satır) | STT, TTS, Wake Word, VAD |
| **Vision** | sentient_vision (2,201 satır) | OCR, multimodal |
| **Türkçe LLM** | sentient_cevahir (1,630 satır) | Cevahir AI V-7, cognitive strategies |
| **Proactive** | sentient_proactive | Zamanlı, olaylı, pattern tetikleyiciler |
| **Akıllı Ev** | sentient_home | Home Assistant entegrasyonu |
| **MCP** | sentient_mcp (3,003 satır) | Claude Desktop, Cursor uyumlu |

## 🔌 Integrations

| Kategori | Sayı |
|----------|------|
| **Messaging** | 20+ kanal (Telegram, Discord, Slack, WhatsApp, Teams...) |
| **Databases** | PostgreSQL, SQLite, Qdrant, ChromaDB, Redis |
| **Cloud** | AWS Bedrock, Azure OpenAI, GCP Vertex AI |
| **Connectors** | Gmail, Calendar, GitHub, Weather, RSS |
| **Browser** | Browser-Use, Lightpanda, Playwright |
| **Memory** | Mem0, ChromaDB, Qdrant, LanceDB |
| **Sandbox** | Docker, E2B SDK, LocalStack |


# ═══════════════════════════════════════════════════════════════════════════════
#  PRICING
# ═══════════════════════════════════════════════════════════════════════════════

## Self-Hosted (On-Premise)

| Plan | Fiyat | Users | Support |
|------|-------|-------|---------|
| **Starter** | $499/ay | 5 | Email |
| **Business** | $1,499/ay | 25 | Email + Chat |
| **Enterprise** | $4,999/ay | Unlimited | 24/7 + SLA |
| **Enterprise Plus** | Custom | Unlimited | Dedicated |

## Cloud (SaaS)

| Plan | Fiyat | Messages | Features |
|------|-------|----------|----------|
| **Pro** | $49/ay | 100K | Basic |
| **Team** | $199/ay | 500K | Analytics |
| **Enterprise** | $999+/ay | Unlimited | Full |

## One-Time License (Perpetual)

| Tip | Fiyat | Özellikler |
|-----|-------|------------|
| **Startup** | $5,000 | 1 yıl support |
| **SMB** | $15,000 | 1 yıl support + updates |
| **Enterprise** | $50,000+ | Lifetime + dedicated |


# ═══════════════════════════════════════════════════════════════════════════════
#  SLA GARANTİLERİ
# ═══════════════════════════════════════════════════════════════════════════════

| Metrik | Startup | SMB | Enterprise | Enterprise Plus |
|--------|---------|-----|------------|-----------------|
| **Uptime SLA** | 99% | 99.5% | 99.9% | 99.99% |
| **Response Time** | 8h | 4h | 2h | 1h |
| **Resolution Time** | 72h | 48h | 24h | 4h |
| **Support Hours** | Business | 24/5 | 24/5 | 24/7 |
| **Dedicated Engineer** | ❌ | ❌ | ❌ | ✅ |


# ═══════════════════════════════════════════════════════════════════════════════
#  RAKİP ANALİZİ
# ═══════════════════════════════════════════════════════════════════════════════

| Özellik | SENTIENT OS | LangChain | AutoGPT | OpenAI |
|---------|-------------|-----------|---------|--------|
| **Açık Kaynak** | ✅ | ✅ | ✅ | ❌ |
| **Self-Hosted** | ✅ | ✅ | ✅ | ❌ |
| **Enterprise Ready** | ✅ | ❌ | ❌ | ✅ |
| **600+ LLM Models** | ✅ | ~50 | ~10 | 5 |
| **23 Kanal** | ✅ | ~5 | 0 | 0 |
| **SOC 2 Compliance** | ✅ | ❌ | ❌ | ✅ |
| **Multi-Agent** | ✅ | ⚡ | ✅ | ❌ |
| **Skills Marketplace** | ✅ | ❌ | ❌ | ❌ |
| **Voice (STT/TTS)** | ✅ | ❌ | ❌ | ✅ |
| **Rust Performance** | ✅ | ❌ | ❌ | ❌ |
| **Türkçe LLM (Cevahir)** | ✅ | ❌ | ❌ | ❌ |


# ═══════════════════════════════════════════════════════════════════════════════
#  KULLANIM SENARYOLARI
# ═══════════════════════════════════════════════════════════════════════════════

## 🏦 Finans
- Fraud detection agents
- Customer service automation
- Compliance monitoring
- Risk analysis

## 🏥 Sağlık
- Patient intake automation
- Medical record analysis
- Appointment scheduling
- HIPAA-compliant chatbots

## 🛒 E-Ticaret
- Customer support agents
- Order processing
- Inventory management
- Personalized recommendations

## 🏭 Üretim
- Predictive maintenance
- Quality control agents
- Supply chain optimization
- IoT data analysis


# ═══════════════════════════════════════════════════════════════════════════════
#  İLETİŞİM
# ═══════════════════════════════════════════════════════════════════════════════

| Kanal | Değer |
|-------|-------|
| **Email** | enterprise@sentient.ai |
| **Discord** | discord.gg/sentient |
| **Phone** | +1 (555) SENTIENT |
| **Website** | https://sentient.ai/enterprise |
| **GitHub** | github.com/nexsusagent-coder/SENTIENT_CORE |

**Demo Talep Formu:** https://sentient.ai/enterprise/demo


# ═══════════════════════════════════════════════════════════════════════════════
#  SON GÜNCELLEME: 11 Nisan 2025
# ═══════════════════════════════════════════════════════════════════════════════
