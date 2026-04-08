//! ═══════════════════════════════════════════════════════════════════════════════
//!  SKILL PARSER - OpenClaw Skill Parse Engine
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{IngestorError, IngestorResult};
use crate::unified_yaml::UnifiedSkill;
use crate::categories::SkillCategory;
use regex::Regex;

/// Parse edilmiş skill verisi
#[derive(Debug, Clone)]
pub struct ParsedSkill {
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: SkillCategory,
    pub url: Option<String>,
    pub github_url: Option<String>,
    pub author: Option<String>,
    pub tags: Vec<String>,
}

/// Skill Parser
pub struct SkillParser {
    skill_pattern: Regex,
    slug_pattern: Regex,
}

impl SkillParser {
    pub fn new() -> IngestorResult<Self> {
        Ok(Self {
            skill_pattern: Regex::new(r"- \[([^\]]+)\]\(([^)]+)\)\s*-\s*(.+)$")
                .map_err(|e| IngestorError::RegexError(e))?,
            slug_pattern: Regex::new(r"skills/([a-zA-Z0-9_-]+)")
                .map_err(|e| IngestorError::RegexError(e))?,
        })
    }
    
    /// Markdown satırından skill parse et
    pub fn parse_markdown_line(&self, line: &str, category: &SkillCategory) -> IngestorResult<Option<ParsedSkill>> {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') || line.starts_with('>') {
            return Ok(None);
        }
        
        if let Some(caps) = self.skill_pattern.captures(line) {
            let name = caps.get(1)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            
            let url = caps.get(2)
                .map(|m| m.as_str().to_string());
            
            let description = caps.get(3)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();
            
            let slug = url.as_ref()
                .and_then(|u| self.slug_pattern.captures(u))
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| name.to_lowercase().replace(" ", "-"));
            
            let author = slug.split('-')
                .next()
                .map(|s| s.to_string());
            
            if name.is_empty() || description.is_empty() {
                return Ok(None);
            }
            
            return Ok(Some(ParsedSkill {
                name,
                slug,
                description,
                category: category.clone(),
                url,
                github_url: None,
                author,
                tags: vec![category.to_string()],
            }));
        }
        
        Ok(None)
    }
    
    /// Markdown dosyasındaki tüm skill'leri parse et
    pub fn parse_markdown_file(&self, content: &str, category: &SkillCategory) -> IngestorResult<Vec<ParsedSkill>> {
        let mut skills = Vec::new();
        
        for line in content.lines() {
            if let Some(skill) = self.parse_markdown_line(line, category)? {
                skills.push(skill);
            }
        }
        
        Ok(skills)
    }
    
    /// ParsedSkill'i UnifiedSkill'e dönüştür
    pub fn to_unified(&self, parsed: &ParsedSkill) -> UnifiedSkill {
        let mut skill = UnifiedSkill::new(&parsed.name, &parsed.description, &parsed.category.to_string());
        
        skill.slug = parsed.slug.clone();
        skill.source_url = parsed.url.clone();
        skill.github_url = parsed.github_url.clone();
        skill.metadata.author = parsed.author.clone();
        skill.metadata.estimated_frequency = self.estimate_frequency(&parsed.description);
        skill.metadata.risk_level = self.assess_risk(&parsed.description).to_string();
        
        skill.sentient_extensions.requires_network = parsed.description.to_lowercase().contains("api")
            || parsed.description.to_lowercase().contains("web")
            || parsed.description.to_lowercase().contains("internet");
        
        skill.sentient_extensions.requires_browser = parsed.description.to_lowercase().contains("browser")
            || parsed.description.to_lowercase().contains("web")
            || parsed.description.to_lowercase().contains("scrape");
        
        skill.sentient_extensions.requires_filesystem = parsed.description.to_lowercase().contains("file")
            || parsed.description.to_lowercase().contains("read")
            || parsed.description.to_lowercase().contains("write");
        
        skill.tags = parsed.tags.clone();
        
        skill
    }
    
    fn estimate_frequency(&self, description: &str) -> u8 {
        let desc = description.to_lowercase();
        
        if desc.contains("daily") || desc.contains("automatic") || desc.contains("real-time") {
            return 9;
        }
        if desc.contains("manage") || desc.contains("automate") || desc.contains("workflow") {
            return 7;
        }
        if desc.contains("help") || desc.contains("assist") || desc.contains("create") {
            return 5;
        }
        if desc.contains("specific") || desc.contains("specialized") || desc.contains("rare") {
            return 3;
        }
        5
    }
    
    fn assess_risk(&self, description: &str) -> &'static str {
        let desc = description.to_lowercase();
        
        if desc.contains("delete") || desc.contains("remove") || desc.contains("destructive")
            || desc.contains("security") || desc.contains("credential") || desc.contains("password") {
            return "high";
        }
        if desc.contains("write") || desc.contains("modify") || desc.contains("update")
            || desc.contains("api") || desc.contains("external") {
            return "medium";
        }
        "low"
    }
}

impl Default for SkillParser {
    fn default() -> Self {
        Self::new().expect("Failed to create SkillParser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_markdown_line() {
        let parser = SkillParser::new().unwrap();
        let line = "- [github-pr-manager](https://clawskills.sh/skills/user-github-pr-manager) - Manage GitHub PRs automatically";
        
        let result = parser.parse_markdown_line(line, &SkillCategory::GitGithub).unwrap();
        
        assert!(result.is_some());
        let skill = result.unwrap();
        assert_eq!(skill.name, "github-pr-manager");
    }
}
