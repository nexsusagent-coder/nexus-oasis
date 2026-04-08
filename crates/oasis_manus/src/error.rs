//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS ERROR - Hata Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Tüm ham hatalar SENTIENT diline çevrilir.

use thiserror::Error;

/// Manus hatası
#[derive(Debug, Error)]
pub enum ManusError {
    /// Sandbox ihlali
    #[error("OASIS-MANUS: Sandbox ihlali - {0}")]
    SandboxViolation(String),
    
    /// Container hatası
    #[error("OASIS-MANUS: Container hatası - {0}")]
    ContainerError(String),
    
    /// Docker bağlantı hatası
    #[error("OASIS-MANUS: Docker bağlantısı kurulamadı - {0}")]
    DockerConnectionFailed(String),
    
    /// Container oluşturma hatası
    #[error("OASIS-MANUS: Container oluşturulamadı - {0}")]
    ContainerCreateFailed(String),
    
    /// Container başlatma hatası
    #[error("OASIS-MANUS: Container başlatılamadı - {0}")]
    ContainerStartFailed(String),
    
    /// Kod çalıştırma hatası
    #[error("OASIS-MANUS: Kod çalıştırma hatası - {0}")]
    ExecutionFailed(String),
    
    /// Timeout hatası
    #[error("OASIS-MANUS: İşlem zaman aşımına uğradı ({0} saniye)")]
    Timeout(u64),
    
    /// Bellek limiti aşıldı
    #[error("OASIS-MANUS: Bellek limiti aşıldı ({0} MB)")]
    MemoryLimitExceeded(u32),
    
    /// CPU limiti aşıldı
    #[error("OASIS-MANUS: CPU limiti aşıldı")]
    CpuLimitExceeded,
    
    /// Ağ erişim hatası
    #[error("OASIS-MANUS: Ağ erişimi engellendi - {0}")]
    NetworkBlocked(String),
    
    /// Dosya erişim hatası
    #[error("OASIS-MANUS: Dosya erişimi engellendi - {0}")]
    FileAccessBlocked(String),
    
    /// Zararlı kod tespiti
    #[error("OASIS-MANUS: Zararlı kod tespit edildi - {0}")]
    MaliciousCodeDetected(String),
    
    /// V-GATE hatası
    #[error("OASIS-MANUS: V-GATE iletişim hatası - {0}")]
    VGateError(String),
    
    /// Planlama hatası
    #[error("OASIS-MANUS: Görev planlama hatası - {0}")]
    PlanningError(String),
    
    /// Doğrulama hatası
    #[error("OASIS-MANUS: Sonuç doğrulama hatası - {0}")]
    VerificationError(String),
    
    /// Geçersiz kod
    #[error("OASIS-MANUS: Geçersiz kod formatı - {0}")]
    InvalidCode(String),
    
    /// Dil desteklenmiyor
    #[error("OASIS-MANUS: Desteklenmeyen dil - {0}")]
    UnsupportedLanguage(String),
    
    /// Container bulunamadı
    #[error("OASIS-MANUS: Container bulunamadı - {0}")]
    ContainerNotFound(String),
    
    /// Kaynak yetersiz
    #[error("OASIS-MANUS: Yetersiz kaynak - {0}")]
    InsufficientResources(String),
    
    /// Genel hata
    #[error("OASIS-MANUS: {0}")]
    General(String),
}

/// Manus sonucu
pub type ManusResult<T> = Result<T, ManusError>;

