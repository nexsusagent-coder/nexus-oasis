//! Ortak tip tanımları

use serde::{Deserialize, Serialize};

/// Cognitive strateji türleri
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    /// Doğrudan yanıt, düşünme adımı yok
    Direct,
    /// İç ses üretimi, adım adım analiz
    Think,
    /// Çoklu perspektif, tartışma
    Debate,
    /// Ağaç yapısında düşünme
    TreeOfThoughts,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Think
    }
}

impl Strategy {
    /// Giriş metnine göre otomatik strateji seç
    pub fn auto_select(input: &str) -> Self {
        let input_lower = input.to_lowercase();
        
        // Karmaşık problem belirteçleri
        if input_lower.contains("neden") || input_lower.contains("nasıl çalışır") {
            return Self::Think;
        }
        
        // Tartışma belirteçleri
        if input_lower.contains("avantaj") || input_lower.contains("dezavantaj") 
            || input_lower.contains("karşılaştır") {
            return Self::Debate;
        }
        
        // Derin analiz belirteçleri
        if input_lower.contains("analiz et") || input_lower.contains("çöz")
            || input_lower.contains("debug") || input_lower.contains("hata") {
            return Self::TreeOfThoughts;
        }
        
        // Varsayılan
        Self::Direct
    }
    
    /// Strateji adını döndür
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Think => "think",
            Self::Debate => "debate",
            Self::TreeOfThoughts => "tree_of_thoughts",
        }
    }
}

/// Üretilen çıktı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOutput {
    /// Üretilen metin
    pub text: String,
    /// Token sayısı
    pub token_count: usize,
    /// Kullanılan strateji
    pub strategy: Strategy,
    /// Düşünce adımları (varsa)
    pub reasoning: Option<Vec<String>>,
    /// İşlem süresi (ms)
    pub duration_ms: u64,
}

/// Tokenizasyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationResult {
    /// Token string'leri
    pub tokens: Vec<String>,
    /// Token ID'leri
    pub ids: Vec<u32>,
    /// OOV oranı
    pub unk_ratio: f32,
}

/// Bellek kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Anahtar
    pub key: String,
    /// Değer
    pub value: String,
    /// Embedding (varsa)
    pub embedding: Option<Vec<f32>>,
    /// Zaman damgası
    pub timestamp: i64,
}

/// Tool tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    /// Tool adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Parametreler
    pub parameters: Vec<String>,
}

/// Model durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelState {
    /// Model başlatıldı mı?
    pub initialized: bool,
    /// Cihaz
    pub device: String,
    /// Vocabulary boyutu
    pub vocab_size: usize,
    /// Embedding boyutu
    pub embed_dim: usize,
    /// Katman sayısı
    pub num_layers: usize,
    /// Head sayısı
    pub num_heads: usize,
    /// Parameter sayısı
    pub param_count: usize,
}

/// Cognitive çıktı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveResult {
    /// Yanıt
    pub response: String,
    /// Kullanılan strateji
    pub strategy: Strategy,
    /// Düşünce adımları
    pub thoughts: Option<Vec<ThoughtStep>>,
    /// Tool çağrıları
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Bellek erişimleri
    pub memory_access: Option<Vec<String>>,
    /// Güven skoru
    pub confidence: f32,
}

/// Düşünce adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtStep {
    /// Adım numarası
    pub step: usize,
    /// Düşünce içeriği
    pub content: String,
    /// Sonuç (varsa)
    pub result: Option<String>,
}

/// Tool çağrısı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool adı
    pub tool: String,
    /// Argümanlar
    pub args: Vec<String>,
    /// Sonuç
    pub result: String,
}

/// Decoding konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodingConfig {
    /// Maksimum yeni token sayısı
    pub max_new_tokens: usize,
    /// Temperature
    pub temperature: f32,
    /// Top-p (nucleus sampling)
    pub top_p: f32,
    /// Top-k
    pub top_k: usize,
    /// Repetition penalty
    pub repetition_penalty: f32,
}

impl Default for DecodingConfig {
    fn default() -> Self {
        Self {
            max_new_tokens: 128,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            repetition_penalty: 1.0,
        }
    }
}
