//! ─── SCOUT YAPILANDIRMA ───

use serde::{Deserialize, Serialize};

/// Scout yapilandirmasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutConfig {
    /// Kullanici agenti
    pub user_agent: String,
    /// Zaman asimi (saniye)
    pub timeout_secs: u64,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Proxy yapilandirmasi
    pub proxy: Option<ProxyConfig>,
    /// Anti-detection ayarlari
    pub stealth: StealthConfig,
    /// Cache ayarlari
    pub cache: CacheConfig,
}

impl Default for ScoutConfig {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".into(),
            timeout_secs: 30,
            max_retries: 3,
            proxy: None,
            stealth: StealthConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

/// Proxy yapilandirmasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Protokol (http, https, socks5)
    pub protocol: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Kullanici adi (opsiyonel)
    pub username: Option<String>,
    /// Sifre (opsiyonel)
    pub password: Option<String>,
    /// Rotasyon modu
    pub rotation: ProxyRotation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProxyRotation {
    /// Rotasyon yok
    None,
    /// Her istekte degistir
    PerRequest,
    /// Her N istekte degistir
    EveryN(u32),
    /// Hata durumunda degistir
    OnError,
}

/// Anti-detection ayarlari
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Headless browser simulasyonu
    pub headless: bool,
    /// Canvas fingerprint rastgelelestirme
    pub canvas_noise: bool,
    /// WebRTC IP sizi
    pub webrtc_leak_prevention: bool,
    /// Timezone spoofing
    pub timezone_spoof: Option<String>,
    /// Dil
    pub language: String,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            headless: true,
            canvas_noise: true,
            webrtc_leak_prevention: true,
            timezone_spoof: Some("Europe/Istanbul".into()),
            language: "tr-TR".into(),
        }
    }
}

/// Cache yapilandirmasi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache aktif
    pub enabled: bool,
    /// Sure (dakika)
    pub ttl_minutes: u64,
    /// Maksimum boyut (MB)
    pub max_size_mb: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_minutes: 60,
            max_size_mb: 100,
        }
    }
}
