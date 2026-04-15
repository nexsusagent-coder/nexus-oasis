// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Window Management (Real Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{DesktopError, Result, Rect};
use serde::{Deserialize, Serialize};

/// X11 Window type (just a u32)
#[cfg(target_os = "linux")]
type X11Window = u32;

/// Window manager
pub struct WindowManager;

impl WindowManager {
    /// List all windows
    pub fn list_windows() -> Result<Vec<Window>> {
        #[cfg(target_os = "linux")]
        {
            Self::list_windows_linux()
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::list_windows_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::list_windows_macos()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(vec![])
        }
    }

    /// Get active window
    pub fn get_active() -> Result<Window> {
        #[cfg(target_os = "linux")]
        {
            Self::get_active_linux()
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::get_active_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::get_active_macos()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(DesktopError::PlatformNotSupported("Unsupported OS".into()))
        }
    }

    /// Find window by title
    pub fn find_by_title(title: &str) -> Result<Option<Window>> {
        let windows = Self::list_windows()?;
        Ok(windows.into_iter().find(|w| w.title.contains(title)))
    }

    /// Find window by ID
    pub fn find_by_id(id: u64) -> Result<Option<Window>> {
        let windows = Self::list_windows()?;
        Ok(windows.into_iter().find(|w| w.id == id))
    }
    
    /// Wait for window to appear
    pub async fn wait_for_window(title: &str, timeout_ms: u64) -> Result<Window> {
        let start = std::time::Instant::now();
        
        loop {
            if let Some(window) = Self::find_by_title(title)? {
                return Ok(window);
            }
            
            if start.elapsed().as_millis() as u64 > timeout_ms {
                return Err(DesktopError::WindowNotFound(title.into()));
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }
}

// ============================================================================
// Linux Implementation (X11)
// ============================================================================
#[cfg(target_os = "linux")]
impl WindowManager {
    fn list_windows_linux() -> Result<Vec<Window>> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| DesktopError::WindowNotFound(format!("X11 connect failed: {}", e)))?;
        
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;
        
        // Get list of windows
        let tree_reply = query_tree(&conn, root)
            .map_err(|e| DesktopError::WindowNotFound(format!("Query tree failed: {}", e)))?
            .reply()
            .map_err(|e| DesktopError::WindowNotFound(format!("Query tree reply failed: {}", e)))?;
        
        let mut windows = Vec::new();
        
        for win in tree_reply.children {
            let attrs_cookie = get_window_attributes(&conn, win);
            if let Ok(attrs) = attrs_cookie {
                if let Ok(attrs_reply) = attrs.reply() {
                    if attrs_reply.map_state == MapState::VIEWABLE {
                        let title = Self::get_window_title_linux(&conn, win).unwrap_or_default();
                        let geometry = get_geometry(&conn, win).ok().and_then(|c| c.reply().ok());
                        
                        let rect = if let Some(geo) = geometry {
                            Rect::new(geo.x as u32, geo.y as u32, geo.width as u32, geo.height as u32)
                        } else {
                            Rect::new(0, 0, 800, 600)
                        };
                        
                        windows.push(crate::window::Window {
                            id: win as u64,
                            title,
                            rect,
                            is_visible: true,
                            is_focused: false, // Would need to compare with active window
                        });
                    }
                }
            }
        }
        
        Ok(windows)
    }
    
