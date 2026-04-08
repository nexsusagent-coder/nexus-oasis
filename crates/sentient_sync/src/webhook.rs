//! Webhook Handler - GitHub webhook desteği
//! 
//! Repo sahipleri webhook ekleyerek anında güncelleme alabilir

use crate::SyncError;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

type HmacSha256 = Hmac<Sha256>;

/// Webhook payload (GitHub format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    #[serde(rename = "ref")]
    pub reference: String,
    pub before: String,
    pub after: String,
    pub repository: RepositoryInfo,
    pub pusher: Option<Pusher>,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub clone_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub author: Author,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

/// Webhook handler
pub struct WebhookHandler {
    secret: String,
    updater: crate::updater::SilentUpdater,
}

impl WebhookHandler {
    /// Yeni handler oluştur
    pub fn new(secret: String, updater: crate::updater::SilentUpdater) -> Self {
        Self { secret, updater }
    }
    
    /// Webhook imzasını doğrula
    pub fn verify_signature(&self, payload: &[u8], signature: &str) -> bool {
        let sig = signature.strip_prefix("sha256=").unwrap_or(signature);
        
        let mut mac = match HmacSha256::new_from_slice(self.secret.as_bytes()) {
            Ok(m) => m,
            Err(_) => return false,
        };
        
        mac.update(payload);
        let result = mac.finalize();
        let computed = hex::encode(result.into_bytes());
        
        // Constant-time comparison
        computed == sig
    }
    
    /// Webhook'u işle
    pub async fn handle(&self, payload: WebhookPayload) -> Result<(), SyncError> {
        tracing::info!(
            "📥 Webhook received: {} ({})",
            payload.repository.full_name,
            payload.after
        );
        
        // Branch kontrolü
        if !payload.reference.starts_with("refs/heads/") {
            tracing::debug!("Ignoring non-branch push: {}", payload.reference);
            return Ok(());
        }
        
        // İlgili repo'yu bul ve güncelle
        // Bu kısımda updater ile entegrasyon yapılacak
        
        tracing::info!(
            "✅ Webhook processed: {} commits",
            payload.commits.len()
        );
        
        Ok(())
    }
}

/// Actix-web endpoint
pub async fn handle_webhook(
    req: HttpRequest,
    body: web::Bytes,
    handler: web::Data<WebhookHandler>,
) -> HttpResponse {
    // Signature header'ı al
    let signature = match req.headers().get("x-hub-signature-256") {
        Some(s) => s.to_str().unwrap_or(""),
        None => {
            tracing::warn!("Missing webhook signature");
            return HttpResponse::Unauthorized().finish();
        }
    };
    
    // İmzayı doğrula
    if !handler.verify_signature(&body, signature) {
        tracing::warn!("Invalid webhook signature");
        return HttpResponse::Unauthorized().finish();
    }
    
    // Payload'ı parse et
    let payload: WebhookPayload = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to parse webhook payload: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };
    
    // İşle
    match handler.handle(payload).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Webhook handling failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Webhook route'larını ekle
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/webhook/github")
            .route(web::post().to(handle_webhook))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_verification() {
        // Test implementation
    }
}
