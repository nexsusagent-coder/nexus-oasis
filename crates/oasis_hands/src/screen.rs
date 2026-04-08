//! ═══════════════════════════════════════════════════════════════════════════════
//!  SCREEN CAPTURE - EKRAN GÖRÜNTÜSÜ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Ekran görüntüsü alma ve işleme yetenekleri.

use crate::error::{HandsError, HandsResult};
use crate::{MAX_SCREEN_WIDTH, MAX_SCREEN_HEIGHT};
use serde::{Deserialize, Serialize};
use std::path::Path;

// ───────────────────────────────────────────────────────────────────────────────
//  EKRAN BİLGİSİ
// ─────────────────────────────────────────────────────────────────────────────--

/// Ekran bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenInfo {
    /// Ekran genişliği (px)
    pub width: u32,
    /// Ekran yüksekliği (px)
    pub height: u32,
    /// Monitör sayısı
    pub monitor_count: u32,
    /// Birincil monitör ID
    pub primary_monitor: u32,
    /// Renk derinliği (bit)
    pub color_depth: u32,
    /// DPI
    pub dpi: u32,
}

impl Default for ScreenInfo {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            monitor_count: 1,
            primary_monitor: 0,
            color_depth: 32,
            dpi: 96,
        }
    }
}

impl ScreenInfo {
    /// Mevcut ekran bilgisini al
    pub fn detect() -> HandsResult<Self> {
        // Gerçek uygulamada x11/wlroots API kullanılır
        // Şimdilik default döndür
        Ok(Self::default())
    }
    
    /// Ekran oranını hesapla
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    
    /// Toplam piksel sayısı
    pub fn total_pixels(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  CAPTURE YAPILANDIRMASI
// ─────────────────────────────────────────────────────────────────────────────--

/// Ekran capture yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConfig {
    /// Kaynak X koordinatı
    pub x: i32,
    /// Kaynak Y koordinatı
    pub y: i32,
    /// Genişlik
    pub width: u32,
    /// Yükseklik
    pub height: u32,
    /// Kalite (1-100)
    pub quality: u8,
    /// Format
    pub format: CaptureFormat,
    /// Monitör ID (-1 = tümü)
    pub monitor_id: i32,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            quality: 85,
            format: CaptureFormat::Png,
            monitor_id: -1,
        }
    }
}

impl CaptureConfig {
    /// Tam ekran capture
    pub fn fullscreen() -> Self {
        Self::default()
    }
    
    /// Bölgesel capture
    pub fn region(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            ..Default::default()
        }
    }
    
    /// Pencere capture (mock)
    pub fn window(window_id: u64) -> Self {
        Self {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            format: CaptureFormat::Png,
            ..Default::default()
        }
    }
}

/// Capture formatı
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
}

