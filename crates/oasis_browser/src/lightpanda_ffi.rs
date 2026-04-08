//! Lightpanda FFI (Foreign Function Interface) Köprüleri
//! Zig tabanlı Lightpanda browser'ın Rust'a entegrasyonu
//!
//! BELLEK GÜVENLİĞİ:
//! - unsafe blokları minimize edilmiştir
//! - Tüm FFI çağrıları wrapper içinde kapsüllenmiştir
//! - Bellek yönetimi Rust tarafında kontrol edilir

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Lightpanda Browser FFI wrapper
/// 
/// # Safety
/// Bu yapı unsafe FFI çağrılarını sarmalar.
/// Kullanıcı doğrudan unsafe çağrı yapmamalıdır.
pub struct LightpandaFFI {
    /// Browser handle (opaque pointer)
    handle: *mut LightpandaHandle,
    /// Başlatıldı mı?
    initialized: bool,
}

/// Opaque handle tipi
#[repr(C)]
pub struct LightpandaHandle {
    _private: [u8; 0], // Zero-sized, opaque
}

/// FFI fonksiyon bildirimleri (Zig kütüphanesinden)
#[link(name = "lightpanda", kind = "static")]
extern "C" {
    /// Browser oluştur
    fn lightpanda_browser_init() -> *mut LightpandaHandle;
    /// Browser kapat
    fn lightpanda_browser_deinit(handle: *mut LightpandaHandle);
    /// Yeni sayfa oluştur
    fn lightpanda_browser_new_page(handle: *mut LightpandaHandle) -> *mut PageHandle;
    /// Sayfaya git
    fn lightpanda_page_navigate(
        page: *mut PageHandle,
        url: *const c_char,
        timeout_ms: u32,
    ) -> i32;
    /// DOM al
    fn lightpanda_page_get_dom(
        page: *mut PageHandle,
        buffer: *mut c_char,
        buffer_size: usize,
    ) -> i32;
    /// Sayfayı kapat
    fn lightpanda_page_close(page: *mut PageHandle);
    /// Ekran görüntüsü al
    fn lightpanda_page_screenshot(
        page: *mut PageHandle,
        path: *const c_char,
        full_page: bool,
    ) -> i32;
    /// JavaScript çalıştır
    fn lightpanda_page_eval(
        page: *mut PageHandle,
        script: *const c_char,
        result: *mut c_char,
        result_size: usize,
    ) -> i32;
}

/// Opaque page handle
#[repr(C)]
pub struct PageHandle {
    _private: [u8; 0],
}

/// FFI hata kodları
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FFIError {
    Success = 0,
    InvalidHandle = -1,
    NavigationFailed = -2,
    Timeout = -3,
    BufferTooSmall = -4,
    JavaScriptError = -5,
    Unknown = -99,
}

impl From<i32> for FFIError {
    fn from(code: i32) -> Self {
        match code {
            0 => FFIError::Success,
            -1 => FFIError::InvalidHandle,
            -2 => FFIError::NavigationFailed,
            -3 => FFIError::Timeout,
            -4 => FFIError::BufferTooSmall,
            -5 => FFIError::JavaScriptError,
            _ => FFIError::Unknown,
        }
    }
}

/// FFI sonuç tipi
pub type FFIResult<T> = Result<T, FFIError>;

impl LightpandaFFI {
    /// Yeni Lightpanda FFI instance oluştur
    /// 
    /// # Safety
    /// Bu fonksiyon unsafe FFI çağrısı yapar ama wrapper tarafından güvenli hale getirilir.
    pub fn new() -> FFIResult<Self> {
        log::info!("🌐 LIGHTPANDA-FFI: Browser başlatılıyor...");
        
        // SAFETY: lightpanda_browser_init güvenli bir şekilde çağrılır
        let handle = unsafe { lightpanda_browser_init() };
        
        if handle.is_null() {
            log::error!("❌ LIGHTPANDA-FFI: Browser başlatılamadı");
            return Err(FFIError::InvalidHandle);
        }
        
        log::info!("✅ LIGHTPANDA-FFI: Browser hazır");
        Ok(Self {
            handle,
            initialized: true,
        })
    }
    
    /// Yeni sayfa oluştur
    pub fn new_page(&mut self) -> FFIResult<LightpandaPage> {
        self.ensure_initialized()?;
        
        // SAFETY: handle geçerli ve initialized true
        let page_handle = unsafe { lightpanda_browser_new_page(self.handle) };
        
        if page_handle.is_null() {
            log::error!("❌ LIGHTPANDA-FFI: Sayfa oluşturulamadı");
            return Err(FFIError::InvalidHandle);
        }
        
        log::info!("✅ LIGHTPANDA-FFI: Yeni sayfa oluşturuldu");
        Ok(LightpandaPage {
            handle: page_handle,
            url: None,
        })
    }
    
