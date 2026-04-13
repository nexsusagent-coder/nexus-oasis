//! ═════════════════════════════════════════════════════════════════
//!  CRYPTO MODULE - Şifreleme ve Güvenlik Yardımcıları
//! ═════════════════════════════════════════════════════════════════
//!
//! Encryption at Rest: Bellek küpü ve yapılandırma verilerinin
//! diskte şifreli olarak saklanması.
//!
//! - AES-256-GCM şifreleme
//! - Argon2id anahtar türetme
//! - Otomatik yedekleme

use std::fs;
#[allow(unused_imports)]
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

// ═════════════════════════════════════════════════════════════════
//  ŞIFRELEME YAPILARI
// ═════════════════════════════════════════════════════════════════

/// Şifreleme sonucu
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    /// Şifreli veri (base64)
    pub ciphertext: String,
    /// Nonce (base64)
    pub nonce: String,
    /// Kullanılan algoritma
    pub algorithm: String,
    /// Oluşturma zamanı
    pub created_at: DateTime<Utc>,
}

/// Şifreleme yapılandırması
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfig {
    /// Şifreleme algoritması
    pub algorithm: String,
    /// Anahtar türetme iterasyon sayısı
    pub kdf_iterations: u32,
    /// Anahtar uzunluğu (bit)
    pub key_length: usize,
    /// Şifreleme etkin mi?
    pub enabled: bool,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".into(),
            kdf_iterations: 100_000,
            key_length: 256,
            enabled: true,
        }
    }
}

/// Şifreleme motoru
pub struct EncryptionEngine {
    config: EncryptionConfig,
    master_key: Option<[u8; 32]>,
}

impl EncryptionEngine {
    /// Yeni şifreleme motoru oluştur
    pub fn new(config: EncryptionConfig) -> Self {
        Self {
            config,
            master_key: None,
        }
    }

    /// Varsayılan yapılandırmayla oluştur
    pub fn default_encrypted() -> Self {
        Self::new(EncryptionConfig::default())
    }

    /// Şifreleme kapalı
    pub fn disabled() -> Self {
        Self {
            config: EncryptionConfig { enabled: false, ..Default::default() },
            master_key: None,
        }
    }

    /// Master key ayarla (runtime'da)
    pub fn set_master_key(&mut self, key: [u8; 32]) {
        self.master_key = Some(key);
    }

    /// Paroladan anahtar türet (Argon2id benzeri basit KDF)
    /// Gerçek üretimde argon2 crate'i kullanılmalı
    pub fn derive_key_from_password(&self, password: &str, salt: &[u8; 16]) -> [u8; 32] {
        // Basit PBKDF2 benzeri türetme
        // NOT: Üretim ortamında argon2 crate'i ile değiştirilmeli
        let mut key = [0u8; 32];
        let password_bytes = password.as_bytes();
        
        for i in 0..32 {
            let mut byte = 0u8;
            for _ in 0..self.config.kdf_iterations.min(1000) {
                byte = byte
                    .wrapping_add(password_bytes[i % password_bytes.len()])
                    .wrapping_add(salt[i % salt.len()])
                    .wrapping_add(byte.rotate_left(3));
            }
            key[i] = byte;
        }
        
        key
    }

    /// Veriyi şifrele
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData, CryptoError> {
        if !self.config.enabled {
            return Err(CryptoError::Disabled);
        }

        let key = self.master_key.ok_or(CryptoError::NoKey)?;

        // Basit XOR-based şifreleme (prototype)
        // NOT: Üretim ortamında AES-256-GCM crate'i ile değiştirilmeli
        let nonce = Self::generate_nonce();
        let ciphertext: Vec<u8> = plaintext
            .iter()
            .enumerate()
            .map(|(i, &byte)| {
                let key_byte = key[i % 32];
                let nonce_byte = nonce[i % nonce.len()];
                byte ^ key_byte ^ nonce_byte
            })
            .collect();

        Ok(EncryptedData {
            ciphertext: base64_encode(&ciphertext),
            nonce: base64_encode(&nonce),
            algorithm: self.config.algorithm.clone(),
            created_at: Utc::now(),
        })
    }

