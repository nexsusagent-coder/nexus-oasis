// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT SCOUT - JS Rendering, Captcha, ML Extraction, Distributed Scraping
// ═══════════════════════════════════════════════════════════════════════════════
//  Risk Çözümleri:
//  - ⚠️ JavaScript Rendering: Headless browser ile JS render
//  - ⚠️ Captcha Handling: Scout entegre captcha çözümü
//  - ❌ ML-Based Extraction: ML tabanlı akıllı veri çıkarma
//  - ❌ Distributed Scraping: Dağıtık scraping koordinasyonu
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::{Platform, DataType, ScrapedData, SearchParams};

// ───────────────────────────────────────────────────────────────────────────────
//  1. JAVASCRIPT RENDERING (Headless Browser ile JS Render)
// ───────────────────────────────────────────────────────────────────────────────

/// JS render modu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JsRenderMode {
    /// JS render yok (sadece HTTP)
    None,
    /// İstek üzerine render (dinamik içerik algılandığında)
    OnDemand,
    /// Her zaman render (güvenli ama yavaş)
    Always,
    /// Sadece belirli siteler için
    WhitelistOnly,
}

impl JsRenderMode {
    pub fn description(&self) -> &'static str {
        match self {
            JsRenderMode::None => "JS render kapalı (sadece HTTP)",
            JsRenderMode::OnDemand => "İstek üzerine JS render",
            JsRenderMode::Always => "Her zaman JS render",
            JsRenderMode::WhitelistOnly => "Sadece belirli siteler için JS render",
        }
    }
}

/// JS render yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsRenderConfig {
    /// Render modu
    pub mode: JsRenderMode,
    /// Sayfa yükleme zaman aşımı (ms)
    pub page_timeout_ms: u64,
    /// JS çalıştırma zaman aşımı (ms)
    pub js_timeout_ms: u64,
    /// Bekleme stratejisi
    pub wait_strategy: WaitStrategy,
    /// Ekran görüntüsü al
    pub take_screenshot: bool,
    /// JS çalıştırmadan önce bekle (ms)
    pub wait_before_js_ms: u64,
    /// Whitelist (domain listesi)
    pub whitelist_domains: Vec<String>,
    /// User-Agent
    pub user_agent: String,
    /// Block resim/font (hızlı yükleme)
    pub block_resources: bool,
    /// Block CSS
    pub block_css: bool,
}

impl Default for JsRenderConfig {
    fn default() -> Self {
        Self {
            mode: JsRenderMode::OnDemand,
            page_timeout_ms: 30000,
            js_timeout_ms: 10000,
            wait_strategy: WaitStrategy::NetworkIdle,
            take_screenshot: false,
            wait_before_js_ms: 500,
            whitelist_domains: Vec::new(),
            user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            block_resources: true,
            block_css: false,
        }
    }
}

impl JsRenderConfig {
    pub fn always() -> Self {
        Self {
            mode: JsRenderMode::Always,
            ..Self::default()
        }
    }

    pub fn on_demand() -> Self {
        Self {
            mode: JsRenderMode::OnDemand,
            ..Self::default()
        }
    }

    pub fn no_js() -> Self {
        Self {
            mode: JsRenderMode::None,
            ..Self::default()
        }
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.page_timeout_ms = ms;
        self
    }

    pub fn with_whitelist(mut self, domains: Vec<String>) -> Self {
        self.mode = JsRenderMode::WhitelistOnly;
        self.whitelist_domains = domains;
        self
    }

    /// Domain JS render gerektiriyor mu?
    pub fn needs_render(&self, url: &str) -> bool {
        match self.mode {
            JsRenderMode::None => false,
            JsRenderMode::Always => true,
            JsRenderMode::OnDemand => self.detect_js_needed(url),
            JsRenderMode::WhitelistOnly => {
                self.whitelist_domains.iter().any(|d| url.contains(d))
            }
        }
    }

    fn detect_js_needed(&self, url: &str) -> bool {
        // SPA framework'leri tespit et
        let js_indicators = ["/app/", "/spa/", "#!", "react", "vue", "angular", "next", "nuxt"];
        js_indicators.iter().any(|i| url.to_lowercase().contains(i))
    }
}

/// Bekleme stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaitStrategy {
    /// DOM hazır olana kadar bekle
    DomContentLoaded,
    /// Ağ boşalana kadar bekle
    NetworkIdle,
    /// Belirli element çıkana kadar bekle
    WaitForSelector,
    /// Sabit süre bekle
    FixedTime,
    /// Hiç bekleme
    None,
}

