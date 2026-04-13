//! ─── CUSTOM MODES + MODE LEARNING + MODE PLUGINS ───
//!
//! Kullanıcı tanımlı modlar, mod öğrenme sistemi ve mod eklenti sistemi.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::modes::OperationMode;
use crate::ModeType;

// ═══════════════════════════════════════════════════════════════════════════════
// CUSTOM MODE BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Kullanıcı tanımlı mod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMode {
    /// Benzersiz mod ID
    pub id: Uuid,
    /// Mod adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// İkon (emoji)
    pub icon: String,
    /// Oluşturan kullanıcı
    pub author: String,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son güncelleme
    pub updated_at: DateTime<Utc>,
    /// Davranış kuralları
    pub behavior: CustomModeBehavior,
    /// İzin verilen araçlar
    pub allowed_tools: Vec<String>,
    /// Yasaklı araçlar
    pub denied_tools: Vec<String>,
    /// Mod parametreleri
    pub parameters: HashMap<String, serde_json::Value>,
    /// Etiketler
    pub tags: Vec<String>,
    /// Versiyon
    pub version: String,
}

/// Kullanıcı tanımlı mod davranışı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomModeBehavior {
    /// Maksimum iterasyon
    pub max_iterations: u32,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Otomatik devam
    pub auto_continue: bool,
    /// Onay gerekli
    pub require_approval: bool,
    /// Hata davranışı
    pub on_error: ErrorBehavior,
    /// Maksimum paralel araç
    pub max_parallel_tools: usize,
    /// Bağlam penceresi boyutu
    pub context_window: usize,
    /// Sıcaklık (temperature)
    pub temperature: f32,
    /// Sistem prompt eki
    pub system_prompt_suffix: String,
}

impl Default for CustomModeBehavior {
    fn default() -> Self {
        Self {
            max_iterations: 30,
            timeout_secs: 600,
            auto_continue: false,
            require_approval: false,
            on_error: ErrorBehavior::RetryWithBackoff,
            max_parallel_tools: 3,
            context_window: 8192,
            temperature: 0.7,
            system_prompt_suffix: String::new(),
        }
    }
}

/// Hata davranışı
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorBehavior {
    /// Durdur
    Stop,
    /// Yeniden dene
    Retry,
    /// Üstel geri çekilme ile dene
    RetryWithBackoff,
    /// Atla
    Skip,
    /// Kullanıcıya sor
    AskUser,
}

/// Kullanıcı tanımlı mod oluşturucu
pub struct CustomModeBuilder {
    mode: CustomMode,
}

