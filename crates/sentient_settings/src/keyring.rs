//! ═════════════════════════════════════════════════════════════════════════════════
//!  SENTIENT KEY RING v1.0.0 - Multi-Key Vault & Dynamic Model Routing
//! ═════════════════════════════════════════════════════════════════════════════════
//! 
//!  Sınırsız API key ve provider yönetimi.
//!  Dynamic model routing ile en uygun/ucuz model seçimi.
//!  Human-in-the-Loop ile onay mekanizması.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ═════════════════════════════════════════════════════════════════════════════════
//  API KEY ENTRY - Tek bir API anahtarı
// ═════════════════════════════════════════════════════════════════════════════════

/// API Key durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyStatus {
    Active,
    Disabled,
    Expired,
    RateLimited,
    QuotaExceeded,
}

/// Tek bir API anahtarı girişi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    /// Benzersiz ID
    pub id: String,
    /// Anahtar adı (ör: "OpenAI Main", "Claude Backup")
    pub name: String,
    /// Provider (openai, anthropic, google, groq, custom, vb.)
    pub provider: String,
    /// API anahtarı (şifreli saklanır)
    pub key: String,
    /// Base URL (custom provider'lar için)
    pub base_url: Option<String>,
    /// API format (openai, anthropic, custom)
    pub api_format: String,
    /// Bu key ile kullanılabilir modeller
    pub available_models: Vec<ModelInfo>,
    /// Durum
    pub status: KeyStatus,
    /// Öncelik (1 = en yüksek)
    pub priority: u8,
    /// Günlük kullanım limiti (0 = limitsiz)
    pub daily_limit: u64,
    /// Bugünkü kullanım
    pub daily_usage: u64,
    /// Maliyet oranı (1.0 = standart, 0.5 = ucuz, 2.0 = pahalı)
    pub cost_factor: f32,
    /// Ortalama response süresi (ms)
    pub avg_latency_ms: u32,
    /// Son kullanım
    pub last_used: Option<DateTime<Utc>>,
    /// Oluşturulma tarihi
    pub created_at: DateTime<Utc>,
    /// Etiketler
    pub tags: Vec<String>,
    /// Ek meta veriler
    pub metadata: HashMap<String, String>,
}

impl ApiKeyEntry {
    /// Yeni API key oluştur
    pub fn new(name: &str, provider: &str, key: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            provider: provider.to_string(),
            key: key.to_string(),
            base_url: None,
            api_format: "openai".to_string(),
            available_models: vec![],
            status: KeyStatus::Active,
            priority: 5,
            daily_limit: 0,
            daily_usage: 0,
            cost_factor: 1.0,
            avg_latency_ms: 500,
            last_used: None,
            created_at: Utc::now(),
            tags: vec![],
            metadata: HashMap::new(),
        }
    }
    
    /// Key kullanılabilir mi?
    pub fn is_available(&self) -> bool {
        match self.status {
            KeyStatus::Active => {
                if self.daily_limit > 0 && self.daily_usage >= self.daily_limit {
                    false
                } else {
                    true
                }
            }
            _ => false,
        }
    }
    
    /// Kullanımı artır
    pub fn increment_usage(&mut self) {
        self.daily_usage += 1;
        self.last_used = Some(Utc::now());
    }
    
    /// Günlük kullanımı sıfırla
    pub fn reset_daily_usage(&mut self) {
        self.daily_usage = 0;
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  MODEL INFO - Model bilgileri
// ═════════════════════════════════════════════════════════════════════════════════

/// Model zorluk seviyesi
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplexityLevel {
    /// Basit görevler: selamlaşma, basit sorgular, short text
    Simple,
    /// Orta görevler: özetleme, analiz, kod açıklama
    Medium,
    /// Karmaşık görevler: kod yazma, multi-step reasoning, research
    Complex,
    /// Çok karmaşık: sistem tasarımı, mimari kararlar
    VeryComplex,
}

impl ComplexityLevel {
    /// Zorluk seviyesine göre önerilen minimum model
    pub fn recommended_model_tier(&self) -> &str {
        match self {
            ComplexityLevel::Simple => "mini",
            ComplexityLevel::Medium => "standard",
            ComplexityLevel::Complex => "advanced",
            ComplexityLevel::VeryComplex => "premium",
        }
    }
}

/// Model bilgileri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID (ör: gpt-4-turbo, claude-3-opus)
    pub id: String,
    /// Görünen ad
    pub display_name: String,
    /// Model katmanı (mini, standard, advanced, premium)
    pub tier: String,
    /// Context window boyutu
    pub context_window: usize,
    /// Input token maliyeti (USD per 1M tokens)
    pub cost_input_per_m: f64,
    /// Output token maliyeti (USD per 1M tokens)
    pub cost_output_per_m: f64,
    /// Ortalama latency (ms)
    pub avg_latency_ms: u32,
    /// Desteklenen özellikler
    pub capabilities: Vec<String>,
    /// Maksimum zorluk seviyesi
    pub max_complexity: ComplexityLevel,
}

