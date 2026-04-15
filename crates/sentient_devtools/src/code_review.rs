//! ═══════════════════════════════════════════════════════════════════════════════
//!  AI-Assisted Code Review System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Automated code review with AI:
//! - Bug detection
//! - Security vulnerability scanning
//! - Code smell detection
//! - Best practices enforcement
//! - Performance analysis

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  REVIEW TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Code review severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Issue category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Bug,
    Security,
    Performance,
    Style,
    Complexity,
    Maintainability,
    Documentation,
    Testing,
    BestPractice,
    Custom(String),
}

/// Code review issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    /// Issue ID
    pub id: String,
    /// Issue title
    pub title: String,
    /// Description
    pub description: String,
    /// Category
    pub category: IssueCategory,
    /// Severity
    pub severity: Severity,
    /// File path
    pub file: String,
    /// Line number
    pub line: u32,
    /// Column
    pub column: Option<u32>,
    /// End line
    pub end_line: Option<u32>,
    /// Code snippet
    pub snippet: Option<String>,
    /// Suggested fix
    pub suggestion: Option<String>,
    /// Rule ID
    pub rule_id: String,
    /// Confidence (0.0 - 1.0)
    pub confidence: f32,
    /// Related issues
    pub related: Vec<String>,
}

/// Code review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResult {
    /// Review ID
    pub id: String,
    /// Files reviewed
    pub files: Vec<String>,
    /// Issues found
    pub issues: Vec<CodeIssue>,
    /// Summary
    pub summary: ReviewSummary,
    /// Metrics
    pub metrics: CodeMetrics,
    /// AI analysis
    pub ai_analysis: Option<AIAnalysis>,
}

/// Review summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewSummary {
    /// Total issues
    pub total_issues: usize,
    /// Issues by severity
    pub by_severity: HashMap<String, usize>,
    /// Issues by category
    pub by_category: HashMap<String, usize>,
    /// Overall score (0-100)
    pub score: f32,
    /// Recommendation
    pub recommendation: String,
}

/// Code metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeMetrics {
    /// Lines of code
    pub lines_of_code: usize,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: u32,
    /// Cognitive complexity
    pub cognitive_complexity: u32,
    /// Maintainability index
    pub maintainability_index: f32,
    /// Code duplication percentage
    pub duplication_percent: f32,
    /// Comment ratio
    pub comment_ratio: f32,
    /// Test coverage
    pub test_coverage: Option<f32>,
}

/// AI-generated analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    /// High-level summary
    pub summary: String,
    /// Key concerns
    pub concerns: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Architecture notes
    pub architecture_notes: Option<String>,
    /// Potential improvements
    pub improvements: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REVIEW RULES
// ═══════════════════════════════════════════════════════════════════════════════

/// Review rule definition
#[derive(Debug, Clone)]
pub struct ReviewRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Description
    pub description: String,
    /// Category
    pub category: IssueCategory,
    /// Default severity
    pub default_severity: Severity,
    /// Language
    pub language: String,
    /// Pattern to match (regex)
    pub pattern: String,
    /// Is enabled
    pub enabled: bool,
}

