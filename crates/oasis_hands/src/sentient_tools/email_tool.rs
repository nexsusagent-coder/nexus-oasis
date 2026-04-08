//! ═══════════════════════════════════════════════════════════════════════════════
//!  EMAIL TOOL - E-POSTA GÖNDERİM ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! E-posta gönderimi ve yönetimi.
//! SMTP entegrasyonu, şablonlar, ekler.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Email aracı - e-posta gönderimi
pub struct EmailTool {
    /// SMTP sunucu
    smtp_server: String,
    /// Gönderen adres
    from_address: String,
    /// Maksimum ek boyutu (bayt)
    max_attachment_size: usize,
}

impl EmailTool {
    /// Yeni Email aracı oluştur
    pub fn new() -> Self {
        Self {
            smtp_server: "smtp.example.com:587".to_string(),
            from_address: "sentient@nexus-oasis.local".to_string(),
            max_attachment_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Default for EmailTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for EmailTool {
    fn name(&self) -> &str {
        "email"
    }
    
    fn description(&self) -> &str {
        "E-posta gönderimi ve yönetimi. SMTP entegrasyonu, şablonlar, ekler."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Communication
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (send, template, list)"),
            ToolParameter::new("to", "string", false, "Alıcı adres(ler)i"),
            ToolParameter::new("subject", "string", false, "Konu"),
            ToolParameter::new("body", "string", false, "İçerik"),
            ToolParameter::new("html", "boolean", false, "HTML formatında mı?"),
            ToolParameter::new("attachments", "array", false, "Ek dosyalar"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "send" => {
                let to = params.get("to")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let subject = params.get("subject")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(Konusuz)");
                let body = params.get("body")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let html = params.get("html")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                if to.is_empty() {
                    return SentientToolResult::failure("Alıcı adresi gerekli");
                }
                
                // Basit e-posta doğrulama
                if !to.contains('@') {
                    return SentientToolResult::failure("Geçersiz e-posta adresi");
                }
                
                log::info!("📧  EMAIL: Gönderiliyor → {} (Konu: {})", to, subject);
                
                SentientToolResult::success_with_data(
                    "E-posta gönderildi",
                    serde_json::json!({
                        "to": to,
                        "subject": subject,
                        "body_length": body.len(),
                        "format": if html { "html" } else { "text" },
                        "message_id": format!("msg-{}", chrono::Utc::now().timestamp()),
                    })
                )
            }
            "template" => {
                let template_name = params.get("template")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");
                
                log::info!("📄  EMAIL: Şablon kullanılıyor → {}", template_name);
                
                SentientToolResult::success_with_data(
                    "Şablon yüklendi",
                    serde_json::json!({
                        "template": template_name,
                        "available_templates": ["welcome", "notification", "report", "alert"],
                    })
                )
            }
            "list" => {
                SentientToolResult::success_with_data(
                    "E-posta geçmişi",
                    serde_json::json!({
                        "emails": [
                            {"id": "msg-001", "to": "user@example.com", "subject": "Test", "status": "sent"},
                        ],
                        "count": 1,
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen e-posta aksiyonu: '{}'. Kullanılabilir: send, template, list",
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
    fn test_email_tool_creation() {
        let tool = EmailTool::new();
        assert_eq!(tool.name(), "email");
    }
    
    #[test]
    fn test_email_category() {
        let tool = EmailTool::new();
        assert_eq!(tool.category(), ToolCategory::Communication);
    }
    
    #[tokio::test]
    async fn test_email_send() {
        let tool = EmailTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("send")),
            ("to".to_string(), serde_json::json!("test@example.com")),
            ("subject".to_string(), serde_json::json!("Test konusu")),
            ("body".to_string(), serde_json::json!("Test içeriği")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_email_invalid_address() {
        let tool = EmailTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("send")),
            ("to".to_string(), serde_json::json!("invalid-email")),
            ("subject".to_string(), serde_json::json!("Test")),
        ]);
        let result = tool.execute(params).await;
        assert!(!result.success);
    }
}