impl ModelInfo {
    /// Toplam maliyeti hesapla
    pub fn calculate_cost(&self, input_tokens: u64, output_tokens: u64) -> f64 {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * self.cost_input_per_m;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * self.cost_output_per_m;
        input_cost + output_cost
    }
    
    /// Basit modeller için sabit liste
    pub fn get_predefined_models() -> Vec<ModelInfo> {
        vec![
            // Mini tier - Basit görevler
            ModelInfo {
                id: "gpt-3.5-turbo".into(),
                display_name: "GPT-3.5 Turbo".into(),
                tier: "mini".into(),
                context_window: 16_385,
                cost_input_per_m: 0.50,
                cost_output_per_m: 1.50,
                avg_latency_ms: 300,
                capabilities: vec!["chat".into()],
                max_complexity: ComplexityLevel::Simple,
            },
            ModelInfo {
                id: "claude-3-haiku".into(),
                display_name: "Claude 3 Haiku".into(),
                tier: "mini".into(),
                context_window: 200_000,
                cost_input_per_m: 0.25,
                cost_output_per_m: 1.25,
                avg_latency_ms: 200,
                capabilities: vec!["chat".into(), "vision".into()],
                max_complexity: ComplexityLevel::Simple,
            },
            ModelInfo {
                id: "gemini-1.5-flash".into(),
                display_name: "Gemini 1.5 Flash".into(),
                tier: "mini".into(),
                context_window: 1_000_000,
                cost_input_per_m: 0.075,
                cost_output_per_m: 0.30,
                avg_latency_ms: 250,
                capabilities: vec!["chat".into(), "vision".into()],
                max_complexity: ComplexityLevel::Simple,
            },
            // Standard tier - Orta görevler
            ModelInfo {
                id: "gpt-4-turbo".into(),
                display_name: "GPT-4 Turbo".into(),
                tier: "standard".into(),
                context_window: 128_000,
                cost_input_per_m: 10.0,
                cost_output_per_m: 30.0,
                avg_latency_ms: 800,
                capabilities: vec!["chat".into(), "vision".into(), "function_calling".into()],
                max_complexity: ComplexityLevel::Medium,
            },
            ModelInfo {
                id: "claude-3-sonnet".into(),
                display_name: "Claude 3 Sonnet".into(),
                tier: "standard".into(),
                context_window: 200_000,
                cost_input_per_m: 3.0,
                cost_output_per_m: 15.0,
                avg_latency_ms: 600,
                capabilities: vec!["chat".into(), "vision".into(), "function_calling".into()],
                max_complexity: ComplexityLevel::Medium,
            },
            // Advanced tier - Karmaşık görevler
            ModelInfo {
                id: "gpt-4o".into(),
                display_name: "GPT-4o".into(),
                tier: "advanced".into(),
                context_window: 128_000,
                cost_input_per_m: 5.0,
                cost_output_per_m: 15.0,
                avg_latency_ms: 500,
                capabilities: vec!["chat".into(), "vision".into(), "function_calling".into()],
                max_complexity: ComplexityLevel::Complex,
            },
            ModelInfo {
                id: "claude-3-opus".into(),
                display_name: "Claude 3 Opus".into(),
                tier: "advanced".into(),
                context_window: 200_000,
                cost_input_per_m: 15.0,
                cost_output_per_m: 75.0,
                avg_latency_ms: 1500,
                capabilities: vec!["chat".into(), "vision".into(), "function_calling".into()],
                max_complexity: ComplexityLevel::Complex,
            },
            // Premium tier - Çok karmaşık görevler
            ModelInfo {
                id: "o1-preview".into(),
                display_name: "OpenAI o1 Preview".into(),
                tier: "premium".into(),
                context_window: 128_000,
                cost_input_per_m: 15.0,
                cost_output_per_m: 60.0,
                avg_latency_ms: 5000,
                capabilities: vec!["chat".into(), "reasoning".into()],
                max_complexity: ComplexityLevel::VeryComplex,
            },
            // Free models (OpenRouter)
            ModelInfo {
                id: "qwen/qwen3-1.7b:free".into(),
                display_name: "Qwen 3 1.7B (Free)".into(),
                tier: "mini".into(),
                context_window: 32_000,
                cost_input_per_m: 0.0,
                cost_output_per_m: 0.0,
                avg_latency_ms: 400,
                capabilities: vec!["chat".into()],
                max_complexity: ComplexityLevel::Simple,
            },
            ModelInfo {
                id: "qwen/qwen3-coder:free".into(),
                display_name: "Qwen 3 Coder (Free)".into(),
                tier: "standard".into(),
                context_window: 32_000,
                cost_input_per_m: 0.0,
                cost_output_per_m: 0.0,
                avg_latency_ms: 600,
                capabilities: vec!["chat".into(), "code".into()],
                max_complexity: ComplexityLevel::Medium,
            },
            // Ollama local models
            ModelInfo {
                id: "llama3.2:latest".into(),
                display_name: "Llama 3.2 (Local)".into(),
                tier: "standard".into(),
                context_window: 128_000,
                cost_input_per_m: 0.0,
                cost_output_per_m: 0.0,
                avg_latency_ms: 300,
                capabilities: vec!["chat".into()],
                max_complexity: ComplexityLevel::Medium,
            },
            ModelInfo {
                id: "qwen2.5-coder:7b".into(),
                display_name: "Qwen 2.5 Coder 7B (Local)".into(),
                tier: "standard".into(),
                context_window: 32_000,
                cost_input_per_m: 0.0,
                cost_output_per_m: 0.0,
                avg_latency_ms: 350,
                capabilities: vec!["chat".into(), "code".into()],
                max_complexity: ComplexityLevel::Medium,
            },
        ]
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  HUMAN-IN-THE-LOOP - Onay mekanizması
// ═════════════════════════════════════════════════════════════════════════════════

/// Model değişim modu
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RoutingMode {
    /// Tam otonom - sistem otomatik seçer
    FullyAutonomous,
    /// Kullanıcı onaylı - her değişimde sorulur
    RequireApproval,
    /// Kullanıcı onaylı + manuel seçim imkanı
    ApprovalWithManualOverride,
}

/// Model değişim onayı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelApprovalRequest {
    /// İstek ID
    pub id: String,
    /// Görev açıklaması
    pub task_description: String,
    /// Algılanan zorluk
    pub detected_complexity: ComplexityLevel,
    /// Önerilen model
    pub recommended_model: ModelInfo,
    /// Önerilen key
    pub recommended_key_id: String,
    /// Alternatif modeller
    pub alternatives: Vec<ModelInfo>,
    /// Tahmini maliyet
    pub estimated_cost: f64,
    /// Tahmini süre
    pub estimated_duration_secs: u32,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
}

/// Model değişim onayı sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelApprovalResponse {
    /// İstek ID
    pub request_id: String,
    /// Onaylandı mı?
    pub approved: bool,
    /// Seçilen model (farklı seçilebilir)
    pub selected_model: String,
    /// Seçilen key ID
    pub selected_key_id: String,
    /// Yanıt zamanı
    pub responded_at: DateTime<Utc>,
}

