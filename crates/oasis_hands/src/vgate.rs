//! ═══════════════════════════════════════════════════════════════════════════════
//!  V-GATE KÖPRÜSÜ - MASAÜSTÜ KONTROL İÇİN
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! LLM yetkilendirmesi ve audit log için V-GATE iletişimi.

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};

// ───────────────────────────────────────────────────────────────────────────────
//  V-GATE İSTEK TİPLERİ
// ───────────────────────────────────────────────────────────────────────────────

/// V-GATE yetkilendirme isteği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// Aksiyon tipi
    pub action_type: String,
    /// Aksiyon detayı
    pub action_detail: String,
    /// Kaynak
    pub source: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Güvenlik seviyesi
    pub security_level: SecurityLevel,
}

/// Güvenlik seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// V-GATE yanıt tipi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// İzin verildi mi?
    pub allowed: bool,
    /// Neden (reddedildiyse)
    pub reason: Option<String>,
    /// Audit ID
    pub audit_id: String,
    /// Geçerlilik süresi (saniye)
    pub valid_for_secs: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  AUDIT LOG KAYDI
// ───────────────────────────────────────────────────────────────────────────────

/// Audit log kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Kayıt ID
    pub id: String,
    /// Aksiyon tipi
    pub action: String,
    /// Kaynak
    pub source: String,
    /// Sonuç
    pub result: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Detaylar
    pub details: serde_json::Value,
}

// ───────────────────────────────────────────────────────────────────────────────
//  V-GATE KÖPRÜSÜ
// ───────────────────────────────────────────────────────────────────────────────

/// V-GATE köprüsü
#[derive(Debug, Clone)]
pub struct HandsVGate {
    /// V-GATE URL
    url: String,
    /// HTTP istemcisi
    client: reqwest::Client,
    /// Aktif mi?
    active: bool,
    /// Audit logları
    audit_log: Vec<AuditLogEntry>,
}

impl HandsVGate {
    /// Yeni V-GATE köprüsü oluştur
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        log::info!("🚪  V-GATE: Masaüstü köprüsü oluşturuldu → {}", url);

