//! ═══════════════════════════════════════════════════════════════════════════════
//!  SCREEN UNDERSTANDING - Ekran Anlama Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Ekran yakalama, bölge sınıflandırma, pencere tespiti ve UI analizi.
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        SCREEN LAYOUT                                    │
//! │  ┌───────────────────────────────────────────────────────────────────┐ │
//! │  │                         MENU BAR                                   │ │
//! │  └───────────────────────────────────────────────────────────────────┘ │
//! │  ┌──────────┬────────────────────────────────────────────────────┐    │ │
//! │  │          │                                                    │    │ │
//! │  │ SIDEBAR  │                    CONTENT                         │    │ │
//! │  │          │                                                    │    │ │
//! │  │          │                                                    │    │ │
//! │  ├──────────┴────────────────────────────────────────────────────┤    │ │
//! │  │                        STATUS BAR                              │    │ │
//! │  └────────────────────────────────────────────────────────────────┘    │ │
//! │  ┌───────────────────────────────────────────────────────────────────┐ │
//! │  │                         TASKBAR / DOCK                            │ │
//! │  └───────────────────────────────────────────────────────────────────┘ │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousResult};
use crate::{Observation};
use crate::vision::UIElement as VisionUIElement;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  SCREEN REGION
// ═══════════════════════════════════════════════════════════════════════════════

/// Ekran bölgesi türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenRegion {
    /// Üst menü çubuğu
    MenuBar,
    /// Araç çubuğu
    Toolbar,
    /// Yan panel
    Sidebar,
    /// Ana içerik alanı
    Content,
    /// Durum çubuğu
    StatusBar,
    /// Açılır pencere/dialog
    Dialog,
    /// Bildirim alanı
    Notification,
    /// Alt görev çubuğu/dock
    Taskbar,
    /// Dock (macOS)
    Dock,
    /// Sistem tepsisi
    SystemTray,
    /// Başlık çubuğu
    TitleBar,
    /// Sekme çubuğu
    TabBar,
    /// Bilinmeyen
    Unknown,
}

impl ScreenRegion {
    /// Bölge tıklanabilir mi?
    pub fn is_clickable(&self) -> bool {
        matches!(self,
            ScreenRegion::MenuBar |
            ScreenRegion::Toolbar |
            ScreenRegion::Sidebar |
            ScreenRegion::Content |
            ScreenRegion::Dialog |
            ScreenRegion::Taskbar |
            ScreenRegion::Dock |
            ScreenRegion::TabBar
        )
    }
    
    /// Bölge interaktif mi?
    pub fn is_interactive(&self) -> bool {
        !matches!(self, ScreenRegion::StatusBar | ScreenRegion::Unknown)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WINDOW INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Pencere bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    /// Pencere ID
    pub id: u64,
    /// Pencere başlığı
    pub title: String,
    /// Uygulama adı
    pub app_name: String,
    /// Pencere sınıfı
    pub class: Option<String>,
    /// Pencere konumu ve boyutu
    pub bounds: Rectangle,
    /// Aktif mi?
    pub is_active: bool,
    /// Focus var mı?
    pub has_focus: bool,
    /// Minimize mi?
    pub is_minimized: bool,
    /// Maximize mi?
    pub is_maximized: bool,
    /// Z-order (üstten kaçinci)
    pub z_order: u32,
    /// Pencere tipi
    pub window_type: WindowType,
}

impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            app_name: String::new(),
            class: None,
            bounds: Rectangle::default(),
            is_active: false,
            has_focus: false,
            is_minimized: false,
            is_maximized: false,
            z_order: 0,
            window_type: WindowType::Normal,
        }
    }
}

/// Pencere türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowType {
    /// Normal pencere
    Normal,
    /// Dialog
    Dialog,
    /// Popup
    Popup,
    /// Tooltip
    Tooltip,
    /// Notification
    Notification,
    /// Splash screen
    Splash,
    /// Menu
    Menu,
    /// Dock/Panel
    Panel,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RECTANGLE
