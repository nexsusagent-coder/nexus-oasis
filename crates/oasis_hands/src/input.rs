//! ═══════════════════════════════════════════════════════════════════════════════
//!  INPUT CONTROL - FARE VE KLAVYE KONTROLÜ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü GUI kontrolü için fare ve klavye girişi yönetimi.

use crate::error::{HandsError, HandsResult};
use crate::sovereign::SovereignPolicy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ───────────────────────────────────────────────────────────────────────────────
//  MOUSE KONTROLÜ
// ─────────────────────────────────────────────────────────────────────────────--

/// Fare butonları
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

impl MouseButton {
    pub fn as_str(&self) -> &'static str {
        match self {
            MouseButton::Left => "left",
            MouseButton::Right => "right",
            MouseButton::Middle => "middle",
            MouseButton::Back => "back",
            MouseButton::Forward => "forward",
        }
    }
}

/// Fare aksiyonları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseAction {
    /// Fareyi hareket ettir
    Move { x: i32, y: i32 },
    /// Yumuşak hareket (animasyonlu)
    MoveSmooth { x: i32, y: i32, duration_ms: u32 },
    /// Tıklama
    Click { button: MouseButton },
    /// Çift tıklama
    DoubleClick { button: MouseButton },
    /// Basılı tut
    Down { button: MouseButton },
    /// Bırak
    Up { button: MouseButton },
    /// Sürükle (bas → hareket → bırak)
    Drag { 
        from_x: i32, from_y: i32, 
        to_x: i32, to_y: i32,
    },
    /// Kaydırma
    Scroll { 
        delta_x: i32, delta_y: i32,
    },
    /// Mevcut pozisyonu al
    GetPosition,
}

impl MouseAction {
    /// X koordinatını getir (varsa)
    pub fn x(&self) -> Option<i32> {
        match self {
            MouseAction::Move { x, .. } => Some(*x),
            MouseAction::MoveSmooth { x, .. } => Some(*x),
            MouseAction::Drag { to_x, .. } => Some(*to_x),
            _ => None,
        }
    }
    
    /// Y koordinatını getir (varsa)
    pub fn y(&self) -> Option<i32> {
        match self {
            MouseAction::Move { y, .. } => Some(*y),
            MouseAction::MoveSmooth { y, .. } => Some(*y),
            MouseAction::Drag { to_y, .. } => Some(*to_y),
            _ => None,
        }
    }
    
    /// Aksiyon açıklaması
    pub fn description(&self) -> String {
        match self {
            MouseAction::Move { x, y } => format!("Fare hareket: ({}, {})", x, y),
            MouseAction::MoveSmooth { x, y, duration_ms } => 
                format!("Yumuşak fare hareket: ({}, {}) {}ms", x, y, duration_ms),
            MouseAction::Click { button } => format!("Tıklama: {:?}", button),
            MouseAction::DoubleClick { button } => format!("Çift tıklama: {:?}", button),
            MouseAction::Down { button } => format!("Basılı: {:?}", button),
            MouseAction::Up { button } => format!("Bırak: {:?}", button),
            MouseAction::Drag { from_x, from_y, to_x, to_y } => 
                format!("Sürükle: ({}, {}) → ({}, {})", from_x, from_y, to_x, to_y),
            MouseAction::Scroll { delta_x, delta_y } => 
                format!("Kaydır: x={}, y={}", delta_x, delta_y),
            MouseAction::GetPosition => "Pozisyon al".into(),
        }
    }
}

/// Fare pozisyonu
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
}

// ───────────────────────────────────────────────────────────────────────────────
//  KLAVYE KONTROLÜ
// ─────────────────────────────────────────────────────────────────────────────--

/// Özel tuşlar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    // Fonksiyon tuşları
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // Yön tuşları
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    
    // Kontrol tuşları
    Escape, Enter, Tab, Backspace, Delete, Insert,
    Home, End, PageUp, PageDown,
    
    // Modifier tuşları
    Shift, Ctrl, Alt, Super, // Super = Windows/Command
    
    // Diğer
    Space, CapsLock, NumLock, ScrollLock,
    
    // Numpad
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    
    // Karakter
    Char(char),
}

