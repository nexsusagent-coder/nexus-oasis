//! ─── SELF-HEALING (OTONOM DÜZELTME) ───
//!
//! 11. Adım: Hata alan kodun otonom olarak analiz edilmesi ve düzeltilmesi.
//! - Hata paterni analizi
//! - Otomatik kod düzeltme önerileri
//! - Yeniden deneme stratejileri
//! - Öğrenen hata veritabanı

use crate::goal::{Goal, Task, TaskResult, TaskStatus};
use crate::execution::{ExecutionResult, StepResult};
use crate::state::AgentState;
use sentient_common::error::{SENTIENTError, SENTIENTResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// ─── SELF-HEALING ENGINE ───
/// 
/// Hataları analiz eden ve otomatik düzeltmeler üreten motor.

pub struct SelfHealingEngine {
    /// Yapılandırma
    config: HealingConfig,
    /// Hata veritabanı (öğrenilen paternler)
    error_patterns: HashMap<String, ErrorPattern>,
    /// Düzeltme geçmişi
    healing_history: Vec<HealingRecord>,
    /// V-GATE URL (LLM erişimi için)
    vgate_url: String,
    /// Model
    model: String,
}

/// Self-Healing yapılandırması
#[derive(Debug, Clone)]
pub struct HealingConfig {
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Otomatik düzeltme aktif mi?
    pub auto_fix_enabled: bool,
    /// Kod analizi aktif mi?
    pub code_analysis_enabled: bool,
    /// Öğrenme aktif mi? (yeni paternleri kaydet)
    pub learning_enabled: bool,
    /// Düzeltme stratejileri
    pub strategies: Vec<HealingStrategy>,
}

impl Default for HealingConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            auto_fix_enabled: true,
            code_analysis_enabled: true,
            learning_enabled: true,
            strategies: vec![
                HealingStrategy::Retry,
                HealingStrategy::AlternativeTool,
                HealingStrategy::SimplifyTask,
                HealingStrategy::RequestHelp,
            ],
        }
    }
}

/// Düzeltme stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealingStrategy {
    /// Basit yeniden deneme
    Retry,
    /// Alternatif araç kullan
    AlternativeTool,
    /// Görevi basitleştir
    SimplifyTask,
    /// Kullanıcıdan yardım iste
    RequestHelp,
    /// Kodu yeniden yaz
    RewriteCode,
    /// Parametreleri ayarla
    AdjustParameters,
    /// Bağımlılıkları kontrol et
    CheckDependencies,
}

/// Hata paterni (öğrenilen)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    /// Pattern ID
    pub id: Uuid,
    /// Hata imzası (hash)
    pub signature: String,
    /// Hata kategorisi
    pub category: ErrorCategory,
    /// Düzeltme önerisi
    pub suggested_fix: String,
    /// Başarı oranı
    pub success_rate: f32,
    /// Kullanım sayısı
    pub usage_count: u32,
    /// Son kullanım
    pub last_used: DateTime<Utc>,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
}

/// Hata kategorisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Ağ hatası
    NetworkError,
    /// Timeout
    Timeout,
    /// Kaynak bulunamadı
    NotFound,
    /// Yetkilendirme hatası
    AuthError,
    /// Sözdizimi hatası
    SyntaxError,
    /// Mantık hatası
    LogicError,
    /// Bellek hatası
    MemoryError,
    /// Modül import hatası
    ImportError,
    /// Tip hatası
    TypeError,
    /// Değer hatası
    ValueError,
    /// Bilinmeyen
    Unknown,
}

impl ErrorCategory {
    /// Hata kategorisinden emoji al
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::NetworkError => "🌐",
            Self::Timeout => "⏰",
            Self::NotFound => "🔍",
            Self::AuthError => "🔐",
            Self::SyntaxError => "📝",
            Self::LogicError => "🧠",
            Self::MemoryError => "💾",
            Self::ImportError => "📦",
            Self::TypeError => "🏷️",
            Self::ValueError => "📊",
            Self::Unknown => "❓",
        }
    }
}

