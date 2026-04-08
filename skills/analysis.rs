//! ═══════════════════════════════════════════════════════════════════════════════
//!  Analysis Skill - Veri Analizi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Skill, SkillInput, SkillOutput, Artifact, ArtifactType};
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Veri analizi yapan skill
pub struct AnalysisSkill {
    id: Uuid,
    config: AnalysisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub max_dataset_size: usize,
    pub output_formats: Vec<String>,
    pub visualization_enabled: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_dataset_size: 100_000,
            output_formats: vec!["json".into(), "csv".into()],
            visualization_enabled: true,
        }
    }
}

impl AnalysisSkill {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            config: AnalysisConfig::default(),
        }
    }
}

impl Skill for AnalysisSkill {
    fn id(&self) -> Uuid { self.id }
    fn name(&self) -> &str { "analysis" }
    fn description(&self) -> &str { "Veri analizi ve görselleştirme" }
    fn version(&self) -> &str { "0.1.0" }
    
    fn execute(&self, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        // Gerçek impl'de Pandas/Polars + Matplotlib kullanılır
        let analysis = format!("{{\"query\": \"{}\", \"result\": \"Analysis complete\"}}", input.query);
        
        Ok(SkillOutput::success("Analiz tamamlandı")
            .with_artifact(Artifact {
                name: "analysis.json".to_string(),
                artifact_type: ArtifactType::Data,
                content: analysis,
                mime_type: Some("application/json".to_string()),
            }))
    }
    
    fn load_config(&mut self, _path: &std::path::Path) -> SENTIENTResult<()> {
        Ok(())
    }
}

impl Default for AnalysisSkill {
    fn default() -> Self { Self::new() }
}