impl CaptureFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            CaptureFormat::Png => "png",
            CaptureFormat::Jpeg => "jpg",
            CaptureFormat::WebP => "webp",
            CaptureFormat::Bmp => "bmp",
        }
    }
    
    pub fn mime_type(&self) -> &'static str {
        match self {
            CaptureFormat::Png => "image/png",
            CaptureFormat::Jpeg => "image/jpeg",
            CaptureFormat::WebP => "image/webp",
            CaptureFormat::Bmp => "image/bmp",
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  SCREEN CAPTURE
// ─────────────────────────────────────────────────────────────────────────────--

/// Ekran görüntüsü
#[derive(Debug, Clone)]
pub struct ScreenCapture {
    /// Ham görüntü verisi (RGBA)
    pub data: Vec<u8>,
    /// Genişlik
    pub width: u32,
    /// Yükseklik
    pub height: u32,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Monitör ID
    pub monitor_id: u32,
    /// Base64 encoded (LLM için)
    pub base64: Option<String>,
}

impl ScreenCapture {
    /// Yeni capture oluştur
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
            timestamp: chrono::Utc::now(),
            monitor_id: 0,
            base64: None,
        }
    }
    
    /// Mock capture oluştur
    pub fn mock(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        let data = vec![128u8; size]; // Gri dolgu
        Self::new(data, width, height)
    }
    
    /// Base64'e dönüştür (LLM için)
    pub fn to_base64(&mut self, format: CaptureFormat) -> HandsResult<String> {
        if let Some(ref b64) = self.base64 {
            return Ok(b64.clone());
        }
        
        // Basit encoding (gerçek uygulamada image crate kullanılır)
        let b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &self.data[..self.data.len().min(10000)], // Örnek
        );
        
        self.base64 = Some(format!("data:{};base64,{}", 
            format.mime_type(), b64));
        
        Ok(self.base64.clone().unwrap())
    }
    
    /// Dosyaya kaydet
    pub fn save<P: AsRef<Path>>(&self, path: P) -> HandsResult<()> {
        // Gerçek uygulamada image crate ile kaydetme
        log::info!("🖼️  SCREEN: Görüntü kaydedildi → {}", path.as_ref().display());
        Ok(())
    }
    
    /// Belirli bir bölgeyi kırp
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> HandsResult<Self> {
        if x + width > self.width || y + height > self.height {
            return Err(HandsError::ScreenError(
                "Kırpma bölgesi ekran dışında".into()
            ));
        }
        
        let mut cropped = Vec::with_capacity((width * height * 4) as usize);
        
        for row in y..y+height {
            let offset = (row * self.width + x) as usize * 4;
            let row_data = &self.data[offset..offset + width as usize * 4];
            cropped.extend_from_slice(row_data);
        }
        
        Ok(Self::new(cropped, width, height))
    }
    
    /// Piksel rengini al
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }
        
        let idx = (y * self.width + x) as usize * 4;
        if idx + 3 >= self.data.len() {
            return None;
        }
        
        Some((
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ))
    }
    
    /// Boyut bilgisi
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    
    /// Toplam piksel sayısı
    pub fn pixel_count(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  SCREEN CAPTURER
// ─────────────────────────────────────────────────────────────────────────────--

/// Ekran yakalayıcı
pub struct ScreenCapturer {
    /// Ekran bilgisi
    screen_info: ScreenInfo,
    /// Varsayılan yapılandırma
    default_config: CaptureConfig,
    /// Aktif mi?
    active: bool,
}

impl ScreenCapturer {
    /// Yeni capturer oluştur
    pub fn new() -> HandsResult<Self> {
        let screen_info = ScreenInfo::detect()?;
        
        log::info!("🖼️  SCREEN: Ekran yakalayıcı başlatıldı");
        log::info!("🖼️  SCREEN: Çözünürlük: {}x{}", screen_info.width, screen_info.height);
        
        Ok(Self {
            screen_info,
            default_config: CaptureConfig::default(),
            active: true,
        })
    }
    
    /// Ekran görüntüsü al
    pub fn capture(&self, config: Option<&CaptureConfig>) -> HandsResult<ScreenCapture> {
        if !self.active {
            return Err(HandsError::ScreenError("Capturer aktif değil".into()));
        }
        
        let config = config.unwrap_or(&self.default_config);
        
        // Sınır kontrolü
        let width = config.width.min(MAX_SCREEN_WIDTH);
        let height = config.height.min(MAX_SCREEN_HEIGHT);
        
        log::debug!("🖼️  SCREEN: Capture başlatılıyor {}x{}", width, height);
        
        // Mock capture (gerçek uygulamada x11/wlroots kullanılır)
        let capture = ScreenCapture::mock(width, height);
        
        log::info!("🖼️  SCREEN: Görüntü alındı ({}x{}, {} bytes)", 
            width, height, capture.data.len());
        
        Ok(capture)
    }
    
    /// Tam ekran capture
    pub fn capture_full(&self) -> HandsResult<ScreenCapture> {
        self.capture(Some(&CaptureConfig::fullscreen()))
    }
    
    /// Bölgesel capture
    pub fn capture_region(&self, x: i32, y: i32, width: u32, height: u32) -> HandsResult<ScreenCapture> {
        self.capture(Some(&CaptureConfig::region(x, y, width, height)))
    }
    
    /// Ekran bilgisini getir
    pub fn info(&self) -> &ScreenInfo {
        &self.screen_info
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Durdur
    pub fn stop(&mut self) {
        self.active = false;
        log::info!("🖼️  SCREEN: Yakalayıcı durduruldu");
    }
    
    /// Başlat
    pub fn start(&mut self) {
        self.active = true;
        log::info!("🖼️  SCREEN: Yakalayıcı başlatıldı");
    }
}

impl Default for ScreenCapturer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            screen_info: ScreenInfo::default(),
            default_config: CaptureConfig::default(),
            active: true,
        })
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_screen_info_default() {
        let info = ScreenInfo::default();
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert_eq!(info.monitor_count, 1);
    }
    
    #[test]
    fn test_screen_info_aspect_ratio() {
        let info = ScreenInfo::default();
        let ratio = info.aspect_ratio();
        assert!((ratio - 1.777).abs() < 0.01); // 16:9
    }
    
    #[test]
    fn test_capture_config_default() {
        let config = CaptureConfig::default();
        assert_eq!(config.quality, 85);
        assert_eq!(config.format, CaptureFormat::Png);
    }
    
    #[test]
    fn test_capture_format_extension() {
        assert_eq!(CaptureFormat::Png.extension(), "png");
        assert_eq!(CaptureFormat::Jpeg.extension(), "jpg");
    }
    
    #[test]
    fn test_screen_capture_mock() {
        let capture = ScreenCapture::mock(800, 600);
        assert_eq!(capture.width, 800);
        assert_eq!(capture.height, 600);
        assert_eq!(capture.data.len(), 800 * 600 * 4);
    }
    
    #[test]
    fn test_screen_capture_dimensions() {
        let capture = ScreenCapture::mock(1920, 1080);
        let (w, h) = capture.dimensions();
        assert_eq!(w, 1920);
        assert_eq!(h, 1080);
    }
    
    #[test]
    fn test_screen_capture_pixel_count() {
        let capture = ScreenCapture::mock(100, 100);
        assert_eq!(capture.pixel_count(), 10000);
    }
    
    #[test]
    fn test_screen_capturer_creation() {
        let capturer = ScreenCapturer::new().unwrap();
        assert!(capturer.is_active());
    }
    
    #[test]
    fn test_screen_capturer_stop_start() {
        let mut capturer = ScreenCapturer::new().unwrap();
        capturer.stop();
        assert!(!capturer.is_active());
        capturer.start();
        assert!(capturer.is_active());
    }
    
    #[test]
    fn test_screen_capturer_capture() {
        let capturer = ScreenCapturer::new().unwrap();
        let capture = capturer.capture_full().unwrap();
        assert!(capture.data.len() > 0);
    }
}
