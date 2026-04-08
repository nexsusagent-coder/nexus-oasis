//! ─── REPORT GENERATOR ───
//!
//! Rapor oluşturma motoru

use crate::{OutputFormat, ReportError, ReportResult, ResearchReport};
use serde::{Deserialize, Serialize};

/// Rapor oluşturucu
pub struct ReportGenerator {
    config: GeneratorConfig,
}

impl ReportGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }
    
    /// Rapor oluştur
    pub async fn generate(&self, report: &ResearchReport, format: OutputFormat) -> ReportResult<String> {
        match format {
            OutputFormat::Markdown => self.generate_markdown(report),
            OutputFormat::Html => self.generate_html(report),
            OutputFormat::Json => self.generate_json(report),
            OutputFormat::Text => self.generate_text(report),
        }
    }
    
    fn generate_markdown(&self, report: &ResearchReport) -> ReportResult<String> {
        Ok(report.to_markdown())
    }
    
    fn generate_html(&self, report: &ResearchReport) -> ReportResult<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html lang=\"tr\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>{}</title>\n", report.title));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("h1 { color: #333; border-bottom: 2px solid #333; padding-bottom: 10px; }\n");
        html.push_str("h2 { color: #555; margin-top: 30px; }\n");
        html.push_str("h3 { color: #666; }\n");
        html.push_str("blockquote { background: #f5f5f5; padding: 10px 20px; border-left: 3px solid #333; }\n");
        html.push_str("code { background: #f0f0f0; padding: 2px 6px; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str(&format!("<h1>{}</h1>\n", report.title));
        
        if !report.abstract_text.is_empty() {
            html.push_str("<h2>Özet</h2>\n");
            html.push_str(&format!("<p>{}</p>\n", report.abstract_text));
        }
        
        for section in &report.sections {
            html.push_str(&self.section_to_html(section));
        }
        
        if !report.conclusion.is_empty() {
            html.push_str("<h2>Sonuç</h2>\n");
            html.push_str(&format!("<p>{}</p>\n", report.conclusion));
        }
        
        if !report.references.is_empty() {
            html.push_str("<h2>Kaynakça</h2>\n<ol>\n");
            for reference in &report.references {
                html.push_str(&format!("<li>{}</li>\n", reference));
            }
            html.push_str("</ol>\n");
        }
        
        html.push_str("</body>\n</html>");
        
        Ok(html)
    }
    
    fn section_to_html(&self, section: &crate::report::ReportSection) -> String {
        let mut html = String::new();
        
        html.push_str(&format!("<h2>{}</h2>\n", section.title));
        html.push_str(&format!("<p>{}</p>\n", section.content));
        
        for sub in &section.subsections {
            html.push_str(&format!("<h3>{}</h3>\n", sub.title));
            html.push_str(&format!("<p>{}</p>\n", sub.content));
        }
        
        html
    }
    
    fn generate_json(&self, report: &ResearchReport) -> ReportResult<String> {
        serde_json::to_string_pretty(report)
            .map_err(|e| ReportError::GenerationError(e.to_string()))
    }
    
    fn generate_text(&self, report: &ResearchReport) -> ReportResult<String> {
        let mut text = String::new();
        
        text.push_str(&format!("{}\n", "=".repeat(60)));
        text.push_str(&format!("{}\n", report.title));
        text.push_str(&format!("{}\n\n", "=".repeat(60)));
        
        if !report.abstract_text.is_empty() {
            text.push_str("ÖZET\n-----\n");
            text.push_str(&format!("{}\n\n", report.abstract_text));
        }
        
        for section in &report.sections {
            text.push_str(&format!("\n{}\n{}\n", section.title, "-".repeat(section.title.len())));
            text.push_str(&format!("{}\n", section.content));
        }
        
        if !report.conclusion.is_empty() {
            text.push_str("\nSONUÇ\n------\n");
            text.push_str(&format!("{}\n", report.conclusion));
        }
        
        if !report.references.is_empty() {
            text.push_str("\nKAYNAKÇA\n--------\n");
            for (i, reference) in report.references.iter().enumerate() {
                text.push_str(&format!("{}. {}\n", i + 1, reference));
            }
        }
        
        Ok(text)
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new(GeneratorConfig::default())
    }
}

/// Oluşturucu yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub max_section_length: usize,
    pub include_metadata: bool,
    pub auto_toc: bool,
    pub style: String,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            max_section_length: 10000,
            include_metadata: true,
            auto_toc: true,
            style: "default".into(),
        }
    }
}