/// Düzeltme kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingRecord {
    /// Kayıt ID
    pub id: Uuid,
    /// İlgili görev ID
    pub task_id: Uuid,
    /// Orijinal hata
    pub original_error: String,
    /// Hata kategorisi
    pub category: ErrorCategory,
    /// Uygulanan strateji
    pub strategy: HealingStrategy,
    /// Düzeltme başarılı mı?
    pub success: bool,
    /// Deneme sayısı
    pub attempts: u32,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

/// Düzeltme sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingResult {
    /// Başarılı mı?
    pub success: bool,
    /// Uygulanan strateji
    pub strategy: HealingStrategy,
    /// Düzeltme açıklaması
    pub description: String,
    /// Yeni görev (varsa)
    pub new_task: Option<Task>,
    /// Önerilen kod düzeltmesi (varsa)
    pub code_fix: Option<CodeFix>,
    /// Deneme sayısı
    pub attempts: u32,
}

/// Kod düzeltmesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFix {
    /// Orijinal kod
    pub original_code: String,
    /// Düzeltilmiş kod
    pub fixed_code: String,
    /// Açıklama
    pub explanation: String,
    /// Satır numaraları (değişen)
    pub changed_lines: Vec<usize>,
}

impl SelfHealingEngine {
    /// Yeni self-healing motoru oluştur
    pub fn new(config: HealingConfig, vgate_url: String, model: String) -> Self {
        let mut engine = Self {
            config,
            error_patterns: HashMap::new(),
            healing_history: Vec::new(),
            vgate_url,
            model,
        };
        
        // Temel hata paternlerini yükle
        engine.load_default_patterns();
        
        engine
    }
    
    /// Varsayılan hata paternlerini yükle
    fn load_default_patterns(&mut self) {
        // Ağ hataları
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "connection_refused".into(),
            category: ErrorCategory::NetworkError,
            suggested_fix: "Hedef sunucu yanıt vermiyor. Alternatif endpoint dene veya bekle.".into(),
            success_rate: 0.7,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
        
        // Timeout
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "timeout".into(),
            category: ErrorCategory::Timeout,
            suggested_fix: "İşlem zaman aşımına uğradı. Task'ı daha küçük parçalara böl.".into(),
            success_rate: 0.8,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
        
        // Import hatası
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "ModuleNotFoundError".into(),
            category: ErrorCategory::ImportError,
            suggested_fix: "Modül yüklü değil. pip install ile yükle veya alternatif modül kullan.".into(),
            success_rate: 0.9,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
        
        // Syntax hatası
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "SyntaxError".into(),
            category: ErrorCategory::SyntaxError,
            suggested_fix: "Sözdizimi hatası var. Kodu yeniden incele ve düzelt.".into(),
            success_rate: 0.85,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
        
        // Type hatası
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "TypeError".into(),
            category: ErrorCategory::TypeError,
            suggested_fix: "Tip uyumsuzluğu var. Değerleri kontrol et ve gerekirse dönüştür.".into(),
            success_rate: 0.75,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
        
        // Value hatası
        self.add_pattern(ErrorPattern {
            id: Uuid::new_v4(),
            signature: "ValueError".into(),
            category: ErrorCategory::ValueError,
            suggested_fix: "Geçersiz değer. Girdileri doğrula ve düzelt.".into(),
            success_rate: 0.7,
            usage_count: 0,
            last_used: Utc::now(),
            created_at: Utc::now(),
        });
    }
    
    /// Pattern ekle
    pub fn add_pattern(&mut self, pattern: ErrorPattern) {
        self.error_patterns.insert(pattern.signature.clone(), pattern);
    }
    
    /// ─── ANA DÜZELTME FONKSİYONU ───
    /// 
    /// Hatalı bir görevi analiz eder ve düzeltmeye çalışır.
    