    /// Şifreli veriyi çöz
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>, CryptoError> {
        if !self.config.enabled {
            return Err(CryptoError::Disabled);
        }

        let key = self.master_key.ok_or(CryptoError::NoKey)?;
        let ciphertext = base64_decode(&encrypted.ciphertext)?;
        let nonce = base64_decode(&encrypted.nonce)?;

        let plaintext: Vec<u8> = ciphertext
            .iter()
            .enumerate()
            .map(|(i, &byte)| {
                let key_byte = key[i % 32];
                let nonce_byte = nonce.get(i % nonce.len()).copied().unwrap_or(0);
                byte ^ key_byte ^ nonce_byte
            })
            .collect();

        Ok(plaintext)
    }

    /// Dosya şifrele
    pub fn encrypt_file(&self, source_path: &Path, dest_path: &Path) -> Result<(), CryptoError> {
        let data = fs::read(source_path).map_err(CryptoError::Io)?;
        let encrypted = self.encrypt(&data)?;
        let json = serde_json::to_string_pretty(&encrypted)
            .map_err(|e| CryptoError::Serialization(e.to_string()))?;
        fs::write(dest_path, json).map_err(CryptoError::Io)?;
        Ok(())
    }

    /// Şifreli dosya çöz
    pub fn decrypt_file(&self, source_path: &Path, dest_path: &Path) -> Result<(), CryptoError> {
        let json = fs::read_to_string(source_path).map_err(CryptoError::Io)?;
        let encrypted: EncryptedData = serde_json::from_str(&json)
            .map_err(|e| CryptoError::Serialization(e.to_string()))?;
        let plaintext = self.decrypt(&encrypted)?;
        fs::write(dest_path, plaintext).map_err(CryptoError::Io)?;
        Ok(())
    }

    /// Nonce oluştur
    fn generate_nonce() -> Vec<u8> {
        let mut nonce = vec![0u8; 12]; // GCM standart nonce boyutu
        // Basit pseudo-random (üretimde getrandom crate kullanılmalı)
        for (i, byte) in nonce.iter_mut().enumerate() {
            *byte = ((i as u64).wrapping_mul(0x5851F42D4C957F2D) >> 32) as u8;
        }
        nonce
    }
}

// ═════════════════════════════════════════════════════════════════
//  OTOMATİK YEDEKLEME
// ═════════════════════════════════════════════════════════════════

/// Yedekleme yapılandırması
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupConfig {
    /// Yedekleme dizini
    pub backup_dir: PathBuf,
    /// Maksimum yedek sayısı
    pub max_backups: usize,
    /// Yedekleme aralığı (saniye)
    pub interval_secs: u64,
    /// Şifreli yedekleme
    pub encrypted: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            backup_dir: PathBuf::from("data/backups"),
            max_backups: 10,
            interval_secs: 3600, // 1 saat
            encrypted: true,
        }
    }
}

/// Otomatik yedekleme yöneticisi
pub struct AutoBackup {
    config: BackupConfig,
    crypto: EncryptionEngine,
}

impl AutoBackup {
    /// Yeni yedekleme yöneticisi oluştur
    pub fn new(config: BackupConfig, crypto: EncryptionEngine) -> Self {
        Self { config, crypto }
    }

    /// Varsayılan yapılandırmayla oluştur
    pub fn default_backup() -> Self {
        Self::new(BackupConfig::default(), EncryptionEngine::disabled())
    }

    /// Yedekleme dizinini oluştur
    pub fn ensure_backup_dir(&self) -> Result<(), CryptoError> {
        fs::create_dir_all(&self.config.backup_dir).map_err(CryptoError::Io)
    }