impl Key {
    pub fn code(&self) -> u32 {
        match self {
            Key::F1 => 0xFFBE,
            Key::F2 => 0xFFBF,
            Key::F3 => 0xFFC0,
            Key::F4 => 0xFFC1,
            Key::F5 => 0xFFC2,
            Key::F6 => 0xFFC3,
            Key::F7 => 0xFFC4,
            Key::F8 => 0xFFC5,
            Key::F9 => 0xFFC6,
            Key::F10 => 0xFFC7,
            Key::F11 => 0xFFC8,
            Key::F12 => 0xFFC9,
            Key::ArrowUp => 0xFF52,
            Key::ArrowDown => 0xFF54,
            Key::ArrowLeft => 0xFF51,
            Key::ArrowRight => 0xFF53,
            Key::Escape => 0xFF1B,
            Key::Enter => 0xFF0D,
            Key::Tab => 0xFF09,
            Key::Backspace => 0xFF08,
            Key::Delete => 0xFFFF,
            Key::Insert => 0xFF63,
            Key::Home => 0xFF50,
            Key::End => 0xFF57,
            Key::PageUp => 0xFF55,
            Key::PageDown => 0xFF56,
            Key::Space => 0x0020,
            Key::Char(c) => *c as u32,
            _ => 0,
        }
    }
}

/// Klavye aksiyonları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyboardAction {
    /// Tek tuş bas
    KeyPress { key: Key },
    /// Tuş basılı tut
    KeyDown { key: Key },
    /// Tuş bırak
    KeyUp { key: Key },
    /// Kısayol (modifier + key)
    Shortcut { 
        modifiers: Vec<Key>, 
        key: Key,
    },
    /// Metin yaz
    TypeText { 
        text: String,
        typing_speed_ms: Option<u32>,
    },
    /// Kopyala (Ctrl+C)
    Copy,
    /// Yapıştır (Ctrl+V)
    Paste,
    /// Kes (Ctrl+X)
    Cut,
    /// Geri al (Ctrl+Z)
    Undo,
    /// Yinele (Ctrl+Y)
    Redo,
    /// Tümünü seç (Ctrl+A)
    SelectAll,
}

impl KeyboardAction {
    /// Metin içeriğini getir (varsa)
    pub fn text(&self) -> Option<&str> {
        match self {
            KeyboardAction::TypeText { text, .. } => Some(text),
            _ => None,
        }
    }
    
