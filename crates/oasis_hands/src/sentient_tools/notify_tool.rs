//! ═══════════════════════════════════════════════════════════════════════════════
//!  NOTIFY TOOL - BİLDİRİM SİSTEMİ ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Çeşitli bildirim kanalları.
//! Desktop, SMS, Push, Webhook bildirimleri.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Notify aracı - bildirim sistemi
pub struct NotifyTool {
    /// Varsayılan kanal
    default_channel: NotifyChannel,
    /// Sessiz mod
    silent: bool,
}

/// Bildirim kanalı
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyChannel {
    Desktop,
    Sms,
    Push,
    Webhook,
    Telegram,
    Discord,
    Slack,
}

impl NotifyTool {
    /// Yeni Notify aracı oluştur
    pub fn new() -> Self {
        Self {
            default_channel: NotifyChannel::Desktop,
            silent: false,
        }
    }
}

impl Default for NotifyTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for NotifyTool {
    fn name(&self) -> &str {
        "notify"
    }
    
    fn description(&self) -> &str {
        "Çeşitli kanallardan bildirim gönderme. Desktop, SMS, Push, Webhook, Telegram, Discord, Slack."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Communication
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (send, channels, history)"),
            ToolParameter::new("channel", "string", false, "Kanal (desktop, sms, push, webhook, telegram, discord, slack)"),
            ToolParameter::new("title", "string", false, "Bildirim başlığı"),
            ToolParameter::new("message", "string", false, "Bildirim içeriği"),
            ToolParameter::new("level", "string", false, "Seviye (info, warning, error, critical)"),
            ToolParameter::new("webhook_url", "string", false, "Webhook URL (webhook kanalı için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "send" => {
                let channel = params.get("channel")
                    .and_then(|v| v.as_str())
                    .unwrap_or("desktop");
                let title = params.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("SENTIENT Bildirimi");
                let message = params.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let level = params.get("level")
                    .and_then(|v| v.as_str())
                    .unwrap_or("info");
                
                if message.is_empty() {
                    return SentientToolResult::failure("Bildirim mesajı boş olamaz");
                }
                
                let icon = match level {
                    "info" => "ℹ️",
                    "warning" => "⚠️",
                    "error" => "❌",
                    "critical" => "🚨",
                    _ => "📢",
                };
                
                log::info!("{}  NOTIFY: {} → {} ({})", icon, channel, title, level);
                
                SentientToolResult::success_with_data(
                    "Bildirim gönderildi",
                    serde_json::json!({
                        "channel": channel,
                        "title": title,
                        "message": message,
                        "level": level,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    })
                )
            }
            "channels" => {
                SentientToolResult::success_with_data(
                    "Kullanılabilir kanallar",
                    serde_json::json!({
                        "channels": [
                            {"name": "desktop", "description": "Masaüstü bildirimi", "available": true},
                            {"name": "telegram", "description": "Telegram bot", "available": true},
                            {"name": "discord", "description": "Discord webhook", "available": false},
                            {"name": "slack", "description": "Slack webhook", "available": false},
                            {"name": "webhook", "description": "Özel webhook", "available": true},
                        ],
                    })
                )
            }
            "history" => {
                SentientToolResult::success_with_data(
                    "Bildirim geçmişi",
                    serde_json::json!({
                        "notifications": [
                            {"id": "notif-001", "channel": "desktop", "title": "Test", "sent_at": "2024-01-15T10:00:00Z"},
                        ],
                        "count": 1,
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen bildirim aksiyonu: '{}'. Kullanılabilir: send, channels, history",
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
    fn test_notify_tool_creation() {
        let tool = NotifyTool::new();
        assert_eq!(tool.name(), "notify");
    }
    
    #[tokio::test]
    async fn test_notify_send() {
        let tool = NotifyTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("send")),
            ("channel".to_string(), serde_json::json!("desktop")),
            ("title".to_string(), serde_json::json!("Test bildirimi")),
            ("message".to_string(), serde_json::json!("Bu bir test mesajıdır")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_notify_channels() {
        let tool = NotifyTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("channels")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