    pub async fn heal(&mut self, task: &Task, error: &str) -> SENTIENTResult<HealingResult> {
        let start_time = std::time::Instant::now();
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🔧  SELF-HEALING: Hata analiz ediliyor...");
        log::info!("════════════════════════════════════════════════════════════");
        log::debug!("   Görev: {}", task.description.chars().take(50).collect::<String>());
        log::debug!("   Hata: {}", error.chars().take(100).collect::<String>());
        
        // 1. Hata kategorisini belirle
        let category = self.categorize_error(error);
        log::info!("   {} Kategori: {:?}", category.emoji(), category);
        
        // 2. Bilinen bir pattern mi?
        let pattern = self.find_pattern(error);
        if let Some(ref p) = pattern {
            log::info!("   📚 Bilinen patern: {} (başarı: {:.0}%)", 
                p.signature, p.success_rate * 100.0
            );
        }
        
        // 3. Strateji seç
        let strategy = self.select_strategy(&category, pattern.as_ref());
        log::info!("   🎯 Strateji: {:?}", strategy);
        
        // 4. Düzeltmeyi uygula
        let result = self.apply_strategy(task, error, &strategy, pattern.as_ref()).await;
        
        let duration_ms = start_time.elapsed().as_millis() as u64;
        
        // 5. Kaydet
        let record = HealingRecord {
            id: Uuid::new_v4(),
            task_id: task.id,
            original_error: error.to_string(),
            category,
            strategy,
            success: result.success,
            attempts: result.attempts,
            duration_ms,
            timestamp: Utc::now(),
        };
        
        self.healing_history.push(record);
        
        log::info!("════════════════════════════════════════════════════════════");
        if result.success {
            log::info!("✅  SELF-HEALING başarılı: {}", result.description);
        } else {
            log::warn!("❌  SELF-HEALING başarısız: {}", result.description);
        }
        log::info!("════════════════════════════════════════════════════════════");
        
        Ok(result)
    }
    
    /// Hata kategorisini belirle
    fn categorize_error(&self, error: &str) -> ErrorCategory {
        let error_lower = error.to_lowercase();
        
        if error_lower.contains("connection") || error_lower.contains("network") || 
           error_lower.contains("socket") || error_lower.contains("refused") {
            return ErrorCategory::NetworkError;
        }
        
        if error_lower.contains("timeout") || error_lower.contains("timed out") {
            return ErrorCategory::Timeout;
        }
        
        if error_lower.contains("not found") || error_lower.contains("404") || 
           error_lower.contains("does not exist") {
            return ErrorCategory::NotFound;
        }
        
        if error_lower.contains("auth") || error_lower.contains("unauthorized") || 
           error_lower.contains("forbidden") || error_lower.contains("401") || 
           error_lower.contains("403") {
            return ErrorCategory::AuthError;
        }
        
        if error_lower.contains("syntax") || error_lower.contains("parse") {
            return ErrorCategory::SyntaxError;
        }
        
        if error_lower.contains("logic") || error_lower.contains("assertion") {
            return ErrorCategory::LogicError;
        }
        
        if error_lower.contains("memory") || error_lower.contains("oom") || 
           error_lower.contains("out of memory") {
            return ErrorCategory::MemoryError;
        }
        
        if error_lower.contains("import") || error_lower.contains("module") || 
           error_lower.contains("not found") && error_lower.contains("module") {
            return ErrorCategory::ImportError;
        }
        
        if error_lower.contains("type") || error_lower.contains("typeerror") {
            return ErrorCategory::TypeError;
        }
        
        if error_lower.contains("value") || error_lower.contains("valueerror") || 
           error_lower.contains("invalid") {
            return ErrorCategory::ValueError;
        }
        
        ErrorCategory::Unknown
    }
    
    /// Bilinen paterni bul
    fn find_pattern(&self, error: &str) -> Option<ErrorPattern> {
        let error_lower = error.to_lowercase();
        
        for (sig, pattern) in &self.error_patterns {
            if error_lower.contains(sig) || sig.to_lowercase().contains(&error_lower) {
                return Some(pattern.clone());
            }
        }
        
        None
    }
    
