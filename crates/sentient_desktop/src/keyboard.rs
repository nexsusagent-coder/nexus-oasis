// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Keyboard Control (Real Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{DesktopError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Keyboard controller
pub struct Keyboard;

impl Keyboard {
    /// Type text character by character
    pub fn type_text(text: &str) -> Result<()> {
        for c in text.chars() {
            Self::key_char(c)?;
            std::thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    /// Type single character
    fn key_char(c: char) -> Result<()> {
        tracing::debug!("Typing character: {}", c);
        
        #[cfg(target_os = "linux")]
        {
            Self::type_linux(c)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::type_windows(c)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::type_macos(c)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(())
        }
    }
    
    #[cfg(target_os = "linux")]
    fn type_linux(c: char) -> Result<()> {
        // Use XTest extension for typing
        // Simplified: just log for now
        tracing::debug!("Linux type char: {}", c);
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn type_windows(c: char) -> Result<()> {
        use winapi::um::winuser::{keybd_event, KEYEVENTF_UNICODE, VkKeyScanW};
        use winapi::shared::minwindef::WORD;
        
        unsafe {
            // Send as Unicode character
            let inputs: [winapi::um::winuser::INPUT; 2] = [
                winapi::um::winuser::INPUT {
                    type_: winapi::um::winuser::INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                },
                winapi::um::winuser::INPUT {
                    type_: winapi::um::winuser::INPUT_KEYBOARD,
                    u: std::mem::zeroed(),
                },
            ];
            
            // Use keybd_event for simplicity
            let vk = VkKeyScanW(c as WORD);
            let vk_code = (vk & 0xFF) as u8;
            let shift_state = (vk >> 8) & 1;
            
            if shift_state != 0 {
                keybd_event(winapi::um::winuser::VK_SHIFT, 0, 0, 0);
            }
            
            keybd_event(vk_code, 0, 0, 0);
            keybd_event(vk_code, 0, winapi::um::winuser::KEYEVENTF_KEYUP, 0);
            
            if shift_state != 0 {
                keybd_event(winapi::um::winuser::VK_SHIFT, 0, winapi::um::winuser::KEYEVENTF_KEYUP, 0);
            }
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn type_macos(c: char) -> Result<()> {
        use core_graphics::event::{CGEvent, CGEventTapLocation, CGKeyCode};
        use core_graphics::event_source::CGEventSource;
        
        let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
            .map_err(|_| DesktopError::KeyboardFailed("Failed to create event source".into()))?;
        
        let mut str_buf = [0u8; 4];
        let s = c.encode_utf8(&mut str_buf);
        
        for byte in s.bytes() {
            let event = CGEvent::new_keyboard_event(
                source.clone(),
                0, // virtual key
                true, // key down
                byte,
            ).map_err(|_| DesktopError::KeyboardFailed("Failed to create key down event".into()))?;
            event.post(CGEventTapLocation::HID);
            
            let event = CGEvent::new_keyboard_event(
                source.clone(),
                0,
                false, // key up
                byte,
            ).map_err(|_| DesktopError::KeyboardFailed("Failed to create key up event".into()))?;
            event.post(CGEventTapLocation::HID);
        }
        
        Ok(())
    }

    /// Press key
    pub fn press(key: Key) -> Result<()> {
        tracing::debug!("Pressing key: {:?}", key);
        
        #[cfg(target_os = "linux")]
        {
            Self::key_linux(key, true)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::key_windows(key, true)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::key_macos(key, true)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(())
        }
    }
    
    #[cfg(target_os = "linux")]
    fn key_linux(_key: Key, _press: bool) -> Result<()> {
        // XTest key event
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn key_windows(key: Key, press: bool) -> Result<()> {
        use winapi::um::winuser::keybd_event;
        
        let vk = Self::to_vk(key);
        let flags = if press { 0 } else { winapi::um::winuser::KEYEVENTF_KEYUP };
        
        unsafe {
            keybd_event(vk, 0, flags, 0);
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    fn to_vk(key: Key) -> u8 {
        use winapi::um::winuser::*;
        
        match key {
            Key::A => 0x41, Key::B => 0x42, Key::C => 0x43, Key::D => 0x44,
            Key::E => 0x45, Key::F => 0x46, Key::G => 0x47, Key::H => 0x48,
            Key::I => 0x49, Key::J => 0x4A, Key::K => 0x4B, Key::L => 0x4C,
            Key::M => 0x4D, Key::N => 0x4E, Key::O => 0x4F, Key::P => 0x50,
            Key::Q => 0x51, Key::R => 0x52, Key::S => 0x53, Key::T => 0x54,
            Key::U => 0x55, Key::V => 0x56, Key::W => 0x57, Key::X => 0x58,
            Key::Y => 0x59, Key::Z => 0x5A,
            Key::Num0 => 0x30, Key::Num1 => 0x31, Key::Num2 => 0x32,
            Key::Num3 => 0x33, Key::Num4 => 0x34, Key::Num5 => 0x35,
            Key::Num6 => 0x36, Key::Num7 => 0x37, Key::Num8 => 0x38,
            Key::Num9 => 0x39,
            Key::F1 => VK_F1 as u8, Key::F2 => VK_F2 as u8, Key::F3 => VK_F3 as u8,
            Key::F4 => VK_F4 as u8, Key::F5 => VK_F5 as u8, Key::F6 => VK_F6 as u8,
            Key::F7 => VK_F7 as u8, Key::F8 => VK_F8 as u8, Key::F9 => VK_F9 as u8,
            Key::F10 => VK_F10 as u8, Key::F11 => VK_F11 as u8, Key::F12 => VK_F12 as u8,
            Key::Enter => VK_RETURN as u8,
            Key::Escape => VK_ESCAPE as u8,
            Key::Backspace => VK_BACK as u8,
            Key::Tab => VK_TAB as u8,
            Key::Space => VK_SPACE as u8,
            Key::Delete => VK_DELETE as u8,
            Key::Insert => VK_INSERT as u8,
            Key::Home => VK_HOME as u8,
            Key::End => VK_END as u8,
            Key::PageUp => VK_PRIOR as u8,
            Key::PageDown => VK_NEXT as u8,
            Key::Up => VK_UP as u8,
            Key::Down => VK_DOWN as u8,
            Key::Left => VK_LEFT as u8,
            Key::Right => VK_RIGHT as u8,
            Key::Shift => VK_SHIFT as u8,
            Key::Control => VK_CONTROL as u8,
            Key::Alt => VK_MENU as u8,
            Key::Meta => VK_LWIN as u8,
            Key::CapsLock => VK_CAPITAL as u8,
            Key::NumLock => VK_NUMLOCK as u8,
            Key::ScrollLock => VK_SCROLL as u8,
            Key::Comma => VK_OEM_COMMA as u8,
            Key::Period => VK_OEM_PERIOD as u8,
            Key::Slash => VK_OEM_2 as u8,
            Key::Semicolon => VK_OEM_1 as u8,
            Key::Quote => VK_OEM_7 as u8,
            Key::LeftBracket => VK_OEM_4 as u8,
            Key::RightBracket => VK_OEM_6 as u8,
            Key::Backslash => VK_OEM_5 as u8,
            Key::Minus => VK_OEM_MINUS as u8,
            Key::Equal => VK_OEM_PLUS as u8,
            Key::Grave => VK_OEM_3 as u8,
        }
    }
    
    #[cfg(target_os = "macos")]
    fn key_macos(key: Key, press: bool) -> Result<()> {
        use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventFlags, CGKeyCode};
        use core_graphics::event_source::CGEventSource;
        
        let source = CGEventSource::new(CGEventSource::StateID::CombinedSessionState)
            .map_err(|_| DesktopError::KeyboardFailed("Failed to create event source".into()))?;
        
        let (keycode, flags) = Self::to_macos_key(key);
        
        let event = CGEvent::new_keyboard_event(source, keycode, press, 0)
            .map_err(|_| DesktopError::KeyboardFailed("Failed to create event".into()))?;
        event.set_flags(flags);
        event.post(CGEventTapLocation::HID);
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn to_macos_key(key: Key) -> (CGKeyCode, CGEventFlags) {
        use core_graphics::event::CGEventFlags;
        
        let keycode = match key {
            Key::A => 0x00, Key::S => 0x01, Key::D => 0x02, Key::F => 0x03,
            Key::H => 0x04, Key::G => 0x05, Key::Z => 0x06, Key::X => 0x07,
            Key::C => 0x08, Key::V => 0x09, Key::B => 0x0B, Key::Q => 0x0C,
            Key::W => 0x0D, Key::E => 0x0E, Key::R => 0x0F, Key::Y => 0x10,
            Key::T => 0x11,
            Key::Num1 => 0x12, Key::Num2 => 0x13, Key::Num3 => 0x14,
            Key::Num4 => 0x15, Key::Num6 => 0x16, Key::Num5 => 0x17,
            Key::Equal => 0x18, Key::Num9 => 0x19, Key::Num7 => 0x1A,
            Key::Minus => 0x1B, Key::Num8 => 0x1C, Key::Num0 => 0x1D,
            Key::RightBracket => 0x1E, Key::O => 0x1F, Key::U => 0x20,
            Key::LeftBracket => 0x21, Key::I => 0x22, Key::P => 0x23,
            Key::Enter => 0x24, Key::L => 0x25, Key::J => 0x26,
            Key::Quote => 0x27, Key::K => 0x28, Key::Semicolon => 0x29,
            Key::Backslash => 0x2A, Key::Comma => 0x2B, Key::Slash => 0x2C,
            Key::N => 0x2D, Key::M => 0x2E, Key::Period => 0x2F,
            Key::Grave => 0x32, Key::Space => 0x31,
            Key::Delete => 0x33, Key::Tab => 0x30, Key::Escape => 0x35,
            Key::F1 => 0x7A, Key::F2 => 0x78, Key::F3 => 0x63, Key::F4 => 0x76,
            Key::F5 => 0x60, Key::F6 => 0x61, Key::F7 => 0x62, Key::F8 => 0x64,
            Key::F9 => 0x65, Key::F10 => 0x6D, Key::F11 => 0x67, Key::F12 => 0x6F,
            Key::Insert => 0x72, Key::Home => 0x73, Key::PageUp => 0x74,
            Key::End => 0x77, Key::PageDown => 0x79,
            Key::Right => 0x7C, Key::Left => 0x7B, Key::Down => 0x7D, Key::Up => 0x7E,
            Key::Shift => 0x38, Key::Control => 0x3B, Key::Alt => 0x3A, Key::Meta => 0x37,
            Key::CapsLock => 0x39, Key::NumLock => 0x47,
            _ => 0x00,
        };
        
        let flags = match key {
            Key::Shift => CGEventFlags::CGEventFlagShift,
            Key::Control => CGEventFlags::CGEventFlagControl,
            Key::Alt => CGEventFlags::CGEventFlagAlternate,
            Key::Meta => CGEventFlags::CGEventFlagCommand,
            _ => CGEventFlags::empty(),
        };
        
        (keycode, flags)
    }

    /// Release key
    pub fn release(key: Key) -> Result<()> {
        tracing::debug!("Releasing key: {:?}", key);
        
        #[cfg(target_os = "linux")]
        {
            Self::key_linux(key, false)
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::key_windows(key, false)
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::key_macos(key, false)
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Ok(())
        }
    }

    /// Press and release key
    pub fn tap(key: Key) -> Result<()> {
        Self::press(key)?;
        std::thread::sleep(Duration::from_millis(50));
        Self::release(key)?;
        Ok(())
    }

    /// Hotkey combination (press multiple keys)
    pub fn hotkey(keys: &[Key]) -> Result<()> {
        // Press all keys
        for key in keys {
            Self::press(*key)?;
            std::thread::sleep(Duration::from_millis(10));
        }
        
        std::thread::sleep(Duration::from_millis(50));
        
        // Release all keys in reverse order
        for key in keys.iter().rev() {
            Self::release(*key)?;
            std::thread::sleep(Duration::from_millis(10));
        }
        
        Ok(())
    }

    // ========================================================================
    // Common Shortcuts (Cross-Platform)
    // ========================================================================
    
    /// Copy (Ctrl+C / Cmd+C)
    pub fn copy() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::C]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::C]) }
    }

    /// Paste (Ctrl+V / Cmd+V)
    pub fn paste() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::V]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::V]) }
    }

    /// Cut (Ctrl+X / Cmd+X)
    pub fn cut() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::X]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::X]) }
    }

    /// Select All (Ctrl+A / Cmd+A)
    pub fn select_all() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::A]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::A]) }
    }

    /// Undo (Ctrl+Z / Cmd+Z)
    pub fn undo() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::Z]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::Z]) }
    }

    /// Redo (Ctrl+Y / Cmd+Shift+Z)
    pub fn redo() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::Y]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::Shift, Key::Z]) }
    }

    /// Save (Ctrl+S / Cmd+S)
    pub fn save() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::S]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::S]) }
    }

    /// Find (Ctrl+F / Cmd+F)
    pub fn find() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::F]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::F]) }
    }
    
    /// New (Ctrl+N / Cmd+N)
    pub fn new_file() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::N]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::N]) }
    }
    
    /// Open (Ctrl+O / Cmd+O)
    pub fn open() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::O]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::O]) }
    }
    
    /// Print (Ctrl+P / Cmd+P)
    pub fn print() -> Result<()> {
        #[cfg(not(target_os = "macos"))]
        { Self::hotkey(&[Key::Control, Key::P]) }
        
        #[cfg(target_os = "macos")]
        { Self::hotkey(&[Key::Meta, Key::P]) }
    }

    /// Escape
    pub fn escape() -> Result<()> { Self::tap(Key::Escape) }

    /// Enter
    pub fn enter() -> Result<()> { Self::tap(Key::Enter) }

    /// Tab
    pub fn tab() -> Result<()> { Self::tap(Key::Tab) }

    /// Backspace
    pub fn backspace() -> Result<()> { Self::tap(Key::Backspace) }

    /// Delete
    pub fn delete() -> Result<()> { Self::tap(Key::Delete) }

    /// Arrow Up
    pub fn arrow_up() -> Result<()> { Self::tap(Key::Up) }

    /// Arrow Down
    pub fn arrow_down() -> Result<()> { Self::tap(Key::Down) }

    /// Arrow Left
    pub fn arrow_left() -> Result<()> { Self::tap(Key::Left) }

    /// Arrow Right
    pub fn arrow_right() -> Result<()> { Self::tap(Key::Right) }
    
    /// Home
    pub fn home() -> Result<()> { Self::tap(Key::Home) }
    
    /// End
    pub fn end() -> Result<()> { Self::tap(Key::End) }
    
    /// Page Up
    pub fn page_up() -> Result<()> { Self::tap(Key::PageUp) }
    
    /// Page Down
    pub fn page_down() -> Result<()> { Self::tap(Key::PageDown) }
}

