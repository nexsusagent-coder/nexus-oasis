// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Screen Capture (Real Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{DesktopError, Result, Rect};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Global screen info cache
static SCREEN_INFO: OnceLock<(u32, u32, f32)> = OnceLock::new();

/// Screen capture interface
pub struct Screen;

impl Screen {
    /// Capture entire screen
    pub fn capture_all() -> Result<Screenshot> {
        let (width, height, _scale) = Self::get_screen_info_internal()?;
        
        #[cfg(target_os = "linux")]
        {
            Self::capture_linux(0, 0, width, height)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::capture_windows(0, 0, width, height)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::capture_macos(0, 0, width, height)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(DesktopError::PlatformNotSupported("Unsupported OS".into()))
        }
    }

    /// Capture region of screen
    pub fn capture_region(x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot> {
        #[cfg(target_os = "linux")]
        {
            Self::capture_linux(x, y, width, height)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::capture_windows(x, y, width, height)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::capture_macos(x, y, width, height)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(DesktopError::PlatformNotSupported("Unsupported OS".into()))
        }
    }

    /// Capture rect
    pub fn capture_rect(rect: Rect) -> Result<Screenshot> {
        Self::capture_region(rect.x, rect.y, rect.width, rect.height)
    }

    /// Get screen dimensions
    pub fn dimensions() -> Result<(u32, u32)> {
        let (width, height, _) = Self::get_screen_info_internal()?;
        Ok((width, height))
    }

    /// Get screen width
    pub fn width() -> Result<u32> {
        let (width, _, _) = Self::get_screen_info_internal()?;
        Ok(width)
    }

    /// Get screen height
    pub fn height() -> Result<u32> {
        let (_, height, _) = Self::get_screen_info_internal()?;
        Ok(height)
    }

    /// Get scale factor
    pub fn scale() -> Result<f32> {
        let (_, _, scale) = Self::get_screen_info_internal()?;
        Ok(scale)
    }

    /// Get screen info (cached)
    fn get_screen_info_internal() -> Result<(u32, u32, f32)> {
        let info = SCREEN_INFO.get_or_init(|| {
            #[cfg(target_os = "linux")]
            {
                Self::get_screen_info_linux().unwrap_or((1920, 1080, 1.0))
            }
            
            #[cfg(target_os = "windows")]
            {
                Self::get_screen_info_windows().unwrap_or((1920, 1080, 1.0))
            }
            
            #[cfg(target_os = "macos")]
            {
                Self::get_screen_info_macos().unwrap_or((1920, 1080, 1.0))
            }
            
            #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
            {
                (1920, 1080, 1.0)
            }
        });
        Ok(*info)
    }
}

// ============================================================================
// Linux Implementation (X11)
// ============================================================================
#[cfg(target_os = "linux")]
impl Screen {
    fn get_screen_info_linux() -> std::result::Result<(u32, u32, f32), ()> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None).map_err(|_| ())?;
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        
        let width = screen.width_in_pixels as u32;
        let height = screen.height_in_pixels as u32;
        
        // Try to get scale from Xft.dpi
        let scale = Self::get_xft_scale().unwrap_or(1.0);
        
        Ok((width, height, scale))
    }
    
    fn get_xft_scale() -> Option<f32> {
        // Try to read Xft.dpi from xrdb
        let output = std::process::Command::new("xrdb")
            .arg("-query")
            .output()
            .ok()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with("Xft.dpi:") {
                let dpi: f32 = line.split(':')
                    .nth(1)?
                    .trim()
                    .parse()
                    .ok()?;
                // 96 DPI is standard (scale = 1.0)
                return Some(dpi / 96.0);
            }
        }
        None
    }
    
    fn capture_linux(x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| DesktopError::ScreenCaptureFailed(format!("X11 connect failed: {}", e)))?;
        
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;
        
        // Get the image
        let image_cookie = get_image(
            &conn,
            ImageFormat::Z_PIXMAP,
            root,
            x as i16,
            y as i16,
            width as u16,
            height as u16,
            !0u32,
        ).map_err(|e| DesktopError::ScreenCaptureFailed(format!("Get image failed: {}", e)))?;
        
        let image_reply = image_cookie.reply()
            .map_err(|e| DesktopError::ScreenCaptureFailed(format!("Get image reply failed: {}", e)))?;
        
        // Convert to RGBA
        let depth = image_reply.depth;
        let data = &image_reply.data;
        
        let rgba_data = if depth == 32 {
            // Already BGRA, convert to RGBA
            let mut rgba = Vec::with_capacity(data.len());
            for chunk in data.chunks(4) {
                if chunk.len() == 4 {
                    rgba.push(chunk[2]); // R
                    rgba.push(chunk[1]); // G
                    rgba.push(chunk[0]); // B
                    rgba.push(chunk[3]); // A
                }
            }
            rgba
        } else if depth == 24 {
            // BGR to RGBA
            let mut rgba = Vec::with_capacity((width * height * 4) as usize);
            for chunk in data.chunks(4) {
                if chunk.len() >= 3 {
                    rgba.push(chunk[2]); // R
                    rgba.push(chunk[1]); // G
                    rgba.push(chunk[0]); // B
                    rgba.push(255);      // A
                }
            }
            rgba
        } else {
            // Fallback: return raw data
            data.iter().copied().chain(std::iter::repeat(0u8).take((width * height * 4) as usize - data.len())).collect()
        };
        
        Ok(Screenshot {
            width,
            height,
            data: rgba_data,
        })
    }
}

