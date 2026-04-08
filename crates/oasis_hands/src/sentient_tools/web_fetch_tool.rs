//! ═══════════════════════════════════════════════════════════════════════════════
//!  WEB FETCH TOOL
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct WebFetchTool {
    policy: SovereignPolicy,
}

impl WebFetchTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self { policy }
    }
    
    pub fn default_tool() -> Self {
        Self::new(SovereignPolicy::developer())
    }
    
    pub async fn fetch(&self, url: &str, to_markdown: bool) -> crate::error::HandsResult<serde_json::Value> {
        // Ağ erişimi kontrolü
        if self.policy.network == crate::sovereign::NetworkPolicy::Blocked {
            return Ok(serde_json::json!({
                "content": "",
                "content_type": "",
                "size": 0,
                "is_error": true,
                "error_message": "Ağ erişimi bu politikada engelli"
            }));
        }
        
        // URL doğrulama
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Ok(serde_json::json!({
                "content": "",
                "is_error": true,
                "error_message": "Geçersiz URL (http/https gerekli)"
            }));
        }
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("SENTIENT/1.0 WebFetch")
            .build()
            .map_err(|e| format!("HTTP client hatası: {}", e))?;
        
        match client.get(url).send().await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Ok(serde_json::json!({
                        "content": "",
                        "is_error": true,
                        "error_message": format!("HTTP hatası: {}", resp.status())
                    }));
                }
                
                let content_type = resp.headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("text/html")
                    .to_string();
                
                let bytes = resp.bytes().await.unwrap_or_default();
                let size = bytes.len();
                
                let content = if to_markdown && content_type.contains("text/html") {
                    html_to_markdown(&String::from_utf8_lossy(&bytes))
                } else {
                    String::from_utf8_lossy(&bytes).to_string()
                };
                
                Ok(serde_json::json!({
                    "content": content,
                    "content_type": content_type,
                    "size": size,
                    "is_error": false
                }))
            }
            Err(e) => Ok(serde_json::json!({
                "content": "",
                "is_error": true,
                "error_message": format!("Bağlantı hatası: {}", e)
            }))
        }
    }
}

fn html_to_markdown(html: &str) -> String {
    let mut md = html.to_string();
    
    // Script ve style kaldır
    if let Ok(re) = regex::Regex::new(r"<script[^>]*>.*?</script>") {
        md = re.replace_all(&md, "").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"<style[^>]*>.*?</style>") {
        md = re.replace_all(&md, "").to_string();
    }
    
    // Başlıklar
    if let Ok(re) = regex::Regex::new(r"<h1[^>]*>(.*?)</h1>") {
        md = re.replace_all(&md, "# $1").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"<h2[^>]*>(.*?)</h2>") {
        md = re.replace_all(&md, "## $1").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"<h3[^>]*>(.*?)</h3>") {
        md = re.replace_all(&md, "### $1").to_string();
    }
    
    // Linkler
    if let Ok(re) = regex::Regex::new(r#"<a[^>]*href="([^"]*)"[^>]*>(.*?)</a>"#) {
        md = re.replace_all(&md, "[$2]($1)").to_string();
    }
    
    // Kalın ve italik
    if let Ok(re) = regex::Regex::new(r"<strong[^>]*>(.*?)</strong>") {
        md = re.replace_all(&md, "**$1**").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"<em[^>]*>(.*?)</em>") {
        md = re.replace_all(&md, "*$1*").to_string();
    }
    
    // Satır sonları
    if let Ok(re) = regex::Regex::new(r"<br\s*/?>") {
        md = re.replace_all(&md, "\n").to_string();
    }
    if let Ok(re) = regex::Regex::new(r"</p>") {
        md = re.replace_all(&md, "\n\n").to_string();
    }
    
    // Tüm HTML etiketlerini kaldır
    if let Ok(re) = regex::Regex::new(r"<[^>]+>") {
        md = re.replace_all(&md, "").to_string();
    }
    
    // HTML entities
    md = md.replace("&nbsp;", " ");
    md = md.replace("&amp;", "&");
    md = md.replace("&lt;", "<");
    md = md.replace("&gt;", ">");
    
    // Fazla boşlukları temizle
    if let Ok(re) = regex::Regex::new(r"\n{3,}") {
        md = re.replace_all(&md, "\n\n").to_string();
    }
    
    md.trim().to_string()
}

#[async_trait]
impl SentientTool for WebFetchTool {
    fn name(&self) -> &str { "web_fetch" }
    
    fn description(&self) -> &str {
        "URL'den içerik çeker ve Markdown'a dönüştürür"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Web }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("url", "string", true, "Çekilecek URL"),
            ToolParameter::with_default("to_markdown", "boolean", "Markdown'a dönüştür", serde_json::json!(true)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let url = params.get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let to_markdown = params.get("to_markdown").and_then(|v| v.as_bool()).unwrap_or(true);
        
        match self.fetch(&url, to_markdown).await {
            Ok(value) => SentientToolResult::success_with_data("İçerik çekildi", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for WebFetchTool {
    fn default() -> Self { Self::default_tool() }
}
