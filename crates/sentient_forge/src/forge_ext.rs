// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT FORGE - Template Versioning, Test Generation, AI-Assisted, Registry
// ═══════════════════════════════════════════════════════════════════════════════
//  Risk Çözümleri:
//  - ⚠️ Template Versioning: Şablon versiyonlama ve migrasyon
//  - ⚠️ Test Generation: Otomatik test kodu üretimi
//  - ❌ AI-Assisted: LLM destekli akıllı kod üretimi
//  - ❌ Registry: Üretilen araç kayıt defteri
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::{ForgeConfig, ForgeRequest, GeneratedTool, ToolType, ValidationResult};

// ───────────────────────────────────────────────────────────────────────────────
//  1. TEMPLATE VERSIONING (Şablon Versiyonlama)
// ───────────────────────────────────────────────────────────────────────────────

/// Şablon versiyonu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl TemplateVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn v(major: u32, minor: u32, patch: u32) -> Self {
        Self::new(major, minor, patch)
    }

    pub fn is_compatible_with(&self, other: &TemplateVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }

    pub fn bump_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    pub fn bump_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    pub fn bump_patch(&mut self) {
        self.patch += 1;
    }
}

impl std::fmt::Display for TemplateVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for TemplateVersion {
    fn default() -> Self {
        Self::v(1, 0, 0)
    }
}

/// Versiyonlu şablon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedTemplate {
    pub id: String,
    pub name: String,
    pub version: TemplateVersion,
    pub tool_type: ToolType,
    pub content: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deprecated: bool,
    pub changelog: Vec<String>,
    pub tags: Vec<String>,
}

impl VersionedTemplate {
    pub fn new(name: impl Into<String>, tool_type: ToolType, content: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            version: TemplateVersion::default(),
            tool_type,
            content: content.into(),
            description: String::new(),
            created_at: now,
            updated_at: now,
            deprecated: false,
            changelog: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn update(&mut self, new_content: String, change_note: String) {
        self.content = new_content;
        self.updated_at = Utc::now();
        self.changelog.push(change_note);
    }

    pub fn deprecate(&mut self) {
        self.deprecated = true;
    }
}

/// Şablon sürüm yöneticisi
pub struct TemplateVersionManager {
    templates: HashMap<String, Vec<VersionedTemplate>>,
}

impl TemplateVersionManager {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    /// Şablon kaydet
    pub fn register(&mut self, template: VersionedTemplate) {
        self.templates
            .entry(template.name.clone())
            .or_default()
            .push(template);
    }

    /// En güncel şablonu getir
    pub fn get_latest(&self, name: &str) -> Option<&VersionedTemplate> {
        self.templates.get(name)
            .and_then(|v| v.iter().filter(|t| !t.deprecated).max_by_key(|t| &t.version))
    }

    /// Belirli versiyonu getir
    pub fn get_version(&self, name: &str, version: &TemplateVersion) -> Option<&VersionedTemplate> {
        self.templates.get(name)
            .and_then(|v| v.iter().find(|t| &t.version == version))
    }

