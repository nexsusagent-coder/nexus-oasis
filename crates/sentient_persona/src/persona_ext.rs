//! ─── PERSONA MARKETPLACE + DYNAMIC ADAPTATION + MULTI-LANGUAGE + ANALYTICS ───
//!
//! Persona pazaryeri, dinamik uyarlama, çok dil desteği ve persona analitiği.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::persona::Persona;

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA MARKETPLACE
// ═══════════════════════════════════════════════════════════════════════════════

/// Pazaryeri kategori
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketplaceCategory {
    General,
    Coding,
    Research,
    Creative,
    Education,
    Business,
    Healthcare,
    Legal,
    Technical,
    Entertainment,
}

/// Pazaryeri persona girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceListing {
    pub id: String,
    pub persona: Persona,
    pub category: MarketplaceCategory,
    pub rating: f32,
    pub downloads: u64,
    pub reviews: Vec<MarketplaceReview>,
    pub featured: bool,
    pub verified: bool,
    pub price: Option<String>, // Ücretsiz veya fiyat
    pub license: String,
    pub tags: Vec<String>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Pazaryeri değerlendirmesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceReview {
    pub id: String,
    pub user_id: String,
    pub rating: u8, // 1-5
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

/// Persona pazaryeri
pub struct PersonaMarketplace {
    listings: HashMap<String, MarketplaceListing>,
    installed: HashMap<String, Persona>,
}

impl PersonaMarketplace {
    pub fn new() -> Self {
        Self {
            listings: HashMap::new(),
            installed: HashMap::new(),
        }
    }

    /// Persona yayınla
    pub fn publish(&mut self, persona: Persona, category: MarketplaceCategory) -> String {
        let id = format!("persona-{}", Uuid::new_v4().as_simple());
        let listing = MarketplaceListing {
            id: id.clone(),
            persona,
            category,
            rating: 0.0,
            downloads: 0,
            reviews: Vec::new(),
            featured: false,
            verified: false,
            price: None,
            license: "MIT".into(),
            tags: Vec::new(),
            published_at: Utc::now(),
            updated_at: Utc::now(),
        };
        log::info!("🏪 Persona pazaryerine yayınlandı: {}", listing.id);
        self.listings.insert(id.clone(), listing);
        id
    }

    /// Persona ara
    pub fn search(&self, query: &str, category: Option<MarketplaceCategory>) -> Vec<&MarketplaceListing> {
        let q = query.to_lowercase();
        self.listings.values()
            .filter(|l| {
                if let Some(cat) = category {
                    if l.category != cat { return false; }
                }
                l.persona.name.to_lowercase().contains(&q) ||
                l.persona.description.to_lowercase().contains(&q) ||
                l.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    /// En popüler personeleri listele
    pub fn trending(&self, limit: usize) -> Vec<&MarketplaceListing> {
        let mut list: Vec<_> = self.listings.values().collect();
        list.sort_by(|a, b| b.downloads.cmp(&a.downloads));
        list.into_iter().take(limit).collect()
    }

    /// En yüksek puanlı personeleri listele
    pub fn top_rated(&self, limit: usize) -> Vec<&MarketplaceListing> {
        let mut list: Vec<_> = self.listings.values().collect();
        list.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        list.into_iter().take(limit).collect()
    }

    /// Persona kur
    pub fn install(&mut self, listing_id: &str) -> Result<Persona, String> {
        let listing = self.listings.get_mut(listing_id)
            .ok_or_else(|| format!("Persona bulunamadı: {}", listing_id))?;
        listing.downloads += 1;
        let persona = listing.persona.clone();
        self.installed.insert(listing_id.into(), persona.clone());
        log::info!("📥 Persona kuruldu: {}", listing_id);
        Ok(persona)
    }

    /// Persona kaldır
    pub fn uninstall(&mut self, listing_id: &str) -> Result<(), String> {
        self.installed.remove(listing_id)
            .ok_or_else(|| format!("Kurulu persona bulunamadı: {}", listing_id))?;
        Ok(())
    }

    /// Değerlendirme ekle
    pub fn add_review(&mut self, listing_id: &str, review: MarketplaceReview) -> Result<(), String> {
        let listing = self.listings.get_mut(listing_id)
            .ok_or_else(|| format!("Persona bulunamadı: {}", listing_id))?;
        listing.reviews.push(review);
        // Ortalama puanı güncelle
        let total: u32 = listing.reviews.iter().map(|r| r.rating as u32).sum();
        listing.rating = total as f32 / listing.reviews.len() as f32;
        Ok(())
    }

    /// Kurulu personeleri listele
    pub fn list_installed(&self) -> Vec<&Persona> {
        self.installed.values().collect()
    }

    /// Kategori bazlı listele
    pub fn by_category(&self, category: MarketplaceCategory) -> Vec<&MarketplaceListing> {
        self.listings.values().filter(|l| l.category == category).collect()
    }
}

impl Default for PersonaMarketplace {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DYNAMIC ADAPTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Adaptasyon sinyali
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationSignal {
    /// Kullanıcı sık sık hata yapıyor → daha basit dil
    UserErrorsIncreased { count: u32 },
    /// Kullanıcı hızlı çalışıyor → daha az açıklama
    UserSpeedIncreased { avg_response_time_ms: u64 },
    /// Kullanıcı detay istiyor → daha açıklamalı
    UserRequestsDetail { topic: String },
    /// Konu değişimi → persona tonu ayarla
    TopicChanged { from: String, to: String },
    /// Duygusal durum → empati artır
    EmotionalState { sentiment: f32, confidence: f32 },
    /// Uzun görev → motivasyon artır
    LongTask { duration_secs: u64 },
}

/// Adaptasyon parametreleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationParams {
    /// Açıklama seviyesi (0.0 = kısa, 1.0 = detaylı)
    pub verbosity: f32,
    /// Resmiyet seviyesi (0.0 = samimi, 1.0 = resmi)
    pub formality: f32,
    /// Teknik seviye (0.0 = basit, 1.0 = uzman)
    pub technical_level: f32,
    /// Empati seviyesi (0.0 = nötr, 1.0 = yüksek empati)
    pub empathy: f32,
    /// Motivasyon seviyesi (0.0 = düşük, 1.0 = yüksek)
    pub motivation: f32,
    /// Hız ayarı (0.0 = yavaş, 1.0 = hızlı)
    pub pace: f32,
}

impl Default for AdaptationParams {
    fn default() -> Self {
        Self {
            verbosity: 0.5,
            formality: 0.5,
            technical_level: 0.5,
            empathy: 0.5,
            motivation: 0.5,
            pace: 0.5,
        }
    }
}

/// Dinamik uyarlama motoru
pub struct DynamicAdaptationEngine {
    /// Mevcut adaptasyon parametreleri
    params: AdaptationParams,
    /// Adaptasyon geçmişi
    history: Vec<AdaptationRecord>,
    /// Öğrenme oranı
    learning_rate: f32,
}

/// Adaptasyon kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRecord {
    pub signal: AdaptationSignal,
    pub params_before: AdaptationParams,
    pub params_after: AdaptationParams,
    pub timestamp: DateTime<Utc>,
}

impl DynamicAdaptationEngine {
    pub fn new() -> Self {
        Self {
            params: AdaptationParams::default(),
            history: Vec::new(),
            learning_rate: 0.1,
        }
    }

    /// Adaptasyon sinyali işle
    pub fn adapt(&mut self, signal: AdaptationSignal) -> AdaptationParams {
        let before = self.params.clone();

        match signal {
            AdaptationSignal::UserErrorsIncreased { count } => {
                // Daha basit ve açıklamalı ol
                self.params.technical_level = (self.params.technical_level - 0.05 * count as f32).max(0.1);
                self.params.verbosity = (self.params.verbosity + 0.1).min(0.9);
                self.params.empathy = (self.params.empathy + 0.1).min(0.9);
            }
            AdaptationSignal::UserSpeedIncreased { .. } => {
                // Daha kısa ve hızlı
                self.params.verbosity = (self.params.verbosity - 0.1).max(0.1);
                self.params.pace = (self.params.pace + 0.1).min(0.9);
            }
            AdaptationSignal::UserRequestsDetail { .. } => {
                self.params.verbosity = (self.params.verbosity + 0.15).min(0.95);
                self.params.technical_level = (self.params.technical_level + 0.1).min(0.9);
            }
            AdaptationSignal::TopicChanged { .. } => {
                // Konu değişimi - formaliyeti ayarla
                self.params.formality = 0.5; // Orta seviyeye dön
            }
            AdaptationSignal::EmotionalState { sentiment, .. } => {
                if sentiment < 0.0 {
                    self.params.empathy = (self.params.empathy + 0.2).min(0.95);
                    self.params.motivation = (self.params.motivation + 0.1).min(0.9);
                }
            }
            AdaptationSignal::LongTask { .. } => {
                self.params.motivation = (self.params.motivation + 0.15).min(0.9);
                self.params.empathy = (self.params.empathy + 0.1).min(0.85);
            }
        }

        // Adaptasyon kaydı
        self.history.push(AdaptationRecord {
            signal,
            params_before: before,
            params_after: self.params.clone(),
            timestamp: Utc::now(),
        });

        self.params.clone()
    }

    /// Mevcut parametreleri al
    pub fn params(&self) -> &AdaptationParams { &self.params }

    /// Parametreleri sıfırla
    pub fn reset(&mut self) { self.params = AdaptationParams::default(); }

    /// Adaptasyon geçmişi
    pub fn history(&self) -> &[AdaptationRecord] { &self.history }
}

impl Default for DynamicAdaptationEngine {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTI-LANGUAGE SUPPORT
// ═══════════════════════════════════════════════════════════════════════════════

/// Dil kodu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LanguageCode {
    Tr, En, De, Fr, Es, It, Pt, Ru, Zh, Ja, Ko, Ar, Hi,
}

impl LanguageCode {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Tr => "Türkçe", Self::En => "English", Self::De => "Deutsch",
            Self::Fr => "Français", Self::Es => "Español", Self::It => "Italiano",
            Self::Pt => "Português", Self::Ru => "Русский", Self::Zh => "中文",
            Self::Ja => "日本語", Self::Ko => "한국어", Self::Ar => "العربية",
            Self::Hi => "हिन्दी",
        }
    }
}

/// Çok dili persona çevirisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaTranslation {
    pub language: LanguageCode,
    pub name: String,
    pub description: String,
    pub role: String,
    pub background: String,
    pub greeting: String,
    pub style_notes: Vec<String>,
}

