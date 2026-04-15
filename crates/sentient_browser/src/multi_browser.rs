//! ═══════════════════════════════════════════════════════════════════════════════
//!  Multi-Browser Support
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Support for multiple browser engines:
//! - Chrome/Chromium
//! - Firefox/Gecko
//! - Safari/WebKit
//! - Edge (Chromium-based)
//! - Headless modes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  BROWSER TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Browser type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Chromium,
    Brave,
    Opera,
}

impl BrowserType {
    pub fn engine(&self) -> BrowserEngine {
        match self {
            BrowserType::Chrome | BrowserType::Edge | BrowserType::Chromium | 
            BrowserType::Brave | BrowserType::Opera => BrowserEngine::Blink,
            BrowserType::Firefox => BrowserEngine::Gecko,
            BrowserType::Safari => BrowserEngine::WebKit,
        }
    }
    
    pub fn executable_name(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "chrome",
            BrowserType::Firefox => "firefox",
            BrowserType::Safari => "safari",
            BrowserType::Edge => "msedge",
            BrowserType::Chromium => "chromium",
            BrowserType::Brave => "brave",
            BrowserType::Opera => "opera",
        }
    }
}

/// Browser engine
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BrowserEngine {
    Blink,   // Chrome, Edge, Chromium
    Gecko,   // Firefox
    WebKit,  // Safari
}

/// Browser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Browser type
    pub browser: BrowserType,
    /// Headless mode
    pub headless: bool,
    /// Window size
    pub window_size: (u32, u32),
    /// User agent
    pub user_agent: Option<String>,
    /// Proxy settings
    pub proxy: Option<ProxyConfig>,
    /// Additional arguments
    pub args: Vec<String>,
    /// Extensions to load
    pub extensions: Vec<String>,
    /// Profile directory
    pub profile_dir: Option<String>,
    /// Download directory
    pub download_dir: Option<String>,
    /// Disable images
    pub disable_images: bool,
    /// Disable JavaScript
    pub disable_javascript: bool,
    /// Ignore certificate errors
    pub ignore_cert_errors: bool,
    /// Timeout (ms)
    pub timeout_ms: u64,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser: BrowserType::Chromium,
            headless: true,
            window_size: (1920, 1080),
            user_agent: None,
            proxy: None,
            args: vec![],
            extensions: vec![],
            profile_dir: None,
            download_dir: None,
            disable_images: false,
            disable_javascript: false,
            ignore_cert_errors: false,
            timeout_ms: 30000,
        }
    }
}

/// Proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub proxy_type: ProxyType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProxyType {
    Http,
    Https,
    Socks4,
    Socks5,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BROWSER SESSION
// ═══════════════════════════════════════════════════════════════════════════════

/// Browser session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    /// Session ID
    pub id: String,
    /// Browser type
    pub browser: BrowserType,
    /// Is headless
    pub headless: bool,
    /// WebSocket debugger URL (CDP)
    pub debugger_url: Option<String>,
    /// PID
    pub pid: Option<u32>,
}

/// Page in browser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Page ID
    pub id: String,
    /// URL
    pub url: String,
    /// Title
    pub title: Option<String>,
    /// Session ID
    pub session_id: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BROWSER POOL
// ═══════════════════════════════════════════════════════════════════════════════

/// Browser error
#[derive(Debug, thiserror::Error)]
pub enum BrowserError {
    #[error("Failed to start browser: {0}")]
    StartFailed(String),
    
    #[error("Browser not found: {0:?}")]
    NotFound(BrowserType),
    
    #[error("Page error: {0}")]
    PageError(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Navigation failed: {0}")]
    NavigationFailed(String),
    
    #[error("Session closed")]
    SessionClosed,
}

/// Multi-browser pool
pub struct BrowserPool {
    browsers: HashMap<String, BrowserSession>,
    configs: HashMap<BrowserType, BrowserConfig>,
    max_per_type: usize,
}

impl BrowserPool {
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        configs.insert(BrowserType::Chromium, BrowserConfig::default());
        
        Self {
            browsers: HashMap::new(),
            configs,
            max_per_type: 5,
        }
    }
    
    /// Add browser configuration
    pub fn add_config(&mut self, browser_type: BrowserType, config: BrowserConfig) {
        self.configs.insert(browser_type, config);
    }
    
    /// Launch a browser
    pub async fn launch(&mut self, browser_type: BrowserType) -> Result<BrowserSession, BrowserError> {
        let config = self.configs.get(&browser_type)
            .cloned()
            .unwrap_or_else(|| BrowserConfig {
                browser: browser_type,
                ..Default::default()
            });
        
        // Check limit
        let count = self.browsers.values()
            .filter(|s| s.browser == browser_type)
            .count();
        
        if count >= self.max_per_type {
            return Err(BrowserError::StartFailed(
                format!("Max {} browsers of type {:?} reached", self.max_per_type, browser_type)
            ));
        }
        
        // In production, actually launch browser
        let session = BrowserSession {
            id: uuid::Uuid::new_v4().to_string(),
            browser: browser_type,
            headless: config.headless,
            debugger_url: Some(format!("ws://localhost:9222/devtools/browser/{}", uuid::Uuid::new_v4())),
            pid: Some(12345),
        };
        
        self.browsers.insert(session.id.clone(), session.clone());
        
        Ok(session)
    }
    