    /// Strateji seç
    fn select_strategy(&self, category: &ErrorCategory, pattern: Option<&ErrorPattern>) -> HealingStrategy {
        // Pattern'den gelen öneri
        if let Some(p) = pattern {
            if p.success_rate > 0.7 {
                // Yüksek başarı oranında, önce basit retry
                return HealingStrategy::Retry;
            }
        }
        
        // Kategoriye göre strateji
        match category {
            ErrorCategory::NetworkError => HealingStrategy::Retry,
            ErrorCategory::Timeout => HealingStrategy::SimplifyTask,
            ErrorCategory::NotFound => HealingStrategy::AlternativeTool,
            ErrorCategory::AuthError => HealingStrategy::RequestHelp,
            ErrorCategory::SyntaxError => HealingStrategy::RewriteCode,
            ErrorCategory::LogicError => HealingStrategy::RewriteCode,
            ErrorCategory::MemoryError => HealingStrategy::SimplifyTask,
            ErrorCategory::ImportError => HealingStrategy::AdjustParameters,
            ErrorCategory::TypeError => HealingStrategy::AdjustParameters,
            ErrorCategory::ValueError => HealingStrategy::AdjustParameters,
            ErrorCategory::Unknown => HealingStrategy::Retry,
        }
    }
    
    /// Stratejiyi uygula
    async fn apply_strategy(
        &mut self,
        task: &Task,
        error: &str,
        strategy: &HealingStrategy,
        pattern: Option<&ErrorPattern>
    ) -> HealingResult {
        match strategy {
            HealingStrategy::Retry => {
                self.apply_retry(task, error).await
            }
            HealingStrategy::AlternativeTool => {
                self.apply_alternative_tool(task, error).await
            }
            HealingStrategy::SimplifyTask => {
                self.apply_simplify_task(task, error).await
            }
            HealingStrategy::RequestHelp => {
                self.apply_request_help(task, error).await
            }
            HealingStrategy::RewriteCode => {
                self.apply_rewrite_code(task, error, pattern).await
            }
            HealingStrategy::AdjustParameters => {
                self.apply_adjust_parameters(task, error).await
            }
            HealingStrategy::CheckDependencies => {
                self.apply_check_dependencies(task, error).await
            }
        }
    }
    
    /// Retry stratejisi
    async fn apply_retry(&self, task: &Task, error: &str) -> HealingResult {
        if task.retry_count >= self.config.max_retries {
            return HealingResult {
                success: false,
                strategy: HealingStrategy::Retry,
                description: format!("Maksimum yeniden deneme sayısına ulaşıldı ({})", self.config.max_retries),
                new_task: None,
                code_fix: None,
                attempts: task.retry_count,
            };
        }
        
        // Yeni görev oluştur (retry count artırılmış)
        let mut new_task = task.clone();
        new_task.retry_count += 1;
        new_task.status = TaskStatus::Pending;
        
        HealingResult {
            success: true,
            strategy: HealingStrategy::Retry,
            description: format!("Görev yeniden denenecek (deneme {}/{})", 
                new_task.retry_count, self.config.max_retries),
            new_task: Some(new_task),
            code_fix: None,
            attempts: task.retry_count + 1,
        }
    }
    
    /// Alternatif araç stratejisi
    async fn apply_alternative_tool(&self, task: &Task, _error: &str) -> HealingResult {
        use crate::goal::ToolType;
        
        // Mevcut araca göre alternatif belirle
        let alternative = match task.tool {
            ToolType::BrowserNavigate => Some(ToolType::WebSearch),
            ToolType::BrowserClick => Some(ToolType::BrowserSearch),
            ToolType::WebSearch => Some(ToolType::BrowserNavigate),
            ToolType::SandboxExecute => Some(ToolType::LlmReason),
            ToolType::LlmQuery => Some(ToolType::LlmReason),
            _ => None,
        };
        
        match alternative {
            Some(alt_tool) => {
                let mut new_task = task.clone();
                new_task.tool = alt_tool.clone();
                new_task.status = TaskStatus::Pending;
                new_task.retry_count += 1;
                
                HealingResult {
                    success: true,
                    strategy: HealingStrategy::AlternativeTool,
                    description: format!("Alternatif araç kullanılacak: {:?}", alt_tool),
                    new_task: Some(new_task),
                    code_fix: None,
                    attempts: 1,
                }
            }
            None => {
                HealingResult {
                    success: false,
                    strategy: HealingStrategy::AlternativeTool,
                    description: "Bu araç için alternatif bulunamadı".into(),
                    new_task: None,
                    code_fix: None,
                    attempts: 0,
                }
            }
        }
    }
    
