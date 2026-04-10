//! ═════════════════════════════════════════════════════════════════
//!  TRANSLATE MODULE
//! ═════════════════════════════════════════════════════════════════

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Python traceback, TypeError vb. ham hataları yakala
    static ref PYTHON_ERR_RE: Regex = Regex::new(r"(?i)(traceback|typeerror|valueerror|keyerror|attributeerror|indexerror|runtimeerror|syntaxerror|module not found|importerror|nonetype)").expect("operation failed");
    // JS/Node.js hatası yakala
    static ref JS_ERR_RE: Regex = Regex::new(r"(?i)(typeerror|referenceerror|syntaxerror|rangeerror|uncaught exception|promise rejection|stacktrace)").expect("operation failed");
}

/// Dış kaynaktan gelen ham hata mesajını yakalar ve
/// SENTIENT'nın anlayacağı Türkçe mesaja çevirir.
pub fn translate_raw_error(raw: &str) -> String {
    let lower = raw.to_lowercase();

    if PYTHON_ERR_RE.is_match(&lower) {
        return "🐍  KÖPRÜ: Python modülünde yürütme hatası algılandı. Modül yeniden başlatılıyor.".to_string();
    }

    if JS_ERR_RE.is_match(&lower) {
        return "🌐  KÖPRÜ: Node.js köprüsünde beklenmeyen durum. Bağlantı yenileniyor.".to_string();
    }

    if lower.contains("timeout") || lower.contains("timed out") || lower.contains("bağlantı") {
        return "🚪  V-GATE: Bağlantı zaman aşımına uğradı. Veri akışı yenileniyor.".to_string();
    }

    if lower.contains("permission denied") || lower.contains("forbidden") || lower.contains("401") || lower.contains("403") {
        return "🔐  KİMLİK: Yetkisiz erişim talebi reddedildi.".to_string();
    }

    if lower.contains("out of memory") || lower.contains("oom") {
        return "🧠  BELLEK: Sistem bellek limiti aşıldı. Kaynaklar temizleniyor.".to_string();
    }

    // Tanınmayan hata → genel mesaj
    format!("⚠  SENTIENT: Bilinmeyen sistem hatası tespit edildi. Günlüğe kaydediliyor: {raw}")
}