/// Çok dili destek yöneticisi
pub struct MultiLanguageSupport {
    /// Varsayılan dil
    default_language: LanguageCode,
    /// Aktif dil
    active_language: LanguageCode,
    /// Çeviriler (persona_id -> dil -> çeviri)
    translations: HashMap<Uuid, HashMap<LanguageCode, PersonaTranslation>>,
    /// Dil algılama eşikleri
    detection_threshold: f32,
}

impl MultiLanguageSupport {
    pub fn new(default: LanguageCode) -> Self {
        Self {
            default_language: default,
            active_language: default,
            translations: HashMap::new(),
            detection_threshold: 0.7,
        }
    }

    /// Dil ekle
    pub fn add_translation(&mut self, persona_id: Uuid, translation: PersonaTranslation) {
        self.translations
            .entry(persona_id)
            .or_insert_with(HashMap::new)
            .insert(translation.language, translation);
    }

    /// Aktif dili değiştir
    pub fn set_language(&mut self, lang: LanguageCode) {
        self.active_language = lang;
    }

    /// Persona'nın belirli bir dildeki çevirisini al
    pub fn get_translation(&self, persona_id: Uuid, lang: LanguageCode) -> Option<&PersonaTranslation> {
        self.translations.get(&persona_id).and_then(|t| t.get(&lang))
    }

