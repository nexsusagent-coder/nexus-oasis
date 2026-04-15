//! ─── RESEARCH REPORT GENERATOR ───
//!
//! Generates comprehensive research reports.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::error::ResearchError;
use crate::types::*;
use crate::agent::Synthesis;
use crate::analysis::AnalysisResults;

/// Research report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResearchReport {
    /// Report title
    pub title: String,
    /// Executive summary
    pub summary: String,
    /// Report sections
    pub sections: Vec<ReportSection>,
    /// Sources cited
    pub sources: Vec<CitedSource>,
    /// Metadata
    pub metadata: ReportMetadata,
}

/// Report section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSection {
    /// Section ID
    pub id: String,
    /// Section title
    pub title: String,
    /// Section content
    pub content: String,
    /// Subsections
    pub subsections: Vec<ReportSection>,
    /// Key points
    pub key_points: Vec<String>,
    /// Source references
    pub source_refs: Vec<String>,
}

impl Default for ReportSection {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: String::new(),
            content: String::new(),
            subsections: Vec::new(),
            key_points: Vec::new(),
            source_refs: Vec::new(),
        }
    }
}

/// Cited source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitedSource {
    /// Source ID
    pub id: String,
    /// Citation text
    pub citation: String,
    /// URL
    pub url: String,
    /// Source type
    pub source_type: SourceType,
    /// Credibility score
    pub credibility: f32,
}

/// Report metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportMetadata {
    /// Generated at
    pub generated_at: DateTime<Utc>,
    /// Model used
    pub model: String,
    /// Language
    pub language: String,
    /// Version
    pub version: String,
    /// Word count
    pub word_count: usize,
    /// Reading time (minutes)
    pub reading_time: usize,
}

/// Report format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    Markdown,
    HTML,
    PDF,
    JSON,
    PlainText,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Markdown
    }
}

/// Report generator
pub struct ReportGenerator {
    format: ReportFormat,
}

impl ReportGenerator {
    pub fn new(format: ReportFormat) -> Self {
        Self { format }
    }

    /// Generate research report
    pub async fn generate(
        &self,
        query: &ResearchQuery,
        sources: &[SourceResult],
        synthesis: Synthesis,
        stats: &ResearchStats,
    ) -> Result<ResearchReport, ResearchError> {
        // Build report structure
        let title = format!("{} Araştırması", query.topic);
        let summary = synthesis.summary.clone();
        
        // Build sections
        let mut sections = Vec::new();
        
        // Introduction section
        sections.push(ReportSection {
            id: "intro".to_string(),
            title: "Giriş".to_string(),
            content: format!("Bu rapor, '{}' konusu hakkında kapsamlı bir araştırma sunmaktadır.", query.topic),
            subsections: Vec::new(),
            key_points: Vec::new(),
            source_refs: Vec::new(),
        });
        
        // Key findings section
        sections.push(ReportSection {
            id: "findings".to_string(),
            title: "Temel Bulgular".to_string(),
            content: String::new(),
            subsections: synthesis.key_findings.iter().map(|f| ReportSection {
                id: uuid::Uuid::new_v4().to_string(),
                title: f.summary.clone(),
                content: String::new(),
                subsections: Vec::new(),
                key_points: f.evidence.iter().map(|e| e.excerpt.clone()).collect(),
                source_refs: f.evidence.iter().map(|e| e.source_id.clone()).collect(),
            }).collect(),
            key_points: Vec::new(),
            source_refs: Vec::new(),
        });
        
        // Sources section
        let cited_sources: Vec<CitedSource> = sources.iter().map(|s| CitedSource {
            id: s.id.clone(),
            citation: format!("{} - {}", s.title, s.url),
            url: s.url.clone(),
            source_type: s.source_type,
            credibility: s.credibility_score,
        }).collect();
        
        // Calculate metadata
        let total_words: usize = sections.iter()
            .map(|s| s.content.split_whitespace().count())
            .sum();
        
        let metadata = ReportMetadata {
            generated_at: Utc::now(),
            model: "qwen/qwen3.6-plus:free".to_string(),
            language: query.language.clone(),
            version: "4.0.0".to_string(),
            word_count: total_words,
            reading_time: (total_words / 200).max(1),
        };

        Ok(ResearchReport {
            title,
            summary,
            sections,
            sources: cited_sources,
            metadata,
        })
    }

    /// Render report to string
    pub fn render(&self, report: &ResearchReport) -> String {
        match self.format {
            ReportFormat::Markdown => self.render_markdown(report),
            ReportFormat::HTML => self.render_html(report),
            ReportFormat::JSON => self.render_json(report),
            ReportFormat::PlainText => self.render_plain(report),
            ReportFormat::PDF => self.render_markdown(report), // Would convert to PDF
        }
    }

    fn render_markdown(&self, report: &ResearchReport) -> String {
        let mut md = String::new();
        
        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!("## Özet\n\n{}\n\n", report.summary));
        
        for section in &report.sections {
            md.push_str(&format!("## {}\n\n", section.title));
            if !section.content.is_empty() {
                md.push_str(&section.content);
                md.push_str("\n\n");
            }
            
            if !section.key_points.is_empty() {
                md.push_str("### Anahtar Noktalar\n\n");
                for point in &section.key_points {
                    md.push_str(&format!("- {}\n", point));
                }
                md.push_str("\n");
            }
            
            for subsection in &section.subsections {
                md.push_str(&format!("### {}\n\n", subsection.title));
                for point in &subsection.key_points {
                    md.push_str(&format!("- {}\n", point));
                }
                md.push_str("\n");
            }
        }
        
