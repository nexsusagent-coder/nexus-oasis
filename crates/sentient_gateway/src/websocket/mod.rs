//! ─── WEBSOCKET MODULE ───
//!
//! Gerçek zamanlı görev takibi için WebSocket desteği:
//! - Görev oluşturma
//! - Durum güncelleme akışı
//! - Sonuç yayını

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::task_manager::{ManagedTask, TaskStatus};

/// ─── WEBSOCKET MESSAGE TYPES ───

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// İstemciden sunucuya
    ClientMessage(ClientMessage),
    
    /// Sunucudan istemciye
    ServerMessage(ServerMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Yeni görev oluştur
    CreateTask {
        goal: String,
        #[serde(default)]
        model: Option<String>,
        #[serde(default)]
        params: serde_json::Value,
    },
    
    /// Görev durumu iste
    GetTask {
        task_id: Uuid,
    },
    
    /// Görev aboneliği
    Subscribe {
        task_id: Uuid,
    },
    
    /// Abonelik iptal
    Unsubscribe {
        task_id: Uuid,
    },
    
    /// Tüm aktif görevleri iste
    ListTasks,
    
    /// İstatistikler
    GetStats,
    
    /// Ping
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Görev oluşturuldu
    TaskCreated {
        task_id: Uuid,
        queue_position: usize,
    },
    
    /// Görev durumu güncellendi
    TaskStatusUpdate {
        task_id: Uuid,
        status: TaskStatus,
        progress: f32,
        timestamp: DateTime<Utc>,
    },
    
    /// Görev tamamlandı
    TaskCompleted {
        task_id: Uuid,
        result: serde_json::Value,
        duration_secs: i64,
    },
    
    /// Görev hatası
    TaskFailed {
        task_id: Uuid,
        error: String,
    },
    
    /// Görev listesi
    TaskList {
        tasks: Vec<ManagedTask>,
    },
    
    /// İstatistikler
    Stats {
        stats: crate::GatewayStats,
    },
    
    /// Hata
    Error {
        code: u16,
        message: String,
    },
    
    /// Pong
    Pong {
        timestamp: DateTime<Utc>,
    },
    
    /// Abonelik onayı
    Subscribed {
        task_id: Uuid,
    },
    
    /// Abonelik iptal onayı
    Unsubscribed {
        task_id: Uuid,
    },
}

/// ─── CONNECTION MANAGER ───

pub struct ConnectionManager {
    /// Aktif bağlantılar (connection_id -> sender)
    connections: Arc<RwLock<HashMap<String, tokio::sync::mpsc::Sender<ServerMessage>>>>,
    
    /// Görev abonelikleri (task_id -> [connection_id])
    subscriptions: Arc<RwLock<HashMap<Uuid, Vec<String>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Yeni bağlantı ekle
    pub async fn add_connection(
        &self,
        connection_id: String,
        sender: tokio::sync::mpsc::Sender<ServerMessage>,
    ) {
        self.connections.write().await.insert(connection_id.clone(), sender);
        log::debug!("ws  Yeni bağlantı: {} (toplam: {})", connection_id, self.connections.read().await.len());
    }
    
    /// Bağlantıyı kaldır
    pub async fn remove_connection(&self, connection_id: &str) {
        self.connections.write().await.remove(connection_id);
        
        // Aboneliklerden de temizle
        let mut subs = self.subscriptions.write().await;
        for (_, subscribers) in subs.iter_mut() {
            subscribers.retain(|id| id != connection_id);
        }
        
        log::debug!("ws  Bağlantı kapatıldı: {} (toplam: {})", connection_id, self.connections.read().await.len());
    }
    
    /// Göreve abone ol
    pub async fn subscribe(&self, connection_id: &str, task_id: Uuid) {
        self.subscriptions
            .write()
            .await
            .entry(task_id)
            .or_insert_with(Vec::new)
            .push(connection_id.to_string());
        
        log::debug!("ws  {} -> {} abone oldu", connection_id, task_id);
    }
    