impl ReviewRule {
    /// Get common rules
    pub fn common_rules() -> Vec<Self> {
        vec![
            // Security
            Self {
                id: "SEC001".to_string(),
                name: "Hardcoded Secret".to_string(),
                description: "Potential hardcoded secret or API key".to_string(),
                category: IssueCategory::Security,
                default_severity: Severity::Critical,
                language: "*".to_string(),
                pattern: r#"(?i)(password|api_key|secret|token)\s*[=:]\s*['\"][^'\"]+['\"]"#.to_string(),
                enabled: true,
            },
            Self {
                id: "SEC002".to_string(),
                name: "SQL Injection Risk".to_string(),
                description: "Potential SQL injection vulnerability".to_string(),
                category: IssueCategory::Security,
                default_severity: Severity::Critical,
                language: "*".to_string(),
                pattern: r#"(?i)(?:execute|exec|query)\s*\([^)]*\+[^)]*\)"#.to_string(),
                enabled: true,
            },
            Self {
                id: "SEC003".to_string(),
                name: "Unsafe Deserialization".to_string(),
                description: "Unsafe deserialization which may lead to RCE".to_string(),
                category: IssueCategory::Security,
                default_severity: Severity::Critical,
                language: "*".to_string(),
                pattern: r#"(?i)(?:pickle\.loads?|yaml\.load|unserialize)\s*\("#.to_string(),
                enabled: true,
            },
            
            // Bugs
            Self {
                id: "BUG001".to_string(),
                name: "Empty Catch Block".to_string(),
                description: "Empty catch block may hide errors".to_string(),
                category: IssueCategory::Bug,
                default_severity: Severity::Warning,
                language: "*".to_string(),
                pattern: r#"catch\s*\([^)]*\)\s*\{\s*\}"#.to_string(),
                enabled: true,
            },
            Self {
                id: "BUG002".to_string(),
                name: "Unused Variable".to_string(),
                description: "Variable is declared but never used".to_string(),
                category: IssueCategory::Bug,
                default_severity: Severity::Warning,
                language: "*".to_string(),
                pattern: r#"(?:let|var|int|string)\s+(\w+)\s*(?:=|;)(?!.*\1)"#.to_string(),
                enabled: true,
            },
            
            // Performance
            Self {
                id: "PERF001".to_string(),
                name: "N+1 Query Pattern".to_string(),
                description: "Potential N+1 query problem".to_string(),
                category: IssueCategory::Performance,
                default_severity: Severity::Warning,
                language: "*".to_string(),
                pattern: r#"for\s*\([^)]+\)\s*\{[^}]*query[^}]*\}"#.to_string(),
                enabled: true,
            },
            Self {
                id: "PERF002".to_string(),
                name: "String Concatenation in Loop".to_string(),
                description: "String concatenation in loop, consider using StringBuilder".to_string(),
                category: IssueCategory::Performance,
                default_severity: Severity::Warning,
                language: "*".to_string(),
                pattern: r#"for\s*\([^)]+\)\s*\{[^}]*\+=[^}]*\}"#.to_string(),
                enabled: true,
            },
            
            // Style
            Self {
                id: "STYLE001".to_string(),
                name: "Long Function".to_string(),
                description: "Function exceeds 50 lines".to_string(),
                category: IssueCategory::Style,
                default_severity: Severity::Info,
                language: "*".to_string(),
                pattern: r#"fn\s+\w+[^{]*\{[^\}]{5000,}"#.to_string(),
                enabled: true,
            },
            
            // Complexity
            Self {
                id: "COMP001".to_string(),
                name: "Deeply Nested Code".to_string(),
                description: "Code is deeply nested, consider refactoring".to_string(),
                category: IssueCategory::Complexity,
                default_severity: Severity::Warning,
                language: "*".to_string(),
                pattern: r#"\{[^\{\}]*\{[^\{\}]*\{[^\{\}]*\{[^\{\}]*\{"#.to_string(),
                enabled: true,
            },
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CODE REVIEWER
// ═══════════════════════════════════════════════════════════════════════════════

/// Code review error
#[derive(Debug, thiserror::Error)]
pub enum ReviewError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("AI analysis failed: {0}")]
    AIAnalysisFailed(String),
}

/// Code review configuration
#[derive(Debug, Clone)]
pub struct ReviewConfig {
    /// Enable AI analysis
    pub enable_ai: bool,
    /// Maximum file size (bytes)
    pub max_file_size: usize,
    /// Enabled rule IDs (empty = all)
    pub enabled_rules: Vec<String>,
    /// Minimum severity to report
    pub min_severity: Severity,
    /// Exclude patterns
    pub exclude_patterns: Vec<String>,
}

impl Default for ReviewConfig {
    fn default() -> Self {
        Self {
            enable_ai: true,
            max_file_size: 1024 * 1024, // 1 MB
            enabled_rules: vec![],
            min_severity: Severity::Info,
            exclude_patterns: vec!["node_modules".to_string(), "target".to_string(), ".git".to_string()],
        }
    }
}

/// AI-assisted code reviewer
pub struct CodeReviewer {
    config: ReviewConfig,
    rules: Vec<ReviewRule>,
}

impl CodeReviewer {
    /// Create a new code reviewer
    pub fn new(config: ReviewConfig) -> Self {
        Self {
            config,
            rules: ReviewRule::common_rules(),
        }
    }
    