    /// Metin dilini algıla (basit yöntem)
    pub fn detect_language(&self, text: &str) -> LanguageCode {
        let text_lower = text.to_lowercase();
        // Türkçe karakterler
        let turkish_chars = ['ç', 'ğ', 'ı', 'ö', 'ş', 'ü'];
        let turkish_count = text_lower.chars().filter(|c| turkish_chars.contains(c)).count();
        if turkish_count as f32 / text_lower.len() as f32 > self.detection_threshold * 0.1 {
            return LanguageCode::Tr;
        }
        // Varsayılan İngilizce
        self.default_language
    }

    /// Desteklenen dilleri listele
    pub fn supported_languages(&self) -> Vec<LanguageCode> {
        vec![LanguageCode::Tr, LanguageCode::En, LanguageCode::De, LanguageCode::Fr,
             LanguageCode::Es, LanguageCode::It, LanguageCode::Pt, LanguageCode::Ru,
             LanguageCode::Zh, LanguageCode::Ja, LanguageCode::Ko, LanguageCode::Ar,
             LanguageCode::Hi]
    }

    /// Aktif dil
    pub fn active_language(&self) -> LanguageCode { self.active_language }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona analitik olayı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: Uuid,
    pub persona_id: Uuid,
    pub event_type: AnalyticsEventType,
    pub timestamp: DateTime<Utc>,
    pub duration_ms: u64,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Analitik olay tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalyticsEventType {
    PersonaActivated,
    PersonaDeactivated,
    PersonaSwitched,
    ConversationStarted,
    ConversationEnded,
    UserFeedback,
    ErrorOccurred,
    ToolCallMade,
    AdaptationTriggered,
}

/// Persona analitik özeti
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonaAnalyticsSummary {
    pub total_activations: u64,
    pub total_conversations: u64,
    pub total_errors: u64,
    pub avg_conversation_duration_ms: f64,
    pub avg_user_rating: f32,
    pub most_used_persona: Option<Uuid>,
    pub adaptation_count: u64,
    pub tool_call_count: u64,
}

/// Persona analitiği motoru
pub struct PersonaAnalytics {
    events: Vec<AnalyticsEvent>,
    summary: PersonaAnalyticsSummary,
}

impl PersonaAnalytics {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            summary: PersonaAnalyticsSummary::default(),
        }
    }

    /// Olay kaydet
    pub fn record(&mut self, event: AnalyticsEvent) {
        match event.event_type {
            AnalyticsEventType::PersonaActivated => self.summary.total_activations += 1,
            AnalyticsEventType::ConversationStarted => self.summary.total_conversations += 1,
            AnalyticsEventType::ErrorOccurred => self.summary.total_errors += 1,
            AnalyticsEventType::ToolCallMade => self.summary.tool_call_count += 1,
            AnalyticsEventType::AdaptationTriggered => self.summary.adaptation_count += 1,
            _ => {}
        }
        self.events.push(event);
    }

    /// Özet al
    pub fn summary(&self) -> &PersonaAnalyticsSummary { &self.summary }

    /// Belirli bir persona'nın olaylarını filtrele
    pub fn events_for_persona(&self, persona_id: Uuid) -> Vec<&AnalyticsEvent> {
        self.events.iter().filter(|e| e.persona_id == persona_id).collect()
    }

    /// Zaman aralığına göre olayları filtrele
    pub fn events_in_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&AnalyticsEvent> {
        self.events.iter().filter(|e| e.timestamp >= start && e.timestamp <= end).collect()
    }

    /// Olayları temizle
    pub fn clear(&mut self) {
        self.events.clear();
        self.summary = PersonaAnalyticsSummary::default();
    }

    /// Toplam olay sayısı
    pub fn total_events(&self) -> usize { self.events.len() }
}