    /// Task basitleştirme stratejisi
    async fn apply_simplify_task(&self, task: &Task, _error: &str) -> HealingResult {
        // Görevi daha küçük parçalara böl
        let sub_tasks = self.split_task(task);
        
        if sub_tasks.is_empty() {
            return HealingResult {
                success: false,
                strategy: HealingStrategy::SimplifyTask,
                description: "Görev daha küçük parçalara bölünemedi".into(),
                new_task: None,
                code_fix: None,
                attempts: 0,
            };
        }
        
        // İlk alt görevi döndür
        let first_sub = sub_tasks.into_iter().next().unwrap();
        
        HealingResult {
            success: true,
            strategy: HealingStrategy::SimplifyTask,
            description: format!("Görev basitleştirildi: {}", 
                first_sub.description.chars().take(40).collect::<String>()),
            new_task: Some(first_sub),
            code_fix: None,
            attempts: 1,
        }
    }
    
    /// Task'ı parçalara böl
    fn split_task(&self, task: &Task) -> Vec<Task> {
        use crate::goal::ToolType;
        
        let mut sub_tasks = Vec::new();
        
        // Basit bölme mantığı
        match task.tool {
            ToolType::SandboxExecute => {
                // Kod çalıştırma -> önce syntax kontrol, sonra çalıştır
                sub_tasks.push(Task::new(
                    "Kodu syntax kontrol et",
                    ToolType::SandboxTest
                ).with_input(task.input.clone()));
                
                sub_tasks.push(Task::new(
                    "Kodu çalıştır",
                    ToolType::SandboxExecute
                ).with_input(task.input.clone()));
            }
            ToolType::BrowserNavigate => {
                // URL'ye git -> önce DNS kontrol, sonra navigate
                sub_tasks.push(Task::new(
                    "URL erişilebilirliğini kontrol et",
                    ToolType::WebSearch
                ));
                
                sub_tasks.push(Task::new(
                    &task.description,
                    ToolType::BrowserNavigate
                ).with_input(task.input.clone()));
            }
            _ => {
                // Diğer durumlar için bölme yok
            }
        }
        
        sub_tasks
    }
    
    /// Yardım isteme stratejisi
    async fn apply_request_help(&self, task: &Task, error: &str) -> HealingResult {
        // Bu strateji başarısız sayılır ama kullanıcıya bilgi verir
        HealingResult {
            success: false,
            strategy: HealingStrategy::RequestHelp,
            description: format!(
                "Kullanıcı yardımı gerekiyor. Görev: {}\nHata: {}",
                task.description.chars().take(50).collect::<String>(),
                error.chars().take(100).collect::<String>()
            ),
            new_task: None,
            code_fix: None,
            attempts: 0,
        }
    }
    
    /// Kod yeniden yazma stratejisi
    async fn apply_rewrite_code(
        &self,
        task: &Task,
        error: &str,
        pattern: Option<&ErrorPattern>
    ) -> HealingResult {
        // Kod analizi ve düzeltme önerisi
        let code_fix = if let Some(input_str) = task.input.as_str() {
            self.analyze_and_fix_code(input_str, error, pattern).await
        } else {
            None
        };
        
        match code_fix {
            Some(fix) => {
                let mut new_task = task.clone();
                new_task.input = serde_json::json!(fix.fixed_code);
                new_task.status = TaskStatus::Pending;
                
                HealingResult {
                    success: true,
                    strategy: HealingStrategy::RewriteCode,
                    description: format!("Kod düzeltildi: {}", fix.explanation),
                    new_task: Some(new_task),
                    code_fix: Some(fix),
                    attempts: 1,
                }
            }
            None => {
                HealingResult {
                    success: false,
                    strategy: HealingStrategy::RewriteCode,
                    description: "Kod analizi başarısız, manuel düzeltme gerekli".into(),
                    new_task: None,
                    code_fix: None,
                    attempts: 0,
                }
            }
        }
    }
    