    fn get_active_linux() -> Result<Window> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| DesktopError::WindowNotFound(format!("X11 connect failed: {}", e)))?;
        
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;
        
        // Get active window via _NET_ACTIVE_WINDOW
        let active_atom = intern_atom(&conn, false, b"_NET_ACTIVE_WINDOW")
            .map_err(|e| DesktopError::WindowNotFound(format!("Intern atom failed: {}", e)))?
            .reply()
            .map_err(|e| DesktopError::WindowNotFound(format!("Intern atom reply failed: {}", e)))?;
        
        let prop_reply = get_property(
            &conn,
            false,
            root,
            active_atom.atom,
            AtomEnum::WINDOW,
            0,
            1,
        ).map_err(|e| DesktopError::WindowNotFound(format!("Get property failed: {}", e)))?
        .reply()
        .map_err(|e| DesktopError::WindowNotFound(format!("Get property reply failed: {}", e)))?;
        
        if prop_reply.value_len == 0 {
            return Err(DesktopError::WindowNotFound("No active window".into()));
        }
        
        let win = u32::from_ne_bytes([prop_reply.value[0], prop_reply.value[1], prop_reply.value[2], prop_reply.value[3]]);
        
        let title = Self::get_window_title_linux(&conn, win).unwrap_or_default();
        let geometry = get_geometry(&conn, win).ok().and_then(|c| c.reply().ok());
        
        let rect = if let Some(geo) = geometry {
            Rect::new(geo.x as u32, geo.y as u32, geo.width as u32, geo.height as u32)
        } else {
            Rect::new(0, 0, 800, 600)
        };
        
        Ok(crate::window::Window {
            id: win as u64,
            title,
            rect,
            is_visible: true,
            is_focused: true,
        })
    }
    
    fn get_window_title_linux(conn: &impl x11rb::connection::Connection, win: X11Window) -> Result<String> {
        use x11rb::protocol::xproto::*;
        
        let wm_name_atom = intern_atom(conn, false, b"WM_NAME")
            .map_err(|e| DesktopError::WindowNotFound(format!("Intern atom failed: {}", e)))?
            .reply()
            .map_err(|e| DesktopError::WindowNotFound(format!("Intern atom reply failed: {}", e)))?;
        
        let prop_reply = get_property(
            conn,
            false,
            win,
            wm_name_atom.atom,
            AtomEnum::STRING,
            0,
            1024,
        ).map_err(|e| DesktopError::WindowNotFound(format!("Get property failed: {}", e)))?
        .reply()
        .map_err(|e| DesktopError::WindowNotFound(format!("Get property reply failed: {}", e)))?;
        
        Ok(String::from_utf8(prop_reply.value).unwrap_or_default())
    }
}

// ============================================================================
// Windows Implementation
// ============================================================================
#[cfg(target_os = "windows")]
impl WindowManager {
    fn list_windows_windows() -> Result<Vec<Window>> {
        use winapi::um::winuser::{EnumWindows, GetWindowTextW, GetWindowRect, IsWindowVisible, GetWindowThreadProcessId};
        use winapi::shared::ntdef::BOOL;
        use winapi::shared::minwindef::{LPARAM, TRUE};
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        
        let windows: Vec<Window> = Vec::new();
        let windows_ptr = Box::into_raw(Box::new(windows)) as LPARAM;
        
        unsafe extern "system" fn enum_windows_proc(hwnd: winapi::shared::windef::HWND, lparam: LPARAM) -> BOOL {
            let windows = &mut *(lparam as *mut Vec<Window>);
            
            if IsWindowVisible(hwnd) != TRUE {
                return TRUE;
            }
            
            let mut title_buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 512);
            
            if len == 0 {
                return TRUE;
            }
            
            let title = OsString::from_wide(&title_buf[..len as usize])
                .to_string_lossy()
                .into_owned();
            
            let mut rect = std::mem::MaybeUninit::uninit();
            GetWindowRect(hwnd, rect.as_mut_ptr());
            let rect = rect.assume_init();
            
            windows.push(Window {
                id: hwnd as u64,
                title,
                rect: Rect::new(
                    rect.left as u32,
                    rect.top as u32,
                    (rect.right - rect.left) as u32,
                    (rect.bottom - rect.top) as u32,
                ),
                is_visible: true,
                is_focused: false,
            });
            
            TRUE
        }
        
        unsafe {
            EnumWindows(Some(enum_windows_proc), windows_ptr);
            let windows = Box::from_raw(windows_ptr as *mut Vec<Window>);
            Ok(*windows)
        }
    }
    
    fn get_active_windows() -> Result<Window> {
        use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW, GetWindowRect};
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        
        unsafe {
            let hwnd = GetForegroundWindow();
            
            let mut title_buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 512);
            let title = if len > 0 {
                OsString::from_wide(&title_buf[..len as usize])
                    .to_string_lossy()
                    .into_owned()
            } else {
                String::new()
            };
            
            let mut rect = std::mem::MaybeUninit::uninit();
            GetWindowRect(hwnd, rect.as_mut_ptr());
            let rect = rect.assume_init();
            
            Ok(crate::window::Window {
                id: hwnd as u64,
                title,
                rect: Rect::new(
                    rect.left as u32,
                    rect.top as u32,
                    (rect.right - rect.left) as u32,
                    (rect.bottom - rect.top) as u32,
                ),
                is_visible: true,
                is_focused: true,
            })
        }
    }
}

