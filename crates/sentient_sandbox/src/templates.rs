// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Sandbox Templates
// ═══════════════════════════════════════════════════════════════════════════════
//  Pre-configured environments for different languages and use cases
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Template ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Template ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Language
    pub language: TemplateLanguage,
    /// Docker image
    pub docker_image: Option<String>,
    /// Pre-installed packages
    pub packages: Vec<String>,
}

impl Template {
    /// Create a new template
    pub fn new(id: impl Into<String>, name: impl Into<String>, language: TemplateLanguage) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            language,
            docker_image: None,
            packages: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set docker image
    pub fn with_docker_image(mut self, image: impl Into<String>) -> Self {
        self.docker_image = Some(image.into());
        self
    }

    /// Add packages
    pub fn with_packages(mut self, packages: Vec<String>) -> Self {
        self.packages = packages;
        self
    }
}

/// Template language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateLanguage {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Go,
    Java,
    Cpp,
    Bash,
    Base,
    Custom,
}

impl TemplateLanguage {
    /// Get default file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::TypeScript => "ts",
            Self::Rust => "rs",
            Self::Go => "go",
            Self::Java => "java",
            Self::Cpp => "cpp",
            Self::Bash => "sh",
            Self::Base => "txt",
            Self::Custom => "txt",
        }
    }

    /// Get default entry point
    pub fn entrypoint(&self) -> &'static str {
        match self {
            Self::Python => "main.py",
            Self::JavaScript => "index.js",
            Self::TypeScript => "index.ts",
            Self::Rust => "main.rs",
            Self::Go => "main.go",
            Self::Java => "Main.java",
            Self::Cpp => "main.cpp",
            Self::Bash => "script.sh",
            Self::Base => "file.txt",
            Self::Custom => "file.txt",
        }
    }

    /// Get run command
    pub fn run_command(&self) -> &'static str {
        match self {
            Self::Python => "python main.py",
            Self::JavaScript => "node index.js",
            Self::TypeScript => "npx tsx index.ts",
            Self::Rust => "cargo run",
            Self::Go => "go run main.go",
            Self::Java => "java Main",
            Self::Cpp => "./main",
            Self::Bash => "bash script.sh",
            Self::Base => "cat file.txt",
            Self::Custom => "echo 'No run command'",
        }
    }
}

/// Builtin E2B templates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinTemplate {
    /// Base template (minimal)
    Base,
    /// Python 3.11
    Python311,
    /// Python with data science libs
    PythonDataScience,
    /// Node.js 20
    Node20,
    /// TypeScript
    TypeScript,
    /// Rust
    Rust,
    /// Go
    Go,
    /// Next.js
    NextJs,
}

impl BuiltinTemplate {
    /// Get template ID for E2B
    pub fn id(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Python311 => "python-3.11",
            Self::PythonDataScience => "python-data-science",
            Self::Node20 => "node-20",
            Self::TypeScript => "typescript",
            Self::Rust => "rust",
            Self::Go => "go",
            Self::NextJs => "nextjs",
        }
    }

    /// Get template name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Base => "Base",
            Self::Python311 => "Python 3.11",
            Self::PythonDataScience => "Python Data Science",
            Self::Node20 => "Node.js 20",
            Self::TypeScript => "TypeScript",
            Self::Rust => "Rust",
            Self::Go => "Go",
            Self::NextJs => "Next.js",
        }
    }

    /// Get language
    pub fn language(&self) -> TemplateLanguage {
        match self {
            Self::Base => TemplateLanguage::Base,
            Self::Python311 => TemplateLanguage::Python,
            Self::PythonDataScience => TemplateLanguage::Python,
            Self::Node20 => TemplateLanguage::JavaScript,
            Self::TypeScript => TemplateLanguage::TypeScript,
            Self::Rust => TemplateLanguage::Rust,
            Self::Go => TemplateLanguage::Go,
            Self::NextJs => TemplateLanguage::JavaScript,
        }
    }

    /// Get pre-installed packages
    pub fn packages(&self) -> &'static [&'static str] {
        match self {
            Self::Base => &[],
            Self::Python311 => &["pip", "venv"],
            Self::PythonDataScience => &["numpy", "pandas", "matplotlib", "scikit-learn", "jupyter"],
            Self::Node20 => &["npm", "yarn"],
            Self::TypeScript => &["typescript", "tsx"],
            Self::Rust => &["cargo", "rustfmt", "clippy"],
            Self::Go => &["go"],
            Self::NextJs => &["next", "react", "react-dom"],
        }
    }

    /// Get all builtin templates
    pub fn all() -> Vec<Self> {
        vec![
            Self::Base,
            Self::Python311,
            Self::PythonDataScience,
            Self::Node20,
            Self::TypeScript,
            Self::Rust,
            Self::Go,
            Self::NextJs,
        ]
    }
}

impl std::fmt::Display for BuiltinTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = Template::new("custom", "Custom", TemplateLanguage::Python)
            .with_description("Custom template")
            .with_packages(vec!["numpy".to_string()]);

        assert_eq!(template.id, "custom");
        assert_eq!(template.language, TemplateLanguage::Python);
        assert_eq!(template.packages, vec!["numpy"]);
    }

    #[test]
    fn test_language_extension() {
        assert_eq!(TemplateLanguage::Python.extension(), "py");
        assert_eq!(TemplateLanguage::Rust.extension(), "rs");
        assert_eq!(TemplateLanguage::JavaScript.extension(), "js");
    }

    #[test]
    fn test_language_entrypoint() {
        assert_eq!(TemplateLanguage::Python.entrypoint(), "main.py");
        assert_eq!(TemplateLanguage::Rust.entrypoint(), "main.rs");
    }

    #[test]
    fn test_builtin_template_ids() {
        assert_eq!(BuiltinTemplate::Python311.id(), "python-3.11");
        assert_eq!(BuiltinTemplate::Node20.id(), "node-20");
        assert_eq!(BuiltinTemplate::Rust.id(), "rust");
    }

    #[test]
    fn test_builtin_template_packages() {
        let pkgs = BuiltinTemplate::PythonDataScience.packages();
        assert!(pkgs.contains(&"numpy"));
        assert!(pkgs.contains(&"pandas"));
    }
}
