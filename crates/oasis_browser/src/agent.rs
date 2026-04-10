//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER AGENT - Otonom Tarayıcı Ajanı
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{BrowserError, BrowserResult};
use crate::sovereign::SovereignSandbox;
use crate::observation::{Observation, ObservationPipeline};
use crate::actions::{BrowserAction, ActionExecutor, ActionResult};
use crate::vgate::BrowserVGate;
use crate::session::{SessionStats, BrowserSession};

/// Browser Agent - Ana otonom birim
pub struct BrowserAgent {
    /// Yapılandırma
    config: AgentConfig,
    /// Sovereign sandbox
    sandbox: SovereignSandbox,
    /// Observation pipeline
    observation_pipeline: ObservationPipeline,
    /// Aksiyon çalıştırıcı
    action_executor: ActionExecutor,
    /// V-GATE köprüsü
    vgate: BrowserVGate,
    /// Oturum
    session: Option<BrowserSession>,
    /// Durum
    state: AgentState,
}

/// Agent yapılandırması
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Headless mod
    pub headless: bool,
    /// User-Agent
    pub user_agent: String,
    /// Sayfa timeout (ms)
    pub page_timeout_ms: u64,
    /// Stealth mod
    pub stealth_mode: bool,
    /// Proxy URL
    pub proxy_url: Option<String>,
    /// Maks. deneme
    pub max_retries: u32,
    /// Maks. iterasyon
    pub max_iterations: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            headless: true,
            user_agent: crate::DEFAULT_USER_AGENT.into(),
            page_timeout_ms: 30000,
            stealth_mode: true,
            proxy_url: None,
            max_retries: 3,
            max_iterations: 50,
        }
    }
}

/// Agent durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// Boşta
    Idle,
    /// Başlatılıyor
    Initializing,
    /// Görev yürütüyor
    Executing,
    /// Bekliyor
    Waiting,
    /// Hata
    Error,
    /// Tamamlandı
    Completed,
}

/// Agent görevi
#[derive(Debug, Clone)]
pub struct AgentTask {
    /// Görev ID
    pub id: String,
    /// Görev açıklaması
    pub description: String,
    /// Başlangıç URL (opsiyonel)
    pub start_url: Option<String>,
    /// Maks. adım
    pub max_steps: u32,
}

impl BrowserAgent {
    /// Yeni agent oluştur
    pub async fn new(config: AgentConfig, vgate: BrowserVGate) -> BrowserResult<Self> {
        log::info!("🤖  AGENT: Browser agent oluşturuluyor...");
        
        let sandbox = SovereignSandbox::new(crate::sovereign::SandboxPolicy::sovereign());
        let observation_pipeline = ObservationPipeline::new();
        let action_executor = ActionExecutor::new();
        
        Ok(Self {
            config,
            sandbox,
            observation_pipeline,
            action_executor,
            vgate,
            session: None,
            state: AgentState::Idle,
        })
    }
    
    /// Tarayıcıyı başlat
    pub async fn start(&mut self) -> BrowserResult<()> {
        if self.session.is_some() {
            return Ok(());
        }
        
        self.state = AgentState::Initializing;
        log::info!("🤖  AGENT: Tarayıcı başlatılıyor...");
        
        // Sandbox'ı aktive et
        self.sandbox.activate()?;
        
        // Session oluştur
        self.session = Some(BrowserSession::new(self.config.clone()));
        
        self.state = AgentState::Idle;
        log::info!("✅  AGENT: Tarayıcı hazır");
        Ok(())
    }
    
