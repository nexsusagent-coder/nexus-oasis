//! ─── AGENTS.md Standard Support ───
//!
//! Parse and use AGENTS.md files for context:
//! - Project-specific AI instructions
//! - Coding standards
//! - Architecture guidelines

use crate::{ContextError, ContextResult, ContextSection};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// AGENTS.md content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsMd {
    pub path: PathBuf,
    pub sections: Vec<AgentsSection>,
    pub raw_content: String,
}

impl AgentsMd {
    /// Load AGENTS.md from a directory
    pub async fn load(dir: &Path) -> ContextResult<Option<Self>> {
        let agents_path = dir.join("AGENTS.md");
        
        if !agents_path.exists() {
            return Ok(None);
        }
        
        let content = tokio::fs::read_to_string(&agents_path).await?;
        let sections = Self::parse_sections(&content);
        
        Ok(Some(Self {
            path: agents_path,
            sections,
            raw_content: content,
        }))
    }
    
    /// Parse AGENTS.md into sections
    fn parse_sections(content: &str) -> Vec<AgentsSection> {
        let mut sections = Vec::new();
        let mut current_section: Option<AgentsSection> = None;
        
        for line in content.lines() {
            // Check for markdown headings
            if line.starts_with("# ") {
                // Push previous section
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(AgentsSection {
                    title: line.strip_prefix("# ").unwrap_or(line).to_string(),
                    content: String::new(),
                    level: 1,
                });
            } else if line.starts_with("## ") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(AgentsSection {
                    title: line.strip_prefix("## ").unwrap_or(line).to_string(),
                    content: String::new(),
                    level: 2,
                });
            } else if line.starts_with("### ") {
                if let Some(section) = current_section.take() {
                    sections.push(section);
                }
                current_section = Some(AgentsSection {
                    title: line.strip_prefix("### ").unwrap_or(line).to_string(),
                    content: String::new(),
                    level: 3,
                });
            } else if let Some(ref mut section) = current_section {
                if !section.content.is_empty() || !line.trim().is_empty() {
                    section.content.push_str(line);
                    section.content.push('\n');
                }
            }
        }
        
        // Push last section
        if let Some(section) = current_section {
            sections.push(section);
        }
        
        sections
    }
    
    /// Get section by title
    pub fn get_section(&self, title: &str) -> Option<&AgentsSection> {
        self.sections.iter().find(|s| s.title == title)
    }
    
    /// Convert to context sections
    pub fn to_context_sections(&self) -> Vec<ContextSection> {
        self.sections
            .iter()
            .map(|s| {
                let mut ctx = ContextSection::new(&s.title, &s.content);
                ctx.priority = match s.level {
                    1 => 10,
                    2 => 7,
                    _ => 5,
                };
                ctx
            })
            .collect()
    }
    
    /// Create default AGENTS.md template
    pub fn create_template() -> String {
        r#"# Project Context

## Overview
Brief description of the project and its goals.

## Architecture
High-level architecture decisions and patterns.

## Coding Standards
- Code style guidelines
- Naming conventions
- File organization

## Key Files
Important files and their purposes.

## Commands
Common development commands:
- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`

## Notes
Any additional context for AI assistants.
"#.to_string()
    }
}

/// A section in AGENTS.md
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsSection {
    pub title: String,
    pub content: String,
    pub level: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_sections() {
        let content = "# Title\n\nContent here\n\n## Subtitle\n\nMore content";
        let sections = AgentsMd::parse_sections(content);
        assert_eq!(sections.len(), 2);
    }
}
