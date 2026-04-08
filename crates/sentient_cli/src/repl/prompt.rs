//! ─── PROMPT MODULU ───
//!
//! Dinamik prompt olusturma ve yonetimi

use colored::Colorize;

/// Prompt durumu
#[derive(Debug, Clone)]
pub struct PromptState {
    /// Mevcut mod
    pub mode: ReplMode,
    /// Aktif modul
    pub active_module: Option<String>,
    /// Ust dizin
    pub context: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReplMode {
    /// Normal mod
    Normal,
    /// Swarm modu
    Swarm,
    /// Debug modu
    Debug,
    /// Admin modu
    Admin,
}

impl PromptState {
    pub fn new() -> Self {
        Self {
            mode: ReplMode::Normal,
            active_module: None,
            context: None,
        }
    }

    pub fn with_mode(mut self, mode: ReplMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_module(mut self, module: impl Into<String>) -> Self {
        self.active_module = Some(module.into());
        self
    }

    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.context = Some(ctx.into());
        self
    }

    /// Prompt string'ini olustur
    pub fn render(&self) -> String {
        let mut parts = Vec::new();

        // Mod gostergesi
        match self.mode {
            ReplMode::Normal => parts.push("sentient".green().bold().to_string()),
            ReplMode::Swarm => parts.push("sentient:swarm".cyan().bold().to_string()),
            ReplMode::Debug => parts.push("sentient:debug".yellow().bold().to_string()),
            ReplMode::Admin => parts.push("sentient:admin".red().bold().to_string()),
        }

        // Modul gostergesi
        if let Some(module) = &self.active_module {
            parts.push(format!("({})", module.blue()).to_string());
        }

        // Baglam gostergesi
        if let Some(ctx) = &self.context {
            parts.push(format!("[{}]", ctx.dimmed()).to_string());
        }

        // Birlestir
        let prompt = parts.join(" ");
        format!("{}> ", prompt)
    }

    /// Mod degistir
    pub fn set_mode(&mut self, mode: ReplMode) {
        self.mode = mode;
    }

    /// Modul degistir
    pub fn set_module(&mut self, module: Option<String>) {
        self.active_module = module;
    }
}

impl Default for PromptState {
    fn default() -> Self {
        Self::new()
    }
}

/// Prompt stilleri
pub fn success_prompt() -> String {
    format!("{} ", "✓".green().bold())
}

pub fn error_prompt() -> String {
    format!("{} ", "✗".red().bold())
}

pub fn warning_prompt() -> String {
    format!("{} ", "⚠".yellow().bold())
}

pub fn info_prompt() -> String {
    format!("{} ", "ℹ".blue().bold())
}

pub fn loading_prompt() -> String {
    format!("{} ", "⏳".yellow().bold())
}