    /// Browser'ı kapat
    pub fn close(&mut self) {
        if self.initialized && !self.handle.is_null() {
            // SAFETY: handle geçerli ve kapatma işlemi güvenli
            unsafe {
                lightpanda_browser_deinit(self.handle);
            }
            self.handle = ptr::null_mut();
            self.initialized = false;
            log::info!("🌐 LIGHTPANDA-FFI: Browser kapatıldı");
        }
    }
    
    fn ensure_initialized(&self) -> FFIResult<()> {
        if !self.initialized || self.handle.is_null() {
            Err(FFIError::InvalidHandle)
        } else {
            Ok(())
        }
    }
}

impl Drop for LightpandaFFI {
    fn drop(&mut self) {
        self.close();
    }
}

impl Default for LightpandaFFI {
    fn default() -> Self {
        Self::new().expect("Lightpanda FFI başlatılamadı")
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  LIGHTPANDA PAGE
// ─────────────────────────────────────────────────────────────────────────────--

/// Lightpanda sayfası
pub struct LightpandaPage {
    handle: *mut PageHandle,
    url: Option<String>,
}

impl LightpandaPage {
    /// URL'ye git
    pub fn navigate(&mut self, url: &str, timeout_ms: u32) -> FFIResult<()> {
        self.ensure_valid()?;
        
        let c_url = CString::new(url)
            .map_err(|_| FFIError::Unknown)?;
        
        log::info!("🌐 LIGHTPANDA-PAGE: Navigating → {}", url.chars().take(50).collect::<String>());
        
        // SAFETY: handle ve c_url geçerli
        let result = unsafe {
            lightpanda_page_navigate(self.handle, c_url.as_ptr(), timeout_ms)
        };
        
        if result == 0 {
            self.url = Some(url.to_string());
            log::info!("✅ LIGHTPANDA-PAGE: Navigation başarılı");
            Ok(())
        } else {
            log::error!("❌ LIGHTPANDA-PAGE: Navigation başarısız → {}", result);
            Err(FFIError::from(result))
        }
    }
    
    /// DOM içeriğini al
    pub fn get_dom(&self) -> FFIResult<String> {
        self.ensure_valid()?;
        
        let mut buffer = vec![0i8; 1024 * 1024]; // 1MB buffer
        
        // SAFETY: buffer boyutu belirtildi
        let result = unsafe {
            lightpanda_page_get_dom(
                self.handle,
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        
        if result >= 0 {
            // SAFETY: buffer null-terminated
            let dom = unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            Ok(dom)
        } else {
            Err(FFIError::from(result))
        }
    }
    
    /// JavaScript çalıştır
    pub fn eval(&self, script: &str) -> FFIResult<String> {
        self.ensure_valid()?;
        
        let c_script = CString::new(script)
            .map_err(|_| FFIError::Unknown)?;
        let mut result_buffer = vec![0i8; 65536]; // 64KB buffer
        
        // SAFETY: tüm parametreler geçerli
        let result = unsafe {
            lightpanda_page_eval(
                self.handle,
                c_script.as_ptr(),
                result_buffer.as_mut_ptr(),
                result_buffer.len(),
            )
        };
        
        if result >= 0 {
            let output = unsafe {
                CStr::from_ptr(result_buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            Ok(output)
        } else {
            Err(FFIError::from(result))
        }
    }
    
    /// Ekran görüntüsü al
    pub fn screenshot(&self, path: &str, full_page: bool) -> FFIResult<()> {
        self.ensure_valid()?;
        
        let c_path = CString::new(path)
            .map_err(|_| FFIError::Unknown)?;
        
        // SAFETY: path null-terminated
        let result = unsafe {
            lightpanda_page_screenshot(
                self.handle,
                c_path.as_ptr(),
                full_page,
            )
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(FFIError::from(result))
        }
    }
    
    /// Mevcut URL'i getir
    pub fn current_url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    
    fn ensure_valid(&self) -> FFIResult<()> {
        if self.handle.is_null() {
            Err(FFIError::InvalidHandle)
        } else {
            Ok(())
        }
    }
}

impl Drop for LightpandaPage {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: handle geçerli ve kapatma güvenli
            unsafe {
                lightpanda_page_close(self.handle);
            }
            log::info!("🌐 LIGHTPANDA-PAGE: Sayfa kapatıldı");
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ffi_error_conversion() {
        assert_eq!(FFIError::from(0), FFIError::Success);
        assert_eq!(FFIError::from(-1), FFIError::InvalidHandle);
        assert_eq!(FFIError::from(-3), FFIError::Timeout);
    }
    
    #[test]
    fn test_ffi_error_debug() {
        let err = FFIError::NavigationFailed;
        assert!(format!("{:?}", err).contains("Navigation"));
    }
    
    // Not: Gerçek FFI testleri için Zig kütüphanesinin derlenmiş olması gerekir
    // Bu testler mock veya integration test olarak çalıştırılmalıdır
}