// ═════════════════════════════════════════════════════════════════════════════════
//  KEY RING - Anahtarlık yöneticisi
// ═════════════════════════════════════════════════════════════════════════════════

/// Key Ring - Anahtarlık
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRing {
    /// Tüm kayıtlı API anahtarları
    pub keys: Vec<ApiKeyEntry>,
    /// Tüm kayıtlı modeller
    pub models: Vec<ModelInfo>,
    /// Routing modu
    pub routing_mode: RoutingMode,
    /// Son kullanılan key ID
    pub last_used_key_id: Option<String>,
    /// Provider bazlı öncelikler
    pub provider_priorities: HashMap<String, u8>,
    /// Maliyet optimizasyonu aktif mi?
    pub cost_optimization: bool,
    /// Minimum kalite seviyesi
    pub min_quality_tier: String,
}

impl Default for KeyRing {
    fn default() -> Self {
        Self {
            keys: vec![],
            models: ModelInfo::get_predefined_models(),
            routing_mode: RoutingMode::ApprovalWithManualOverride,
            last_used_key_id: None,
            provider_priorities: HashMap::new(),
            cost_optimization: true,
            min_quality_tier: "mini".into(),
        }
    }
}

impl KeyRing {
    /// Yeni boş KeyRing oluştur
    pub fn new() -> Self {
        Self::default()
    }
    
