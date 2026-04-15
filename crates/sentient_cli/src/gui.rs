//! ═══════════════════════════════════════════════════════════════════════════════
//!  CLI GUI Mode - Terminal User Interface
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Rich terminal interface for CLI:
//! - Interactive menus
//! - Progress indicators
//! - Color themes
//! - Keyboard navigation

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  GUI TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text: Color,
    pub error: Color,
    pub success: Color,
    pub warning: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Magenta,
            background: Color::Black,
            text: Color::White,
            error: Color::Red,
            success: Color::Green,
            warning: Color::Yellow,
        }
    }
}

/// ANSI Color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB { r: u8, g: u8, b: u8 },
}

impl Color {
    pub fn to_ansi(&self, foreground: bool) -> String {
        let prefix = if foreground { 38 } else { 48 };
        match self {
            Color::Black => format!("\x1b[{}m", if foreground { 30 } else { 40 }),
            Color::Red => format!("\x1b[{}m", if foreground { 31 } else { 41 }),
            Color::Green => format!("\x1b[{}m", if foreground { 32 } else { 42 }),
            Color::Yellow => format!("\x1b[{}m", if foreground { 33 } else { 43 }),
            Color::Blue => format!("\x1b[{}m", if foreground { 34 } else { 44 }),
            Color::Magenta => format!("\x1b[{}m", if foreground { 35 } else { 45 }),
            Color::Cyan => format!("\x1b[{}m", if foreground { 36 } else { 46 }),
            Color::White => format!("\x1b[{}m", if foreground { 37 } else { 47 }),
            Color::BrightBlack => format!("\x1b[{}m", if foreground { 90 } else { 100 }),
            Color::BrightRed => format!("\x1b[{}m", if foreground { 91 } else { 101 }),
            Color::BrightGreen => format!("\x1b[{}m", if foreground { 92 } else { 102 }),
            Color::BrightYellow => format!("\x1b[{}m", if foreground { 93 } else { 103 }),
            Color::BrightBlue => format!("\x1b[{}m", if foreground { 94 } else { 104 }),
            Color::BrightMagenta => format!("\x1b[{}m", if foreground { 95 } else { 105 }),
            Color::BrightCyan => format!("\x1b[{}m", if foreground { 96 } else { 106 }),
            Color::BrightWhite => format!("\x1b[{}m", if foreground { 97 } else { 107 }),
            Color::RGB { r, g, b } => format!("\x1b[{};2;{};{};{}m", prefix, r, g, b),
        }
    }
}

/// Menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub shortcut: Option<char>,
    pub icon: Option<String>,
    pub enabled: bool,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            icon: None,
            enabled: true,
            children: vec![],
        }
    }
    
    pub fn with_shortcut(mut self, c: char) -> Self {
        self.shortcut = Some(c);
        self
    }
    
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Progress indicator
#[derive(Debug, Clone)]
pub enum ProgressStyle {
    Spinner,
    Bar { width: usize },
    Percentage,
    Dots,
}

/// Progress state
#[derive(Debug, Clone)]
pub struct Progress {
    pub current: u64,
    pub total: u64,
    pub message: String,
    pub style: ProgressStyle,
    spinner_frame: u8,
}

impl Progress {
    pub fn new(total: u64, message: impl Into<String>) -> Self {
        Self {
            current: 0,
            total,
            message: message.into(),
            style: ProgressStyle::Bar { width: 40 },
            spinner_frame: 0,
        }
    }
    
    pub fn inc(&mut self, delta: u64) {
        self.current = (self.current + delta).min(self.total);
    }
    
    pub fn set(&mut self, current: u64) {
        self.current = current.min(self.total);
    }
    
    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }
    
    pub fn render(&mut self) -> String {
        match &self.style {
            ProgressStyle::Spinner => {
                let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let frame = frames[self.spinner_frame as usize % frames.len()];
                self.spinner_frame = (self.spinner_frame + 1) % frames.len() as u8;
                format!("{} {}", frame, self.message)
            }
            ProgressStyle::Bar { width } => {
                let pct = if self.total > 0 { self.current as f64 / self.total as f64 } else { 0.0 };
                let filled = (pct * *width as f64) as usize;
                let empty = width - filled;
                format!(
                    "{} [{}{}] {:.0}% - {}",
                    self.message,
                    "█".repeat(filled),
                    "░".repeat(empty),
                    pct * 100.0,
                    self.current
                )
            }
            ProgressStyle::Percentage => {
                let pct = if self.total > 0 { self.current as f64 / self.total as f64 * 100.0 } else { 0.0 };
                format!("{}: {:.1}%", self.message, pct)
            }
            ProgressStyle::Dots => {
                let dots = ".".repeat((self.spinner_frame as usize % 4) + 1);
                self.spinner_frame = (self.spinner_frame + 1) % 4;
                format!("{}{}", self.message, dots)
            }
        }
    }
}

/// Text style
#[derive(Debug, Clone, Copy)]
pub enum TextStyle {
    Normal,
    Bold,
    Italic,
    Underline,
    Dim,
}