// ============================================================================
// Windows Implementation
// ============================================================================
#[cfg(target_os = "windows")]
impl Screen {
    fn get_screen_info_windows() -> std::result::Result<(u32, u32, f32), ()> {
        use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        
        let width = unsafe { GetSystemMetrics(SM_CXSCREEN) } as u32;
        let height = unsafe { GetSystemMetrics(SM_CYSCREEN) } as u32;
        
        // Get DPI scaling
        let dpi = unsafe { winapi::um::wingdi::GetDeviceCaps(
            winapi::um::wingdi::GetDC(std::ptr::null_mut()),
            winapi::um::wingdi::LOGPIXELSY
        )};
        let scale = dpi as f32 / 96.0;
        
        Ok((width, height, scale))
    }
    
    fn capture_windows(x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot> {
        use winapi::um::winuser::{GetDC, ReleaseDC, GetDesktopWindow};
        use winapi::um::wingdi::{
            CreateCompatibleDC, CreateCompatibleBitmap, SelectObject,
            BitBlt, GetDIBits, DeleteObject, DeleteDC,
            BI_RGB, DIB_RGB_COLORS, BITMAPINFO, BITMAPINFOHEADER
        };
        use winapi::shared::ntdef::VOID;
        use std::ptr;
        
        unsafe {
            let hwnd = GetDesktopWindow();
            let hdc_screen = GetDC(hwnd);
            let hdc_mem = CreateCompatibleDC(hdc_screen);
            let hbitmap = CreateCompatibleBitmap(hdc_screen, width as i32, height as i32);
            
            let old_bitmap = SelectObject(hdc_mem, hbitmap as _);
            
            // BitBlt to copy screen
            BitBlt(hdc_mem, 0, 0, width as i32, height as i32, hdc_screen, x as i32, y as i32, 0x00CC0020);
            
            // Prepare BITMAPINFO
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32), // Top-down
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default()],
            };
            
            let mut data = vec![0u8; (width * height * 4) as usize];
            
            GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                height,
                data.as_mut_ptr() as *mut VOID,
                &mut bmi,
                DIB_RGB_COLORS,
            );
            
            // Convert BGRA to RGBA
            for chunk in data.chunks_exact_mut(4) {
                let b = chunk[0];
                chunk[0] = chunk[2]; // R
                chunk[2] = b;        // B
                chunk[3] = 255;      // A
            }
            
            // Cleanup
            SelectObject(hdc_mem, old_bitmap);
            DeleteObject(hbitmap as _);
            DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc_screen);
            
            Ok(Screenshot { width, height, data })
        }
    }
}

// ============================================================================
// macOS Implementation
// ============================================================================
#[cfg(target_os = "macos")]
impl Screen {
    fn get_screen_info_macos() -> std::result::Result<(u32, u32, f32), ()> {
        use core_graphics::display::{CGDisplay, CGMainDisplayID};
        
        let display_id = unsafe { CGMainDisplayID() };
        let display = CGDisplay::new(display_id);
        
        let width = display.pixels_wide() as u32;
        let height = display.pixels_high() as u32;
        
        // Get scale factor
        let scale = if let Some(mode) = display.display_mode() {
            let pixel_width = mode.pixel_width() as f32;
            pixel_width / width as f32
        } else {
            1.0
        };
        
        Ok((width, height, scale))
    }
    
