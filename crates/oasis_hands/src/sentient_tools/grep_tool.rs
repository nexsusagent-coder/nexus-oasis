//! ═══════════════════════════════════════════════════════════════════════════════
//!  GREP TOOL - METİN ARAMA ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Dosya ve metin içinde arama yapar.
//! Regex destekli, güvenli ve hızlı.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;

/// Grep aracı - metin arama
pub struct GrepTool {
    /// Maksimum sonuç sayısı
    max_results: usize,
}

impl GrepTool {
    /// Yeni Grep aracı oluştur
    pub fn new() -> Self {
        Self {
            max_results: 1000,
        }
    }
    
    /// Dosya içinde ara
    fn search_in_file(&self, pattern: &str, content: &str, case_sensitive: bool) -> Vec<GrepMatch> {
        let mut matches = Vec::new();
        
        let re = if case_sensitive {
            Regex::new(pattern).ok()
        } else {
            Regex::new(&format!("(?i){}", pattern)).ok()
        };
        
        let re = match re {
            Some(r) => r,
            None => return matches,
        };
        
        for (line_num, line) in content.lines().enumerate() {
            if re.is_match(line) {
                matches.push(GrepMatch {
                    line_number: line_num + 1,
                    line: line.to_string(),
                    matches: re.find_iter(line).map(|m| m.as_str().to_string()).collect(),
                });
                
                if matches.len() >= self.max_results {
                    break;
                }
            }
        }
        
        matches
    }
}

impl Default for GrepTool {
    fn default() -> Self {
        Self::new()
    }
}

/// Arama sonucu
#[derive(Debug, Clone, serde::Serialize)]
pub struct GrepMatch {
    /// Satır numarası
    pub line_number: usize,
    /// Satır içeriği
    pub line: String,
    /// Eşleşmeler
    pub matches: Vec<String>,
}

#[async_trait]
impl SentientTool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }
    
    fn description(&self) -> &str {
        "Metin ve dosya içinde regex ile arama yapar. Büyük/küçük harf duyarsız arama destekli."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Data
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("pattern", "string", true, "Aranacak regex pattern'i"),
            ToolParameter::new("content", "string", true, "Arama yapılacak metin"),
            ToolParameter::new("case_sensitive", "boolean", false, "Büyük/küçük harf duyarlı mı?"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let pattern = params.get("pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let content = params.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let case_sensitive = params.get("case_sensitive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        if pattern.is_empty() {
            return SentientToolResult::failure("Arama pattern'i boş olamaz");
        }
        
        // Regex doğrulama
        if Regex::new(pattern).is_err() {
            return SentientToolResult::failure(&format!("Geçersiz regex pattern: {}", pattern));
        }
        
        let matches = self.search_in_file(pattern, content, case_sensitive);
        
        SentientToolResult::success_with_data(
            &format!("{} eşleşme bulundu", matches.len()),
            serde_json::json!({
                "pattern": pattern,
                "total_matches": matches.len(),
                "results": matches,
            })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grep_tool_creation() {
        let tool = GrepTool::new();
        assert_eq!(tool.name(), "grep");
    }
    
    #[test]
    fn test_grep_search() {
        let tool = GrepTool::new();
        let content = "Merhaba Dünya\nBu bir test satırı\nMerhaba tekrar";
        let matches = tool.search_in_file("Merhaba", content, true);
        assert_eq!(matches.len(), 2);
    }
    
    #[test]
    fn test_grep_case_insensitive() {
        let tool = GrepTool::new();
        let content = "MERHABA\nmerhaba\nMerHaba";
        let matches = tool.search_in_file("merhaba", content, false);
        assert_eq!(matches.len(), 3);
    }
    
    #[tokio::test]
    async fn test_grep_execute() {
        let tool = GrepTool::new();
        let params = HashMap::from([
            ("pattern".to_string(), serde_json::json!("test")),
            ("content".to_string(), serde_json::json!("Bu bir test satırı\nTest ediyoruz")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