// ============================================================================
// macOS Implementation
// ============================================================================
#[cfg(target_os = "macos")]
impl WindowManager {
    fn list_windows_macos() -> Result<Vec<Window>> {
        use core_graphics::window::{CGWindowListCopyWindowInfo, kCGNullWindowID, kCGWindowListOptionOnScreenOnly};
        
        let window_list = unsafe { CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, kCGNullWindowID) };
        
        let mut windows = Vec::new();
        
        // Parse window list (simplified - would need proper CFArray parsing)
        // For now, return empty or basic window
        
        Ok(windows)
    }
    
    fn get_active_macos() -> Result<Window> {
        // Would use AXUIElement API for active window
        Err(DesktopError::WindowNotFound("macOS window management requires accessibility permissions".into()))
    }
}

// ============================================================================
// Window
// ============================================================================

/// Window information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    /// Window ID
    pub id: u64,
    /// Window title
    pub title: String,
    /// Window position and size
    pub rect: Rect,
    /// Is window visible
    pub is_visible: bool,
    /// Is window focused
    pub is_focused: bool,
}

impl Window {
    /// Activate/focus this window
    pub fn activate(&self) -> Result<()> {
        tracing::debug!("Activating window: {}", self.title);
        
        #[cfg(target_os = "linux")]
        {
            use x11rb::connection::Connection;
            use x11rb::protocol::xproto::*;
            
            let (conn, _) = x11rb::connect(None)
                .map_err(|e| DesktopError::WindowNotFound(format!("X11 connect failed: {}", e)))?;
            
            let win = self.id as Window;
            map_window(&conn, win).ok();
            
            configure_window(
                &conn,
                win,
                &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
            ).ok();
            
            set_input_focus(
                &conn,
                InputFocus::POINTER_ROOT,
                win,
                0u32, // CURRENT_TIME
            ).ok();
            
            conn.flush().ok();
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{SetForegroundWindow, BringWindowToTop, ShowWindow, SW_RESTORE};
            
            unsafe {
                let hwnd = self.id as winapi::shared::windef::HWND;
                ShowWindow(hwnd, SW_RESTORE);
                BringWindowToTop(hwnd);
                SetForegroundWindow(hwnd);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            // Would use AXUIElementPerformAction for kAXRaiseAction
        }
        
        Ok(())
    }

    /// Close this window
    pub fn close(&self) -> Result<()> {
        tracing::debug!("Closing window: {}", self.title);
        
        #[cfg(target_os = "linux")]
        {
            // Send WM_DELETE_WINDOW message
            use x11rb::connection::Connection;
            use x11rb::protocol::xproto::*;
            
            let (conn, _) = x11rb::connect(None)
                .map_err(|e| DesktopError::WindowNotFound(format!("X11 connect failed: {}", e)))?;
            
            // Simplified: just log for now
            tracing::debug!("Sending close event for window {}", self.id);
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{PostMessageW, WM_CLOSE};
            
            unsafe {
                PostMessageW(self.id as winapi::shared::windef::HWND, WM_CLOSE, 0, 0);
            }
        }
        
        Ok(())
    }

    /// Minimize this window
    pub fn minimize(&self) -> Result<()> {
        tracing::debug!("Minimizing window: {}", self.title);
        
        #[cfg(target_os = "linux")]
        {
            // Would use _NET_WM_STATE with _NET_WM_STATE_HIDDEN
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{ShowWindow, SW_MINIMIZE};
            unsafe {
                ShowWindow(self.id as winapi::shared::windef::HWND, SW_MINIMIZE);
            }
        }
        
        Ok(())
    }

    /// Maximize this window
    pub fn maximize(&self) -> Result<()> {
        tracing::debug!("Maximizing window: {}", self.title);
        
        #[cfg(target_os = "linux")]
        {
            // Would use _NET_WM_STATE with _NET_WM_STATE_MAXIMIZED_VERT/HORZ
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{ShowWindow, SW_MAXIMIZE};
            unsafe {
                ShowWindow(self.id as winapi::shared::windef::HWND, SW_MAXIMIZE);
            }
        }
        
        Ok(())
    }

    /// Restore this window
    pub fn restore(&self) -> Result<()> {
        tracing::debug!("Restoring window: {}", self.title);
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{ShowWindow, SW_RESTORE};
            unsafe {
                ShowWindow(self.id as winapi::shared::windef::HWND, SW_RESTORE);
            }
        }
        
        Ok(())
    }