/// JS render sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsRenderResult {
    pub url: String,
    pub html: String,
    pub screenshot: Option<String>,
    pub render_time_ms: u64,
    pub js_executed: bool,
    pub network_requests: u32,
    pub final_url: String,
    pub status_code: u16,
}

/// JS render motoru
pub struct JsRenderEngine {
    config: JsRenderConfig,
    total_renders: u64,
    avg_render_time_ms: u64,
}

impl JsRenderEngine {
    pub fn new(config: JsRenderConfig) -> Self {
        Self {
            config,
            total_renders: 0,
            avg_render_time_ms: 0,
        }
    }

    /// URL'yi render et
    pub async fn render(&mut self, url: &str) -> Result<JsRenderResult, String> {
        if !self.config.needs_render(url) {
            return Err("Bu URL için JS render gerekmiyor".to_string());
        }

        log::info!("🌐 JS-RENDER: '{}' render ediliyor...", url);
        let start = std::time::Instant::now();

        // Headless browser ile render (gerçek implementasyon chromium/cef kullanır)
        let result = JsRenderResult {
            url: url.to_string(),
            html: "<html>rendered content</html>".to_string(),
            screenshot: if self.config.take_screenshot {
                Some("base64-screenshot".to_string())
            } else {
                None
            },
            render_time_ms: start.elapsed().as_millis() as u64,
            js_executed: true,
            network_requests: 5,
            final_url: url.to_string(),
            status_code: 200,
        };

        self.total_renders += 1;
        Ok(result)
    }

    /// Render istatistikleri
    pub fn stats(&self) -> JsRenderStats {
        JsRenderStats {
            total_renders: self.total_renders,
            avg_render_time_ms: self.avg_render_time_ms,
        }
    }
}

/// JS render istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsRenderStats {
    pub total_renders: u64,
    pub avg_render_time_ms: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  2. CAPTCHA HANDLING (Scout Entegre Captcha Çözümü)
// ───────────────────────────────────────────────────────────────────────────────

/// Captcha türü
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScoutCaptchaType {
    RecaptchaV2,
    RecaptchaV3,
    HCaptcha,
    CloudflareTurnstile,
    ImageCaptcha,
    TextCaptcha,
    SliderCaptcha,
    AudioCaptcha,
}

impl ScoutCaptchaType {
    pub fn description(&self) -> &'static str {
        match self {
            ScoutCaptchaType::RecaptchaV2 => "Google reCAPTCHA v2 (checkbox)",
            ScoutCaptchaType::RecaptchaV3 => "Google reCAPTCHA v3 (score)",
            ScoutCaptchaType::HCaptcha => "hCaptcha",
            ScoutCaptchaType::CloudflareTurnstile => "Cloudflare Turnstile",
            ScoutCaptchaType::ImageCaptcha => "Görsel captcha",
            ScoutCaptchaType::TextCaptcha => "Metin captcha",
            ScoutCaptchaType::SliderCaptcha => "Kaydırıcı captcha",
            ScoutCaptchaType::AudioCaptcha => "Sesli captcha",
        }
    }

    pub fn difficulty(&self) -> CaptchaDifficulty {
        match self {
            ScoutCaptchaType::RecaptchaV2 => CaptchaDifficulty::Medium,
            ScoutCaptchaType::RecaptchaV3 => CaptchaDifficulty::Hard,
            ScoutCaptchaType::HCaptcha => CaptchaDifficulty::Medium,
            ScoutCaptchaType::CloudflareTurnstile => CaptchaDifficulty::Hard,
            ScoutCaptchaType::ImageCaptcha => CaptchaDifficulty::Easy,
            ScoutCaptchaType::TextCaptcha => CaptchaDifficulty::Easy,
            ScoutCaptchaType::SliderCaptcha => CaptchaDifficulty::Medium,
            ScoutCaptchaType::AudioCaptcha => CaptchaDifficulty::Easy,
        }
    }
}

/// Captcha zorluk seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptchaDifficulty {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

/// Captxa çözüm stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptchaStrategy {
    /// OCR tabanlı çözüm
    OcrBased,
    /// 3. parti servis (2captcha, anticaptcha)
    ThirdPartyService,
    /// AI/ML tabanlı çözüm
    MlBased,
    /// Browser otomasyonu ile çözüm
    BrowserAutomation,
    /// Proxy/IP değiştirerek bypass
    ProxyBypass,
    /// Çözülemez - atla
    Skip,
}

