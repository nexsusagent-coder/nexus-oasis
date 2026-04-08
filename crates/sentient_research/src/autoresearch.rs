//! AutoResearch PyO3 Wrapper
//! Python AutoResearch modülünün Rust'a native entegrasyonu

use crate::error::{ResearchError, ResearchResult};
use crate::graph::SearchGraph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AutoResearch Wrapper
pub struct AutoResearchWrapper {
    /// Yapılandırma
    config: AutoResearchConfig,
}

/// AutoResearch yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoResearchConfig {
    /// Maksimum araştırma derinliği
    pub max_depth: u32,
    /// Her dalda maksimum alt başlık
    pub max_subtopics: u32,
    /// Kaynak başına maksimum alıntı
    pub max_citations_per_source: u32,
    /// Çıktı formatı
    pub output_format: OutputFormat,
    /// Dil
    pub language: String,
}

impl Default for AutoResearchConfig {
    fn default() -> Self {
        Self {
            max_depth: 3,
            max_subtopics: 5,
            max_citations_per_source: 3,
            output_format: OutputFormat::Markdown,
            language: "tr".into(),
        }
    }
}

/// Çıktı formatı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    HTML,
    JSON,
    PDF,
    LaTeX,
}

/// Araştırma planı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchPlan {
    /// Plan ID
    pub id: String,
    /// Ana konu
    pub topic: String,
    /// Alt konular
    pub subtopics: Vec<Subtopic>,
    /// Kaynaklar
    pub sources: Vec<Source>,
    /// Oluşturulma zamanı
    pub created_at: String,
    /// Durum
    pub status: PlanStatus,
}

/// Alt konu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtopic {
    /// Başlık
    pub title: String,
    /// Sorular
    pub questions: Vec<String>,
    /// Anahtar kelimeler
    pub keywords: Vec<String>,
    /// Derinlik seviyesi
    pub depth: u32,
}

/// Kaynak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// URL
    pub url: String,
    /// Başlık
    pub title: String,
    /// Tür
    pub source_type: SourceType,
    /// Güvenilirlik skoru
    pub credibility_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Academic,
    News,
    Blog,
    Documentation,
    Government,
    Wikipedia,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanStatus {
    Draft,
    Researching,
    Completed,
    Failed,
}

impl AutoResearchWrapper {
    /// Yeni AutoResearch wrapper oluştur (native)
    pub fn new_fallback() -> Self {
        log::info!("🔬 AUTORESEARCH: Native wrapper oluşturuluyor...");
        Self {
            config: AutoResearchConfig::default(),
        }
    }
    
    /// Araştırma planı oluştur
    pub async fn research(&mut self, topic: &str) -> ResearchResult<ResearchPlan> {
        log::info!("🔬 AUTORESEARCH: Plan oluşturuluyor → {}", topic.chars().take(50).collect::<String>());
        
        // Konuyu doğrula
        self.validate_topic(topic)?;
        
        // Plan oluştur
        let plan = self.create_plan(topic)?;
        
        Ok(plan)
    }
    
    /// Araştırma planı oluştur (detaylı)
    fn create_plan(&self, topic: &str) -> ResearchResult<ResearchPlan> {
        let plan_id = uuid::Uuid::new_v4().to_string();
        
        // Alt konular oluştur
        let subtopics = self.generate_subtopics(topic)?;
        
        // Kaynaklar ara
        let sources = self.search_sources(topic)?;
        
        Ok(ResearchPlan {
            id: plan_id,
            topic: topic.to_string(),
            subtopics,
            sources,
            created_at: chrono::Utc::now().to_rfc3339(),
            status: PlanStatus::Draft,
        })
    }
    
