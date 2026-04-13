# ═══════════════════════════════════════════════════════════════════════════════
#  OASIS_HANDS - DETAYLI ANALİZ RAPORU (45,316 SATIR)
# ═══════════════════════════════════════════════════════════════════════════════
#
# Tarih: 12 Nisan 2026
# Kapsam: Tool Library, Human Mimicry, Setup Wizard, Security Layer
# İstatistik: 301 dosya, 629 struct, 937 impl, 1992 public fn
#
# ═──────────────────────────────────────────────────────────────────────────────

## 📊 GENEL BAKIŞ

| Kategori | Sayı | Açıklama |
|----------|------|----------|
| **Toplam Satır** | 45,316 | En büyük crate |
| **Toplam Dosya** | 301 | Rust kaynak dosyası |
| **Struct** | 629 | Veri yapısı |
| **Impl** | 937 | Trait implementasyonu |
| **Public Fn** | 1992 | Açık fonksiyon |
| **Wrappers** | 239 | Tool implementasyonu |

---

## 🏗️ MODÜL YAPISI

### Ana Modüller (20 adet)

| Modül | Dosya | Satır | Açıklama |
|-------|-------|-------|----------|
| `lib.rs` | 1 | ~500 | Ana modül + re-exports |
| `agent.rs` | 1 | ~800 | Ajan yönetimi |
| `alert.rs` | 1 | ~500 | Uyarı sistemi |
| `emergency.rs` | 1 | ~1200 | Acil durdurma |
| `error.rs` | 1 | ~300 | Hata tanımları |
| `executor.rs` | 1 | ~900 | Komut yürütme |
| `history.rs` | 1 | ~1500 | Undo/Redo sistemi |
| `input.rs` | 1 | ~600 | Giriş yönetimi |
| `rate_limiter.rs` | 1 | ~1000 | Hız sınırlama |
| `recorder.rs` | 1 | ~1600 | Aksiyon kaydı |
| `sandbox.rs` | 1 | ~1100 | Sandbox modu |
| `screen.rs` | 1 | ~400 | Ekran işlemleri |
| `session.rs` | 1 | ~500 | Oturum yönetimi |
| `skill_loader.rs` | 1 | ~300 | Skill yükleme |
| `sovereign.rs` | 1 | ~2000 | Yasal sınırlar |
| `time_rules.rs` | 1 | ~1000 | Zaman kuralları |
| `tools.rs` | 1 | ~800 | Araç kayıt defteri |
| `vgate.rs` | 1 | ~800 | V-GATE entegrasyonu |
| `vision.rs` | 1 | ~400 | Görüntü işleme |
| `sentient_tool.rs` | 1 | ~600 | Tool trait |

### Alt Modüller (4 adet)

| Modül | Dosya | Satır | Açıklama |
|-------|-------|-------|----------|
| `human_mimicry/` | 6 | ~3000 | İnsan taklidi |
| `setup/` | 7 | ~5000 | Kurulum sihirbazı |
| `wrappers/` | 239 | ~25000 | Tool implementasyonları |
| `sentient_tools/` | 10 | ~3000 | SENTIENT araçları |

---

## 🧠 HUMAN_MIMICRY - İNSAN TAKLİDİ

### Dosya Yapısı
```
human_mimicry/
├── mod.rs           (150+ satır) - Ana modül
├── behavior_model.rs (800+ satır) - Davranış modeli
├── bezier.rs        (400+ satır) - Bezier eğrileri
├── bumblebee.rs     (500+ satır) - Rastgele hareket
├── mouse_dynamics.rs (600+ satır) - Fare dinamikleri
└── typing_dynamics.rs (500+ satır) - Yazma dinamikleri
```

### HumanConfig (25+ Parametre)

