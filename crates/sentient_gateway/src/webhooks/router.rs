//! ─── WEBHOOK ROUTER ───
//!
//! Gelen webhook'ları provider'a göre ilgili handler'a yönlendirir.
//! Event parsing ve transformation yapar.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{
    WebhookProvider, WebhookResult, WebhookEvent,
    providers::{GitHubPayload, StripePayload, N8nPayload, SlackPayload, GenericPayload, WebhookPayload},
    signature::SignatureVerifier,
    event::EventType,
};

/// ─── WEBHOOK ROUTE ───

#[derive(Debug, Clone)]
pub struct WebhookRoute {
    /// Provider
    pub provider: WebhookProvider,
    
    /// Endpoint path
    pub path: String,
    
    /// Secret (opsiyonel)
    pub secret: Option<String>,
    
    /// Aktif mi?
    pub enabled: bool,
    
    /// Otomatik görev oluştur
    pub auto_create_task: bool,
    
    /// Event filtresi (sadece bu event'leri işle)
    pub event_filter: Vec<String>,
}

impl WebhookRoute {
    pub fn new(provider: WebhookProvider) -> Self {
        let path = format!("/webhook/{}", provider);
        Self {
            provider,
            path,
            secret: None,
            enabled: true,
            auto_create_task: true,
            event_filter: vec![],
        }
    }
    
    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }
    
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }
    
    pub fn filter_events(mut self, events: Vec<&str>) -> Self {
        self.event_filter = events.into_iter().map(|s| s.into()).collect();
        self
    }
    
    pub fn disable_auto_task(mut self) -> Self {
        self.auto_create_task = false;
        self
    }
}

/// ─── WEBHOOK ROUTER ───

pub struct WebhookRouter {
    /// Rotalar
    routes: Arc<RwLock<Vec<WebhookRoute>>>,
    
    /// İmza doğrulayıcı
    signature_verifier: Arc<RwLock<SignatureVerifier>>,
    
    /// Event sender (Orchestrator'a)
    event_sender: Option<tokio::sync::mpsc::Sender<WebhookEvent>>,
}

impl WebhookRouter {
    /// Yeni router oluştur
    pub fn new(vgate_url: impl Into<String>) -> Self {
        Self {
            routes: Arc::new(RwLock::new(Vec::new())),
            signature_verifier: Arc::new(RwLock::new(
                SignatureVerifier::new(vgate_url)
            )),
            event_sender: None,
        }
    }
    
    /// Event sender ayarla
    pub fn with_event_sender(mut self, sender: tokio::sync::mpsc::Sender<WebhookEvent>) -> Self {
        self.event_sender = Some(sender);
        self
    }
    
    /// Rota ekle
    pub async fn add_route(&self, route: WebhookRoute) {
        // Secret'ı verifier'a ekle
        if let Some(secret) = &route.secret {
            let mut verifier = self.signature_verifier.write().await;
            // Secret'ı sakla
            verifier.secrets.insert(route.provider.clone(), secret.clone());
        }
        
        self.routes.write().await.push(route);
    }
    
    /// Varsayılan rotaları ekle
    pub async fn setup_defaults(&self) {
        self.add_route(WebhookRoute::new(WebhookProvider::GitHub)).await;
        self.add_route(WebhookRoute::new(WebhookProvider::Stripe)).await;
        self.add_route(WebhookRoute::new(WebhookProvider::N8n)).await;
        self.add_route(WebhookRoute::new(WebhookProvider::Slack)).await;
    }
    
    /// Webhook'u yönlendir ve işle
    pub async fn route(
        &self,
        provider: &str,
        headers: &std::collections::HashMap<String, String>,
        body: &str,
    ) -> SENTIENTResult<WebhookResult> {
        let start = std::time::Instant::now();
        
        // Provider'ı parse et
        let provider: WebhookProvider = provider.parse()
            .map_err(|e| SENTIENTError::ValidationError(format!("Bilinmeyen provider: {}", e)))?;
        
        // Rotayı bul
        let route = self.routes.read().await
            .iter()
            .find(|r| r.provider == provider && r.enabled)
            .cloned()
            .ok_or_else(|| SENTIENTError::General(format!("{} için aktif rota yok", provider)))?;
        
        // İmza doğrulama
        let signature = self.extract_signature(provider.clone(), headers, body).await?;
        if signature.is_some() || route.secret.is_some() {
            let verifier = self.signature_verifier.read().await;
            let sig = signature.as_deref().unwrap_or("");
            verifier.verify(&provider, body.as_bytes(), sig)?;
            log::info!("webhook  İmza doğrulandı: {}", provider);
        }
        
        // Event'i parse et
        let event = self.parse_event(&provider, headers, body, &route).await?;
        
        log::info!("webhook  Event alındı: {}", event.summary());
        
        // Event'i gönder
        if let Some(sender) = &self.event_sender {
            sender.send(event.clone()).await
                .map_err(|e| SENTIENTError::General(format!("Event gönderilemedi: {}", e)))?;
        }
        
        let result = WebhookResult::success(event.summary())
            .with_duration(start.elapsed().as_millis() as u64);
        
        Ok(result)
    }
    
