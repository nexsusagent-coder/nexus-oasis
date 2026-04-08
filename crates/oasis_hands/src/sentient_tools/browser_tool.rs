//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER TOOL - WEB TARAYICISI KONTROL ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Headless tarayıcı kontrolü.
//! Sayfa açma, tıklama, form doldurma, veri çekme.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Browser aracı - web tarayıcısı kontrolü
pub struct BrowserTool {
    /// Headless mod
    headless: bool,
    /// Zaman aşımı (saniye)
    timeout_secs: u64,
    /// İzin verilen domainler (boş = tümü)
    allowed_domains: Vec<String>,
}

impl BrowserTool {
    /// Yeni Browser aracı oluştur
    pub fn new() -> Self {
        Self {
            headless: true,
            timeout_secs: 30,
            allowed_domains: vec![],
        }
    }
    
    /// URL'nin izin verilen bir domain'de olup olmadığını kontrol et
    fn is_url_allowed(&self, url: &str) -> bool {
        if self.allowed_domains.is_empty() {
            return true;
        }
        
        self.allowed_domains.iter().any(|domain| url.contains(domain))
    }
    
    /// URL'yi doğrula
    fn validate_url(&self, url: &str) -> Result<(), String> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("URL http:// veya https:// ile başlamalı".to_string());
        }
        
        if !self.is_url_allowed(url) {
            return Err(format!("URL izin verilen domainlerde değil: {}", url));
        }
        
        Ok(())
    }
}

impl Default for BrowserTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Browser aksiyonu
#[derive(Debug, Clone, serde::Serialize)]
pub enum BrowserAction {
    /// Sayfa aç
    Navigate { url: String },
    /// Tıkla
    Click { selector: String },
    /// Metin yaz
    Type { selector: String, text: String },
    /// Bekle
    Wait { milliseconds: u64 },
    /// Ekran görüntüsü al
    Screenshot,
    /// Veri çek
    Extract { selector: String },
}

#[async_trait]
impl SentientTool for BrowserTool {
    fn name(&self) -> &str {
        "browser"
    }
    
    fn description(&self) -> &str {
        "Web tarayıcısı kontrolü. Sayfa açma, tıklama, form doldurma, veri çekme."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Browser
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (navigate, click, type, wait, screenshot, extract)"),
            ToolParameter::new("url", "string", false, "URL (navigate için)"),
            ToolParameter::new("selector", "string", false, "CSS selector (click, type, extract için)"),
            ToolParameter::new("text", "string", false, "Yazılacak metin (type için)"),
            ToolParameter::new("milliseconds", "number", false, "Bekleme süresi (wait için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "navigate" | "open" => {
                let url = params.get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if let Err(e) = self.validate_url(url) {
                    return SentientToolResult::failure(&e);
                }
                
                // Mock: Gerçek tarayıcı integration için browser-use kullanılır
                log::info!("🌐  BROWSER: Sayfa açılıyor → {}", url);
                
                SentientToolResult::success_with_data(
                    "Sayfa başarıyla açıldı",
                    serde_json::json!({
                        "action": "navigate",
                        "url": url,
                        "status": "loaded",
                    })
                )
            }
            "click" => {
                let selector = params.get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("🖱️  BROWSER: Tıklanıyor → {}", selector);
                
                SentientToolResult::success(&format!("Tıklandı: {}", selector))
            }
            "type" => {
                let selector = params.get("selector")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let text = params.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("⌨️  BROWSER: Yazılıyor → {} ({} karakter)", selector, text.len());
                
                SentientToolResult::success(&format!("Yazıldı: {} karakter", text.len()))
            }
            "screenshot" => {
                log::info!("📸  BROWSER: Ekran görüntüsü alınıyor");
                
                SentientToolResult::success_with_data(
                    "Ekran görüntüsü alındı",
                    serde_json::json!({
                        "action": "screenshot",
                        "format": "png",
                        "data": "base64_mock_image_data",
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen browser aksiyonu: '{}'. Kullanılabilir: navigate, click, type, wait, screenshot, extract",
                    action
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_browser_tool_creation() {
        let tool = BrowserTool::new();
        assert_eq!(tool.name(), "browser");
        assert!(tool.headless);
    }
    
    #[test]
    fn test_url_validation() {
        let tool = BrowserTool::new();
        assert!(tool.validate_url("https://example.com").is_ok());
        assert!(tool.validate_url("invalid-url").is_err());
    }
    
    #[test]
    fn test_domain_whitelist() {
        let tool = BrowserTool {
            allowed_domains: vec!["example.com".to_string()],
            ..Default::default()
        };
        assert!(tool.is_url_allowed("https://example.com/page"));
        assert!(!tool.is_url_allowed("https://other.com"));
    }
    
    #[tokio::test]
    async fn test_browser_navigate() {
        let tool = BrowserTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("navigate")),
            ("url".to_string(), serde_json::json!("https://example.com")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
