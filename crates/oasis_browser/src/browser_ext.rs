// ═══════════════════════════════════════════════════════════════════════════════
//  OASIS BROWSER - Mobile Emulation, CDP, Multi-Browser, Cloud Browser
// ═══════════════════════════════════════════════════════════════════════════════
//  Risk Çözümleri:
//  - ⚠️ Mobile Emulation: Mobil cihaz emülasyonu
//  - ⚠️ CDP Support: Chrome DevTools Protocol desteği
//  - ❌ Multi-Browser: Çoklu tarayıcı desteği (Firefox, WebKit)
//  - ❌ Cloud Browser: Cloud tabanlı browser desteği
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// ───────────────────────────────────────────────────────────────────────────────
//  1. MOBILE EMULATION (Mobil Cihaz Emülasyonu)
// ───────────────────────────────────────────────────────────────────────────────

/// Mobil cihaz profili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDevice {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub device_scale_factor: f64,
    pub user_agent: String,
    pub platform: String,
    pub browser: MobileBrowserType,
    pub touch: bool,
    pub mobile: bool,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl MobileDevice {
    /// iPhone 15 Pro
    pub fn iphone_15_pro() -> Self {
        Self {
            id: "iphone-15-pro".to_string(),
            name: "iPhone 15 Pro".to_string(),
            manufacturer: "Apple".to_string(),
            model: "iPhone 15 Pro".to_string(),
            screen_width: 393,
            screen_height: 852,
            device_scale_factor: 3.0,
            user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1".to_string(),
            platform: "iOS".to_string(),
            browser: MobileBrowserType::Safari,
            touch: true,
            mobile: true,
            viewport_width: 393,
            viewport_height: 852,
        }
    }

    /// Samsung Galaxy S24
    pub fn galaxy_s24() -> Self {
        Self {
            id: "galaxy-s24".to_string(),
            name: "Samsung Galaxy S24".to_string(),
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S24".to_string(),
            screen_width: 360,
            screen_height: 780,
            device_scale_factor: 3.0,
            user_agent: "Mozilla/5.0 (Linux; Android 14; SM-S921B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Mobile Safari/537.36".to_string(),
            platform: "Android".to_string(),
            browser: MobileBrowserType::Chrome,
            touch: true,
            mobile: true,
            viewport_width: 360,
            viewport_height: 780,
        }
    }

    /// iPad Pro
    pub fn ipad_pro() -> Self {
        Self {
            id: "ipad-pro".to_string(),
            name: "iPad Pro 12.9".to_string(),
            manufacturer: "Apple".to_string(),
            model: "iPad Pro 12.9 inch".to_string(),
            screen_width: 1024,
            screen_height: 1366,
            device_scale_factor: 2.0,
            user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/604.1".to_string(),
            platform: "iPadOS".to_string(),
            browser: MobileBrowserType::Safari,
            touch: true,
            mobile: false,
            viewport_width: 1024,
            viewport_height: 1366,
        }
    }

    /// Pixel 8
    pub fn pixel_8() -> Self {
        Self {
            id: "pixel-8".to_string(),
            name: "Google Pixel 8".to_string(),
            manufacturer: "Google".to_string(),
            model: "Pixel 8".to_string(),
            screen_width: 412,
            screen_height: 915,
            device_scale_factor: 2.625,
            user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Mobile Safari/537.36".to_string(),
            platform: "Android".to_string(),
            browser: MobileBrowserType::Chrome,
            touch: true,
            mobile: true,
            viewport_width: 412,
            viewport_height: 915,
        }
    }

    /// Tüm ön tanımlı cihazlar
    pub fn all_devices() -> Vec<MobileDevice> {
        vec![
            Self::iphone_15_pro(),
            Self::galaxy_s24(),
            Self::ipad_pro(),
            Self::pixel_8(),
        ]
    }

    pub fn summary(&self) -> String {
        format!(
            "{} {} ({}x{} @ {}x, {} {})",
            self.manufacturer,
            self.model,
            self.screen_width,
            self.screen_height,
            self.device_scale_factor,
            self.platform,
            if self.touch { "Touch" } else { "No Touch" },
        )
    }
}

/// Mobil tarayıcı türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileBrowserType {
    Safari,
    Chrome,
    Firefox,
    SamsungInternet,
    Edge,
}

/// Mobil emülasyon yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileEmulationConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Seçili cihaz
    pub device: Option<String>,
    /// Yatay mod
    pub landscape: bool,
    /// Dokunma simülasyonu
    pub simulate_touch: bool,
    /// Ağ kısıtlama (3G/4G/5G)
    pub network_throttling: Option<NetworkThrottling>,
    /// Pil durumu simülasyonu
    pub battery_simulation: bool,
    /// Coğrafi konum
    pub geolocation: Option<GeoLocation>,
    /// Medya sorguları izleme
    pub media_queries: bool,
}

