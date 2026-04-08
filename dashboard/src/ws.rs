//! ═══════════════════════════════════════════════════════════════════════════════
//!  WEBSOCKET - Real-time Stats Streaming
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::Dashboard;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(dashboard): State<Dashboard>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, dashboard))
}

async fn handle_socket(socket: WebSocket, dashboard: Dashboard) {
    let (tx, mut rx) = socket.split();
    
    // tx'i Arc ile sar
    let tx = std::sync::Arc::new(tokio::sync::Mutex::new(tx));
    let tx_clone = tx.clone();
    
    // İlk mesajı gönder
    let stats = get_live_stats(&dashboard).await;
    let _ = tx.lock().await.send(Message::Text(stats)).await;
    
    // Periyodik güncelleme task'i
    let send_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            
            let stats = get_live_stats(&dashboard).await;
            
            // Rastgele log ekle
            if rand::random() && rand::random() {
                let messages = vec![
                    ("INFO", "SKILL", "Skill execution completed"),
                    ("DEBUG", "V-GATE", "Proxy request forwarded"),
                    ("INFO", "MEMORY", "Context window updated"),
                    ("WARN", "SYSTEM", "High CPU usage detected"),
                    ("INFO", "BROWSER", "Page loaded successfully"),
                ];
                let idx = rand::random::<usize>() % messages.len();
                dashboard.add_log(messages[idx].0, messages[idx].1, messages[idx].2).await;
            }
            
            let mut tx_guard = tx.lock().await;
            if tx_guard.send(Message::Text(stats)).await.is_err() {
                break;
            }
        }
    });
    
    // Receive loop
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = rx.next().await {
            if let Message::Text(text) = msg {
                if text == "ping" {
                    let mut tx_guard = tx_clone.lock().await;
                    if tx_guard.send(Message::Text("pong".to_string())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

async fn get_live_stats(dashboard: &Dashboard) -> String {
    // Sistem istatistiklerini güncelle
    dashboard.update_system_stats().await;
    
    let stats = dashboard.stats.read().await;
    let logs = dashboard.logs.read().await;
    let vgate = *dashboard.vgate_connected.read().await;
    
    json!({
        "type": "stats",
        "data": {
            "total_skills": stats.total_skills,
            "loaded_skills": stats.loaded_skills,
            "available_tools": stats.available_tools,
            "active_tasks": stats.active_tasks,
            "memory_usage_mb": (stats.memory_usage_mb as u64),
            "cpu_usage_percent": (stats.cpu_usage_percent as u32),
            "uptime_secs": stats.uptime_secs,
            "vgate_connected": vgate,
            "logs": logs.iter().rev().take(10).cloned().collect::<Vec<_>>()
        }
    }).to_string()
}