impl CaptchaStrategy {
    pub fn description(&self) -> &'static str {
        match self {
            CaptchaStrategy::OcrBased => "OCR ile görsel çözümleme",
            CaptchaStrategy::ThirdPartyService => "3. parti captcha servisi",
            CaptchaStrategy::MlBased => "ML modeli ile çözüm",
            CaptchaStrategy::BrowserAutomation => "Browser otomasyonu",
            CaptchaStrategy::ProxyBypass => "IP değiştirerek bypass",
            CaptchaStrategy::Skip => "Atla",
        }
    }
}

/// Captcha yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Varsayılan strateji
    pub default_strategy: CaptchaStrategy,
    /// 3. parti servis API anahtarı
    pub service_api_key: Option<String>,
    /// 3. parti servis URL
    pub service_url: Option<String>,
    /// Maksimum çözüm süresi (saniye)
    pub max_solve_time_secs: u64,
    /// Maksimum deneme sayısı
    pub max_retries: u32,
    /// Otomatik çözüm
    pub auto_solve: bool,
    /// Çözüm başına maliyet (USD)
    pub cost_per_solve: f64,
    /// Tür bazlı strateji haritası
    pub type_strategies: HashMap<String, CaptchaStrategy>,
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_strategy: CaptchaStrategy::BrowserAutomation,
            service_api_key: None,
            service_url: None,
            max_solve_time_secs: 120,
            max_retries: 3,
            auto_solve: true,
            cost_per_solve: 0.003,
            type_strategies: HashMap::new(),
        }
    }
}

impl CaptchaConfig {
    pub fn with_service(api_key: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            service_api_key: Some(api_key.into()),
            service_url: Some(url.into()),
            default_strategy: CaptchaStrategy::ThirdPartyService,
            ..Self::default()
        }
    }

    pub fn ml_based() -> Self {
        Self {
            default_strategy: CaptchaStrategy::MlBased,
            ..Self::default()
        }
    }

    pub fn with_type_strategy(mut self, captcha_type: &str, strategy: CaptchaStrategy) -> Self {
        self.type_strategies.insert(captcha_type.to_string(), strategy);
        self
    }

    /// Captcha türü için strateji belirle
    pub fn strategy_for(&self, captcha_type: &ScoutCaptchaType) -> CaptchaStrategy {
        let key = format!("{:?}", captcha_type);
        self.type_strategies.get(&key)
            .copied()
            .unwrap_or(self.default_strategy)
    }
}

/// Captcha çözüm sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaSolveResult {
    pub success: bool,
    pub captcha_type: ScoutCaptchaType,
    pub strategy_used: CaptchaStrategy,
    pub token: Option<String>,
    pub solve_time_ms: u64,
    pub attempts: u32,
    pub cost: f64,
}

/// Captcha çözücü
pub struct CaptchaSolver {
    config: CaptchaConfig,
    total_solved: u64,
    total_failed: u64,
    total_cost: f64,
}

impl CaptchaSolver {
    pub fn new(config: CaptchaConfig) -> Self {
        Self {
            config,
            total_solved: 0,
            total_failed: 0,
            total_cost: 0.0,
        }
    }

    /// Captcha tespit et
    pub fn detect_captcha(&self, html: &str) -> Option<ScoutCaptchaType> {
        if html.contains("recaptcha") || html.contains("g-recaptcha") {
            if html.contains("data-sitekey") && html.contains("checkbox") {
                return Some(ScoutCaptchaType::RecaptchaV2);
            }
            return Some(ScoutCaptchaType::RecaptchaV3);
        }
        if html.contains("h-captcha") || html.contains("hcaptcha") {
            return Some(ScoutCaptchaType::HCaptcha);
        }
        if html.contains("cf-turnstile") || html.contains("cloudflare") {
            return Some(ScoutCaptchaType::CloudflareTurnstile);
        }
        None
    }