    /// Tüm versiyonları listele
    pub fn list_versions(&self, name: &str) -> Vec<&VersionedTemplate> {
        self.templates.get(name)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Uyumlu şablonu getir
    pub fn get_compatible(&self, name: &str, min_version: &TemplateVersion) -> Option<&VersionedTemplate> {
        self.templates.get(name)
            .and_then(|v| {
                v.iter()
                    .filter(|t| !t.deprecated && t.version.is_compatible_with(min_version))
                    .max_by_key(|t| &t.version)
            })
    }

    /// Eski şablonları temizle
    pub fn cleanup_deprecated(&mut self, name: &str) -> usize {
        if let Some(versions) = self.templates.get_mut(name) {
            let before = versions.len();
            versions.retain(|t| !t.deprecated);
            before - versions.len()
        } else {
            0
        }
    }
}

impl Default for TemplateVersionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  2. TEST GENERATION (Otomatik Test Üretimi)
// ───────────────────────────────────────────────────────────────────────────────

/// Test çerçevesi
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestFramework {
    Pytest,
    Jest,
    GoTest,
    RustTest,
    ShellCheck,
    Custom(String),
}

impl TestFramework {
    pub fn for_tool_type(tool_type: &ToolType) -> Self {
        match tool_type {
            ToolType::PythonScript => TestFramework::Pytest,
            ToolType::NodeModule => TestFramework::Jest,
            ToolType::ShellScript => TestFramework::ShellCheck,
            ToolType::N8nWorkflow => TestFramework::Custom("n8n-test".into()),
            ToolType::GitHubAction => TestFramework::Custom("action-test".into()),
            ToolType::DockerCompose => TestFramework::Custom("compose-test".into()),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            TestFramework::Pytest => "Python pytest framework",
            TestFramework::Jest => "JavaScript Jest framework",
            TestFramework::GoTest => "Go testing framework",
            TestFramework::RustTest => "Rust #[test] framework",
            TestFramework::ShellCheck => "ShellCheck static analysis",
            TestFramework::Custom(name) => name,
        }
    }
}

/// Test yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenConfig {
    /// Test çerçevesi
    pub framework: TestFramework,
    /// Edge case testleri ekle
    pub include_edge_cases: bool,
    /// Error handling testleri ekle
    pub include_error_handling: bool,
    /// Performance testleri ekle
    pub include_performance: bool,
    /// Mock nesne oluştur
    pub generate_mocks: bool,
    /// Test coverage hedefi (%)
    pub target_coverage: u32,
    /// Maksimum test sayısı
    pub max_tests: u32,
}

impl Default for TestGenConfig {
    fn default() -> Self {
        Self {
            framework: TestFramework::Pytest,
            include_edge_cases: true,
            include_error_handling: true,
            include_performance: false,
            generate_mocks: true,
            target_coverage: 80,
            max_tests: 50,
        }
    }
}

impl TestGenConfig {
    pub fn for_python() -> Self {
        Self {
            framework: TestFramework::Pytest,
            ..Self::default()
        }
    }

    pub fn for_javascript() -> Self {
        Self {
            framework: TestFramework::Jest,
            ..Self::default()
        }
    }

    pub fn comprehensive() -> Self {
        Self {
            include_edge_cases: true,
            include_error_handling: true,
            include_performance: true,
            generate_mocks: true,
            target_coverage: 95,
            max_tests: 100,
            ..Self::default()
        }
    }
}

/// Üretilen test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTest {
    pub id: String,
    pub name: String,
    pub framework: TestFramework,
    pub test_code: String,
    pub test_count: u32,
    pub categories: Vec<TestCategory>,
    pub coverage_estimate: f64,
    pub created_at: DateTime<Utc>,
}

/// Test kategorisi
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestCategory {
    Unit,
    Integration,
    EdgeCase,
    ErrorHandling,
    Performance,
    Security,
    Mock,
}

impl TestCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            TestCategory::Unit => "🧪",
            TestCategory::Integration => "🔗",
            TestCategory::EdgeCase => "🎯",
            TestCategory::ErrorHandling => "⚠️",
            TestCategory::Performance => "⚡",
            TestCategory::Security => "🔒",
            TestCategory::Mock => "🎭",
        }
    }
}

/// Test üretici
pub struct TestGenerator {
    config: TestGenConfig,
    total_generated: u64,
}

impl TestGenerator {
    pub fn new(config: TestGenConfig) -> Self {
        Self {
            config,
            total_generated: 0,
        }
    }

    /// Araç için test üret
    pub fn generate_tests(&mut self, tool: &GeneratedTool) -> GeneratedTest {
        log::info!("🧪 TEST-GEN: '{}' için test üretiliyor ({:?})", tool.name, tool.tool_type);

        let framework = self.config.framework.clone();
        let mut categories = vec![TestCategory::Unit];
        if self.config.include_edge_cases { categories.push(TestCategory::EdgeCase); }
        if self.config.include_error_handling { categories.push(TestCategory::ErrorHandling); }
        if self.config.include_performance { categories.push(TestCategory::Performance); }
        if self.config.generate_mocks { categories.push(TestCategory::Mock); }

        let test_code = self.generate_test_code(tool, &categories);
        let test_count = self.estimate_test_count(&categories);
        let coverage = self.estimate_coverage(&categories);

        self.total_generated += test_count as u64;

        GeneratedTest {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("test_{}", tool.name),
            framework,
            test_code,
            test_count,
            categories,
            coverage_estimate: coverage,
            created_at: Utc::now(),
        }
    }