    /// API key ekle
    pub fn add_key(&mut self, key: ApiKeyEntry) -> String {
        let id = key.id.clone();
        self.keys.push(key);
        id
    }
    
    /// API key sil
    pub fn remove_key(&mut self, id: &str) -> bool {
        let initial_len = self.keys.len();
        self.keys.retain(|k| k.id != id);
        self.keys.len() != initial_len
    }
    
    /// ID ile key bul
    pub fn get_key(&self, id: &str) -> Option<&ApiKeyEntry> {
        self.keys.iter().find(|k| k.id == id)
    }
    
    /// ID ile key bul (mutable)
    pub fn get_key_mut(&mut self, id: &str) -> Option<&mut ApiKeyEntry> {
        self.keys.iter_mut().find(|k| k.id == id)
    }
    
    /// Provider'a göre key'leri listele
    pub fn get_keys_by_provider(&self, provider: &str) -> Vec<&ApiKeyEntry> {
        self.keys.iter()
            .filter(|k| k.provider == provider && k.is_available())
            .collect()
    }
    
    /// Tüm aktif key'leri listele
    pub fn get_active_keys(&self) -> Vec<&ApiKeyEntry> {
        self.keys.iter()
            .filter(|k| k.is_available())
            .collect()
    }
    
    /// Model ID ile model bilgisi bul
    pub fn get_model(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.iter().find(|m| m.id == model_id)
    }
    
    /// Key'e atanmış modelleri al
    pub fn get_models_for_key(&self, key: &ApiKeyEntry) -> Vec<&ModelInfo> {
        if key.available_models.is_empty() {
            // Tüm modelleri provider'a göre filtrele
            self.models.iter()
                .filter(|m| self.model_matches_provider(&m.id, &key.provider))
                .collect()
        } else {
            // Sadece atanmış modeller
            self.models.iter()
                .filter(|m| key.available_models.iter().any(|km| km.id == m.id))
                .collect()
        }
    }
    
    /// Model ID provider'a uyuyor mu?
    fn model_matches_provider(&self, model_id: &str, provider: &str) -> bool {
        match provider.to_lowercase().as_str() {
            "openai" => model_id.starts_with("gpt") || model_id.starts_with("o1"),
            "anthropic" => model_id.starts_with("claude"),
            "google" => model_id.starts_with("gemini"),
            "ollama" => !model_id.contains("/"), // Ollama modellerinde / yok
            "openrouter" | "custom" => true, // Her model olabilir
            _ => true,
        }
    }
    
