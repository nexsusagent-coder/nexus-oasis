//! ═════════════════════════════════════════════════════════════════════════════════
//!  SENTIENT DYNAMIC ROUTER v1.0.0 - Complexity-Based Model Selection
//! ═════════════════════════════════════════════════════════════════════════════════
//! 
//!  Görev zorluğunu analiz edip en uygun/en ucuz modeli seçen otonom router.
//!  Human-in-the-Loop ile onay mekanizması.

use sentient_settings::{
    KeyRing, KeyRingManager, ApiKeyEntry, 
    KeyRingModelInfo, 
    ComplexityLevel, RoutingMode, ModelApprovalRequest, ModelApprovalResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;

// ═════════════════════════════════════════════════════════════════════════════════
//  TASK COMPLEXITY ANALYZER - Görev Zorluk Analizi
// ═════════════════════════════════════════════════════════════════════════════════

/// Görev tipi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    /// Basit soru/cevap
    SimpleQA,
    /// Metin özetleme
    Summarization,
    /// Kod açıklama
    CodeExplanation,
    /// Kod yazma
    CodeGeneration,
    /// Analiz/reasoning
    Analysis,
    /// Çok adımlı görev
    MultiStepTask,
    /// Araştırma
    Research,
    /// Sistem tasarımı
    SystemDesign,
    /// Kreatif yazma
    CreativeWriting,
    /// Veri işleme
    DataProcessing,
    /// Web etkileşimi
    WebInteraction,
    /// Dosya işlemleri
    FileOperations,
}

/// Görev analizi sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalysis {
    /// Algılanan görev tipi
    pub task_type: TaskType,
    /// Zorluk seviyesi
    pub complexity: ComplexityLevel,
    /// Tahmini token sayısı (input)
    pub estimated_input_tokens: u64,
    /// Tahmini token sayısı (output)
    pub estimated_output_tokens: u64,
    /// Zorluk faktörleri
    pub complexity_factors: Vec<String>,
    /// Güven skoru (0.0 - 1.0)
    pub confidence: f32,
    /// Önerilen model tier
    pub recommended_tier: String,
}

/// Görev zorluk analizi
pub struct ComplexityAnalyzer {
    /// Anahtar kelimeler ve zorluk eşleşmeleri
    complexity_keywords: HashMap<String, ComplexityLevel>,
    /// Task type keywords
    task_type_keywords: HashMap<TaskType, Vec<String>>,
}