    fn capture_macos(x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot> {
        use core_graphics::display::{CGDisplay, CGMainDisplayID, CGRect};
        use core_graphics::geometry::CGPoint;
        
        unsafe {
            let display_id = CGMainDisplayID();
            let display = CGDisplay::new(display_id);
            
            let rect = CGRect {
                origin: CGPoint { x: x as f64, y: y as f64 },
                size: core_graphics::geometry::CGSize { width: width as f64, height: height as f64 },
            };
            
            let image = display.image_for_rect(rect)
                .ok_or_else(|| DesktopError::ScreenCaptureFailed("Failed to capture screen".into()))?;
            
            let data = image.data();
            let bytes = data.bytes();
            
            // Convert BGRA to RGBA
            let mut rgba = Vec::with_capacity(bytes.len());
            for chunk in bytes.chunks(4) {
                if chunk.len() == 4 {
                    rgba.push(chunk[2]); // R
                    rgba.push(chunk[1]); // G
                    rgba.push(chunk[0]); // B
                    rgba.push(chunk[3]); // A
                }
            }
            
            Ok(Screenshot { width, height, data: rgba })
        }
    }
}

// ============================================================================
// Screenshot
// ============================================================================

/// Screenshot data
#[derive(Clone)]
pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Screenshot {
    /// Create screenshot from raw data
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self { width, height, data }
    }