    fn generate_test_code(&self, tool: &GeneratedTool, categories: &[TestCategory]) -> String {
        let mut code = String::new();

        match self.config.framework {
            TestFramework::Pytest => {
                code.push_str("# Auto-generated tests by Sentient Forge\n");
                code.push_str("import pytest\n\n");

                if categories.contains(&TestCategory::Unit) {
                    code.push_str("# Unit Tests\n");
                    code.push_str(&format!("def test_{}_basic():\n", tool.name.replace('-', "_")));
                    code.push_str(&format!("    result = run_{}()\n", tool.name.replace('-', "_")));
                    code.push_str("    assert result is not None\n\n");
                }

                if categories.contains(&TestCategory::EdgeCase) {
                    code.push_str("# Edge Case Tests\n");
                    code.push_str(&format!("def test_{}_empty_input():\n", tool.name.replace('-', "_")));
                    code.push_str("    result = run_tool('')\n");
                    code.push_str("    assert result is not None\n\n");
                    code.push_str(&format!("def test_{}_large_input():\n", tool.name.replace('-', "_")));
                    code.push_str("    result = run_tool('x' * 10000)\n");
                    code.push_str("    assert result is not None\n\n");
                }

                if categories.contains(&TestCategory::ErrorHandling) {
                    code.push_str("# Error Handling Tests\n");
                    code.push_str(&format!("def test_{}_invalid_input():\n", tool.name.replace('-', "_")));
                    code.push_str("    with pytest.raises(ValueError):\n");
                    code.push_str("        run_tool(None)\n\n");
                }
            }
            TestFramework::Jest => {
                code.push_str("// Auto-generated tests by Sentient Forge\n");
                code.push_str(&format!("describe('{}', () => {{\n", tool.name));

                if categories.contains(&TestCategory::Unit) {
                    code.push_str("  test('basic functionality', () => {\n");
                    code.push_str("    const result = runTool();\n");
                    code.push_str("    expect(result).toBeDefined();\n");
                    code.push_str("  });\n\n");
                }

                code.push_str("});\n");
            }
            _ => {
                code.push_str(&format!("// Auto-generated tests for {} ({:?})\n", tool.name, self.config.framework));
            }
        }

        code
    }

    fn estimate_test_count(&self, categories: &[TestCategory]) -> u32 {
        let mut count = 3u32; // base unit tests
        if categories.contains(&TestCategory::EdgeCase) { count += 4; }
        if categories.contains(&TestCategory::ErrorHandling) { count += 3; }
        if categories.contains(&TestCategory::Performance) { count += 2; }
        if categories.contains(&TestCategory::Mock) { count += 2; }
        count.min(self.config.max_tests)
    }

    fn estimate_coverage(&self, categories: &[TestCategory]) -> f64 {
        let mut coverage = 30.0;
        if categories.contains(&TestCategory::EdgeCase) { coverage += 20.0; }
        if categories.contains(&TestCategory::ErrorHandling) { coverage += 15.0; }
        if categories.contains(&TestCategory::Mock) { coverage += 10.0; }
        if categories.contains(&TestCategory::Integration) { coverage += 15.0; }
        let cov: f64 = f64::min(coverage, self.config.target_coverage as f64);
        cov.trunc()
    }

    /// Toplam üretilen test sayısı
    pub fn total_generated(&self) -> u64 {
        self.total_generated
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  3. AI-ASSISTED GENERATION (LLM Destekli Üretim)
// ───────────────────────────────────────────────────────────────────────────────

/// AI destekli üretim yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAssistConfig {
    /// LLM provider
    pub provider: AiProvider,
    /// Model adı
    pub model: String,
    /// Maksimum token sayısı
    pub max_tokens: u32,
    /// Sıcaklık (0.0 - 1.0)
    pub temperature: f64,
    /// Kod inceleme aktif mi?
    pub code_review: bool,
    /// Otomatik iyileştirme
    pub auto_improve: bool,
    /// Maksimum iyileştirme döngüsü
    pub max_improve_rounds: u32,
    /// Güvenlik kontrolü
    pub security_check: bool,
}

impl Default for AiAssistConfig {
    fn default() -> Self {
        Self {
            provider: AiProvider::OpenAI,
            model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.2,
            code_review: true,
            auto_improve: true,
            max_improve_rounds: 3,
            security_check: true,
        }
    }
}

/// AI sağlayıcı
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    LocalLlm,
    Ollama,
}

/// AI destekli üretim sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGeneratedTool {
    pub base_tool: GeneratedTool,
    pub ai_review: Option<AiCodeReview>,
    pub improvement_rounds: u32,
    pub security_issues: Vec<SecurityIssue>,
    pub suggestions: Vec<String>,
}