impl ComplexityAnalyzer {
    /// Yeni analyzer oluştur
    pub fn new() -> Self {
        let mut complexity_keywords = HashMap::new();
        
        // Basit görev kelimeleri
        for keyword in &["merhaba", "selam", "nasılsın", "ne", "kaç", "hangi", "kim", "hello", "hi", "what is", "who is"] {
            complexity_keywords.insert(keyword.to_lowercase(), ComplexityLevel::Simple);
        }
        
        // Orta görev kelimeleri
        for keyword in &["özetle", "açıkla", "anlat", "analyze", "explain", "summarize", "compare", "nasıl çalışır"] {
            complexity_keywords.insert(keyword.to_lowercase(), ComplexityLevel::Medium);
        }
        
        // Karmaşık görev kelimeleri
        for keyword in &["yaz", "kod", "geliştir", "implement", "create", "build", "design", "refactor", "debug", "fix"] {
            complexity_keywords.insert(keyword.to_lowercase(), ComplexityLevel::Complex);
        }
        
        // Çok karmaşık görev kelimeleri
        for keyword in &["mimari", "sistem tasarla", "optimize", "architecture", "migrate", "scale", "redesign", "distribute"] {
            complexity_keywords.insert(keyword.to_lowercase(), ComplexityLevel::VeryComplex);
        }
        
        // Task type keywords
        let mut task_type_keywords = HashMap::new();
        
        task_type_keywords.insert(TaskType::SimpleQA, vec![
            "merhaba", "selam", "nasılsın", "hello", "hi", "what is", "who is", "ne", "kim"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::Summarization, vec![
            "özetle", "özet", "summarize", "summary", "kısa", "kısaca"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::CodeExplanation, vec![
            "kod açıkla", "bu kod ne yapıyor", "explain code", "nasıl çalışır"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::CodeGeneration, vec![
            "yaz", "kod yaz", "implement", "oluştur", "create", "build", "geliştir", "develop"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::Analysis, vec![
            "analiz", "analyze", "incele", "examine", "değerlendir", "evaluate"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::MultiStepTask, vec![
            "adım", "sırayla", "önce", "sonra", "step", "then", "multi", "süreç"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::Research, vec![
            "araştır", "research", "bul", "ara", "search", "investigate", "incele"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::SystemDesign, vec![
            "mimari", "tasarla", "architecture", "design", "sistem", "system", "yapı"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::WebInteraction, vec![
            "web", "site", "browser", "tara", "scrape", "aç", "click", "form"
        ].into_iter().map(|s| s.to_string()).collect());
        
        task_type_keywords.insert(TaskType::FileOperations, vec![
            "dosya", "file", "oku", "read", "yaz", "write", "kopyala", "copy", "sil", "delete"
        ].into_iter().map(|s| s.to_string()).collect());
        
        Self {
            complexity_keywords,
            task_type_keywords,
        }
    }
    
    /// Görevi analiz et
    pub fn analyze(&self, task_description: &str) -> TaskAnalysis {
        let lower_desc = task_description.to_lowercase();
        
        // Zorluk seviyesi tespit
        let mut complexity = ComplexityLevel::Simple;
        let mut complexity_factors = Vec::new();
        
        for (keyword, level) in &self.complexity_keywords {
            if lower_desc.contains(keyword) {
                // Daha yüksek zorluk seviyesini优先 et
                if self.compare_complexity(level, &complexity) {
                    complexity = *level;
                    complexity_factors.push(format!("Keyword: '{}'", keyword));
                }
            }
        }
        
        // Ek zorluk faktörleri
        let word_count = lower_desc.split_whitespace().count();
        if word_count > 100 {
            complexity = self.max_complexity(&complexity, &ComplexityLevel::Medium);
            complexity_factors.push(format!("Long task ({} words)", word_count));
        }
        if word_count > 300 {
            complexity = self.max_complexity(&complexity, &ComplexityLevel::Complex);
            complexity_factors.push(format!("Very long task ({} words)", word_count));
        }
        
        // Çoklu görev kontrolü
        if lower_desc.contains("ve") || lower_desc.contains("sonra") || lower_desc.contains("ve ardından") {
            complexity = self.max_complexity(&complexity, &ComplexityLevel::Medium);
            complexity_factors.push("Multiple tasks detected".to_string());
        }
        
        // Kod bloğu kontrolü
        if lower_desc.contains("```") || lower_desc.contains("fn ") || lower_desc.contains("def ") {
            complexity = self.max_complexity(&complexity, &ComplexityLevel::Complex);
            complexity_factors.push("Code block detected".to_string());
        }
        
        // Task type tespit
        let task_type = self.detect_task_type(&lower_desc);
        
        // Token tahmini
        let estimated_input_tokens = (word_count as f64 * 1.3) as u64; // Approximate
        let estimated_output_tokens = match complexity {
            ComplexityLevel::Simple => 100,
            ComplexityLevel::Medium => 500,
            ComplexityLevel::Complex => 2000,
            ComplexityLevel::VeryComplex => 5000,
        };
        
        // Güven skoru
        let confidence = if complexity_factors.is_empty() {
            0.5 // Varsayılan, düşük güven
        } else {
            (0.7 + (complexity_factors.len() as f32 * 0.1)).min(0.95)
        };
        
        TaskAnalysis {
            task_type,
            complexity,
            estimated_input_tokens,
            estimated_output_tokens,
            complexity_factors,
            confidence,
            recommended_tier: complexity.recommended_model_tier().to_string(),
        }
    }
    
    /// Task type tespit et
    fn detect_task_type(&self, lower_desc: &str) -> TaskType {
        let mut best_match: Option<TaskType> = None;
        let mut best_count = 0;
        
        for (task_type, keywords) in &self.task_type_keywords {
            let count = keywords.iter()
                .filter(|k| lower_desc.contains(k.to_lowercase().as_str()))
                .count();
            
            if count > best_count {
                best_count = count;
                best_match = Some(task_type.clone());
            }
        }
        
        best_match.unwrap_or(TaskType::SimpleQA)
    }
    
    /// İki zorluk seviyesini karşılaştır (daha yüksek olanı döndür)
    fn compare_complexity(&self, a: &ComplexityLevel, b: &ComplexityLevel) -> bool {
        let a_val = match a {
            ComplexityLevel::Simple => 1,
            ComplexityLevel::Medium => 2,
            ComplexityLevel::Complex => 3,
            ComplexityLevel::VeryComplex => 4,
        };
        let b_val = match b {
            ComplexityLevel::Simple => 1,
            ComplexityLevel::Medium => 2,
            ComplexityLevel::Complex => 3,
            ComplexityLevel::VeryComplex => 4,
        };
        a_val > b_val
    }
    
    /// İki zorluk seviyesinden maksimum olanı döndür
    fn max_complexity(&self, a: &ComplexityLevel, b: &ComplexityLevel) -> ComplexityLevel {
        if self.compare_complexity(a, b) {
            *a
        } else {
            *b
        }
    }
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  DYNAMIC ROUTER - Model Seçim Router'ı
// ═════════════════════════════════════════════════════════════════════════════════

/// Router konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    /// Maliyet optimizasyonu aktif mi?
    pub cost_optimization: bool,
    /// Minimum kalite tier'ı
    pub min_quality_tier: String,
    /// Prefer free models when possible
    pub prefer_free: bool,
    /// Maximum latency threshold (ms)
    pub max_latency_ms: u32,
    /// Fallback model (emergency)
    pub fallback_model: String,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            cost_optimization: true,
            min_quality_tier: "mini".into(),
            prefer_free: true,
            max_latency_ms: 5000,
            fallback_model: "qwen/qwen3-1.7b:free".into(),
        }
    }
}

/// Router kararı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterDecision {
    /// Seçilen model
    pub model: KeyRingModelInfo,
    /// Seçilen key
    pub key: ApiKeyEntry,
    /// Analiz sonucu
    pub analysis: TaskAnalysis,
    /// Tahmini maliyet
    pub estimated_cost: f64,
    /// Seçim nedeni
    pub reason: String,
    /// Alternatifler
    pub alternatives: Vec<(KeyRingModelInfo, ApiKeyEntry, f64)>,
}

/// Onay callback tipi
pub type ApprovalCallback = Box<dyn Fn(ModelApprovalRequest) -> Option<ModelApprovalResponse> + Send + Sync>;

/// Dynamic Router
pub struct DynamicRouter {
    /// KeyRing manager
    keyring_manager: Arc<KeyRingManager>,
    /// Complexity analyzer
    analyzer: ComplexityAnalyzer,
    /// Router config
    config: RouterConfig,
    /// Onay callback (Human-in-the-Loop)
    approval_callback: Option<Arc<ApprovalCallback>>,
    /// Approval channel (async)
    approval_tx: Option<mpsc::Sender<ModelApprovalRequest>>,
    approval_rx: Option<Arc<RwLock<Option<ModelApprovalResponse>>>>,
}

impl DynamicRouter {
    /// Yeni router oluştur
    pub fn new(keyring_manager: Arc<KeyRingManager>) -> Self {
        Self {
            keyring_manager,
            analyzer: ComplexityAnalyzer::new(),
            config: RouterConfig::default(),
            approval_callback: None,
            approval_tx: None,
            approval_rx: None,
        }
    }
    
    /// Config ile oluştur
    pub fn with_config(keyring_manager: Arc<KeyRingManager>, config: RouterConfig) -> Self {
        Self {
            keyring_manager,
            analyzer: ComplexityAnalyzer::new(),
            config,
            approval_callback: None,
            approval_tx: None,
            approval_rx: None,
        }
    }
    
    /// Onay callback'i ayarla
    pub fn set_approval_callback<F>(&mut self, callback: F)
    where
        F: Fn(ModelApprovalRequest) -> Option<ModelApprovalResponse> + Send + Sync + 'static,
    {
        self.approval_callback = Some(Arc::new(Box::new(callback)));
    }
    
    /// Routing modunu al
    pub async fn get_routing_mode(&self) -> RoutingMode {
        let keyring = self.keyring_manager.get_keyring().await;
        keyring.routing_mode
    }
    
    /// Görev için en uygun modeli seç (ANA FONKSİYON)
    pub async fn route(&self, task_description: &str) -> anyhow::Result<RouterDecision> {
        log::info!("🎯 DYNAMIC ROUTER: Görev analiz ediliyor...");
        
        // 1. Görevi analiz et
        let analysis = self.analyzer.analyze(task_description);
        
        log::info!("   📊 Zorluk: {:?} (güven: {:.0}%)", analysis.complexity, analysis.confidence * 100.0);
        log::info!("   📋 Task Type: {:?}", analysis.task_type);
        log::info!("   🎯 Önerilen Tier: {}", analysis.recommended_tier);
        
        // 2. KeyRing'den aktif key'leri al
        let keyring = self.keyring_manager.get_keyring().await;
        let active_keys = keyring.get_active_keys();
        
        if active_keys.is_empty() {
            log::warn!("⚠️  Aktif API key yok! Fallback model kullanılacak.");
            return self.create_fallback_decision(analysis.clone());
        }
        
        // 3. Uygun modelleri filtrele ve sırala
        let candidates = self.find_candidates(&keyring, &analysis);
        
        if candidates.is_empty() {
            log::warn!("⚠️  Uygun model bulunamadı! Fallback model kullanılacak.");
            return self.create_fallback_decision(analysis.clone());
        }
        
        // 4. En uygun modeli seç
        let (best_model, best_key, best_cost) = self.select_best(&candidates, &analysis);
        
        log::info!("   ✅ Seçilen Model: {} ({})", best_model.display_name, best_key.name);
        log::info!("   💰 Tahmini Maliyet: ${:.4}", best_cost);
        
        // 5. Human-in-the-Loop kontrolü
        let routing_mode = keyring.routing_mode;
        
        match routing_mode {
            RoutingMode::FullyAutonomous => {
                // Tam otonom - direkt karar ver
                let task_type = analysis.task_type.clone();
                Ok(RouterDecision {
                    model: best_model.clone(),
                    key: best_key.clone(),
                    analysis,
                    estimated_cost: best_cost,
                    reason: format!("Auto-selected for {:?} task", task_type),
                    alternatives: candidates.into_iter().take(5).collect(),
                })
            }
            RoutingMode::RequireApproval | RoutingMode::ApprovalWithManualOverride => {
                // Onay gerekiyor
                self.request_approval(best_model, best_key, candidates, analysis).await
            }
        }
    }
    
    /// Aday modelleri bul
    fn find_candidates(&self, keyring: &KeyRing, analysis: &TaskAnalysis) -> Vec<(KeyRingModelInfo, ApiKeyEntry, f64)> {
        let mut candidates = Vec::new();
        
        for key in keyring.get_active_keys() {
            let models = keyring.get_models_for_key(key);
            
            for model in models {
                // Zorluk seviyesi kontrolü
                if !self.is_model_suitable(model, &analysis.complexity) {
                    continue;
                }
                
                // Tier kontrolü
                if !self.is_tier_acceptable(&model.tier) {
                    continue;
                }
                
                // Latency kontrolü
                if model.avg_latency_ms > self.config.max_latency_ms {
                    continue;
                }
                
                // Maliyet hesapla
                let cost = model.calculate_cost(
                    analysis.estimated_input_tokens,
                    analysis.estimated_output_tokens,
                );
                
                // Free model kontrolü
                if self.config.prefer_free && cost == 0.0 {
                    candidates.insert(0, (model.clone(), key.clone(), cost));
                } else {
                    candidates.push((model.clone(), key.clone(), cost));
                }
            }
        }
        
        // Maliyete göre sırala (eğer cost_optimization aktifse)
        if self.config.cost_optimization {
            candidates.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        }
        
        candidates
    }
    
    /// Model zorluk seviyesine uygun mu?
    fn is_model_suitable(&self, model: &KeyRingModelInfo, required: &ComplexityLevel) -> bool {
        let model_level = match model.max_complexity {
            ComplexityLevel::Simple => 1,
            ComplexityLevel::Medium => 2,
            ComplexityLevel::Complex => 3,
            ComplexityLevel::VeryComplex => 4,
        };
        
        let required_level = match required {
            ComplexityLevel::Simple => 1,
            ComplexityLevel::Medium => 2,
            ComplexityLevel::Complex => 3,
            ComplexityLevel::VeryComplex => 4,
        };
        
        model_level >= required_level
    }
    
    /// Tier kabul edilebilir mi?
    fn is_tier_acceptable(&self, tier: &str) -> bool {
        let min_level = match self.config.min_quality_tier.as_str() {
            "mini" => 1,
            "standard" => 2,
            "advanced" => 3,
            "premium" => 4,
            _ => 1,
        };
        
        let tier_level = match tier {
            "mini" => 1,
            "standard" => 2,
            "advanced" => 3,
            "premium" => 4,
            _ => 1,
        };
        
        tier_level >= min_level
    }
    
    /// En iyi adayı seç
    fn select_best(
        &self,
        candidates: &[(KeyRingModelInfo, ApiKeyEntry, f64)],
        analysis: &TaskAnalysis,
    ) -> (KeyRingModelInfo, ApiKeyEntry, f64) {
        // Öncelikle free ve uygun modellere bak
        for (model, key, cost) in candidates {
            if *cost == 0.0 && self.is_model_suitable(model, &analysis.complexity) {
                return (model.clone(), key.clone(), *cost);
            }
        }
        
        // Değilse ilk uygun olanı seç (zaten sıralı)
        candidates.first()
            .map(|(m, k, c)| (m.clone(), k.clone(), *c))
            .unwrap_or_else(|| {
                // Emergency fallback
                (
                    KeyRingModelInfo::get_predefined_models().into_iter().next().unwrap(),
                    ApiKeyEntry::new("Fallback", "custom", ""),
                    0.0,
                )
            })
    }
    
    /// Onay iste
    async fn request_approval(
        &self,
        model: KeyRingModelInfo,
        key: ApiKeyEntry,
        candidates: Vec<(KeyRingModelInfo, ApiKeyEntry, f64)>,
        analysis: TaskAnalysis,
    ) -> anyhow::Result<RouterDecision> {
        let request = ModelApprovalRequest {
            id: uuid::Uuid::new_v4().to_string(),
            task_description: analysis.task_type.to_string(),
            detected_complexity: analysis.complexity,
            recommended_model: model.clone(),
            recommended_key_id: key.id.clone(),
            alternatives: candidates.iter().take(5).map(|(m, _, _)| m.clone()).collect(),
            estimated_cost: model.calculate_cost(analysis.estimated_input_tokens, analysis.estimated_output_tokens),
            estimated_duration_secs: (model.avg_latency_ms / 1000) as u32,
            created_at: chrono::Utc::now(),
        };
        
        // Onay callback varsa çağır
        if let Some(callback) = &self.approval_callback {
            if let Some(response) = callback(request.clone()) {
                // Kullanıcı farklı model seçmiş olabilir
                let keyring = self.keyring_manager.get_keyring().await;
                
                if response.selected_model != model.id {
                    // Farklı model seçilmiş
                    if let Some((new_model, new_key, new_cost)) = candidates.iter()
                        .find(|(m, _, _)| m.id == response.selected_model)
                    {
                        return Ok(RouterDecision {
                            model: new_model.clone(),
                            key: new_key.clone(),
                            analysis,
                            estimated_cost: *new_cost,
                            reason: "User-selected model".to_string(),
                            alternatives: candidates.into_iter().take(5).collect(),
                        });
                    }
                }
                
                // Önerilen model onaylandı
                return Ok(RouterDecision {
                    model,
                    key,
                    analysis,
                    estimated_cost: request.estimated_cost,
                    reason: "User approved".to_string(),
                    alternatives: candidates.into_iter().take(5).collect(),
                });
            }
        }
        
        // Callback yoksa varsayılan olarak onayla
        log::warn!("⚠️  Approval callback set edilmemiş, otomatik onaylanıyor.");
        Ok(RouterDecision {
            model,
            key,
            analysis,
            estimated_cost: request.estimated_cost,
            reason: "Auto-approved (no callback)".to_string(),
            alternatives: candidates.into_iter().take(5).collect(),
        })
    }
    
    /// Fallback karar oluştur
    fn create_fallback_decision(&self, analysis: TaskAnalysis) -> anyhow::Result<RouterDecision> {
        let fallback_model = KeyRingModelInfo::get_predefined_models()
            .into_iter()
            .find(|m| m.id == self.config.fallback_model)
            .unwrap_or_else(|| KeyRingModelInfo::get_predefined_models().into_iter().next().unwrap());
        
        Ok(RouterDecision {
            model: fallback_model.clone(),
            key: ApiKeyEntry::new("Fallback", "ollama", ""),
            analysis,
            estimated_cost: 0.0,
            reason: "Fallback model (no active keys)".to_string(),
            alternatives: vec![],
        })
    }
}

// TaskType için Display implementation
impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::SimpleQA => write!(f, "Basit Soru/Cevap"),
            TaskType::Summarization => write!(f, "Metin Özetleme"),
            TaskType::CodeExplanation => write!(f, "Kod Açıklama"),
            TaskType::CodeGeneration => write!(f, "Kod Yazma"),
            TaskType::Analysis => write!(f, "Analiz"),
            TaskType::MultiStepTask => write!(f, "Çok Adımlı Görev"),
            TaskType::Research => write!(f, "Araştırma"),
            TaskType::SystemDesign => write!(f, "Sistem Tasarımı"),
            TaskType::CreativeWriting => write!(f, "Kreatif Yazma"),
            TaskType::DataProcessing => write!(f, "Veri İşleme"),
            TaskType::WebInteraction => write!(f, "Web Etkileşimi"),
            TaskType::FileOperations => write!(f, "Dosya İşlemleri"),
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_complexity_analyzer_simple() {
        let analyzer = ComplexityAnalyzer::new();
        let analysis = analyzer.analyze("Merhaba, nasılsın?");
        
        assert_eq!(analysis.complexity, ComplexityLevel::Simple);
    }
    
    #[test]
    fn test_complexity_analyzer_complex() {
        let analyzer = ComplexityAnalyzer::new();
        let analysis = analyzer.analyze("Bana bir web scraper yaz, Python kullanarak bir siteden veri çeksin");
        
        assert!(matches!(analysis.complexity, ComplexityLevel::Complex | ComplexityLevel::VeryComplex));
    }
    
    #[test]
    fn test_complexity_analyzer_code() {
        let analyzer = ComplexityAnalyzer::new();
        let analysis = analyzer.analyze("Bu kodu açıkla: fn main() { println!(\"Hello\"); }");
        
        assert_eq!(analysis.task_type, TaskType::CodeExplanation);
    }
    
    #[test]
    fn test_task_analysis() {
        let analyzer = ComplexityAnalyzer::new();
        
        // Simple QA
        let analysis = analyzer.analyze("What is Rust?");
        assert_eq!(analysis.task_type, TaskType::SimpleQA);
        
        // Code generation
        let analysis = analyzer.analyze("Write a function to sort an array");
        assert!(matches!(analysis.task_type, TaskType::CodeGeneration));
    }
    
    #[test]
    fn test_model_info_cost() {
        let model = KeyRingModelInfo::get_predefined_models()
            .into_iter()
            .find(|m| m.id == "gpt-4-turbo")
            .unwrap();
        
        let cost = model.calculate_cost(1_000_000, 1_000_000);
        assert!((cost - 40.0).abs() < 0.01); // 10 + 30 = 40
    }
    
    #[tokio::test]
    async fn test_dynamic_router() {
        let keyring_manager = Arc::new(KeyRingManager::new());
        let router = DynamicRouter::new(keyring_manager);
        
        // Fallback test (no keys)
        let result = router.route("Merhaba").await;
        assert!(result.is_ok());
    }
}