```rust
pub struct HumanConfig {
    // Fare Ayarları
    pub mouse_speed: f32,              // 200-800 px/s
    pub mouse_pattern: MousePattern,   // Linear, Curved, Wavy, Natural, Adaptive
    pub mouse_acceleration: f32,       // 0.5-2.0
    pub mouse_accuracy: f32,           // 0.8-1.0
    
    // Klavye Ayarları
    pub typing_wpm: u32,               // 20-120 WPM
    pub typing_profile: TypingProfile, // Beginner, Intermediate, Expert, Custom
    pub typing_variance: f32,          // 0.0-0.5
    pub error_rate: f32,               // 0.0-0.05
    
    // Davranış Ayarları
    pub attention_span_secs: u32,      // 60-3600
    pub distraction_rate: f32,         // 0.0-0.3
    pub fatigue_rate: f32,             // %/saat
    pub decision_delay_ms: (u32, u32), // Min-max gecikme
    pub hesitation_rate: f32,          // 0.0-0.2
    
    // Fiziksel Ayarlar
    pub hand_preference: HandPreference, // Left, Right, Ambidextrous
    pub scroll_behavior: ScrollBehavior, // Smooth, Step, Natural
    
    // Gelişmiş Ayarlar
    pub micro_pause_rate: f32,         // Mikro duraklama
    pub burst_typing: bool,            // Patlayıcı yazma
    pub natural_movement: bool,        // Doğal hareket
}
```

### MousePattern

```rust
pub enum MousePattern {
    Linear,      // Düz çizgi
    Curved,      // Eğri çizgi (Bezier)
    Wavy,        // Dalgalı hareket
    Natural,     // İnsan benzeri (rastgele sapmalar)
    Adaptive,    // Göreve göre adapte
}
```

### TypingProfile

```rust
pub enum TypingProfile {
    Beginner,     // 20-40 WPM, yüksek hata
    Intermediate, // 40-70 WPM, orta hata
    Expert,       // 70-100 WPM, düşük hata
    Custom {
        wpm: u32,
        error_rate: f32,
        burst_factor: f32,
    },
}
```

### BehaviorModel

```rust
pub struct BehaviorModel {
    config: HumanConfig,
    fatigue_level: f32,
    distraction_state: DistractionState,
    current_session_time: Duration,
}

pub enum DistractionState {
    Focused,
    MildlyDistracted,
    Distracted,
    HighlyDistracted,
}

impl BehaviorModel {
    pub fn generate_mouse_path(&self, from: Point, to: Point) -> Vec<Point>;
    pub fn generate_typing_delay(&self) -> Duration;
    pub fn should_make_error(&self) -> bool;
    pub fn update_fatigue(&mut self, elapsed: Duration);
    pub fn get_current_wpm(&self) -> u32;
}
```

---

## 🔧 SETUP WIZARD - KURULUM SİHRİBAZI

### Dosya Yapısı
```
setup/
├── mod.rs        (300+ satır) - Ana modül
├── wizard.rs     (2000+ satır) - TUI wizard
├── config.rs     (800+ satır) - Yapılandırma
├── permissions.rs (600+ satır) - İzin sistemi
├── profiles.rs   (400+ satır) - Profil yönetimi
├── approval.rs   (500+ satır) - Onay sistemi
└── tests.rs      (400+ satır) - Testler
```

### SetupWizard

```rust
pub struct SetupWizard {
    mode: WizardMode,
    config: SetupConfig,
    permissions: PermissionManager,
    profiles: ProfileManager,
    approval: ApprovalManager,
}

pub enum WizardMode {
    Auto,        // Otomatik kurulum
    Interactive, // İnteraktif TUI
    Silent,      // Sessiz kurulum
    Later,       // Sonra yap
    TestOnly,    // Sadece test
    Repair,      // Onarım modu
}
```

### PermissionManager

```rust
pub struct PermissionManager {
    granted: HashSet<Permission>,
    denied: HashSet<Permission>,
    pending: Vec<PermissionRequest>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Permission {
    // Dosya Sistemi
    FileRead,
    FileWrite,
    FileDelete,
    DirectoryCreate,
    
    // Ağ
    NetworkHttp,
    NetworkWebSocket,
    NetworkDns,
    
    // Sistem
    ProcessExecute,
    ProcessKill,
    SystemInfo,
    
    // Donanım
    MouseControl,
    KeyboardControl,
    ScreenCapture,
    AudioCapture,
    
    // Özel
    LlmAccess,
    ToolExecution,
    SandboxExecution,
}
```