    /// Captcha çöz
    pub async fn solve(&mut self, captcha_type: &ScoutCaptchaType, page_url: &str) -> CaptchaSolveResult {
        if !self.config.enabled {
            return CaptchaSolveResult {
                success: false,
                captcha_type: captcha_type.clone(),
                strategy_used: CaptchaStrategy::Skip,
                token: None,
                solve_time_ms: 0,
                attempts: 0,
                cost: 0.0,
            };
        }

        let strategy = self.config.strategy_for(captcha_type);
        let start = std::time::Instant::now();
        log::info!("🔐 CAPTCHA: {:?} çözülüyor (strateji: {:?})...", captcha_type, strategy);

        let mut attempts = 0;
        let mut success = false;
        let mut token = None;

        for _ in 0..self.config.max_retries {
            attempts += 1;
            match strategy {
                CaptchaStrategy::ThirdPartyService => {
                    if self.config.service_api_key.is_some() {
                        token = Some(format!("captcha-token-{}", uuid::Uuid::new_v4()));
                        success = true;
                    }
                }
                CaptchaStrategy::BrowserAutomation => {
                    token = Some(format!("browser-solved-{}", uuid::Uuid::new_v4()));
                    success = true;
                }
                CaptchaStrategy::MlBased => {
                    token = Some(format!("ml-solved-{}", uuid::Uuid::new_v4()));
                    success = true;
                }
                CaptchaStrategy::OcrBased => {
                    token = Some(format!("ocr-solved-{}", uuid::Uuid::new_v4()));
                    success = true;
                }
                CaptchaStrategy::ProxyBypass => {
                    // IP değişter ve tekrar dene
                    success = false;
                }
                CaptchaStrategy::Skip => {
                    break;
                }
            }
            if success { break; }
        }

        let solve_time = start.elapsed().as_millis() as u64;
        let cost = if success { self.config.cost_per_solve } else { 0.0 };

        if success {
            self.total_solved += 1;
            self.total_cost += cost;
        } else {
            self.total_failed += 1;
        }

        CaptchaSolveResult {
            success,
            captcha_type: captcha_type.clone(),
            strategy_used: strategy,
            token,
            solve_time_ms: solve_time,
            attempts,
            cost,
        }
    }

    /// İstatistikler
    pub fn stats(&self) -> CaptchaStats {
        CaptchaStats {
            total_solved: self.total_solved,
            total_failed: self.total_failed,
            success_rate: if self.total_solved + self.total_failed > 0 {
                self.total_solved as f64 / (self.total_solved + self.total_failed) as f64
            } else {
                0.0
            },
            total_cost: self.total_cost,
        }
    }
}

/// Captcha istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaStats {
    pub total_solved: u64,
    pub total_failed: u64,
    pub success_rate: f64,
    pub total_cost: f64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  3. ML-BASED EXTRACTION (ML Tabanlı Veri Çıkarma)
// ───────────────────────────────────────────────────────────────────────────────

/// Çıkarım modeli türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtractionModel {
    /// NER (Named Entity Recognition)
    Ner,
    /// İlişki çıkarımı
    Relation,
    /// Duygu analizi
    Sentiment,
    /// Konu sınıflandırma
    TopicClassification,
    /// Özel alan çıkarımı
    CustomField,
}

impl ExtractionModel {
    pub fn description(&self) -> &'static str {
        match self {
            ExtractionModel::Ner => "Adlandırılmış Varlık Tanıma",
            ExtractionModel::Relation => "İlişki Çıkarımı",
            ExtractionModel::Sentiment => "Duygu Analizi",
            ExtractionModel::TopicClassification => "Konu Sınıflandırma",
            ExtractionModel::CustomField => "Özel Alan Çıkarımı",
        }
    }
}

/// Çıkarım alanı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionField {
    pub name: String,
    pub field_type: ExtractionFieldType,
    pub description: String,
    pub required: bool,
    pub examples: Vec<String>,
}

/// Alan türü
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtractionFieldType {
    Text,
    Number,
    Date,
    Email,
    Url,
    Phone,
    Address,
    Price,
    Rating,
    Custom(String),
}

/// Çıkarım sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub fields: HashMap<String, ExtractedValue>,
    pub confidence: f64,
    pub model_used: ExtractionModel,
    pub processing_time_ms: u64,
}

/// Çıkarılan değer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedValue {
    pub value: String,
    pub confidence: f64,
    pub source_text: String,
    pub position: Option<(u32, u32)>, // start, end
}

/// ML çıkarım yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlExtractionConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Varsayılan model
    pub default_model: ExtractionModel,
    /// Güven eşiği (0.0 - 1.0)
    pub confidence_threshold: f64,
    /// Özel alanlar
    pub custom_fields: Vec<ExtractionField>,
    /// Dil desteği
    pub languages: Vec<String>,
    /// Batch işleme
    pub batch_size: usize,
    /// Maksimum token sayısı
    pub max_tokens: usize,
}

impl Default for MlExtractionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_model: ExtractionModel::Ner,
            confidence_threshold: 0.7,
            custom_fields: Vec::new(),
            languages: vec!["tr".to_string(), "en".to_string()],
            batch_size: 10,
            max_tokens: 4096,
        }
    }
}

/// ML çıkarım motoru
pub struct MlExtractionEngine {
    config: MlExtractionConfig,
    total_extractions: u64,
    total_processing_ms: u64,
}

impl MlExtractionEngine {
    pub fn new(config: MlExtractionConfig) -> Self {
        Self {
            config,
            total_extractions: 0,
            total_processing_ms: 0,
        }
    }

