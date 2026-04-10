//! API routes

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    response::{IntoResponse, Json},
    Extension,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::*;
use crate::{ApiResponse, HealthResponse};

/// Health check endpoint
pub async fn health(
    Extension(state): Extension<Arc<super::server::ServerState>>,
) -> impl IntoResponse {
    let uptime = state.start_time.elapsed().as_secs();

    Json(HealthResponse::healthy(crate::WEB_VERSION, uptime)
        .with_component("api", true)
        .with_component("memory", true))
}

/// Status endpoint
pub async fn status() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "version": crate::WEB_VERSION,
        "status": "running",
    })))
}

/// Auth login
pub async fn auth_login() -> impl IntoResponse {
    // Placeholder - would validate credentials and return JWT
    Json(ApiResponse::success(serde_json::json!({
        "token": "jwt-token-placeholder",
        "expires_in": 3600,
    })))
}

/// Auth logout
pub async fn auth_logout() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// Auth refresh
pub async fn auth_refresh() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "token": "new-jwt-token",
        "expires_in": 3600,
    })))
}

/// Query parameters for list endpoints
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub search: Option<String>,
}

/// Users list
pub async fn users_list(
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    let users = vec![
        User::new("user1").with_email("user1@example.com"),
        User::new("user2").with_email("user2@example.com"),
    ];

    Json(ApiResponse::success(serde_json::json!({
        "users": users,
        "pagination": Pagination::new(
            query.page.unwrap_or(1),
            query.per_page.unwrap_or(20),
            2
        )
    })))
}

/// Get user
pub async fn users_get(
    Path(id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::success(User::new(&id)))
}

/// Update user
pub async fn users_update(
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "message": "User updated"
    })))
}

/// Delete user
pub async fn users_delete(
    Path(_id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "message": "User deleted"
    })))
}

/// Agents list
pub async fn agents_list() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "agents": []
    })))
}

/// Create agent
pub async fn agents_create() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "id": "agent-123",
        "status": "created"
    })))
}

/// Get agent
pub async fn agents_get(
    Path(id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "id": id,
        "name": "Agent",
        "status": "ready"
    })))
}

/// Chat request
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub conversation_id: Option<String>,
}

/// Chat with agent
pub async fn agents_chat(
    Path(_id): Path<String>,
    Json(_req): Json<ChatRequest>,
) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "response": "Hello! How can I help you?",
        "conversation_id": "conv-123"
    })))
}

/// Stream with agent (SSE placeholder)
pub async fn agents_stream(
    Path(_id): Path<String>,
) -> impl IntoResponse {
    // Would return SSE stream
    Json(ApiResponse::success(serde_json::json!({
        "stream_url": "/api/v1/stream/agent-123"
    })))
}

/// Skills list
pub async fn skills_list() -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "skills": []
    })))
}

/// Get skill
pub async fn skills_get(
    Path(id): Path<String>,
) -> impl IntoResponse {
    Json(ApiResponse::success(serde_json::json!({
        "id": id,
        "name": "Skill",
        "description": "A skill"
    })))
}

/// WebSocket handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket))
}

/// Handle WebSocket connection
async fn handle_websocket(
    mut socket: axum::extract::ws::WebSocket,
) {
    use axum::extract::ws::Message;

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    // Echo back
                    let response = WsMessage::Chat {
                        content: format!("Echo: {}", text),
                        conversation_id: None,
                    };

                    let json = serde_json::to_string(&response).unwrap();
                    if socket.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
                Message::Ping(data) => {
                    if socket.send(Message::Pong(data)).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_query() {
        let query = ListQuery {
            page: Some(2),
            per_page: Some(10),
            search: Some("test".to_string()),
        };

        assert_eq!(query.page, Some(2));
    }

    #[test]
    fn test_chat_request() {
        let req = ChatRequest {
            message: "Hello".to_string(),
            conversation_id: Some("conv-1".to_string()),
        };

        assert_eq!(req.message, "Hello");
    }
}