    /// İmza header'ını çıkar
    async fn extract_signature(
        &self,
        provider: WebhookProvider,
        headers: &std::collections::HashMap<String, String>,
        _body: &str,
    ) -> SENTIENTResult<Option<String>> {
        let signature = match provider {
            WebhookProvider::GitHub => {
                headers.get("x-hub-signature-256")
                    .or_else(|| headers.get("x-hub-signature"))
                    .cloned()
            }
            WebhookProvider::Stripe => {
                headers.get("stripe-signature").cloned()
            }
            WebhookProvider::Slack => {
                headers.get("x-slack-signature")
                    .map(|s| format!("v0={}", s))
            }
            _ => headers.get("x-signature")
                .or_else(|| headers.get("signature"))
                .cloned(),
        };
        
        Ok(signature)
    }
    
    /// Event'i parse et
    async fn parse_event(
        &self,
        provider: &WebhookProvider,
        headers: &std::collections::HashMap<String, String>,
        body: &str,
        route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        match provider {
            WebhookProvider::GitHub => {
                self.parse_github_event(headers, body, route).await
            }
            WebhookProvider::Stripe => {
                self.parse_stripe_event(body, route).await
            }
            WebhookProvider::N8n => {
                self.parse_n8n_event(body, route).await
            }
            WebhookProvider::Slack => {
                self.parse_slack_event(headers, body, route).await
            }
            _ => {
                self.parse_generic_event(provider, body, route).await
            }
        }
    }
    
    /// GitHub event parse
    async fn parse_github_event(
        &self,
        headers: &std::collections::HashMap<String, String>,
        body: &str,
        _route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        let event_type = headers.get("x-github-event")
            .map(|s| s.as_str())
            .unwrap_or("unknown");
        
        let mut payload: GitHubPayload = serde_json::from_str(body)
            .map_err(|e| SENTIENTError::ValidationError(format!("GitHub payload parse hatası: {}", e)))?;
        payload.raw = body.to_string();
        
        let event = match event_type {
            "push" => {
                let branch = payload.r#ref.as_ref()
                    .and_then(|r| r.strip_prefix("refs/heads/").map(|s| s.to_string()))
                    .unwrap_or_else(|| "unknown".into());
                let repo = payload.repository.as_ref()
                    .map(|r| r.full_name.as_str())
                    .unwrap_or("unknown");
                
                WebhookEvent::github_push(repo, &branch, payload.commits.len())
                    .with_payload(payload.as_json()?)
            }
            "pull_request" => {
                let repo = payload.repository.as_ref()
                    .map(|r| r.full_name.as_str())
                    .unwrap_or("unknown");
                let action = payload.action.as_deref().unwrap_or("unknown");
                let pr = payload.pull_request.as_ref();
                
                WebhookEvent::github_pr(
                    repo,
                    pr.map(|p| p.number).unwrap_or(0),
                    action,
                    pr.map(|p| p.title.as_str()).unwrap_or("")
                ).with_payload(payload.as_json()?)
            }
            "issues" => {
                let repo = payload.repository.as_ref()
                    .map(|r| r.full_name.as_str())
                    .unwrap_or("unknown");
                let action = payload.action.as_deref().unwrap_or("unknown");
                let issue = payload.issue.as_ref();
                
                WebhookEvent::github_issue(
                    repo,
                    issue.map(|i| i.number).unwrap_or(0),
                    action,
                    issue.map(|i| i.title.as_str()).unwrap_or("")
                ).with_payload(payload.as_json()?)
            }
            _ => {
                WebhookEvent::new(WebhookProvider::GitHub, EventType::CustomEvent, event_type)
                    .with_message(payload.summary())
                    .with_payload(payload.as_json()?)
            }
        };
        
        Ok(event)
    }
    
