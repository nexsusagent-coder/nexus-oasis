//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS V-GATE - LLM Köprüsü
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! V-GATE üzerinden LLM iletişimi.

use crate::error::ManusResult;

/// ─── MANUS V-GATE ───

#[derive(Clone)]
pub struct ManusVGate {
    /// V-GATE URL
    url: String,
    /// Model adı
    model: String,
    /// System prompt
    system_prompt: String,
}

impl ManusVGate {
    /// Yeni V-GATE köprüsü oluştur
    pub fn new(url: &str) -> Self {
        Self {
            url: url.into(),
            model: "qwen/qwen3-1.7b:free".into(),
            system_prompt: MANUS_SYSTEM_PROMPT.into(),
        }
    }
    
    /// Kod üret
    pub async fn generate_code(&self, task: &str, language: &str) -> ManusResult<String> {
        log::info!("🔗  MANUS-VGATE: Kod üretme isteği → {} ({})", task, language);
        
        // Simülasyon - gerçek V-GATE entegrasyonu için
        // sentient_vgate kullanılacak
        
        let code = format!(
            r#"# Üretilen kod: {}
# Dil: {}

def main():
    print("Görev: {}")
    # Kod mantığı buraya
    pass

if __name__ == "__main__":
    main()
"#,
            task, language, task
        );
        
        Ok(code)
    }
    
    /// Hata düzeltme önerisi al
    pub async fn suggest_fix(&self, code: &str, error: &str) -> ManusResult<String> {
        log::info!("🔗  MANUS-VGATE: Hata düzeltme önerisi isteniyor");
        
        let suggestion = format!(
            "# Önerilen düzeltme\n# Hata: {}\n# Orijinal kod:\n{}\n\n# Düzeltilmiş:\n# (LLM önerisi burada)",
            error, code
        );
        
        Ok(suggestion)
    }
    
    /// Sonuç doğrula
    pub async fn verify_result(&self, expected: &str, actual: &str) -> ManusResult<bool> {
        log::info!("🔗  MANUS-VGATE: Sonuç doğrulanıyor");
        
        // Basit içerme kontrolü
        Ok(actual.contains(expected) || expected.contains(actual))
    }
}

/// Manus System Prompt
static MANUS_SYSTEM_PROMPT: &str = r###"
Sen SENTIENT'nin kod yazma ve calistirma ajani Manus'sin.

GOREV:
1. Kullanici isteklerini koda donustur
2. Kodu Docker sandbox icinde calistir
3. Sonuclari dogrula ve raporla

KURALLAR:
1. Sadece guvenli kod yaz (dosya sistemi erisimi yok)
2. Hatalari SENTIENT dilinde raporla
3. Maksimum 60 saniye timeout
4. Memory limiti: 256 MB

DESTEKLENEN DILLER:
- Python (varsayilan)
- JavaScript
- Bash
- Rust
- Go

CIKTI FORMATI:
1. Kod aciklamasi
2. Calistirilan kod
3. Sonuc
4. Hata (varsa)
"###;
