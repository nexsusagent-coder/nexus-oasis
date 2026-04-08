//! ─── REPORT TEMPLATES ───
//!
//! Hazır rapor şablonları

use crate::{ReportError, ReportResult, ResearchReport, ReportSection};
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;

/// Rapor şablonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub name: String,
    pub description: String,
    pub sections: Vec<TemplateSection>,
}

impl ReportTemplate {
    pub fn render(&self, data: &serde_json::Value) -> ReportResult<ResearchReport> {
        let mut report = ResearchReport::new(&self.name);
        
        for section in &self.sections {
            let content = self.render_content(&section.template, data)?;
            report.add_section(ReportSection::new(&section.title, content));
        }
        
        Ok(report)
    }
    
    fn render_content(&self, template: &str, data: &serde_json::Value) -> ReportResult<String> {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        
        handlebars.render_template(template, data)
            .map_err(|e| ReportError::TemplateError(e.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    pub title: String,
    pub template: String,
    pub required: bool,
}

/// Şablon kütüphanesi
#[derive(Debug, Clone, Default)]
pub struct TemplateLibrary {
    templates: std::collections::HashMap<String, ReportTemplate>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        let mut templates = std::collections::HashMap::new();
        
        // Araştırma raporu şablonu
        templates.insert("research".into(), ReportTemplate {
            name: "Araştırma Raporu".into(),
            description: "Standart araştırma raporu formatı".into(),
            sections: vec![
                TemplateSection {
                    title: "Giriş".into(),
                    template: "{{intro}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Metodoloji".into(),
                    template: "{{methodology}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Bulgular".into(),
                    template: "{{findings}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Tartışma".into(),
                    template: "{{discussion}}".into(),
                    required: false,
                },
                TemplateSection {
                    title: "Sonuç".into(),
                    template: "{{conclusion}}".into(),
                    required: true,
                },
            ],
        });
        
        // Teknik rapor şablonu
        templates.insert("technical".into(), ReportTemplate {
            name: "Teknik Rapor".into(),
            description: "Teknik dokümantasyon formatı".into(),
            sections: vec![
                TemplateSection {
                    title: "Özet".into(),
                    template: "{{summary}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Arka Plan".into(),
                    template: "{{background}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Teknik Detaylar".into(),
                    template: "{{technical_details}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Uygulama".into(),
                    template: "{{implementation}}".into(),
                    required: false,
                },
            ],
        });
        
        // Hızlı özet şablonu
        templates.insert("summary".into(), ReportTemplate {
            name: "Hızlı Özet".into(),
            description: "Kısa özet raporu".into(),
            sections: vec![
                TemplateSection {
                    title: "Ana Bulgular".into(),
                    template: "{{findings}}".into(),
                    required: true,
                },
                TemplateSection {
                    title: "Öneriler".into(),
                    template: "{{recommendations}}".into(),
                    required: false,
                },
            ],
        });
        
        Self { templates }
    }
    
    pub fn get(&self, name: &str) -> ReportResult<ReportTemplate> {
        self.templates.get(name).cloned()
            .ok_or_else(|| ReportError::TemplateNotFound(name.into()))
    }
    
    pub fn list(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}
