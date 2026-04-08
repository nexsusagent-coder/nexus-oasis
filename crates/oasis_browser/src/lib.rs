//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS BROWSER - L4: EXECUTION KATMANI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Browser-Use aracının SENTIENT'ya tam asimilasyonu.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  L1 SOVEREIGN ANAYASASI:
//!  ───────────────────────
//!  ✓ Tarayıcı DIŞ WEB'de otonom gezinir
//!  ✓ YEREL DOSYA SİSTEMİNE ERİŞİM YASAKTIR (Sovereign Sandbox)
//!  ✓ DOM → Observation (LLM-optimized) dönüşümü
//!  ✓ V-GATE üzerinden şifreli LLM iletişimi
//!  ✓ Tüm hatalar SENTIENT diline çevrilir
//! ═──────────────────────────────────────────────────────────────────────────────
//!
//! MİMARİ:
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         OASIS BROWSER                                        │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    SOVEREIGN SANDBOX (L1)                           │   │
//! │  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐           │   │
//! │  │  │ FileSystem    │  │ Network       │  │ Process       │           │   │
//! │  │  │   BLOCKED ❌  │  │   Allowed ✅  │  │   Limited ⚠️ │           │   │
//! │  │  └───────────────┘  └───────────────┘  └───────────────┘           │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    BROWSER AGENT                                     │   │
//! │  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐        │   │
//! │  │  │ Navigate  │  │ Click     │  │ Type      │  │ Extract   │        │   │
//! │  │  └───────────┘  └───────────┘  └───────────┘  └───────────┘        │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    OBSERVATION PIPELINE                              │   │
//! │  │  DOM → Pruning → Compression → Structuring → LLM-Ready Format       │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                    │                                        │
//! │                                    ▼                                        │
//! │  ┌─────────────────────────────────────────────────────────────────────┐   │
//! │  │                    V-GATE (L2)                                       │   │
//! │  │  LLM Request → Guardrails → Encrypted Channel → LLM Response       │   │
//! │  └─────────────────────────────────────────────────────────────────────┘   │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

pub mod error;
pub mod sovereign;
pub mod observation;
pub mod actions;
pub mod agent;
pub mod vgate;
pub mod session;
pub mod stealth;
pub mod tools;
pub mod profile;

// Human-Mimicry & Stealth Modülleri
pub mod recap;
pub mod proxy;
#[cfg(feature = "lightpanda-ffi")]
pub mod lightpanda_ffi;

// Re-exports
pub use error::{BrowserError, BrowserResult};
pub use sovereign::{SovereignSandbox, SandboxPolicy, FileAccess, NetworkAccess};
pub use observation::{Observation, ObservationPipeline, DOMElement, PageState};
pub use actions::{BrowserAction, ActionExecutor, ActionResult};
pub use agent::{BrowserAgent, AgentConfig, AgentState, AgentTask};
pub use vgate::BrowserVGate;
pub use session::{BrowserSession, SessionConfig, SessionStats};
pub use stealth::{StealthEngine, StealthConfig, Fingerprint};
pub use recap::{ReCapEngine, ReCapConfig, CaptchaType, CaptchaSolution};
pub use proxy::{ProxyPool, Proxy, ProxyConfig, ProxyType, ProxyPoolStats};
pub use profile::{
    ProfileManager, ProfileError, ProfileResult,
    BrowserProfile, ProfileMetadata, ProfileStatus,
    CookieData, SameSite, PREDEFINED_PROFILES,
};
#[cfg(feature = "lightpanda-ffi")]
pub use lightpanda_ffi::{LightpandaFFI, LightpandaPage, FFIError, FFIResult};

// New Tools Re-exports
pub use tools::{BrowserTool, BrowserToolInput, BrowserToolOutput, BrowserMetadata, ToolContext, Tool};

use sentient_common::error::SENTIENTResult;

/// Browser-Use asimilasyon sürümü
pub const OASIS_BROWSER_VERSION: &str = "0.1.0-sentient";

/// Varsayılan browser user-agent
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36";

/// Maksimum sayfa yükleme süresi (ms)
pub const DEFAULT_PAGE_TIMEOUT_MS: u64 = 30000;

/// Maksimum observation boyutu (token sayısı yaklaşık)
pub const MAX_OBSERVATION_TOKENS: usize = 8000;

// ───────────────────────────────────────────────────────────────────────────────
//  OASIS BROWSER MANAGER
// ─────────────────────────────────────────────────────────────────────────────--

/// Oasis Browser yöneticisi - Ana giriş noktası
pub struct OasisBrowser {
    /// Sovereign sandbox
    sandbox: sovereign::SovereignSandbox,
    /// Browser agent
    agent: Option<agent::BrowserAgent>,
    /// V-GATE köprüsü
    vgate: vgate::BrowserVGate,
    /// Yapılandırma
    config: BrowserConfig,
    /// Başlatıldı mı?
    initialized: bool,
}

/// Browser yapılandırması
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Headless mod
    pub headless: bool,
    /// V-GATE URL
    pub vgate_url: String,
    /// Sayfa zaman aşımı (ms)
    pub page_timeout_ms: u64,
    /// User-Agent
    pub user_agent: String,
    /// Stealth modu aktif mi?
    pub stealth_mode: bool,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Proxy URL (opsiyonel)
    pub proxy_url: Option<String>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            headless: true,
            vgate_url: "http://127.0.0.1:1071".into(),
            page_timeout_ms: DEFAULT_PAGE_TIMEOUT_MS,
            user_agent: DEFAULT_USER_AGENT.into(),
            stealth_mode: true,
            max_retries: 3,
            proxy_url: None,
        }
    }
}