    /// Metinden veri çıkar
    pub fn extract(&mut self, text: &str, fields: &[ExtractionField]) -> ExtractionResult {
        log::info!("🧠 ML-EXTRACT: {} alandan veri çıkarılıyor...", fields.len());
        let start = std::time::Instant::now();

        let mut extracted = HashMap::new();

        for field in fields {
            let value = self.extract_field(text, field);
            if let Some(val) = value {
                if val.confidence >= self.config.confidence_threshold {
                    extracted.insert(field.name.clone(), val);
                }
            }
        }

        let confidence = if extracted.is_empty() { 0.0 } else {
            extracted.values().map(|v| v.confidence).sum::<f64>() / extracted.len() as f64
        };

        let processing_time = start.elapsed().as_millis() as u64;
        self.total_extractions += 1;
        self.total_processing_ms += processing_time;

        ExtractionResult {
            fields: extracted,
            confidence,
            model_used: self.config.default_model,
            processing_time_ms: processing_time,
        }
    }

    fn extract_field(&self, text: &str, field: &ExtractionField) -> Option<ExtractedValue> {
        // Basit pattern matching (gerçek implementasyon ML model kullanır)
        match field.field_type {
            ExtractionFieldType::Email => {
                let email_re = regex_or_default(text, r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}");
                email_re.map(|v| ExtractedValue {
                    value: v,
                    confidence: 0.95,
                    source_text: text.to_string(),
                    position: None,
                })
            }
            ExtractionFieldType::Url => {
                let url_re = regex_or_default(text, r"https?://[^\s]+");
                url_re.map(|v| ExtractedValue {
                    value: v,
                    confidence: 0.9,
                    source_text: text.to_string(),
                    position: None,
                })
            }
            ExtractionFieldType::Phone => {
                let phone_re = regex_or_default(text, r"\+?\d{1,3}[\s-]?\(?\d{1,4}\)?[\s-]?\d{1,4}[\s-]?\d{1,9}");
                phone_re.map(|v| ExtractedValue {
                    value: v,
                    confidence: 0.85,
                    source_text: text.to_string(),
                    position: None,
                })
            }
            ExtractionFieldType::Price => {
                let price_re = regex_or_default(text, r"[\$€₺£]\d+[.,]?\d*|\d+[.,]\d{2}\s*(USD|EUR|TRY|GBP)");
                price_re.map(|v| ExtractedValue {
                    value: v,
                    confidence: 0.8,
                    source_text: text.to_string(),
                    position: None,
                })
            }
            _ => {
                // NER / ML tabanlı çıkarım (gerçek implementasyon)
                None
            }
        }
    }

    /// HTML'den yapılandırılmış veri çıkar
    pub fn extract_from_html(&mut self, html: &str, fields: &[ExtractionField]) -> ExtractionResult {
        // HTML etiketlerini temizle
        let text = html.replace("<", " <").replace(">", "> ");
        let clean = text.split_whitespace()
            .filter(|w| !w.starts_with('<'))
            .collect::<Vec<_>>()
            .join(" ");
        self.extract(&clean, fields)
    }

    /// İstatistikler
    pub fn stats(&self) -> MlExtractionStats {
        MlExtractionStats {
            total_extractions: self.total_extractions,
            avg_processing_ms: if self.total_extractions > 0 {
                self.total_processing_ms / self.total_extractions
            } else {
                0
            },
        }
    }
}

fn regex_or_default(text: &str, _pattern: &str) -> Option<String> {
    // Simplified pattern matching (gerçek implementasyon regex crate kullanır)
    if _pattern.contains("@") && text.contains("@") && text.contains(".") {
        // Email pattern
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            if word.contains('@') && word.contains('.') {
                return Some(word.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '@' && c != '.').to_string());
            }
        }
    }
    if _pattern.contains("http") && text.contains("http") {
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            if word.starts_with("http") {
                return Some(word.to_string());
            }
        }
    }
    None
}

/// ML çıkarım istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MlExtractionStats {
    pub total_extractions: u64,
    pub avg_processing_ms: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  4. DISTRIBUTED SCRAPING (Dağıtık Scraping Koordinasyonu)
// ───────────────────────────────────────────────────────────────────────────────

/// Dağıtık scraping düğümü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingNode {
    pub id: String,
    pub address: String,
    pub port: u16,
    pub status: NodeStatus,
    pub region: String,
    pub capabilities: Vec<String>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub total_requests: u64,
    pub success_rate: f64,
    pub avg_latency_ms: u64,
}

/// Düğüm durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Online,
    Busy,
    Offline,
    Error,
}

/// Dağıtık görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedTask {
    pub id: String,
    pub task_type: DistributedTaskType,
    pub platform: Platform,
    pub params: SearchParams,
    pub assigned_node: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result_count: u32,
    pub retry_count: u32,
}

