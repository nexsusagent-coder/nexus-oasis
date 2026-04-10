//! ═══════════════════════════════════════════════════════════════════════════════
//!  SED TOOL - METİN DÖNÜŞTÜRME ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Metin içinde bul-değiştir işlemleri.
//! Regex destekli, güvenli dönüşümler.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;

/// Sed aracı - metin dönüştürme
pub struct SedTool {
    /// Maksimum değişiklik sayısı
    max_replacements: usize,
}

impl SedTool {
    /// Yeni Sed aracı oluştur
    pub fn new() -> Self {
        Self {
            max_replacements: 10000,
        }
    }
    
    /// Bul ve değiştir
    fn replace(&self, pattern: &str, replacement: &str, content: &str, global: bool) -> Result<String, String> {
        let re = Regex::new(pattern)
            .map_err(|e| format!("Geçersiz regex: {}", e))?;
        
        if global {
            Ok(re.replace_all(content, replacement).to_string())
        } else {
            Ok(re.replace(content, replacement).to_string())
        }
    }
    
    /// Satır silme
    fn delete_lines(&self, pattern: &str, content: &str) -> Result<String, String> {
        let re = Regex::new(pattern)
            .map_err(|e| format!("Geçersiz regex: {}", e))?;
        
        let result: Vec<&str> = content
            .lines()
            .filter(|line| !re.is_match(line))
            .collect();
        
        Ok(result.join("\n"))
    }
}

impl Default for SedTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for SedTool {
    fn name(&self) -> &str {
        "sed"
    }
    
    fn description(&self) -> &str {
        "Metin dönüştürme aracı. Bul-değiştir, satır silme, regex ile işlemler."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Data
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("operation", "string", true, "İşlem türü (replace, delete)"),
            ToolParameter::new("pattern", "string", true, "Aranacak regex pattern"),
            ToolParameter::new("replacement", "string", false, "Yeni değer (replace için)"),
            ToolParameter::new("content", "string", true, "İşlenecek metin"),
            ToolParameter::new("global", "boolean", false, "Tüm eşleşmeleri değiştir"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let operation = params.get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("replace");
        
        let pattern = params.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let content = params.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        if pattern.is_empty() {
            return SentientToolResult::failure("Pattern boş olamaz");
        }
        
        let result = match operation {
            "replace" => {
                let replacement = params.get("replacement")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let global = params.get("global")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                
                self.replace(pattern, replacement, content, global)
            }
            "delete" | "d" => {
                self.delete_lines(pattern, content)
            }
            _ => {
                return SentientToolResult::failure(&format!(
                    "Bilinmeyen işlem: '{}'. Kullanılabilir: replace, delete",
                    operation
                ));
            }
        };
        
        match result {
            Ok(transformed) => SentientToolResult::success_with_data(
                "Dönüştürme tamamlandı",
                serde_json::json!({
                    "operation": operation,
                    "pattern": pattern,
                    "result": transformed,
                    "original_length": content.len(),
                    "result_length": transformed.len(),
                })
            ),
            Err(e) => SentientToolResult::failure(&e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sed_tool_creation() {
        let tool = SedTool::new();
        assert_eq!(tool.name(), "sed");
    }
    
    #[test]
    fn test_sed_replace() {
        let tool = SedTool::new();
        let result = tool.replace("test", "deneme", "Bu bir test cümlesi", true);
        assert!(result.is_ok());
        assert!(result.expect("operation failed").contains("deneme"));
    }
    
    #[test]
    fn test_sed_delete() {
        let tool = SedTool::new();
        let content = "Satır 1\nSilinecek satır\nSatır 3";
        let result = tool.delete_lines("Silinecek", content);
        assert!(result.is_ok());
        let deleted = result.expect("operation failed");
        assert!(!deleted.contains("Silinecek"));
    }
    
    #[tokio::test]
    async fn test_sed_execute_replace() {
        let tool = SedTool::new();
        let params = HashMap::from([
            ("operation".to_string(), serde_json::json!("replace")),
            ("pattern".to_string(), serde_json::json!("eski")),
            ("replacement".to_string(), serde_json::json!("yeni")),
            ("content".to_string(), serde_json::json!("Bu eski bir metin")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