impl TextStyle {
    pub fn to_ansi(&self) -> &'static str {
        match self {
            TextStyle::Normal => "\x1b[0m",
            TextStyle::Bold => "\x1b[1m",
            TextStyle::Italic => "\x1b[3m",
            TextStyle::Underline => "\x1b[4m",
            TextStyle::Dim => "\x1b[2m",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GUI COMPONENTS
// ═══════════════════════════════════════════════════════════════════════════════

/// Terminal GUI
pub struct TerminalGui {
    width: usize,
    height: usize,
    colors: ColorScheme,
    buffer: Vec<Vec<char>>,
    dirty: bool,
}

impl TerminalGui {
    pub fn new() -> Self {
        Self {
            width: 80,
            height: 24,
            colors: ColorScheme::default(),
            buffer: vec![vec![' '; 80]; 24],
            dirty: false,
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer = vec![vec![' '; self.width]; self.height];
        self.dirty = true;
    }
    
    pub fn print_at(&mut self, x: usize, y: usize, text: &str) {
        if y >= self.height { return; }
        for (i, c) in text.chars().enumerate() {
            if x + i >= self.width { break; }
            self.buffer[y][x + i] = c;
        }
        self.dirty = true;
    }
    
    pub fn draw_box(&mut self, x: usize, y: usize, w: usize, h: usize, title: Option<&str>) {
        if x + w > self.width || y + h > self.height { return; }
        
        // Corners
        self.buffer[y][x] = '┌';
        self.buffer[y][x + w - 1] = '┐';
        self.buffer[y + h - 1][x] = '└';
        self.buffer[y + h - 1][x + w - 1] = '┘';
        
        // Horizontal lines
        for i in 1..w - 1 {
            self.buffer[y][x + i] = '─';
            self.buffer[y + h - 1][x + i] = '─';
        }
        
        // Vertical lines
        for j in 1..h - 1 {
            self.buffer[y + j][x] = '│';
            self.buffer[y + j][x + w - 1] = '│';
        }
        
        // Title
        if let Some(t) = title {
            let title_text = format!(" {} ", t);
            for (i, c) in title_text.chars().enumerate() {
                if x + 2 + i >= x + w - 1 { break; }
                self.buffer[y][x + 2 + i] = c;
            }
        }
        
        self.dirty = true;
    }
    
    pub fn draw_menu(&mut self, x: usize, y: usize, items: &[MenuItem], selected: usize) {
        for (i, item) in items.iter().enumerate() {
            let prefix = if i == selected { "► " } else { "  " };
            let text = format!("{}{}", prefix, item.label);
            self.print_at(x, y + i, &text);
        }
        self.dirty = true;
    }
    
    pub fn render(&self) -> String {
        let mut output = String::new();
        for row in &self.buffer {
            output.extend(row.iter());
            output.push('\n');
        }
        output
    }
    
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.buffer = vec![vec![' '; width]; height];
        self.dirty = true;
    }
}

impl Default for TerminalGui {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INPUT HANDLING
// ═══════════════════════════════════════════════════════════════════════════════

/// Key event
#[derive(Debug, Clone)]
pub enum KeyEvent {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Function(u8),
    Ctrl(char),
    Alt(char),
}

/// Input handler
pub struct InputHandler {
    buffer: Vec<char>,
    cursor: usize,
    history: Vec<String>,
    history_index: Option<usize>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: None,
        }
    }
    
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        match key {
            KeyEvent::Char(c) => {
                self.buffer.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyEvent::Backspace => {
                if self.cursor > 0 {
                    self.buffer.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            KeyEvent::Delete => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
            }
            KeyEvent::Left => {
                if self.cursor > 0 { self.cursor -= 1; }
            }
            KeyEvent::Right => {
                if self.cursor < self.buffer.len() { self.cursor += 1; }
            }
            KeyEvent::Home => self.cursor = 0,
            KeyEvent::End => self.cursor = self.buffer.len(),
            KeyEvent::Enter => {
                let line: String = self.buffer.drain(..).collect();
                self.cursor = 0;
                if !line.is_empty() {
                    self.history.push(line.clone());
                }
                self.history_index = None;
                return Some(line);
            }
            KeyEvent::Up => {
                if !self.history.is_empty() {
                    self.history_index = Some(
                        self.history_index.map(|i| i.saturating_sub(1))
                            .unwrap_or(self.history.len() - 1)
                    );
                    if let Some(idx) = self.history_index {
                        self.buffer = self.history[idx].chars().collect();
                        self.cursor = self.buffer.len();
                    }
                }
            }
            KeyEvent::Down => {
                if let Some(idx) = self.history_index {
                    if idx + 1 < self.history.len() {
                        self.history_index = Some(idx + 1);
                        self.buffer = self.history[idx + 1].chars().collect();
                    } else {
                        self.history_index = None;
                        self.buffer.clear();
                    }
                    self.cursor = self.buffer.len();
                }
            }
            KeyEvent::Escape => {
                self.buffer.clear();
                self.cursor = 0;
                self.history_index = None;
            }
            _ => {}
        }
        None
    }
    
    pub fn get_buffer(&self) -> String {
        self.buffer.iter().collect()
    }
    
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_bar() {
        let mut p = Progress::new(100, "Loading");
        p.set(50);
        let rendered = p.render();
        assert!(rendered.contains("50%"));
    }
    
    #[test]
    fn test_input_handler() {
        let mut handler = InputHandler::new();
        
        handler.handle_key(KeyEvent::Char('H'));
        handler.handle_key(KeyEvent::Char('i'));
        let result = handler.handle_key(KeyEvent::Enter);
        
        assert_eq!(result, Some("Hi".to_string()));
    }
    
    #[test]
    fn test_terminal_gui() {
        let mut gui = TerminalGui::new();
        gui.draw_box(0, 0, 20, 5, Some("Test"));
        gui.print_at(2, 2, "Hello");
        
        let output = gui.render();
        assert!(output.contains("Test"));
    }
}