impl Default for PersonaAnalytics {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace() {
        let mut marketplace = PersonaMarketplace::new();
        let persona = Persona::default();
        let id = marketplace.publish(persona, MarketplaceCategory::Coding);
        
        // Default persona name is "SENTIENT" — search for it
        let results = marketplace.search("SENTIENT", None);
        assert!(!results.is_empty());
        
        let trending = marketplace.trending(10);
        assert!(!trending.is_empty());
    }

    #[test]
    fn test_dynamic_adaptation() {
        let mut engine = DynamicAdaptationEngine::new();
        
        let params = engine.adapt(AdaptationSignal::UserErrorsIncreased { count: 3 });
        assert!(params.technical_level < 0.5); // Daha basit
        assert!(params.verbosity > 0.5); // Daha açıklamalı
        
        let params2 = engine.adapt(AdaptationSignal::UserSpeedIncreased { avg_response_time_ms: 200 });
        assert!(params2.pace > 0.5); // Daha hızlı
    }

    #[test]
    fn test_multi_language() {
        let mut ml = MultiLanguageSupport::new(LanguageCode::En);
        ml.set_language(LanguageCode::Tr);
        assert_eq!(ml.active_language(), LanguageCode::Tr);
        
        // Türkçe algılama
        let detected = ml.detect_language("Merhaba, nasılsın? Bugün hava çok güzel.");
        assert_eq!(detected, LanguageCode::Tr);
    }

    #[test]
    fn test_analytics() {
        let mut analytics = PersonaAnalytics::new();
        
        let event = AnalyticsEvent {
            id: Uuid::new_v4(),
            persona_id: Uuid::new_v4(),
            event_type: AnalyticsEventType::PersonaActivated,
            timestamp: Utc::now(),
            duration_ms: 1000,
            metadata: HashMap::new(),
        };
        
        analytics.record(event);
        assert_eq!(analytics.summary().total_activations, 1);
    }
}