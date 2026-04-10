//! ═══════════════════════════════════════════════════════════════════════════════
//!  RULE ENGINE - Knowledge Base Kuralları
//! ═══════════════════════════════════════════════════════════════════════════════

use std::path::Path;
use std::fs;
use regex::Regex;

/// Kural
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
    pub pattern: Option<String>,
    pub required: bool,
    pub severity: Severity,
}

/// Kural tipi
#[derive(Debug, Clone)]
pub enum RuleType {
    /// Dosya varlığı kontrolü
    FileExists,
    /// İçerik pattern kontrolü
    ContentPattern,
    /// Modül yapısı kontrolü
    ModuleStructure,
    /// Dependency kontrolü
    Dependency,
    /// Test coverage kontrolü
    TestCoverage,
    /// Dökümantasyon kontrolü
    Documentation,
}

/// Severity seviyesi
#[derive(Debug, Clone)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

/// Rule Engine
pub struct RuleEngine {
    knowledge_dir: std::path::PathBuf,
}

impl RuleEngine {
    pub fn new(knowledge_dir: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            knowledge_dir: knowledge_dir.to_path_buf(),
        })
    }
    
    /// Knowledge base'den kuralları yükle
    pub fn load_rules(&self) -> anyhow::Result<Vec<Rule>> {
        let mut rules = Vec::new();
        
        // Varsayılan kurallar
        rules.extend(self.default_rules());
        
        // Knowledge base dosyalarından kuralları çıkar
        if self.knowledge_dir.exists() {
            for entry in walkdir::WalkDir::new(&self.knowledge_dir)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(path) {
                        rules.extend(self.extract_rules_from_markdown(&content));
                    }
                }
            }
        }
        
        Ok(rules)
    }
    
    /// Varsayılan kurallar
    fn default_rules(&self) -> Vec<Rule> {
        vec![
            Rule {
                name: "lib.rs_exists".to_string(),
                description: "Her crate'ta lib.rs olmalı".to_string(),
                rule_type: RuleType::ModuleStructure,
                pattern: Some("lib.rs".to_string()),
                required: true,
                severity: Severity::High,
            },
            Rule {
                name: "cargo_toml_valid".to_string(),
                description: "Cargo.toml geçerli olmalı".to_string(),
                rule_type: RuleType::FileExists,
                pattern: Some("Cargo.toml".to_string()),
                required: true,
                severity: Severity::Critical,
            },
            Rule {
                name: "tests_exist".to_string(),
                description: "Her modülde test olmalı".to_string(),
                rule_type: RuleType::TestCoverage,
                pattern: Some("#[cfg(test)]".to_string()),
                required: false,
                severity: Severity::Medium,
            },
            Rule {
                name: "documentation_header".to_string(),
                description: "Dosyalar dökümantasyon başlığı ile başlamalı".to_string(),
                rule_type: RuleType::Documentation,
                pattern: Some(r"^//!".to_string()),
                required: false,
                severity: Severity::Low,
            },
            Rule {
                name: "no_todo".to_string(),
                description: "TODO comment'ları kalmamalı".to_string(),
                rule_type: RuleType::ContentPattern,
                pattern: Some(r"TODO|FIXME|XXX".to_string()),
                required: false,
                severity: Severity::Low,
            },
            Rule {
                name: "error_handling".to_string(),
                description: "Hata yönetimi kullanılmalı".to_string(),
                rule_type: RuleType::ContentPattern,
                pattern: Some(r"anyhow::Result|Result<".to_string()),
                required: false,
                severity: Severity::Medium,
            },
            Rule {
                name: "no_unwrap".to_string(),
                description: "unwrap() kullanılmamalı".to_string(),
                rule_type: RuleType::ContentPattern,
                pattern: Some(r"\.unwrap\(\)".to_string()),
                required: false,
                severity: Severity::Medium,
            },
            Rule {
                name: "knowledge_base_exists".to_string(),
                description: "Knowledge base dizini olmalı".to_string(),
                rule_type: RuleType::FileExists,
                pattern: Some("knowledge_base".to_string()),
                required: true,
                severity: Severity::High,
            },
            Rule {
                name: "skill_library_min".to_string(),
                description: "En az 5400 skill olmalı".to_string(),
                rule_type: RuleType::Dependency,
                pattern: None,
                required: true,
                severity: Severity::High,
            },
        ]
    }
    
    /// Markdown'dan kuralları çıkar
    fn extract_rules_from_markdown(&self, content: &str) -> Vec<Rule> {
        let mut rules = Vec::new();
        
        // Başlıkları kontrol et
        let _heading_re = Regex::new(r"^#+\s+(.+)$").expect("operation failed");
        let checkbox_re = Regex::new(r"- \[([ xX])\]\s+(.+)$").expect("operation failed");
        
        for line in content.lines() {
            // Checkbox kuralları
            if let Some(caps) = checkbox_re.captures(line) {
                let checked = &caps[1] != " ";
                let text = &caps[2];
                
                rules.push(Rule {
                    name: text.to_lowercase().replace(" ", "_"),
                    description: text.to_string(),
                    rule_type: RuleType::ContentPattern,
                    pattern: None,
                    required: checked,
                    severity: Severity::Medium,
                });
            }
        }
        
        rules
    }
}