    /// Aboneliği iptal et
    pub async fn unsubscribe(&self, connection_id: &str, task_id: Uuid) {
        if let Some(subscribers) = self.subscriptions.write().await.get_mut(&task_id) {
            subscribers.retain(|id| id != connection_id);
        }
    }
    
    /// Görev güncellemesi yayınla
    pub async fn broadcast_task_update(&self, task_id: Uuid, message: ServerMessage) {
        let subs = self.subscriptions.read().await;
        
        if let Some(subscribers) = subs.get(&task_id) {
            let connections = self.connections.read().await;
            
            for conn_id in subscribers {
                if let Some(sender) = connections.get(conn_id) {
                    let _ = sender.send(message.clone()).await;
                }
            }
        }
    }
    
    /// Herkese yayınla
    pub async fn broadcast_all(&self, message: ServerMessage) {
        let connections = self.connections.read().await;
        
        for sender in connections.values() {
            let _ = sender.send(message.clone()).await;
        }
    }
    
    /// Belirli bir bağlantıya gönder
    pub async fn send_to(&self, connection_id: &str, message: ServerMessage) -> bool {
        if let Some(sender) = self.connections.read().await.get(connection_id) {
            sender.send(message).await.is_ok()
        } else {
            false
        }
    }
    
    /// Aktif bağlantı sayısı
    pub async fn connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── WEBSOCKET HANDLER ───

pub async fn handle_websocket(
    socket: WebSocket,
    connection_id: String,
    manager: Arc<ConnectionManager>,
    dispatcher: Arc<crate::dispatcher::TaskDispatcher>,
) {
    use tokio::sync::mpsc;
    
    // Kanal oluştur
    let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);
    
    // Bağlantıyı kaydet
    manager.add_connection(connection_id.clone(), tx).await;
    
    // WebSocket split
    let (mut ws_sender, mut ws_receiver) = socket.split();
    
    // Giden mesajlar için task
    let conn_id_for_send = connection_id.clone();
    let manager_clone = manager.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
        manager_clone.remove_connection(&conn_id_for_send).await;
    });
    
    // Gelen mesajları işle
    let conn_id_for_recv = connection_id.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // JSON parse et
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        handle_client_message(
                            &client_msg,
                            &conn_id_for_recv,
                            &manager,
                            &dispatcher,
                        ).await;
                    } else {
                        // Hata mesajı gönder
                        manager.send_to(&conn_id_for_recv, ServerMessage::Error {
                            code: 400,
                            message: "Geçersiz mesaj formatı".into(),
                        }).await;
                    }
                }
                Ok(Message::Ping(_data)) => {
                    // Pong gönder
                    let _ = manager.send_to(&conn_id_for_recv, ServerMessage::Pong {
                        timestamp: Utc::now(),
                    }).await;
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Err(_) => break,
                _ => {}
            }
        }
    });
    
    // Task'ları bekle
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
    
    log::debug!("ws  Bağlantı sonlandı: {}", connection_id);
}

