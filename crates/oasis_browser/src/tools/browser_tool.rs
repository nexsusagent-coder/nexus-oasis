//! Browser Tool - Lightpanda & browser-use pattern'inden adapte
//! SENTIENT oasis-browser crate için

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::{info, warn, debug};

use crate::tools::{Tool, ToolError, ToolResult, ToolContext};

/// Browser işlemleri için input şeması
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserToolInput {
    /// İşlem türü: fetch, screenshot, click, type, scroll
    pub action: String,
    /// Hedef URL (fetch, navigate için)
    pub url: Option<String>,
    /// Seçici (click, type için)
    pub selector: Option<String>,
    /// Yazılacak metin (type için)
    pub text: Option<String>,
    /// Ekran görüntüsü formatı
    pub format: Option<String>,
}

/// Browser işlem sonucu
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserToolOutput {
    pub success: bool,
    pub content: Option<String>,
    pub screenshot: Option<String>, // Base64 encoded
    pub error: Option<String>,
    pub metadata: Option<BrowserMetadata>,
}

/// Sayfa metadata'sı
#[derive(Debug, Serialize, Deserialize)]
pub struct BrowserMetadata {
    pub url: String,
    pub title: Option<String>,
    pub status_code: u16,
    pub content_type: Option<String>,
    pub load_time_ms: u64,
}

/// Browser Tool - Web sayfası işlemleri
pub struct BrowserTool {
    client: Client,
    cdp_endpoint: String,
}

impl BrowserTool {
    pub fn new() -> Self {
        let cdp_endpoint = std::env::var("LIGHTPANDA_CDP_URL")
            .unwrap_or_else(|_| "ws://localhost:9222".to_string());
        
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("SENTIENT/1.0 (Oasis Browser)")
                .build()
                .expect("operation failed"),
            cdp_endpoint,
        }
    }
}

#[async_trait]
impl Tool for BrowserTool {
    type Input = BrowserToolInput;
    type Output = BrowserToolOutput;
    
    fn name(&self) -> &'static str {
        "browser_tool"
    }
    
    fn description(&self) -> &'static str {
        "Web sayfası işlemleri: fetch (HTML/Markdown), screenshot, navigate. Lightpanda backend."
    }
    
    fn is_read_only(&self, input: &Self::Input) -> bool {
        matches!(input.action.as_str(), "fetch" | "screenshot")
    }
    
    async fn execute(
        &self,
        input: Self::Input,
        _context: &ToolContext,
    ) -> ToolResult<Self::Output> {
        match input.action.as_str() {
            "fetch" => self.fetch_url(&input).await,
            "screenshot" => self.take_screenshot(&input).await,
            "markdown" => self.fetch_markdown(&input).await,
            _ => Err(ToolError::InvalidInput(format!(
                "Bilinmeyen browser aksiyonu: {}", input.action
            ))),
        }
    }
}

impl BrowserTool {
    /// URL'den HTML çek
    async fn fetch_url(&self, input: &BrowserToolInput) -> ToolResult<BrowserToolOutput> {
        let url = input.url.as_ref().ok_or_else(|| {
            ToolError::InvalidInput("URL gerekli".to_string())
        })?;
        
        // SOVEREIGN.md kontrolü - yasaklı domain'ler
        let blocked_domains = ["localhost", "127.0.0.1", "0.0.0.0", "internal.", "private."];
        for blocked in blocked_domains {
            if url.contains(blocked) {
                warn!("🔒 SOVEREIGN: Engellenen domain: {}", url);
                return Err(ToolError::PermissionDenied(
                    format!("Domain '{}' SOVEREIGN.md tarafından engellendi", blocked)
                ));
            }
        }
        
        info!("🌐 Fetching: {}", url);
        let start = std::time::Instant::now();
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;
        
        let status = response.status().as_u16();
        let content_type = response.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        
        let content = response.text().await
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;
        
        debug!("✅ Fetched {} bytes in {}ms", content.len(), start.elapsed().as_millis());
        
        Ok(BrowserToolOutput {
            success: status >= 200 && status < 300,
            content: Some(content),
            screenshot: None,
            error: None,
            metadata: Some(BrowserMetadata {
                url: url.clone(),
                title: None,
                status_code: status,
                content_type,
                load_time_ms: start.elapsed().as_millis() as u64,
            }),
        })
    }
    
    /// URL'den Markdown formatında çek
    async fn fetch_markdown(&self, input: &BrowserToolInput) -> ToolResult<BrowserToolOutput> {
        let mut result = self.fetch_url(input).await?;
        
        if let Some(html) = &result.content {
            // HTML'i Markdown'a çevir (basit implementation)
            let markdown = html_to_markdown(html);
            result.content = Some(markdown);
        }
        
        Ok(result)
    }
    
    /// Ekran görüntüsü al (CDP üzerinden)
    async fn take_screenshot(&self, input: &BrowserToolInput) -> ToolResult<BrowserToolOutput> {
        // CDP (Chrome DevTools Protocol) ile screenshot
        // Lightpanda WebSocket bağlantısı
        
        info!("📸 Screenshot request for: {:?}", input.url);
        
        // Basit implementation - gerçek CDP entegrasyonu için
        // websocket bağlantısı gerekiyor
        
        Ok(BrowserToolOutput {
            success: false,
            content: None,
            screenshot: None,
            error: Some("CDP screenshot henüz implement edilmedi".to_string()),
            metadata: None,
        })
    }
}

/// Basit HTML'den Markdown dönüşümü
fn html_to_markdown(html: &str) -> String {
    // Basit implementation - production'da proper parser kullanılmalı
    let mut md = html.to_string();
    
    // Remove script and style
    md = regex::Regex::new(r"<script[^>]*>.*?</script>")
        .expect("operation failed")
        .replace_all(&md, "").to_string();
    md = regex::Regex::new(r"<style[^>]*>.*?</style>")
        .expect("operation failed")
        .replace_all(&md, "").to_string();
    
    // Headers
    md = regex::Regex::new(r"<h1[^>]*>(.*?)</h1>")
        .expect("operation failed")
        .replace_all(&md, "# $1\n").to_string();
    md = regex::Regex::new(r"<h2[^>]*>(.*?)</h2>")
        .expect("operation failed")
        .replace_all(&md, "## $1\n").to_string();
    md = regex::Regex::new(r"<h3[^>]*>(.*?)</h3>")
        .expect("operation failed")
        .replace_all(&md, "### $1\n").to_string();
    
    // Links
    md = regex::Regex::new(r#"<a[^>]*href="([^"]*)"[^>]*>(.*?)</a>"#)
        .expect("operation failed")
        .replace_all(&md, "[$2]($1)").to_string();
    
    // Remove remaining tags
    md = regex::Regex::new(r"<[^>]+>")
        .expect("operation failed")
        .replace_all(&md, "").to_string();
    
    // Clean whitespace
    md = regex::Regex::new(r"\n{3,}")
        .expect("operation failed")
        .replace_all(&md, "\n\n").to_string();
    
    md.trim().to_string()
}

impl Default for BrowserTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_html_to_markdown() {
        let html = r#"<h1>Title</h1><p>Paragraph <a href="http://example.com">link</a></p>"#;
        let md = html_to_markdown(html);
        assert!(md.contains("# Title"));
        assert!(md.contains("[link](http://example.com)"));
    }
}
