//! ═══════════════════════════════════════════════════════════════════════════════
//!  VISION ENGINE - GÖRÜNTÜ ANALİZİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! OCR, UI element tespiti ve görüntü analizi yetenekleri.

use crate::error::HandsResult;
use crate::screen::ScreenCapture;
use serde::{Deserialize, Serialize};

// ───────────────────────────────────────────────────────────────────────────────
//  UI ELEMENT
// ───────────────────────────────────────────────────────────────────────────────

/// UI element tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    Button,
    Input,
    Link,
    Text,
    Image,
    Icon,
    Menu,
    MenuItem,
    Checkbox,
    Radio,
    Dropdown,
    Window,
    Dialog,
    Tab,
    Panel,
    Scrollbar,
    Slider,
    Progress,
    Unknown,
}

impl ElementType {
    pub fn is_clickable(&self) -> bool {
        matches!(self, 
            ElementType::Button | 
            ElementType::Link | 
            ElementType::MenuItem |
            ElementType::Checkbox |
            ElementType::Radio |
            ElementType::Icon
        )
    }
    
    pub fn is_interactive(&self) -> bool {
        matches!(self,
            ElementType::Button |
            ElementType::Input |
            ElementType::Link |
            ElementType::MenuItem |
            ElementType::Checkbox |
            ElementType::Radio |
            ElementType::Dropdown |
            ElementType::Slider |
            ElementType::Tab
        )
    }
}

/// UI element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// Element ID (unique)
    pub id: String,
    /// Element tipi
    pub element_type: ElementType,
    /// X koordinatı
    pub x: i32,
    /// Y koordinatı
    pub y: i32,
    /// Genişlik
    pub width: u32,
    /// Yükseklik
    pub height: u32,
    /// Metin içeriği (varsa)
    pub text: Option<String>,
    /// OCR güvenilirlik skoru
    pub confidence: f32,
    /// Alt elementler
    pub children: Vec<UIElement>,
    /// Ebeveyn ID
    pub parent_id: Option<String>,
    /// Ek özellikler
    pub attributes: std::collections::HashMap<String, String>,
}

impl UIElement {
    /// Merkez noktası
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width as i32 / 2, self.y + self.height as i32 / 2)
    }
    
    /// Sınır alanı
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.x + self.width as i32, self.y + self.height as i32)
    }
    
    /// Belirli bir noktayı içeriyor mu?
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width as i32 &&
        y >= self.y && y < self.y + self.height as i32
    }
    
    /// Başka bir elementle kesişiyor mu?
    pub fn intersects(&self, other: &UIElement) -> bool {
        let (x1_min, y1_min, x1_max, y1_max) = self.bounds();
        let (x2_min, y2_min, x2_max, y2_max) = other.bounds();
        
        x1_min < x2_max && x1_max > x2_min && y1_min < y2_max && y1_max > y2_min
    }
    
    /// Alan hesabı
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    
    /// Tıklanabilir mi?
    pub fn is_clickable(&self) -> bool {
        self.element_type.is_clickable()
    }
    
    /// Etkileşimli mi?
    pub fn is_interactive(&self) -> bool {
        self.element_type.is_interactive()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  VISION SONUÇ
// ───────────────────────────────────────────────────────────────────────────────

/// Görüntü analizi sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResult {
    /// Tespit edilen elementler
    pub elements: Vec<UIElement>,
    /// OCR metin
    pub text_content: Option<String>,
    /// İşlem süresi (ms)
    pub processing_time_ms: u64,
    /// Kullanılan model
    pub model: String,
    /// Güvenilirlik skoru
    pub overall_confidence: f32,
}

impl VisionResult {
    /// Element bul (açıklama ile)
    pub fn find_element(&self, description: &str) -> Option<&UIElement> {
        let desc_lower = description.to_lowercase();
        
        self.elements.iter().find(|e| {
            // Tip eşleşmesi
            if format!("{:?}", e.element_type).to_lowercase().contains(&desc_lower) {
                return true;
            }
            
            // Metin eşleşmesi
            if let Some(ref text) = e.text {
                if text.to_lowercase().contains(&desc_lower) {
                    return true;
                }
            }
            
            false
        })
    }
    