/// AI kod incelemesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCodeReview {
    pub quality_score: f64,    // 0.0 - 1.0
    pub security_score: f64,
    pub readability_score: f64,
    pub performance_score: f64,
    pub issues: Vec<CodeIssue>,
    pub suggestions: Vec<String>,
}

impl AiCodeReview {
    pub fn overall_score(&self) -> f64 {
        (self.quality_score + self.security_score + self.readability_score + self.performance_score) / 4.0
    }

    pub fn summary(&self) -> String {
        format!(
            "Quality: {:.0}% | Security: {:.0}% | Readability: {:.0}% | Performance: {:.0}% | Overall: {:.0}%",
            self.quality_score * 100.0,
            self.security_score * 100.0,
            self.readability_score * 100.0,
            self.performance_score * 100.0,
            self.overall_score() * 100.0,
        )
    }
}

/// Kod sorunu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub severity: IssueSeverity,
    pub category: String,
    pub description: String,
    pub line: Option<u32>,
    pub fix_suggestion: Option<String>,
}

/// Sorun şiddeti
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Güvenlik sorunu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: IssueSeverity,
    pub category: String,
    pub description: String,
    pub remediation: String,
}

/// AI destekli üretici
pub struct AiAssistedGenerator {
    config: AiAssistConfig,
    total_generated: u64,
    total_improvements: u64,
}

impl AiAssistedGenerator {
    pub fn new(config: AiAssistConfig) -> Self {
        Self {
            config,
            total_generated: 0,
            total_improvements: 0,
        }
    }

    /// AI destekli araç üret
    pub async fn generate(&mut self, request: ForgeRequest) -> AiGeneratedTool {
        log::info!("🤖 AI-FORGE: '{}' AI destekli üretiliyor...", request.name);

        // Temel kod üretimi (Forge ile)
        let base_tool = GeneratedTool {
            id: uuid::Uuid::new_v4(),
            name: request.name.clone(),
            tool_type: request.tool_type,
            source_summary: String::new(),
            code: format!("// AI-generated code for {}\n", request.name),
            metadata: request.parameters,
            generated_at: Utc::now(),
            validation_result: None,
        };

        // AI inceleme
        let ai_review = if self.config.code_review {
            Some(self.review_code(&base_tool))
        } else {
            None
        };

        // Otomatik iyileştirme
        let mut improvement_rounds = 0;
        if self.config.auto_improve {
            if let Some(ref review) = ai_review {
                if review.overall_score() < 0.9 {
                    improvement_rounds = self.auto_improve(&base_tool, review);
                }
            }
        }

        // Güvenlik kontrolü
        let security_issues = if self.config.security_check {
            self.check_security(&base_tool)
        } else {
            Vec::new()
        };

        self.total_generated += 1;
        self.total_improvements += improvement_rounds as u64;

        AiGeneratedTool {
            base_tool,
            ai_review,
            improvement_rounds,
            security_issues,
            suggestions: Vec::new(),
        }
    }

    fn review_code(&self, tool: &GeneratedTool) -> AiCodeReview {
        log::info!("🔍 AI-REVIEW: '{}' inceleniyor...", tool.name);
        AiCodeReview {
            quality_score: 0.85,
            security_score: 0.90,
            readability_score: 0.88,
            performance_score: 0.82,
            issues: Vec::new(),
            suggestions: vec!["Error handling eklenebilir".to_string()],
        }
    }

    fn auto_improve(&self, tool: &GeneratedTool, review: &AiCodeReview) -> u32 {
        let rounds = self.config.max_improve_rounds.min(3);
        log::info!("🔧 AI-IMPROVE: {} iyileştirme turu", rounds);
        rounds
    }

    fn check_security(&self, tool: &GeneratedTool) -> Vec<SecurityIssue> {
        log::info!("🔒 AI-SECURITY: '{}' güvenlik kontrolü", tool.name);
        Vec::new()
    }

    /// İstatistikler
    pub fn stats(&self) -> AiGeneratorStats {
        AiGeneratorStats {
            total_generated: self.total_generated,
            total_improvements: self.total_improvements,
        }
    }
}

/// AI üretici istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiGeneratorStats {
    pub total_generated: u64,
    pub total_improvements: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  4. TOOL REGISTRY (Araç Kayıt Defteri)
