//! ─── SENTIENT Skills Importer Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::{Skill, SkillManifest, SkillMetadata, SkillSource};

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new("test-skill", "Test Skill")
            .with_version("1.0.0")
            .with_description("A test skill")
            .with_author("test")
            .with_category("test")
            .with_tag("testing");
        
        assert_eq!(skill.id, "test-skill");
        assert_eq!(skill.manifest.name, "Test Skill");
        assert_eq!(skill.manifest.version, "1.0.0");
        assert_eq!(skill.manifest.description, "A test skill");
        assert_eq!(skill.metadata.category, "test");
        assert!(skill.metadata.tags.contains(&"testing".to_string()));
    }

    #[test]
    fn test_skill_display() {
        let skill = Skill::new("test", "Test")
            .with_version("1.0.0")
            .with_description("Test skill")
            .with_author("author");
        
        let display = format!("{}", skill);
        assert!(display.contains("Test"));
        assert!(display.contains("1.0.0"));
        assert!(display.contains("author"));
    }

    #[test]
    fn test_manifest_yaml() {
        let yaml = crate::skill::EXAMPLE_MANIFEST;
        let manifest: SkillManifest = serde_yaml::from_str(yaml).expect("operation failed");
        
        assert_eq!(manifest.name, "translator");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.author, "sentient");
        assert!(manifest.dependencies.contains(&"@translate/core".to_string()));
    }
}