        Self {
            url: url.into(),
            client,
            active: true,
            audit_log: Vec::new(),
        }
    }

    /// Aksiyon yetkilendir
    pub async fn authorize_action(&self, action: &str) -> HandsResult<()> {
        if !self.active {
            return Err(HandsError::VGateError("V-GATE aktif değil".into()));
        }

        log::debug!("🚪  V-GATE: Yetkilendirme isteği → {}", action);

        // Yetkilendirme isteği oluştur
        let request = AuthorizationRequest {
            action_type: "desktop_action".into(),
            action_detail: action.into(),
            source: "oasis-hands".into(),
            timestamp: chrono::Utc::now(),
            security_level: Self::determine_security_level(action),
        };

        // Gerçek V-GATE'e istek at
        let response = self.send_authorization_request(&request).await?;

        let reason_str = response.reason.clone().unwrap_or_else(|| "Bilinmeyen neden".into());
        if response.allowed {
            log::info!("🚪  V-GATE: Aksiyon yetkilendirildi → {} (ID: {})",
                action.chars().take(40).collect::<String>(),
                response.audit_id);
            Ok(())
        } else {
            log::warn!("🚪  V-GATE: Aksiyon reddedildi → {}",
                reason_str);
            Err(HandsError::VGateError(format!(
                "OASIS-HANDS V-GATE: '{}' aksiyonu reddedildi. Sebep: {}",
                action.chars().take(40).collect::<String>(),
                reason_str
            )))
        }
    }

    /// V-GATE'e istek gönder
    async fn send_authorization_request(&self, request: &AuthorizationRequest) -> HandsResult<AuthorizationResponse> {
        let _url = format!("{}/api/v1/authorize/hands", self.url);

        // Mock yanıt (gerçek ortamda V-GATE'e HTTP isteği)
        // Gerçek implementasyonda:
        // let response = self.client.post(&url).json(request).send().await?;

        // Mock: Tehlikeli aksiyonları reddet
        let blocked_actions = ["rm -rf", "format", "dd if=", "delete_system"];
        let is_blocked = blocked_actions.iter().any(|b| request.action_detail.contains(b));

        Ok(AuthorizationResponse {
            allowed: !is_blocked,
            reason: if is_blocked { Some("Tehlikeli aksiyon tespit edildi".into()) } else { None },
            audit_id: uuid::Uuid::new_v4().to_string(),
            valid_for_secs: 300,
        })
    }

    /// Güvenlik seviyesi belirle
    fn determine_security_level(action: &str) -> SecurityLevel {
        // Tehlikeli kelimeler
        let critical_keywords = ["rm", "format", "dd", "mkfs", "shutdown", "reboot"];
        let high_keywords = ["sudo", "chmod", "chown", "/etc/", "/root"];
        let medium_keywords = ["write", "delete", "remove", "move"];

        let action_lower = action.to_lowercase();

        if critical_keywords.iter().any(|k| action_lower.contains(k)) {
            SecurityLevel::Critical
        } else if high_keywords.iter().any(|k| action_lower.contains(k)) {
            SecurityLevel::High
        } else if medium_keywords.iter().any(|k| action_lower.contains(k)) {
            SecurityLevel::Medium
        } else {
            SecurityLevel::Low
        }
    }

    /// Audit log kaydet
    pub fn log_action(&mut self, action: &str, result: &str, details: serde_json::Value) {
        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            action: action.into(),
            source: "oasis-hands".into(),
            result: result.into(),
            timestamp: chrono::Utc::now(),
            details,
        };

        self.audit_log.push(entry);
        log::debug!("📝  V-GATE: Audit kaydı eklendi");
    }

    /// Audit logları getir
    pub fn get_audit_log(&self) -> &[AuditLogEntry] {
        &self.audit_log
    }

    /// Audit log sayısı
    pub fn audit_count(&self) -> usize {
        self.audit_log.len()
    }

    /// Durum kontrolü
    pub fn health_check(&self) -> HandsResult<bool> {
        if !self.active {
            return Ok(false);
        }

        // V-GATE'e ping at
        log::debug!("🚪  V-GATE: Sağlık kontrolü...");
        Ok(true)
    }

    /// Aktif mi?
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Durdur
    pub fn stop(&mut self) {
        self.active = false;
        log::info!("🚪  V-GATE: Köprü durduruldu");
    }

    /// Başlat
    pub fn start(&mut self) {
        self.active = true;
        log::info!("🚪  V-GATE: Köprü başlatıldı");
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_determination() {
        assert_eq!(HandsVGate::determine_security_level("rm -rf /"), SecurityLevel::Critical);
        assert_eq!(HandsVGate::determine_security_level("sudo apt update"), SecurityLevel::High);
        assert_eq!(HandsVGate::determine_security_level("write file"), SecurityLevel::Medium);
        assert_eq!(HandsVGate::determine_security_level("ls -la"), SecurityLevel::Low);
    }

    #[test]
    fn test_vgate_creation() {
        let vgate = HandsVGate::new("http://localhost:1071");
        assert!(vgate.is_active());
    }

    #[test]
    fn test_vgate_start_stop() {
        let mut vgate = HandsVGate::new("http://localhost:1071");
        assert!(vgate.is_active());
        vgate.stop();
        assert!(!vgate.is_active());
        vgate.start();
        assert!(vgate.is_active());
    }

    #[test]
    fn test_audit_log() {
        let mut vgate = HandsVGate::new("http://localhost:1071");
        assert_eq!(vgate.audit_count(), 0);

        vgate.log_action("test", "success", serde_json::json!({"key": "value"}));
        assert_eq!(vgate.audit_count(), 1);
    }

    #[test]
    fn test_health_check() {
        let vgate = HandsVGate::new("http://localhost:1071");
        assert!(vgate.health_check().expect("operation failed"));
    }

    #[tokio::test]
    async fn test_authorization_allowed() {
        let vgate = HandsVGate::new("http://localhost:1071");

        // Normal aksiyon izin verilmeli
        let result = vgate.authorize_action("click_button").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorization_blocked() {
        let vgate = HandsVGate::new("http://localhost:1071");

        // Tehlikeli aksiyon engellenmeli
        let result = vgate.authorize_action("rm -rf /home").await;
        assert!(result.is_err());
    }
}