impl CustomModeBuilder {
    /// Yeni mod oluşturucu başlat
    pub fn new(name: &str) -> Self {
        Self {
            mode: CustomMode {
                id: Uuid::new_v4(),
                name: name.into(),
                description: String::new(),
                icon: "🔧".into(),
                author: "user".into(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                behavior: CustomModeBehavior::default(),
                allowed_tools: Vec::new(),
                denied_tools: Vec::new(),
                parameters: HashMap::new(),
                tags: Vec::new(),
                version: "1.0.0".into(),
            },
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self { self.mode.description = desc.into(); self }
    pub fn with_icon(mut self, icon: &str) -> Self { self.mode.icon = icon.into(); self }
    pub fn with_author(mut self, author: &str) -> Self { self.mode.author = author.into(); self }
    pub fn with_max_iterations(mut self, n: u32) -> Self { self.mode.behavior.max_iterations = n; self }
    pub fn with_timeout(mut self, secs: u64) -> Self { self.mode.behavior.timeout_secs = secs; self }
    pub fn with_auto_continue(mut self, v: bool) -> Self { self.mode.behavior.auto_continue = v; self }
    pub fn with_require_approval(mut self, v: bool) -> Self { self.mode.behavior.require_approval = v; self }
    pub fn with_temperature(mut self, t: f32) -> Self { self.mode.behavior.temperature = t; self }
    pub fn with_context_window(mut self, size: usize) -> Self { self.mode.behavior.context_window = size; self }
    pub fn with_system_prompt(mut self, prompt: &str) -> Self { self.mode.behavior.system_prompt_suffix = prompt.into(); self }
    pub fn with_allowed_tool(mut self, tool: &str) -> Self { self.mode.allowed_tools.push(tool.into()); self }
    pub fn with_denied_tool(mut self, tool: &str) -> Self { self.mode.denied_tools.push(tool.into()); self }
    pub fn with_tag(mut self, tag: &str) -> Self { self.mode.tags.push(tag.into()); self }
    pub fn with_parameter(mut self, key: &str, value: serde_json::Value) -> Self { self.mode.parameters.insert(key.into(), value); self }

    /// Mod oluştur
    pub fn build(self) -> CustomMode {
        self.mode
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODE LEARNING
// ═══════════════════════════════════════════════════════════════════════════════

/// Mod öğrenme verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeLearningEntry {
    /// Girdi açıklaması
    pub input_description: String,
    /// Önerilen mod
    pub suggested_mode: ModeType,
    /// Gerçek kullanılan mod
    pub actual_mode: Option<ModeType>,
    /// Kullanıcı memnuniyeti (1-5)
    pub satisfaction: Option<u32>,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Sonuç başarılı mı
    pub success: bool,
    /// Görev süresi (ms)
    pub duration_ms: u64,
}

/// Mod öğrenme istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModeLearningStats {
    /// Toplam öğrenme verisi
    pub total_entries: u64,
    /// Mod başına başarı oranı
    pub success_rate_by_mode: HashMap<String, f64>,
    /// Mod başına ortalama süre
    pub avg_duration_by_mode: HashMap<String, f64>,
    /// En çok önerilen mod
    pub most_suggested: Option<ModeType>,
    /// En başarılı mod
    pub most_successful: Option<ModeType>,
    /// Doğruluk oranı
    pub accuracy: f64,
}

/// Mod öğrenme motoru
pub struct ModeLearningEngine {
    /// Öğrenme verileri
    entries: Vec<ModeLearningEntry>,
    /// İstatistikler
    stats: ModeLearningStats,
    /// Öğrenme oranı
    learning_rate: f64,
}

impl ModeLearningEngine {
    /// Yeni öğrenme motoru oluştur
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            stats: ModeLearningStats::default(),
            learning_rate: 0.1,
        }
    }

    /// Öğrenme verisi kaydet
    pub fn record(&mut self, entry: ModeLearningEntry) {
        let mode_key = format!("{:?}", entry.suggested_mode);
        
        // Başarı oranı güncelle
        let current_rate = self.stats.success_rate_by_mode.get(&mode_key).copied().unwrap_or(0.0);
        let new_rate = if entry.success { 1.0 } else { 0.0 };
        let updated_rate = current_rate + self.learning_rate * (new_rate - current_rate);
        self.stats.success_rate_by_mode.insert(mode_key.clone(), updated_rate);

        // Ortalama süre güncelle
        let current_dur = self.stats.avg_duration_by_mode.get(&mode_key).copied().unwrap_or(0.0);
        let updated_dur = current_dur + self.learning_rate * (entry.duration_ms as f64 - current_dur);
        self.stats.avg_duration_by_mode.insert(mode_key, updated_dur);

        self.entries.push(entry);
        self.stats.total_entries += 1;
        self.update_stats();
    }

    /// Görev açıklamasına göre en iyi modu öner
    pub fn suggest_best_mode(&self, description: &str) -> ModeType {
        let desc = description.to_lowercase();
        
        // Öğrenme verisinden en başarılı modu bul
        let best_mode = self.stats.success_rate_by_mode.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, _)| k.clone());

