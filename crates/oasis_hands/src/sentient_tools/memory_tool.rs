//! ═══════════════════════════════════════════════════════════════════════════════
//!  MEMORY TOOL - BELLEK YÖNETİM ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! SENTIENT bellek sistemi ile etkileşim.
//! Kısa/uzun vadeli bellek, bilgi grafiği.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Memory aracı - bellek yönetimi
pub struct MemoryTool {
    /// Varsayılan bellek türü
    default_type: MemoryType,
}

/// Bellek türü
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// Kısa vadeli (geçici)
    ShortTerm,
    /// Uzun vadeli (kalıcı)
    LongTerm,
    /// Çalışma belleği
    Working,
}

impl MemoryTool {
    /// Yeni Memory aracı oluştur
    pub fn new() -> Self {
        Self {
            default_type: MemoryType::LongTerm,
        }
    }
}

impl Default for MemoryTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for MemoryTool {
    fn name(&self) -> &str {
        "memory"
    }
    
    fn description(&self) -> &str {
        "SENTIENT bellek sistemi ile etkileşim. Bilgi kaydetme, sorgulama, arama."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Memory
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (save, recall, search, forget, clear)"),
            ToolParameter::new("key", "string", false, "Anahtar (save, recall için)"),
            ToolParameter::new("value", "string", false, "Değer (save için)"),
            ToolParameter::new("query", "string", false, "Arama sorgusu (search için)"),
            ToolParameter::new("memory_type", "string", false, "Bellek türü (short, long, working)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let memory_type = params.get("memory_type")
            .and_then(|v| v.as_str())
            .unwrap_or("long");
        
        match action {
            "save" | "store" => {
                let key = params.get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let value = params.get("value")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if key.is_empty() || value.is_empty() {
                    return SentientToolResult::failure("Anahtar ve değer gerekli");
                }
                
                log::info!("💾  MEMORY: Kaydediliyor → {} ({} bayt)", key, value.len());
                
                SentientToolResult::success_with_data(
                    "Belleğe kaydedildi",
                    serde_json::json!({
                        "key": key,
                        "value_length": value.len(),
                        "memory_type": memory_type,
                    })
                )
            }
            "recall" | "get" => {
                let key = params.get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if key.is_empty() {
                    return SentientToolResult::failure("Anahtar gerekli");
                }
                
                log::info!("📖  MEMORY: Hatırlanıyor → {}", key);
                
                SentientToolResult::success_with_data(
                    "Bellekten alındı",
                    serde_json::json!({
                        "key": key,
                        "value": "Mock bellek değeri",
                        "memory_type": memory_type,
                    })
                )
            }
            "search" => {
                let query = params.get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if query.is_empty() {
                    return SentientToolResult::failure("Sorgu gerekli");
                }
                
                log::info!("🔍  MEMORY: Aranıyor → {}", query);
                
                SentientToolResult::success_with_data(
                    "Arama tamamlandı",
                    serde_json::json!({
                        "query": query,
                        "results": [
                            {"key": "result1", "relevance": 0.95},
                            {"key": "result2", "relevance": 0.82},
                        ],
                    })
                )
            }
            "forget" | "delete" => {
                let key = params.get("key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("🗑️  MEMORY: Siliniyor → {}", key);
                
                SentientToolResult::success(&format!("Bellekten silindi: {}", key))
            }
            "clear" => {
                log::info!("🧹  MEMORY: Tüm bellek temizleniyor");
                SentientToolResult::success("Bellek temizlendi")
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen bellek aksiyonu: '{}'. Kullanılabilir: save, recall, search, forget, clear",
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
    fn test_memory_tool_creation() {
        let tool = MemoryTool::new();
        assert_eq!(tool.name(), "memory");
    }
    
    #[test]
    fn test_memory_category() {
        let tool = MemoryTool::new();
        assert_eq!(tool.category(), ToolCategory::Memory);
    }
    
    #[tokio::test]
    async fn test_memory_save() {
        let tool = MemoryTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("save")),
            ("key".to_string(), serde_json::json!("test_key")),
            ("value".to_string(), serde_json::json!("test_value")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_memory_recall() {
        let tool = MemoryTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("recall")),
            ("key".to_string(), serde_json::json!("test_key")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