    /// Alt konular oluştur
    fn generate_subtopics(&self, topic: &str) -> ResearchResult<Vec<Subtopic>> {
        let subtopics = vec![
            Subtopic {
                title: format!("{} nedir?", topic),
                questions: vec![
                    format!("{} tanımı nedir?", topic),
                    format!("{} tarihçesi nedir?", topic),
                ],
                keywords: vec!["tanım".into(), "tarihçe".into(), "giriş".into()],
                depth: 1,
            },
            Subtopic {
                title: format!("{} bileşenleri", topic),
                questions: vec![
                    format!("{} ana bileşenleri nelerdir?", topic),
                    format!("{} nasıl çalışır?", topic),
                ],
                keywords: vec!["bileşenler".into(), "çalışma prensibi".into(), "mimarisi".into()],
                depth: 2,
            },
            Subtopic {
                title: format!("{} uygulamaları", topic),
                questions: vec![
                    format!("{} nerede kullanılır?", topic),
                    format!("{} örnekleri nelerdir?", topic),
                ],
                keywords: vec!["uygulama".into(), "örnek".into(), "kullanım".into()],
                depth: 2,
            },
        ];
        
        Ok(subtopics)
    }
    
    /// Kaynakları ara
    fn search_sources(&self, topic: &str) -> ResearchResult<Vec<Source>> {
        let sources = vec![
            Source {
                url: format!("https://wikipedia.org/wiki/{}", topic.replace(' ', "_")),
                title: format!("{} - Wikipedia", topic),
                source_type: SourceType::Wikipedia,
                credibility_score: 0.8,
            },
            Source {
                url: format!("https://scholar.google.com/search?q={}", urlencoding::encode(topic)),
                title: format!("{} - Akademik Kaynaklar", topic),
                source_type: SourceType::Academic,
                credibility_score: 0.95,
            },
        ];
        
        Ok(sources)
    }
    
    /// Konu doğrulama
    fn validate_topic(&self, topic: &str) -> ResearchResult<()> {
        if topic.trim().is_empty() {
            return Err(ResearchError::InvalidQuery {
                reason: "Konu boş olamaz".into()
            });
        }
        
        if topic.len() > 500 {
            return Err(ResearchError::InvalidQuery {
                reason: "Konu çok uzun (max 500 karakter)".into()
            });
        }
        
        Ok(())
    }
    
    /// Planı araştır (araştırma yap)
    pub async fn execute_plan(&mut self) -> ResearchResult<SearchGraph> {
        // Native implementasyon
        let mut graph = SearchGraph::new("Research Plan Execution");
        let root_id = graph.root_id.clone();
        
        // Simüle edilmiş node'lar
        for i in 0..3 {
            let node_id = graph.add_node(&root_id, &format!("Research node {}", i + 1));
            graph.set_response(&node_id, &format!("Result {}", i + 1));
        }
        
        Ok(graph)
    }
    
    /// Kapat
    pub fn close(&mut self) -> ResearchResult<()> {
        log::info!("🔬 AUTORESEARCH: Kapatıldı");
        Ok(())
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_autoresearch_config_default() {
        let config = AutoResearchConfig::default();
        assert_eq!(config.max_depth, 3);
        assert_eq!(config.max_subtopics, 5);
    }
    
    #[test]
    fn test_plan_creation() {
        let plan = ResearchPlan {
            id: "test-plan".into(),
            topic: "Test Topic".into(),
            subtopics: vec![],
            sources: vec![],
            created_at: chrono::Utc::now().to_rfc3339(),
            status: PlanStatus::Draft,
        };
        
        assert_eq!(plan.status, PlanStatus::Draft);
    }
    
    #[test]
    fn test_subtopic_creation() {
        let subtopic = Subtopic {
            title: "Test Subtopic".into(),
            questions: vec!["Question 1?".into()],
            keywords: vec!["keyword".into()],
            depth: 1,
        };
        
        assert_eq!(subtopic.depth, 1);
        assert_eq!(subtopic.questions.len(), 1);
    }
    
    #[tokio::test]
    async fn test_research() {
        let mut wrapper = AutoResearchWrapper::new_fallback();
        let result = wrapper.research("test topic").await;
        
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.status, PlanStatus::Draft);
    }
}

// mod urlencoding {
//     pub fn encode(s: &str) -> String {
//         urlencoding::encode(s).into_owned()
//     }
// }
