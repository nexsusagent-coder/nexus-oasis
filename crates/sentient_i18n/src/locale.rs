//! Locale detection and management

use super::Language;
use std::env;

/// Detect system language
pub fn detect_language() -> Language {
    // Try environment variable first
    if let Ok(lang) = env::var("SENTIENT_LANG") {
        if let Ok(language) = lang.parse() {
            return language;
        }
    }

    // Try LANG environment variable (Unix)
    if let Ok(lang) = env::var("LANG") {
        let lang_code = lang.split('.')
            .next()
            .unwrap_or("en")
            .split('_')
            .next()
            .unwrap_or("en");
        
        if let Ok(language) = lang_code.parse() {
            return language;
        }
    }

    // Try LC_ALL (Unix)
    if let Ok(lang) = env::var("LC_ALL") {
        let lang_code = lang.split('.')
            .next()
            .unwrap_or("en")
            .split('_')
            .next()
            .unwrap_or("en");
        
        if let Ok(language) = lang_code.parse() {
            return language;
        }
    }

    // Default to English
    Language::English
}

/// Locale information
#[derive(Debug, Clone)]
pub struct Locale {
    pub language: Language,
    pub region: Option<String>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
}

impl Locale {
    /// Create a new locale
    pub fn new(language: Language) -> Self {
        Self {
            language,
            region: None,
            timezone: None,
            currency: None,
        }
    }

    /// Detect locale from system
    pub fn detect() -> Self {
        let language = detect_language();
        
        let region = env::var("LANG")
            .ok()
            .and_then(|lang| {
                lang.split('_')
                    .nth(1)
                    .map(|s| s.split('.').next().unwrap_or(s).to_string())
            });

        let timezone = env::var("TZ").ok();

        Self {
            language,
            region,
            timezone,
            currency: None,
        }
    }

    /// Set region
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string());
        self
    }

    /// Set timezone
    pub fn with_timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_string());
        self
    }

    /// Set currency
    pub fn with_currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_string());
        self
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::detect()
    }
}

/// Get locale-specific date format
pub fn date_format(language: Language) -> &'static str {
    match language {
        Language::English => "%B %d, %Y",
        Language::Turkish => "%d %B %Y",
        Language::German => "%d.%m.%Y",
        Language::French => "%d/%m/%Y",
        Language::Spanish => "%d/%m/%Y",
        Language::Japanese => "%Y年%m月%d日",
        Language::Chinese => "%Y年%m月%d日",
        Language::Russian => "%d.%m.%Y",
    }
}

/// Get locale-specific time format
pub fn time_format(language: Language) -> &'static str {
    match language {
        Language::English => "%I:%M %p",
        Language::Turkish => "%H:%M",
        Language::German => "%H:%M",
        Language::French => "%H:%M",
        Language::Spanish => "%H:%M",
        Language::Japanese => "%H:%M",
        Language::Chinese => "%H:%M",
        Language::Russian => "%H:%M",
    }
}

/// Get locale-specific number format
pub fn number_format(language: Language) -> NumberFormat {
    match language {
        Language::English => NumberFormat {
            decimal_separator: '.',
            thousands_separator: ',',
        },
        Language::Turkish => NumberFormat {
            decimal_separator: ',',
            thousands_separator: '.',
        },
        Language::German => NumberFormat {
            decimal_separator: ',',
            thousands_separator: '.',
        },
        Language::French => NumberFormat {
            decimal_separator: ',',
            thousands_separator: ' ',
        },
        Language::Spanish => NumberFormat {
            decimal_separator: ',',
            thousands_separator: '.',
        },
        Language::Japanese => NumberFormat {
            decimal_separator: '.',
            thousands_separator: ',',
        },
        Language::Chinese => NumberFormat {
            decimal_separator: '.',
            thousands_separator: ',',
        },
        Language::Russian => NumberFormat {
            decimal_separator: ',',
            thousands_separator: ' ',
        },
    }
}

/// Number format configuration
#[derive(Debug, Clone, Copy)]
pub struct NumberFormat {
    pub decimal_separator: char,
    pub thousands_separator: char,
}

impl NumberFormat {
    /// Format a number
    pub fn format(&self, n: f64, decimals: usize) -> String {
        let formatted = format!("{:.1$}", n, decimals);
        let parts: Vec<&str> = formatted.split('.').collect();
        
        let int_part = parts[0];
        let dec_part = parts.get(1).unwrap_or(&"");
        
        // Add thousands separator
        let mut result = String::new();
        for (i, c) in int_part.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(self.thousands_separator);
            }
            result.push(c);
        }
        let int_part: String = result.chars().rev().collect();
        
        if decimals > 0 {
            format!("{}{}{}", int_part, self.decimal_separator, dec_part)
        } else {
            int_part
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_detect() {
        let locale = Locale::detect();
        assert!(matches!(locale.language, Language::_));
    }

    #[test]
    fn test_locale_with_region() {
        let locale = Locale::new(Language::English)
            .with_region("US")
            .with_timezone("America/New_York");
        
        assert_eq!(locale.region, Some("US".to_string()));
        assert_eq!(locale.timezone, Some("America/New_York".to_string()));
    }

    #[test]
    fn test_date_format() {
        let fmt = date_format(Language::English);
        assert!(!fmt.is_empty());
    }

    #[test]
    fn test_number_format() {
        let nf = number_format(Language::English);
        assert_eq!(nf.decimal_separator, '.');
        assert_eq!(nf.thousands_separator, ',');
        
        let nf = number_format(Language::German);
        assert_eq!(nf.decimal_separator, ',');
        assert_eq!(nf.thousands_separator, '.');
    }

    #[test]
    fn test_number_formatting() {
        let nf = number_format(Language::English);
        let formatted = nf.format(1234567.89, 2);
        assert!(formatted.contains("1,234,567"));
    }
}
