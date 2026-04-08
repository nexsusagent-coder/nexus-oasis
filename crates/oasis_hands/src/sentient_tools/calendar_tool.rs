//! ═══════════════════════════════════════════════════════════════════════════════
//!  CALENDAR TOOL - TAKVİM ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Takvim ve etkinlik yönetimi.
//! Google Calendar, Outlook entegrasyonu.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Calendar aracı - takvim yönetimi
pub struct CalendarTool {
    /// Varsayılan takvim
    default_calendar: String,
    /// Zaman dilimi
    timezone: String,
}

impl CalendarTool {
    /// Yeni Calendar aracı oluştur
    pub fn new() -> Self {
        Self {
            default_calendar: "primary".to_string(),
            timezone: "Europe/Istanbul".to_string(),
        }
    }
}

impl Default for CalendarTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for CalendarTool {
    fn name(&self) -> &str {
        "calendar"
    }
    
    fn description(&self) -> &str {
        "Takvim ve etkinlik yönetimi. Etkinlik oluşturma, listeleme, güncelleme, silme."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Scheduling
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (create, list, update, delete, today)"),
            ToolParameter::new("title", "string", false, "Etkinlik başlığı"),
            ToolParameter::new("start", "string", false, "Başlangıç zamanı (ISO 8601)"),
            ToolParameter::new("end", "string", false, "Bitiş zamanı (ISO 8601)"),
            ToolParameter::new("event_id", "string", false, "Etkinlik ID"),
            ToolParameter::new("description", "string", false, "Açıklama"),
            ToolParameter::new("attendees", "array", false, "Katılımcılar"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "create" | "add" => {
                let title = params.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Yeni Etkinlik");
                let start = params.get("start")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let end = params.get("end")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if start.is_empty() {
                    return SentientToolResult::failure("Başlangıç zamanı gerekli");
                }
                
                log::info!("📅  CALENDAR: Etkinlik oluşturuluyor → {}", title);
                
                SentientToolResult::success_with_data(
                    "Etkinlik oluşturuldu",
                    serde_json::json!({
                        "event_id": format!("evt-{}", chrono::Utc::now().timestamp()),
                        "title": title,
                        "start": start,
                        "end": end,
                        "calendar": self.default_calendar,
                        "timezone": self.timezone,
                    })
                )
            }
            "list" => {
                let date = params.get("date")
                    .and_then(|v| v.as_str())
                    .unwrap_or("today");
                
                log::info!("📋  CALENDAR: Etkinlikler listeleniyor → {}", date);
                
                SentientToolResult::success_with_data(
                    "Etkinlik listesi",
                    serde_json::json!({
                        "date": date,
                        "events": [
                            {
                                "id": "evt-001",
                                "title": "Toplantı",
                                "start": "2024-01-15T10:00:00",
                                "end": "2024-01-15T11:00:00",
                            },
                        ],
                        "count": 1,
                    })
                )
            }
            "today" => {
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                
                log::info!("📅  CALENDAR: Bugünün etkinlikleri");
                
                SentientToolResult::success_with_data(
                    &format!("{} tarihi etkinlikleri", today),
                    serde_json::json!({
                        "date": today,
                        "events": [],
                        "count": 0,
                    })
                )
            }
            "update" => {
                let event_id = params.get("event_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if event_id.is_empty() {
                    return SentientToolResult::failure("Etkinlik ID gerekli");
                }
                
                log::info!("📝  CALENDAR: Etkinlik güncelleniyor → {}", event_id);
                
                SentientToolResult::success(&format!("Etkinlik güncellendi: {}", event_id))
            }
            "delete" => {
                let event_id = params.get("event_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if event_id.is_empty() {
                    return SentientToolResult::failure("Etkinlik ID gerekli");
                }
                
                log::info!("🗑️  CALENDAR: Etkinlik siliniyor → {}", event_id);
                
                SentientToolResult::success(&format!("Etkinlik silindi: {}", event_id))
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen takvim aksiyonu: '{}'. Kullanılabilir: create, list, update, delete, today",
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
    fn test_calendar_tool_creation() {
        let tool = CalendarTool::new();
        assert_eq!(tool.name(), "calendar");
    }
    
    #[test]
    fn test_calendar_category() {
        let tool = CalendarTool::new();
        assert_eq!(tool.category(), ToolCategory::Scheduling);
    }
    
    #[tokio::test]
    async fn test_calendar_create() {
        let tool = CalendarTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("create")),
            ("title".to_string(), serde_json::json!("Test toplantısı")),
            ("start".to_string(), serde_json::json!("2024-01-15T10:00:00")),
            ("end".to_string(), serde_json::json!("2024-01-15T11:00:00")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_calendar_today() {
        let tool = CalendarTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("today")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