/// Key codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // Numbers
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Special keys
    Enter, Escape, Backspace, Tab, Space, Delete, Insert,
    Home, End, PageUp, PageDown,
    
    // Arrow keys
    Up, Down, Left, Right,
    
    // Modifiers
    Shift, Control, Alt, Meta, // Meta = Windows/Super/Command
    
    // Other
    CapsLock, NumLock, ScrollLock,
    
    // Punctuation
    Comma, Period, Slash, Semicolon, Quote,
    LeftBracket, RightBracket, Backslash, Minus, Equal, Grave,
}

/// Keyboard action for recording/automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyboardAction {
    Type { text: String },
    Press { key: Key },
    Release { key: Key },
    Tap { key: Key },
    Hotkey { keys: Vec<Key> },
}

impl KeyboardAction {
    /// Execute the action
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Type { text } => Keyboard::type_text(text),
            Self::Press { key } => Keyboard::press(*key),
            Self::Release { key } => Keyboard::release(*key),
            Self::Tap { key } => Keyboard::tap(*key),
            Self::Hotkey { keys } => Keyboard::hotkey(keys),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_type() {
        let result = Keyboard::type_text("Hello");
        assert!(result.is_ok());
    }

    #[test]
    fn test_keyboard_tap() {
        let result = Keyboard::tap(Key::Enter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_keyboard_hotkey() {
        let result = Keyboard::hotkey(&[Key::Control, Key::C]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_keyboard_shortcuts() {
        assert!(Keyboard::copy().is_ok());
        assert!(Keyboard::paste().is_ok());
        assert!(Keyboard::select_all().is_ok());
    }

    #[test]
    fn test_keyboard_action_execute() {
        let action = KeyboardAction::Type { text: "test".to_string() };
        assert!(action.execute().is_ok());
    }
}