// ═══════════════════════════════════════════════════════════════════════════════

/// Dikdörtgen bölge
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        }
    }
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            x: x1.min(x2),
            y: y1.min(y2),
            width: (x2 - x1).abs() as u32,
            height: (y2 - y1).abs() as u32,
        }
    }
    
    /// Merkez noktası
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width as i32 / 2, self.y + self.height as i32 / 2)
    }
    
    /// Nokta içeride mi?
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width as i32 &&
        y >= self.y && y < self.y + self.height as i32
    }
    
    /// Başka bir dikdörtgenle kesişiyor mu?
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width as i32 &&
        self.x + self.width as i32 > other.x &&
        self.y < other.y + other.height as i32 &&
        self.y + self.height as i32 > other.y
    }
    
    /// Alan
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCREEN ANALYSIS RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Ekran analiz sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenAnalysis {
    /// Ekran boyutu
    pub screen_size: (u32, u32),
    /// Tespit edilen bölgeler
    pub regions: Vec<RegionInfo>,
    /// Aktif pencere
    pub active_window: Option<WindowInfo>,
    /// Tüm pencereler
    pub windows: Vec<WindowInfo>,
    /// UI elementler
    pub elements: Vec<VisionUIElement>,
    /// Metin içeriği
    pub text_content: String,
    /// Fare pozisyonu
    pub mouse_position: (i32, i32),
    /// Desktop environment
    pub desktop_env: DesktopEnvironment,
}

/// Bölge bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    /// Bölge türü
    pub region_type: ScreenRegion,
    /// Bölge sınırları
    pub bounds: Rectangle,
    /// Güven skoru
    pub confidence: f32,
    /// Bölge içeriği özeti
    pub content_summary: String,
}

/// Desktop environment türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    Macos,
    Windows,
    I3,
    Sway,
    Unknown,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCREEN UNDERSTANDING ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Ekran anlama motoru
pub struct ScreenUnderstanding {
    /// Ekran boyutu (cache)
    screen_size: (u32, u32),
    /// Son ekran görüntüsü
    last_capture: Option<Vec<u8>>,
    /// Son analiz
    last_analysis: Option<ScreenAnalysis>,
    /// Desktop environment detection
    desktop_detection: DesktopDetector,
    /// Window manager
    window_manager: WindowManager,
    /// Region classifier
    region_classifier: RegionClassifier,
}

impl ScreenUnderstanding {
    /// Yeni motor oluştur
    pub fn new() -> Self {
        log::info!("🖥️ SCREEN: Ekran anlama motoru başlatılıyor...");
        
        Self {
            screen_size: (1920, 1080),
            last_capture: None,
            last_analysis: None,
            desktop_detection: DesktopDetector::new(),
            window_manager: WindowManager::new(),
            region_classifier: RegionClassifier::new(),
        }
    }
    
    /// Ekranı yakala ve analiz et
    pub async fn capture_and_analyze(&mut self) -> AutonomousResult<Observation> {
        log::debug!("🖥️ SCREEN: Ekran yakalanıyor...");
        
        // 1. Ekran yakala
        let capture = self.capture_screen().await?;
        
        // 2. Desktop environment tespit et
        let desktop_env = self.desktop_detection.detect();
        
        // 3. Pencereleri tespit et
        let windows = self.window_manager.list_windows().await?;
        let active_window = windows.iter().find(|w| w.is_active).cloned();
        
        // 4. Bölgeleri sınıflandır
        let regions = self.region_classifier.classify(&capture, &windows);
        
        // 5. UI elementleri tespit et
        let elements = self.detect_ui_elements(&capture).await?;
        
        // 6. OCR ile metin çıkar
        let text_content = self.extract_text(&capture).await?;
        
        // 7. Fare pozisyonunu al
        let mouse_position = self.get_mouse_position().await?;
        
        // 8. Screenshot'ı base64'e çevir
        let screenshot_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &capture
        );
        
