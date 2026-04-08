//! Citation ve Reference Yönetimi
//! APA, MLA, Chicago ve diğer referans stilleri

use crate::web_search::SearchResult;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Citation yöneticisi
pub struct CitationManager {
    /// Referans stili
    style: ReferenceStyle,
    /// Mevcut citation'lar
    citations: Vec<Citation>,
    /// Referans sayacı
    ref_counter: HashMap<String, u32>,
}

/// Referans stili
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ReferenceStyle {
    APA,
    MLA,
    Chicago,
    Harvard,
    IEEE,
    Vancouver,
    Turabian,
}

/// Citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    /// Citation ID
    pub id: u32,
    /// Kaynak başlık
    pub title: String,
    /// Yazarlar
    pub authors: Vec<String>,
    /// Yayın yılı
    pub year: Option<u32>,
    /// URL
    pub url: String,
    /// Erişim tarihi
    pub accessed_date: String,
    /// Yayınlayan
    pub publisher: Option<String>,
    /// Oluşturulmuş citation metni
    pub formatted: String,
}

impl CitationManager {
    /// Yeni citation yöneticisi oluştur
    pub fn new(style: ReferenceStyle) -> Self {
        Self {
            style,
            citations: Vec::new(),
            ref_counter: HashMap::new(),
        }
    }
    
    /// SearchResult'lardan citation'lar oluştur
    pub fn cite(&self, results: &[SearchResult]) -> Vec<Citation> {
        let mut citations = Vec::new();
        
        for (i, result) in results.iter().enumerate() {
            let citation = Citation {
                id: (i + 1) as u32,
                title: result.title.clone(),
                authors: self.extract_authors(&result),
                year: self.extract_year(&result),
                url: result.url.clone(),
                accessed_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                publisher: self.extract_publisher(&result),
                formatted: self.format_citation(&result, i + 1),
            };
            
            citations.push(citation);
        }
        
        citations
    }
    
    /// Yazar bilgisi çıkar
    fn extract_authors(&self, result: &SearchResult) -> Vec<String> {
        // Gerçek uygulamada sayfa içeriğinden çıkarılır
        match result.source_type {
            crate::web_search::SourceType::Wikipedia => {
                vec!["Wikipedia Contributors".into()]
            }
            crate::web_search::SourceType::Academic => {
                vec!["Author, A.".into()]
            }
            _ => vec![]
        }
    }
    
    /// Yıl çıkar
    fn extract_year(&self, result: &SearchResult) -> Option<u32> {
        // URL'den veya içerikten yıl çıkarımı
        if result.url.contains("2024") {
            Some(2024)
        } else if result.url.contains("2023") {
            Some(2023)
        } else {
            Some(chrono::Utc::now().year() as u32)
        }
    }
    
    /// Yayınlayan çıkar
    fn extract_publisher(&self, result: &SearchResult) -> Option<String> {
        match result.source_type {
            crate::web_search::SourceType::Wikipedia => Some("Wikipedia Foundation".into()),
            crate::web_search::SourceType::Government => Some("Government Official".into()),
            crate::web_search::SourceType::Academic => Some("Academic Journal".into()),
            _ => None,
        }
    }
    
    /// Citation'ı formatla
    fn format_citation(&self, result: &SearchResult, index: usize) -> String {
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        match self.style {
            ReferenceStyle::APA => {
                format!("[{}] {}. (n.d.). {}. Retrieved {}", 
                    index, result.title, result.url, today)
            }
            ReferenceStyle::MLA => {
                format!("[{}] \"{}\" Web. {}.", 
                    index, result.title, today)
            }
            ReferenceStyle::Chicago => {
                format!("[{}] {} Accessed {}.", 
                    index, result.url, today)
            }
            ReferenceStyle::IEEE => {
                format!("[{}] \"{}\" [Online]. Available: {}", 
                    index, result.title, result.url)
            }
            _ => format!("[{}] {} - {}", index, result.title, result.url)
        }
    }
    
    /// Referans listesi oluştur
    pub fn generate_references(&self) -> String {
        let mut refs = String::new();
        
        for citation in &self.citations {
            refs.push_str(&citation.formatted);
            refs.push_str("\n\n");
        }
        
        refs
    }
    
    /// Bibliyografya oluştur (JSON)
    pub fn to_bibliography(&self) -> String {
        serde_json::to_string_pretty(&self.citations).unwrap_or_default()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_citation_manager_creation() {
        let manager = CitationManager::new(ReferenceStyle::APA);
        assert!(manager.citations.is_empty());
    }
    
    #[test]
    fn test_citation_from_search_result() {
        let manager = CitationManager::new(ReferenceStyle::APA);
        let results = vec![
            SearchResult {
                rank: 1,
                title: "Test Title".into(),
                url: "https://example.com".into(),
                snippet: "Test".into(),
                source_type: crate::web_search::SourceType::Wikipedia,
                credibility: 0.8,
            }
        ];
        
        let citations = manager.cite(&results);
        assert_eq!(citations.len(), 1);
        assert!(citations[0].formatted.contains("[1]"));
    }
    
    #[test]
    fn test_different_styles() {
        let results = vec![
            SearchResult {
                rank: 1,
                title: "Test".into(),
                url: "https://example.com".into(),
                snippet: "".into(),
                source_type: crate::web_search::SourceType::Other,
                credibility: 0.5,
            }
        ];
        
        let apa = CitationManager::new(ReferenceStyle::APA).cite(&results);
        let mla = CitationManager::new(ReferenceStyle::MLA).cite(&results);
        
        assert_ne!(apa[0].formatted, mla[0].formatted);
    }
}