    /// Close a browser session
    pub async fn close(&mut self, session_id: &str) -> Result<(), BrowserError> {
        if self.browsers.remove(session_id).is_some() {
            // In production, kill browser process
            Ok(())
        } else {
            Err(BrowserError::SessionClosed)
        }
    }
    
    /// Get browser session
    pub fn get(&self, session_id: &str) -> Option<&BrowserSession> {
        self.browsers.get(session_id)
    }
    
    /// List all sessions
    pub fn list(&self) -> Vec<&BrowserSession> {
        self.browsers.values().collect()
    }
    
    /// Close all sessions
    pub async fn close_all(&mut self) {
        self.browsers.clear();
    }
    
    /// Get available browser types
    pub fn available_browsers(&self) -> Vec<BrowserType> {
        // In production, check actual installed browsers
        vec![
            BrowserType::Chromium,
            BrowserType::Chrome,
            BrowserType::Firefox,
        ]
    }
}

impl Default for BrowserPool {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PAGE ACTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Page actions (CDP/WebDriver commands)
pub struct PageActions;

impl PageActions {
    /// Navigate to URL
    pub async fn navigate(session: &BrowserSession, url: &str) -> Result<Page, BrowserError> {
        // In production, use CDP or WebDriver
        Ok(Page {
            id: uuid::Uuid::new_v4().to_string(),
            url: url.to_string(),
            title: None,
            session_id: session.id.clone(),
        })
    }
    
    /// Take screenshot
    pub async fn screenshot(session: &BrowserSession, page_id: &str) -> Result<Vec<u8>, BrowserError> {
        // In production, capture actual screenshot
        Ok(vec![])
    }
    
    /// Execute JavaScript
    pub async fn execute_js(
        session: &BrowserSession,
        page_id: &str,
        script: &str,
    ) -> Result<serde_json::Value, BrowserError> {
        // In production, execute via CDP
        Ok(serde_json::json!({ "result": "executed" }))
    }
    
    /// Get page content
    pub async fn get_content(session: &BrowserSession, page_id: &str) -> Result<String, BrowserError> {
        // In production, get via CDP
        Ok("<html><body>Mock content</body></html>".to_string())
    }
    
    /// Click element
    pub async fn click(
        session: &BrowserSession,
        page_id: &str,
        selector: &str,
    ) -> Result<(), BrowserError> {
        // In production, use WebDriver or CDP
        Ok(())
    }
    
    /// Type text
    pub async fn type_text(
        session: &BrowserSession,
        page_id: &str,
        selector: &str,
        text: &str,
    ) -> Result<(), BrowserError> {
        Ok(())
    }
    
    /// Wait for selector
    pub async fn wait_for(
        session: &BrowserSession,
        page_id: &str,
        selector: &str,
        timeout_ms: u64,
    ) -> Result<(), BrowserError> {
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STEALTH MODE
// ═══════════════════════════════════════════════════════════════════════════════

/// Stealth configuration for bot detection evasion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Hide webdriver flag
    pub hide_webdriver: bool,
    /// Randomize viewport
    pub random_viewport: bool,
    /// Human-like mouse movements
    pub human_mouse: bool,
    /// Random delays
    pub random_delays: bool,
    /// Fake canvas fingerprint
    pub fake_canvas: bool,
    /// Fake WebGL fingerprint
    pub fake_webgl: bool,
    /// Fake audio context
    pub fake_audio: bool,
    /// Override plugins
    pub override_plugins: bool,
    /// Override languages
    pub override_languages: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            hide_webdriver: true,
            random_viewport: true,
            human_mouse: true,
            random_delays: true,
            fake_canvas: true,
            fake_webgl: true,
            fake_audio: true,
            override_plugins: true,
            override_languages: true,
        }
    }
}

impl BrowserConfig {
    /// Create stealth configuration
    pub fn stealth(browser: BrowserType) -> Self {
        Self {
            browser,
            headless: false, // Stealth works better in non-headless
            args: vec![
                "--disable-blink-features=AutomationControlled".to_string(),
                "--disable-infobars".to_string(),
                "--no-first-run".to_string(),
                "--no-default-browser-check".to_string(),
            ],
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_browser_types() {
        assert_eq!(BrowserType::Chrome.engine(), BrowserEngine::Blink);
        assert_eq!(BrowserType::Firefox.engine(), BrowserEngine::Gecko);
        assert_eq!(BrowserType::Safari.engine(), BrowserEngine::WebKit);
    }
    
    #[tokio::test]
    async fn test_browser_pool() {
        let mut pool = BrowserPool::new();
        
        let session = pool.launch(BrowserType::Chromium).await.unwrap();
        assert_eq!(session.browser, BrowserType::Chromium);
        
        let sessions = pool.list();
        assert_eq!(sessions.len(), 1);
    }
    
    #[test]
    fn test_stealth_config() {
        let config = BrowserConfig::stealth(BrowserType::Chrome);
        assert!(!config.headless);
        assert!(!config.args.is_empty());
    }
}