        md.push_str("## Kaynaklar\n\n");
        for (i, source) in report.sources.iter().enumerate() {
            md.push_str(&format!("{}. [{}] {} (Güvenilirlik: {:.0}%)\n", 
                i + 1, 
                source.citation,
                source.source_type.display_name(),
                source.credibility
            ));
        }
        
        md.push_str(&format!("\n---\n*Oluşturulma: {} | Kelime: {} | Okuma süresi: {}dk*\n",
            report.metadata.generated_at.format("%Y-%m-%d %H:%M"),
            report.metadata.word_count,
            report.metadata.reading_time
        ));
        
        md
    }

    fn render_html(&self, report: &ResearchReport) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", report.title));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }\n");
        html.push_str("h1 { color: #1a1a1a; }\n");
        html.push_str("h2 { color: #333; margin-top: 2rem; }\n");
        html.push_str("h3 { color: #555; }\n");
        html.push_str(".source { margin-left: 1rem; color: #666; }\n");
        html.push_str(".meta { font-size: 0.875rem; color: #888; margin-top: 2rem; }\n");
        html.push_str("</style>\n</head>\n<body>\n");
        
        html.push_str(&format!("<h1>{}</h1>\n", report.title));
        html.push_str(&format!("<p><strong>Özet:</strong> {}</p>\n", report.summary));
        
        for section in &report.sections {
            html.push_str(&format!("<h2>{}</h2>\n", section.title));
            if !section.content.is_empty() {
                html.push_str(&format!("<p>{}</p>\n", section.content));
            }
            
            if !section.key_points.is_empty() {
                html.push_str("<ul>\n");
                for point in &section.key_points {
                    html.push_str(&format!("<li>{}</li>\n", point));
                }
                html.push_str("</ul>\n");
            }
            
            for subsection in &section.subsections {
                html.push_str(&format!("<h3>{}</h3>\n<ul>\n", subsection.title));
                for point in &subsection.key_points {
                    html.push_str(&format!("<li>{}</li>\n", point));
                }
                html.push_str("</ul>\n");
            }
        }
        
        html.push_str("<h2>Kaynaklar</h2>\n<ol>\n");
        for source in &report.sources {
            html.push_str(&format!("<li class=\"source\"><a href=\"{}\">{}</a> ({} - {:.0}%)</li>\n",
                source.url, source.citation, source.source_type.display_name(), source.credibility
            ));
        }
        html.push_str("</ol>\n");
        
        html.push_str(&format!(
            "<p class=\"meta\">Oluşturulma: {} | Kelime: {} | Okuma süresi: {}dk</p>\n",
            report.metadata.generated_at.format("%Y-%m-%d %H:%M"),
            report.metadata.word_count,
            report.metadata.reading_time
        ));
        
        html.push_str("</body>\n</html>");
        html
    }

    fn render_json(&self, report: &ResearchReport) -> String {
        serde_json::to_string_pretty(report).unwrap_or_default()
    }

    fn render_plain(&self, report: &ResearchReport) -> String {
        let mut text = String::new();
        
        text.push_str(&format!("{}\n{}\n\n", report.title, "=".repeat(report.title.len())));
        text.push_str(&format!("ÖZET:\n{}\n\n", report.summary));
        
        for section in &report.sections {
            text.push_str(&format!("{}\n{}\n", section.title, "-".repeat(section.title.len())));
            if !section.content.is_empty() {
                text.push_str(&format!("{}\n", section.content));
            }
            
            for point in &section.key_points {
                text.push_str(&format!("• {}\n", point));
            }
            
            for subsection in &section.subsections {
                text.push_str(&format!("\n  {}\n", subsection.title));
                for point in &subsection.key_points {
                    text.push_str(&format!("    - {}\n", point));
                }
            }
            text.push_str("\n");
        }
        
        text.push_str("KAYNAKLAR:\n");
        for (i, source) in report.sources.iter().enumerate() {
            text.push_str(&format!("{}. {} ({:.0}%)\n", i + 1, source.citation, source.credibility));
        }
        
        text
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new(ReportFormat::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_section_default() {
        let section = ReportSection::default();
        assert!(!section.id.is_empty());
    }

    #[test]
    fn test_markdown_render() {
        let report = ResearchReport {
            title: "Test Report".to_string(),
            summary: "Test summary".to_string(),
            sections: vec![ReportSection {
                id: "test".to_string(),
                title: "Test Section".to_string(),
                content: "Test content".to_string(),
                key_points: vec!["Point 1".to_string()],
                ..Default::default()
            }],
            sources: vec![],
            metadata: ReportMetadata {
                word_count: 100,
                reading_time: 1,
                ..Default::default()
            },
        };
        
        let generator = ReportGenerator::new(ReportFormat::Markdown);
        let md = generator.render(&report);
        
        assert!(md.contains("# Test Report"));
        assert!(md.contains("Test summary"));
    }

    #[test]
    fn test_html_render() {
        let report = ResearchReport {
            title: "Test".to_string(),
            summary: "Summary".to_string(),
            sections: vec![],
            sources: vec![],
            metadata: ReportMetadata::default(),
        };
        
        let generator = ReportGenerator::new(ReportFormat::HTML);
        let html = generator.render(&report);
        
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>Test</title>"));
    }
}
