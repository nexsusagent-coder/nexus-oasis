//! ═══════════════════════════════════════════════════════════════════════════════
//!  ENHANCED VISION - Gelişmiş Görü Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::AutonomousResult;
use crate::screen::{Rectangle, ScreenRegion};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  ELEMENT TYPE
// ═══════════════════════════════════════════════════════════════════════════════

/// UI element türü
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
    Table,
    TableRow,
    TableCell,
    List,
    ListItem,
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
            ElementType::Icon |
            ElementType::Tab
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
    
    pub fn is_text_input(&self) -> bool {
        matches!(self, ElementType::Input | ElementType::Dropdown)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  UI ELEMENT
// ═══════════════════════════════════════════════════════════════════════════════

/// UI Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    /// Element ID
    pub id: String,
    /// Element türü
    pub element_type: ElementType,
    /// X koordinatı
    pub x: i32,
    /// Y koordinatı
    pub y: i32,
    /// Genişlik
    pub width: u32,
    /// Yükseklik
    pub height: u32,
    /// Metin içeriği
    pub text: Option<String>,
    /// Güven skoru
    pub confidence: f32,
    /// Bounds
    pub bounds: Rectangle,
    /// İnteraktif mi?
    pub is_interactive: bool,
    /// Görünür mü?
    pub visible: bool,
    /// Etiket
    pub label: Option<String>,
    /// Değer
    pub value: Option<String>,
    /// Placeholder
    pub placeholder: Option<String>,
    /// Aktif mi?
    pub enabled: bool,
    /// CSS sınıfları
    pub classes: Vec<String>,
    /// Nitelikler
    pub attributes: HashMap<String, String>,
    /// Ek veri
    pub metadata: HashMap<String, String>,
}

impl UIElement {
    /// Merkez nokta
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width as i32 / 2, self.y + self.height as i32 / 2)
    }
    
    /// Nokta içeride mi?
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.bounds.contains(x, y)
    }
    
    /// Açıklama üret
    pub fn describe(&self) -> String {
        if let Some(text) = &self.text {
            format!("{:?}: \"{}\"", self.element_type, text)
        } else if let Some(label) = &self.label {
            format!("{:?}: [{}]", self.element_type, label)
        } else {
            format!("{:?} at ({}, {})", self.element_type, self.x, self.y)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VISION RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Görü analizi sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResult {
    /// Elementler
    pub elements: Vec<UIElement>,
    /// Metin içeriği
    pub full_text: String,
    /// İşlem süresi
    pub processing_time_ms: u64,
    /// Model
    pub model: String,
    /// Genel güven
    pub overall_confidence: f32,
    /// Tespit edilen bölgeler
    pub regions: Vec<DetectedRegion>,
}

/// Tespit edilen bölge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedRegion {
    pub region_type: ScreenRegion,
    pub bounds: Rectangle,
    pub confidence: f32,
    pub elements_count: usize,
}

/// Gözlem sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// Gözlem ID
    pub id: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Ekran görüntüsü (base64)
    pub screenshot: Option<String>,
    /// Ekran boyutu
    pub screen_size: (u32, u32),
    /// Tespit edilen elementler
    pub elements: Vec<UIElement>,
    /// Metin içeriği
    pub text_content: String,
    /// Fare pozisyonu
    pub mouse_position: (i32, i32),
}

impl Default for Observation {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            screenshot: None,
            screen_size: (1920, 1080),
            elements: vec![],
            text_content: String::new(),
            mouse_position: (0, 0),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ENHANCED VISION ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Gelişmiş görü motoru
pub struct EnhancedVision {
    /// OCR aktif
    ocr_enabled: bool,
    /// UI detection aktif
    ui_detection_enabled: bool,
    /// Template matching aktif
    template_matching_enabled: bool,
    /// Minimum güven eşiği
    confidence_threshold: f32,
    /// Model adı
    model: String,
}

impl EnhancedVision {
    pub fn new() -> Self {
        log::info!("👁️ VISION: Gelişmiş görü motoru başlatılıyor...");
        
        Self {
            ocr_enabled: true,
            ui_detection_enabled: true,
            template_matching_enabled: true,
            confidence_threshold: 0.7,
            model: "sentient-vision-v2".into(),
        }
    }
    
    /// Görüntüyü analiz et
    pub async fn analyze(&self, image_data: &[u8]) -> AutonomousResult<VisionResult> {
        let start = std::time::Instant::now();
        
        log::debug!("👁️ VISION: Analyzing {} bytes", image_data.len());
        
        let mut elements = Vec::new();
        
        // UI element tespiti
        if self.ui_detection_enabled {
            elements = self.detect_ui_elements(image_data).await?;
        }
        
        // OCR
        let full_text = if self.ocr_enabled {
            self.perform_ocr(image_data).await?
        } else {
            String::new()
        };
        
        let processing_time = start.elapsed().as_millis() as u64;
        
        log::info!("👁️ VISION: {} elements detected, {}ms", 
            elements.len(), processing_time);
        
        Ok(VisionResult {
            elements,
            full_text,
            processing_time_ms: processing_time,
            model: self.model.clone(),
            overall_confidence: 0.85,
            regions: vec![],
        })
    }
    