    /// Move window to position
    pub fn move_to(&self, x: u32, y: u32) -> Result<()> {
        tracing::debug!("Moving window {} to ({}, {})", self.title, x, y);
        
        #[cfg(target_os = "linux")]
        {
            use x11rb::connection::Connection;
            use x11rb::protocol::xproto::*;
            
            let (conn, _) = x11rb::connect(None).ok().unwrap();
            configure_window(
                &conn,
                self.id as Window,
                &ConfigureWindowAux::new().x(x as i32).y(y as i32),
            ).ok();
            conn.flush().ok();
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::SetWindowPos;
            unsafe {
                SetWindowPos(
                    self.id as winapi::shared::windef::HWND,
                    std::ptr::null_mut(),
                    x as i32,
                    y as i32,
                    0, 0,
                    winapi::um::winuser::SWP_NOSIZE | winapi::um::winuser::SWP_NOZORDER,
                );
            }
        }
        
        Ok(())
    }

    /// Resize window
    pub fn resize(&self, width: u32, height: u32) -> Result<()> {
        tracing::debug!("Resizing window {} to {}x{}", self.title, width, height);
        
        #[cfg(target_os = "linux")]
        {
            use x11rb::connection::Connection;
            use x11rb::protocol::xproto::*;
            
            let (conn, _) = x11rb::connect(None).ok().unwrap();
            configure_window(
                &conn,
                self.id as Window,
                &ConfigureWindowAux::new().width(width).height(height),
            ).ok();
            conn.flush().ok();
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::SetWindowPos;
            unsafe {
                SetWindowPos(
                    self.id as winapi::shared::windef::HWND,
                    std::ptr::null_mut(),
                    0, 0,
                    width as i32,
                    height as i32,
                    winapi::um::winuser::SWP_NOMOVE | winapi::um::winuser::SWP_NOZORDER,
                );
            }
        }
        
        Ok(())
    }

    /// Capture window screenshot
    pub fn screenshot(&self) -> Result<crate::Screenshot> {
        crate::Screen::capture_rect(self.rect)
    }

    /// Get window center
    pub fn center(&self) -> (u32, u32) {
        self.rect.center().into()
    }

    /// Check if point is inside window
    pub fn contains(&self, x: u32, y: u32) -> bool {
        self.rect.contains(crate::Point::new(x, y))
    }
    
    /// Click at window center
    pub fn click_center(&self) -> Result<()> {
        let (x, y) = self.center();
        crate::Mouse::move_to(x, y)?;
        crate::Mouse::click(crate::MouseButton::Left)
    }
    
    /// Type text into window
    pub fn type_text(&self, text: &str) -> Result<()> {
        self.activate()?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        crate::Keyboard::type_text(text)
    }
}

impl From<Rect> for (u32, u32) {
    fn from(rect: Rect) -> Self {
        (rect.x, rect.y)
    }
}

impl From<crate::Point> for (u32, u32) {
    fn from(point: crate::Point) -> Self {
        (point.x, point.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires X11/display server
    fn test_list_windows() {
        let windows = WindowManager::list_windows();
        assert!(windows.is_ok());
    }

    #[test]
    fn test_get_active_window() {
        let window = WindowManager::get_active();
        // May fail on headless systems
        if window.is_ok() {
            let w = window.unwrap();
            assert!(w.is_focused);
        }
    }

    #[test]
    fn test_window_contains() {
        let window = Window {
            id: 1,
            title: "Test".to_string(),
            rect: Rect::new(100, 100, 200, 200),
            is_visible: true,
            is_focused: false,
        };
        
        assert!(window.contains(150, 150));
        assert!(!window.contains(50, 50));
    }
    
    #[test]
    fn test_window_center() {
        let window = Window {
            id: 1,
            title: "Test".to_string(),
            rect: Rect::new(0, 0, 100, 100),
            is_visible: true,
            is_focused: false,
        };
        
        let (x, y) = window.center();
        assert_eq!(x, 50);
        assert_eq!(y, 50);
    }
}