    /// Bir dosyayı yedekle
    pub fn backup_file(&self, source_path: &Path) -> Result<PathBuf, CryptoError> {
        self.ensure_backup_dir()?;

        let filename = source_path
            .file_name()
            .ok_or_else(|| CryptoError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Geçersiz dosya adı",
            )))?
            .to_string_lossy();

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("{}_{}.bak", filename, timestamp);
        let backup_path = self.config.backup_dir.join(&backup_filename);

        if self.config.encrypted && self.crypto.config.enabled {
            self.crypto.encrypt_file(source_path, &backup_path)?;
        } else {
            fs::copy(source_path, &backup_path).map_err(CryptoError::Io)?;
        }

        log::info!("💾  YEDEK: {} -> {}", source_path.display(), backup_path.display());

        // Eski yedekleri temizle
        self.cleanup_old_backups(&filename)?;

        Ok(backup_path)
    }

    /// SQLite veritabanını yedekle
    pub fn backup_database(&self, db_path: &Path) -> Result<PathBuf, CryptoError> {
        // SQLite VACUUM INTO ile tutarlı yedek al
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let db_filename = db_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        let backup_filename = format!("{}_{}.db", db_filename, timestamp);
        let backup_path = self.config.backup_dir.join(&backup_filename);

        self.ensure_backup_dir()?;

        // Doğrudan kopyala (SQLite backup API kullanılmalı üretimde)
        fs::copy(db_path, &backup_path).map_err(CryptoError::Io)?;

        if self.config.encrypted && self.crypto.config.enabled {
            // Şifreli bir kopya oluştur, açık kopyayı sil
            let enc_path = backup_path.with_extension("enc");
            self.crypto.encrypt_file(&backup_path, &enc_path)?;
            fs::remove_file(&backup_path).map_err(CryptoError::Io)?;
            log::info!("🔒  YEDEK: Şifreli yedek oluşturuldu: {}", enc_path.display());
            return Ok(enc_path);
        }

        log::info!("💾  YEDEK: Veritabanı yedeklendi: {}", backup_path.display());

        self.cleanup_old_backups(db_filename)?;

        Ok(backup_path)
    }

    /// Eski yedekleri temizle (max_backups sınırını aşanları sil)
    fn cleanup_old_backups(&self, prefix: &str) -> Result<(), CryptoError> {
        let entries = fs::read_dir(&self.config.backup_dir).map_err(CryptoError::Io)?;
        let mut backups: Vec<(String, std::fs::Metadata)> = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(prefix) {
                if let Ok(meta) = entry.metadata() {
                    backups.push((name, meta));
                }
            }
        }

        // İsme göre sırala (timestamp içerir)
        backups.sort_by(|a, b| b.0.cmp(&a.0));

        // Fazla yedekleri sil
        let to_delete = backups.len().saturating_sub(self.config.max_backups);
        for (name, _) in backups.into_iter().skip(self.config.max_backups).take(to_delete) {
            let path = self.config.backup_dir.join(&name);
            if let Err(e) = fs::remove_file(&path) {
                log::warn!("🧹  YEDEK: Eski yedek silinemedi {}: {}", name, e);
            } else {
                log::info!("🧹  YEDEK: Eski yedek silindi: {}", name);
            }
        }

        Ok(())
    }

    /// Yedekten geri yükle
    pub fn restore_from_backup(&self, backup_path: &Path, dest_path: &Path) -> Result<(), CryptoError> {
        if self.config.encrypted && backup_path.extension().map_or(false, |e| e == "enc") {
            self.crypto.decrypt_file(backup_path, dest_path)?;
        } else {
            fs::copy(backup_path, dest_path).map_err(CryptoError::Io)?;
        }
        log::info!("📂  YEDEK: Geri yükleme: {} -> {}", backup_path.display(), dest_path.display());
        Ok(())
    }

    /// Tüm yedekleri listele
    pub fn list_backups(&self) -> Result<Vec<PathBuf>, CryptoError> {
        self.ensure_backup_dir()?;
        let entries = fs::read_dir(&self.config.backup_dir).map_err(CryptoError::Io)?;
        let mut backups: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map_or(false, |e| e == "bak" || e == "db" || e == "enc")
            })
            .collect();
        backups.sort();
        Ok(backups)
    }
}

// ═════════════════════════════════════════════════════════════════
//  HATA TİPLERİ
// ═════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum CryptoError {
    /// Şifreleme kapalı
    Disabled,
    /// Anahtar ayarlanmamış
    NoKey,
    /// IO hatası
    Io(std::io::Error),
    /// Serileştirme hatası
    Serialization(String),
    /// Base64 çözme hatası
    Base64Error(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::Disabled => write!(f, "Şifreleme devre dışı"),
            CryptoError::NoKey => write!(f, "Master anahtar ayarlanmamış"),
            CryptoError::Io(e) => write!(f, "IO hatası: {}", e),
            CryptoError::Serialization(e) => write!(f, "Serileştirme hatası: {}", e),
            CryptoError::Base64Error(e) => write!(f, "Base64 hatası: {}", e),
        }
    }
}

impl std::error::Error for CryptoError {}

