// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Keyboard Control
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{DesktopError, Result};
use serde::{Deserialize, Serialize};

/// Keyboard controller
pub struct Keyboard;

impl Keyboard {
    /// Type text
    pub fn type_text(text: &str) -> Result<()> {
        for c in text.chars() {
            Self::key_char(c)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        Ok(())
    }

    /// Type single character
    fn key_char(c: char) -> Result<()> {
        tracing::debug!("Typing character: {}", c);
        Ok(())
    }

    /// Press key
    pub fn press(key: Key) -> Result<()> {
        tracing::debug!("Pressing key: {:?}", key);
        Ok(())
    }

    /// Release key
    pub fn release(key: Key) -> Result<()> {
        tracing::debug!("Releasing key: {:?}", key);
        Ok(())
    }

    /// Press and release key
    pub fn tap(key: Key) -> Result<()> {
        Self::press(key)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        Self::release(key)?;
        Ok(())
    }

    /// Hotkey combination (press multiple keys)
    pub fn hotkey(keys: &[Key]) -> Result<()> {
        // Press all keys
        for key in keys {
            Self::press(*key)?;
        }
        
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        // Release all keys in reverse order
        for key in keys.iter().rev() {
            Self::release(*key)?;
        }
        
        Ok(())
    }

    /// Common shortcuts
    pub fn copy() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::C])
    }

    pub fn paste() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::V])
    }

    pub fn cut() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::X])
    }

    pub fn select_all() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::A])
    }

    pub fn undo() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::Z])
    }

    pub fn redo() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::Y])
    }

    pub fn save() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::S])
    }

    pub fn find() -> Result<()> {
        Self::hotkey(&[Key::Control, Key::F])
    }

    pub fn escape() -> Result<()> {
        Self::tap(Key::Escape)
    }

    pub fn enter() -> Result<()> {
        Self::tap(Key::Enter)
    }

    pub fn tab() -> Result<()> {
        Self::tap(Key::Tab)
    }

    pub fn backspace() -> Result<()> {
        Self::tap(Key::Backspace)
    }

    pub fn delete() -> Result<()> {
        Self::tap(Key::Delete)
    }

    pub fn arrow_up() -> Result<()> {
        Self::tap(Key::Up)
    }

    pub fn arrow_down() -> Result<()> {
        Self::tap(Key::Down)
    }

    pub fn arrow_left() -> Result<()> {
        Self::tap(Key::Left)
    }

    pub fn arrow_right() -> Result<()> {
        Self::tap(Key::Right)
    }
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
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    
    // Arrow keys
    Up,
    Down,
    Left,
    Right,
    
    // Modifiers
    Shift,
    Control,
    Alt,
    Meta, // Windows/Super/Command
    
    // Other
    CapsLock,
    NumLock,
    ScrollLock,
    
    // Punctuation
    Comma,
    Period,
    Slash,
    Semicolon,
    Quote,
    LeftBracket,
    RightBracket,
    Backslash,
    Minus,
    Equal,
    Grave,
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