### ApprovalManager

```rust
pub struct ApprovalManager {
    pending_approvals: Vec<PendingApproval>,
    approved: Vec<ApprovedAction>,
    denied: Vec<DeniedAction>,
}

pub struct PendingApproval {
    pub action_id: Uuid,
    pub action_type: ActionType,
    pub description: String,
    pub risk_level: RiskLevel,
    pub requested_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

pub enum RiskLevel {
    Low,      // Otomatik onaylanabilir
    Medium,   // Kullanıcı onayı gerekli
    High,     // Detaylı onay gerekli
    Critical, // Çoklu onay gerekli
}
```

---

## 🔒 SECURITY LAYER - GÜVENLİK KATMANI

### Emergency Stop

```rust
pub struct EmergencyStop {
    hotkey: Hotkey,
    callback: Box<dyn Fn() + Send + Sync>,
    triggered: AtomicBool,
    trigger_count: AtomicU64,
}

impl EmergencyStop {
    pub fn new(hotkey: Hotkey) -> Self;
    pub fn register_callback(&mut self, callback: Box<dyn Fn() + Send + Sync>);
    pub fn trigger(&self);
    pub fn reset(&self);
    pub fn is_triggered(&self) -> bool;
}

// Varsayılan: Ctrl+Shift+Escape
pub const DEFAULT_EMERGENCY_HOTKEY: Hotkey = Hotkey {
    modifiers: vec![Modifier::Ctrl, Modifier::Shift],
    key: Key::Escape,
};
```

### Rate Limiter

```rust
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
    violations: Vec<RateLimitViolation>,
}

pub struct RateLimit {
    pub resource: String,
    pub max_requests: u32,
    pub window_secs: u64,
    pub current_count: u32,
    pub reset_at: DateTime<Utc>,
}

pub struct RateLimitViolation {
    pub resource: String,
    pub attempted_at: DateTime<Utc>,
    pub wait_until: DateTime<Utc>,
    pub severity: ViolationSeverity,
}

impl RateLimiter {
    pub fn check(&self, resource: &str) -> RateLimitResult;
    pub fn record(&mut self, resource: &str);
    pub fn get_wait_time(&self, resource: &str) -> Option<Duration>;
}
```

### Sovereign (Forbidden Regions)

```rust
pub struct Sovereign {
    forbidden_regions: Vec<ForbiddenRegion>,
    rules: Vec<SovereignRule>,
}

pub struct ForbiddenRegion {
    pub id: String,
    pub region_type: RegionType,
    pub paths: Vec<PathBuf>,
    pub urls: Vec<Url>,
    pub actions: Vec<String>,
}

pub enum RegionType {
    FileSystem,  // Dosya sistemi bölgesi
    Network,     // Ağ bölgesi
    Process,     // İşlem bölgesi
    Data,        // Veri bölgesi
    Custom,      // Özel bölge
}

impl Sovereign {
    pub fn check_path(&self, path: &Path) -> SovereignResult;
    pub fn check_url(&self, url: &Url) -> SovereignResult;
    pub fn check_action(&self, action: &str) -> SovereignResult;
}
```

### Time Rules

```rust
pub struct TimeRules {
    rules: Vec<TimeRule>,
    current_schedule: Option<Schedule>,
}

pub struct TimeRule {
    pub id: String,
    pub allowed_actions: Vec<String>,
    pub schedule: Schedule,
    pub timezone: String,
}

pub struct Schedule {
    pub days: Vec<DayOfWeek>,
    pub start_time: Time,
    pub end_time: Time,
}

impl TimeRules {
    pub fn is_allowed(&self, action: &str) -> bool;
    pub fn get_next_allowed_time(&self, action: &str) -> Option<DateTime<Utc>>;
}
```

