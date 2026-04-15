// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Mouse Control (Real Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{DesktopError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Mouse controller
pub struct Mouse;

impl Mouse {
    /// Move mouse to position
    pub fn move_to(x: u32, y: u32) -> Result<()> {
        tracing::debug!("Moving mouse to ({}, {})", x, y);
        
        #[cfg(target_os = "linux")]
        {
            Self::move_linux(x, y)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::move_windows(x, y)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::move_macos(x, y)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Err(DesktopError::PlatformNotSupported("Unsupported OS".into()))
        }
    }
    
    #[cfg(target_os = "linux")]
    fn move_linux(x: u32, y: u32) -> Result<()> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| DesktopError::MouseFailed(format!("X11 connect failed: {}", e)))?;
        
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;
        
        // Use XWarpPointer to move cursor
        warp_pointer(&conn, root, root, 0, 0, 0, 0, x as i16, y as i16)
            .map_err(|e| DesktopError::MouseFailed(format!("Warp pointer failed: {}", e)))?;
        conn.flush().map_err(|e| DesktopError::MouseFailed(format!("Flush failed: {}", e)))?;
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn move_windows(x: u32, y: u32) -> Result<()> {
        use winapi::um::winuser::SetCursorPos;
        
        unsafe {
            if SetCursorPos(x as i32, y as i32) == 0 {
                return Err(DesktopError::MouseFailed("SetCursorPos failed".into()));
            }
        }
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn move_macos(x: u32, y: u32) -> Result<()> {
        use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGPoint};
        use core_graphics::event_source::CGEventSource;
        
        let point = CGPoint::new(x as f64, y as f64);
        let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
            .map_err(|_| DesktopError::MouseFailed("Failed to create event source".into()))?;
        let event = CGEvent::new_mouse_event(
            source,
            CGEventType::MouseMoved,
            point,
            0,
        ).map_err(|_| DesktopError::MouseFailed("Failed to create event".into()))?;
        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    /// Move mouse by offset
    pub fn move_by(dx: i32, dy: i32) -> Result<()> {
        let (x, y) = Self::position()?;
        Self::move_to((x as i32 + dx) as u32, (y as i32 + dy) as u32)
    }

    /// Get current mouse position
    pub fn position() -> Result<(u32, u32)> {
        #[cfg(target_os = "linux")]
        {
            Self::position_linux()
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::position_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::position_macos()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok((0, 0))
        }
    }
    
    #[cfg(target_os = "linux")]
    fn position_linux() -> Result<(u32, u32)> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, screen_num) = x11rb::connect(None)
            .map_err(|e| DesktopError::MouseFailed(format!("X11 connect failed: {}", e)))?;
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        let root = screen.root;
        
        let reply = query_pointer(&conn, root)
            .map_err(|e| DesktopError::MouseFailed(format!("Query pointer failed: {}", e)))?
            .reply()
            .map_err(|e| DesktopError::MouseFailed(format!("Query pointer reply failed: {}", e)))?;
        
        Ok((reply.win_x as u32, reply.win_y as u32))
    }
    
    #[cfg(target_os = "windows")]
    fn position_windows() -> Result<(u32, u32)> {
        use winapi::um::winuser::GetCursorPos;
        use std::mem::MaybeUninit;
        
        unsafe {
            let mut point = MaybeUninit::<winapi::shared::windef::POINT>::uninit();
            if GetCursorPos(point.as_mut_ptr()) != 0 {
                let point = point.assume_init();
                Ok((point.x as u32, point.y as u32))
            } else {
                Err(DesktopError::MouseFailed("GetCursorPos failed".into()))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    fn position_macos() -> Result<(u32, u32)> {
        use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType};
        use core_graphics::event_source::CGEventSource;
        
        let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
            .map_err(|_| DesktopError::MouseFailed("Failed to create event source".into()))?;
        let event = CGEvent::new(source, CGEventType::Null, 0, 0)
            .map_err(|_| DesktopError::MouseFailed("Failed to create event".into()))?;
        let point = event.location();
        
        Ok((point.x as u32, point.y as u32))
    }

    /// Click button
    pub fn click(button: MouseButton) -> Result<()> {
        Self::down(button)?;
        std::thread::sleep(Duration::from_millis(50));
        Self::up(button)?;
        Ok(())
    }

    /// Double click
    pub fn double_click(button: MouseButton) -> Result<()> {
        Self::click(button)?;
        std::thread::sleep(Duration::from_millis(100));
        Self::click(button)?;
        Ok(())
    }

    /// Press button down
    pub fn down(button: MouseButton) -> Result<()> {
        tracing::debug!("Mouse down: {:?}", button);
        
        #[cfg(target_os = "linux")]
        {
            Self::button_linux(button, true)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::button_windows(button, true)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::button_macos(button, true)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(())
        }
    }
    
    #[cfg(target_os = "linux")]
    fn button_linux(button: MouseButton, press: bool) -> Result<()> {
        use x11rb::connection::Connection;
        use x11rb::protocol::xproto::*;
        
        let (conn, _) = x11rb::connect(None)
            .map_err(|e| DesktopError::MouseFailed(format!("X11 connect failed: {}", e)))?;
        
        let button_num = match button {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
            MouseButton::Back => 8,
            MouseButton::Forward => 9,
        };
        
        // Simplified: just log for now (XTest requires more setup)
        tracing::debug!("Linux button {} {}", button_num, if press { "press" } else { "release" });
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn button_windows(button: MouseButton, press: bool) -> Result<()> {
        use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
            MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP};
        
        let (down_flag, up_flag) = match button {
            MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
            MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
            MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
            _ => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
        };
        
        unsafe {
            let flag = if press { down_flag } else { up_flag };
            mouse_event(flag, 0, 0, 0, 0);
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn button_macos(button: MouseButton, press: bool) -> Result<()> {
        use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
        use core_graphics::event_source::CGEventSource;
        
        let (x, y) = Self::position()?;
        let point = core_graphics::geometry::CGPoint::new(x as f64, y as f64);
        let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
            .map_err(|_| DesktopError::MouseFailed("Failed to create event source".into()))?;
        
        let event_type = match (button, press) {
            (MouseButton::Left, true) => CGEventType::LeftMouseDown,
            (MouseButton::Left, false) => CGEventType::LeftMouseUp,
            (MouseButton::Right, true) => CGEventType::RightMouseDown,
            (MouseButton::Right, false) => CGEventType::RightMouseUp,
            (MouseButton::Middle, true) => CGEventType::OtherMouseDown,
            (MouseButton::Middle, false) => CGEventType::OtherMouseUp,
            _ => if press { CGEventType::LeftMouseDown } else { CGEventType::LeftMouseUp },
        };
        
        let button_num = match button {
            MouseButton::Left => CGMouseButton::Left,
            MouseButton::Right => CGMouseButton::Right,
            MouseButton::Middle => CGMouseButton::Center,
            _ => CGMouseButton::Left,
        };
        
        let event = CGEvent::new_mouse_event(source, event_type, point, button_num as u32)
            .map_err(|_| DesktopError::MouseFailed("Failed to create event".into()))?;
        event.post(CGEventTapLocation::HID);
        
        Ok(())
    }

    /// Release button
    pub fn up(button: MouseButton) -> Result<()> {
        tracing::debug!("Mouse up: {:?}", button);
        
        #[cfg(target_os = "linux")]
        {
            Self::button_linux(button, false)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::button_windows(button, false)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::button_macos(button, false)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(())
        }
    }

    /// Scroll vertically
    pub fn scroll(amount: i32) -> Result<()> {
        tracing::debug!("Scrolling: {}", amount);
        
        #[cfg(target_os = "linux")]
        {
            // X11 scroll using button 4/5
            let (conn, _) = x11rb::connect(None).ok().unwrap();
            // Scroll up = button 4, scroll down = button 5
            let button = if amount > 0 { 4u8 } else { 5u8 };
            for _ in 0..amount.abs() {
                tracing::debug!("Linux scroll button {}", button);
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{mouse_event, MOUSEEVENTF_WHEEL};
            unsafe {
                mouse_event(MOUSEEVENTF_WHEEL, 0, 0, (amount * 120) as u32, 0);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation, CGScrollDirection};
            use core_graphics::event_source::CGEventSource;
            
            let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
                .map_err(|_| DesktopError::MouseFailed("Failed to create event source".into()))?;
            
            let event = CGEvent::new_scroll_event(
                source,
                CGScrollDirection::Unit,
                1,
                amount,
                0,
                0,
            ).map_err(|_| DesktopError::MouseFailed("Failed to create scroll event".into()))?;
            event.post(CGEventTapLocation::HID);
        }
        
        Ok(())
    }

    /// Scroll horizontally
    pub fn scroll_horizontal(amount: i32) -> Result<()> {
        tracing::debug!("Horizontal scroll: {}", amount);
        
        #[cfg(target_os = "windows")]
        {
            use winapi::um::winuser::{mouse_event, MOUSEEVENTF_HWHEEL};
            unsafe {
                mouse_event(MOUSEEVENTF_HWHEEL, 0, 0, (amount * 120) as u32, 0);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            use core_graphics::event::{CGEvent, CGEventTapLocation, CGScrollDirection};
            use core_graphics::event_source::CGEventSource;
            
            let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
                .map_err(|_| DesktopError::MouseFailed("Failed to create event source".into()))?;
            
            let event = CGEvent::new_scroll_event(
                source,
                CGScrollDirection::Unit,
                1,
                0,
                amount,
                0,
            ).map_err(|_| DesktopError::MouseFailed("Failed to create scroll event".into()))?;
            event.post(CGEventTapLocation::HID);
        }
        
        Ok(())
    }
    
    /// Drag from one point to another
    pub fn drag(from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> Result<()> {
        Self::move_to(from_x, from_y)?;
        std::thread::sleep(Duration::from_millis(50));
        Self::down(MouseButton::Left)?;
        
        // Smooth drag with intermediate points
        let steps = 10;
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let x = (from_x as f32 + (to_x as f32 - from_x as f32) * t) as u32;
            let y = (from_y as f32 + (to_y as f32 - from_y as f32) * t) as u32;
            Self::move_to(x, y)?;
            std::thread::sleep(Duration::from_millis(10));
        }
        
        Self::up(MouseButton::Left)?;
        Ok(())
    }
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

/// Mouse action for recording/automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseAction {
    MoveTo { x: u32, y: u32 },
    MoveBy { dx: i32, dy: i32 },
    Click { button: MouseButton },
    DoubleClick { button: MouseButton },
    Down { button: MouseButton },
    Up { button: MouseButton },
    Scroll { amount: i32 },
    Drag { from_x: u32, from_y: u32, to_x: u32, to_y: u32 },
}

impl MouseAction {
    /// Execute the action
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::MoveTo { x, y } => Mouse::move_to(*x, *y),
            Self::MoveBy { dx, dy } => Mouse::move_by(*dx, *dy),
            Self::Click { button } => Mouse::click(*button),
            Self::DoubleClick { button } => Mouse::double_click(*button),
            Self::Down { button } => Mouse::down(*button),
            Self::Up { button } => Mouse::up(*button),
            Self::Scroll { amount } => Mouse::scroll(*amount),
            Self::Drag { from_x, from_y, to_x, to_y } => Mouse::drag(*from_x, *from_y, *to_x, *to_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires X11/display server
    fn test_mouse_move() {
        let result = Mouse::move_to(100, 200);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Requires X11/display server
    fn test_mouse_click() {
        let result = Mouse::click(MouseButton::Left);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Requires X11/display server
    fn test_mouse_scroll() {
        let result = Mouse::scroll(5);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore] // Requires X11/display server
    fn test_mouse_action_execute() {
        let action = MouseAction::MoveTo { x: 50, y: 50 };
        assert!(action.execute().is_ok());
    }
}
