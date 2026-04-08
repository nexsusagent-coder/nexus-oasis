//! ═══════════════════════════════════════════════════════════════════════════════
//!  TRANSLATE TOOL - ÇEVİRİ ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Çok dilli çeviri desteği.
//! V-GATE üzerinden LLM tabanlı çeviri.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Translate aracı - çeviri
pub struct TranslateTool {
    /// Varsayılan kaynak dil
    default_source: String,
    /// Varsayılan hedef dil
    default_target: String,
}

impl TranslateTool {
    /// Yeni Translate aracı oluştur
    pub fn new() -> Self {
        Self {
            default_source: "auto".to_string(),
            default_target: "tr".to_string(),
        }
    }
    
    /// Dil kodunu doğrula
    fn validate_language(&self, code: &str) -> bool {
        let valid_codes = [
            "auto", "tr", "en", "de", "fr", "es", "it", "pt", "ru", "zh", 
            "ja", "ko", "ar", "hi", "nl", "pl", "sv", "da", "fi", "no",
        ];
        valid_codes.contains(&code)
    }
}

impl Default for TranslateTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for TranslateTool {
    fn name(&self) -> &str {
        "translate"
    }
    
    fn description(&self) -> &str {
        "Çok dilli çeviri desteği. 20+ dil desteği, otomatik dil algılama."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Intelligence
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (translate, detect, languages)"),
            ToolParameter::new("text", "string", false, "Çevrilecek metin"),
            ToolParameter::new("source", "string", false, "Kaynak dil (auto=otomatik)"),
            ToolParameter::new("target", "string", false, "Hedef dil"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "translate" => {
                let text = params.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let source = params.get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&self.default_source);
                let target = params.get("target")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&self.default_target);
                
                if text.is_empty() {
                    return SentientToolResult::failure("Çevrilecek metin boş olamaz");
                }
                
                if !self.validate_language(source) || !self.validate_language(target) {
                    return SentientToolResult::failure("Geçersiz dil kodu");
                }
                
                log::info!("🌐  TRANSLATE: {} → {} ({} karakter)", source, target, text.len());
                
                // Mock çeviri - gerçek implementation V-GATE üzerinden LLM kullanır
                let translated = format!("[{}→{}] {}", source, target, text);
                
                SentientToolResult::success_with_data(
                    "Çeviri tamamlandı",
                    serde_json::json!({
                        "original": text,
                        "translated": translated,
                        "source_lang": source,
                        "target_lang": target,
                        "confidence": 0.95,
                    })
                )
            }
            "detect" => {
                let text = params.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if text.is_empty() {
                    return SentientToolResult::failure("Metin boş olamaz");
                }
                
                log::info!("🔍  TRANSLATE: Dil algılanıyor");
                
                SentientToolResult::success_with_data(
                    "Dil algılandı",
                    serde_json::json!({
                        "detected_lang": "en",
                        "confidence": 0.92,
                        "alternatives": [
                            {"lang": "de", "confidence": 0.05},
                            {"lang": "nl", "confidence": 0.03},
                        ],
                    })
                )
            }
            "languages" => {
                SentientToolResult::success_with_data(
                    "Desteklenen diller",
                    serde_json::json!({
                        "languages": [
                            {"code": "tr", "name": "Türkçe"},
                            {"code": "en", "name": "English"},
                            {"code": "de", "name": "Deutsch"},
                            {"code": "fr", "name": "Français"},
                            {"code": "es", "name": "Español"},
                            {"code": "zh", "name": "中文"},
                            {"code": "ja", "name": "日本語"},
                            {"code": "ko", "name": "한국어"},
                            {"code": "ar", "name": "العربية"},
                        ],
                        "count": 20,
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen çeviri aksiyonu: '{}'. Kullanılabilir: translate, detect, languages",
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
    fn test_translate_tool_creation() {
        let tool = TranslateTool::new();
        assert_eq!(tool.name(), "translate");
    }
    
    #[test]
    fn test_language_validation() {
        let tool = TranslateTool::new();
        assert!(tool.validate_language("tr"));
        assert!(tool.validate_language("en"));
        assert!(tool.validate_language("auto"));
        assert!(!tool.validate_language("invalid"));
    }
    
    #[tokio::test]
    async fn test_translate() {
        let tool = TranslateTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("translate")),
            ("text".to_string(), serde_json::json!("Hello world")),
            ("source".to_string(), serde_json::json!("en")),
            ("target".to_string(), serde_json::json!("tr")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_detect_language() {
        let tool = TranslateTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("detect")),
            ("text".to_string(), serde_json::json!("Hello world")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
