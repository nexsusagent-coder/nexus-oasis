//! ═══════════════════════════════════════════════════════════════════════════════
//!  BRIEF TOOL - Context Özetleme ve Bilgi Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Brief Tool - Context özeti ve bilgi yönetimi
pub struct BriefTool {
    max_brief_size: usize,
}

/// Brief türleri
#[derive(Debug, Clone)]
pub enum BriefType {
    /// Proje özeti
    Project,
    /// Oturum özeti
    Session,
    /// Kod özeti
    Code,
    /// Doküman özeti
    Document,
    /// Özelleştirilmiş
    Custom,
}

impl BriefTool {
    pub fn new() -> Self {
        Self { max_brief_size: 8000 }
    }
    
    pub fn default_tool() -> Self {
        Self::new()
    }
    
    /// Metin özeti oluştur
    pub fn summarize(&self, text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            return text.to_string();
        }
        
        // Cümlelere ayır
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        if sentences.is_empty() {
            return text.chars().take(max_length).collect::<String>() + "...";
        }
        
        // İlk ve son cümleleri al
        let mut summary = String::new();
        let mut current_len = 0;
        
        // Başlangıç cümleleri
        for sentence in &sentences {
            if current_len + sentence.len() + 1 > max_length / 2 {
                break;
            }
            if !summary.is_empty() {
                summary.push(' ');
            }
            summary.push_str(sentence);
            summary.push('.');
            current_len += sentence.len() + 2;
        }
        
        // ... ekle
        if sentences.len() > 2 {
            summary.push_str(" ... ");
            
            // Son cümleler
            let _remaining = max_length - current_len - 10;
            for sentence in sentences.iter().rev().take(2) {
                if summary.len() + sentence.len() > max_length {
                    break;
                }
                summary.push_str(sentence);
                summary.push('.');
            }
        }
        
        summary
    }
    
    /// Structured brief oluştur
    pub fn create_brief(&self, brief_type: BriefType, content: &str, context: &HashMap<String, String>) -> serde_json::Value {
        let summary = self.summarize(content, self.max_brief_size);
        
        let brief_type_str = match brief_type {
            BriefType::Project => "project",
            BriefType::Session => "session",
            BriefType::Code => "code",
            BriefType::Document => "document",
            BriefType::Custom => "custom",
        };
        
        let mut brief = serde_json::json!({
            "type": brief_type_str,
            "summary": summary,
            "original_length": content.len(),
            "brief_length": summary.len(),
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        
        // Context ekle
        if let serde_json::Value::Object(ref mut map) = brief {
            for (key, value) in context {
                map.insert(key.clone(), serde_json::json!(value));
            }
        }
        
        brief
    }
    
    /// Anahtar kelimeleri çıkar
    pub fn extract_keywords(&self, text: &str, count: usize) -> Vec<String> {
        let stop_words = ["the", "a", "an", "is", "are", "was", "were", "be", "been", 
                         "being", "have", "has", "had", "do", "does", "did", "will",
                         "would", "could", "should", "may", "might", "must", "shall",
                         "can", "need", "dare", "ought", "used", "to", "of", "in",
                         "for", "on", "with", "at", "by", "from", "as", "into",
                         "through", "during", "before", "after", "above", "below",
                         "between", "under", "again", "further", "then", "once"];
        
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        
        for word in text.to_lowercase().split_whitespace() {
            let word = word.trim_matches(|c: char| !c.is_alphanumeric());
            if word.len() > 2 && !stop_words.contains(&&*word) {
                *word_freq.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        
        let mut keywords: Vec<_> = word_freq.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        
        keywords.into_iter().take(count).map(|(word, _)| word).collect()
    }
}

#[async_trait]
impl SentientTool for BriefTool {
    fn name(&self) -> &str { "brief" }
    
    fn description(&self) -> &str {
        "Context özetleme ve bilgi yönetimi. Özet oluştur, anahtar kelime çıkar."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Analysis }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon: summarize, keywords, brief"),
            ToolParameter::new("content", "string", false, "İçerik metni"),
            ToolParameter::new("max_length", "integer", false, "Maksimum özet uzunluğu"),
            ToolParameter::new("type", "string", false, "Brief türü: project, session, code, document"),
            ToolParameter::new("keyword_count", "integer", false, "Çıkarılacak anahtar kelime sayısı"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "summarize" => {
                let content = params.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let max_length = params.get("max_length")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(2000) as usize;
                
                let summary = self.summarize(content, max_length);
                
                SentientToolResult::success_with_data(
                    "Özet oluşturuldu",
                    serde_json::json!({
                        "summary": summary,
                        "original_length": content.len(),
                        "brief_length": summary.len(),
                        "compression_ratio": if content.len() > 0 {
                            (summary.len() as f64 / content.len() as f64 * 100.0) as u32
                        } else {
                            0
                        }
                    })
                )
            }
            "keywords" => {
                let content = params.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let count = params.get("keyword_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(10) as usize;
                
                let keywords = self.extract_keywords(content, count);
                
                SentientToolResult::success_with_data(
                    &format!("{} anahtar kelime çıkarıldı", keywords.len()),
                    serde_json::json!({
                        "keywords": keywords,
                        "count": keywords.len()
                    })
                )
            }
            "brief" => {
                let content = params.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let brief_type = match params.get("type").and_then(|v| v.as_str()) {
                    Some("project") => BriefType::Project,
                    Some("session") => BriefType::Session,
                    Some("code") => BriefType::Code,
                    Some("document") => BriefType::Document,
                    _ => BriefType::Custom,
                };
                
                // Context parametrelerini topla
                let context: HashMap<String, String> = params.iter()
                    .filter(|(k, _)| !["action", "content", "type"].contains(&k.as_str()))
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect();
                
                let brief = self.create_brief(brief_type, content, &context);
                
                SentientToolResult::success_with_data(
                    "Brief oluşturuldu",
                    brief
                )
            }
            _ => SentientToolResult::failure(&format!("Bilinmeyen aksiyon: {}", action))
        }
    }
}

impl Default for BriefTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_summarize() {
        let tool = BriefTool::new();
        let long_text = "This is a test. It has multiple sentences. Each sentence is important. But we only need the key parts.";
        let summary = tool.summarize(long_text, 50);
        
        assert!(summary.len() <= 60); // Allow for ellipsis
    }
    
    #[test]
    fn test_keywords() {
        let tool = BriefTool::new();
        let text = "Rust is a programming language. Rust is fast. Rust is safe.";
        let keywords = tool.extract_keywords(text, 5);
        
        assert!(keywords.contains(&"rust".to_string()));
        assert!(keywords.contains(&"programming".to_string()));
    }
}