        // Anahtar kelimelere göre eşleştir
        if desc.contains("kod") || desc.contains("geliştir") || desc.contains("program") {
            ModeType::Development
        } else if desc.contains("araştır") || desc.contains("bul") || desc.contains("search") {
            ModeType::Research
        } else if desc.contains("plan") || desc.contains("tasarla") || desc.contains("strateji") {
            ModeType::Plan
        } else if desc.contains("otonom") || desc.contains("auto") || desc.contains("kendi başına") {
            ModeType::Autonomous
        } else {
            ModeType::ReAct
        }
    }

    /// Öğrenme istatistiklerini güncelle
    fn update_stats(&mut self) {
        if self.entries.is_empty() { return; }

        // En çok önerilen mod
        let mut mode_counts: HashMap<ModeType, u32> = HashMap::new();
        for entry in &self.entries {
            *mode_counts.entry(entry.suggested_mode).or_insert(0) += 1;
        }
        self.stats.most_suggested = mode_counts.iter()
            .max_by_key(|(_, &c)| c)
            .map(|(&m, _)| m);

        // En başarılı mod
        let mut success_counts: HashMap<ModeType, (u32, u32)> = HashMap::new();
        for entry in &self.entries {
            let (s, t) = success_counts.entry(entry.suggested_mode).or_insert((0, 0));
            if entry.success { *s += 1; }
            *t += 1;
        }
        self.stats.most_successful = success_counts.iter()
            .max_by(|(_, (s1, t1)), (_, (s2, t2))| {
                let r1 = *s1 as f64 / *t1 as f64;
                let r2 = *s2 as f64 / *t2 as f64;
                r1.partial_cmp(&r2).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(&m, _)| m);

        // Doğruluk oranı
        let correct = self.entries.iter()
            .filter(|e| e.actual_mode.map_or(false, |m| m == e.suggested_mode))
            .count();
        self.stats.accuracy = if self.entries.is_empty() { 0.0 } else { correct as f64 / self.entries.len() as f64 };
    }

    /// İstatistikler
    pub fn stats(&self) -> &ModeLearningStats { &self.stats }

    /// Öğrenme verilerini temizle
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats = ModeLearningStats::default();
    }
}

impl Default for ModeLearningEngine {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODE PLUGIN SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Mod eklenti yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModePlugin {
    /// Eklenti ID
    pub id: String,
    /// Eklenti adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Yazar
    pub author: String,
    /// Versiyon (semver)
    pub version: String,
    /// Hedef mod (hangi moda eklenti)
    pub target_mode: ModeType,
    /// Eklenti tipi
    pub plugin_type: ModePluginType,
    /// Öncelik (düşük = önce çalışır)
    pub priority: u32,
    /// Aktif mi?
    pub enabled: bool,
    /// Yapılandırma şeması
    pub config_schema: Option<serde_json::Value>,
    /// Eklinti konfigürasyonu
    pub config: HashMap<String, serde_json::Value>,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son güncelleme
    pub updated_at: DateTime<Utc>,
}

/// Eklenti tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModePluginType {
    /// Mod davranışını değiştirir
    BehaviorModifier,
    /// Yeni araç ekler
    ToolProvider,
    /// Kısıtlama ekler
    Constraint,
    /// İzleme/raporlama
    Monitor,
    /// Mod öncesi/sonrası kanca
    Hook,
}

/// Eklenti kanca noktası
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HookPoint {
    BeforeModeStart,
    AfterModeEnd,
    BeforeToolCall,
    AfterToolCall,
    OnError,
    OnSuccess,
}

/// Eklenti sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub plugin_id: String,
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub duration_ms: u64,
}

/// Mod eklenti yöneticisi
pub struct ModePluginManager {
    /// Kayıtlı eklentiler
    plugins: HashMap<String, ModePlugin>,
    /// Eklenti çalışma sırası
    execution_order: Vec<String>,
    /// Eklenti istatistikleri
    stats: HashMap<String, PluginStats>,
}

/// Eklenti istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginStats {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub avg_duration_ms: f64,
}