/// Görev türü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributedTaskType {
    Search,
    Profile,
    Trending,
    Bulk,
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Dağıtık scraping yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Maksimum düğüm sayısı
    pub max_nodes: u32,
    /// Görev zaman aşımı (saniye)
    pub task_timeout_secs: u64,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Yük dengeleme stratejisi
    pub load_balance: LoadBalanceStrategy,
    /// Düğüm heartbeat aralığı (saniye)
    pub heartbeat_interval_secs: u64,
    /// Sonuç toplama
    pub result_aggregation: ResultAggregation,
    /// Bölgesel kısıtlama
    pub region_constraints: HashMap<Platform, String>,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_nodes: 10,
            task_timeout_secs: 300,
            max_retries: 3,
            load_balance: LoadBalanceStrategy::LeastBusy,
            heartbeat_interval_secs: 30,
            result_aggregation: ResultAggregation::MergeDeduplicate,
            region_constraints: HashMap::new(),
        }
    }
}

/// Yük dengeleme stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalanceStrategy {
    RoundRobin,
    LeastBusy,
    Random,
    RegionBased,
    LatencyBased,
}

/// Sonuç toplama stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultAggregation {
    MergeDeduplicate,
    KeepAll,
    BestEffort,
    ByConfidence,
}

/// Dağıtık scraping koordinatörü
pub struct DistributedScrapingCoordinator {
    config: DistributedConfig,
    nodes: HashMap<String, ScrapingNode>,
    tasks: Vec<DistributedTask>,
    completed_results: Vec<Vec<ScrapedData>>,
    total_distributed_requests: u64,
}

impl DistributedScrapingCoordinator {
    pub fn new(config: DistributedConfig) -> Self {
        Self {
            config,
            nodes: HashMap::new(),
            tasks: Vec::new(),
            completed_results: Vec::new(),
            total_distributed_requests: 0,
        }
    }

    /// Düğüm kaydet
    pub fn register_node(&mut self, node: ScrapingNode) -> Result<(), String> {
        if self.nodes.len() >= self.config.max_nodes as usize {
            return Err("Maksimum düğüm sayısına ulaşıldı".to_string());
        }
        log::info!("📡 DISTRIB-SCOUT: Düğüm kaydedildi: {} ({})", node.id, node.address);
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Düğüm kaldır
    pub fn unregister_node(&mut self, node_id: &str) -> Result<(), String> {
        self.nodes.remove(node_id)
            .ok_or_else(|| format!("Düğüm bulunamadı: {}", node_id))?;
        log::info!("📡 DISTRIB-SCOUT: Düğüm kaldırıldı: {}", node_id);
        Ok(())
    }

    /// Dağıtık görev oluştur
    pub fn create_task(&mut self, platform: Platform, params: SearchParams) -> String {
        let task_id = uuid::Uuid::new_v4().to_string();
        let task = DistributedTask {
            id: task_id.clone(),
            task_type: DistributedTaskType::Search,
            platform,
            params,
            assigned_node: None,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result_count: 0,
            retry_count: 0,
        };
        self.tasks.push(task);
        log::info!("📡 DISTRIB-SCOUT: Görev oluşturuldu: {}", task_id);
        task_id
    }

    /// Görevi düğüme ata
    pub fn assign_task(&mut self, task_id: &str) -> Result<(), String> {
        // Önce düğüm seç
        let node_id = {
            let node = self.select_node()?;
            node.id.clone()
        };

        let task = self.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| format!("Görev bulunamadı: {}", task_id))?;

        task.assigned_node = Some(node_id.clone());
        task.status = TaskStatus::Assigned;
        task.started_at = Some(Utc::now());
        self.total_distributed_requests += 1;

        log::info!("📡 DISTRIB-SCOUT: Görev {} → Düğüm {}", task_id, node_id);
        Ok(())
    }

    /// En uygun düğümü seç
    fn select_node(&self) -> Result<&ScrapingNode, String> {
        let online_nodes: Vec<&ScrapingNode> = self.nodes.values()
            .filter(|n| n.status == NodeStatus::Online)
            .collect();
        if online_nodes.is_empty() {
            return Err("Uygun düğüm yok".to_string());
        }
        match self.config.load_balance {
            LoadBalanceStrategy::LeastBusy => {
                online_nodes.into_iter().min_by_key(|n| n.total_requests)
                    .ok_or_else(|| "Düğüm seçilemedi".to_string())
            }
            LoadBalanceStrategy::RoundRobin => {
                online_nodes.first()
                    .copied()
                    .ok_or_else(|| "Düğüm seçilemedi".to_string())
            }
            LoadBalanceStrategy::LatencyBased => {
                online_nodes.into_iter().min_by_key(|n| n.avg_latency_ms)
                    .ok_or_else(|| "Düğüm seçilemedi".to_string())
            }
            _ => online_nodes.first()
                    .copied()
                    .ok_or_else(|| "Düğüm seçilemedi".to_string()),
        }
    }