    /// Aksiyon açıklaması
    pub fn description(&self) -> String {
        match self {
            KeyboardAction::KeyPress { key } => format!("Tuş bas: {:?}", key),
            KeyboardAction::KeyDown { key } => format!("Tuş basılı: {:?}", key),
            KeyboardAction::KeyUp { key } => format!("Tuş bırak: {:?}", key),
            KeyboardAction::Shortcut { modifiers, key } => {
                let mods: String = modifiers.iter()
                    .map(|k| format!("{:?}", k))
                    .collect::<Vec<_>>()
                    .join("+");
                format!("Kısayol: {}+{:?}", mods, key)
            }
            KeyboardAction::TypeText { text, .. } => {
                let preview = text.chars().take(50).collect::<String>();
                format!("Metin yaz: \"{}\"", preview)
            }
            KeyboardAction::Copy => "Kopyala".into(),
            KeyboardAction::Paste => "Yapıştır".into(),
            KeyboardAction::Cut => "Kes".into(),
            KeyboardAction::Undo => "Geri al".into(),
            KeyboardAction::Redo => "Yinele".into(),
            KeyboardAction::SelectAll => "Tümünü seç".into(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  INPUT CONTROLLER
// ─────────────────────────────────────────────────────────────────────────────--

/// Giriş kontrolcüsü
pub struct InputController {
    /// Sovereign policy referansı
    policy: SovereignPolicy,
    /// Mevcut fare pozisyonu
    mouse_position: MousePosition,
    /// Basılı tuşlar
    pressed_keys: Vec<Key>,
    /// Basılı fare butonları
    pressed_buttons: Vec<MouseButton>,
    /// Aktif mi?
    active: bool,
    /// Varsayılan yazma hızı (ms/karakter)
    default_typing_speed: u32,
}

impl InputController {
    /// Yeni kontrolcü oluştur
    pub fn new(policy: SovereignPolicy) -> HandsResult<Self> {
        log::info!("🖱️  INPUT: Giriş kontrolcü başlatıldı");
        
        Ok(Self {
            policy,
            mouse_position: MousePosition { x: 0, y: 0 },
            pressed_keys: Vec::new(),
            pressed_buttons: Vec::new(),
            active: true,
            default_typing_speed: 50,
        })
    }
    
    /// Fare aksiyonu çalıştır
    pub async fn execute_mouse(&mut self, action: MouseAction) -> HandsResult<()> {
        if !self.active {
            return Err(HandsError::InputError("Kontrolcü aktif değil".into()));
        }
        
        // Sovereign kontrolü
        self.policy.validate_mouse_action(&action)?;
        
        log::debug!("🖱️  INPUT: Fare aksiyonu → {}", action.description());
        
        match action {
            MouseAction::Move { x, y } => {
                self.mouse_position = MousePosition { x, y };
                // Gerçek uygulamada enigo/x11 kullanılır
                log::info!("🖱️  INPUT: Fare hareket ({}, {})", x, y);
            }
            MouseAction::MoveSmooth { x, y, duration_ms } => {
                // Animasyonlu hareket
                self.animate_mouse_move(x, y, duration_ms).await?;
            }
            MouseAction::Click { button } => {
                self.pressed_buttons.push(button);
                // Tıklama simülasyonu
                self.pressed_buttons.retain(|b| b != &button);
                log::info!("🖱️  INPUT: Tıklama ({:?})", button);
            }
            MouseAction::DoubleClick { button } => {
                log::info!("🖱️  INPUT: Çift tıklama ({:?})", button);
            }
            MouseAction::Down { button } => {
                self.pressed_buttons.push(button);
            }
            MouseAction::Up { button } => {
                self.pressed_buttons.retain(|b| b != &button);
            }
            MouseAction::Drag { from_x, from_y, to_x, to_y } => {
                self.mouse_position = MousePosition { x: from_x, y: from_y };
                self.pressed_buttons.push(MouseButton::Left);
                tokio::time::sleep(Duration::from_millis(100)).await;
                self.mouse_position = MousePosition { x: to_x, y: to_y };
                self.pressed_buttons.retain(|b| b != &MouseButton::Left);
                log::info!("🖱️  INPUT: Sürükle ({}, {}) → ({}, {})", from_x, from_y, to_x, to_y);
            }
            MouseAction::Scroll { delta_x, delta_y } => {
                log::info!("🖱️  INPUT: Kaydırma (x={}, y={})", delta_x, delta_y);
            }
            MouseAction::GetPosition => {
                // Pozisyon zaten tracked
            }
        }
        
        Ok(())
    }
    
    /// Klavye aksiyonu çalıştır
    pub async fn execute_keyboard(&mut self, action: KeyboardAction) -> HandsResult<()> {
        if !self.active {
            return Err(HandsError::InputError("Kontrolcü aktif değil".into()));
        }
        
        // Sovereign kontrolü
        self.policy.validate_keyboard_action(&action)?;
        
        log::debug!("⌨️  INPUT: Klavye aksiyonu → {}", action.description());
        
        match action {
            KeyboardAction::KeyPress { key } => {
                self.simulate_key_press(key).await?;
            }
            KeyboardAction::KeyDown { key } => {
                if !self.pressed_keys.contains(&key) {
                    self.pressed_keys.push(key);
                }
            }
            KeyboardAction::KeyUp { key } => {
                self.pressed_keys.retain(|k| k != &key);
            }
            KeyboardAction::Shortcut { ref modifiers, key } => {
                // Modifierları bas
                for m in modifiers {
                    self.pressed_keys.push(*m);
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
                // Ana tuşu bas
                self.simulate_key_press(key).await?;
                tokio::time::sleep(Duration::from_millis(50)).await;
                // Modifierları bırak
                for m in modifiers {
                    self.pressed_keys.retain(|k| k != m);
                }
                log::info!("⌨️  INPUT: Kısayol {:?}", action.description());
            }
            KeyboardAction::TypeText { text, typing_speed_ms } => {
                let speed = typing_speed_ms.unwrap_or(self.default_typing_speed);
                for c in text.chars() {
                    self.simulate_key_press(Key::Char(c)).await?;
                    tokio::time::sleep(Duration::from_millis(speed as u64)).await;
                }
                log::info!("⌨️  INPUT: Metin yazıldı ({} karakter)", text.len());
            }
            KeyboardAction::Copy => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('c')).await?;
            }
            KeyboardAction::Paste => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('v')).await?;
            }
            KeyboardAction::Cut => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('x')).await?;
            }
            KeyboardAction::Undo => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('z')).await?;
            }
            KeyboardAction::Redo => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('y')).await?;
            }
            KeyboardAction::SelectAll => {
                self.execute_shortcut(vec![Key::Ctrl], Key::Char('a')).await?;
            }
        }
        
        Ok(())
    }
    
    /// Animasyonlu fare hareketi
    async fn animate_mouse_move(&mut self, to_x: i32, to_y: i32, duration_ms: u32) -> HandsResult<()> {
        let from_x = self.mouse_position.x;
        let from_y = self.mouse_position.y;
        
        let steps = (duration_ms / 16) as usize; // ~60fps
        let dx = (to_x - from_x) as f32 / steps as f32;
        let dy = (to_y - from_y) as f32 / steps as f32;
        
        for i in 0..steps {
            let x = (from_x as f32 + dx * i as f32) as i32;
            let y = (from_y as f32 + dy * i as f32) as i32;
            self.mouse_position = MousePosition { x, y };
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
        
        self.mouse_position = MousePosition { x: to_x, y: to_y };
        Ok(())
    }
    
    /// Tuş basma simülasyonu
    async fn simulate_key_press(&mut self, key: Key) -> HandsResult<()> {
        self.pressed_keys.push(key);
        tokio::time::sleep(Duration::from_millis(50)).await;
        self.pressed_keys.retain(|k| k != &key);
        Ok(())
    }
    
    /// Kısayol çalıştır (recursive olmadan)
    async fn execute_shortcut(&mut self, modifiers: Vec<Key>, key: Key) -> HandsResult<()> {
        // Modifierları bas
        for m in &modifiers {
            self.pressed_keys.push(*m);
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        // Ana tuşu bas
        self.simulate_key_press(key).await?;
        tokio::time::sleep(Duration::from_millis(50)).await;
        // Modifierları bırak
        for m in &modifiers {
            self.pressed_keys.retain(|k| k != m);
        }
        log::info!("⌨️  INPUT: Kısayol Ctrl+{:?}", key);
        Ok(())
    }
    
    /// Mevcut fare pozisyonu
    pub fn mouse_position(&self) -> MousePosition {
        self.mouse_position
    }
    
    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }
    
    /// Durdur
    pub fn stop(&mut self) {
        // Tüm tuşları bırak
        self.pressed_keys.clear();
        self.pressed_buttons.clear();
        self.active = false;
        log::info!("🖱️  INPUT: Kontrolcü durduruldu");
    }
    
    /// Başlat
    pub fn start(&mut self) {
        self.active = true;
        log::info!("🖱️  INPUT: Kontrolcü başlatıldı");
    }
    
    /// Basılı tuşları temizle (acil durum)
    pub fn emergency_release(&mut self) {
        self.pressed_keys.clear();
        self.pressed_buttons.clear();
        log::warn!("⚠️  INPUT: Acil durum tuş bırakma!");
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mouse_button_as_str() {
        assert_eq!(MouseButton::Left.as_str(), "left");
        assert_eq!(MouseButton::Right.as_str(), "right");
    }
    
    #[test]
    fn test_mouse_action_coordinates() {
        let action = MouseAction::Move { x: 100, y: 200 };
        assert_eq!(action.x(), Some(100));
        assert_eq!(action.y(), Some(200));
    }
    
    #[test]
    fn test_mouse_action_description() {
        let action = MouseAction::Click { button: MouseButton::Left };
        assert!(action.description().contains("Tıklama"));
    }
    
    #[test]
    fn test_key_code() {
        assert_eq!(Key::F1.code(), 0xFFBE);
        assert_eq!(Key::Enter.code(), 0xFF0D);
        assert_eq!(Key::Char('a').code(), 'a' as u32);
    }
    
    #[test]
    fn test_keyboard_action_text() {
        let action = KeyboardAction::TypeText { 
            text: "Hello World".into(), 
            typing_speed_ms: Some(50) 
        };
        assert_eq!(action.text(), Some("Hello World"));
    }
    
    #[test]
    fn test_keyboard_action_description() {
        let action = KeyboardAction::Copy;
        assert_eq!(action.description(), "Kopyala");
    }
    
    #[tokio::test]
    async fn test_input_controller_creation() {
        let policy = SovereignPolicy::strict();
        let controller = InputController::new(policy).unwrap();
        assert!(controller.is_active());
    }
    
    #[tokio::test]
    async fn test_mouse_move() {
        let policy = SovereignPolicy::strict();
        let mut controller = InputController::new(policy).unwrap();
        
        let action = MouseAction::Move { x: 500, y: 300 };
        controller.execute_mouse(action).await.unwrap();
        
        let pos = controller.mouse_position();
        assert_eq!(pos.x, 500);
        assert_eq!(pos.y, 300);
    }
    
    #[tokio::test]
    async fn test_emergency_release() {
        let policy = SovereignPolicy::strict();
        let mut controller = InputController::new(policy).unwrap();
        
        controller.pressed_keys.push(Key::Ctrl);
        controller.pressed_buttons.push(MouseButton::Left);
        
        controller.emergency_release();
        
        assert!(controller.pressed_keys.is_empty());
        assert!(controller.pressed_buttons.is_empty());
    }
}