    /// Review code content
    pub fn review(&self, file_path: &str, content: &str) -> Result<ReviewResult, ReviewError> {
        let mut issues = Vec::new();
        let mut metrics = CodeMetrics::default();
        
        // Calculate basic metrics
        metrics.lines_of_code = content.lines().count();
        metrics.cyclomatic_complexity = self.calculate_complexity(content);
        metrics.maintainability_index = self.calculate_maintainability(content, metrics.lines_of_code);
        
        // Apply rules
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if !self.config.enabled_rules.is_empty() && 
               !self.config.enabled_rules.contains(&rule.id) {
                continue;
            }
            
            let rule_issues = self.apply_rule(rule, file_path, content);
            issues.extend(rule_issues);
        }
        
        // Filter by severity
        issues.retain(|i| i.severity as u8 >= self.config.min_severity as u8);
        
        // Generate summary
        let summary = self.generate_summary(&issues);
        
        // Generate AI analysis (simulated)
        let ai_analysis = if self.config.enable_ai {
            Some(AIAnalysis {
                summary: format!("Reviewed {} lines of code. Found {} issues.", 
                    metrics.lines_of_code, issues.len()),
                concerns: issues.iter()
                    .filter(|i| i.severity as u8 >= Severity::Warning as u8)
                    .map(|i| i.title.clone())
                    .take(5)
                    .collect(),
                recommendations: self.generate_recommendations(&issues),
                architecture_notes: None,
                improvements: vec!["Consider adding unit tests".to_string()],
            })
        } else {
            None
        };
        
        Ok(ReviewResult {
            id: uuid::Uuid::new_v4().to_string(),
            files: vec![file_path.to_string()],
            issues,
            summary,
            metrics,
            ai_analysis,
        })
    }
    
    /// Apply a single rule
    fn apply_rule(&self, rule: &ReviewRule, file_path: &str, content: &str) -> Vec<CodeIssue> {
        let mut issues = Vec::new();
        
        // Simple pattern matching (in production, use regex crate)
        for (line_num, line) in content.lines().enumerate() {
            if self.matches_pattern(line, &rule.pattern) {
                issues.push(CodeIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: rule.name.clone(),
                    description: rule.description.clone(),
                    category: rule.category.clone(),
                    severity: rule.default_severity,
                    file: file_path.to_string(),
                    line: (line_num + 1) as u32,
                    column: None,
                    end_line: None,
                    snippet: Some(line.trim().chars().take(100).collect()),
                    suggestion: None,
                    rule_id: rule.id.clone(),
                    confidence: 0.8,
                    related: vec![],
                });
            }
        }
        
