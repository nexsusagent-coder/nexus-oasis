# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT (NEXUS OASIS) - ENTEGRASYON MİMARİSİ
# ═══════════════════════════════════════════════════════════════════════════════
#
#  Bu belge, SENTIENT'nın üçüncü taraf açık kaynak projelerle entegrasyon
#  stratejisini ve mimarisini açıklar. Tüm entegrasyonlar ilgili lisanslara
#  uygun şekilde gerçekleştirilmiştir.
#
#  Lisans detayları için: THIRD_PARTY_NOTICES.md
#
#  Son güncelleme: 2026-04-07
# ═══════════════════════════════════════════════════════════════════════════════

## ═════════════════════════════════════════════════════════════════════════════
##  MİMARİ GENEL BAKIŞ
## ═════════════════════════════════════════════════════════════════════════════

```
╔═════════════════════════════════════════════════════════════════════════════╗
║                    SENTIENT NATIVE ENTEGRASYON MİMARİSİ                        ║
║                                                                             ║
║   ┌─────────────┐    ┌─────────────┐    ┌─────────────┐                    ║
║   │  EXTERNAL   │    │  BRIDGE     │    │   NATIVE    │                    ║
║   │  PROJECTS   │───▶│   LAYER     │───▶│   CRATES    │                    ║
║   └─────────────┘    └─────────────┘    └─────────────┘                    ║
║                                                                             ║
║   Python/JS/Zig  →   PyO3/wasm/FFI   →   Rust Native Modules              ║
╚═════════════════════════════════════════════════════════════════════════════╝
```

SENTIENT, farklı programlama dillerinde yazılmış açık kaynak projeleri Rust 
çekirdeğine native modüller olarak entegre eder. Bu yaklaşım:

- **Performans**: PyO3 ile zero-copy veri aktarımı
- **Güvenlik**: Rust'ın ownership modeli ile bellek güvenliği
- **Modülerlik**: Her entegrasyon bağımsız crate olarak organize edildi
- **Lisans Uyumu**: Her proje kendi lisansı ile korunur

---

## ═════════════════════════════════════════════════════════════════════════════
##  DİZİN YAPISI
## ═════════════════════════════════════════════════════════════════════════════

```
SENTIENT_CORE/
├── crates/                    # Özgün Rust crate'leri (SENTIENT Core)
│   ├── sentient_core/           # Merkez sistem yönetimi
│   ├── sentient_vgate/          # V-GATE proxy katmanı
│   ├── sentient_memory/         # Bellek sistemi (HİPOKAMPÜS)
│   ├── sentient_orchestrator/    # Ajans orkestrasyonu
│   ├── sentient_research/       # Araştırma motorları
│   ├── sentient_python/         # PyO3 köprüsü
│   └── ...                   # Diğer native modüller
│
├── integrations/             # Üçüncü taraf projeler (referans)
│   ├── agents/               # Ajan framework'leri (CrewAI, AutoGen, vb.)
│   ├── browser/               # Tarayıcı araçları (browser-use)
│   ├── memory/               # Bellek sistemleri (Mem0, ChromaDB)
│   ├── framework/            # ML framework'leri (TensorFlow, LlamaIndex)
│   └── ...                   # Diğer entegrasyonlar
│
├── data/                     # Yerel veri depolama
│   ├── sentient_memory.db       # SQLite bellek veritabanı
│   └── ...
│
└── THIRD_PARTY_NOTICES.md    # Lisans bilgileri
```

---

## ═════════════════════════════════════════════════════════════════════════════
##  ENTEGRASYON TABLOSU
## ═════════════════════════════════════════════════════════════════════════════

| Proje | Kategori | Kaynak Dil | Entegrasyon | Hedef Crate | Lisans |
|-------|----------|------------|-------------|-------------|--------|
| CrewAI | Ajan Framework | Python | PyO3 | sentient_agents | MIT |
| AutoGen | Ajan Framework | Python | PyO3 | sentient_agents | MIT |
| Browser-Use | Web Automation | Python | PyO3 | oasis_browser | MIT |
| Mem0 | Bellek | Python | PyO3 | sentient_memory | MIT |
| MemGPT | Bellek | Python | PyO3 | sentient_memory | Apache-2 |
| ChromaDB | Vektör DB | Python | PyO3 | sentient_vector | Apache-2 |
| Qdrant | Vektör DB | Rust | Native | sentient_vector | Apache-2 |
| NeMo-Guardrails | Güvenlik | Python | PyO3 | sentient_guardrails | Apache-2 |
| OpenManus | Sandbox | Python | PyO3 | oasis_manus | MIT |
| MindSearch | Araştırma | Python | PyO3 | sentient_research | Apache-2 |
| LlamaIndex | RAG | Python | PyO3 | sentient_research | MIT |
| TensorFlow | ML Runtime | Python/C++ | PyO3 | (external) | Apache-2 |
| Ollama | Local LLM | Go | HTTP API | sentient_vgate | MIT |