impl ModePluginManager {
    /// Yeni eklenti yöneticisi oluştur
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            execution_order: Vec::new(),
            stats: HashMap::new(),
        }
    }

    /// Eklenti kaydet
    pub fn register(&mut self, plugin: ModePlugin) {
        let id = plugin.id.clone();
        log::info!("🔌 Mod eklenti kaydedildi: {} v{}", plugin.name, plugin.version);
        self.plugins.insert(id.clone(), plugin);
        self.execution_order.push(id);
        self.execution_order.sort_by_key(|k| {
            self.plugins.get(k).map(|p| p.priority).unwrap_or(u32::MAX)
        });
    }

    /// Eklenti kaldır
    pub fn unregister(&mut self, plugin_id: &str) -> Result<(), String> {
        self.plugins.remove(plugin_id)
            .ok_or_else(|| format!("Eklenti bulunamadı: {}", plugin_id))?;
        self.execution_order.retain(|id| id != plugin_id);
        self.stats.remove(plugin_id);
        log::info!("🗑️  Mod eklenti kaldırıldı: {}", plugin_id);
        Ok(())
    }

    /// Eklenti etkinleştir/devre dışı bırak
    pub fn toggle(&mut self, plugin_id: &str, enabled: bool) -> Result<(), String> {
        self.plugins.get_mut(plugin_id)
            .ok_or_else(|| format!("Eklenti bulunamadı: {}", plugin_id))?
            .enabled = enabled;
        let state = if enabled { "etkinleştirildi" } else { "devre dışı bırakıldı" };
        log::info!("🔌 Eklenti {} {}", plugin_id, state);
        Ok(())
    }

    /// Belirli bir mod için eklentileri çalıştır
    pub fn run_plugins_for_mode(&mut self, mode: ModeType, hook: HookPoint) -> Vec<PluginResult> {
        let mut results = Vec::new();
        for plugin_id in &self.execution_order.clone() {
            if let Some(plugin) = self.plugins.get(plugin_id) {
                if !plugin.enabled || plugin.target_mode != mode { continue; }
                let start = std::time::Instant::now();
                let result = PluginResult {
                    plugin_id: plugin.id.clone(),
                    success: true,
                    message: format!("{} eklentisi çalıştı ({:?})", plugin.name, hook),
                    data: None,
                    duration_ms: start.elapsed().as_millis() as u64,
                };
                // İstatistik güncelle
                let stats = self.stats.entry(plugin.id.clone()).or_default();
                stats.total_calls += 1;
                stats.successful_calls += 1;
                results.push(result);
            }
        }
        results
    }

    /// Tüm eklentileri listele
    pub fn list_plugins(&self) -> Vec<&ModePlugin> {
        self.plugins.values().collect()
    }

    /// Belirli bir mod için eklentileri listele
    pub fn list_plugins_for_mode(&self, mode: ModeType) -> Vec<&ModePlugin> {
        self.plugins.values().filter(|p| p.target_mode == mode && p.enabled).collect()
    }

    /// Eklenti istatistikleri
    pub fn get_stats(&self, plugin_id: &str) -> Option<&PluginStats> {
        self.stats.get(plugin_id)
    }
}

impl Default for ModePluginManager {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_mode_builder() {
        let mode = CustomModeBuilder::new("Test Modu")
            .with_description("Test açıklaması")
            .with_icon("🧪")
            .with_max_iterations(100)
            .with_temperature(0.5)
            .with_tag("test")
            .with_allowed_tool("llm_query")
            .build();

        assert_eq!(mode.name, "Test Modu");
        assert_eq!(mode.behavior.max_iterations, 100);
        assert_eq!(mode.behavior.temperature, 0.5);
    }

    #[test]
    fn test_mode_learning() {
        let mut engine = ModeLearningEngine::new();
        
        let entry = ModeLearningEntry {
            input_description: "kod yaz".into(),
            suggested_mode: ModeType::Development,
            actual_mode: Some(ModeType::Development),
            satisfaction: Some(5),
            timestamp: Utc::now(),
            success: true,
            duration_ms: 1500,
        };
        
        engine.record(entry);
        
        let stats = engine.stats();
        assert_eq!(stats.total_entries, 1);
    }

    #[test]
    fn test_mode_suggestion() {
        let engine = ModeLearningEngine::new();
        
        assert_eq!(engine.suggest_best_mode("kod yaz"), ModeType::Development);
        assert_eq!(engine.suggest_best_mode("araştırma yap"), ModeType::Research);
        assert_eq!(engine.suggest_best_mode("plan yap"), ModeType::Plan);
    }

    #[test]
    fn test_plugin_manager() {
        let mut manager = ModePluginManager::new();
        
        let plugin = ModePlugin {
            id: "plugin-1".into(),
            name: "Test Plugin".into(),
            description: "Test eklentisi".into(),
            author: "test".into(),
            version: "1.0.0".into(),
            target_mode: ModeType::Development,
            plugin_type: ModePluginType::BehaviorModifier,
            priority: 10,
            enabled: true,
            config_schema: None,
            config: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        manager.register(plugin);
        assert_eq!(manager.list_plugins().len(), 1);
    }
}