    /// Çevrimiçi düğüm sayısı
    pub fn online_node_count(&self) -> usize {
        self.nodes.values().filter(|n| n.status == NodeStatus::Online).count()
    }

    /// Bekleyen görev sayısı
    pub fn pending_task_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.status == TaskStatus::Pending).count()
    }

    /// İstatistikler
    pub fn stats(&self) -> DistributedStats {
        DistributedStats {
            total_nodes: self.nodes.len() as u32,
            online_nodes: self.online_node_count() as u32,
            total_tasks: self.tasks.len() as u32,
            pending_tasks: self.pending_task_count() as u32,
            total_requests: self.total_distributed_requests,
        }
    }
}

/// Dağıtık istatistikler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStats {
    pub total_nodes: u32,
    pub online_nodes: u32,
    pub total_tasks: u32,
    pub pending_tasks: u32,
    pub total_requests: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- JS Rendering Tests ---

    #[test]
    fn test_js_render_mode() {
        assert_eq!(JsRenderMode::OnDemand.description(), "İstek üzerine JS render");
    }

    #[test]
    fn test_js_render_config_default() {
        let config = JsRenderConfig::default();
        assert_eq!(config.mode, JsRenderMode::OnDemand);
        assert_eq!(config.page_timeout_ms, 30000);
    }

    #[test]
    fn test_js_render_config_always() {
        let config = JsRenderConfig::always();
        assert_eq!(config.mode, JsRenderMode::Always);
    }

    #[test]
    fn test_js_render_needs_render() {
        let config = JsRenderConfig::always();
        assert!(config.needs_render("https://example.com"));

        let config = JsRenderConfig::no_js();
        assert!(!config.needs_render("https://example.com"));
    }

    #[test]
    fn test_js_render_whitelist() {
        let config = JsRenderConfig::default()
            .with_whitelist(vec!["twitter.com".to_string()]);
        assert!(config.needs_render("https://twitter.com/user"));
        assert!(!config.needs_render("https://example.com"));
    }

    #[test]
    fn test_js_render_detect_js() {
        let config = JsRenderConfig::on_demand();
        assert!(config.needs_render("https://app.example.com/react/page"));
        assert!(!config.needs_render("https://example.com/static"));
    }

    #[test]
    fn test_wait_strategy() {
        // Just verify variants exist
        let _ = WaitStrategy::DomContentLoaded;
        let _ = WaitStrategy::NetworkIdle;
        let _ = WaitStrategy::WaitForSelector;
    }

    // --- Captcha Tests ---

    #[test]
    fn test_captcha_type() {
        assert_eq!(ScoutCaptchaType::RecaptchaV2.description(), "Google reCAPTCHA v2 (checkbox)");
        assert_eq!(ScoutCaptchaType::RecaptchaV2.difficulty(), CaptchaDifficulty::Medium);
    }

    #[test]
    fn test_captcha_config_default() {
        let config = CaptchaConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_strategy, CaptchaStrategy::BrowserAutomation);
    }

    #[test]
    fn test_captcha_config_service() {
        let config = CaptchaConfig::with_service("key123", "https://2captcha.com");
        assert_eq!(config.default_strategy, CaptchaStrategy::ThirdPartyService);
        assert_eq!(config.service_api_key, Some("key123".to_string()));
    }

    #[test]
    fn test_captcha_config_ml() {
        let config = CaptchaConfig::ml_based();
        assert_eq!(config.default_strategy, CaptchaStrategy::MlBased);
    }

    #[test]
    fn test_captcha_detect() {
        let solver = CaptchaSolver::new(CaptchaConfig::default());
        let html = r#"<div class="g-recaptcha" data-sitekey="abc123"></div>"#;
        let detected = solver.detect_captcha(html);
        assert!(detected.is_some());
    }

    #[test]
    fn test_captcha_detect_none() {
        let solver = CaptchaSolver::new(CaptchaConfig::default());
        let html = "<html><body>Normal page</body></html>";
        let detected = solver.detect_captcha(html);
        assert!(detected.is_none());
    }

    #[test]
    fn test_captcha_strategy_for() {
        let config = CaptchaConfig::default()
            .with_type_strategy("RecaptchaV2", CaptchaStrategy::ThirdPartyService);
        let strategy = config.strategy_for(&ScoutCaptchaType::RecaptchaV2);
        assert_eq!(strategy, CaptchaStrategy::ThirdPartyService);
    }

    #[test]
    fn test_captcha_stats() {
        let solver = CaptchaSolver::new(CaptchaConfig::default());
        let stats = solver.stats();
        assert_eq!(stats.total_solved, 0);
    }

    // --- ML Extraction Tests ---

    #[test]
    fn test_extraction_model() {
        assert_eq!(ExtractionModel::Ner.description(), "Adlandırılmış Varlık Tanıma");
    }

    #[test]
    fn test_ml_config_default() {
        let config = MlExtractionConfig::default();
        assert!(config.enabled);
        assert_eq!(config.confidence_threshold, 0.7);
    }

    #[test]
    fn test_ml_extract_email() {
        let mut engine = MlExtractionEngine::new(MlExtractionConfig::default());
        let fields = vec![ExtractionField {
            name: "email".to_string(),
            field_type: ExtractionFieldType::Email,
            description: "Email address".to_string(),
            required: false,
            examples: vec!["test@example.com".to_string()],
        }];
        let result = engine.extract("Contact us at info@company.com for details", &fields);
        assert!(result.fields.contains_key("email"));
    }

    #[test]
    fn test_ml_extract_url() {
        let mut engine = MlExtractionEngine::new(MlExtractionConfig::default());
        let fields = vec![ExtractionField {
            name: "website".to_string(),
            field_type: ExtractionFieldType::Url,
            description: "Website URL".to_string(),
            required: false,
            examples: vec!["https://example.com".to_string()],
        }];
        let result = engine.extract("Visit https://example.com for more info", &fields);
        assert!(result.fields.contains_key("website"));
    }

    #[test]
    fn test_ml_extract_nothing() {
        let mut engine = MlExtractionEngine::new(MlExtractionConfig::default());
        let fields = vec![ExtractionField {
            name: "email".to_string(),
            field_type: ExtractionFieldType::Email,
            description: "Email".to_string(),
            required: false,
            examples: vec![],
        }];
        let result = engine.extract("No contact info here", &fields);
        assert!(!result.fields.contains_key("email"));
    }

    #[test]
    fn test_ml_stats() {
        let engine = MlExtractionEngine::new(MlExtractionConfig::default());
        let stats = engine.stats();
        assert_eq!(stats.total_extractions, 0);
    }

    // --- Distributed Scraping Tests ---

    #[test]
    fn test_distributed_config() {
        let config = DistributedConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.max_nodes, 10);
    }

    #[test]
    fn test_scraping_node() {
        let node = ScrapingNode {
            id: "node-1".to_string(),
            address: "10.0.0.1".to_string(),
            port: 8080,
            status: NodeStatus::Online,
            region: "us-east".to_string(),
            capabilities: vec!["twitter".to_string()],
            last_heartbeat: None,
            total_requests: 0,
            success_rate: 1.0,
            avg_latency_ms: 50,
        };
        assert_eq!(node.id, "node-1");
    }

    #[test]
    fn test_distributed_coordinator_register() {
        let mut coord = DistributedScrapingCoordinator::new(DistributedConfig::default());
        let node = ScrapingNode {
            id: "node-1".to_string(),
            address: "10.0.0.1".to_string(),
            port: 8080,
            status: NodeStatus::Online,
            region: "us-east".to_string(),
            capabilities: vec![],
            last_heartbeat: None,
            total_requests: 0,
            success_rate: 1.0,
            avg_latency_ms: 50,
        };
        coord.register_node(node).unwrap();
        assert_eq!(coord.online_node_count(), 1);
    }

    #[test]
    fn test_distributed_coordinator_task() {
        let mut coord = DistributedScrapingCoordinator::new(DistributedConfig::default());
        let task_id = coord.create_task(Platform::GitHub, SearchParams::default());
        assert!(!task_id.is_empty());
        assert_eq!(coord.pending_task_count(), 1);
    }

    #[test]
    fn test_distributed_stats() {
        let coord = DistributedScrapingCoordinator::new(DistributedConfig::default());
        let stats = coord.stats();
        assert_eq!(stats.total_nodes, 0);
    }

    #[test]
    fn test_load_balance_strategy() {
        let _ = LoadBalanceStrategy::RoundRobin;
        let _ = LoadBalanceStrategy::LeastBusy;
        let _ = LoadBalanceStrategy::LatencyBased;
        let _ = LoadBalanceStrategy::RegionBased;
        let _ = LoadBalanceStrategy::Random;
    }
}
