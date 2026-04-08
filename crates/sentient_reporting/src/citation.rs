//! ─── CITATION ───
//!
//! Kaynakça yönetimi

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Kaynakça girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub id: Uuid,
    pub citation_type: CitationType,
    pub authors: Vec<String>,
    pub title: String,
    pub year: Option<u16>,
    pub source: String,
    pub url: Option<String>,
    pub doi: Option<String>,
    pub pages: Option<String>,
    pub extra: HashMap<String, String>,
}

impl Citation {
    pub fn new(title: impl Into<String>, authors: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            citation_type: CitationType::Article,
            title: title.into(),
            authors,
            year: None,
            source: String::new(),
            url: None,
            doi: None,
            pages: None,
            extra: HashMap::new(),
        }
    }
    
    pub fn with_year(mut self, year: u16) -> Self {
        self.year = Some(year);
        self
    }
    
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    
    pub fn with_doi(mut self, doi: impl Into<String>) -> Self {
        self.doi = Some(doi.into());
        self
    }
    
    /// Belirtilen stilde formatla
    pub fn format(&self, style: CitationStyle) -> String {
        match style {
            CitationStyle::APA => self.format_apa(),
            CitationStyle::MLA => self.format_mla(),
            CitationStyle::Chicago => self.format_chicago(),
            CitationStyle::Harvard => self.format_harvard(),
            CitationStyle::IEEE => self.format_ieee(),
        }
    }
    
    fn format_apa(&self) -> String {
        let mut citation = String::new();
        
        // Yazarlar
        if self.authors.len() == 1 {
            citation.push_str(&format!("{}, ", self.authors[0]));
        } else if self.authors.len() == 2 {
            citation.push_str(&format!("{} & {}", self.authors[0], self.authors[1]));
        } else {
            citation.push_str(&format!("{} et al.", self.authors[0]));
        }
        
        // Yıl
        if let Some(year) = self.year {
            citation.push_str(&format!(" ({})", year));
        }
        
        // Başlık
        citation.push_str(&format!(". {}.", self.title));
        
        // Kaynak
        if !self.source.is_empty() {
            citation.push_str(&format!(" {}.", self.source));
        }
        
        // DOI veya URL
        if let Some(doi) = &self.doi {
            citation.push_str(&format!(" https://doi.org/{}", doi));
        } else if let Some(url) = &self.url {
            citation.push_str(&format!(" {}", url));
        }
        
        citation
    }
    
    fn format_mla(&self) -> String {
        let mut citation = String::new();
        
        if !self.authors.is_empty() {
            citation.push_str(&format!("{}. ", self.authors[0]));
        }
        
        citation.push_str(&format!("\"{}.\" ", self.title));
        
        if !self.source.is_empty() {
            citation.push_str(&format!("{}, ", self.source));
        }
        
        if let Some(year) = self.year {
            citation.push_str(&format!("{}, ", year));
        }
        
        if let Some(url) = &self.url {
            citation.push_str(url);
        }
        
        citation
    }
    
    fn format_chicago(&self) -> String {
        let mut citation = String::new();
        
        if !self.authors.is_empty() {
            citation.push_str(&format!("{} ", self.authors[0]));
        }
        
        if let Some(year) = self.year {
            citation.push_str(&format!("({}). ", year));
        }
        
        citation.push_str(&format!("\"{}.\" ", self.title));
        
        if !self.source.is_empty() {
            citation.push_str(&format!("{}. ", self.source));
        }
        
        citation
    }
    
    fn format_harvard(&self) -> String {
        self.format_apa() // Harvard APA'ya benzer
    }
    
    fn format_ieee(&self) -> String {
        let mut citation = String::new();
        
        if !self.authors.is_empty() {
            let initials: String = self.authors.iter()
                .map(|a| a.chars().next().unwrap_or(' ').to_string())
                .collect::<Vec<_>>()
                .join(". ");
            citation.push_str(&format!("{}., ", initials));
        }
        
        citation.push_str(&format!("\"{}\", ", self.title));
        
        if !self.source.is_empty() {
            citation.push_str(&format!("{}, ", self.source));
        }
        
        if let Some(year) = self.year {
            citation.push_str(&format!("{}, ", year));
        }
        
        citation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationType {
    Article,
    Book,
    Conference,
    Website,
    Report,
    Thesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationStyle {
    APA,
    MLA,
    Chicago,
    Harvard,
    IEEE,
}

/// Kaynakça yöneticisi
#[derive(Debug, Clone, Default)]
pub struct CitationManager {
    citations: Vec<Citation>,
}

impl CitationManager {
    pub fn new() -> Self {
        Self {
            citations: Vec::new(),
        }
    }
    
    pub fn add(&mut self, citation: Citation) {
        self.citations.push(citation);
    }
    
    pub fn list(&self) -> Vec<Citation> {
        self.citations.clone()
    }
    
    pub fn format_all(&self, style: CitationStyle) -> String {
        self.citations.iter()
            .map(|c| c.format(style))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    pub fn count(&self) -> usize {
        self.citations.len()
    }
}