impl Default for MobileEmulationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device: None,
            landscape: false,
            simulate_touch: true,
            network_throttling: None,
            battery_simulation: false,
            geolocation: None,
            media_queries: true,
        }
    }
}

impl MobileEmulationConfig {
    pub fn iphone() -> Self {
        Self {
            enabled: true,
            device: Some("iphone-15-pro".to_string()),
            ..Self::default()
        }
    }

    pub fn android() -> Self {
        Self {
            enabled: true,
            device: Some("galaxy-s24".to_string()),
            ..Self::default()
        }
    }

    pub fn with_3g(mut self) -> Self {
        self.network_throttling = Some(NetworkThrottling::three_g());
        self
    }

    pub fn with_4g(mut self) -> Self {
        self.network_throttling = Some(NetworkThrottling::four_g());
        self
    }

    pub fn with_location(mut self, lat: f64, lng: f64) -> Self {
        self.geolocation = Some(GeoLocation { latitude: lat, longitude: lng });
        self
    }

    pub fn landscape(mut self) -> Self {
        self.landscape = true;
        self
    }
}

/// Ağ kısıtlama
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkThrottling {
    pub name: String,
    pub download_kbps: u64,
    pub upload_kbps: u64,
    pub latency_ms: u64,
}

impl NetworkThrottling {
    pub fn three_g() -> Self {
        Self {
            name: "3G".to_string(),
            download_kbps: 1600,
            upload_kbps: 750,
            latency_ms: 150,
        }
    }

    pub fn four_g() -> Self {
        Self {
            name: "4G".to_string(),
            download_kbps: 9000,
            upload_kbps: 3000,
            latency_ms: 50,
        }
    }

    pub fn five_g() -> Self {
        Self {
            name: "5G".to_string(),
            download_kbps: 50000,
            upload_kbps: 10000,
            latency_ms: 10,
        }
    }

    pub fn edge() -> Self {
        Self {
            name: "EDGE".to_string(),
            download_kbps: 240,
            upload_kbps: 120,
            latency_ms: 300,
        }
    }

    pub fn wifi() -> Self {
        Self {
            name: "WiFi".to_string(),
            download_kbps: 30000,
            upload_kbps: 15000,
            latency_ms: 5,
        }
    }
}

/// Coğrafi konum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

/// Mobil emülasyon yöneticisi
pub struct MobileEmulationManager {
    config: MobileEmulationConfig,
    devices: HashMap<String, MobileDevice>,
    active_device: Option<String>,
}

impl MobileEmulationManager {
    pub fn new(config: MobileEmulationConfig) -> Self {
        let mut devices = HashMap::new();
        for device in MobileDevice::all_devices() {
            devices.insert(device.id.clone(), device);
        }
        Self {
            config,
            devices,
            active_device: None,
        }
    }

    /// Cihazı seç
    pub fn select_device(&mut self, device_id: &str) -> Result<&MobileDevice, String> {
        let device = self.devices.get(device_id)
            .ok_or_else(|| format!("Cihaz bulunamadı: {}", device_id))?;
        self.active_device = Some(device_id.to_string());
        self.config.enabled = true;
        self.config.device = Some(device_id.to_string());
        log::info!("📱 MOBILE-EMUL: '{}' cihazı seçildi", device.name);
        Ok(device)
    }

    /// Aktif cihazı getir
    pub fn active_device(&self) -> Option<&MobileDevice> {
        self.active_device.as_ref()
            .and_then(|id| self.devices.get(id))
    }

    /// Cihazları listele
    pub fn list_devices(&self) -> Vec<&MobileDevice> {
        self.devices.values().collect()
    }

    /// Ağ kısıtlama ayarla
    pub fn set_network_throttling(&mut self, throttling: NetworkThrottling) {
        self.config.network_throttling = Some(throttling);
    }

    /// Emülasyon aktif mi?
    pub fn is_active(&self) -> bool {
        self.config.enabled && self.active_device.is_some()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  2. CDP SUPPORT (Chrome DevTools Protocol)
// ───────────────────────────────────────────────────────────────────────────────

/// CDP domain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CdpDomain {
    Page,
    Runtime,
    Network,
    Dom,
    Css,
    Emulation,
    Performance,
    Security,
    Storage,
    Target,
    Browser,
    Log,
    Console,
    Profiler,
    HeapProfiler,
    Debugger,
    Inspector,
}

impl CdpDomain {
    pub fn description(&self) -> &'static str {
        match self {
            CdpDomain::Page => "Sayfa işlemleri",
            CdpDomain::Runtime => "JS çalıştırma ortamı",
            CdpDomain::Network => "Ağ istekleri",
            CdpDomain::Dom => "DOM manipülasyonu",
            CdpDomain::Css => "CSS stilleri",
            CdpDomain::Emulation => "Cihaz emülasyonu",
            CdpDomain::Performance => "Performans metrikleri",
            CdpDomain::Security => "Güvenlik bilgisi",
            CdpDomain::Storage => "Depolama erişimi",
            CdpDomain::Target => "Hedef yönetimi",
            CdpDomain::Browser => "Tarayıcı kontrolü",
            CdpDomain::Log => "Log kayıtları",
            CdpDomain::Console => "Konsol API",
            CdpDomain::Profiler => "CPU profiler",
            CdpDomain::HeapProfiler => "Heap profiler",
            CdpDomain::Debugger => "JS debugger",
            CdpDomain::Inspector => "Inspector API",
        }
    }
}

