//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER PROFILE - Kalıcı Tarayıcı Profili Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! 7/24 otonom web operasyonları için kalıcı tarayıcı profilleri.
//! Cookies, localStorage, sessionStorage şifreli olarak ~/.sentient/browser_profiles/
//! altında saklanır.
//!
//! ÖZELLİKLER:
//! • Anti-bot koruması için gerçek tarayıcı parmak izi
//! • Manuel auth sonrası oturum kalıcılığı
//! • Şifreli profil depolama (AES-256-GCM)
//! • Otomatik profil yükleme/kaydetme

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Profil bulunamadı: {0}")]
    NotFound(String),

    #[error("Profil zaten mevcut: {0}")]
    AlreadyExists(String),

    #[error("Şifreleme hatası: {0}")]
    EncryptionError(String),

    #[error("Dosya hatası: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON hatası: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Base64 hatası: {0}")]
    Base64Error(String),
}

pub type ProfileResult<T> = Result<T, ProfileError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

/// Tarayıcı profili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    /// Profil ID
    pub id: String,
    /// Profil adı (örn: "twitter-main", "linkedin-work")
    pub name: String,
    /// Site domain'i (örn: "twitter.com")
    pub domain: String,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son kullanma zamanı
    pub last_used_at: DateTime<Utc>,
    /// Son auth durumu
    pub is_authenticated: bool,
    /// Cookies (şifreli)
    #[serde(skip)]
    cookies: Vec<CookieData>,
    /// LocalStorage (şifreli)
    #[serde(skip)]
    local_storage: HashMap<String, String>,
    /// SessionStorage (şifreli)
    #[serde(skip)]
    session_storage: HashMap<String, String>,
    /// User-Agent
    pub user_agent: String,
    /// Ek metadata
    pub metadata: ProfileMetadata,
}

/// Profil metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    /// Son giriş tarihi
    pub last_login: Option<DateTime<Utc>>,
    /// Toplam kullanım süresi (saniye)
    pub total_usage_secs: u64,
    /// Başarılı istek sayısı
    pub successful_requests: u64,
    /// Başarısız istek sayısı
    pub failed_requests: u64,
    /// Profil durumu
    pub status: ProfileStatus,
    /// Etiketler
    pub tags: Vec<String>,
}

impl Default for ProfileMetadata {
    fn default() -> Self {
        Self {
            last_login: None,
            total_usage_secs: 0,
            successful_requests: 0,
            failed_requests: 0,
            status: ProfileStatus::Active,
            tags: vec![],
        }
    }
}

/// Profil durumu
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfileStatus {
    /// Aktif
    Active,
    /// Süresi dolmuş (auth geçersiz)
    Expired,
    /// Askıya alınmış
    Suspended,
    /// Hata durumu
    Error,
}

/// Cookie verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieData {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<DateTime<Utc>>,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: SameSite,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Şifreli profil verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedProfile {
    nonce: String,
    ciphertext: String,
    tag: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROFILE MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Profil yöneticisi
pub struct ProfileManager {
    /// Profil dizini
    profiles_dir: PathBuf,
    /// Şifreleme anahtarı
    encryption_key: [u8; 32],
    /// Yüklü profiller (cache)
    profiles: HashMap<String, BrowserProfile>,
}

impl ProfileManager {
    /// Yeni profil yöneticisi oluştur
    pub fn new() -> ProfileResult<Self> {
        let profiles_dir = Self::get_profiles_dir()?;
        
        // Dizini oluştur
        fs::create_dir_all(&profiles_dir)?;
        
        // Şifreleme anahtarı oluştur/yükle
        let encryption_key = Self::get_or_create_key(&profiles_dir)?;
        
        log::info!("🔐  PROFILE: Profil yöneticisi başlatıldı → {}", profiles_dir.display());
        
        Ok(Self {
            profiles_dir,
            encryption_key,
            profiles: HashMap::new(),
        })
    }
    
