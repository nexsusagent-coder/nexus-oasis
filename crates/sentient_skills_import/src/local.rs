//! ─── Local Directory Skills ───

use crate::{Skill, SkillsError};

impl super::SkillsImporter {
    /// Load skill from local directory
    pub fn load_local(&self, path: &str) -> Result<Skill, SkillsError> {
        let skill_path = std::path::Path::new(path);
        
        // Check for manifest
        let manifest_path = skill_path.join("skill.yaml");
        if !manifest_path.exists() {
            return Err(SkillsError::InvalidManifest(
                "No skill.yaml found".into()
            ));
        }
        
        // Read manifest
        let manifest_content = std::fs::read_to_string(manifest_path)?;
        let manifest: crate::skill::SkillManifest = serde_yaml::from_str(&manifest_content)?;
        
        // Create skill ID from directory name
        let skill_id = skill_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| manifest.name.to_lowercase().replace(' ', "-"));
        
        Ok(Skill {
            id: skill_id,
            manifest,
            metadata: crate::skill::SkillMetadata {
                category: "local".into(),
                tags: vec!["local".into()],
                rating: None,
                downloads: None,
                created_at: None,
                updated_at: None,
            },
            source: crate::skill::SkillSource::Local,
            installed: true,
            local_path: Some(path.into()),
        })
    }
    
    /// Scan directory for skills
    pub fn scan_directory(&self, dir: &str) -> Result<Vec<Skill>, SkillsError> {
        let mut skills = Vec::new();
        let path = std::path::Path::new(dir);
        
        if !path.exists() {
            return Ok(skills);
        }
        
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let skill_path = entry.path();
            
            if skill_path.is_dir() {
                let manifest = skill_path.join("skill.yaml");
                if manifest.exists() {
                    if let Ok(skill) = self.load_local(&skill_path.to_string_lossy()) {
                        skills.push(skill);
                    }
                }
            }
        }
        
        Ok(skills)
    }
}
