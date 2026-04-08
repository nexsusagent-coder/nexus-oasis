//! ─── BELLEK TİPLERİ ───
//!
//! SENTIENT'nın bilişsel bellek sistemdeki tipler ve yapılar.

use sentient_common::error::SENTIENTError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK TİPLERİ
// ─────────────────────────────────────────────────────────────────────────────

/// Ana bellek tipleri (insan hafızası modeli)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    /// Episodik Bellek: Kişisel deneyimler, olaylar
    /// "Dün marketten elma aldım"
    Episodic,
    
    /// Semantik Bellek: Genel bilgiler, gerçekler
    /// "Elma bir meyvedir"
    Semantic,
    
    /// Prosedürel Bellek: Beceriler, yöntemler
    /// "Elma nasıl yenir"
    Procedural,
    
    /// Kısa süreli çalışma belleği
    Working,
    
    /// Duygusal bellek (duygusal bağlam)
    Emotional,
    
    /// Meta-bellek (bellek hakkında bellek)
    Meta,
}

impl Default for MemoryType {
    fn default() -> Self {
        Self::Semantic
    }
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Episodic => write!(f, "Episodik"),
            Self::Semantic => write!(f, "Semantik"),
            Self::Procedural => write!(f, "Prosedürel"),
            Self::Working => write!(f, "Çalışma"),
            Self::Emotional => write!(f, "Duygusal"),
            Self::Meta => write!(f, "Meta"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ÖNEM SEVİYESİ
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek önemi (0.0 - 1.0)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Importance(f32);

impl Importance {
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
    
    pub fn low() -> Self { Self(0.2) }
    pub fn medium() -> Self { Self(0.5) }
    pub fn high() -> Self { Self(0.8) }
    pub fn critical() -> Self { Self(1.0) }
    
    pub fn value(&self) -> f32 { self.0 }
    
    /// Decay sonrası yeni önem
    pub fn decay(&self, rate: f32) -> Self {
        Self::new(self.0 * (1.0 - rate))
    }
}

impl Default for Importance {
    fn default() -> Self {
        Self::medium()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK GİRDİSİ
// ─────────────────────────────────────────────────────────────────────────────

/// Belleğe kayıt için girdi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInput {
    /// İçerik
    pub content: String,
    /// Bellek tipi
    pub memory_type: MemoryType,
    /// Kaynak (nereden geldi)
    pub source: MemorySource,
    /// Meta veriler
    pub metadata: HashMap<String, serde_json::Value>,
    /// Önemi
    pub importance: Importance,
    /// Etiketler
    pub tags: Vec<String>,
    /// TTL (saniye) - None = kalıcı
    pub ttl_seconds: Option<i64>,
}

impl MemoryInput {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            memory_type: MemoryType::Semantic,
            source: MemorySource::Internal,
            metadata: HashMap::new(),
            importance: Importance::medium(),
            tags: Vec::new(),
            ttl_seconds: None,
        }
    }
    
    pub fn with_type(mut self, memory_type: MemoryType) -> Self {
        self.memory_type = memory_type;
        self
    }
    
    pub fn with_source(mut self, source: MemorySource) -> Self {
        self.source = source;
        self
    }
    
    pub fn with_importance(mut self, importance: Importance) -> Self {
        self.importance = importance;
        self
    }
    
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    pub fn with_ttl(mut self, seconds: i64) -> Self {
        self.ttl_seconds = Some(seconds);
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK KAYNAĞI
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek kaynağı
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySource {
    /// Kullanıcı girdisi
    UserInput,
    /// LLM çıktısı
    LLMOutput,
    /// Web araştırması
    WebResearch { url: Option<String> },
    /// Kod çalıştırma
    CodeExecution,
    /// İçsel çıkarım
    InternalInference,
    /// Konsolidasyon sonucu
    Consolidation,
    /// Sistem içi
    Internal,
    /// Dış import
    External { source: String },
}

impl Default for MemorySource {
    fn default() -> Self {
        Self::Internal
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK KAYDI (STORED)
// ─────────────────────────────────────────────────────────────────────────────

/// Saklanmış bellek kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Benzersiz ID
    pub id: Uuid,
    /// İçerik
    pub content: String,
    /// Bellek tipi
    pub memory_type: MemoryType,
    /// Kaynak
    pub source: MemorySource,
    /// Vektör embedding (opsiyonel)
    pub embedding: Option<Vec<f32>>,
    /// Meta veriler
    pub metadata: HashMap<String, serde_json::Value>,
    /// Önemi (dinamik)
    pub importance: Importance,
    /// Erişim sayısı
    pub access_count: u32,
    /// Etiketler
    pub tags: Vec<String>,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son erişim
    pub last_accessed: DateTime<Utc>,
    /// Son güncelleme
    pub updated_at: DateTime<Utc>,
    /// TTL
    pub ttl_seconds: Option<i64>,
    /// Son doğrulama
    pub last_validated: Option<DateTime<Utc>>,
    /// Doğruluk skoru
    pub confidence: f32,
}

impl MemoryEntry {
    /// Yeni bellek kaydı oluştur
    pub fn from_input(input: MemoryInput) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content: input.content,
            memory_type: input.memory_type,
            source: input.source,
            embedding: None,
            metadata: input.metadata,
            importance: input.importance,
            access_count: 0,
            tags: input.tags,
            created_at: now,
            last_accessed: now,
            updated_at: now,
            ttl_seconds: input.ttl_seconds,
            last_validated: None,
            confidence: 1.0,
        }
    }
    
    /// Erişim sayısını artır
    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Utc::now();
    }
    
    /// Önemi artır (başarılı kullanım)
    pub fn reinforce(&mut self, delta: f32) {
        let new_importance = (self.importance.value() + delta).min(1.0);
        self.importance = Importance::new(new_importance);
    }
    
    /// TTL süresi doldu mu?
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_seconds {
            let expires_at = self.created_at + chrono::Duration::seconds(ttl);
            return Utc::now() > expires_at;
        }
        false
    }
    