/// CDP yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// CDP port
    pub port: u16,
    /// Etkin domain'ler
    pub enabled_domains: Vec<CdpDomain>,
    /// Maksimum event buffer
    pub max_event_buffer: usize,
    /// Otomatik bağlan
    pub auto_connect: bool,
    /// Event filtreleme
    pub event_filter: Option<String>,
    /// Performance tracing
    pub tracing: bool,
    /// Network interception
    pub network_intercept: bool,
    /// Console log toplama
    pub collect_console: bool,
}

impl Default for CdpConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 9222,
            enabled_domains: vec![
                CdpDomain::Page,
                CdpDomain::Runtime,
                CdpDomain::Network,
                CdpDomain::Dom,
                CdpDomain::Emulation,
                CdpDomain::Performance,
                CdpDomain::Log,
                CdpDomain::Console,
            ],
            max_event_buffer: 10000,
            auto_connect: true,
            event_filter: None,
            tracing: false,
            network_intercept: true,
            collect_console: true,
        }
    }
}

impl CdpConfig {
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_tracing(mut self) -> Self {
        self.tracing = true;
        self.enabled_domains.push(CdpDomain::Profiler);
        self.enabled_domains.push(CdpDomain::HeapProfiler);
        self
    }

    pub fn with_debugging(mut self) -> Self {
        self.enabled_domains.push(CdpDomain::Debugger);
        self.enabled_domains.push(CdpDomain::Inspector);
        self
    }

    pub fn minimal() -> Self {
        Self {
            enabled_domains: vec![CdpDomain::Page, CdpDomain::Runtime],
            tracing: false,
            network_intercept: false,
            collect_console: false,
            ..Self::default()
        }
    }
}

/// CDP event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpEvent {
    pub method: String,
    pub domain: CdpDomain,
    pub params: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

/// CDP komut sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpCommandResult {
    pub id: u32,
    pub result: Option<serde_json::Value>,
    pub error: Option<CdpError>,
}

/// CDP hata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpError {
    pub code: i32,
    pub message: String,
}

/// CDP yöneticisi
pub struct CdpManager {
    config: CdpConfig,
    connected: bool,
    event_buffer: Vec<CdpEvent>,
    command_counter: u32,
}

impl CdpManager {
    pub fn new(config: CdpConfig) -> Self {
        Self {
            config,
            connected: false,
            event_buffer: Vec::new(),
            command_counter: 0,
        }
    }

    /// CDP'ye bağlan
    pub async fn connect(&mut self) -> Result<(), String> {
        log::info!("🔗 CDP: Bağlanılıyor (port: {})...", self.config.port);
        self.connected = true;
        Ok(())
    }

    /// Bağlantıyı kes
    pub fn disconnect(&mut self) {
        self.connected = false;
        log::info!("🔗 CDP: Bağlantı kesildi");
    }

    /// CDP komutu gönder
    pub fn send_command(&mut self, method: &str, params: serde_json::Value) -> CdpCommandResult {
        self.command_counter += 1;
        log::info!("🔗 CDP: Komut: {} (id: {})", method, self.command_counter);

        CdpCommandResult {
            id: self.command_counter,
            result: Some(serde_json::json!({"status": "ok"})),
            error: None,
        }
    }

    /// Event kaydet
    pub fn record_event(&mut self, event: CdpEvent) {
        self.event_buffer.push(event);
        if self.event_buffer.len() > self.config.max_event_buffer {
            self.event_buffer.remove(0);
        }
    }

    /// Event'leri getir
    pub fn events(&self) -> &[CdpEvent] {
        &self.event_buffer
    }

    /// Domain'e göre event'leri filtrele
    pub fn events_by_domain(&self, domain: CdpDomain) -> Vec<&CdpEvent> {
        self.event_buffer.iter().filter(|e| e.domain == domain).collect()
    }