    /// Parametre ayarlama stratejisi
    async fn apply_adjust_parameters(&self, task: &Task, error: &str) -> HealingResult {
        // Parametreleri analiz et ve düzelt
        let mut new_task = task.clone();
        
        // Basit parametre düzeltmeleri
        if error.to_lowercase().contains("import") {
            // Import hatası -> modül yükleme task'ı ekle
            return HealingResult {
                success: true,
                strategy: HealingStrategy::AdjustParameters,
                description: "Gerekli modüller yüklenecek".into(),
                new_task: Some(Task::new(
                    "Gerekli modülleri yükle",
                    crate::goal::ToolType::SandboxInstall
                )),
                code_fix: None,
                attempts: 1,
            };
        }
        
        if error.to_lowercase().contains("timeout") {
            // Timeout -> daha uzun bekleme
            if let Some(obj) = new_task.input.as_object_mut() {
                obj.insert("timeout".into(), serde_json::json!(60));
            }
            
            return HealingResult {
                success: true,
                strategy: HealingStrategy::AdjustParameters,
                description: "Timeout süresi artırıldı".into(),
                new_task: Some(new_task),
                code_fix: None,
                attempts: 1,
            };
        }
        
        HealingResult {
            success: false,
            strategy: HealingStrategy::AdjustParameters,
            description: "Parametre ayarlaması yapılamadı".into(),
            new_task: None,
            code_fix: None,
            attempts: 0,
        }
    }
    
    /// Bağımlılık kontrol stratejisi
    async fn apply_check_dependencies(&self, task: &Task, _error: &str) -> HealingResult {
        // Bağımlılıkları kontrol et
        let unmet = task.dependencies.iter().filter(|id| {
            // Basit simülasyon - gerçek implementasyonda task status kontrol edilir
            false
        }).count();
        
        if unmet > 0 {
            HealingResult {
                success: false,
                strategy: HealingStrategy::CheckDependencies,
                description: format!("{} bağımlılık karşılanmadı", unmet),
                new_task: None,
                code_fix: None,
                attempts: 0,
            }
        } else {
            HealingResult {
                success: true,
                strategy: HealingStrategy::CheckDependencies,
                description: "Tüm bağımlılıklar karşılandı".into(),
                new_task: Some(task.clone()),
                code_fix: None,
                attempts: 1,
            }
        }
    }
    
    /// Kod analizi ve düzeltme
    async fn analyze_and_fix_code(
        &self,
        code: &str,
        error: &str,
        pattern: Option<&ErrorPattern>
    ) -> Option<CodeFix> {
        // Basit syntax düzeltmeleri
        let mut fixed_code = code.to_string();
        let mut changed_lines = Vec::new();
        let mut explanation = String::new();
        
        // Yaygın hataları düzelt
        let fixes = vec![
            // Eksik parantez
            ("(\n", "()\n", "Eksik kapanış parantezi eklendi"),
            ("[\n", "[]\n", "Eksik kapanış köşeli parantez eklendi"),
            ("{\n", "{}\n", "Eksik kapanış süslü parantez eklendi"),
            // Çift virgül
            (",,", ",", "Çift virgül düzeltildi"),
            // Trailing whitespace (önemsiz ama düzelt)
            ("  \n", "\n", "Gereksiz boşluklar temizlendi"),
        ];
        
        for (orig, fix, desc) in fixes {
            if fixed_code.contains(orig) {
                fixed_code = fixed_code.replace(orig, fix);
                explanation.push_str(desc);
                explanation.push_str(". ");
            }
        }
        
        // Pattern'den gelen öneri
        if let Some(p) = pattern {
            explanation.push_str(&format!("Öneri: {}", p.suggested_fix));
        }
        
        if fixed_code != code {
            // Değişen satırları bul
            for (i, (l1, l2)) in code.lines().zip(fixed_code.lines()).enumerate() {
                if l1 != l2 {
                    changed_lines.push(i + 1);
                }
            }
            
            Some(CodeFix {
                original_code: code.to_string(),
                fixed_code,
                explanation: if explanation.is_empty() { 
                    "Kod düzeltildi".into() 
                } else { 
                    explanation 
                },
                changed_lines,
            })
        } else {
            None
        }
    }
    
    /// ─── RAPORLAMA ───
    