impl OasisBrowser {
    /// Yeni Oasis Browser oluştur
    pub fn new(config: BrowserConfig) -> Self {
        log::info!("🌐  OASIS-BROWSER: L4 EXECUTION katmanı başlatılıyor...");
        
        let sandbox = sovereign::SovereignSandbox::new(sovereign::SandboxPolicy::sovereign());
        let vgate = vgate::BrowserVGate::new(&config.vgate_url);
        
        log::info!("🌐  OASIS-BROWSER: Sovereign Sandbox aktif - Dosya sistemi erişimi ENGELLENDİ");
        
        Self {
            sandbox,
            agent: None,
            vgate,
            config,
            initialized: false,
        }
    }
    
    /// Browser'ı başlat
    pub async fn initialize(&mut self) -> BrowserResult<()> {
        if self.initialized {
            return Ok(());
        }
        
        log::info!("🌐  OASIS-BROWSER: Tarayıcı başlatılıyor...");
        
        // Sovereign sandbox'i aktive et
        self.sandbox.activate()?;
        
        // Browser agent oluştur
        let agent_config = agent::AgentConfig {
            headless: self.config.headless,
            user_agent: self.config.user_agent.clone(),
            page_timeout_ms: self.config.page_timeout_ms,
            stealth_mode: self.config.stealth_mode,
            proxy_url: self.config.proxy_url.clone(),
            max_retries: self.config.max_retries,
            max_iterations: 50,
        };
        
        self.agent = Some(agent::BrowserAgent::new(agent_config, self.vgate.clone()).await?);
        self.initialized = true;
        
        log::info!("✅  OASIS-BROWSER: Tarayıcı hazır - Sadece DIŞ WEB erişimi aktif");
        Ok(())
    }
    
    /// Görev çalıştır (ana giriş noktası)
    pub async fn execute_task(&mut self, task: &str) -> BrowserResult<observation::Observation> {
        self.ensure_initialized()?;
        
        log::info!("🌐  OASIS-BROWSER: Görev alındı → {}", task.chars().take(50).collect::<String>());
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            BrowserError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        // Sovereign kontrolü
        self.sandbox.check_network_access("execute_task")?;
        
        agent.execute_task(task).await
    }
    
    /// URL'ye git
    pub async fn navigate(&mut self, url: &str) -> BrowserResult<observation::Observation> {
        self.ensure_initialized()?;
        
        // URL doğrulama - sadece dış web
        self.sandbox.validate_url(url)?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            BrowserError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.navigate(url).await
    }
    
    /// DOM'dan observation çıkar
    pub async fn observe(&mut self) -> BrowserResult<observation::Observation> {
        self.ensure_initialized()?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            BrowserError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.observe().await
    }
    
    /// Aksiyon çalıştır
    pub async fn act(&mut self, action: actions::BrowserAction) -> BrowserResult<actions::ActionResult> {
        self.ensure_initialized()?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            BrowserError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.act(action).await
    }
    
    /// Ekran görüntüsü al
    pub async fn screenshot(&mut self, full_page: bool) -> BrowserResult<String> {
        self.ensure_initialized()?;
        
        let agent = self.agent.as_mut().ok_or_else(|| {
            BrowserError::NotInitialized("Agent başlatılmadı".into())
        })?;
        
        agent.screenshot(full_page).await
    }
    
    /// Tarayıcıyı kapat
    pub async fn close(&mut self) -> BrowserResult<()> {
        if let Some(ref mut agent) = self.agent {
            agent.close().await?;
        }
        
        self.sandbox.deactivate()?;
        self.initialized = false;
        
        log::info!("🌐  OASIS-BROWSER: Tarayıcı kapatıldı");
        Ok(())
    }
    
    /// Oturum istatistikleri
    pub fn stats(&self) -> session::SessionStats {
        if let Some(ref agent) = self.agent {
            agent.stats()
        } else {
            session::SessionStats::default()
        }
    }
    
    /// Başlatıldı mı?
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Sandbox politikasını getir
    pub fn sandbox_policy(&self) -> &sovereign::SandboxPolicy {
        self.sandbox.policy()
    }
    
    // ─── Yardımcı Metodlar ───
    
    fn ensure_initialized(&self) -> BrowserResult<()> {
        if !self.initialized {
            Err(BrowserError::NotInitialized(
                "Oasis Browser başlatılmadı. Önce initialize() çağırın.".into()
            ))
        } else {
            Ok(())
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = BrowserConfig::default();
        assert!(config.headless);
        assert!(config.stealth_mode);
        assert_eq!(config.max_retries, 3);
    }
    
    #[test]
    fn test_sovereign_sandbox_creation() {
        let sandbox = sovereign::SovereignSandbox::new(sovereign::SandboxPolicy::sovereign());
        assert!(sandbox.policy().file_access == sovereign::FileAccess::Blocked);
    }
    
    #[test]
    fn test_oasis_browser_creation() {
        let browser = OasisBrowser::new(BrowserConfig::default());
        assert!(!browser.is_initialized());
    }
    
    #[test]
    fn test_url_validation() {
        let sandbox = sovereign::SovereignSandbox::new(sovereign::SandboxPolicy::sovereign());
        
        // Geçerli URL
        assert!(sandbox.validate_url("https://example.com").is_ok());
        
        // Geçersiz URL (file://)
        assert!(sandbox.validate_url("file:///etc/passwd").is_err());
        
        // Geçersiz URL (localhost)
        assert!(sandbox.validate_url("http://localhost:8080").is_err());
    }
}