    /// Key sayısı
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }
    
    /// Aktif key sayısı
    pub fn active_key_count(&self) -> usize {
        self.keys.iter().filter(|k| k.is_available()).count()
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  KEY RING MANAGER - Thread-safe yönetici
// ═════════════════════════════════════════════════════════════════════════════════

/// Key Ring Manager
pub struct KeyRingManager {
    keyring: Arc<RwLock<KeyRing>>,
    pending_approvals: Arc<RwLock<HashMap<String, ModelApprovalRequest>>>,
}

impl KeyRingManager {
    /// Yeni manager oluştur
    pub fn new() -> Self {
        Self {
            keyring: Arc::new(RwLock::new(KeyRing::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Mevcut KeyRing ile oluştur
    pub fn with_keyring(keyring: KeyRing) -> Self {
        Self {
            keyring: Arc::new(RwLock::new(keyring)),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// KeyRing'i al
    pub async fn get_keyring(&self) -> KeyRing {
        self.keyring.read().await.clone()
    }
    
    /// KeyRing'i güncelle
    pub async fn update_keyring<F>(&self, f: F) 
    where
        F: FnOnce(&mut KeyRing)
    {
        let mut keyring = self.keyring.write().await;
        f(&mut keyring);
    }
    
    /// API key ekle
    pub async fn add_key(&self, key: ApiKeyEntry) -> String {
        let mut keyring = self.keyring.write().await;
        keyring.add_key(key)
    }
    
    /// API key sil
    pub async fn remove_key(&self, id: &str) -> bool {
        let mut keyring = self.keyring.write().await;
        keyring.remove_key(id)
    }
    
    /// Routing modunu ayarla
    pub async fn set_routing_mode(&self, mode: RoutingMode) {
        let mut keyring = self.keyring.write().await;
        keyring.routing_mode = mode;
    }
    
    /// Onay bekle
    pub async fn wait_for_approval(&self, request: ModelApprovalRequest) -> Option<ModelApprovalResponse> {
        let request_id = request.id.clone();
        
        // Bekleyen onaylara ekle
        self.pending_approvals.write().await.insert(request_id.clone(), request.clone());
        
        // Gerçek uygulamada burada bir channel veya event beklenecek
        // Şimdilik onay bekliyor durumunu döndürüyoruz
        None
    }
    
    /// Onay ver
    pub async fn submit_approval(&self, response: ModelApprovalResponse) -> bool {
        let mut pending = self.pending_approvals.write().await;
        pending.remove(&response.request_id).is_some()
    }
    
    /// Bekleyen onayları al
    pub async fn get_pending_approvals(&self) -> Vec<ModelApprovalRequest> {
        self.pending_approvals.read().await.values().cloned().collect()
    }
}

impl Default for KeyRingManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═════════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_key_entry_creation() {
        let entry = ApiKeyEntry::new("Test Key", "openai", "sk-test123");
        assert_eq!(entry.provider, "openai");
        assert!(entry.is_available());
    }
    
    #[test]
    fn test_key_ring_add_remove() {
        let mut keyring = KeyRing::new();
        
        let key = ApiKeyEntry::new("OpenAI Main", "openai", "sk-test");
        let id = keyring.add_key(key);
        
        assert_eq!(keyring.key_count(), 1);
        assert!(keyring.get_key(&id).is_some());
        
        keyring.remove_key(&id);
        assert_eq!(keyring.key_count(), 0);
    }
    
    #[test]
    fn test_model_cost_calculation() {
        let model = ModelInfo {
            id: "test-model".into(),
            display_name: "Test Model".into(),
            tier: "standard".into(),
            context_window: 4096,
            cost_input_per_m: 10.0,
            cost_output_per_m: 30.0,
            avg_latency_ms: 500,
            capabilities: vec![],
            max_complexity: ComplexityLevel::Medium,
        };
        
        let cost = model.calculate_cost(1_000_000, 500_000);
        assert!((cost - 25.0).abs() < 0.01);
    }
    
    #[test]
    fn test_complexity_level() {
        assert_eq!(ComplexityLevel::Simple.recommended_model_tier(), "mini");
        assert_eq!(ComplexityLevel::Complex.recommended_model_tier(), "advanced");
    }
    
    #[test]
    fn test_predefined_models() {
        let models = ModelInfo::get_predefined_models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id == "gpt-4-turbo"));
    }
}