    /// İstatistikler
    pub fn stats(&self) -> HealingStats {
        let total = self.healing_history.len();
        let successful = self.healing_history.iter().filter(|r| r.success).count();
        
        let mut by_category: HashMap<ErrorCategory, u32> = HashMap::new();
        let mut by_strategy: HashMap<HealingStrategy, (u32, u32)> = HashMap::new();
        
        for record in &self.healing_history {
            *by_category.entry(record.category).or_insert(0) += 1;
            
            let entry = by_strategy.entry(record.strategy).or_insert((0, 0));
            if record.success {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
        
        HealingStats {
            total_healings: total as u32,
            successful_healings: successful as u32,
            failed_healings: (total - successful) as u32,
            success_rate: if total > 0 { successful as f32 / total as f32 } else { 0.0 },
            patterns_learned: self.error_patterns.len() as u32,
            by_category,
            by_strategy,
        }
    }
    
    /// Rapor
    pub fn report(&self) -> String {
        let stats = self.stats();
        
        format!(
            r#"
════════════════════════════════════════════════════════════
  🔧 SELF-HEALING RAPORU
════════════════════════════════════════════════════════════
  Toplam Düzeltme:     {}
  Başarılı:            {}
  Başarısız:           {}
  Başarı Oranı:        {:.1}%
  Öğrenilen Patern:    {}
  ────────────────────────────────────────────────────────────
  Son 5 Düzeltme:
{}════════════════════════════════════════════════════════════"#,
            stats.total_healings,
            stats.successful_healings,
            stats.failed_healings,
            stats.success_rate * 100.0,
            stats.patterns_learned,
            self.healing_history.iter().rev().take(5)
                .map(|r| format!(
                    "    {} {:?} -> {}\n",
                    if r.success { "✓" } else { "✗" },
                    r.strategy,
                    r.original_error.chars().take(40).collect::<String>()
                ))
                .collect::<String>()
        )
    }
}

/// Healing istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingStats {
    pub total_healings: u32,
    pub successful_healings: u32,
    pub failed_healings: u32,
    pub success_rate: f32,
    pub patterns_learned: u32,
    pub by_category: HashMap<ErrorCategory, u32>,
    pub by_strategy: HashMap<HealingStrategy, (u32, u32)>,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goal::ToolType;
    
    #[test]
    fn test_healing_config_default() {
        let config = HealingConfig::default();
        assert!(config.auto_fix_enabled);
        assert_eq!(config.max_retries, 3);
    }
    
    #[test]
    fn test_error_categorization() {
        let engine = SelfHealingEngine::new(
            HealingConfig::default(),
            "http://localhost:1071".into(),
            "test".into()
        );
        
        assert_eq!(
            engine.categorize_error("Connection refused"),
            ErrorCategory::NetworkError
        );
        
        assert_eq!(
            engine.categorize_error("Timeout occurred"),
            ErrorCategory::Timeout
        );
        
        assert_eq!(
            engine.categorize_error("SyntaxError: invalid syntax"),
            ErrorCategory::SyntaxError
        );
    }
    
    #[test]
    fn test_strategy_selection() {
        let engine = SelfHealingEngine::new(
            HealingConfig::default(),
            "http://localhost:1071".into(),
            "test".into()
        );
        
        let strategy = engine.select_strategy(&ErrorCategory::NetworkError, None);
        assert_eq!(strategy, HealingStrategy::Retry);
        
        let strategy = engine.select_strategy(&ErrorCategory::SyntaxError, None);
        assert_eq!(strategy, HealingStrategy::RewriteCode);
    }
    
    #[tokio::test]
    async fn test_retry_strategy() {
        let mut engine = SelfHealingEngine::new(
            HealingConfig::default(),
            "http://localhost:1071".into(),
            "test".into()
        );
        
        let task = Task::new("Test", ToolType::Calculator);
        let result = engine.heal(&task, "Network error").await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.strategy, HealingStrategy::Retry);
    }
    
    #[test]
    fn test_pattern_loading() {
        let engine = SelfHealingEngine::new(
            HealingConfig::default(),
            "http://localhost:1071".into(),
            "test".into()
        );
        
        assert!(engine.error_patterns.contains_key(&"timeout".to_string()));
        assert!(engine.error_patterns.contains_key(&"SyntaxError".to_string()));
    }
}
