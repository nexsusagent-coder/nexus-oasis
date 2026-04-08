//! ─── CIKTI FORMATLARI ───

use crate::{GeneratedTool, ToolType};
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};

/// Cikti formati
pub trait OutputFormatter: Send + Sync {
    /// Formatla
    fn format(&self, tool: &GeneratedTool) -> SENTIENTResult<String>;
    
    /// Dosya uzantisi
    fn extension(&self) -> &'static str;
}

/// JSON Formatter
pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl OutputFormatter for JsonFormatter {
    fn format(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        if self.pretty {
            serde_json::to_string_pretty(tool).map_err(|e| e.into())
        } else {
            serde_json::to_string(tool).map_err(|e| e.into())
        }
    }
    
    fn extension(&self) -> &'static str {
        "json"
    }
}

/// Python Formatter
pub struct PythonFormatter {
    add_docstring: bool,
    add_type_hints: bool,
}

impl PythonFormatter {
    pub fn new() -> Self {
        Self {
            add_docstring: true,
            add_type_hints: true,
        }
    }
}

impl OutputFormatter for PythonFormatter {
    fn format(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        // Kod zaten uretilmis, sadece ek bilgileri ekle
        Ok(tool.code.clone())
    }
    
    fn extension(&self) -> &'static str {
        "py"
    }
}

impl Default for PythonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// YAML Formatter
pub struct YamlFormatter;

impl YamlFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl OutputFormatter for YamlFormatter {
    fn format(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        // YAML formati
        let frontmatter = format!("# {} - SENTIENT Forge\n# Generated: {}\n", 
            tool.name, tool.generated_at.format("%Y-%m-%d %H:%M:%S UTC"));
        
        Ok(format!("{}{}", frontmatter, tool.code))
    }
    
    fn extension(&self) -> &'static str {
        "yml"
    }
}

impl Default for YamlFormatter {
    fn default() -> Self {
        Self::new()
    }
}

/// Markdown dokumantasyon
pub struct MarkdownFormatter {
    include_code: bool,
}

impl MarkdownFormatter {
    pub fn new(include_code: bool) -> Self {
        Self { include_code }
    }
}

impl OutputFormatter for MarkdownFormatter {
    fn format(&self, tool: &GeneratedTool) -> SENTIENTResult<String> {
        let mut md = String::new();
        
        md.push_str(&format!("# {}\n\n", tool.name));
        md.push_str(&format!("**Tur**: {:?}\n\n", tool.tool_type));
        md.push_str(&format!("**Uretilme**: {}\n\n", tool.generated_at.format("%Y-%m-%d %H:%M:%S UTC")));
        md.push_str(&format!("**Kaynak**: {}\n\n", tool.source_summary));
        
        if self.include_code {
            md.push_str("## Kod\n\n");
            let ext = match tool.tool_type {
                ToolType::PythonScript => "python",
                ToolType::N8nWorkflow => "json",
                ToolType::GitHubAction => "yaml",
                ToolType::DockerCompose => "yaml",
                ToolType::NodeModule => "javascript",
                ToolType::ShellScript => "bash",
            };
            md.push_str(&format!("```{}\n{}\n```\n", ext, tool.code));
        }
        
        if let Some(ref validation) = tool.validation_result {
            md.push_str(&format!("\n## Validation\n\n- **Gecerli**: {}\n- **Skor**: {}/100\n",
                validation.valid, validation.score));
        }
        
        Ok(md)
    }
    
    fn extension(&self) -> &'static str {
        "md"
    }
}

/// Factory
pub fn create_formatter(tool_type: &ToolType) -> Box<dyn OutputFormatter> {
    match tool_type {
        ToolType::N8nWorkflow => Box::new(JsonFormatter::new(true)),
        ToolType::PythonScript => Box::new(PythonFormatter::new()),
        ToolType::GitHubAction => Box::new(YamlFormatter::new()),
        ToolType::DockerCompose => Box::new(YamlFormatter::new()),
        _ => Box::new(JsonFormatter::new(true)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    fn create_test_tool() -> GeneratedTool {
        GeneratedTool {
            id: uuid::Uuid::new_v4(),
            name: "test".into(),
            tool_type: ToolType::PythonScript,
            source_summary: "test".into(),
            code: "print('hello')".into(),
            metadata: HashMap::new(),
            generated_at: chrono::Utc::now(),
            validation_result: None,
        }
    }
    
    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter::new(true);
        let tool = create_test_tool();
        let output = formatter.format(&tool).unwrap();
        assert!(output.contains("test"));
    }
    
    #[test]
    fn test_markdown_formatter() {
        let formatter = MarkdownFormatter::new(true);
        let tool = create_test_tool();
        let output = formatter.format(&tool).unwrap();
        assert!(output.contains("# test"));
    }
}