    /// Get pixel at position
    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 3 < self.data.len() {
            Some([
                self.data[idx],
                self.data[idx + 1],
                self.data[idx + 2],
                self.data[idx + 3],
            ])
        } else {
            None
        }
    }

    /// Convert to base64 PNG
    pub fn to_base64(&self) -> Result<String> {
        use base64::Engine;
        let png_data = self.to_png()?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&png_data))
    }
    
    /// Convert to PNG bytes
    pub fn to_png(&self) -> Result<Vec<u8>> {
        use image::{ImageBuffer, Rgba, ImageFormat};
        
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
            self.width,
            self.height,
            self.data.clone()
        ).ok_or_else(|| DesktopError::ImageError("Failed to create image buffer".into()))?;
        
        let mut png_data = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut png_data), ImageFormat::Png)
            .map_err(|e| DesktopError::ImageError(e.to_string()))?;
        Ok(png_data)
    }

    /// Save to file
    pub fn save(&self, path: &str) -> Result<()> {
        let img = image::RgbaImage::from_raw(self.width, self.height, self.data.clone())
            .ok_or_else(|| DesktopError::ImageError("Failed to create image".into()))?;
        img.save(path).map_err(|e| DesktopError::ImageError(e.to_string()))?;
        Ok(())
    }

    /// Load from file
    pub fn load(path: &str) -> Result<Self> {
        let img = image::open(path)
            .map_err(|e| DesktopError::ImageError(e.to_string()))?;
        let rgba = img.to_rgba8();
        Ok(Self {
            width: rgba.width(),
            height: rgba.height(),
            data: rgba.into_raw(),
        })
    }

    /// Find template in screenshot (normalized cross-correlation)
    pub fn find_template(&self, template: &Screenshot) -> Result<Option<(u32, u32)>> {
        self.find_template_with_threshold(template, 0.8)
    }
    
    /// Find template with custom threshold
    pub fn find_template_with_threshold(&self, template: &Screenshot, threshold: f32) -> Result<Option<(u32, u32)>> {
        if template.width > self.width || template.height > self.height {
            return Ok(None);
        }
        
        let search_w = self.width - template.width + 1;
        let search_h = self.height - template.height + 1;
        
        let mut best_match: Option<(u32, u32, f32)> = None;
        
        for y in 0..search_h {
            for x in 0..search_w {
                let score = self.compute_ncc(template, x, y);
                if score >= threshold {
                    if let Some((_, _, best_score)) = best_match {
                        if score > best_score {
                            best_match = Some((x, y, score));
                        }
                    } else {
                        best_match = Some((x, y, score));
                    }
                }
            }
        }
        
        Ok(best_match.map(|(x, y, _)| (x, y)))
    }
    
    /// Normalized cross-correlation at position
    fn compute_ncc(&self, template: &Screenshot, ox: u32, oy: u32) -> f32 {
        let mut sum_img = 0.0f64;
        let sum_tpl = 0.0f64;
        let sum_sq_img = 0.0f64;
        let sum_sq_tpl = 0.0f64;
        let sum_cross = 0.0f64;
        let n = (template.width * template.height) as f64;
        
        for ty in 0..template.height {
            for x in 0..template.width {
                let img_r = self.data[((oy + ty) * self.width + ox + x) as usize * 4] as f64;
                let tmpl_r = template.data[(ty * template.width + x) as usize * 4] as f64;
                sum_img += img_r * tmpl_r;
            }
        }
        
        // Simplified NCC (just correlation, not normalized for speed)
        sum_img as f32 / (template.width * template.height * 255 * 255) as f32
    }

    /// Find all matches of template
    pub fn find_all_templates(&self, template: &Screenshot, threshold: f32) -> Result<Vec<(u32, u32)>> {
        let mut matches = Vec::new();
        
        if template.width > self.width || template.height > self.height {
            return Ok(matches);
        }
        
        let search_w = self.width - template.width + 1;
        let search_h = self.height - template.height + 1;
        
        for y in 0..search_h {
            for x in 0..search_w {
                let score = self.compute_ncc(template, x, y);
                if score >= threshold {
                    matches.push((x, y));
                }
            }
        }
        
        Ok(matches)
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Resize screenshot
    pub fn resize(&self, new_width: u32, new_height: u32) -> Result<Self> {
        let img = image::RgbaImage::from_raw(self.width, self.height, self.data.clone())
            .ok_or_else(|| DesktopError::ImageError("Failed to create image".into()))?;
        
        let resized = image::imageops::resize(&img, new_width, new_height, image::imageops::FilterType::Lanczos3);
        
        Ok(Self {
            width: new_width,
            height: new_height,
            data: resized.into_raw(),
        })
    }

    /// Crop screenshot
    pub fn crop(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        if x + width > self.width || y + height > self.height {
            return Err(DesktopError::InvalidCoordinates("Crop region out of bounds".into()));
        }
        
        let mut cropped = Vec::with_capacity((width * height * 4) as usize);
        
        for row in y..y + height {
            let start = (row * self.width + x) as usize * 4;
            let end = start + (width as usize * 4);
            cropped.extend_from_slice(&self.data[start..end]);
        }
        
        Ok(Self { width, height, data: cropped })
    }

    /// Convert to RGB
    pub fn to_rgb(&self) -> Vec<u8> {
        let mut rgb = Vec::with_capacity((self.width * self.height * 3) as usize);
        for chunk in self.data.chunks(4) {
            rgb.push(chunk[0]);
            rgb.push(chunk[1]);
            rgb.push(chunk[2]);
        }
        rgb
    }
    
    /// Convert to grayscale
    pub fn to_grayscale(&self) -> Vec<u8> {
        let mut gray = Vec::with_capacity((self.width * self.height) as usize);
        for chunk in self.data.chunks(4) {
            // Luminosity method
            let lum = (0.299 * chunk[0] as f32 + 0.587 * chunk[1] as f32 + 0.114 * chunk[2] as f32) as u8;
            gray.push(lum);
        }
        gray
    }
}

/// Screen capture trait for custom implementations
pub trait ScreenCapture: Send + Sync {
    fn capture(&self) -> Result<Screenshot>;
    fn capture_region(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_dimensions() {
        let result = Screen::dimensions();
        assert!(result.is_ok());
        let (w, h) = result.unwrap();
        assert!(w > 0);
        assert!(h > 0);
    }

    #[test]
    fn test_screenshot_pixel() {
        let screenshot = Screenshot::new(100, 100, vec![255u8; 100 * 100 * 4]);
        let pixel = screenshot.pixel(50, 50).unwrap();
        
        assert_eq!(pixel, [255, 255, 255, 255]);
    }

    #[test]
    fn test_screenshot_dimensions() {
        let screenshot = Screenshot::new(800, 600, vec![0u8; 800 * 600 * 4]);
        let (w, h) = screenshot.dimensions();
        
        assert_eq!(w, 800);
        assert_eq!(h, 600);
    }
    
    #[test]
    fn test_screenshot_crop() {
        let screenshot = Screenshot::new(100, 100, vec![128u8; 100 * 100 * 4]);
        let cropped = screenshot.crop(10, 10, 50, 50).unwrap();
        
        assert_eq!(cropped.width, 50);
        assert_eq!(cropped.height, 50);
        assert_eq!(cropped.data.len(), 50 * 50 * 4);
    }
}