        // Sonucu oluştur
        let analysis = ScreenAnalysis {
            screen_size: self.screen_size,
            regions,
            active_window,
            windows,
            elements: elements.clone(),
            text_content: text_content.clone(),
            mouse_position,
            desktop_env,
        };
        
        self.last_analysis = Some(analysis.clone());
        
        Ok(Observation {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            screenshot: Some(screenshot_b64),
            screen_size: self.screen_size,
            elements,
            text_content,
            mouse_position,
        })
    }
    
    /// Ekran yakala
    async fn capture_screen(&mut self) -> AutonomousResult<Vec<u8>> {
        log::debug!("🖥️ SCREEN: Screenshot alınıyor...");
        
        // Try native capture first
        if let Some((bytes, width, height)) = native_capture::capture_screen() {
            self.screen_size = (width, height);
            self.last_capture = Some(bytes.clone());
            log::info!("🖥️ SCREEN: Screenshot alındı {}x{}", width, height);
            return Ok(bytes);
        }
        
        // Fallback: Mock capture
        log::warn!("🖥️ SCREEN: Native capture kullanılamıyor, mock kullanılıyor");
        let capture = vec![0u8; (self.screen_size.0 * self.screen_size.1 * 4) as usize];
        self.last_capture = Some(capture.clone());
        Ok(capture)
    }
    
    /// UI elementlerini tespit et
    async fn detect_ui_elements(&self, _capture: &[u8]) -> AutonomousResult<Vec<VisionUIElement>> {
        // Try to use native window info if available
        if let Some((title, app_name, x, y, w, h)) = native_window::get_active_window_info() {
            log::debug!("🖥️ SCREEN: Active window: {} ({})", title, app_name);
            
            // Return window-based elements
            return Ok(vec![
                VisionUIElement {
                    id: "active-window".into(),
                    element_type: crate::vision::ElementType::Window,
                    x,
                    y,
                    width: w,
                    height: h,
                    text: Some(title.clone()),
                    confidence: 0.95,
                    bounds: Rectangle::new(x, y, w, h),
                    is_interactive: true,
                    visible: true,
                    label: None,
                    value: None,
                    placeholder: None,
                    enabled: true,
                    classes: vec![],
                    attributes: std::collections::HashMap::new(),
                    metadata: std::collections::HashMap::new(),
                },
            ]);
        }
        
        // Fallback: Mock elements for testing
        log::debug!("🖥️ SCREEN: Native window detection kullanılamıyor, mock elements");
        Ok(vec![
            VisionUIElement {
                id: "elem-1".into(),
                element_type: crate::vision::ElementType::Button,
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
                attributes: std::collections::HashMap::new(),
                metadata: std::collections::HashMap::new(),
            },
            VisionUIElement {
                id: "elem-2".into(),
                element_type: crate::vision::ElementType::Input,
                x: 250,
                y: 100,
                width: 300,
                height: 35,
                text: None,
                confidence: 0.88,
                bounds: Rectangle::new(250, 100, 300, 35),
                is_interactive: true,
                visible: true,
                label: None,
                value: None,
                placeholder: None,
                enabled: true,
                classes: vec![],
                attributes: std::collections::HashMap::new(),
                metadata: std::collections::HashMap::new(),
            },
        ])
    }
    
    /// OCR ile metin çıkar
    async fn extract_text(&self, capture: &[u8]) -> AutonomousResult<String> {
        // Try native OCR if available
        if let Some(text) = native_ocr::extract_text(capture, self.screen_size.0, self.screen_size.1) {
            log::debug!("🖥️ SCREEN: OCR ile {} karakter çıkarıldı", text.len());
            return Ok(text);
        }
        
        // Fallback
        Ok("Örnek metin içeriği".into())
    }
    
    /// Fare pozisyonunu al
    async fn get_mouse_position(&self) -> AutonomousResult<(i32, i32)> {
        // Try native input control if available
        if let Some(pos) = native_input::get_mouse_position() {
            return Ok(pos);
        }
        
        // Fallback: Screen center (mock)
        Ok((self.screen_size.0 as i32 / 2, self.screen_size.1 as i32 / 2))
    }
    
    /// Son analizi al
    pub fn last_analysis(&self) -> Option<&ScreenAnalysis> {
        self.last_analysis.as_ref()
    }
    
    /// Ekran boyutunu al
    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }
}

