//! ─── A8 RESEARCH REPORTING ───
//!
//! SENTIENT'nın araştırma raporlama sistemi.
//! Farklı formatlarda rapor üretimi.
//!
//! Özellikler:
//! - Markdown raporları
//! - PDF çıktısı
//! - HTML raporları
//! - JSON çıktısı

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - Otomatik kaynakça

pub mod report;
pub mod generator;
pub mod citation;
pub mod templates;
pub mod formats;

pub use report::{ResearchReport, ReportSection, ReportMetadata};
pub use generator::{ReportGenerator, GeneratorConfig};
pub use citation::{CitationManager, Citation, CitationStyle};
pub use templates::{ReportTemplate, TemplateLibrary};

use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
// REPORT ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error("Rapor hatası: {0}")]
    General(String),
    
    #[error("Şablon bulunamadı: {0}")]
    TemplateNotFound(String),
    
    #[error("Oluşturma hatası: {0}")]
    GenerationError(String),
    
    #[error("Dışa aktarma hatası: {0}")]
    ExportError(String),
    
    #[error("Şablon hatası: {0}")]
    TemplateError(String),
}

pub type ReportResult<T> = Result<T, ReportError>;

// ═══════════════════════════════════════════════════════════════════════════════
// REPORT ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Rapor motoru
pub struct ReportEngine {
    generator: ReportGenerator,
    templates: TemplateLibrary,
    citations: Arc<RwLock<CitationManager>>,
}

impl ReportEngine {
    pub fn new() -> Self {
        Self {
            generator: ReportGenerator::new(GeneratorConfig::default()),
            templates: TemplateLibrary::default(),
            citations: Arc::new(RwLock::new(CitationManager::new())),
        }
    }
    
    /// Yeni rapor oluştur
    pub async fn create_report(&self, title: &str) -> ReportResult<ResearchReport> {
        let report = ResearchReport::new(title);
        Ok(report)
    }
    
    /// Rapora bölüm ekle
    pub async fn add_section(&self, report: &mut ResearchReport, section: ReportSection) {
        report.add_section(section);
    }
    
    /// Rapor oluştur
    pub async fn generate(&self, report: &ResearchReport, format: OutputFormat) -> ReportResult<String> {
        self.generator.generate(report, format).await
    }
    
    /// Şablondan rapor oluştur
    pub async fn from_template(&self, template_name: &str, data: &serde_json::Value) -> ReportResult<ResearchReport> {
        let template = self.templates.get(template_name)?;
        let report = template.render(data)?;
        Ok(report)
    }
    
    /// Kaynakça ekle
    pub async fn add_citation(&self, citation: Citation) {
        self.citations.write().await.add(citation);
    }
    
    /// Kaynakça listesini getir
    pub async fn get_citations(&self) -> Vec<Citation> {
        self.citations.read().await.list()
    }
    
    /// Kaynakça formatla
    pub async fn format_bibliography(&self, style: CitationStyle) -> String {
        self.citations.read().await.format_all(style)
    }
}

impl Default for ReportEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Çıktı formatı
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Markdown,
    Html,
    Json,
    Text,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_report_creation() {
        let engine = ReportEngine::new();
        let report = engine.create_report("Test Raporu").await.expect("operation failed");
        
        assert_eq!(report.title, "Test Raporu");
    }
    
    #[tokio::test]
    async fn test_report_generation() {
        let engine = ReportEngine::new();
        let mut report = engine.create_report("Test").await.expect("operation failed");
        
        report.add_section(ReportSection::new("Giriş", "Bu bir test raporudur."));
        
        let output = engine.generate(&report, OutputFormat::Markdown).await.expect("operation failed");
        assert!(output.contains("# Test"));
    }
}
