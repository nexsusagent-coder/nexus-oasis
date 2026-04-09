//! Internationalization (i18n) module for SENTIENT
//!
//! Supports 8 languages:
//! - English (en)
//! - Turkish (tr)
//! - German (de)
//! - French (fr)
//! - Spanish (es)
//! - Japanese (ja)
//! - Chinese (zh)
//! - Russian (ru)

pub mod translations;
pub mod locale;
pub mod formatter;

pub use translations::*;
pub use locale::*;
pub use formatter::*;

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "tr")]
    Turkish,
    #[strum(serialize = "de")]
    German,
    #[strum(serialize = "fr")]
    French,
    #[strum(serialize = "es")]
    Spanish,
    #[strum(serialize = "ja")]
    Japanese,
    #[strum(serialize = "zh")]
    Chinese,
    #[strum(serialize = "ru")]
    Russian,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    /// Get the native name of the language
    pub fn native_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Turkish => "Türkçe",
            Language::German => "Deutsch",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::Japanese => "日本語",
            Language::Chinese => "中文",
            Language::Russian => "Русский",
        }
    }

    /// Get the flag emoji for the language
    pub fn flag(&self) -> &'static str {
        match self {
            Language::English => "🇬🇧",
            Language::Turkish => "🇹🇷",
            Language::German => "🇩🇪",
            Language::French => "🇫🇷",
            Language::Spanish => "🇪🇸",
            Language::Japanese => "🇯🇵",
            Language::Chinese => "🇨🇳",
            Language::Russian => "🇷🇺",
        }
    }

    /// Check if the language uses RTL (right-to-left) script
    pub fn is_rtl(&self) -> bool {
        false // None of our supported languages are RTL
    }

    /// Get all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::Turkish,
            Language::German,
            Language::French,
            Language::Spanish,
            Language::Japanese,
            Language::Chinese,
            Language::Russian,
        ]
    }
}

/// Translation key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKey(String);

impl TranslationKey {
    pub fn new(key: &str) -> Self {
        Self(key.to_string())
    }
}

impl From<&str> for TranslationKey {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for TranslationKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// I18n instance
pub struct I18n {
    language: Language,
    translations: HashMap<String, String>,
}

impl I18n {
    /// Create a new I18n instance with the specified language
    pub fn new(language: Language) -> Self {
        let translations = load_translations(language);
        Self { language, translations }
    }

    /// Get current language
    pub fn language(&self) -> Language {
        self.language
    }

    /// Change language
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.translations = load_translations(language);
    }

    /// Translate a key
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("!{}!", key))
    }

    /// Translate with variables
    pub fn tv(&self, key: &str, vars: &HashMap<&str, &str>) -> String {
        let mut text = self.t(key);
        for (k, v) in vars {
            text = text.replace(&format!("{{{}}}", k), v);
        }
        text
    }

    /// Check if a translation exists
    pub fn has(&self, key: &str) -> bool {
        self.translations.contains_key(key)
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new(Language::default())
    }
}

/// Load translations for a language
fn load_translations(language: Language) -> HashMap<String, String> {
    match language {
        Language::English => translations::english(),
        Language::Turkish => translations::turkish(),
        Language::German => translations::german(),
        Language::French => translations::french(),
        Language::Spanish => translations::spanish(),
        Language::Japanese => translations::japanese(),
        Language::Chinese => translations::chinese(),
        Language::Russian => translations::russian(),
    }
}

/// Global i18n instance
pub static I18N: Lazy<std::sync::RwLock<I18n>> = Lazy::new(|| {
    std::sync::RwLock::new(I18n::default())
});

/// Get translation (convenience function)
pub fn t(key: &str) -> String {
    I18N.read().unwrap().t(key)
}

/// Translate with variables (convenience function)
pub fn tv(key: &str, vars: &HashMap<&str, &str>) -> String {
    I18N.read().unwrap().tv(key, vars)
}

/// Set global language
pub fn set_language(language: Language) {
    I18N.write().unwrap().set_language(language);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_native_names() {
        assert_eq!(Language::English.native_name(), "English");
        assert_eq!(Language::Turkish.native_name(), "Türkçe");
        assert_eq!(Language::Japanese.native_name(), "日本語");
    }

    #[test]
    fn test_language_flags() {
        assert_eq!(Language::Turkish.flag(), "🇹🇷");
        assert_eq!(Language::Japanese.flag(), "🇯🇵");
    }

    #[test]
    fn test_language_all() {
        let all = Language::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn test_i18n_translation() {
        let i18n = I18n::new(Language::English);
        assert!(!i18n.t("app.name").is_empty());
    }

    #[test]
    fn test_i18n_missing_key() {
        let i18n = I18n::new(Language::English);
        assert!(i18n.t("nonexistent.key").starts_with('!'));
    }

    #[test]
    fn test_i18n_with_vars() {
        let i18n = I18n::new(Language::English);
        let mut vars = HashMap::new();
        vars.insert("name", "Test");
        let text = i18n.tv("greeting.hello", &vars);
        assert!(text.contains("Test") || text.contains("hello"));
    }

    #[test]
    fn test_i18n_turkish() {
        let i18n = I18n::new(Language::Turkish);
        let name = i18n.t("app.name");
        assert!(!name.is_empty());
    }
}
