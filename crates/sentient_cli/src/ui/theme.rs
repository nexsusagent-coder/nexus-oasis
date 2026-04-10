//! ─── THEME ───
//!
//! Renk paleti ve tema ayarlari

use colored::Color;

/// SENTIENT Tema rengi
#[derive(Clone, Debug)]
pub struct Theme {
    /// Birincil renk
    pub primary: Color,
    /// Ikincil renk
    pub secondary: Color,
    /// Basari rengi
    pub success: Color,
    /// Hata rengi
    pub error: Color,
    /// Uyari rengi
    pub warning: Color,
    /// Bilgi rengi
    pub info: Color,
    /// Metin rengi
    pub text: Color,
    /// Arka plan rengi
    pub muted: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::ocean()
    }
}

impl Theme {
    /// Okyanus temasi (varsayilan)
    pub fn ocean() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Blue,
            text: Color::White,
            muted: Color::BrightBlack,
        }
    }

    /// Karanlik tema
    pub fn dark() -> Self {
        Self {
            primary: Color::Magenta,
            secondary: Color::Blue,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::Cyan,
            text: Color::BrightWhite,
            muted: Color::BrightBlack,
        }
    }

    /// Neon tema
    pub fn neon() -> Self {
        Self {
            primary: Color::BrightCyan,
            secondary: Color::BrightMagenta,
            success: Color::BrightGreen,
            error: Color::BrightRed,
            warning: Color::BrightYellow,
            info: Color::BrightBlue,
            text: Color::BrightWhite,
            muted: Color::White,
        }
    }

    /// Minimal tema
    pub fn minimal() -> Self {
        Self {
            primary: Color::White,
            secondary: Color::BrightBlack,
            success: Color::Green,
            error: Color::Red,
            warning: Color::Yellow,
            info: Color::White,
            text: Color::White,
            muted: Color::BrightBlack,
        }
    }
}

use std::sync::RwLock;

/// Global tema (thread-safe)
static CURRENT_THEME: RwLock<Theme> = RwLock::new(Theme {
    primary: Color::Cyan,
    secondary: Color::Blue,
    success: Color::Green,
    error: Color::Red,
    warning: Color::Yellow,
    info: Color::Blue,
    text: Color::White,
    muted: Color::BrightBlack,
});

/// Temayi ayarla
pub fn set_theme(theme: Theme) {
    if let Ok(mut current) = CURRENT_THEME.write() {
        *current = theme;
    }
}

/// Mevcut temayi al
pub fn current_theme() -> Theme {
    CURRENT_THEME.read().map(|t| t.clone()).unwrap_or_default()
}

/// Emoji sablonlari
pub mod emoji {
    pub const SUCCESS: &str = "✓";
    pub const ERROR: &str = "✗";
    pub const WARNING: &str = "⚠";
    pub const INFO: &str = "ℹ";
    pub const LOADING: &str = "⏳";
    pub const ROBOT: &str = "🤖";
    pub const CORE: &str = "🧠";
    pub const BRAIN: &str = "🧠";
    pub const MEMORY: &str = "💾";
    pub const LOCK: &str = "🔒";
    pub const BUG: &str = "🐛";
    pub const ROCKET: &str = "🚀";
    pub const CHART: &str = "📊";
    pub const PLUG: &str = "🔌";
    pub const GLOBE: &str = "🌐";
    pub const TERMINAL: &str = "💻";
    pub const PACKAGE: &str = "📦";
    pub const CLIPBOARD: &str = "📋";
}

/// Border karakterleri
pub mod border {
    pub const HORIZONTAL: &str = "─";
    pub const VERTICAL: &str = "│";
    pub const TOP_LEFT: &str = "╭";
    pub const TOP_RIGHT: &str = "╮";
    pub const BOTTOM_LEFT: &str = "╰";
    pub const BOTTOM_RIGHT: &str = "╯";
    pub const DOUBLE_HORIZONTAL: &str = "═";
    pub const DOUBLE_VERTICAL: &str = "║";
    pub const HEAVY_HORIZONTAL: &str = "━";
    pub const HEAVY_VERTICAL: &str = "┃";
}

/// Simgeler
pub mod symbols {
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
    pub const BULLET: &str = "•";
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const DOT: &str = "·";
    pub const HORIZONTAL_LINE: &str = "─";
    pub const VERTICAL_LINE: &str = "│";
}

/// Baslik bicimi
pub fn title(text: &str) -> String {
    use colored::Colorize;
    format!("\n{}\n", text.cyan().bold())
}

/// Alt baslik bicimi
pub fn subtitle(text: &str) -> String {
    use colored::Colorize;
    format!("{} {}", "▶".cyan(), text.yellow())
}

/// Section basligi
pub fn section_header(text: &str) -> String {
    use colored::Colorize;
    format!("\n{}\n  {}\n", "────────────────────────────────────────".cyan(), text.yellow().bold())
}

/// Ayirici cizgi
pub fn divider() -> String {
    use colored::Colorize;
    "────────────────────────────────────────────".cyan().to_string()
}

/// Bos satir
pub fn blank() -> String {
    "\n".to_string()
}
