//! Formatters for locale-aware output

use super::Language;
use super::locale::{date_format, time_format, number_format, NumberFormat};
use chrono::{DateTime, Utc, TimeZone};
use std::fmt::Display;

/// Format a date for a specific language
pub fn format_date(date: &DateTime<Utc>, language: Language) -> String {
    date.format(date_format(language)).to_string()
}

/// Format a time for a specific language
pub fn format_time(date: &DateTime<Utc>, language: Language) -> String {
    date.format(time_format(language)).to_string()
}

/// Format a datetime for a specific language
pub fn format_datetime(date: &DateTime<Utc>, language: Language) -> String {
    let date_str = format_date(date, language);
    let time_str = format_time(date, language);
    format!("{} {}", date_str, time_str)
}

/// Format a number for a specific language
pub fn format_number(n: f64, language: Language, decimals: usize) -> String {
    number_format(language).format(n, decimals)
}

/// Format a currency value
pub fn format_currency(amount: f64, currency: &str, language: Language) -> String {
    let nf = number_format(language);
    let formatted = nf.format(amount, 2);
    
    match language {
        Language::English => format!("{}{}", currency, formatted),
        Language::Turkish => format!("{} {}", formatted, currency),
        Language::German => format!("{} {}", formatted, currency),
        Language::French => format!("{} {}", formatted, currency),
        Language::Spanish => format!("{} {}", formatted, currency),
        Language::Japanese => format!("{}{}", formatted, currency),
        Language::Chinese => format!("{}{}", currency, formatted),
        Language::Russian => format!("{} {}", formatted, currency),
    }
}

/// Format a percentage
pub fn format_percentage(value: f64, language: Language) -> String {
    let nf = number_format(language);
    let formatted = nf.format(value * 100.0, 1);
    
    match language {
        Language::Turkish | Language::German | Language::French | Language::Spanish | Language::Russian => {
            format!("{}%", formatted)
        }
        _ => format!("{}%", formatted),
    }
}

/// Relative time formatter
pub fn format_relative_time(date: &DateTime<Utc>, language: Language) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(*date);
    
    let seconds = diff.num_seconds();
    let minutes = diff.num_minutes();
    let hours = diff.num_hours();
    let days = diff.num_days();
    
    match language {
        Language::English => {
            if seconds < 60 {
                "just now".to_string()
            } else if minutes < 60 {
                format!("{} minutes ago", minutes)
            } else if hours < 24 {
                format!("{} hours ago", hours)
            } else if days < 7 {
                format!("{} days ago", days)
            } else {
                format_date(date, language)
            }
        }
        Language::Turkish => {
            if seconds < 60 {
                "az önce".to_string()
            } else if minutes < 60 {
                format!("{} dakika önce", minutes)
            } else if hours < 24 {
                format!("{} saat önce", hours)
            } else if days < 7 {
                format!("{} gün önce", days)
            } else {
                format_date(date, language)
            }
        }
        Language::German => {
            if seconds < 60 {
                "gerade eben".to_string()
            } else if minutes < 60 {
                format!("vor {} Minuten", minutes)
            } else if hours < 24 {
                format!("vor {} Stunden", hours)
            } else if days < 7 {
                format!("vor {} Tagen", days)
            } else {
                format_date(date, language)
            }
        }
        Language::French => {
            if seconds < 60 {
                "à l'instant".to_string()
            } else if minutes < 60 {
                format!("il y a {} minutes", minutes)
            } else if hours < 24 {
                format!("il y a {} heures", hours)
            } else if days < 7 {
                format!("il y a {} jours", days)
            } else {
                format_date(date, language)
            }
        }
        Language::Spanish => {
            if seconds < 60 {
                "hace poco".to_string()
            } else if minutes < 60 {
                format!("hace {} minutos", minutes)
            } else if hours < 24 {
                format!("hace {} horas", hours)
            } else if days < 7 {
                format!("hace {} días", days)
            } else {
                format_date(date, language)
            }
        }
        Language::Japanese => {
            if seconds < 60 {
                "たった今".to_string()
            } else if minutes < 60 {
                format!("{}分前", minutes)
            } else if hours < 24 {
                format!("{}時間前", hours)
            } else if days < 7 {
                format!("{}日前", days)
            } else {
                format_date(date, language)
            }
        }
        Language::Chinese => {
            if seconds < 60 {
                "刚刚".to_string()
            } else if minutes < 60 {
                format!("{}分钟前", minutes)
            } else if hours < 24 {
                format!("{}小时前", hours)
            } else if days < 7 {
                format!("{}天前", days)
            } else {
                format_date(date, language)
            }
        }
        Language::Russian => {
            if seconds < 60 {
                "только что".to_string()
            } else if minutes < 60 {
                format!("{} минут назад", minutes)
            } else if hours < 24 {
                format!("{} часов назад", hours)
            } else if days < 7 {
                format!("{} дней назад", days)
            } else {
                format_date(date, language)
            }
        }
    }
}

/// Byte size formatter
pub fn format_bytes(bytes: u64, language: Language) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    let nf = number_format(language);
    
    if bytes >= TB {
        format!("{} TB", nf.format(bytes as f64 / TB as f64, 2))
    } else if bytes >= GB {
        format!("{} GB", nf.format(bytes as f64 / GB as f64, 2))
    } else if bytes >= MB {
        format!("{} MB", nf.format(bytes as f64 / MB as f64, 2))
    } else if bytes >= KB {
        format!("{} KB", nf.format(bytes as f64 / KB as f64, 2))
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let date = Utc.ymd(2024, 1, 15).and_hms(12, 30, 0);
        let formatted = format_date(&date, Language::English);
        assert!(formatted.contains("2024"));
    }

    #[test]
    fn test_format_number() {
        let formatted = format_number(1234.56, Language::English, 2);
        assert!(formatted.contains("1,234"));
        
        let formatted = format_number(1234.56, Language::German, 2);
        assert!(formatted.contains("1.234"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500, Language::English), "500 B");
        assert!(format_bytes(1500, Language::English).contains("KB"));
        assert!(format_bytes(1500000, Language::English).contains("MB"));
    }

    #[test]
    fn test_format_relative_time() {
        let now = Utc::now();
        let formatted = format_relative_time(&now, Language::English);
        assert_eq!(formatted, "just now");
        
        let formatted = format_relative_time(&now, Language::Turkish);
        assert_eq!(formatted, "az önce");
    }
}
