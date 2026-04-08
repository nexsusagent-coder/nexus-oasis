//! ═══════════════════════════════════════════════════════════════════════════════
//!  WEB SEARCH TOOL - SearXNG Entegrasyonu
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use crate::sovereign::SovereignPolicy;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct WebSearchTool {
    policy: SovereignPolicy,
    searx_url: String,
}

impl WebSearchTool {
    pub fn new(policy: SovereignPolicy) -> Self {
        Self {
            policy,
            searx_url: "http://localhost:8888".to_string(),
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new(SovereignPolicy::developer())
    }
    
    pub async fn search(&self, query: &str, max_results: usize) -> crate::error::HandsResult<serde_json::Value> {
        // Ağ erişimi kontrolü
        if self.policy.network == crate::sovereign::NetworkPolicy::Blocked {
            return Ok(serde_json::json!({
                "results": [],
                "total": 0,
                "is_error": true,
                "error_message": "Ağ erişimi bu politikada engelli"
            }));
        }
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("HTTP client hatası: {}", e))?;
        
        let response = client
            .get(&format!("{}/search", self.searx_url))
            .query(&[("q", query), ("format", "json")])
            .send()
            .await;
        
        match response {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Ok(serde_json::json!({
                        "results": [],
                        "total": 0,
                        "is_error": true,
                        "error_message": format!("SearXNG hatası: {}", resp.status())
                    }));
                }
                
                let json: serde_json::Value = resp.json().await.unwrap_or(serde_json::json!({}));
                let mut results = Vec::new();
                
                if let Some(results_array) = json.get("results").and_then(|r| r.as_array()) {
                    for item in results_array.iter().take(max_results) {
                        results.push(serde_json::json!({
                            "title": item.get("title").and_then(|t| t.as_str()).unwrap_or(""),
                            "url": item.get("url").and_then(|u| u.as_str()).unwrap_or(""),
                            "snippet": item.get("content").and_then(|c| c.as_str()).unwrap_or("")
                        }));
                    }
                }
                
                Ok(serde_json::json!({
                    "results": results,
                    "total": results.len(),
                    "is_error": false
                }))
            }
            Err(e) => Ok(serde_json::json!({
                "results": [],
                "total": 0,
                "is_error": true,
                "error_message": format!("Bağlantı hatası: {}. SearXNG çalışıyor mu?", e)
            }))
        }
    }
}

#[async_trait]
impl SentientTool for WebSearchTool {
    fn name(&self) -> &str { "web_search" }
    
    fn description(&self) -> &str {
        "Web araması yapar (SearXNG üzerinden)"
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Web }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("query", "string", true, "Arama sorgusu"),
            ToolParameter::with_default("max_results", "integer", "Maksimum sonuç", serde_json::json!(10)),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let query = params.get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let max_results = params.get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        
        match self.search(&query, max_results).await {
            Ok(value) => SentientToolResult::success_with_data("Arama tamamlandı", value),
            Err(e) => SentientToolResult::failure(&format!("Hata: {}", e)),
        }
    }
}

impl Default for WebSearchTool {
    fn default() -> Self { Self::default_tool() }
}