    /// Stripe event parse
    async fn parse_stripe_event(
        &self,
        body: &str,
        _route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        let mut payload: StripePayload = serde_json::from_str(body)
            .map_err(|e| SENTIENTError::ValidationError(format!("Stripe payload parse hatası: {}", e)))?;
        payload.raw = body.to_string();
        
        let event = WebhookEvent::stripe_payment(
            &payload.event_type,
            None,
            None
        ).with_payload(payload.as_json()?);
        
        Ok(event)
    }
    
    /// n8n event parse
    async fn parse_n8n_event(
        &self,
        body: &str,
        _route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        let mut payload: N8nPayload = serde_json::from_str(body)
            .map_err(|e| SENTIENTError::ValidationError(format!("n8n payload parse hatası: {}", e)))?;
        payload.raw = body.to_string();
        
        let event = WebhookEvent::n8n_workflow(
            payload.workflow_name.as_deref().unwrap_or("unknown"),
            payload.event.as_deref().unwrap_or("trigger"),
            payload.as_json()?
        );
        
        Ok(event)
    }
    
    /// Slack event parse
    async fn parse_slack_event(
        &self,
        _headers: &std::collections::HashMap<String, String>,
        body: &str,
        _route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        let mut payload: SlackPayload = serde_json::from_str(body)
            .map_err(|e| SENTIENTError::ValidationError(format!("Slack payload parse hatası: {}", e)))?;
        payload.raw = body.to_string();
        
        // URL verification challenge
        if payload.event_type == "url_verification" {
            return Ok(WebhookEvent::new(WebhookProvider::Slack, EventType::HealthCheck, "url_verification")
                .with_message("Slack URL verification")
                .with_payload(serde_json::json!({"challenge": payload.challenge}))
            );
        }
        
        // Message event
        let event = if let Some(slack_event) = &payload.event {
            WebhookEvent::slack_message(
                slack_event.channel.as_deref().unwrap_or("unknown"),
                slack_event.user.as_deref().unwrap_or("unknown"),
                slack_event.text.as_deref().unwrap_or("")
            ).with_payload(payload.as_json()?)
        } else {
            WebhookEvent::new(WebhookProvider::Slack, EventType::Message, &payload.event_type)
                .with_message(payload.summary())
                .with_payload(payload.as_json()?)
        };
        
        Ok(event)
    }
    
    /// Generic event parse
    async fn parse_generic_event(
        &self,
        provider: &WebhookProvider,
        body: &str,
        _route: &WebhookRoute,
    ) -> SENTIENTResult<WebhookEvent> {
        let mut payload: GenericPayload = serde_json::from_str(body)
            .unwrap_or_else(|_| GenericPayload {
                provider: provider.to_string(),
                event_type: "unknown".into(),
                data: std::collections::HashMap::new(),
                raw: body.to_string(),
            });
        payload.raw = body.to_string();
        
        let event = WebhookEvent::new(
            provider.clone(),
            EventType::CustomEvent,
            &payload.event_type
        )
        .with_message(payload.summary())
        .with_payload(payload.as_json()?);
        
        Ok(event)
    }
    
    /// Rotaları listele
    pub async fn list_routes(&self) -> Vec<WebhookRoute> {
        self.routes.read().await.clone()
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webhook_route_creation() {
        let route = WebhookRoute::new(WebhookProvider::GitHub)
            .with_secret("gh_secret")
            .with_path("/custom/github");
        
        assert_eq!(route.provider, WebhookProvider::GitHub);
        assert_eq!(route.path, "/custom/github");
        assert!(route.secret.is_some());
        assert!(route.enabled);
    }
    
    #[test]
    fn test_route_event_filter() {
        let route = WebhookRoute::new(WebhookProvider::GitHub)
            .filter_events(vec!["push", "pull_request"]);
        
        assert_eq!(route.event_filter.len(), 2);
        assert!(route.event_filter.contains(&"push".to_string()));
    }
    
    #[tokio::test]
    async fn test_router_creation() {
        let router = WebhookRouter::new("http://localhost:1071");
        router.setup_defaults().await;
        
        let routes = router.list_routes().await;
        assert_eq!(routes.len(), 4);
    }
}