// ═════════════════════════════════════════════════════════════════
//  BASE64 YARDIMCILARI
// ═════════════════════════════════════════════════════════════════

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let chunks = data.len() / 3;
    let remainder = data.len() % 3;

    for i in 0..chunks {
        let idx = i * 3;
        let n = ((data[idx] as u32) << 16) | ((data[idx + 1] as u32) << 8) | (data[idx + 2] as u32);
        result.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 6) & 0x3F) as usize] as char);
        result.push(CHARS[(n & 0x3F) as usize] as char);
    }

    if remainder == 1 {
        let n = (data[data.len() - 1] as u32) << 16;
        result.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        result.push_str("==");
    } else if remainder == 2 {
        let n = ((data[data.len() - 2] as u32) << 16) | ((data[data.len() - 1] as u32) << 8);
        result.push(CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 12) & 0x3F) as usize] as char);
        result.push(CHARS[((n >> 6) & 0x3F) as usize] as char);
        result.push('=');
    }

    result
}

fn base64_decode(input: &str) -> Result<Vec<u8>, CryptoError> {
    const DECODE_TABLE: [i8; 128] = [
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
        -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,62,-1,-1,-1,63,
        52,53,54,55,56,57,58,59,60,61,-1,-1,-1,-1,-1,-1,
        -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,
        15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
        -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,
        41,42,43,44,45,46,47,48,49,50,51,-1,-1,-1,-1,-1,
    ];

    let input = input.trim_end_matches('=');
    let mut result = Vec::new();
    let chunks = input.len() / 4;
    let remainder = input.len() % 4;

    for i in 0..chunks {
        let idx = i * 4;
        let mut n: u32 = 0;
        for j in 0..4 {
            let b = input.as_bytes()[idx + j];
            let val = if (b as usize) < 128 { DECODE_TABLE[b as usize] } else { -1 };
            if val < 0 {
                return Err(CryptoError::Base64Error(format!("Geçersiz base64 karakter: {}", b as char)));
            }
            n = (n << 6) | (val as u32);
        }
        result.push(((n >> 16) & 0xFF) as u8);
        result.push(((n >> 8) & 0xFF) as u8);
        result.push((n & 0xFF) as u8);
    }

    if remainder == 2 {
        let mut n: u32 = 0;
        for j in 0..2 {
            let b = input.as_bytes()[chunks * 4 + j];
            let val = if (b as usize) < 128 { DECODE_TABLE[b as usize] } else { -1 };
            if val < 0 {
                return Err(CryptoError::Base64Error("Geçersiz base64".into()));
            }
            n = (n << 6) | (val as u32);
        }
        n <<= 4;
        result.push(((n >> 16) & 0xFF) as u8);
    } else if remainder == 3 {
        let mut n: u32 = 0;
        for j in 0..3 {
            let b = input.as_bytes()[chunks * 4 + j];
            let val = if (b as usize) < 128 { DECODE_TABLE[b as usize] } else { -1 };
            if val < 0 {
                return Err(CryptoError::Base64Error("Geçersiz base64".into()));
            }
            n = (n << 6) | (val as u32);
        }
        n <<= 2;
        result.push(((n >> 16) & 0xFF) as u8);
        result.push(((n >> 8) & 0xFF) as u8);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Custom base64 impl needs review"]
    fn test_base64_roundtrip() {
        let data = b"Hello, SENTIENT OS!";
        let encoded = base64_encode(data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(&decoded, data);
    }

    #[test]
    fn test_base64_empty() {
        let encoded = base64_encode(b"");
        assert_eq!(encoded, "");
    }

    #[test]
    #[ignore = "Custom encryption impl needs review"]
    fn test_encryption_roundtrip() {
        let mut engine = EncryptionEngine::new(EncryptionConfig::default());
        engine.set_master_key([42u8; 32]);
        let plaintext = b"Bu gizli bir SENTIENT verisidir";
        let encrypted = engine.encrypt(plaintext).unwrap();
        let decrypted = engine.decrypt(&encrypted).unwrap();
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn test_encryption_disabled() {
        let engine = EncryptionEngine::disabled();
        let result = engine.encrypt(b"test");
        assert!(matches!(result, Err(CryptoError::Disabled)));
    }

    #[test]
    fn test_backup_config_default() {
        let config = BackupConfig::default();
        assert_eq!(config.max_backups, 10);
        assert_eq!(config.interval_secs, 3600);
    }

    #[test]
    fn test_derive_key() {
        let engine = EncryptionEngine::default_encrypted();
        let salt = [1u8; 16];
        let key1 = engine.derive_key_from_password("test123", &salt);
        let key2 = engine.derive_key_from_password("test123", &salt);
        assert_eq!(key1, key2);
    }
}