        issues
    }
    
    /// Simple pattern matching
    fn matches_pattern(&self, text: &str, pattern: &str) -> bool {
        // Simplified - in production use regex
        let pattern_lower = pattern.to_lowercase();
        let text_lower = text.to_lowercase();
        
        // Check for keywords
        if pattern_lower.contains("password") && 
           (text_lower.contains("password") && text_lower.contains("=")) {
            return true;
        }
        if pattern_lower.contains("api_key") && 
           (text_lower.contains("api_key") && text_lower.contains("=")) {
            return true;
        }
        if pattern_lower.contains("secret") && 
           text_lower.contains("secret") && text_lower.contains("=") {
            return true;
        }
        
        // Check for empty catch
        if pattern.contains("catch") && text.contains("catch") {
            let after_catch = text.split("catch").nth(1).unwrap_or("");
            if after_catch.contains("{}") || after_catch.trim() == "{ }" {
                return true;
            }
        }
        
        false
    }
    
    /// Calculate cyclomatic complexity
    fn calculate_complexity(&self, content: &str) -> u32 {
        let mut complexity = 1u32;
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("if ") || line.starts_with("if(") ||
               line.starts_with("else if") ||
               line.starts_with("for ") || line.starts_with("for(") ||
               line.starts_with("while ") || line.starts_with("while(") ||
               line.starts_with("match ") || line.contains(" match ") ||
               line.starts_with("case ") ||
               line.contains("&&") || line.contains("||") {
                complexity += 1;
            }
        }
        
        complexity
    }
    
    /// Calculate maintainability index
    fn calculate_maintainability(&self, content: &str, loc: usize) -> f32 {
        let complexity = self.calculate_complexity(content);
        let comment_lines = content.lines()
            .filter(|l| l.trim().starts_with("//") || l.trim().starts_with("/*") || l.trim().starts_with("*"))
            .count();
        
        if loc == 0 {
            return 100.0;
        }
        
        // Simplified maintainability index
        let volume = loc as f32;
        let volume_factor = (171.0 - 5.2 * volume.ln()).max(0.0);
        let complexity_factor = (171.0 - 0.23 * complexity as f32).max(0.0);
        let comment_factor = (171.0 + 16.2 * (comment_lines as f32 + 1.0).ln()).min(171.0);
        
        ((volume_factor + complexity_factor + comment_factor) / 171.0 * 100.0).min(100.0)
    }
    
    /// Generate summary
    fn generate_summary(&self, issues: &[CodeIssue]) -> ReviewSummary {
        let mut by_severity = HashMap::new();
        let mut by_category = HashMap::new();
        
        for issue in issues {
            *by_severity.entry(format!("{:?}", issue.severity)).or_insert(0) += 1;
            *by_category.entry(format!("{:?}", issue.category)).or_insert(0) += 1;
        }
        
        // Calculate score
        let critical = *by_severity.get("Critical").unwrap_or(&0) as f32;
        let error = *by_severity.get("Error").unwrap_or(&0) as f32;
        let warning = *by_severity.get("Warning").unwrap_or(&0) as f32;
        
        let score = (100.0 - critical * 20.0 - error * 10.0 - warning * 2.0).max(0.0);
        
        let recommendation = if score >= 80.0 {
            "Code looks good! Only minor issues found.".to_string()
        } else if score >= 60.0 {
            "Consider addressing the warnings before merging.".to_string()
        } else if score >= 40.0 {
            "Several issues found. Review recommended.".to_string()
        } else {
            "Critical issues found! Do not merge without fixes.".to_string()
        };
        
        ReviewSummary {
            total_issues: issues.len(),
            by_severity,
            by_category,
            score,
            recommendation,
        }
    }
    
    /// Generate recommendations
    fn generate_recommendations(&self, issues: &[CodeIssue]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        let has_security = issues.iter().any(|i| matches!(i.category, IssueCategory::Security));
        let has_bug = issues.iter().any(|i| matches!(i.category, IssueCategory::Bug));
        
        if has_security {
            recommendations.push("Address security vulnerabilities before deployment".to_string());
        }
        if has_bug {
            recommendations.push("Fix potential bugs to improve reliability".to_string());
        }
        
        recommendations.push("Add unit tests for new functionality".to_string());
        recommendations.push("Update documentation as needed".to_string());
        
        recommendations
    }
}

impl Default for CodeReviewer {
    fn default() -> Self {
        Self::new(ReviewConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_review_code() {
        let reviewer = CodeReviewer::default();
        
        let code = r#"
fn main() {
    let password = "secret123";
    let api_key = "sk-12345";
    println!("Hello");
}
"#;
        
        let result = reviewer.review("test.rs", code).unwrap();
        
        // Should detect hardcoded secrets
        assert!(!result.issues.is_empty());
    }
    
    #[test]
    fn test_complexity_calculation() {
        let reviewer = CodeReviewer::default();
        
        let code = r#"
fn test() {
    if x {
        if y {
            for i in 0..10 {
                if z {
                    // nested
                }
            }
        }
    }
}
"#;
        
        let complexity = reviewer.calculate_complexity(code);
        assert!(complexity > 1);
    }
    
    #[test]
    fn test_score_calculation() {
        let reviewer = CodeReviewer::default();
        
        let issues = vec![
            CodeIssue {
                id: "1".to_string(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                category: IssueCategory::Bug,
                severity: Severity::Critical,
                file: "test.rs".to_string(),
                line: 1,
                column: None,
                end_line: None,
                snippet: None,
                suggestion: None,
                rule_id: "TEST".to_string(),
                confidence: 1.0,
                related: vec![],
            },
        ];
        
        let summary = reviewer.generate_summary(&issues);
        
        assert!(summary.score <= 80.0);
    }
}