    /// Tıklanabilir elementleri getir
    pub fn clickable_elements(&self) -> Vec<&UIElement> {
        self.elements.iter().filter(|e| e.is_clickable()).collect()
    }
    
    /// Etkileşimli elementleri getir
    pub fn interactive_elements(&self) -> Vec<&UIElement> {
        self.elements.iter().filter(|e| e.is_interactive()).collect()
    }
    
    /// Metin içeren elementleri getir
    pub fn text_elements(&self) -> Vec<&UIElement> {
        self.elements.iter().filter(|e| e.text.is_some()).collect()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  VISION MOTORU
// ───────────────────────────────────────────────────────────────────────────────

/// Görüntü analiz motoru
pub struct VisionEngine {
    /// OCR aktif mi?
    ocr_enabled: bool,
    /// UI tespit aktif mi?
    ui_detection_enabled: bool,
    /// Minimum güvenilirlik eşiği
    confidence_threshold: f32,
    /// Kullanılan model
    model: String,
}

impl VisionEngine {
    /// Yeni vision motoru oluştur
    pub fn new(ocr_enabled: bool) -> Self {
        log::info!("👁️  VISION: Görüntü analiz motoru başlatılıyor...");
        log::info!("👁️  VISION: OCR: {}, UI Detection: {}", ocr_enabled, true);
        
        Self {
            ocr_enabled,
            ui_detection_enabled: true,
            confidence_threshold: 0.7,
            model: "sentient-vision-v1".into(),
        }
    }
    
    /// Ekran görüntüsünü analiz et
    pub async fn analyze(&self, capture: &ScreenCapture) -> HandsResult<VisionResult> {
        let start = std::time::Instant::now();
        
        log::debug!("👁️  VISION: Görüntü analizi başlıyor ({}x{})", 
            capture.width, capture.height);
        
        let mut elements = Vec::new();
        
        // UI element tespiti (mock)
        if self.ui_detection_enabled {
            elements = self.detect_ui_elements(capture).await?;
        }
        
        // OCR (mock)
        let text_content = if self.ocr_enabled {
            self.perform_ocr(capture).await?
        } else {
            None
        };
        
        let processing_time = start.elapsed().as_millis() as u64;
        
        log::info!("👁️  VISION: {} element tespit edildi ({}ms)", 
            elements.len(), processing_time);
        
        Ok(VisionResult {
            elements,
            text_content,
            processing_time_ms: processing_time,
            model: self.model.clone(),
            overall_confidence: 0.85,
        })
    }
    
    /// UI element tespiti
    async fn detect_ui_elements(&self, _capture: &ScreenCapture) -> HandsResult<Vec<UIElement>> {
        // Mock tespit - gerçek uygulamada ML modeli kullanılır
        let elements = vec![
            UIElement {
                id: "elem-1".into(),
                element_type: ElementType::Button,
                x: 100,
                y: 100,
                width: 120,
                height: 40,
                text: Some("Tamam".into()),
                confidence: 0.92,
                children: vec![],
                parent_id: None,
                attributes: std::collections::HashMap::new(),
            },
            UIElement {
                id: "elem-2".into(),
                element_type: ElementType::Input,
                x: 250,
                y: 100,
                width: 300,
                height: 35,
                text: None,
                confidence: 0.88,
                children: vec![],
                parent_id: None,
                attributes: [("placeholder".into(), "Arama...".into())].into_iter().collect(),
            },
            UIElement {
                id: "elem-3".into(),
                element_type: ElementType::Link,
                x: 600,
                y: 200,
                width: 80,
                height: 25,
                text: Some("Daha fazla".into()),
                confidence: 0.95,
                children: vec![],
                parent_id: None,
                attributes: std::collections::HashMap::new(),
            },
        ];
        
        Ok(elements)
    }
    
    /// OCR işlemini gerçekleştir
    async fn perform_ocr(&self, _capture: &ScreenCapture) -> HandsResult<Option<String>> {
        // Mock OCR - gerçek uygulamada Tesseract/Benabrait kullanılır
        Ok(Some("Örnek metin içeriği".into()))
    }
    
    /// Element bul (açıklama ile)
    pub async fn find_element(&self, capture: &ScreenCapture, description: &str) -> HandsResult<Option<UIElement>> {
        let result = self.analyze(capture).await?;
        
        if let Some(element) = result.find_element(description) {
            log::info!("👁️  VISION: Element bulundu → '{}' ({:?})", 
                description, element.element_type);
            Ok(Some(element.clone()))
        } else {
            log::warn!("👁️  VISION: Element bulunamadı → '{}'", description);
            Ok(None)
        }
    }
    
    /// En yakın elementi bul
    pub async fn find_nearest_element(&self, capture: &ScreenCapture, x: i32, y: i32) -> HandsResult<Option<UIElement>> {
        let result = self.analyze(capture).await?;
        
        let nearest = result.elements.iter()
            .min_by(|a, b| {
                let dist_a = ((a.x - x).pow(2) + (a.y - y).pow(2)) as f64;
                let dist_b = ((b.x - x).pow(2) + (b.y - y).pow(2)) as f64;
                dist_a.partial_cmp(&dist_b).unwrap()
            });
        
        Ok(nearest.cloned())
    }
    
    /// Metin içeren elementleri bul
    pub async fn find_text_elements(&self, capture: &ScreenCapture, text: &str) -> HandsResult<Vec<UIElement>> {
        let result = self.analyze(capture).await?;
        
        let matching: Vec<_> = result.elements.iter()
            .filter(|e| {
                e.text.as_ref()
                    .map(|t| t.to_lowercase().contains(&text.to_lowercase()))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        
        log::info!("👁️  VISION: '{}' içeren {} element bulundu", text, matching.len());
        Ok(matching)
    }
    
    /// Güvenilirlik eşiğini ayarla
    pub fn set_confidence_threshold(&mut self, threshold: f32) {
        self.confidence_threshold = threshold.clamp(0.0, 1.0);
    }
}

impl Default for VisionEngine {
    fn default() -> Self {
        Self::new(true)
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_element_type_clickable() {
        assert!(ElementType::Button.is_clickable());
        assert!(ElementType::Link.is_clickable());
        assert!(!ElementType::Text.is_clickable());
    }
    
    #[test]
    fn test_element_type_interactive() {
        assert!(ElementType::Input.is_interactive());
        assert!(ElementType::Dropdown.is_interactive());
        assert!(!ElementType::Image.is_interactive());
    }
    
    #[test]
    fn test_ui_element_center() {
        let element = UIElement {
            id: "test".into(),
            element_type: ElementType::Button,
            x: 100,
            y: 100,
            width: 50,
            height: 30,
            text: None,
            confidence: 0.9,
            children: vec![],
            parent_id: None,
            attributes: std::collections::HashMap::new(),
        };
        
        let (cx, cy) = element.center();
        assert_eq!(cx, 125);
        assert_eq!(cy, 115);
    }
    
    #[test]
    fn test_ui_element_contains() {
        let element = UIElement {
            id: "test".into(),
            element_type: ElementType::Button,
            x: 100,
            y: 100,
            width: 50,
            height: 30,
            text: None,
            confidence: 0.9,
            children: vec![],
            parent_id: None,
            attributes: std::collections::HashMap::new(),
        };
        
        assert!(element.contains(110, 110));
        assert!(!element.contains(200, 200));
    }
    
    #[test]
    fn test_ui_element_area() {
        let element = UIElement {
            id: "test".into(),
            element_type: ElementType::Button,
            x: 0,
            y: 0,
            width: 100,
            height: 50,
            text: None,
            confidence: 0.9,
            children: vec![],
            parent_id: None,
            attributes: std::collections::HashMap::new(),
        };
        
        assert_eq!(element.area(), 5000);
    }
    
    #[tokio::test]
    async fn test_vision_engine_creation() {
        let engine = VisionEngine::new(true);
        assert!(engine.ocr_enabled);
    }
    
    #[tokio::test]
    async fn test_vision_analyze() {
        let engine = VisionEngine::new(true);
        let capture = ScreenCapture::mock(800, 600);
        
        let result = engine.analyze(&capture).await.unwrap();
        assert!(result.elements.len() > 0);
    }
    
    #[tokio::test]
    async fn test_find_element() {
        let engine = VisionEngine::new(true);
        let capture = ScreenCapture::mock(800, 600);
        
        let element = engine.find_element(&capture, "button").await.unwrap();
        assert!(element.is_some());
    }
}
