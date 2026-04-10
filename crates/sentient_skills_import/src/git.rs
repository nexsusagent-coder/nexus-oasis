//! ─── Git Repository Skills ───

use crate::{Skill, SkillsError};

impl super::SkillsImporter {
    /// Install skill from Git repository
    pub async fn install_from_git(&self, repo_url: &str) -> Result<Skill, SkillsError> {
        // Clone repository
        let skill_id = extract_skill_id_from_url(repo_url);
        let skill_id_clone = skill_id.clone();
        
        let output = tokio::process::Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(repo_url)
            .arg(&skill_id)
            .output()
            .await
            .map_err(|e| SkillsError::Network(format!("Git clone failed: {}", e)))?;
        
        if !output.status.success() {
            return Err(SkillsError::Install(
                String::from_utf8_lossy(&output.stderr).into_owned()
            ));
        }
        
        // Read manifest
        let manifest_path = format!("{}/skill.yaml", skill_id);
        let manifest_content = tokio::fs::read_to_string(&manifest_path)
            .await
            .map_err(|e| SkillsError::Install(format!("No skill.yaml: {}", e)))?;
        
        let manifest: crate::skill::SkillManifest = serde_yaml::from_str(&manifest_content)?;
        
        Ok(Skill {
            id: skill_id_clone,
            manifest,
            metadata: crate::skill::SkillMetadata {
                category: "custom".into(),
                tags: vec!["git".into()],
                rating: None,
                downloads: None,
                created_at: None,
                updated_at: None,
            },
            source: crate::skill::SkillSource::Git,
            installed: true,
            local_path: Some(skill_id),
        })
    }
}

fn extract_skill_id_from_url(url: &str) -> String {
    url.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("skill")
        .trim_start_matches("skill-")
        .to_string()
}