    /// Embed boyutu
    pub fn embedding_dim(&self) -> Option<usize> {
        self.embedding.as_ref().map(|e| e.len())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ARAMA SONUCU
// ─────────────────────────────────────────────────────────────────────────────

/// Arama sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Bulunan bellek
    pub memory: MemoryEntry,
    /// Benzerlik skoru (0.0 - 1.0)
    pub similarity: f32,
    /// Arama tipi
    pub search_type: SearchType,
}

/// Arama tipi
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    /// Vektör benzerliği
    VectorSimilarity,
    /// Anahtar kelime
    KeywordMatch,
    /// Etiket eşleşmesi
    TagMatch,
    /// Hibrit
    Hybrid,
    /// Zaman tabanlı
    Temporal,
}

// ─────────────────────────────────────────────────────────────────────────────
// RAG CONTEXT
// ─────────────────────────────────────────────────────────────────────────────

/// RAG için hazırlanan context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagContext {
    /// Sorgu
    pub query: String,
    /// Getirilen bellekler
    pub retrieved_memories: Vec<SearchResult>,
    /// Toplam token tahmini
    pub estimated_tokens: usize,
    /// Context metni
    pub context_text: String,
    /// Kaynak tipi
    pub source_types: Vec<MemoryType>,
}

impl RagContext {
    pub fn is_empty(&self) -> bool {
        self.retrieved_memories.is_empty()
    }
    
    pub fn memory_count(&self) -> usize {
        self.retrieved_memories.len()
    }
    
    /// LLM için formatlanmış context
    pub fn format_for_llm(&self) -> String {
        if self.retrieved_memories.is_empty() {
            return String::new();
        }
        
        let mut formatted = String::from("📚 İLGİLİ BELLEKLER:\n\n");
        
        for (i, result) in self.retrieved_memories.iter().enumerate() {
            formatted.push_str(&format!(
                "[{}] [{}] (önem: {:.1}, benzerlik: {:.2})\n{}\n\n",
                i + 1,
                result.memory.memory_type,
                result.memory.importance.value(),
                result.similarity,
                result.memory.content
            ));
        }
        
        formatted
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// İLİŞKİ TİPLERİ
// ─────────────────────────────────────────────────────────────────────────────

/// Bellekler arası ilişki tipleri
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    /// A ait B'ye
    PartOf,
    /// A neden B
    Causes,
    /// A zıttı B
    OppositeOf,
    /// A örneği B
    ExampleOf,
    /// A benzer B'ye
    SimilarTo,
    /// A gelene B (zaman)
    Precedes,
    /// A türetilmiş B'den
    DerivedFrom,
    /// A gerektirir B'yi
    Requires,
    /// A ilgili B ile
    RelatedTo,
    /// A referans B'ye
    References,
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartOf => write!(f, "parçası"),
            Self::Causes => write!(f, "neden"),
            Self::OppositeOf => write!(f, "zıttı"),
            Self::ExampleOf => write!(f, "örneği"),
            Self::SimilarTo => write!(f, "benzer"),
            Self::Precedes => write!(f, "öncül"),
            Self::DerivedFrom => write!(f, "türevi"),
            Self::Requires => write!(f, "gerektirir"),
            Self::RelatedTo => write!(f, "ilişkili"),
            Self::References => write!(f, "referans"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK İSTATİSTİKLERİ
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_memories: u64,
    pub by_type: HashMap<MemoryType, u64>,
    pub total_embeddings: u64,
    pub total_relations: u64,
    pub avg_importance: f32,
    pub avg_access_count: f32,
    pub oldest_memory: Option<DateTime<Utc>>,
    pub newest_memory: Option<DateTime<Utc>>,
    pub expired_count: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK SORGU SEÇENEKLERİ
// ─────────────────────────────────────────────────────────────────────────────

/// Arama seçenekleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Maksimum sonuç
    pub limit: usize,
    /// Minimum benzerlik eşiği
    pub min_similarity: f32,
    /// Bellek tipi filtresi
    pub memory_types: Option<Vec<MemoryType>>,
    /// Etiket filtresi
    pub tags: Option<Vec<String>>,
    /// Zaman aralığı
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Minimum önem
    pub min_importance: Option<f32>,
    /// Arama tipi
    pub search_type: SearchType,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            limit: 10,
            min_similarity: 0.5,
            memory_types: None,
            tags: None,
            time_range: None,
            min_importance: None,
            search_type: SearchType::Hybrid,
        }
    }
}

