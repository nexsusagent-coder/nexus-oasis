//! ─── PRP (Product Requirements Prompt) Workflow ───
//!
//! Context engineering workflow:
//! - Define requirements as structured prompts
//! - Progressive context building
//! - Template-based PRP generation

use crate::{ContextError, ContextResult, ContextSection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// PRP Workflow
pub struct PrpWorkflow {
    templates: HashMap<String, PrpTemplate>,
}

impl PrpWorkflow {
    pub fn new() -> Self {
        Self {
            templates: Self::load_default_templates(),
        }
    }
    
    fn load_default_templates() -> HashMap<String, PrpTemplate> {
        let mut templates = HashMap::new();
        
        // Feature implementation template
        templates.insert("feature".into(), PrpTemplate {
            id: "feature".into(),
            name: "Feature Implementation".into(),
            sections: vec![
                PrpSectionTemplate {
                    name: "Goal".into(),
                    description: "What we want to achieve".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Context".into(),
                    description: "Existing codebase context".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Requirements".into(),
                    description: "Functional requirements".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Constraints".into(),
                    description: "Technical constraints".into(),
                    required: false,
                },
                PrpSectionTemplate {
                    name: "Examples".into(),
                    description: "Reference examples".into(),
                    required: false,
                },
            ],
        });
        
        // Bug fix template
        templates.insert("bugfix".into(), PrpTemplate {
            id: "bugfix".into(),
            name: "Bug Fix".into(),
            sections: vec![
                PrpSectionTemplate {
                    name: "Problem".into(),
                    description: "What's wrong".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Expected".into(),
                    description: "Expected behavior".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Actual".into(),
                    description: "Actual behavior".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Steps".into(),
                    description: "Steps to reproduce".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Context".into(),
                    description: "Relevant code context".into(),
                    required: false,
                },
            ],
        });
        
        // Refactoring template
        templates.insert("refactor".into(), PrpTemplate {
            id: "refactor".into(),
            name: "Code Refactoring".into(),
            sections: vec![
                PrpSectionTemplate {
                    name: "Current".into(),
                    description: "Current implementation".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Issues".into(),
                    description: "Problems with current code".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Target".into(),
                    description: "Desired structure".into(),
                    required: true,
                },
                PrpSectionTemplate {
                    name: "Constraints".into(),
                    description: "What must stay the same".into(),
                    required: false,
                },
            ],
        });
        
        templates
    }
    
    pub fn get_template(&self, id: &str) -> Option<&PrpTemplate> {
        self.templates.get(id)
    }
    
    pub fn list_templates(&self) -> Vec<&PrpTemplate> {
        self.templates.values().collect()
    }
    
    pub fn create_prp(&self, template_id: &str, values: HashMap<String, String>) -> ContextResult<ProductRequirementsPrompt> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| ContextError::NotFound(format!("Template not found: {}", template_id)))?;
        
        let mut sections = Vec::new();
        
        for section_tpl in &template.sections {
            let content = values.get(&section_tpl.name).cloned().unwrap_or_default();
            
            if section_tpl.required && content.is_empty() {
                return Err(ContextError::ParseError(format!("Required section '{}' is empty", section_tpl.name)));
            }
            
            if !content.is_empty() {
                sections.push(PrpSection {
                    name: section_tpl.name.clone(),
                    content,
                });
            }
        }
        
        Ok(ProductRequirementsPrompt {
            template_id: template_id.to_string(),
            name: template.name.clone(),
            sections,
        })
    }
}

impl Default for PrpWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

/// PRP Template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrpTemplate {
    pub id: String,
    pub name: String,
    pub sections: Vec<PrpSectionTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrpSectionTemplate {
    pub name: String,
    pub description: String,
    pub required: bool,
}

/// Product Requirements Prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRequirementsPrompt {
    pub template_id: String,
    pub name: String,
    pub sections: Vec<PrpSection>,
}

impl ProductRequirementsPrompt {
    /// Convert to a single prompt string
    pub fn to_prompt(&self) -> String {
        let mut prompt = format!("# {}\n\n", self.name);
        
        for section in &self.sections {
            prompt.push_str(&format!("## {}\n{}\n\n", section.name, section.content));
        }
        
        prompt
    }
    
    /// Convert to context sections
    pub fn to_context_sections(&self) -> Vec<ContextSection> {
        self.sections
            .iter()
            .map(|s| ContextSection::new(&s.name, &s.content))
            .collect()
    }
    
    /// Estimate total tokens
    pub fn estimate_tokens(&self) -> u32 {
        self.sections.iter().map(|s| crate::estimate_tokens(&s.content)).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrpSection {
    pub name: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prp_workflow() {
        let workflow = PrpWorkflow::new();
        assert!(!workflow.list_templates().is_empty());
    }
    
    #[test]
    fn test_create_prp() {
        let workflow = PrpWorkflow::new();
        let mut values = HashMap::new();
        values.insert("Goal".into(), "Add user authentication".into());
        values.insert("Context".into(), "Rust backend with Actix".into());
        values.insert("Requirements".into(), "JWT tokens, refresh flow".into());
        
        let prp = workflow.create_prp("feature", values).unwrap();
        assert_eq!(prp.sections.len(), 3);
    }
}