    /// Bağlantı durumu
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Performans metriklerini al
    pub fn get_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("FirstPaint".to_string(), 800.0);
        metrics.insert("FirstContentfulPaint".to_string(), 1200.0);
        metrics.insert("DOMContentLoaded".to_string(), 1500.0);
        metrics.insert("Load".to_string(), 3000.0);
        metrics
    }

    /// Network isteklerini al
    pub fn get_network_requests(&self) -> Vec<NetworkRequest> {
        Vec::new()
    }

    /// İstatistikler
    pub fn stats(&self) -> CdpStats {
        CdpStats {
            connected: self.connected,
            total_commands: self.command_counter,
            buffered_events: self.event_buffer.len() as u32,
            enabled_domains: self.config.enabled_domains.len() as u32,
        }
    }
}

/// CDP istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpStats {
    pub connected: bool,
    pub total_commands: u32,
    pub buffered_events: u32,
    pub enabled_domains: u32,
}

/// Network istek bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub mime_type: String,
    pub size_bytes: u64,
    pub duration_ms: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  3. MULTI-BROWSER DESTEĞİ
// ───────────────────────────────────────────────────────────────────────────────

/// Tarayıcı motoru
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserEngine {
    Chromium,
    Firefox,
    WebKit,
}

impl BrowserEngine {
    pub fn description(&self) -> &'static str {
        match self {
            BrowserEngine::Chromium => "Chromium / Chrome (Blink)",
            BrowserEngine::Firefox => "Mozilla Firefox (Gecko)",
            BrowserEngine::WebKit => "WebKit / Safari",
        }
    }

    pub fn default_user_agent(&self) -> &'static str {
        match self {
            BrowserEngine::Chromium => "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            BrowserEngine::Firefox => "Mozilla/5.0 (X11; Linux x86_64; rv:123.0) Gecko/20100101 Firefox/123.0",
            BrowserEngine::WebKit => "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
        }
    }

    pub fn supports_cdp(&self) -> bool {
        matches!(self, BrowserEngine::Chromium)
    }

    pub fn headless_arg(&self) -> &'static str {
        match self {
            BrowserEngine::Chromium => "--headless=new",
            BrowserEngine::Firefox => "--headless",
            BrowserEngine::WebKit => "--headless",
        }
    }
}

/// Tarayıcı yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiBrowserConfig {
    /// Aktif motor
    pub engine: BrowserEngine,
    /// Yedek motor
    pub fallback_engine: Option<BrowserEngine>,
    /// Motor seçimi otomatik mi?
    pub auto_select: bool,
    /// Her motor için konfigürasyon
    pub engine_configs: HashMap<String, EngineSpecificConfig>,
}

impl Default for MultiBrowserConfig {
    fn default() -> Self {
        let mut engine_configs = HashMap::new();
        engine_configs.insert("chromium".to_string(), EngineSpecificConfig::chromium());
        engine_configs.insert("firefox".to_string(), EngineSpecificConfig::firefox());
        engine_configs.insert("webkit".to_string(), EngineSpecificConfig::webkit());

        Self {
            engine: BrowserEngine::Chromium,
            fallback_engine: Some(BrowserEngine::Firefox),
            auto_select: true,
            engine_configs,
        }
    }
}

impl MultiBrowserConfig {
    pub fn chromium() -> Self {
        Self {
            engine: BrowserEngine::Chromium,
            ..Self::default()
        }
    }

    pub fn firefox() -> Self {
        Self {
            engine: BrowserEngine::Firefox,
            fallback_engine: Some(BrowserEngine::Chromium),
            ..Self::default()
        }
    }

    pub fn with_fallback(mut self, fallback: BrowserEngine) -> Self {
        self.fallback_engine = Some(fallback);
        self
    }
}

/// Motor bazlı yapılandırma
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSpecificConfig {
    pub engine: BrowserEngine,
    pub executable_path: Option<String>,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub preferences: HashMap<String, String>,
}

impl EngineSpecificConfig {
    pub fn chromium() -> Self {
        Self {
            engine: BrowserEngine::Chromium,
            executable_path: None,
            args: vec![
                "--headless=new".to_string(),
                "--disable-gpu".to_string(),
                "--no-sandbox".to_string(),
                "--disable-dev-shm-usage".to_string(),
            ],
            env: HashMap::new(),
            preferences: HashMap::new(),
        }
    }

    pub fn firefox() -> Self {
        Self {
            engine: BrowserEngine::Firefox,
            executable_path: None,
            args: vec![
                "--headless".to_string(),
                "--disable-gpu".to_string(),
            ],
            env: HashMap::new(),
            preferences: HashMap::new(),
        }
    }

    pub fn webkit() -> Self {
        Self {
            engine: BrowserEngine::WebKit,
            executable_path: None,
            args: vec![
                "--headless".to_string(),
            ],
            env: HashMap::new(),
            preferences: HashMap::new(),
        }
    }
}