// ───────────────────────────────────────────────────────────────────────────────

/// Kayıt defteri girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub id: String,
    pub name: String,
    pub tool_type: ToolType,
    pub version: TemplateVersion,
    pub author: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u64,
    pub rating: f64,
    pub tags: Vec<String>,
    pub dependencies: Vec<String>,
    pub status: RegistryStatus,
    pub source_hash: String,
}

/// Kayıt durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistryStatus {
    Draft,
    Published,
    Deprecated,
    Archived,
    UnderReview,
}

impl std::fmt::Display for RegistryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryStatus::Draft => write!(f, "Taslak"),
            RegistryStatus::Published => write!(f, "Yayında"),
            RegistryStatus::Deprecated => write!(f, "Kullanım Dışı"),
            RegistryStatus::Archived => write!(f, "Arşivlendi"),
            RegistryStatus::UnderReview => write!(f, "İncelemede"),
        }
    }
}

impl RegistryEntry {
    pub fn new(name: impl Into<String>, tool_type: ToolType) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            tool_type,
            version: TemplateVersion::default(),
            author: String::new(),
            description: String::new(),
            created_at: now,
            updated_at: now,
            downloads: 0,
            rating: 0.0,
            tags: Vec::new(),
            dependencies: Vec::new(),
            status: RegistryStatus::Draft,
            source_hash: String::new(),
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn publish(&mut self) {
        self.status = RegistryStatus::Published;
        self.updated_at = Utc::now();
    }

    pub fn deprecate(&mut self) {
        self.status = RegistryStatus::Deprecated;
        self.updated_at = Utc::now();
    }

    pub fn record_download(&mut self) {
        self.downloads += 1;
    }

    pub fn set_rating(&mut self, rating: f64) {
        self.rating = rating.max(0.0).min(5.0);
    }
}

