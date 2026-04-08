//! ═══════════════════════════════════════════════════════════════════════════════
//!  CodeGen Skill - Kod Üretimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{Skill, SkillInput, SkillOutput, Artifact, ArtifactType};
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kod üreten skill
pub struct CodeGenSkill {
    id: Uuid,
    config: CodeGenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenConfig {
    pub languages: Vec<String>,
    pub max_lines: usize,
    pub include_comments: bool,
    pub format_output: bool,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            languages: vec!["python".into(), "rust".into(), "javascript".into()],
            max_lines: 500,
            include_comments: true,
            format_output: true,
        }
    }
}

impl CodeGenSkill {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            config: CodeGenConfig::default(),
        }
    }
}

impl Skill for CodeGenSkill {
    fn id(&self) -> Uuid { self.id }
    fn name(&self) -> &str { "codegen" }
    fn description(&self) -> &str { "Kod üretimi ve programlama yardımı" }
    fn version(&self) -> &str { "0.1.0" }
    
    fn execute(&self, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        // Gerçek impl'de V-GATE + OpenManus Sandbox kullanılır
        let code = format!("// Generated code for: {}\nfn main() {{\n    println!(\"Hello!\");\n}}", input.query);
        
        Ok(SkillOutput::success("Kod üretildi")
            .with_artifact(Artifact {
                name: "generated.rs".to_string(),
                artifact_type: ArtifactType::Code,
                content: code,
                mime_type: Some("text/x-rust".to_string()),
            }))
    }
    
    fn load_config(&mut self, _path: &std::path::Path) -> SENTIENTResult<()> {
        Ok(())
    }
}

impl Default for CodeGenSkill {
    fn default() -> Self { Self::new() }
}