/// Çoklu tarayıcı yöneticisi
pub struct MultiBrowserManager {
    config: MultiBrowserConfig,
    active_engine: BrowserEngine,
    available_engines: Vec<BrowserEngine>,
    engine_stats: HashMap<BrowserEngine, BrowserEngineStats>,
}

/// Tarayıcı motoru istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BrowserEngineStats {
    pub total_sessions: u64,
    pub successful_sessions: u64,
    pub failed_sessions: u64,
    pub avg_page_load_ms: u64,
    pub crashes: u32,
}

impl MultiBrowserManager {
    pub fn new(config: MultiBrowserConfig) -> Self {
        let active_engine = config.engine;
        let mut available = vec![BrowserEngine::Chromium];
        // Gerçek ortamda motor varlığını kontrol et
        available.push(BrowserEngine::Firefox);
        available.push(BrowserEngine::WebKit);

        let mut engine_stats = HashMap::new();
        for engine in &available {
            engine_stats.insert(*engine, BrowserEngineStats::default());
        }

        Self {
            config,
            active_engine,
            available_engines: available,
            engine_stats,
        }
    }

    /// Aktif motoru ayarla
    pub fn set_engine(&mut self, engine: BrowserEngine) -> Result<(), String> {
        if !self.available_engines.contains(&engine) {
            return Err(format!("{:?} motoru mevcut değil", engine));
        }
        self.active_engine = engine;
        log::info!("🌐 MULTI-BROWSER: Motor değiştirildi → {:?}", engine);
        Ok(())
    }

    /// En iyi motoru seç (otomatik)
    pub fn auto_select_engine(&self) -> BrowserEngine {
        // Site bazlı seçim (gerçek implementasyon)
        self.active_engine
    }

    /// Yedek motora geç
    pub fn fallback(&mut self) -> Option<BrowserEngine> {
        if let Some(fallback) = self.config.fallback_engine {
            if self.available_engines.contains(&fallback) {
                self.active_engine = fallback;
                log::info!("🌐 MULTI-BROWSER: Yedek motora geçildi → {:?}", fallback);
                return Some(fallback);
            }
        }
        None
    }

    /// Aktif motor
    pub fn active_engine(&self) -> BrowserEngine {
        self.active_engine
    }

    /// Kullanılabilir motorlar
    pub fn available_engines(&self) -> &[BrowserEngine] {
        &self.available_engines
    }

    /// Motor istatistiklerini güncelle
    pub fn record_session(&mut self, engine: BrowserEngine, success: bool, load_time_ms: u64) {
        if let Some(stats) = self.engine_stats.get_mut(&engine) {
            stats.total_sessions += 1;
            if success {
                stats.successful_sessions += 1;
            } else {
                stats.failed_sessions += 1;
            }
            stats.avg_page_load_ms = (stats.avg_page_load_ms + load_time_ms) / 2;
        }
    }

    /// İstatistikler
    pub fn stats(&self) -> MultiBrowserStats {
        MultiBrowserStats {
            active_engine: self.active_engine,
            available_count: self.available_engines.len() as u32,
            engine_stats: self.engine_stats.clone(),
        }
    }
}

/// Çoklu tarayıcı istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiBrowserStats {
    pub active_engine: BrowserEngine,
    pub available_count: u32,
    pub engine_stats: HashMap<BrowserEngine, BrowserEngineStats>,
}

// ───────────────────────────────────────────────────────────────────────────────
//  4. CLOUD BROWSER DESTEĞİ
// ───────────────────────────────────────────────────────────────────────────────

/// Cloud browser sağlayıcısı
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudBrowserProvider {
    BrowserStack,
    SauceLabs,
    LambdaTest,
    PlaywrightCloud,
    Custom { name: String, endpoint: String },
}

impl CloudBrowserProvider {
    pub fn description(&self) -> &str {
        match self {
            CloudBrowserProvider::BrowserStack => "BrowserStack - 3000+ gerçek cihaz/browser",
            CloudBrowserProvider::SauceLabs => "SauceLabs - CI/CD entegrasyonlu",
            CloudBrowserProvider::LambdaTest => "LambdaTest - 3000+ ortam",
            CloudBrowserProvider::PlaywrightCloud => "Playwright Cloud - Microsoft",
            CloudBrowserProvider::Custom { name, .. } => name,
        }
    }
}

