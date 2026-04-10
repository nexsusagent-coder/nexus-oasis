//! Middleware

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::auth::AuthService;
use crate::types::*;

/// Authentication middleware
pub async fn auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    // Check for Authorization header
    let auth_header = request.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = &header[7..];

            // Get auth service from extension
            if let Some(auth) = request.extensions().get::<Arc<AuthService>>() {
                if let Ok(claims) = auth.validate_token(token) {
                    // Add claims to request
                    let mut request = request;
                    request.extensions_mut().insert(claims);
                    return next.run(request).await;
                }
            }
        }
    }

    // Unauthorized
    (StatusCode::UNAUTHORIZED, Json(json!({
        "error": "Unauthorized",
        "status": 401
    }))).into_response()
}

/// Request ID middleware
pub async fn request_id_middleware(
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Add to extensions
    request.extensions_mut().insert(request_id.clone());

    let mut response = next.run(request).await;

    // Add to response headers
    response.headers_mut().insert(
        "X-Request-ID",
        request_id.parse().unwrap(),
    );

    response
}

/// Logging middleware
pub async fn logging_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    tracing::info!(
        "{} {} - {}ms",
        method,
        uri,
        duration.as_millis()
    );

    response
}

/// Rate limit state
#[derive(Debug, Clone, Default)]
pub struct RateLimitState {
    requests: std::collections::HashMap<String, Vec<std::time::Instant>>,
}

/// Rate limit middleware
pub async fn rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    // Get client IP (simplified)
    let client_ip = request.headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    // Check rate limit (simplified - would use proper state)
    // For now, just pass through

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_state() {
        let state = RateLimitState::default();
        assert!(state.requests.is_empty());
    }
}
