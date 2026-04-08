# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT (NEXUS OASIS) - THIRD-PARTY COMPONENTS NOTICE
# ═══════════════════════════════════════════════════════════════════════════════
#
#  SENTIENT, çeşitli açık kaynak projelerin entegrasyonu ile güçlendirilmiştir.
#  Bu belge, kullanılan üçüncü taraf bileşenleri ve lisans koşullarını listeler.
#
#  SENTIENT'nın özgün Rust çekirdek kodu (crates/sentient_*) MIT lisansı ile lisanslanmıştır.
#  Aşağıdaki üçüncü taraf bileşenler kendi lisans koşullarına tabidir.
#
#  Son güncelleme: 2026-04-07
# ═══════════════════════════════════════════════════════════════════════════════

## ═════════════════════════════════════════════════════════════════════════════
##  BÖLÜM 1: RUST KÜTÜPHANELERİ (Cargo Dependencies)
## ═════════════════════════════════════════════════════════════════════════════

Bu kütüphaneler Cargo.toml'da tanımlıdır ve crates.io'dan derlenir.
Kaynak kodu SENTIENT deposuna dahil DEĞİLDİR.

| Kütüphane        | Sürüm  | Lisans       | Kaynak                        |
|------------------|--------|--------------|-------------------------------|
| tokio            | 1.x    | MIT          | https://tokio.rs              |
| serde            | 1.x    | MIT/Apache-2 | https://serde.rs              |
| serde_json       | 1.x    | MIT/Apache-2 | https://github.com/serde-rs   |
| uuid             | 1.x    | MIT/Apache-2 | https://github.com/uuid-rs    |
| chrono           | 0.4    | MIT/Apache-2 | https://github.com/chronotope |
| rusqlite         | 0.35   | MIT          | https://github.com/rusqlite   |
| reqwest          | 0.12   | MIT/Apache-2 | https://github.com/seanmonstar|
| axum             | 0.8    | MIT          | https://github.com/tokio-rs   |
| tower            | 0.5    | MIT          | https://github.com/tower-rs   |
| pyo3             | 0.25   | MIT/Apache-2 | https://pyo3.rs               |
| async-trait      | 0.1    | MIT/Apache-2 | https://github.com/dtolnay    |
| thiserror        | 2.x    | MIT/Apache-2 | https://github.com/dtolnay    |
| anyhow           | 1.x    | MIT/Apache-2 | https://github.com/dtolnay    |
| crossbeam        | 0.8    | MIT/Apache-2 | https://github.com/crossbeam-rs|
| parking_lot      | 0.12   | MIT/Apache-2 | https://github.com/Amanieu    |
| clap             | 4.x    | MIT/Apache-2 | https://github.com/clap-rs    |
| regex            | 1.x    | MIT/Apache-2 | https://github.com/rust-lang/regex|
| log              | 0.4    | MIT/Apache-2 | https://github.com/rust-lang/log|
| env_logger       | 0.11   | MIT/Apache-2 | https://github.com/env-logger-rs|
| tracing          | 0.1    | MIT          | https://github.com/tokio-rs/tracing|

## ═════════════════════════════════════════════════════════════════════════════
##  BÖLÜM 2: PYTHON ENTEGRASYONLARI (integrations/)
## ═════════════════════════════════════════════════════════════════════════════

Bu projeler /integrations/ dizininde referans alınmıştır.
SENTIENT, PyO3 köprüsü ile bu projeleri "native modül" olarak sarmalar.

### 2.1: Agent Frameworks

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| CrewAI         | MIT      | https://github.com/joaomdmoura/crewAI     | Çoklu ajan orkestrasyonu|
| AutoGen        | MIT      | https://github.com/microsoft/autogen      | Konuşma bazlı ajanlar   |
| AutoGPT        | MIT      | https://github.com/Significant-Gravitas/AutoGPT | Otonom görev döngüsü |
| OpenHands      | MIT      | https://github.com/All-Hands-AI/OpenHands | Kod yazma ajanı        |
| PraisonAI      | MIT      | https://github.com/mervinpraison/PraisonAI | Multi-framework ajan   |
| Phidata        | MIT      | https://github.com/phidatahq/phidata     | Agent toolbox           |
| Agent-S        | MIT      | https://github.com/sierra-research/agent-s | Masaüstü kontrolü     |
| Agency-Agents  | MIT      | https://github.com/mr-destructive/agency_agents | Basit ajan framework|