/// Hata çevirici - ham hataları SENTIENT diline çevir
pub fn translate_error(raw: &str) -> String {
    // Docker hataları
    if raw.contains("Cannot connect to the Docker daemon") {
        return "Docker servisi çalışmıyor. Lütfen Docker'ı başlatın.".into();
    }
    
    if raw.contains("No such container") {
        return "Belirtilen container bulunamadı. Süresi dolmuş olabilir.".into();
    }
    
    if raw.contains("container already exists") {
        return "Aynı isimde bir container zaten mevcut.".into();
    }
    
    if raw.contains("OOMKilled") {
        return "Container bellek limitini aştığı için sonlandırıldı.".into();
    }
    
    // Python hataları
    if raw.contains("SyntaxError") {
        return "Kod sözdizimi hatası içeriyor. Lütfen kontrol edin.".into();
    }
    
    if raw.contains("IndentationError") {
        return "Kod girintileme hatası içeriyor. Python girintilerine dikkat edin.".into();
    }
    
    if raw.contains("ImportError") || raw.contains("ModuleNotFoundError") {
        return "Gerekli Python modülü bulunamadı. Yüklü olmayabilir.".into();
    }
    
    // Node.js hataları
    if raw.contains("SyntaxError") && raw.contains("Unexpected token") {
        return "JavaScript sözdizimi hatası. Beklenmeyen karakter bulundu.".into();
    }
    
    if raw.contains("Cannot find module") {
        return "Gerekli Node.js modülü bulunamadı. npm install gerekebilir.".into();
    }
    
    // Bash hataları
    if raw.contains("command not found") {
        return "Belirtilen komut sistemde yüklü değil.".into();
    }
    
    if raw.contains("Permission denied") {
        return "İzin hatası. Dosya veya komut erişimi reddedildi.".into();
    }
    
    // Timeout
    if raw.contains("timeout") || raw.contains("timed out") {
        return "İşlem zaman aşımına uğradı. Süre limiti aşıldı.".into();
    }
    
    // Ağ hataları
    if raw.contains("network") && raw.contains("unreachable") {
        return "Ağ erişimi sağlanamadı. Bağlantıyı kontrol edin.".into();
    }
    
    // Varsayılan
    format!("Sistem hatası: {}", raw.chars().take(100).collect::<String>())
}

/// Zararlı kod pattern'leri
pub const MALICIOUS_PATTERNS: &[&str] = &[
    // Dosya sistemi
    "rm -rf",
    "rm -r /",
    "mkfs",
    "dd if=/dev/",
    "format",
    
    // Sistem
    "shutdown",
    "reboot",
    "init 0",
    "init 6",
    
    // Ağ
    "iptables",
    "netstat -",
    "ifconfig",
    "tcpdump",
    
    // Yetki
    "chmod 777",
    "chown root",
    "sudo",
    "su root",
    
    // Python tehlikeli
    "os.system",
    "subprocess.call",
    "eval(",
    "exec(",
    "__import__",
    "compile(",
    
    // Node.js tehlikeli
    "child_process",
    "eval(",
    "Function(",
    "vm.runInContext",
    
    // Şüpheli
    "/etc/passwd",
    "/etc/shadow",
    ".ssh/",
    "id_rsa",
];

/// Zararlı kod kontrolü
pub fn detect_malicious_code(code: &str) -> Option<String> {
    let lower = code.to_lowercase();
    
    for pattern in MALICIOUS_PATTERNS {
        if lower.contains(&pattern.to_lowercase()) {
            return Some(format!("Şüpheli pattern tespit edildi: '{}'", pattern));
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_docker_error() {
        let raw = "Cannot connect to the Docker daemon";
        let translated = translate_error(raw);
        assert!(translated.contains("Docker servisi"));
    }

    #[test]
    fn test_translate_python_error() {
        let raw = "SyntaxError: invalid syntax";
        let translated = translate_error(raw);
        assert!(translated.contains("sözdizimi hatası"));
    }

    #[test]
    fn test_detect_malicious_code() {
        let code = "import os; os.system('rm -rf /')";
        let result = detect_malicious_code(code);
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_safe_code() {
        let code = "print('Hello, World!')";
        let result = detect_malicious_code(code);
        assert!(result.is_none());
    }

    #[test]
    fn test_malicious_patterns() {
        assert!(detect_malicious_code("os.system('ls')").is_some());
        assert!(detect_malicious_code("eval(user_input)").is_some());
        assert!(detect_malicious_code("sudo apt install").is_some());
    }
}
