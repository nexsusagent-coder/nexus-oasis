// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Computer Use / GUI Automation
// ═══════════════════════════════════════════════════════════════════════════════
//  Control desktop like a human
//  - Screen capture
//  - Mouse control
//  - Keyboard input
//  - Window management
//  - OCR support
// ═══════════════════════════════════════════════════════════════════════════════

// Suppress warnings for stub implementations
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod screen;
pub mod mouse;
pub mod keyboard;
pub mod window;
pub mod error;

pub use screen::{Screen, ScreenCapture, Screenshot};
pub use mouse::{Mouse, MouseButton, MouseAction};
pub use keyboard::{Keyboard, Key, KeyboardAction};
pub use window::{Window, WindowManager};
pub use error::{DesktopError, Result};

use serde::{Deserialize, Serialize};

/// Desktop automation controller
#[derive(Debug, Clone)]
pub struct Desktop {
    /// Screen dimensions
    pub width: u32,
    pub height: u32,
    /// Scaling factor (for HiDPI)
    pub scale: f32,
}

impl Desktop {
    /// Create new desktop controller
    pub fn new() -> Result<Self> {
        let (width, height, scale) = Self::get_screen_info()?;
        Ok(Self { width, height, scale })
    }

    /// Get screen information
    fn get_screen_info() -> Result<(u32, u32, f32)> {
        // Default values, actual implementation would query system
        Ok((1920, 1080, 1.0))
    }

    /// Take screenshot
    pub fn screenshot(&self) -> Result<Screenshot> {
        Screen::capture_all()
    }

    /// Take screenshot of region
    pub fn screenshot_region(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Screenshot> {
        Screen::capture_region(x, y, width, height)
    }

    /// Move mouse to position
    pub fn move_mouse(&self, x: u32, y: u32) -> Result<()> {
        Mouse::move_to(x, y)
    }

    /// Click at position
    pub fn click(&self, x: u32, y: u32, button: MouseButton) -> Result<()> {
        Mouse::move_to(x, y)?;
        Mouse::click(button)
    }

    /// Type text
    pub fn type_text(&self, text: &str) -> Result<()> {
        Keyboard::type_text(text)
    }

    /// Press key
    pub fn press_key(&self, key: Key) -> Result<()> {
        Keyboard::press(key)
    }

    /// Hotkey combination
    pub fn hotkey(&self, keys: &[Key]) -> Result<()> {
        Keyboard::hotkey(keys)
    }

    /// Get mouse position
    pub fn mouse_position(&self) -> Result<(u32, u32)> {
        Mouse::position()
    }

    /// Scroll
    pub fn scroll(&self, amount: i32) -> Result<()> {
        Mouse::scroll(amount)
    }

    /// Drag from one point to another
    pub fn drag(&self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> Result<()> {
        Mouse::move_to(from_x, from_y)?;
        Mouse::down(MouseButton::Left)?;
        Mouse::move_to(to_x, to_y)?;
        Mouse::up(MouseButton::Left)?;
        Ok(())
    }

    /// Find element on screen (template matching)
    pub fn find_on_screen(&self, template: &Screenshot) -> Result<Option<(u32, u32)>> {
        let screen = self.screenshot()?;
        screen.find_template(template)
    }

    /// Wait for element to appear
    pub async fn wait_for(&self, template: &Screenshot, timeout_ms: u64) -> Result<(u32, u32)> {
        let start = std::time::Instant::now();
        
        loop {
            if let Some(pos) = self.find_on_screen(template)? {
                return Ok(pos);
            }
            
            if start.elapsed().as_millis() as u64 > timeout_ms {
                return Err(DesktopError::Timeout);
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    /// Click on element
    pub fn click_element(&self, template: &Screenshot) -> Result<bool> {
        if let Some((x, y)) = self.find_on_screen(template)? {
            self.click(x, y, MouseButton::Left)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get all windows
    pub fn windows(&self) -> Result<Vec<Window>> {
        WindowManager::list_windows()
    }

    /// Get active window
    pub fn active_window(&self) -> Result<Window> {
        WindowManager::get_active()
    }
}

impl Default for Desktop {
    fn default() -> Self {
        Self::new().unwrap_or(Self {
            width: 1920,
            height: 1080,
            scale: 1.0,
        })
    }
}

/// Screen coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

/// Rectangle region
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x 
            && point.x < self.x + self.width
            && point.y >= self.y 
            && point.y < self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2, self.y + self.height / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point::new(100, 200);
        assert_eq!(point.x, 100);
        assert_eq!(point.y, 200);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 10, 100, 100);
        
        assert!(rect.contains(Point::new(50, 50)));
        assert!(rect.contains(Point::new(10, 10)));
        assert!(!rect.contains(Point::new(200, 200)));
    }

    #[test]
    fn test_rect_center() {
        let rect = Rect::new(0, 0, 100, 100);
        let center = rect.center();
        
        assert_eq!(center.x, 50);
        assert_eq!(center.y, 50);
    }

    #[test]
    fn test_desktop_creation() {
        let desktop = Desktop::default();
        assert!(desktop.width > 0);
        assert!(desktop.height > 0);
    }
}
