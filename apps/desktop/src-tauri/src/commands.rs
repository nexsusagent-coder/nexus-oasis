//! ─── Tauri Commands ───

use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use crate::{AppState, AppConfig};

// ─── Configuration ───

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn set_config(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    let mut current = state.config.lock().await;
    *current = config;
    // TODO: Save to disk
    Ok(())
}

// ─── Chat ───

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
}

#[tauri::command]
pub async fn chat(
    state: State<'_, AppState>,
    request: ChatRequest,
) -> Result<ChatResponse, String> {
    let config = state.config.lock().await;
    let api_key = config.api_key.clone().unwrap_or_default();
    let model = request.model.unwrap_or_else(|| config.model.clone());
    drop(config);
    
    // Call OpenAI API
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": model,
        "messages": request.messages,
        "stream": false
    });
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    let message = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Error: No response")
        .to_string();
    
    Ok(ChatResponse {
        message: ChatMessage {
            role: "assistant".into(),
            content: message,
        },
        done: true,
    })
}

#[tauri::command]
pub async fn chat_stream(
    app: AppHandle,
    state: State<'_, AppState>,
    request: ChatRequest,
) -> Result<(), String> {
    // TODO: Implement SSE streaming
    Ok(())
}

#[tauri::command]
pub async fn stop_generation(state: State<'_, AppState>) -> Result<(), String> {
    // TODO: Cancel ongoing request
    Ok(())
}

// ─── Voice ───

#[tauri::command]
pub async fn start_voice(state: State<'_, AppState>) -> Result<(), String> {
    let mut active = state.voice_active.lock().await;
    *active = true;
    Ok(())
}

#[tauri::command]
pub async fn stop_voice(state: State<'_, AppState>) -> Result<(), String> {
    let mut active = state.voice_active.lock().await;
    *active = false;
    Ok(())
}

#[tauri::command]
pub async fn get_voice_status(state: State<'_, AppState>) -> Result<bool, String> {
    let active = state.voice_active.lock().await;
    Ok(*active)
}

// ─── Channels ───

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub name: String,
    pub connected: bool,
    pub unread: u32,
}

#[tauri::command]
pub async fn send_message(
    channel: String,
    recipient: String,
    message: String,
) -> Result<(), String> {
    // TODO: Use sentient_channels
    Ok(())
}

#[tauri::command]
pub async fn get_channels() -> Result<Vec<ChannelInfo>, String> {
    Ok(vec![
        ChannelInfo { name: "telegram".into(), connected: true, unread: 5 },
        ChannelInfo { name: "discord".into(), connected: true, unread: 3 },
        ChannelInfo { name: "whatsapp".into(), connected: false, unread: 0 },
    ])
}

#[tauri::command]
pub async fn connect_channel(channel: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn disconnect_channel(channel: String) -> Result<(), String> {
    Ok(())
}

// ─── Skills ───

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub installed: bool,
}

#[tauri::command]
pub async fn search_skills(query: String) -> Result<Vec<SkillInfo>, String> {
    // TODO: Connect to sentient_marketplace
    Ok(vec![
        SkillInfo {
            id: "code-review".into(),
            name: "Code Review".into(),
            description: "AI-powered code review assistant".into(),
            author: "sentient".into(),
            version: "1.0.0".into(),
            installed: true,
        },
        SkillInfo {
            id: "translator".into(),
            name: "Translator".into(),
            description: "Multi-language translation".into(),
            author: "community".into(),
            version: "2.1.0".into(),
            installed: false,
        },
    ])
}

#[tauri::command]
pub async fn install_skill(skill_id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn list_installed_skills() -> Result<Vec<SkillInfo>, String> {
    Ok(vec![])
}

// ─── System ───

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub platform: String,
    pub arch: String,
    pub rust_version: String,
}

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        version: env!("CARGO_PKG_VERSION").into(),
        platform: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        rust_version: env!("CARGO_PKG_RUST_VERSION").into(),
    })
}

#[tauri::command]
pub async fn open_logs(app: AppHandle) -> Result<(), String> {
    // TODO: Open log directory
    Ok(())
}

#[tauri::command]
pub async fn check_updates() -> Result<Option<String>, String> {
    // TODO: Check GitHub for new releases
    Ok(None)
}