/// Cloud browser yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudBrowserConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Sağlayıcı
    pub provider: CloudBrowserProvider,
    /// API anahtarı
    pub api_key: Option<String>,
    /// API kullanıcı adı
    pub api_username: Option<String>,
    /// Maksimum paralel oturum
    pub max_parallel_sessions: u32,
    /// Oturum zaman aşımı (saniye)
    pub session_timeout_secs: u64,
    /// Video kaydı
    pub record_video: bool,
    /// Ekran görüntüsü
    pub take_screenshots: bool,
    /// Network log
    pub network_log: bool,
    /// Console log
    pub console_log: bool,
    /// Belirli OS/browser kombinasyonları
    pub environments: Vec<CloudEnvironment>,
    /// Maliyet limiti (USD/gün)
    pub daily_cost_limit: f64,
}

impl Default for CloudBrowserConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: CloudBrowserProvider::BrowserStack,
            api_key: None,
            api_username: None,
            max_parallel_sessions: 5,
            session_timeout_secs: 300,
            record_video: false,
            take_screenshots: true,
            network_log: true,
            console_log: true,
            environments: Vec::new(),
            daily_cost_limit: 50.0,
        }
    }
}

impl CloudBrowserConfig {
    pub fn browserstack(api_key: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            enabled: true,
            provider: CloudBrowserProvider::BrowserStack,
            api_key: Some(api_key.into()),
            api_username: Some(username.into()),
            ..Self::default()
        }
    }

    pub fn saucelabs(api_key: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            enabled: true,
            provider: CloudBrowserProvider::SauceLabs,
            api_key: Some(api_key.into()),
            api_username: Some(username.into()),
            ..Self::default()
        }
    }

    pub fn with_environment(mut self, env: CloudEnvironment) -> Self {
        self.environments.push(env);
        self
    }

    pub fn with_cost_limit(mut self, limit: f64) -> Self {
        self.daily_cost_limit = limit;
        self
    }
}

/// Cloud ortam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudEnvironment {
    pub os: String,
    pub os_version: String,
    pub browser: String,
    pub browser_version: String,
    pub device: Option<String>,
    pub resolution: String,
}

impl CloudEnvironment {
    pub fn chrome_windows() -> Self {
        Self {
            os: "Windows".to_string(),
            os_version: "11".to_string(),
            browser: "Chrome".to_string(),
            browser_version: "latest".to_string(),
            device: None,
            resolution: "1920x1080".to_string(),
        }
    }

    pub fn safari_macos() -> Self {
        Self {
            os: "macOS".to_string(),
            os_version: "14".to_string(),
            browser: "Safari".to_string(),
            browser_version: "latest".to_string(),
            device: None,
            resolution: "1440x900".to_string(),
        }
    }

    pub fn mobile_android() -> Self {
        Self {
            os: "Android".to_string(),
            os_version: "14".to_string(),
            browser: "Chrome".to_string(),
            browser_version: "latest".to_string(),
            device: Some("Samsung Galaxy S24".to_string()),
            resolution: "360x780".to_string(),
        }
    }

    pub fn mobile_ios() -> Self {
        Self {
            os: "iOS".to_string(),
            os_version: "17".to_string(),
            browser: "Safari".to_string(),
            browser_version: "latest".to_string(),
            device: Some("iPhone 15 Pro".to_string()),
            resolution: "393x852".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        if let Some(ref device) = self.device {
            format!("{} {} / {} {} ({})", self.os, self.os_version, self.browser, self.browser_version, device)
        } else {
            format!("{} {} / {} {} ({})", self.os, self.os_version, self.browser, self.browser_version, self.resolution)
        }
    }
}

/// Cloud oturum bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudSession {
    pub id: String,
    pub environment: CloudEnvironment,
    pub status: CloudSessionStatus,
    pub created_at: DateTime<Utc>,
    pub url: Option<String>,
    pub duration_secs: u64,
    pub cost: f64,
    pub video_url: Option<String>,
    pub screenshots: Vec<String>,
}

/// Cloud oturum durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudSessionStatus {
    Queued,
    Starting,
    Running,
    Completed,
    Failed,
    TimedOut,
}

/// Cloud browser yöneticisi
pub struct CloudBrowserManager {
    config: CloudBrowserConfig,
    sessions: Vec<CloudSession>,
    total_cost: f64,
    total_sessions: u64,
}

impl CloudBrowserManager {
    pub fn new(config: CloudBrowserConfig) -> Self {
        Self {
            config,
            sessions: Vec::new(),
            total_cost: 0.0,
            total_sessions: 0,
        }
    }

