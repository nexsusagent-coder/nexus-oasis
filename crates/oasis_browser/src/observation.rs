//! ═══════════════════════════════════════════════════════════════════════════════
//!  OBSERVATION PIPELINE - DOM → LLM OPTİMİZED FORMAT
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Web sayfası DOM'unu LLM'in verimli işleyebileceği "Observation" formatına dönüştürür.
//!
//! ═──────────────────────────────────────────────────────────────────────────────
//!  DÖNÜŞÜM AŞAMALARI:
//!  ─────────────────
//!  1. DOM Extraction → Ham DOM ağacı
//!  2. Pruning        → Gereksiz elementleri kaldır (script, style, meta, vs.)
//!  3. Compression    → Tekrarlanan yapıları özetle
//!  4. Structuring    → Hiyerarşik, LLM-dostu format
//!  5. Token Budget   → MAX_OBSERVATION_TOKENS sınırla
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::BrowserResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────────────────
//  OBSERVATION (ANA VERI YAPISI)
// ─────────────────────────────────────────────────────────────────────────────--

/// LLM için optimize edilmiş sayfa gözlemi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// Gözlem ID
    pub id: String,
    /// Zaman damgası
    pub timestamp: String,
    /// Mevcut URL
    pub url: String,
    /// Sayfa başlığı
    pub title: String,
    /// Sayfa durumu
    pub state: PageState,
    /// Görünür elementler
    pub elements: Vec<DOMElement>,
    /// Etkileşim elementleri (buton, link, input)
    pub interactive_elements: Vec<InteractiveElement>,
    /// Sayfa metni özeti
    pub text_content: String,
    /// Bağlantılar
    pub links: Vec<LinkInfo>,
    /// Formlar
    pub forms: Vec<FormInfo>,
    /// Metadata (LLM'ye ek bilgi)
    pub metadata: ObservationMetadata,
    /// Token tahmini
    pub estimated_tokens: usize,
}

/// Sayfa durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PageState {
    /// Sayfa yükleniyor
    Loading,
    /// Sayfa hazır
    Ready,
    /// Hata oluştu
    Error,
    /// Timeout
    Timeout,
}

/// DOM Element (basitleştirilmiş)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DOMElement {
    /// Element ID (hash)
    pub id: String,
    /// Element türü (div, span, a, button, input, ...)
    pub tag: String,
    /// Element içeriği (kısaltılmış)
    pub text: Option<String>,
    /// CSS sınıfları
    pub classes: Vec<String>,
    /// data-* nitelikleri
    pub data_attrs: HashMap<String, String>,
    /// ARIA nitelikleri
    pub aria: HashMap<String, String>,
    /// Pozisyon (ekran koordinatları)
    pub bounds: Option<ElementBounds>,
    /// Görünür mü?
    pub visible: bool,
    /// Derinlik seviyesi
    pub depth: u32,
    /// Element yolu (CSS selector)
    pub selector: String,
}

/// Etkileşim elementi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveElement {
    /// Element ID
    pub id: String,
    /// Element türü
    pub element_type: InteractiveType,
    /// Açıklama (LLM için)
    pub description: String,
    /// CSS selector
    pub selector: String,
    /// XPath
    pub xpath: String,
    /// Mevcut değer (input için)
    pub value: Option<String>,
    /// Yer tutucu metin
    pub placeholder: Option<String>,
    /// Devre dışı mı?
    pub disabled: bool,
    /// Pozisyon
    pub bounds: ElementBounds,
}

/// Etkileşim element türü
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InteractiveType {
    Button,
    Link,
    TextInput,
    TextArea,
    Select,
    Checkbox,
    Radio,
    FileInput,
    SubmitButton,
    Image,
    Video,
    Audio,
    Iframe,
    Other,
}

/// Element sınırları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Bağlantı bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub text: String,
    pub href: String,
    pub internal: bool,
    pub selector: String,
}

/// Form bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormInfo {
    pub id: Option<String>,
    pub action: Option<String>,
    pub method: String,
    pub fields: Vec<FormField>,
}

/// Form alanı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub field_type: String,
    pub label: Option<String>,
    pub required: bool,
    pub selector: String,
}

/// Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationMetadata {
    /// Sayfa dili
    pub language: Option<String>,
    /// Charset
    pub charset: Option<String>,
    /// Viewport boyutları
    pub viewport: ViewportInfo,
    /// Scroll pozisyonu
    pub scroll: ScrollInfo,
    /// Sayfa boyutu (byte)
    pub page_size: usize,
    /// DOM node sayısı
    pub dom_nodes: usize,
    /// Yükleme süresi (ms)
    pub load_time_ms: u64,
    /// Kaynak sayısı
    pub resource_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportInfo {
    pub width: u32,
    pub height: u32,
    pub device_pixel_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollInfo {
    pub scroll_x: f64,
    pub scroll_y: f64,
    pub scroll_max_x: f64,
    pub scroll_max_y: f64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  OBSERVATION PIPELINE
// ─────────────────────────────────────────────────────────────────────────────--

/// Observation üretim hattı
pub struct ObservationPipeline {
    /// Maksimum token
    max_tokens: usize,
    /// Prune edilecek elementler
    prune_tags: Vec<String>,
    /// Prune edilecek class'lar
    prune_classes: Vec<String>,
    /// Element derinlik sınırı
    max_depth: u32,
    /// Metin uzunluk sınırı
    max_text_length: usize,
}

impl Default for ObservationPipeline {
    fn default() -> Self {
        Self {
            max_tokens: crate::MAX_OBSERVATION_TOKENS,
            prune_tags: vec![
                "script".into(),
                "style".into(),
                "noscript".into(),
                "meta".into(),
                "link".into(),
                "head".into(),
                "svg".into(),
                "path".into(),
            ],
            prune_classes: vec![
                "hidden".into(),
                "sr-only".into(),
                "visually-hidden".into(),
                "ads".into(),
                "advertisement".into(),
                "banner".into(),
                "popup".into(),
                "modal-backdrop".into(),
            ],
            max_depth: 15,
            max_text_length: 200,
        }
    }
}

impl ObservationPipeline {
    /// Yeni pipeline oluştur
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Ham DOM'dan observation üret
    pub fn process(&self, raw_dom: &str, url: &str) -> BrowserResult<Observation> {
        log::debug!("🔍  OBSERVATION: DOM işleniyor... ({} byte)", raw_dom.len());
        
        // 1) Parse DOM
        let parsed = self.parse_dom(raw_dom)?;
        
        // 2) Prune
        let pruned = self.prune_elements(parsed);
        
        // 3) Extract interactive elements
        let interactive = self.extract_interactive(&pruned);
        
        // 4) Build observation
        let observation = self.build_observation(pruned, interactive, url);
        
        // 5) Token budget kontrol
        let observation = self.enforce_token_budget(observation)?;
        
        log::info!("🔍  OBSERVATION: {} element, ~{} token", 
            observation.elements.len(), observation.estimated_tokens);
        
        Ok(observation)
    }
    
    /// DOM parse et (basitleştirilmiş)
    fn parse_dom(&self, _raw_dom: &str) -> BrowserResult<Vec<DOMElement>> {
        // Gerçek implementasyonda html5ever veya scraper kullanılır
        // Şimdilik mock data dönüyoruz
        Ok(vec![])
    }
    
    /// Gereksiz elementleri kaldır
    fn prune_elements(&self, mut elements: Vec<DOMElement>) -> Vec<DOMElement> {
        elements.retain(|e| {
            // Tag kontrolü
            if self.prune_tags.contains(&e.tag.to_lowercase()) {
                return false;
            }
            
            // Class kontrolü
            for class in &e.classes {
                if self.prune_classes.iter().any(|pc| class.contains(pc)) {
                    return false;
                }
            }
            
            // Derinlik kontrolü
            if e.depth > self.max_depth {
                return false;
            }
            
            true
        });
        
        elements
    }
    
    /// Etkileşim elementlerini çıkar
    fn extract_interactive(&self, elements: &[DOMElement]) -> Vec<InteractiveElement> {
        let mut interactive = Vec::new();
        
        for elem in elements {
            let int_type = match elem.tag.to_lowercase().as_str() {
                "a" => Some(InteractiveType::Link),
                "button" => Some(InteractiveType::Button),
                "input" => {
                    // Input type'a göre
                    Some(InteractiveType::TextInput) // Basitleştirilmiş
                }
                "textarea" => Some(InteractiveType::TextArea),
                "select" => Some(InteractiveType::Select),
                "img" => Some(InteractiveType::Image),
                "video" => Some(InteractiveType::Video),
                "iframe" => Some(InteractiveType::Iframe),
                _ => {
                    // role veya tabindex kontrolü
                    if elem.aria.contains_key("role") || elem.data_attrs.contains_key("tabindex") {
                        Some(InteractiveType::Other)
                    } else {
                        None
                    }
                }
            };
            
            if let Some(element_type) = int_type {
                interactive.push(InteractiveElement {
                    id: elem.id.clone(),
                    element_type,
                    description: self.generate_description(elem),
                    selector: elem.selector.clone(),
                    xpath: format!("//{}", elem.tag), // Basitleştirilmiş
                    value: None,
                    placeholder: elem.data_attrs.get("placeholder").cloned(),
                    disabled: elem.classes.contains(&"disabled".to_string()),
                    bounds: elem.bounds.clone().unwrap_or(ElementBounds {
                        x: 0.0, y: 0.0, width: 0.0, height: 0.0
                    }),
                });
            }
        }
        
        interactive
    }
    
    /// Element açıklaması üret
    fn generate_description(&self, elem: &DOMElement) -> String {
        // Önce aria-label
        if let Some(label) = elem.aria.get("label") {
            return label.clone();
        }
        
        // Sonra text içeriği
        if let Some(text) = &elem.text {
            let shortened = text.chars().take(50).collect::<String>();
            return if text.len() > 50 {
                format!("{}...", shortened)
            } else {
                shortened
            };
        }
        
        // Sonra title veya alt
        if let Some(title) = elem.data_attrs.get("title") {
            return title.clone();
        }
        
        if let Some(alt) = elem.data_attrs.get("alt") {
            return alt.clone();
        }
        
        // Fallback
        format!("<{}>", elem.tag)
    }
    
    /// Observation oluştur
    fn build_observation(
        &self, 
        elements: Vec<DOMElement>,
        interactive: Vec<InteractiveElement>,
        url: &str,
    ) -> Observation {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        // Metin içeriği topla
        let text_content = elements
            .iter()
            .filter_map(|e| e.text.clone())
            .collect::<Vec<_>>()
            .join(" ");
        
        // Token tahmini (kabaca: 1 token ≈ 4 karakter)
        let estimated_tokens = text_content.len() / 4 
            + interactive.len() * 50 
            + elements.len() * 20;
        
        Observation {
            id,
            timestamp,
            url: url.to_string(),
            title: String::new(),
            state: PageState::Ready,
            elements,
            interactive_elements: interactive,
            text_content,
            links: vec![],
            forms: vec![],
            metadata: ObservationMetadata {
                language: None,
                charset: Some("utf-8".into()),
                viewport: ViewportInfo {
                    width: 1920,
                    height: 1080,
                    device_pixel_ratio: 1.0,
                },
                scroll: ScrollInfo {
                    scroll_x: 0.0,
                    scroll_y: 0.0,
                    scroll_max_x: 0.0,
                    scroll_max_y: 0.0,
                },
                page_size: 0,
                dom_nodes: 0,
                load_time_ms: 0,
                resource_count: 0,
            },
            estimated_tokens,
        }
    }
    
    /// Token bütçesini uygula
    fn enforce_token_budget(&self, mut observation: Observation) -> BrowserResult<Observation> {
        if observation.estimated_tokens <= self.max_tokens {
            return Ok(observation);
        }
        
        log::warn!("⚠️  OBSERVATION: Token bütçesi aşıldı ({} > {}), budanıyor...",
            observation.estimated_tokens, self.max_tokens);
        
        // Metni kırp
        if observation.text_content.len() > self.max_tokens * 3 {
            observation.text_content = observation.text_content
                .chars()
                .take(self.max_tokens * 3)
                .collect();
        }
        
        // Element sayısını azalt (önemli olanları koru)
        if observation.elements.len() > self.max_tokens / 20 {
            // Interactive elementleri koru
            let interactive_ids: Vec<_> = observation.interactive_elements
                .iter()
                .map(|i| i.id.as_str())
                .collect();
            
            observation.elements.sort_by_key(|e| {
                // Interactive elementlere öncelik ver
                if interactive_ids.contains(&e.id.as_str()) {
                    0
                } else if e.visible {
                    1
                } else {
                    2
                }
            });
            
            observation.elements.truncate(self.max_tokens / 20);
        }
        
        // Token'ı yeniden hesapla
        observation.estimated_tokens = observation.text_content.len() / 4
            + observation.interactive_elements.len() * 50
            + observation.elements.len() * 20;
        
        Ok(observation)
    }
    
    /// Observation'ı LLM için string formatına çevir
    pub fn to_llm_format(&self, observation: &Observation) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("=== SAYFA DURUMU ===\n"));
        output.push_str(&format!("URL: {}\n", observation.url));
        output.push_str(&format!("Başlık: {}\n", observation.title));
        output.push_str(&format!("Durum: {:?}\n\n", observation.state));
        
        if !observation.interactive_elements.is_empty() {
            output.push_str("=== ETKİLEŞİM ELEMENTLERİ ===\n");
            for (i, elem) in observation.interactive_elements.iter().enumerate() {
                output.push_str(&format!(
                    "[{}] {} - '{}' ({} selector)\n",
                    i + 1,
                    format!("{:?}", elem.element_type),
                    elem.description,
                    elem.selector
                ));
            }
            output.push_str("\n");
        }
        
        if !observation.text_content.is_empty() {
            output.push_str("=== SAYFA İÇERİĞİ ===\n");
            output.push_str(&observation.text_content);
            output.push_str("\n");
        }
        
        output
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_creation() {
        let pipeline = ObservationPipeline::new();
        assert_eq!(pipeline.max_tokens, 8000);
    }
    
    #[test]
    fn test_prune_tags() {
        let pipeline = ObservationPipeline::new();
        assert!(pipeline.prune_tags.contains(&"script".to_string()));
        assert!(pipeline.prune_tags.contains(&"style".to_string()));
    }
    
    #[test]
    fn test_interactive_type_detection() {
        let elem = DOMElement {
            id: "test".into(),
            tag: "button".into(),
            text: Some("Tıkla".into()),
            classes: vec![],
            data_attrs: HashMap::new(),
            aria: HashMap::new(),
            bounds: None,
            visible: true,
            depth: 1,
            selector: "button".into(),
        };
        
        let pipeline = ObservationPipeline::new();
        let interactive = pipeline.extract_interactive(&[elem]);
        
        assert_eq!(interactive.len(), 1);
        assert_eq!(interactive[0].element_type, InteractiveType::Button);
    }
    
    #[test]
    fn test_token_budget() {
        let mut observation = Observation {
            id: "test".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            url: "https://example.com".into(),
            title: "Test".into(),
            state: PageState::Ready,
            elements: vec![],
            interactive_elements: vec![],
            text_content: "a".repeat(50000), // Çok uzun metin
            links: vec![],
            forms: vec![],
            metadata: ObservationMetadata::default(),
            estimated_tokens: 50000,
        };
        
        let pipeline = ObservationPipeline::new();
        let result = pipeline.enforce_token_budget(observation).expect("operation failed");
        
        assert!(result.estimated_tokens <= pipeline.max_tokens * 2); // Esnek bütçe
    }
    
    #[test]
    fn test_llm_format() {
        let observation = Observation {
            id: "test".into(),
            timestamp: "2024-01-01T00:00:00Z".into(),
            url: "https://example.com".into(),
            title: "Test Sayfa".into(),
            state: PageState::Ready,
            elements: vec![],
            interactive_elements: vec![InteractiveElement {
                id: "btn1".into(),
                element_type: InteractiveType::Button,
                description: "Gönder".into(),
                selector: "#submit-btn".into(),
                xpath: "//button".into(),
                value: None,
                placeholder: None,
                disabled: false,
                bounds: ElementBounds { x: 0.0, y: 0.0, width: 100.0, height: 40.0 },
            }],
            text_content: "Merhaba dünya".into(),
            links: vec![],
            forms: vec![],
            metadata: ObservationMetadata::default(),
            estimated_tokens: 100,
        };
        
        let pipeline = ObservationPipeline::new();
        let formatted = pipeline.to_llm_format(&observation);
        
        assert!(formatted.contains("SAYFA DURUMU"));
        assert!(formatted.contains("ETKİLEŞİM ELEMENTLERİ"));
        assert!(formatted.contains("Gönder"));
    }
}

impl Default for ObservationMetadata {
    fn default() -> Self {
        Self {
            language: None,
            charset: Some("utf-8".into()),
            viewport: ViewportInfo {
                width: 1920,
                height: 1080,
                device_pixel_ratio: 1.0,
            },
            scroll: ScrollInfo {
                scroll_x: 0.0,
                scroll_y: 0.0,
                scroll_max_x: 0.0,
                scroll_max_y: 0.0,
            },
            page_size: 0,
            dom_nodes: 0,
            load_time_ms: 0,
            resource_count: 0,
        }
    }
}