---

## 📦 WRAPPERS - TOOL IMPLEMENTATIONS

### Kategoriler (239 Tool)

| Kategori | Sayı | Örnekler |
|----------|------|----------|
| **OpenClaw** | 5 | browser_navigate, code_execute, file_read, web_search |
| **OpenHarness** | 200+ | app, bash_tool, ask_user, bridge, builtin, ... |
| **Sentient Tools** | 30+ | memory, search, vision, etc. |

### OpenClaw Tools

```rust
// openclaw_browser_navigate.rs
pub struct BrowserNavigate {
    url: String,
    wait_for: Option<String>,
    screenshot: bool,
}

// openclaw_code_execute.rs
pub struct CodeExecute {
    language: Language,
    code: String,
    timeout: Duration,
}

// openclaw_file_read.rs
pub struct FileRead {
    path: PathBuf,
    encoding: Encoding,
}

// openclaw_web_search.rs
pub struct WebSearch {
    query: String,
    provider: SearchProvider,
    max_results: usize,
}
```

### OpenHarness Tools (Örnekler)

| Tool | Açıklama |
|------|----------|
| `bash_tool` | Shell komutları |
| `ask_user_question` | Kullanıcıya soru |
| `brief_tool` | Özetleme |
| `list_mcp_resources` | MCP kaynakları |
| `send_message` | Mesaj gönderme |
| `cron_list` | Zamanlanmış görevler |
| `select_modal` | Seçim modalı |
| `app` | Uygulama yönetimi |
| `adapter` | Adaptör yönetimi |
| `agent_tool` | Ajan aracı |
| `bridge` | Köprü yönetimi |

---

## 📊 HISTORY - UNDO/REO SİSTEMİ

### Action History

```rust
pub struct ActionHistory {
    actions: Vec<RecordedAction>,
    current_index: usize,
    max_actions: usize,
}

pub struct RecordedAction {
    pub id: Uuid,
    pub action_type: ActionType,
    pub timestamp: DateTime<Utc>,
    pub input: Value,
    pub output: Value,
    pub undo_data: Option<UndoData>,
    pub can_undo: bool,
}

pub struct UndoData {
    pub undo_action: ActionType,
    pub undo_params: Value,
}

impl ActionHistory {
    pub fn record(&mut self, action: RecordedAction);
    pub fn undo(&mut self) -> Option<RecordedAction>;
    pub fn redo(&mut self) -> Option<RecordedAction>;
    pub fn get_history(&self) -> &[RecordedAction];
    pub fn clear(&mut self);
}
```

---

## 🔴 EKSİKLİKLER VE İYİLEŞTİRMELER

| # | Eksiklik | Öncelik | Açıklama |
|---|----------|---------|----------|
| 1 | ❌ **Platform Implementation** | 🔴 Kritik | enigo/rdev native |
| 2 | ⚠️ **Touch Input** | 🟡 Orta | Dokunmatik giriş desteği |
| 3 | ⚠️ **Multi-Monitor** | 🟡 Orta | Çoklu monitör desteği |
| 4 | ❌ **Accessibility** | 🟢 Düşük | Erişilebilirlik API |
| 5 | ⚠️ **Macro Recording** | 🟡 Orta | Makro kayıt/oynatma |

---

## 📈 TAMAMLANMA DURUMU

| Kategori | Tamamlanma | Not |
|----------|------------|-----|
| Human Mimicry | 95% | 25+ parametre |
| Setup Wizard | 98% | 6 mod |
| Security Layer | 90% | 8 katman |
| Tool Wrappers | 85% | 239 araç |
| History System | 90% | Undo/Redo |
| Rate Limiting | 95% | Token bucket |
| Sovereign | 90% | Yasal sınırlar |
| Time Rules | 85% | Zaman kuralları |

**Genel: %91 Tamamlanma**

---

*Analiz Tarihi: 12 Nisan 2026*
*Bu crate SENTIENT'ın en büyük ve en kapsamlı modülüdür*
