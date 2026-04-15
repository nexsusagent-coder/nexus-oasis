//! ─── Context Builder ───
//!
//! Build context for AI interactions with:
//! - Priority-based section ordering
//! - Token budget management
//! - File inclusion

use crate::{ContextConfig, ContextError, ContextResult, ContextSection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Context builder
pub struct ContextBuilder {
    config: ContextConfig,
    sections: Vec<ContextSection>,
    files: Vec<PathBuf>,
    variables: HashMap<String, String>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            config: ContextConfig::default(),
            sections: Vec::new(),
            files: Vec::new(),
            variables: HashMap::new(),
        }
    }
    
    pub fn with_config(mut self, config: ContextConfig) -> Self {
        self.config = config;
        self
    }
    
    pub fn section(mut self, section: ContextSection) -> Self {
        self.sections.push(section);
        self
    }
    
    pub fn add_section(mut self, name: &str, content: &str) -> Self {
        self.sections.push(ContextSection::new(name, content));
        self
    }
    
    pub fn file(mut self, path: PathBuf) -> Self {
        self.files.push(path);
        self
    }
    
    pub fn variable(mut self, key: &str, value: &str) -> Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Build the final context
    pub async fn build(&self) -> ContextResult<BuiltContext> {
        let mut all_sections = self.sections.clone();
        
        // Load file contents
        for file_path in &self.files {
            if file_path.exists() {
                let content = tokio::fs::read_to_string(file_path).await?;
                let name = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("file");
                all_sections.push(ContextSection::new(name, &content));
            }
        }
        
        // Sort by priority (higher first)
        all_sections.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // Apply token budget
        let max_content_tokens = self.config.max_tokens - self.config.reserved_output_tokens;
        let mut total_tokens = 0u32;
        let mut final_sections = Vec::new();
        
        for section in all_sections {
            if total_tokens + section.token_count <= max_content_tokens {
                total_tokens += section.token_count;
                final_sections.push(section);
            } else {
                // Try to truncate
                let remaining = max_content_tokens - total_tokens;
                if remaining > 100 {
                    let truncated = truncate_section(&section, remaining);
                    final_sections.push(truncated);
                    break;
                }
            }
        }
        
        Ok(BuiltContext {
            sections: final_sections,
            total_tokens,
            config: self.config.clone(),
        })
    }
    
    /// Build as a single prompt string
    pub async fn build_prompt(&self) -> ContextResult<String> {
        let built = self.build().await?;
        Ok(built.to_prompt())
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Built context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltContext {
    pub sections: Vec<ContextSection>,
    pub total_tokens: u32,
    pub config: ContextConfig,
}

impl BuiltContext {
    pub fn to_prompt(&self) -> String {
        let mut prompt = String::new();
        
        for section in &self.sections {
            prompt.push_str(&format!("# {}\n\n{}\n\n", section.name, section.content));
        }
        
        prompt
    }
    
    pub fn get_section(&self, name: &str) -> Option<&ContextSection> {
        self.sections.iter().find(|s| s.name == name)
    }
}

/// Truncate a section to fit token budget
fn truncate_section(section: &ContextSection, max_tokens: u32) -> ContextSection {
    let max_chars = (max_tokens * 4) as usize;
    let truncated_content = if section.content.len() > max_chars {
        format!("{}...\n[truncated]", &section.content[..max_chars])
    } else {
        section.content.clone()
    };
    
    ContextSection {
        name: section.name.clone(),
        content: truncated_content,
        token_count: max_tokens,
        priority: section.priority,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_context_builder() {
        let ctx = ContextBuilder::new()
            .add_section("Test", "Hello world")
            .build()
            .await
            .unwrap();
        
        assert_eq!(ctx.sections.len(), 1);
    }
}