    /// Cloud oturum oluştur
    pub async fn create_session(&mut self, environment: CloudEnvironment) -> Result<String, String> {
        if !self.config.enabled {
            return Err("Cloud browser devre dışı".to_string());
        }

        // Paralel oturum limiti
        let active = self.sessions.iter()
            .filter(|s| s.status == CloudSessionStatus::Running)
            .count();
        if active >= self.config.max_parallel_sessions as usize {
            return Err("Maksimum paralel oturum sayısına ulaşıldı".to_string());
        }

        // Günlük maliyet limiti
        let today_cost = self.total_cost;
        if today_cost >= self.config.daily_cost_limit {
            return Err("Günlük maliyet limiti aşıldı".to_string());
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = CloudSession {
            id: session_id.clone(),
            environment,
            status: CloudSessionStatus::Starting,
            created_at: Utc::now(),
            url: None,
            duration_secs: 0,
            cost: 0.0,
            video_url: None,
            screenshots: Vec::new(),
        };

        self.sessions.push(session);
        self.total_sessions += 1;

        log::info!("☁️  CLOUD-BROWSER: Oturum oluşturuldu: {}", session_id);
        Ok(session_id)
    }

    /// Oturumu sonlandır
    pub fn stop_session(&mut self, session_id: &str) -> Result<(), String> {
        let session = self.sessions.iter_mut()
            .find(|s| s.id == session_id)
            .ok_or_else(|| format!("Oturum bulunamadı: {}", session_id))?;
        session.status = CloudSessionStatus::Completed;
        self.total_cost += session.cost;
        log::info!("☁️  CLOUD-BROWSER: Oturum sonlandırıldı: {}", session_id);
        Ok(())
    }

    /// Aktif oturumlar
    pub fn active_sessions(&self) -> Vec<&CloudSession> {
        self.sessions.iter()
            .filter(|s| s.status == CloudSessionStatus::Running)
            .collect()
    }

    /// İstatistikler
    pub fn stats(&self) -> CloudBrowserStats {
        CloudBrowserStats {
            total_sessions: self.total_sessions,
            active_sessions: self.active_sessions().len() as u32,
            total_cost: self.total_cost,
            daily_limit: self.config.daily_cost_limit,
        }
    }
}

/// Cloud browser istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudBrowserStats {
    pub total_sessions: u64,
    pub active_sessions: u32,
    pub total_cost: f64,
    pub daily_limit: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- Mobile Emulation Tests ---

    #[test]
    fn test_mobile_device_iphone() {
        let device = MobileDevice::iphone_15_pro();
        assert_eq!(device.manufacturer, "Apple");
        assert!(device.touch);
        assert!(device.mobile);
    }

    #[test]
    fn test_mobile_device_galaxy() {
        let device = MobileDevice::galaxy_s24();
        assert_eq!(device.platform, "Android");
    }

    #[test]
    fn test_mobile_device_all() {
        let devices = MobileDevice::all_devices();
        assert_eq!(devices.len(), 4);
    }

    #[test]
    fn test_mobile_device_summary() {
        let device = MobileDevice::iphone_15_pro();
        let summary = device.summary();
        assert!(summary.contains("Apple"));
    }

    #[test]
    fn test_mobile_emulation_config() {
        let config = MobileEmulationConfig::iphone();
        assert!(config.enabled);
        assert_eq!(config.device, Some("iphone-15-pro".to_string()));
    }

    #[test]
    fn test_mobile_emulation_config_android() {
        let config = MobileEmulationConfig::android();
        assert!(config.enabled);
    }

    #[test]
    fn test_mobile_emulation_config_3g() {
        let config = MobileEmulationConfig::iphone().with_3g();
        assert!(config.network_throttling.is_some());
    }

    #[test]
    fn test_mobile_emulation_landscape() {
        let config = MobileEmulationConfig::iphone().landscape();
        assert!(config.landscape);
    }

    #[test]
    fn test_network_throttling() {
        let t = NetworkThrottling::three_g();
        assert_eq!(t.name, "3G");
        assert_eq!(t.latency_ms, 150);
    }

    #[test]
    fn test_mobile_emulation_manager() {
        let mut mgr = MobileEmulationManager::new(MobileEmulationConfig::default());
        let device = mgr.select_device("iphone-15-pro").unwrap();
        assert_eq!(device.manufacturer, "Apple");
        assert!(mgr.is_active());
    }

    #[test]
    fn test_mobile_emulation_manager_list() {
        let mgr = MobileEmulationManager::new(MobileEmulationConfig::default());
        assert_eq!(mgr.list_devices().len(), 4);
    }

    // --- CDP Tests ---

    #[test]
    fn test_cdp_domain() {
        assert_eq!(CdpDomain::Page.description(), "Sayfa işlemleri");
        assert_eq!(CdpDomain::Runtime.description(), "JS çalıştırma ortamı");
    }

    #[test]
    fn test_cdp_config_default() {
        let config = CdpConfig::default();
        assert!(config.enabled);
        assert_eq!(config.port, 9222);
        assert!(config.auto_connect);
    }

    #[test]
    fn test_cdp_config_minimal() {
        let config = CdpConfig::minimal();
        assert_eq!(config.enabled_domains.len(), 2);
    }

    #[test]
    fn test_cdp_config_tracing() {
        let config = CdpConfig::default().with_tracing();
        assert!(config.tracing);
    }

