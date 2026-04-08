//! ─── RESEARCH REPORT ───
//!
//! Araştırma raporu yapısı

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Araştırma raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchReport {
    /// Benzersiz ID
    pub id: Uuid,
    /// Başlık
    pub title: String,
    /// Özet
    pub abstract_text: String,
    /// Bölümler
    pub sections: Vec<ReportSection>,
    /// Sonuç
    pub conclusion: String,
    /// Kaynakça
    pub references: Vec<String>,
    /// Metadata
    pub metadata: ReportMetadata,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Güncelleme zamanı
    pub updated_at: DateTime<Utc>,
}

impl ResearchReport {
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            abstract_text: String::new(),
            sections: Vec::new(),
            conclusion: String::new(),
            references: Vec::new(),
            metadata: ReportMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Bölüm ekle
    pub fn add_section(&mut self, section: ReportSection) {
        self.sections.push(section);
        self.updated_at = Utc::now();
    }
    
    /// Özet ayarla
    pub fn set_abstract(&mut self, text: impl Into<String>) {
        self.abstract_text = text.into();
        self.updated_at = Utc::now();
    }
    
    /// Sonuç ayarla
    pub fn set_conclusion(&mut self, text: impl Into<String>) {
        self.conclusion = text.into();
        self.updated_at = Utc::now();
    }
    
    /// Kaynak ekle
    pub fn add_reference(&mut self, reference: impl Into<String>) {
        self.references.push(reference.into());
        self.updated_at = Utc::now();
    }
    
    /// Markdown formatına dönüştür
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        // Başlık
        md.push_str(&format!("# {}\n\n", self.title));
        
        // Metadata
        md.push_str(&format!("> Oluşturulma: {}\n", self.created_at.format("%Y-%m-%d %H:%M")));
        if let Some(author) = &self.metadata.author {
            md.push_str(&format!("> Yazar: {}\n", author));
        }
        md.push_str("\n---\n\n");
        
        // Özet
        if !self.abstract_text.is_empty() {
            md.push_str("## Özet\n\n");
            md.push_str(&self.abstract_text);
            md.push_str("\n\n");
        }
        
        // Bölümler
        for section in &self.sections {
            md.push_str(&section.to_markdown());
            md.push_str("\n");
        }
        
        // Sonuç
        if !self.conclusion.is_empty() {
            md.push_str("## Sonuç\n\n");
            md.push_str(&self.conclusion);
            md.push_str("\n\n");
        }
        
        // Kaynakça
        if !self.references.is_empty() {
            md.push_str("## Kaynakça\n\n");
            for (i, ref_text) in self.references.iter().enumerate() {
                md.push_str(&format!("{}. {}\n", i + 1, ref_text));
            }
        }
        
        md
    }
    
    /// Toplam kelime sayısı
    pub fn word_count(&self) -> usize {
        let mut count = self.title.split_whitespace().count();
        count += self.abstract_text.split_whitespace().count();
        count += self.conclusion.split_whitespace().count();
        for section in &self.sections {
            count += section.word_count();
        }
        count
    }
}

/// Rapor bölümü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    /// Başlık
    pub title: String,
    /// İçerik
    pub content: String,
    /// Alt bölümler
    pub subsections: Vec<ReportSection>,
    /// Sıra
    pub order: usize,
    /// Etiketler
    pub tags: Vec<String>,
}

impl ReportSection {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            subsections: Vec::new(),
            order: 0,
            tags: Vec::new(),
        }
    }
    
    pub fn add_subsection(&mut self, section: ReportSection) {
        self.subsections.push(section);
    }
    
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("## {}\n\n", self.title));
        md.push_str(&self.content);
        md.push_str("\n\n");
        
        for sub in &self.subsections {
            md.push_str(&format!("### {}\n\n", sub.title));
            md.push_str(&sub.content);
            md.push_str("\n\n");
        }
        
        md
    }
    
    pub fn word_count(&self) -> usize {
        let mut count = self.title.split_whitespace().count();
        count += self.content.split_whitespace().count();
        for sub in &self.subsections {
            count += sub.word_count();
        }
        count
    }
}

/// Rapor metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub author: Option<String>,
    pub version: String,
    pub tags: Vec<String>,
    pub source_query: Option<String>,
    pub tokens_used: u64,
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ReportMetadata {
    fn default() -> Self {
        Self {
            author: Some("SENTIENT".into()),
            version: "1.0.0".into(),
            tags: Vec::new(),
            source_query: None,
            tokens_used: 0,
            extra: HashMap::new(),
        }
    }
}
