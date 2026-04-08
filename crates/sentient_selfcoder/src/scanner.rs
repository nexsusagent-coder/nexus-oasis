//! ═══════════════════════════════════════════════════════════════════════════════
//!  CODEBASE SCANNER
//! ═══════════════════════════════════════════════════════════════════════════════

use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;
use walkdir::WalkDir;

use crate::rules::{Rule, RuleType};

/// Gap (eksiklik)
#[derive(Debug, Clone)]
pub struct Gap {
    pub rule_name: String,
    pub description: String,
    pub file: Option<PathBuf>,
    pub line: Option<usize>,
    pub suggestion: Option<String>,
    pub severity: String,
}

/// Tarama sonucu
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub files_scanned: usize,
    pub rust_modules: usize,
    pub yaml_files: usize,
    pub md_files: usize,
    pub total_lines: usize,
    pub crates: Vec<String>,
}

/// Codebase Scanner
pub struct CodebaseScanner {
    root: PathBuf,
}

impl CodebaseScanner {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }
    
    /// Codebase'i tara
    pub fn scan(&self) -> anyhow::Result<ScanResult> {
        let mut result = ScanResult {
            files_scanned: 0,
            rust_modules: 0,
            yaml_files: 0,
            md_files: 0,
            total_lines: 0,
            crates: Vec::new(),
        };
        
        // Crates dizinini tara
        let crates_dir = self.root.join("crates");
        if crates_dir.exists() {
            for entry in std::fs::read_dir(&crates_dir)? {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.path().file_name() {
                            result.crates.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        
        // Tüm dosyaları tara
        for entry in WalkDir::new(&self.root)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Target ve .git dizinlerini atla
            if path.to_str().map(|s| s.contains("/target/") || s.contains("/.git/")).unwrap_or(false) {
                continue;
            }
            
            result.files_scanned += 1;
            
            match path.extension().and_then(|e| e.to_str()) {
                Some("rs") => {
                    result.rust_modules += 1;
                    if let Ok(content) = fs::read_to_string(path) {
                        result.total_lines += content.lines().count();
                    }
                }
                Some("yaml") | Some("yml") => {
                    result.yaml_files += 1;
                }
                Some("md") => {
                    result.md_files += 1;
                }
                _ => {}
            }
        }
        
        Ok(result)
    }
    
    /// Gap'leri bul
    pub fn find_gaps(&self, rules: &[Rule], scan_result: &ScanResult) -> anyhow::Result<Vec<Gap>> {
        let mut gaps = Vec::new();
        
        for rule in rules {
            match rule.rule_type {
                RuleType::FileExists => {
                    if let Some(pattern) = &rule.pattern {
                        let path = self.root.join(pattern);
                        if !path.exists() {
                            gaps.push(Gap {
                                rule_name: rule.name.clone(),
                                description: rule.description.clone(),
                                file: Some(path),
                                line: None,
                                suggestion: Some(format!("Dosya oluştur: {}", pattern)),
                                severity: format!("{:?}", rule.severity),
                            });
                        }
                    }
                }
                RuleType::ModuleStructure => {
                    // Her crate'te lib.rs kontrolü
                    for crate_name in &scan_result.crates {
                        let lib_path = self.root.join("crates").join(crate_name).join("src").join("lib.rs");
                        let main_path = self.root.join("crates").join(crate_name).join("src").join("main.rs");
                        
                        if !lib_path.exists() && !main_path.exists() {
                            gaps.push(Gap {
                                rule_name: rule.name.clone(),
                                description: format!("Crate '{}' için lib.rs veya main.rs yok", crate_name),
                                file: Some(self.root.join("crates").join(crate_name)),
                                line: None,
                                suggestion: Some("lib.rs oluştur".to_string()),
                                severity: format!("{:?}", rule.severity),
                            });
                        }
                    }
                }
                RuleType::ContentPattern => {
                    if let Some(pattern) = &rule.pattern {
                        gaps.extend(self.find_content_gaps(pattern, rule)?);
                    }
                }
                RuleType::TestCoverage => {
                    // Test coverage kontrolü
                    gaps.extend(self.find_test_gaps(rule)?);
                }
                RuleType::Documentation => {
                    // Dökümantasyon kontrolü
                    gaps.extend(self.find_doc_gaps(rule)?);
                }
                RuleType::Dependency => {
                    // Skill library kontrolü
                    if rule.name == "skill_library_min" {
                        let skill_count = self.count_skills()?;
                        if skill_count < 5400 {
                            gaps.push(Gap {
                                rule_name: rule.name.clone(),
                                description: format!("Skill library yetersiz: {} < 5400", skill_count),
                                file: Some(self.root.join("data/skills")),
                                line: None,
                                suggestion: Some("Daha fazla skill ingest et".to_string()),
                                severity: format!("{:?}", rule.severity),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(gaps)
    }
    
    /// İçerik pattern gap'lerini bul
    fn find_content_gaps(&self, pattern: &str, rule: &Rule) -> anyhow::Result<Vec<Gap>> {
        let mut gaps = Vec::new();
        let regex = Regex::new(pattern)?;
        
        for entry in WalkDir::new(&self.root)
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.extension().map(|e| e != "rs").unwrap_or(true) {
                continue;
            }
            
            if path.to_str().map(|s| s.contains("/target/")).unwrap_or(false) {
                continue;
            }
            
            if let Ok(content) = fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if regex.is_match(line) {
                        gaps.push(Gap {
                            rule_name: rule.name.clone(),
                            description: rule.description.clone(),
                            file: Some(path.to_path_buf()),
                            line: Some(i + 1),
                            suggestion: Some("Bu satırı düzelt".to_string()),
                            severity: format!("{:?}", rule.severity),
                        });
                    }
                }
            }
        }
        
        Ok(gaps)
    }
    
    /// Test gap'lerini bul
    fn find_test_gaps(&self, rule: &Rule) -> anyhow::Result<Vec<Gap>> {
        let mut gaps = Vec::new();
        
        for entry in WalkDir::new(&self.root.join("crates"))
            .min_depth(2)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.file_name().map(|n| n == "lib.rs" || n == "main.rs").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(path) {
                    if !content.contains("#[cfg(test)]") {
                        gaps.push(Gap {
                            rule_name: rule.name.clone(),
                            description: "Test modülü yok".to_string(),
                            file: Some(path.to_path_buf()),
                            line: None,
                            suggestion: Some("#[cfg(test)] mod tests { } ekle".to_string()),
                            severity: format!("{:?}", rule.severity),
                        });
                    }
                }
            }
        }
        
        Ok(gaps)
    }
    
    /// Dökümantasyon gap'lerini bul
    fn find_doc_gaps(&self, rule: &Rule) -> anyhow::Result<Vec<Gap>> {
        let mut gaps = Vec::new();
        
        for entry in WalkDir::new(&self.root.join("crates"))
            .min_depth(2)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.extension().map(|e| e == "rs").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(path) {
                    if !content.starts_with("//!") {
                        gaps.push(Gap {
                            rule_name: rule.name.clone(),
                            description: "Dökümantasyon başlığı yok".to_string(),
                            file: Some(path.to_path_buf()),
                            line: Some(1),
                            suggestion: Some("//! Modül açıklaması ekle".to_string()),
                            severity: format!("{:?}", rule.severity),
                        });
                    }
                }
            }
        }
        
        Ok(gaps)
    }
    
    /// Skill sayısını say
    fn count_skills(&self) -> anyhow::Result<usize> {
        let skills_dir = self.root.join("data/skills");
        
        if !skills_dir.exists() {
            return Ok(0);
        }
        
        let count = WalkDir::new(&skills_dir)
            .min_depth(1)
            .into_iter()
            .filter(|e| {
                e.as_ref().ok()
                    .and_then(|e| e.path().extension())
                    .map(|e| e == "yaml")
                    .unwrap_or(false)
            })
            .count();
        
        Ok(count)
    }
}