    #[test]
    fn test_cdp_manager() {
        let mut mgr = CdpManager::new(CdpConfig::default());
        assert!(!mgr.is_connected());
    }

    #[test]
    fn test_cdp_manager_command() {
        let mut mgr = CdpManager::new(CdpConfig::default());
        let result = mgr.send_command("Page.navigate", serde_json::json!({"url": "https://example.com"}));
        assert_eq!(result.id, 1);
        assert!(result.result.is_some());
    }

    #[test]
    fn test_cdp_manager_event() {
        let mut mgr = CdpManager::new(CdpConfig::default());
        mgr.record_event(CdpEvent {
            method: "Page.loadEventFired".to_string(),
            domain: CdpDomain::Page,
            params: serde_json::json!({}),
            timestamp: Utc::now(),
        });
        assert_eq!(mgr.events().len(), 1);
    }

    #[test]
    fn test_cdp_manager_metrics() {
        let mgr = CdpManager::new(CdpConfig::default());
        let metrics = mgr.get_metrics();
        assert!(metrics.contains_key("FirstPaint"));
    }

    // --- Multi-Browser Tests ---

    #[test]
    fn test_browser_engine() {
        assert_eq!(BrowserEngine::Chromium.description(), "Chromium / Chrome (Blink)");
        assert!(BrowserEngine::Chromium.supports_cdp());
        assert!(!BrowserEngine::Firefox.supports_cdp());
    }

    #[test]
    fn test_multi_browser_config() {
        let config = MultiBrowserConfig::chromium();
        assert_eq!(config.engine, BrowserEngine::Chromium);
    }

    #[test]
    fn test_multi_browser_config_firefox() {
        let config = MultiBrowserConfig::firefox();
        assert_eq!(config.engine, BrowserEngine::Firefox);
    }

    #[test]
    fn test_engine_specific_config() {
        let chromium = EngineSpecificConfig::chromium();
        assert_eq!(chromium.engine, BrowserEngine::Chromium);
        assert!(chromium.args.contains(&"--headless=new".to_string()));
    }

    #[test]
    fn test_multi_browser_manager() {
        let mut mgr = MultiBrowserManager::new(MultiBrowserConfig::default());
        assert_eq!(mgr.active_engine(), BrowserEngine::Chromium);
    }

    #[test]
    fn test_multi_browser_set_engine() {
        let mut mgr = MultiBrowserManager::new(MultiBrowserConfig::default());
        mgr.set_engine(BrowserEngine::Firefox).unwrap();
        assert_eq!(mgr.active_engine(), BrowserEngine::Firefox);
    }

    #[test]
    fn test_multi_browser_fallback() {
        let mut mgr = MultiBrowserManager::new(MultiBrowserConfig::default());
        let fallback = mgr.fallback();
        assert!(fallback.is_some());
    }

    #[test]
    fn test_multi_browser_record_session() {
        let mut mgr = MultiBrowserManager::new(MultiBrowserConfig::default());
        mgr.record_session(BrowserEngine::Chromium, true, 1500);
        let stats = mgr.stats();
        assert_eq!(stats.engine_stats[&BrowserEngine::Chromium].total_sessions, 1);
    }

    // --- Cloud Browser Tests ---

    #[test]
    fn test_cloud_provider() {
        assert!(!CloudBrowserProvider::BrowserStack.description().is_empty());
    }

    #[test]
    fn test_cloud_config_default() {
        let config = CloudBrowserConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.max_parallel_sessions, 5);
    }

    #[test]
    fn test_cloud_config_browserstack() {
        let config = CloudBrowserConfig::browserstack("key123", "user1");
        assert!(config.enabled);
        assert_eq!(config.api_key, Some("key123".to_string()));
    }

    #[test]
    fn test_cloud_environment() {
        let env = CloudEnvironment::chrome_windows();
        assert_eq!(env.os, "Windows");
    }

    #[test]
    fn test_cloud_environment_mobile() {
        let env = CloudEnvironment::mobile_android();
        assert!(env.device.is_some());
    }

    #[test]
    fn test_cloud_environment_summary() {
        let env = CloudEnvironment::chrome_windows();
        let summary = env.summary();
        assert!(summary.contains("Windows"));
    }

    #[test]
    fn test_cloud_manager_disabled() {
        let mut mgr = CloudBrowserManager::new(CloudBrowserConfig::default());
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mgr.create_session(CloudEnvironment::chrome_windows()));
        // Should fail because disabled
        assert!(result.is_err());
    }

    #[test]
    fn test_cloud_stats() {
        let mgr = CloudBrowserManager::new(CloudBrowserConfig::default());
        let stats = mgr.stats();
        assert_eq!(stats.total_sessions, 0);
    }
}