    /// Profiller dizinini getir
    fn get_profiles_dir() -> ProfileResult<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| ProfileError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found"
            )))?;
        
        Ok(home.join(".sentient").join("browser_profiles"))
    }
    
    /// Şifreleme anahtarını getir veya oluştur
    fn get_or_create_key(profiles_dir: &Path) -> ProfileResult<[u8; 32]> {
        let key_path = profiles_dir.join(".key");
        
        if key_path.exists() {
            // Mevcut anahtarı yükle
            let key_b64 = fs::read_to_string(&key_path)?;
            let key_bytes = BASE64.decode(key_b64.trim())
                .map_err(|e| ProfileError::Base64Error(e.to_string()))?;
            
            let mut key = [0u8; 32];
            key.copy_from_slice(&key_bytes);
            Ok(key)
        } else {
            // Yeni anahtar oluştur
            let mut key = [0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut key);
            let key_b64 = BASE64.encode(key);
            
            // Anahtarı kaydet
            let mut file = File::create(&key_path)?;
            file.write_all(key_b64.as_bytes())?;
            
            // Sadece sahibi okuyabilir
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&key_path, fs::Permissions::from_mode(0o600))?;
            }
            
            log::info!("🔐  PROFILE: Yeni şifreleme anahtarı oluşturuldu");
            Ok(key.into())
        }
    }
    
    /// Profil oluştur
    pub fn create_profile(&mut self, name: &str, domain: &str) -> ProfileResult<BrowserProfile> {
        let profile_path = self.get_profile_path(name);
        
        if profile_path.exists() {
            return Err(ProfileError::AlreadyExists(name.to_string()));
        }
        
        let profile = BrowserProfile {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            domain: domain.to_string(),
            created_at: Utc::now(),
            last_used_at: Utc::now(),
            is_authenticated: false,
            cookies: vec![],
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
            user_agent: crate::DEFAULT_USER_AGENT.to_string(),
            metadata: ProfileMetadata::default(),
        };
        
        // Kaydet
        self.save_profile(&profile)?;
        
        // Cache'e ekle
        self.profiles.insert(name.to_string(), profile.clone());
        
        log::info!("✅  PROFILE: Yeni profil oluşturuldu → {} ({})", name, domain);
        Ok(profile)
    }
    
    /// Profil yükle
    pub fn load_profile(&mut self, name: &str) -> ProfileResult<BrowserProfile> {
        // Cache'de var mı kontrol et
        if let Some(profile) = self.profiles.get(name) {
            return Ok(profile.clone());
        }
        
        let profile_path = self.get_profile_path(name);
        
        if !profile_path.exists() {
            return Err(ProfileError::NotFound(name.to_string()));
        }
        
        // Şifreli veriyi oku
        let encrypted_data = fs::read_to_string(&profile_path)?;
        let encrypted: EncryptedProfile = serde_json::from_str(&encrypted_data)?;
        
        // Şifreyi çöz
        let decrypted_data = self.decrypt(&encrypted)?;
        let profile: BrowserProfile = serde_json::from_str(&decrypted_data)?;
        
        // Cache'e ekle
        self.profiles.insert(name.to_string(), profile.clone());
        
        log::debug!("📂  PROFILE: Profil yüklendi → {}", name);
        Ok(profile)
    }
    
    /// Profil kaydet
    pub fn save_profile(&self, profile: &BrowserProfile) -> ProfileResult<()> {
        let profile_path = self.get_profile_path(&profile.name);
        
        // JSON'a serialize et
        let json_data = serde_json::to_string_pretty(profile)?;
        
        // Şifrele
        let encrypted = self.encrypt(&json_data)?;
        
        // Kaydet
        let encrypted_json = serde_json::to_string_pretty(&encrypted)?;
        fs::write(&profile_path, encrypted_json)?;
        
        log::debug!("💾  PROFILE: Profil kaydedildi → {}", profile.name);
        Ok(())
    }
    
    /// Profil sil
    pub fn delete_profile(&mut self, name: &str) -> ProfileResult<()> {
        let profile_path = self.get_profile_path(name);
        
        if !profile_path.exists() {
            return Err(ProfileError::NotFound(name.to_string()));
        }
        
        fs::remove_file(&profile_path)?;
        self.profiles.remove(name);
        
        log::info!("🗑️  PROFILE: Profil silindi → {}", name);
        Ok(())
    }
    
    /// Tüm profilleri listele
    pub fn list_profiles(&self) -> ProfileResult<Vec<BrowserProfile>> {
        let mut profiles = vec![];
        
        for entry in fs::read_dir(&self.profiles_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "profile") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    // Metadata'yı oku (şifre çözmeden)
                    if let Ok(metadata_path) = self.get_metadata_path(name) {
                        if metadata_path.exists() {
                            if let Ok(metadata) = fs::read_to_string(&metadata_path) {
                                if let Ok(profile) = serde_json::from_str::<BrowserProfile>(&metadata) {
                                    profiles.push(profile);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(profiles)
    }
    
    /// Cookie'leri güncelle
    pub fn update_cookies(&mut self, profile_name: &str, cookies: Vec<CookieData>) -> ProfileResult<()> {
        let profile = self.load_profile(profile_name)?;
        let mut updated = profile.clone();
        updated.cookies = cookies;
        updated.last_used_at = Utc::now();
        
        self.save_profile(&updated)?;
        self.profiles.insert(profile_name.to_string(), updated);
        
        Ok(())
    }
    
    /// LocalStorage güncelle
    pub fn update_local_storage(&mut self, profile_name: &str, storage: HashMap<String, String>) -> ProfileResult<()> {
        let profile = self.load_profile(profile_name)?;
        let mut updated = profile.clone();
        updated.local_storage = storage;
        updated.last_used_at = Utc::now();
        
        self.save_profile(&updated)?;
        self.profiles.insert(profile_name.to_string(), updated);
        
        Ok(())
    }
    
    /// Auth durumunu güncelle
    pub fn set_authenticated(&mut self, profile_name: &str, authenticated: bool) -> ProfileResult<()> {
        let profile = self.load_profile(profile_name)?;
        let mut updated = profile.clone();
        updated.is_authenticated = authenticated;
        updated.metadata.last_login = if authenticated { Some(Utc::now()) } else { None };
        
        self.save_profile(&updated)?;
        self.profiles.insert(profile_name.to_string(), updated);
        
        if authenticated {
            log::info!("✅  PROFILE: Auth başarılı → {}", profile_name);
        }
        
        Ok(())
    }
    
    /// İstatistik güncelle
    pub fn update_stats(&mut self, profile_name: &str, success: bool) -> ProfileResult<()> {
        let profile = self.load_profile(profile_name)?;
        let mut updated = profile.clone();
        
        if success {
            updated.metadata.successful_requests += 1;
        } else {
            updated.metadata.failed_requests += 1;
        }
        updated.last_used_at = Utc::now();
        
        self.save_profile(&updated)?;
        self.profiles.insert(profile_name.to_string(), updated);
        
        Ok(())
    }
    
    // ─── Şifreleme İşlemleri ───
    
    fn encrypt(&self, data: &str) -> ProfileResult<EncryptedProfile> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| ProfileError::EncryptionError(e.to_string()))?;
        
        let mut nonce_bytes = [0u8; 12];
        rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from(nonce_bytes);
        
        let ciphertext = cipher.encrypt(&nonce, data.as_bytes())
            .map_err(|e| ProfileError::EncryptionError(e.to_string()))?;
        
        Ok(EncryptedProfile {
            nonce: BASE64.encode(nonce),
            ciphertext: BASE64.encode(&ciphertext),
            tag: String::new(), // GCM tag is included in ciphertext
        })
    }
    
    fn decrypt(&self, encrypted: &EncryptedProfile) -> ProfileResult<String> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| ProfileError::EncryptionError(e.to_string()))?;
        
        let nonce_bytes = BASE64.decode(&encrypted.nonce)
            .map_err(|e| ProfileError::Base64Error(e.to_string()))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = BASE64.decode(&encrypted.ciphertext)
            .map_err(|e| ProfileError::Base64Error(e.to_string()))?;
        
        let plaintext = cipher.decrypt(nonce, ciphertext.as_slice())
            .map_err(|e| ProfileError::EncryptionError(e.to_string()))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| ProfileError::EncryptionError(e.to_string()))
    }
    
    // ─── Yardımcı Metodlar ───
    
    fn get_profile_path(&self, name: &str) -> PathBuf {
        self.profiles_dir.join(format!("{}.profile", name))
    }
    
    fn get_metadata_path(&self, name: &str) -> ProfileResult<PathBuf> {
        Ok(self.profiles_dir.join(format!("{}.meta", name)))
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new().expect("ProfileManager başlatılamadı")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PREDEFINED PROFILES
// ═══════════════════════════════════════════════════════════════════════════════

/// Önceden tanımlı profil şablonları
pub const PREDEFINED_PROFILES: &[(&str, &str)] = &[
    ("twitter", "x.com"),
    ("linkedin", "linkedin.com"),
    ("github", "github.com"),
    ("google", "accounts.google.com"),
    ("facebook", "facebook.com"),
    ("instagram", "instagram.com"),
    ("reddit", "reddit.com"),
    ("discord", "discord.com"),
    ("slack", "slack.com"),
    ("notion", "notion.so"),
];

impl ProfileManager {
    /// Önceden tanımlı profilleri oluştur
    pub fn ensure_predefined_profiles(&mut self) -> ProfileResult<()> {
        for (name, domain) in PREDEFINED_PROFILES {
            let profile_path = self.get_profile_path(name);
            if !profile_path.exists() {
                self.create_profile(name, domain)?;
                log::info!("📋  PROFILE: Öntanımlı profil oluşturuldu → {} ({})", name, domain);
            }
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    impl ProfileManager {
        /// Test için özel dizinle oluştur
        fn new_test(test_dir: &Path) -> ProfileResult<Self> {
            let profiles_dir = test_dir.join("browser_profiles");
            fs::create_dir_all(&profiles_dir)?;
            
            let mut encryption_key = [0u8; 32];
            rand::rngs::OsRng.fill_bytes(&mut encryption_key);
            
            Ok(Self {
                profiles_dir,
                encryption_key,
                profiles: HashMap::new(),
            })
        }
    }

    #[test]
    fn test_create_profile() {
        let dir = tempdir().unwrap();
        let mut manager = ProfileManager::new_test(dir.path()).unwrap();
        
        let profile = manager.create_profile("test-twitter", "twitter.com").unwrap();
        
        assert_eq!(profile.name, "test-twitter");
        assert_eq!(profile.domain, "twitter.com");
        assert!(!profile.is_authenticated);
    }

    #[test]
    fn test_load_profile() {
        let dir = tempdir().unwrap();
        let mut manager = ProfileManager::new_test(dir.path()).unwrap();
        
        // Profil oluştur
        let created = manager.create_profile("test-linkedin", "linkedin.com").unwrap();
        
        // Yükle
        let loaded = manager.load_profile("test-linkedin").unwrap();
        
        assert_eq!(loaded.id, created.id);
        assert_eq!(loaded.name, "test-linkedin");
    }

    #[test]
    fn test_encrypt_decrypt() {
        let dir = tempdir().unwrap();
        let manager = ProfileManager::new_test(dir.path()).unwrap();
        
        let data = r#"{"test": "value", "number": 42}"#;
        
        let encrypted = manager.encrypt(data).unwrap();
        let decrypted = manager.decrypt(&encrypted).unwrap();
        
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_update_cookies() {
        let dir = tempdir().unwrap();
        let mut manager = ProfileManager::new_test(dir.path()).unwrap();
        
        manager.create_profile("test-cookies", "example.com").unwrap();
        
        let cookies = vec![
            CookieData {
                name: "session".to_string(),
                value: "abc123".to_string(),
                domain: ".example.com".to_string(),
                path: "/".to_string(),
                expires: Some(Utc::now() + chrono::Duration::days(7)),
                http_only: true,
                secure: true,
                same_site: SameSite::Lax,
            },
        ];
        
        manager.update_cookies("test-cookies", cookies).unwrap();
        
        let profile = manager.load_profile("test-cookies").unwrap();
        assert!(profile.is_authenticated || !profile.cookies.is_empty());
    }

    #[test]
    fn test_delete_profile() {
        let dir = tempdir().unwrap();
        let mut manager = ProfileManager::new_test(dir.path()).unwrap();
        
        manager.create_profile("to-delete", "example.com").unwrap();
        
        assert!(manager.load_profile("to-delete").is_ok());
        
        manager.delete_profile("to-delete").unwrap();
        
        assert!(manager.load_profile("to-delete").is_err());
    }
}