impl Default for ScreenUnderstanding {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DESKTOP DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Desktop environment tespitçisi
struct DesktopDetector {
    detected: Option<DesktopEnvironment>,
}

impl DesktopDetector {
    fn new() -> Self {
        Self { detected: None }
    }
    
    fn detect(&mut self) -> DesktopEnvironment {
        if let Some(detected) = self.detected {
            return detected;
        }
        
        // Real detection using environment variables
        #[cfg(target_os = "linux")]
        {
            use std::env;
            
            // Check XDG_CURRENT_DESKTOP
            if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
                let desktop_lower = desktop.to_lowercase();
                self.detected = Some(if desktop_lower.contains("gnome") {
                    DesktopEnvironment::Gnome
                } else if desktop_lower.contains("kde") {
                    DesktopEnvironment::Kde
                } else if desktop_lower.contains("xfce") {
                    DesktopEnvironment::Xfce
                } else if desktop_lower.contains("i3") {
                    DesktopEnvironment::I3
                } else if desktop_lower.contains("sway") {
                    DesktopEnvironment::Sway
                } else {
                    DesktopEnvironment::Unknown
                });
            }
            
            // Check DESSKTOP_SESSION as fallback
            if self.detected.is_none() {
                if let Ok(session) = env::var("DESKTOP_SESSION") {
                    let session_lower = session.to_lowercase();
                    self.detected = Some(if session_lower.contains("gnome") {
                        DesktopEnvironment::Gnome
                    } else if session_lower.contains("kde") || session_lower.contains("plasma") {
                        DesktopEnvironment::Kde
                    } else if session_lower.contains("xfce") {
                        DesktopEnvironment::Xfce
                    } else if session_lower.contains("i3") {
                        DesktopEnvironment::I3
                    } else if session_lower.contains("sway") {
                        DesktopEnvironment::Sway
                    } else {
                        DesktopEnvironment::Unknown
                    });
                }
            }
            
            if self.detected.is_none() {
                self.detected = Some(DesktopEnvironment::Unknown);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            self.detected = Some(DesktopEnvironment::Macos);
        }
        
        #[cfg(target_os = "windows")]
        {
            self.detected = Some(DesktopEnvironment::Windows);
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            self.detected = Some(DesktopEnvironment::Unknown);
        }
        
        self.detected.unwrap_or(DesktopEnvironment::Unknown)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WINDOW MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Pencere yöneticisi arayüzü
struct WindowManager {
    windows: Vec<WindowInfo>,
}

impl WindowManager {
    fn new() -> Self {
        Self { windows: vec![] }
    }
    
    async fn list_windows(&mut self) -> AutonomousResult<Vec<WindowInfo>> {
        // Try native active window detection first
        if let Some((title, app_name, x, y, w, h)) = native_window::get_active_window_info() {
            self.windows = vec![
                WindowInfo {
                    id: 1,
                    title,
                    app_name,
                    class: None,
                    bounds: Rectangle::new(x, y, w, h),
                    is_active: true,
                    has_focus: true,
                    is_minimized: false,
                    is_maximized: false,
                    z_order: 0,
                    window_type: WindowType::Normal,
                },
            ];
            return Ok(self.windows.clone());
        }
        
        // Fallback: Mock windows
        log::debug!("🖥️ SCREEN: Native window listing not available, using mock");
        self.windows = vec![
            WindowInfo {
                id: 1,
                title: "Terminal".into(),
                app_name: "gnome-terminal".into(),
                class: Some("Gnome-terminal".into()),
                bounds: Rectangle::new(0, 0, 960, 1080),
                is_active: true,
                has_focus: true,
                is_minimized: false,
                is_maximized: false,
                z_order: 0,
                window_type: WindowType::Normal,
            },
            WindowInfo {
                id: 2,
                title: "Firefox".into(),
                app_name: "firefox".into(),
                class: Some("Firefox".into()),
                bounds: Rectangle::new(960, 0, 960, 1080),
                is_active: false,
                has_focus: false,
                is_minimized: false,
                is_maximized: false,
                z_order: 1,
                window_type: WindowType::Normal,
            },
        ];
        
        Ok(self.windows.clone())
    }
    
    async fn get_active_window(&self) -> AutonomousResult<Option<WindowInfo>> {
        // Try native first
        if let Some((title, app_name, x, y, w, h)) = native_window::get_active_window_info() {
            return Ok(Some(WindowInfo {
                id: 1,
                title,
                app_name,
                class: None,
                bounds: Rectangle::new(x, y, w, h),
                is_active: true,
                has_focus: true,
                is_minimized: false,
                is_maximized: false,
                z_order: 0,
                window_type: WindowType::Normal,
            }));
        }
        
        Ok(self.windows.iter().find(|w| w.is_active).cloned())
    }
    
    async fn focus_window(&mut self, window_id: u64) -> AutonomousResult<()> {
        // Update internal state
        for window in &mut self.windows {
            window.is_active = window.id == window_id;
            window.has_focus = window.id == window_id;
        }
        
        // Note: Real focus requires window manager interaction (xdotool, wmctrl, etc.)
        // This would need additional implementation
        log::debug!("🖥️ SCREEN: Focus window {} requested", window_id);
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REGION CLASSIFIER
// ═══════════════════════════════════════════════════════════════════════════════

/// Bölge sınıflandırıcı
struct RegionClassifier {
    rules: Vec<RegionRule>,
}

/// Bölge sınıflandırma kuralı
struct RegionRule {
    region_type: ScreenRegion,
    bounds_hint: Option<Rectangle>,
    keywords: Vec<String>,
}

impl RegionClassifier {
    fn new() -> Self {
        Self {
            rules: vec![
                RegionRule {
                    region_type: ScreenRegion::MenuBar,
                    bounds_hint: Some(Rectangle::new(0, 0, 1920, 25)),
                    keywords: vec!["File".into(), "Edit".into(), "View".into()],
                },
                RegionRule {
                    region_type: ScreenRegion::Taskbar,
                    bounds_hint: Some(Rectangle::new(0, 1050, 1920, 30)),
                    keywords: vec![],
                },
            ],
        }
    }
    
    fn classify(&self, _capture: &[u8], windows: &[WindowInfo]) -> Vec<RegionInfo> {
        let mut regions = Vec::new();
        
        // Aktif pencere varsa, bölgeleri tahmin et
        if let Some(active) = windows.iter().find(|w| w.is_active) {
            let bounds = active.bounds;
            
            // Title bar
            regions.push(RegionInfo {
                region_type: ScreenRegion::TitleBar,
                bounds: Rectangle::new(bounds.x, bounds.y, bounds.width, 30),
                confidence: 0.9,
                content_summary: active.title.clone(),
            });
            
            // Content area (tahmin)
            regions.push(RegionInfo {
                region_type: ScreenRegion::Content,
                bounds: Rectangle::new(
                    bounds.x + 10,
                    bounds.y + 40,
                    bounds.width - 20,
                    bounds.height - 80,
                ),
                confidence: 0.7,
                content_summary: "Main content area".into(),
            });
        }
        
        // Global regions
        regions.push(RegionInfo {
            region_type: ScreenRegion::Taskbar,
            bounds: Rectangle::new(0, 1050, 1920, 30),
            confidence: 0.95,
            content_summary: "System taskbar".into(),
        });
        
        regions
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  NATIVE SCREEN CAPTURE MODULE
// ═══════════════════════════════════════════════════════════════════════════════

/// Native screen capture implementation
#[cfg(feature = "screenshots")]
mod native_capture {
    use screenshots::Screen;
    use image::ImageBuffer;
    use image::Rgba;
    
    /// Capture screen using screenshots crate
    pub fn capture_screen() -> Option<(Vec<u8>, u32, u32)> {
        let screens = Screen::all().ok()?;
        let screen = screens.first()?;
        
        let image = screen.capture().ok()?;
        let (width, height) = (image.width(), image.height());
        
        // Convert to raw RGBA bytes
        let bytes = image.into_raw();
        
        Some((bytes, width, height))
    }
    
    /// Capture specific region
    pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Option<Vec<u8>> {
        let screens = Screen::all().ok()?;
        let screen = screens.first()?;
        
        let image = screen.capture_area(x, y, width, height).ok()?;
        Some(image.into_raw())
    }
    
    /// Get screen dimensions
    pub fn get_screen_size() -> Option<(u32, u32)> {
        let screens = Screen::all().ok()?;
        let screen = screens.first()?;
        let display_info = screen.display_info;
        Some((display_info.width as u32, display_info.height as u32))
    }
}

/// Fallback when screenshots feature is disabled
#[cfg(not(feature = "screenshots"))]
mod native_capture {
    pub fn capture_screen() -> Option<(Vec<u8>, u32, u32)> {
        log::warn!("Screen capture not available - enable 'screenshots' feature");
        None
    }
    
    pub fn capture_region(_x: i32, _y: i32, _width: u32, _height: u32) -> Option<Vec<u8>> {
        None
    }
    
    pub fn get_screen_size() -> Option<(u32, u32)> {
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  NATIVE INPUT CONTROL MODULE
// ═══════════════════════════════════════════════════════════════════════════════

/// Native mouse/keyboard control
#[cfg(all(feature = "enigo", not(target_os = "macos")))]
mod native_input {
    use enigo::{Enigo, MouseControllable, KeyboardControllable, Key, Button};
    use std::sync::Mutex;
    
    static ENIGO: Mutex<Option<Enigo>> = Mutex::new(None);
    
    fn get_enigo() -> Option<Enigo> {
        Enigo::new().ok()
    }
    
    /// Move mouse to position
    pub fn move_mouse(x: i32, y: i32) -> bool {
        if let Ok(mut enigo) = Enigo::new() {
            enigo.mouse_move_to(x, y);
            return true;
        }
        false
    }
    
    /// Click at position
    pub fn click(x: i32, y: i32) -> bool {
        if let Ok(mut enigo) = Enigo::new() {
            enigo.mouse_move_to(x, y);
            enigo.mouse_click(Button::Left);
            return true;
        }
        false
    }
    
    /// Type text
    pub fn type_text(text: &str) -> bool {
        if let Ok(mut enigo) = Enigo::new() {
            enigo.text(text);
            return true;
        }
        false
    }
    
    /// Press key
    pub fn press_key(key: &str) -> bool {
        if let Ok(mut enigo) = Enigo::new() {
            let enigo_key = match key.to_lowercase().as_str() {
                "enter" => Key::Return,
                "tab" => Key::Tab,
                "escape" | "esc" => Key::Escape,
                "backspace" => Key::Backspace,
                "space" => Key::Space,
                "up" => Key::UpArrow,
                "down" => Key::DownArrow,
                "left" => Key::LeftArrow,
                "right" => Key::RightArrow,
                _ => return false,
            };
            enigo.key_click(enigo_key);
            return true;
        }
        false
    }
    
    /// Get mouse position (using rdev if available)
    pub fn get_mouse_position() -> Option<(i32, i32)> {
        // enigo doesn't support getting position in newer versions
        // This would need rdev or x11rb integration
        None
    }
}

/// macOS enigo has different API
#[cfg(all(feature = "enigo", target_os = "macos"))]
mod native_input {
    pub fn get_mouse_position() -> Option<(i32, i32)> { None }
    pub fn move_mouse(_x: i32, _y: i32) -> bool { false }
    pub fn click(_x: i32, _y: i32) -> bool { false }
    pub fn type_text(_text: &str) -> bool { false }
    pub fn press_key(_key: &str) -> bool { false }
}

/// Fallback when enigo is disabled
#[cfg(not(feature = "enigo"))]
mod native_input {
    pub fn get_mouse_position() -> Option<(i32, i32)> {
        log::warn!("Input control not available - enable 'enigo' feature");
        None
    }
    pub fn move_mouse(_x: i32, _y: i32) -> bool { false }
    pub fn click(_x: i32, _y: i32) -> bool { false }
    pub fn type_text(_text: &str) -> bool { false }
    pub fn press_key(_key: &str) -> bool { false }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  NATIVE ACTIVE WINDOW MODULE
// ═══════════════════════════════════════════════════════════════════════════════

/// Native active window detection
#[cfg(feature = "active-win-pos-rs")]
mod native_window {
    use active_win_pos_rs::get_active_window;
    
    /// Get active window info
    pub fn get_active_window_info() -> Option<(String, String, i32, i32, u32, u32)> {
        let window = get_active_window().ok()??;
        Some((
            window.title,
            window.process_name,
            window.position.x,
            window.position.y,
            window.position.width,
            window.position.height,
        ))
    }
}

/// Fallback when active-win-pos-rs is disabled
#[cfg(not(feature = "active-win-pos-rs"))]
mod native_window {
    pub fn get_active_window_info() -> Option<(String, String, i32, i32, u32, u32)> {
        log::debug!("Active window detection not available - enable 'active-window' feature");
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  NATIVE OCR MODULE
// ═══════════════════════════════════════════════════════════════════════════════

/// Native OCR using Tesseract
#[cfg(feature = "tesseract")]
mod native_ocr {
    use tesseract::Tesseract;
    
    /// Extract text from image bytes
    pub fn extract_text(image_bytes: &[u8], width: u32, height: u32) -> Option<String> {
        let mut tess = Tesseract::new(None, Some("eng")).ok()?;
        tess = tess.set_image_from_mem(image_bytes, width, height, 3).ok()?;
        tess.get_text().ok()
    }
}

/// Fallback when tesseract is disabled
#[cfg(not(feature = "tesseract"))]
mod native_ocr {
    pub fn extract_text(_image_bytes: &[u8], _width: u32, _height: u32) -> Option<String> {
        log::debug!("OCR not available - enable 'ocr' feature and install tesseract");
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_screen_region_clickable() {
        assert!(ScreenRegion::MenuBar.is_clickable());
        assert!(ScreenRegion::Content.is_clickable());
        assert!(!ScreenRegion::StatusBar.is_clickable());
    }
    
    #[test]
    fn test_rectangle_center() {
        let rect = Rectangle::new(100, 100, 200, 100);
        assert_eq!(rect.center(), (200, 150));
    }
    
    #[test]
    fn test_rectangle_contains() {
        let rect = Rectangle::new(0, 0, 100, 100);
        assert!(rect.contains(50, 50));
        assert!(!rect.contains(150, 150));
    }
    
    #[test]
    fn test_rectangle_intersects() {
        let rect1 = Rectangle::new(0, 0, 100, 100);
        let rect2 = Rectangle::new(50, 50, 100, 100);
        assert!(rect1.intersects(&rect2));
        
        let rect3 = Rectangle::new(200, 200, 100, 100);
        assert!(!rect1.intersects(&rect3));
    }
    
    #[tokio::test]
    async fn test_screen_understanding_creation() {
        let screen = ScreenUnderstanding::new();
        assert_eq!(screen.screen_size(), (1920, 1080));
    }
    
    #[tokio::test]
    async fn test_capture_and_analyze() {
        let mut screen = ScreenUnderstanding::new();
        let obs = screen.capture_and_analyze().await.expect("operation failed");
        
        assert!(!obs.id.is_empty());
        assert_eq!(obs.screen_size, (1920, 1080));
    }
}