impl SearchOptions {
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    pub fn with_min_similarity(mut self, threshold: f32) -> Self {
        self.min_similarity = threshold;
        self
    }
    
    pub fn with_types(mut self, types: Vec<MemoryType>) -> Self {
        self.memory_types = Some(types);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BELLEK KONSOLİDASYON SONUCU
// ─────────────────────────────────────────────────────────────────────────────

/// Konsolidasyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    /// İşlenen bellek sayısı
    pub processed_count: usize,
    /// Oluşturulan yeni anı sayısı
    pub created_memories: usize,
    /// Güçlendirilen bellek sayısı
    pub reinforced_count: usize,
    /// Silinen bellek sayısı
    pub pruned_count: usize,
    /// Yeni ilişkiler
    pub new_relations: usize,
    /// Süre (ms)
    pub duration_ms: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// HATA TİPLERİ
// ─────────────────────────────────────────────────────────────────────────────

pub type MemoryResult<T> = Result<T, MemoryError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryError {
    DatabaseError(String),
    EmbeddingError(String),
    NotFound(Uuid),
    InvalidInput(String),
    StorageFull,
    Timeout,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(s) => write!(f, "Veritabanı hatası: {}", s),
            Self::EmbeddingError(s) => write!(f, "Embedding hatası: {}", s),
            Self::NotFound(id) => write!(f, "Bellek bulunamadı: {}", id),
            Self::InvalidInput(s) => write!(f, "Geçersiz girdi: {}", s),
            Self::StorageFull => write!(f, "Bellek deposu dolu"),
            Self::Timeout => write!(f, "İşlem zaman aşımına uğradı"),
        }
    }
}

impl std::error::Error for MemoryError {}

impl From<MemoryError> for SENTIENTError {
    fn from(e: MemoryError) -> Self {
        SENTIENTError::Memory(e.to_string())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_type_display() {
        assert_eq!(MemoryType::Episodic.to_string(), "Episodik");
        assert_eq!(MemoryType::Semantic.to_string(), "Semantik");
    }
    
    #[test]
    fn test_importance() {
        let imp = Importance::new(0.5);
        assert_eq!(imp.value(), 0.5);
        
        let decayed = imp.decay(0.2);
        assert!((decayed.value() - 0.4).abs() < 0.01);
    }
    
    #[test]
    fn test_memory_input_builder() {
        let input = MemoryInput::new("Test içerik")
            .with_type(MemoryType::Episodic)
            .with_importance(Importance::high())
            .with_tag("test")
            .with_ttl(3600);
        
        assert_eq!(input.content, "Test içerik");
        assert_eq!(input.memory_type, MemoryType::Episodic);
        assert_eq!(input.tags, vec!["test"]);
        assert_eq!(input.ttl_seconds, Some(3600));
    }
    
    #[test]
    fn test_memory_entry() {
        let input = MemoryInput::new("Test")
            .with_type(MemoryType::Semantic);
        let mut entry = MemoryEntry::from_input(input);
        
        assert_eq!(entry.access_count, 0);
        entry.access();
        assert_eq!(entry.access_count, 1);
        
        entry.reinforce(0.1);
        assert!((entry.importance.value() - 0.6).abs() < 0.01);
    }
    
    #[test]
    fn test_rag_context() {
        let context = RagContext {
            query: "Test".into(),
            retrieved_memories: vec![],
            estimated_tokens: 0,
            context_text: String::new(),
            source_types: vec![],
        };
        
        assert!(context.is_empty());
        assert_eq!(context.memory_count(), 0);
    }
    
    #[test]
    fn test_search_options() {
        let opts = SearchOptions::default()
            .with_limit(5)
            .with_min_similarity(0.7);
        
        assert_eq!(opts.limit, 5);
        assert!((opts.min_similarity - 0.7).abs() < 0.01);
    }
}
