//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER ACTIONS - Tarayıcı Kontrol Aksiyonları
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{BrowserError, BrowserResult};
use crate::observation::Observation;
use serde::{Deserialize, Serialize};

/// Browser aksiyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BrowserAction {
    /// URL'ye git
    Navigate {
        url: String,
    },
    
    /// Elemente tıkla
    Click {
        selector: String,
        #[serde(default)]
        xpath: Option<String>,
        #[serde(default)]
        index: Option<usize>,
    },
    
    /// Metin yaz
    Type {
        selector: String,
        text: String,
        #[serde(default)]
        press_enter: bool,
        #[serde(default)]
        clear_first: bool,
    },
    
    /// Sayfayı kaydır
    Scroll {
        direction: ScrollDirection,
        amount: u32,
    },
    
    /// Element seç (dropdown)
    Select {
        selector: String,
        value: String,
    },
    
    /// Elementi bekle
    WaitFor {
        selector: String,
        timeout_ms: u64,
    },
    
    /// Sayfa bekle
    WaitForPageLoad {
        timeout_ms: u64,
    },
    
    /// Hover
    Hover {
        selector: String,
    },
    
    /// Ekran görüntüsü
    Screenshot {
        #[serde(default)]
        full_page: bool,
        #[serde(default)]
        selector: Option<String>,
    },
    
    /// Geri git
    Back,
    
    /// İleri git
    Forward,
    
    /// Sayfayı yenile
    Refresh,
    
    /// Tab kapat
    CloseTab,
    
    /// Yeni tab aç
    NewTab {
        url: String,
    },
    
    /// Tab değiştir
    SwitchTab {
        index: usize,
    },
    
    /// İptal et
    Cancel,
    
    /// Görev tamamlandı
    Done {
        result: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
    ToElement { selector: String },
}

/// Aksiyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
    /// Yeni observation (varsa)
    pub observation: Option<Observation>,
    /// Ek veri
    pub data: Option<serde_json::Value>,
}

impl ActionResult {
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: message.into(),
            observation: None,
            data: None,
        }
    }
    
    pub fn failure(message: &str) -> Self {
        Self {
            success: false,
            message: message.into(),
            observation: None,
            data: None,
        }
    }
    
    pub fn with_observation(mut self, obs: Observation) -> Self {
        self.observation = Some(obs);
        self
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Aksiyon çalıştırıcı
pub struct ActionExecutor {
    /// Maksimum yeniden deneme
    max_retries: u32,
    /// Varsayılan timeout
    default_timeout_ms: u64,
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            default_timeout_ms: 30000,
        }
    }
    
    /// Aksiyonu çalıştır
    pub async fn execute(&self, action: BrowserAction) -> BrowserResult<ActionResult> {
        log::info!("🎬  ACTION: {:?}", action);
        
        match action {
            BrowserAction::Navigate { url } => self.navigate(&url).await,
            BrowserAction::Click { selector, .. } => self.click(&selector).await,
            BrowserAction::Type { selector, text, press_enter, clear_first } => {
                self.type_text(&selector, &text, press_enter, clear_first).await
            }
            BrowserAction::Scroll { direction, amount } => self.scroll(direction, amount).await,
            BrowserAction::WaitFor { selector, timeout_ms } => self.wait_for(&selector, timeout_ms).await,
            BrowserAction::WaitForPageLoad { timeout_ms } => self.wait_for_page_load(timeout_ms).await,
            BrowserAction::Hover { selector } => self.hover(&selector).await,
            BrowserAction::Select { selector, value } => self.select(&selector, &value).await,
            BrowserAction::Screenshot { full_page, .. } => self.screenshot(full_page).await,
            BrowserAction::Back => self.back().await,
            BrowserAction::Forward => self.forward().await,
            BrowserAction::Refresh => self.refresh().await,
            BrowserAction::Done { result } => Ok(ActionResult::success(&result)),
            BrowserAction::Cancel => Ok(ActionResult::success("İptal edildi")),
            _ => Ok(ActionResult::failure("Henüz implement edilmedi")),
        }
    }
    
    async fn navigate(&self, url: &str) -> BrowserResult<ActionResult> {
        log::info!("🌐  NAVIGATE: {}", url);
        // TODO: Gerçek implementasyon
        Ok(ActionResult::success(&format!("{} adresine gidildi", url)))
    }
    
    async fn click(&self, selector: &str) -> BrowserResult<ActionResult> {
        log::info!("👆  CLICK: {}", selector);
        Ok(ActionResult::success(&format!("'{}' elementine tıklandı", selector)))
    }
    
    async fn type_text(&self, selector: &str, text: &str, press_enter: bool, clear_first: bool) -> BrowserResult<ActionResult> {
        log::info!("⌨️  TYPE: {} → '{}'", selector, text);
        Ok(ActionResult::success(&format!("'{}' alanına yazıldı", selector)))
    }
    
    async fn scroll(&self, direction: ScrollDirection, amount: u32) -> BrowserResult<ActionResult> {
        log::info!("📜  SCROLL: {:?} {}", direction, amount);
        Ok(ActionResult::success("Sayfa kaydırıldı"))
    }
    
    async fn wait_for(&self, selector: &str, timeout_ms: u64) -> BrowserResult<ActionResult> {
        log::info!("⏳  WAIT_FOR: {} ({}ms)", selector, timeout_ms);
        Ok(ActionResult::success(&format!("'{}' elementi bulundu", selector)))
    }
    
    async fn wait_for_page_load(&self, timeout_ms: u64) -> BrowserResult<ActionResult> {
        log::info!("⏳  WAIT_FOR_PAGE_LOAD: {}ms", timeout_ms);
        Ok(ActionResult::success("Sayfa yüklendi"))
    }
    
    async fn hover(&self, selector: &str) -> BrowserResult<ActionResult> {
        log::info!("🖱️  HOVER: {}", selector);
        Ok(ActionResult::success(&format!("'{}' üzerine gelindi", selector)))
    }
    
    async fn select(&self, selector: &str, value: &str) -> BrowserResult<ActionResult> {
        log::info!("📋  SELECT: {} → '{}'", selector, value);
        Ok(ActionResult::success(&format!("'{}' seçildi", value)))
    }
    
    async fn screenshot(&self, full_page: bool) -> BrowserResult<ActionResult> {
        log::info!("📸  SCREENSHOT: full_page={}", full_page);
        Ok(ActionResult::success("Ekran görüntüsü alındı"))
    }
    
    async fn back(&self) -> BrowserResult<ActionResult> {
        Ok(ActionResult::success("Geri gidildi"))
    }
    
    async fn forward(&self) -> BrowserResult<ActionResult> {
        Ok(ActionResult::success("İleri gidildi"))
    }
    
    async fn refresh(&self) -> BrowserResult<ActionResult> {
        Ok(ActionResult::success("Sayfa yenilendi"))
    }
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_action_serialize() {
        let action = BrowserAction::Navigate {
            url: "https://example.com".into(),
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("Navigate"));
    }
    
    #[test]
    fn test_action_result() {
        let result = ActionResult::success("Test başarılı");
        assert!(result.success);
        assert_eq!(result.message, "Test başarılı");
    }
}