    /// Element bul
    pub async fn find_element(&self, image_data: &[u8], description: &str) -> AutonomousResult<Option<UIElement>> {
        let result = self.analyze(image_data).await?;
        
        let desc_lower = description.to_lowercase();
        
        for element in &result.elements {
            // Tip eşleşmesi
            if format!("{:?}", element.element_type).to_lowercase().contains(&desc_lower) {
                return Ok(Some(element.clone()));
            }
            
            // Metin eşleşmesi
            if let Some(text) = &element.text {
                if text.to_lowercase().contains(&desc_lower) {
                    return Ok(Some(element.clone()));
                }
            }
            
            // Label eşleşmesi
            if let Some(label) = &element.label {
                if label.to_lowercase().contains(&desc_lower) {
                    return Ok(Some(element.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Koordinata göre element bul
    pub async fn find_element_at(&self, image_data: &[u8], x: i32, y: i32) -> AutonomousResult<Option<UIElement>> {
        let result = self.analyze(image_data).await?;
        
        for element in &result.elements {
            if element.contains(x, y) {
                return Ok(Some(element.clone()));
            }
        }
        
        Ok(None)
    }
    
    /// Metin içeren elementleri bul
    pub async fn find_elements_with_text(&self, image_data: &[u8], text: &str) -> AutonomousResult<Vec<UIElement>> {
        let result = self.analyze(image_data).await?;
        
        let matching: Vec<_> = result.elements.iter()
            .filter(|e| {
                e.text.as_ref()
                    .map(|t| t.to_lowercase().contains(&text.to_lowercase()))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        
        Ok(matching)
    }
    
    /// Tıklanabilir elementleri bul
    pub async fn find_clickable_elements(&self, image_data: &[u8]) -> AutonomousResult<Vec<UIElement>> {
        let result = self.analyze(image_data).await?;
        
        let clickable: Vec<_> = result.elements.iter()
            .filter(|e| e.element_type.is_clickable())
            .cloned()
            .collect();
        
        Ok(clickable)
    }
    
    /// UI element tespiti
    async fn detect_ui_elements(&self, _image_data: &[u8]) -> AutonomousResult<Vec<UIElement>> {
        // OCR integration would happen at a higher level
        // For production, integrate with:
        // - ONNX Runtime for UI detection model
        // - Accessibility APIs (AT-SPI on Linux, UI Automation on Windows)
        // - Tesseract for OCR (feature: tesseract)
        
        Ok(vec![
            UIElement {
                id: "btn-1".into(),
                element_type: ElementType::Button,
                x: 100,
                y: 100,
                width: 120,
                height: 40,
                text: Some("Tamam".into()),
                confidence: 0.92,
                bounds: Rectangle::new(100, 100, 120, 40),
                is_interactive: true,
                visible: true,
                label: None,
                value: None,
                placeholder: None,
                enabled: true,
                classes: vec![],
                attributes: HashMap::new(),
                metadata: HashMap::new(),
            },
            UIElement {
                id: "input-1".into(),
                element_type: ElementType::Input,
                x: 250,
                y: 100,
                width: 300,
                height: 35,
                text: None,
                confidence: 0.88,
                bounds: Rectangle::new(250, 100, 300, 35),
                is_interactive: true,
                visible: true,
                label: Some("Kullanıcı adı".into()),
                value: None,
                placeholder: Some("Kullanıcı adınızı girin".into()),
                enabled: true,
                classes: vec![],
                attributes: HashMap::new(),
                metadata: HashMap::new(),
            },
        ])
    }
    
    /// OCR - text extraction from image
    async fn perform_ocr(&self, _image_data: &[u8]) -> AutonomousResult<String> {
        // Tesseract OCR integration (feature: tesseract)
        // When enabled, this will extract text from screenshots
        
        // Fallback - mock text
        Ok("Örnek metin içeriği".into())
    }
    
    /// Template matching - find pattern in image
    pub async fn find_template(&self, _image_data: &[u8], _template: &[u8]) -> AutonomousResult<Option<(i32, i32)>> {
        // OpenCV template matching (future: opencv-rust feature)
        // For now, simple placeholder
        
        Ok(Some((960, 540)))
    }
}

impl Default for EnhancedVision {
    fn default() -> Self {
        Self::new()
    }
}

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
    fn test_ui_element_center() {
        let elem = UIElement {
            id: "test".into(),
            element_type: ElementType::Button,
            x: 100,
            y: 100,
            width: 100,
            height: 50,
            text: None,
            confidence: 0.9,
            bounds: Rectangle::new(100, 100, 100, 50),
            is_interactive: true,
            visible: true,
            label: None,
            value: None,
            placeholder: None,
            enabled: true,
            classes: vec![],
            attributes: HashMap::new(),
            metadata: HashMap::new(),
        };
        
        assert_eq!(elem.center(), (150, 125));
    }
    
    #[tokio::test]
    async fn test_vision_creation() {
        let vision = EnhancedVision::new();
        assert!(vision.ocr_enabled);
    }
}