async fn handle_client_message(
    msg: &ClientMessage,
    connection_id: &str,
    manager: &Arc<ConnectionManager>,
    dispatcher: &Arc<crate::dispatcher::TaskDispatcher>,
) {
    match msg {
        ClientMessage::CreateTask { goal, model, params: _ } => {
            let request = crate::GatewayRequest::new(
                goal.clone(),
                crate::RequestSource::WebSocket {
                    connection_id: connection_id.into(),
                }
            );
            
            let request = if let Some(m) = model {
                request.with_model(m)
            } else {
                request
            };
            
            match dispatcher.dispatch(request).await {
                Ok(result) => {
                    if result.accepted {
                        // Abone ol
                        manager.subscribe(connection_id, result.task_id).await;
                        
                        manager.send_to(connection_id, ServerMessage::TaskCreated {
                            task_id: result.task_id,
                            queue_position: result.queue_position,
                        }).await;
                    } else {
                        manager.send_to(connection_id, ServerMessage::Error {
                            code: 400,
                            message: result.message,
                        }).await;
                    }
                }
                Err(e) => {
                    manager.send_to(connection_id, ServerMessage::Error {
                        code: 500,
                        message: e.to_sentient_message(),
                    }).await;
                }
            }
        }
        
        ClientMessage::GetTask { task_id } => {
            if let Some(task) = dispatcher.get_task(*task_id).await {
                manager.send_to(connection_id, ServerMessage::TaskStatusUpdate {
                    task_id: task.id,
                    status: task.status,
                    progress: task.progress,
                    timestamp: Utc::now(),
                }).await;
            } else {
                manager.send_to(connection_id, ServerMessage::Error {
                    code: 404,
                    message: "Görev bulunamadı".into(),
                }).await;
            }
        }
        
        ClientMessage::Subscribe { task_id } => {
            manager.subscribe(connection_id, *task_id).await;
            manager.send_to(connection_id, ServerMessage::Subscribed {
                task_id: *task_id,
            }).await;
        }
        
        ClientMessage::Unsubscribe { task_id } => {
            manager.unsubscribe(connection_id, *task_id).await;
            manager.send_to(connection_id, ServerMessage::Unsubscribed {
                task_id: *task_id,
            }).await;
        }
        
        ClientMessage::ListTasks => {
            let tasks = dispatcher.get_active_tasks().await;
            manager.send_to(connection_id, ServerMessage::TaskList {
                tasks,
            }).await;
        }
        
        ClientMessage::GetStats => {
            let stats = dispatcher.stats().await;
            manager.send_to(connection_id, ServerMessage::Stats {
                stats,
            }).await;
        }
        
        ClientMessage::Ping => {
            manager.send_to(connection_id, ServerMessage::Pong {
                timestamp: Utc::now(),
            }).await;
        }
    }
}

#[allow(dead_code)]
trait SENTIENTMessage {
    fn to_sentient_message(&self) -> String;
}

impl SENTIENTMessage for crate::SENTIENTError {
    fn to_sentient_message(&self) -> String {
        match self {
            Self::General(s) => format!("SENTIENT Hatası: {}", s),
            Self::ValidationError(s) => format!("Doğrulama Hatası: {}", s),
            _ => "Bilinmeyen hata".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_message_serialization() {
        let msg = ClientMessage::CreateTask {
            goal: "Test hedefi".into(),
            model: Some("qwen/qwen3-1.7b:free".into()),
            params: serde_json::json!({}),
        };
        
        let json = serde_json::to_string(&msg).expect("operation failed");
        assert!(json.contains("\"action\":\"create_task\""));
        assert!(json.contains("Test hedefi"));
    }
    
    #[test]
    fn test_server_message_serialization() {
        let msg = ServerMessage::TaskCreated {
            task_id: Uuid::new_v4(),
            queue_position: 5,
        };
        
        let json = serde_json::to_string(&msg).expect("operation failed");
        assert!(json.contains("\"event\":\"task_created\""));
    }
    
    #[tokio::test]
    async fn test_connection_manager() {
        let manager = ConnectionManager::new();
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        
        manager.add_connection("conn1".into(), tx).await;
        assert_eq!(manager.connection_count().await, 1);
        
        manager.remove_connection("conn1").await;
        assert_eq!(manager.connection_count().await, 0);
    }
    
    #[tokio::test]
    async fn test_subscription() {
        let manager = ConnectionManager::new();
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let task_id = Uuid::new_v4();
        
        manager.add_connection("conn1".into(), tx).await;
        manager.subscribe("conn1", task_id).await;
        
        // Subscription should exist
        let subs = manager.subscriptions.read().await;
        assert!(subs.contains_key(&task_id));
    }
}