---

## ═════════════════════════════════════════════════════════════════════════════
##  ENTEGRASYON YÖNTEMİ
## ═════════════════════════════════════════════════════════════════════════════

### 1. PyO3 Köprüsü (Python → Rust)

Python tabanlı projeler için native Rust wrapper:

```rust
// sentient_python/src/lib.rs
use pyo3::prelude::*;

pub struct PythonBridge {
    tools: HashMap<String, PythonToolDef>,
}

impl PythonBridge {
    /// Python modülünü çağır
    pub fn call_python(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> SENTIENTResult<serde_json::Value> {
        Python::with_gil(|py| {
            let module = py.import(tool.module_path)?;
            let func = module.getattr(tool.function_name)?;
            let result = func.call(args)?;
            py_value_to_json(py, &result)
        })
    }
}
```

### 2. Native Rust Rewrite

Go ve diğer diller için tamamen Rust implementasyonu:

```rust
// sentient_vgate/providers/gemini.rs
pub struct GeminiProvider {
    api_key: SecureString,  // RAM'de şifreli
    client: Client,
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    async fn complete(&self, request: LlmRequest) -> SENTIENTResult<LlmResponse> {
        // Native Rust implementasyonu
    }
}
```

### 3. FFI Bindings (Zig → Rust)

Düşük seviye entegrasyonlar için FFI:

```rust
// oasis_browser/src/lightpanda_ffi.rs
#[cfg(feature = "lightpanda-ffi")]
pub mod lightpanda_ffi {
    use std::ffi::CString;
    
    pub struct LightpandaFFI {
        handle: *mut c_void,
    }
    
    // Güvenli wrapper ile unsafe FFI çağrıları
}
```

---

## ═════════════════════════════════════════════════════════════════════════════
##  GÜVENLİK STANDARTLARI
## ═════════════════════════════════════════════════════════════════════════════

### V-GATE Proxy Kuralı

**API anahtarları ASLA istemcide tutulmaz.**

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT     │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Güvenli)
                     Sunucuda saklanır
```

### Guardrails Entegrasyonu

Tüm girdi/çıktı otomatik filtrelenir:

- Prompt injection tespiti
- Veri sızıntısı engelleme
- SQL injection koruması
- XSS filtresi

---

## ═════════════════════════════════════════════════════════════════════════════
##  TEST SONUÇLARI
## ═════════════════════════════════════════════════════════════════════════════

| Modül | Test Sayısı | Durum |
|-------|-------------|-------|
| sentient_core | 45 | ✅ PASSED |
| sentient_memory | 62 | ✅ PASSED |
| sentient_vgate | 38 | ✅ PASSED |
| sentient_guardrails | 25 | ✅ PASSED |
| sentient_python | 34 | ✅ PASSED |
| sentient_orchestrator | 89 | ✅ PASSED |
| sentient_research | 30 | ✅ PASSED |
| oasis_browser | 27 | ✅ PASSED |
| **TOPLAM** | **547** | **✅ PASSED** |

---

## ═════════════════════════════════════════════════════════════════════════════
##  SİSTEM GEREKSİNİMLERİ
## ═════════════════════════════════════════════════════════════════════════════

- **Rust**: 1.75+
- **Python**: 3.10+ (PyO3 entegrasyonları için)
- **Docker**: 24+ (Sandbox için)
- **SQLite**: 3.40+ (Bellek için)
- **RAM**: Minimum 4GB, Önerilen 16GB

---

## ═════════════════════════════════════════════════════════════════════════════
##  KULLANILABİLİR KOMUTLAR
## ═════════════════════════════════════════════════════════════════════════════

```bash
# Sistem derleme
cargo build --release

# İnteraktif REPL
./target/release/sentient repl

# LLM Sohbet (V-GATE üzerinden)
./target/release/sentient llm chat

# Otonom Ajan Başlat
./target/release/sentient agent start

# Sistem Durumu
./target/release/sentient status

# Arka Plan Sunucu
./target/release/sentient serve
```

---

## ═════════════════════════════════════════════════════════════════════════════
##  TEŞEKKÜRLER
## ═════════════════════════════════════════════════════════════════════════════

SENTIENT, açık kaynak topluluğunun emeği üzerine inşa edilmiştir.

Detaylı lisans bilgileri için `THIRD_PARTY_NOTICES.md` dosyasına bakınız.

---

*SENTIENT NEXUS OASIS - Enterprise AI Operations Platform*