### 2.2: Browser & Web Scraping

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| Browser-Use    | MIT      | https://github.com/browser-use/browser-use| LLM kontrolü tarayıcı   |
| Lightpanda     | MIT      | https://github.com/lightpanda-org         | Headless browser engine |

### 2.3: Memory & Knowledge Graph

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| Mem0           | MIT      | https://github.com/mem0ai/mem0            | Bellek yönetimi         |
| MemGPT         | Apache-2 | https://github.com/cpacker/memgpt         | Uzun süreli hafıza      |

### 2.4: Search & Research

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| MindSearch     | Apache-2 | https://github.com/InternLM/mindsearch    | Web araştırma ajanı     |

### 2.5: Execution & Sandbox

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| OpenManus      | MIT      | https://github.com/mannaandpoem/openmanus | Docker sandbox          |
| Open Interpreter| AGPL-3  | https://github.com/OpenInterpreter/open-interpreter | Kod yorumlayıcı |

### 2.6: Vector Database

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| Qdrant         | Apache-2 | https://github.com/qdrant/qdrant          | Vektör arama motoru     |
| ChromaDB       | Apache-2 | https://github.com/chroma-core/chroma     | Embedding veritabanı    |

### 2.7: Security & Guardrails

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| NeMo-Guardrails| Apache-2 | https://github.com/NVIDIA/NeMo-Guardrails | Güvenlik filtreleri     |

### 2.8: Development Tools

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| Continue-Dev   | Apache-2 | https://github.com/continuedev/continue   | IDE entegrasyonu        |
| Aider          | Apache-2 | https://github.com/paul-gauthier/aider    | AI kod asistanı         |

### 2.9: ML Frameworks

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| LlamaIndex     | MIT      | https://github.com/run-llama/llama_index  | RAG framework           |
| TensorFlow     | Apache-2 | https://github.com/tensorflow/tensorflow  | ML runtime              |
| Ollama         | MIT      | https://github.com/ollama/ollama          | Local LLM inference     |

### 2.10: CLI & Tools

| Proje          | Lisans   | Kaynak                                    | Amaç                    |
|----------------|----------|-------------------------------------------|-------------------------|
| Fabric         | MIT      | https://github.com/danielmiessler/fabric  | Prompt augmentation     |
| OpenCLI        | MIT      | https://github.com/opencli-project        | CLI framework           |

## ═════════════════════════════════════════════════════════════════════════════
##  BÖLÜM 3: KULLANIM KOŞULLARI
## ═════════════════════════════════════════════════════════════════════════════

1. **MIT Lisansı**: Kaynak kodu serbestçe kullanılabilir, değiştirilebilir ve dağıtılabilir.
   Yalnızca telif hakkı bildirimi ve lisans metni korunmalıdır.

2. **Apache-2.0 Lisansı**: MIT'e benzer, ek olarak patent hakkı koruması sağlar.
   Değiştirilmiş dosyalarda bildirim gerekebilir.

3. **AGPL-3.0 Lisansı**: Ağ üzerinden kullanım durumlarında kaynak kodun paylaşılmasını gerektirir.
   SENTIENT'da AGPL'li bileşenler isteğe bağlı entegrasyon olarak sunulur.

## ═════════════════════════════════════════════════════════════════════════════
##  BÖLÜM 4: TEŞEKKÜRLER
## ═════════════════════════════════════════════════════════════════════════════

SENTIENT, aşağıdaki açık kaynak topluluklarının emeği üzerine inşa edilmiştir:

- Rust Ekibi (tokio, serde ve diğer harika kütüphaneler için)
- Python Yazılım Vakfı
- PyO3 Projesi (Python-Rust köprüsü için)
- Yukarıda listelenen tüm proje yazarları ve katkıda bulunanlar

Katkılarınız olmadan bu proje mümkün olmazdı.

## ═════════════════════════════════════════════════════════════════════════════
##  BÖLÜM 5: İLETİŞİM
## ═════════════════════════════════════════════════════════════════════════════

Lisans ile ilgili sorularınız için:
- GitHub Issues: https://github.com/[your-org]/sentient-core/issues
- E-posta: sentient@nexus-oasis.local

---

*Bu belge otomatik olarak oluşturulmuştur. Lisans metinlerinin orijinali için
ilgili projenin kaynak deposuna bakınız.*