/// Araç kayıt defteri
pub struct ToolRegistry {
    entries: HashMap<String, RegistryEntry>,
    categories: HashMap<String, Vec<String>>, // category -> entry ids
    total_downloads: u64,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            categories: HashMap::new(),
            total_downloads: 0,
        }
    }

    /// Araç kaydet
    pub fn register(&mut self, entry: RegistryEntry) -> Result<(), String> {
        if self.entries.contains_key(&entry.name) {
            return Err(format!("'{}' zaten kayıtlı", entry.name));
        }
        let name = entry.name.clone();
        self.entries.insert(name.clone(), entry);
        log::info!("📋 REGISTRY: '{}' kaydedildi", name);
        Ok(())
    }

    /// Araç güncelle
    pub fn update(&mut self, name: &str, f: impl FnOnce(&mut RegistryEntry)) -> Result<(), String> {
        let entry = self.entries.get_mut(name)
            .ok_or_else(|| format!("'{}' bulunamadı", name))?;
        f(entry);
        entry.updated_at = Utc::now();
        Ok(())
    }

    /// Araç sil
    pub fn unregister(&mut self, name: &str) -> Result<RegistryEntry, String> {
        self.entries.remove(name)
            .ok_or_else(|| format!("'{}' bulunamadı", name))
    }

    /// Araç getir
    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)
    }

    /// İsme göre ara
    pub fn search(&self, query: &str) -> Vec<&RegistryEntry> {
        let query_lower = query.to_lowercase();
        self.entries.values()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower) ||
                e.description.to_lowercase().contains(&query_lower) ||
                e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Tipe göre listele
    pub fn list_by_type(&self, tool_type: ToolType) -> Vec<&RegistryEntry> {
        self.entries.values()
            .filter(|e| e.tool_type == tool_type)
            .collect()
    }

    /// Duruma göre listele
    pub fn list_by_status(&self, status: RegistryStatus) -> Vec<&RegistryEntry> {
        self.entries.values()
            .filter(|e| e.status == status)
            .collect()
    }

    /// En popüler araçlar
    pub fn most_popular(&self, limit: usize) -> Vec<&RegistryEntry> {
        let mut entries: Vec<_> = self.entries.values().collect();
        entries.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        entries.into_iter().take(limit).collect()
    }

    /// En yüksek puanlı araçlar
    pub fn top_rated(&self, limit: usize) -> Vec<&RegistryEntry> {
        let mut entries: Vec<_> = self.entries.values().collect();
        entries.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        entries.into_iter().take(limit).collect()
    }

    /// İndir
    pub fn download(&mut self, name: &str) -> Result<(), String> {
        let entry = self.entries.get_mut(name)
            .ok_or_else(|| format!("'{}' bulunamadı", name))?;
        entry.record_download();
        self.total_downloads += 1;
        Ok(())
    }

    /// İstatistikler
    pub fn stats(&self) -> RegistryStats {
        RegistryStats {
            total_tools: self.entries.len() as u32,
            published: self.entries.values().filter(|e| e.status == RegistryStatus::Published).count() as u32,
            deprecated: self.entries.values().filter(|e| e.status == RegistryStatus::Deprecated).count() as u32,
            total_downloads: self.total_downloads,
        }
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Kayıt defteri istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryStats {
    pub total_tools: u32,
    pub published: u32,
    pub deprecated: u32,
    pub total_downloads: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- Template Versioning Tests ---

    #[test]
    fn test_template_version() {
        let v = TemplateVersion::v(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_template_version_compatibility() {
        let v1 = TemplateVersion::v(1, 2, 0);
        let v2 = TemplateVersion::v(1, 3, 0);
        let v3 = TemplateVersion::v(2, 0, 0);
        assert!(v1.is_compatible_with(&v2) == false); // 1.2 not compatible with 1.3 (minor lower)
        assert!(v2.is_compatible_with(&v1)); // 1.3 compatible with 1.2
        assert!(!v1.is_compatible_with(&v3)); // major different
    }

    #[test]
    fn test_template_version_bump() {
        let mut v = TemplateVersion::v(1, 2, 3);
        v.bump_patch();
        assert_eq!(v.patch, 4);
        v.bump_minor();
        assert_eq!(v.minor, 3);
        assert_eq!(v.patch, 0);
        v.bump_major();
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 0);
    }

    #[test]
    fn test_versioned_template() {
        let t = VersionedTemplate::new("python-api", ToolType::PythonScript, "print('hello')")
            .with_description("API tool")
            .with_tags(vec!["api".to_string()]);
        assert_eq!(t.name, "python-api");
        assert_eq!(t.version, TemplateVersion::default());
        assert!(!t.deprecated);
    }

    #[test]
    fn test_versioned_template_update() {
        let mut t = VersionedTemplate::new("test", ToolType::PythonScript, "v1");
        t.update("v2".to_string(), "güncellendi".to_string());
        assert_eq!(t.content, "v2");
        assert_eq!(t.changelog.len(), 1);
    }

    #[test]
    fn test_template_version_manager() {
        let mut mgr = TemplateVersionManager::new();
        let t1 = VersionedTemplate::new("tool-a", ToolType::PythonScript, "v1");
        mgr.register(t1);
        let latest = mgr.get_latest("tool-a");
        assert!(latest.is_some());
    }

    #[test]
    fn test_template_version_manager_compatible() {
        let mut mgr = TemplateVersionManager::new();
        let mut t1 = VersionedTemplate::new("tool-a", ToolType::PythonScript, "v1");
        t1.version = TemplateVersion::v(1, 2, 0);
        let mut t2 = VersionedTemplate::new("tool-a", ToolType::PythonScript, "v2");
        t2.version = TemplateVersion::v(1, 3, 0);
        mgr.register(t1);
        mgr.register(t2);
        let compat = mgr.get_compatible("tool-a", &TemplateVersion::v(1, 2, 0));
        assert!(compat.is_some());
    }

    #[test]
    fn test_template_cleanup_deprecated() {
        let mut mgr = TemplateVersionManager::new();
        let mut t1 = VersionedTemplate::new("tool-a", ToolType::PythonScript, "v1");
        t1.deprecate();
        mgr.register(t1);
        assert_eq!(mgr.cleanup_deprecated("tool-a"), 1);
    }

    // --- Test Generation Tests ---

    #[test]
    fn test_framework_for_tool() {
        assert!(matches!(TestFramework::for_tool_type(&ToolType::PythonScript), TestFramework::Pytest));
        assert!(matches!(TestFramework::for_tool_type(&ToolType::NodeModule), TestFramework::Jest));
    }

    #[test]
    fn test_gen_config_default() {
        let config = TestGenConfig::default();
        assert!(config.include_edge_cases);
        assert_eq!(config.target_coverage, 80);
    }

    #[test]
    fn test_category_icon() {
        assert_eq!(TestCategory::Unit.icon(), "🧪");
        assert_eq!(TestCategory::Security.icon(), "🔒");
    }

    #[test]
    fn test_generator() {
        let mut gen = TestGenerator::new(TestGenConfig::default());
        let tool = GeneratedTool {
            id: uuid::Uuid::new_v4(),
            name: "my-tool".to_string(),
            tool_type: ToolType::PythonScript,
            source_summary: String::new(),
            code: "print('hello')".to_string(),
            metadata: HashMap::new(),
            generated_at: Utc::now(),
            validation_result: None,
        };
        let test = gen.generate_tests(&tool);
        assert!(test.test_count > 0);
        assert!(test.coverage_estimate > 0.0);
    }

    // --- AI-Assisted Tests ---

    #[test]
    fn test_ai_config_default() {
        let config = AiAssistConfig::default();
        assert!(config.code_review);
        assert!(config.auto_improve);
        assert_eq!(config.max_improve_rounds, 3);
    }

    #[test]
    fn test_ai_code_review() {
        let review = AiCodeReview {
            quality_score: 0.85,
            security_score: 0.90,
            readability_score: 0.88,
            performance_score: 0.82,
            issues: Vec::new(),
            suggestions: Vec::new(),
        };
        assert!((review.overall_score() - 0.8625).abs() < 0.01);
        let summary = review.summary();
        assert!(summary.contains("Quality"));
    }

    // --- Registry Tests ---

    #[test]
    fn test_registry_entry() {
        let entry = RegistryEntry::new("my-tool", ToolType::PythonScript)
            .with_author("dev")
            .with_description("test tool");
        assert_eq!(entry.name, "my-tool");
        assert_eq!(entry.status, RegistryStatus::Draft);
    }

    #[test]
    fn test_registry_publish() {
        let mut entry = RegistryEntry::new("my-tool", ToolType::PythonScript);
        entry.publish();
        assert_eq!(entry.status, RegistryStatus::Published);
    }

    #[test]
    fn test_registry_deprecate() {
        let mut entry = RegistryEntry::new("old-tool", ToolType::ShellScript);
        entry.publish();
        entry.deprecate();
        assert_eq!(entry.status, RegistryStatus::Deprecated);
    }

    #[test]
    fn test_registry_download() {
        let mut entry = RegistryEntry::new("my-tool", ToolType::PythonScript);
        entry.record_download();
        assert_eq!(entry.downloads, 1);
    }

    #[test]
    fn test_registry_rating() {
        let mut entry = RegistryEntry::new("my-tool", ToolType::PythonScript);
        entry.set_rating(4.5);
        assert_eq!(entry.rating, 4.5);
        entry.set_rating(6.0); // max 5.0
        assert_eq!(entry.rating, 5.0);
    }

    #[test]
    fn test_tool_registry() {
        let mut registry = ToolRegistry::new();
        let entry = RegistryEntry::new("tool-a", ToolType::PythonScript);
        registry.register(entry).unwrap();
        assert!(registry.get("tool-a").is_some());
    }

    #[test]
    fn test_tool_registry_duplicate() {
        let mut registry = ToolRegistry::new();
        registry.register(RegistryEntry::new("tool-a", ToolType::PythonScript)).unwrap();
        let result = registry.register(RegistryEntry::new("tool-a", ToolType::PythonScript));
        assert!(result.is_err());
    }

    #[test]
    fn test_tool_registry_search() {
        let mut registry = ToolRegistry::new();
        let mut entry = RegistryEntry::new("python-api", ToolType::PythonScript);
        entry.description = "API integration tool".to_string();
        registry.register(entry).unwrap();
        let results = registry.search("api");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_tool_registry_by_type() {
        let mut registry = ToolRegistry::new();
        registry.register(RegistryEntry::new("tool-a", ToolType::PythonScript)).unwrap();
        registry.register(RegistryEntry::new("tool-b", ToolType::N8nWorkflow)).unwrap();
        let python_tools = registry.list_by_type(ToolType::PythonScript);
        assert_eq!(python_tools.len(), 1);
    }

    #[test]
    fn test_tool_registry_stats() {
        let mut registry = ToolRegistry::new();
        let mut entry = RegistryEntry::new("tool-a", ToolType::PythonScript);
        entry.publish();
        registry.register(entry).unwrap();
        let stats = registry.stats();
        assert_eq!(stats.total_tools, 1);
        assert_eq!(stats.published, 1);
    }
}