    /// Görev yürüt (ANA GİRİŞ NOKTASI)
    pub async fn execute_task(&mut self, task: &str) -> BrowserResult<Observation> {
        self.ensure_started().await?;
        
        log::info!("🤖  AGENT: Görev alındı → {}", task.chars().take(50).collect::<String>());
        
        self.state = AgentState::Executing;
        
        let mut iteration = 0;
        let mut last_observation: Option<Observation> = None;
        
        while iteration < self.config.max_iterations {
            iteration += 1;
            
            log::debug!("🤖  AGENT: İterasyon {} / {}", iteration, self.config.max_iterations);
            
            // 1. Mevcut durumu gözlemle
            let observation = self.observe().await?;
            last_observation = Some(observation.clone());
            
            // 2. V-GATE üzerinden aksiyon kararı al
            let action = self.vgate.get_next_action(&observation, task).await?;
            
            log::info!("🤖  AGENT: Aksiyon → {:?}", action);
            
            // 3. Aksiyonu çalıştır
            match action {
                BrowserAction::Done { result } => {
                    log::info!("🤖  AGENT: Görev tamamlandı → {}", result);
                    self.state = AgentState::Completed;
                    break;
                }
                BrowserAction::Cancel => {
                    log::info!("🤖  AGENT: Görev iptal edildi");
                    self.state = AgentState::Idle;
                    break;
                }
                _ => {
                    let result = self.act(action).await?;
                    if !result.success {
                        log::warn!("⚠️  AGENT: Aksiyon başarısız → {}", result.message);
                    }
                }
            }
        }
        
        if iteration >= self.config.max_iterations {
            log::warn!("⚠️  AGENT: Maksimum iterasyon aşıldı");
            self.state = AgentState::Error;
        }
        
        last_observation.ok_or_else(|| BrowserError::Other("Observation üretilemedi".into()))
    }
    
    /// URL'ye git
    pub async fn navigate(&mut self, url: &str) -> BrowserResult<Observation> {
        self.ensure_started().await?;
        
        // Sovereign URL doğrulama
        self.sandbox.validate_url(url)?;
        self.sandbox.check_network_access("navigate")?;
        
        log::info!("🧭  AGENT: Navigating to {}", url);
        
        let action = BrowserAction::Navigate { url: url.into() };
        let _result = self.action_executor.execute(action).await?;
        
        self.observe().await
    }
    
    /// Sayfayı gözlemle
    pub async fn observe(&mut self) -> BrowserResult<Observation> {
        self.ensure_started().await?;
        
        // DOM extraction - headless browser integration
        let html = self.extract_dom().await?;
        let url = self.current_url().await.unwrap_or_default();
        let observation = self.observation_pipeline.process(&html, &url)?;
        
        Ok(observation)
    }
    
    /// DOM extraction
    async fn extract_dom(&self) -> BrowserResult<String> {
        // Mock DOM for development - real impl would use headless browser
        Ok("<html><body>Mock DOM</body></html>".into())
    }
    
    /// Aksiyon çalıştır
    pub async fn act(&mut self, action: BrowserAction) -> BrowserResult<ActionResult> {
        self.ensure_started().await?;
        
        // Sovereign kontrolü (URL'ler için)
        if let BrowserAction::Navigate { ref url } = action {
            self.sandbox.validate_url(url)?;
        }
        
        self.action_executor.execute(action).await
    }
    
    /// Ekran görüntüsü al
    pub async fn screenshot(&mut self, _full_page: bool) -> BrowserResult<String> {
        self.ensure_started().await?;
        
        // Base64 encoded placeholder - real impl uses headless browser screenshot
        // Production: Use chromiumoxide or headless_chrome for actual screenshots
        Ok("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".into())
    }
    
    /// Tarayıcıyı kapat
    pub async fn close(&mut self) -> BrowserResult<()> {
        if let Some(ref mut session) = self.session {
            session.close();
        }
        
        self.sandbox.deactivate()?;
        self.session = None;
        self.state = AgentState::Idle;
        
        log::info!("🤖  AGENT: Tarayıcı kapatıldı");
        Ok(())
    }
    
    /// Oturum istatistikleri
    pub fn stats(&self) -> SessionStats {
        self.session.as_ref()
            .map(|s| s.stats())
            .unwrap_or_default()
    }
    
    /// Mevcut URL
    async fn current_url(&self) -> BrowserResult<String> {
        // Return tracked URL from session state
        // Production: Get from actual browser session
        Ok("about:blank".into())
    }
    
    /// Başlatıldı mı kontrol et
    async fn ensure_started(&self) -> BrowserResult<()> {
        if self.session.is_none() {
            Err(BrowserError::NotInitialized(
                "Agent başlatılmadı. Önce start() çağırın.".into()
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert!(config.headless);
        assert!(config.stealth_mode);
        assert_eq!(config.max_iterations, 50);
    }
    
    #[test]
    fn test_agent_task_creation() {
        let task = AgentTask {
            id: "test-1".into(),
            description: "Test görevi".into(),
            start_url: Some("https://example.com".into()),
            max_steps: 10,
        };
        
        assert_eq!(task.max_steps, 10);
    }
}
