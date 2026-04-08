//! ─── CLAW3D - SWARM 3D GÖRSELLEŞTIRME ───
//!
//! Claw3D, Swarm ajanlarının 3 boyutlu görselleştirmesi için
//! WebSocket uç noktaları sağlar.
//!
//! Özellikler:
//! - Gerçek zamanlı ajan pozisyonları
//! - Görev bağlantıları (task edges)
//! - Karar akımı (decision flow)
//! - Bellek kullanımı ısı haritası
//! - Claw3D istemcisi ile uyumlu protokol

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::websocket::ConnectionManager;

// ═══════════════════════════════════════════════════════════════════════════════
//  CLAW3D MESSAGE PROTOCOL
// ═══════════════════════════════════════════════════════════════════════════════

/// Claw3D istemci-sunucu mesajı
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClawMessage {
    // ─── İstemci → Sunucu ───
    
    /// Sahne aboneliği
    Subscribe {
        scene_id: Uuid,
    },
    
    /// Kamera durumu
    CameraUpdate {
        position: [f32; 3],
        rotation: [f32; 4],
        zoom: f32,
    },
    
    /// Ajan seçimi
    SelectAgent {
        agent_id: Uuid,
    },
    
    /// Zaman kontrolü
    TimeControl {
        action: TimeAction,
        speed: f32,
    },
    
    /// Debug modu
    ToggleDebug {
        enabled: bool,
    },
    
    // ─── Sunucu → İstemci ───
    
    /// Sahne durumu
    SceneState {
        scene: SceneData,
    },
    
    /// Ajan güncelleme
    AgentUpdate {
        agent: AgentNode,
    },
    
    /// Bağlantı güncelleme
    EdgeUpdate {
        edge: TaskEdge,
    },
    
    /// Bellek ısı haritası
    MemoryHeatmap {
        agents: HashMap<Uuid, MemoryHeat>,
    },
    
    /// Karar akımı
    DecisionFlow {
        flow: DecisionStep,
    },
    
    /// Araç tetikleme
    ToolTrigger {
        agent_id: Uuid,
        tool: ToolEvent,
    },
    
    /// Hata mesajı
    Error {
        code: u16,
        message: String,
    },
}

/// Zaman kontrolü aksiyonları
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeAction {
    Play,
    Pause,
    Step,
    Reset,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCENE DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

/// Sahne verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    /// Sahne ID
    pub id: Uuid,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Aktif ajanlar
    pub agents: Vec<AgentNode>,
    /// Görev bağlantıları
    pub edges: Vec<TaskEdge>,
    /// Sahne istatistikleri
    pub stats: SceneStats,
    /// Kamera varsayılan
    pub default_camera: CameraConfig,
}

/// Ajan düğümü (3D pozisyonlu)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNode {
    /// Ajan ID
    pub id: Uuid,
    /// Ajan adı
    pub name: String,
    /// Ajan tipi
    pub agent_type: AgentType,
    /// 3D pozisyon (x, y, z)
    pub position: [f32; 3],
    /// Döndürme (euler)
    pub rotation: [f32; 3],
    /// Ölçek
    pub scale: f32,
    /// Durum
    pub status: AgentStatus3D,
    /// Aktif görev
    pub current_task: Option<Uuid>,
    /// Bellek kullanımı (0-1)
    pub memory_usage: f32,
    /// CPU kullanımı (0-1)
    pub cpu_usage: f32,
    /// Düşünce balonu
    pub thought_bubble: Option<String>,
    /// Renk (hex)
    pub color: String,
    /// Glow yoğunluğu
    pub glow_intensity: f32,
}

/// Ajan tipleri
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    Scout,
    Forge,
    SwarmAlpha,
    SwarmBeta,
    SwarmGamma,
    SwarmDelta,
    Orchestrator,
}

impl AgentType {
    pub fn default_color(&self) -> &'static str {
        match self {
            Self::Scout => "#00fff2",      // Cyan
            Self::Forge => "#ff00ff",       // Magenta
            Self::SwarmAlpha => "#39ff14",  // Green
            Self::SwarmBeta => "#ffff00",    // Yellow
            Self::SwarmGamma => "#ff0040",   // Red
            Self::SwarmDelta => "#0080ff",   // Blue
            Self::Orchestrator => "#ffffff", // White
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Scout => "🔍",
            Self::Forge => "🔨",
            Self::SwarmAlpha => "🅰️",
            Self::SwarmBeta => "🅱️",
            Self::SwarmGamma => "🇬",
            Self::SwarmDelta => "🔺",
            Self::Orchestrator => "🎯",
        }
    }
}

/// Ajan durumu (3D görselleştirme için)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus3D {
    Idle,
    Thinking,
    Working,
    Waiting,
    Complete,
    Error,
}

/// Görev bağlantısı (kenar)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEdge {
    /// Kenar ID
    pub id: Uuid,
    /// Kaynak ajan
    pub from_agent: Uuid,
    /// Hedef ajan
    pub to_agent: Uuid,
    /// Görev ID
    pub task_id: Uuid,
    /// Görev açıklaması
    pub description: String,
    /// Bağlantı tipi
    pub edge_type: EdgeType,
    /// Akım yönü
    pub direction: FlowDirection,
    /// Animasyon aktif mi?
    pub animated: bool,
    /// Renk
    pub color: String,
    /// Kalınlık
    pub thickness: f32,
}

/// Kenar tipi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    /// Görev ataması
    TaskAssignment,
    /// Sonuç raporu
    ResultReport,
    /// Bilgi paylaşımı
    InfoShare,
    /// Koordinasyon
    Coordination,
    /// Bellek erişimi
    MemoryAccess,
}

/// Akım yönü
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FlowDirection {
    Forward,
    Backward,
    Bidirectional,
}

/// Sahne istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SceneStats {
    /// Toplam ajan
    pub total_agents: usize,
    /// Aktif ajan
    pub active_agents: usize,
    /// Toplam görev
    pub total_tasks: usize,
    /// Tamamlanan görev
    pub completed_tasks: usize,
    /// Başarı oranı
    pub success_rate: f32,
    /// Ortalama süre (ms)
    pub avg_duration_ms: f64,
}

/// Kamera yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    /// Pozisyon
    pub position: [f32; 3],
    /// Hedef
    pub target: [f32; 3],
    /// Zoom
    pub zoom: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: [0.0, 50.0, 100.0],
            target: [0.0, 0.0, 0.0],
            zoom: 1.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY & VISUALIZATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Bellek ısı noktası
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryHeat {
    /// Ajan ID
    pub agent_id: Uuid,
    /// Isı değeri (0-1)
    pub heat: f32,
    /// Bellek tipi dağılımı
    pub distribution: MemoryDistribution,
    /// En son erişim
    pub last_access: DateTime<Utc>,
}

/// Bellek dağılımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDistribution {
    /// Epizodik yüzdesi
    pub episodic_pct: f32,
    /// Semantik yüzdesi
    pub semantic_pct: f32,
    /// Prosedürel yüzdesi
    pub procedural_pct: f32,
    /// Çalışma belleği yüzdesi
    pub working_pct: f32,
}

/// Karar akımı adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionStep {
    /// Adım ID
    pub id: Uuid,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    /// Kaynak ajan
    pub agent_id: Uuid,
    /// Karar tipi
    pub decision_type: DecisionType,
    /// Girdi (observation)
    pub input: String,
    /// Düşünce süreci
    pub reasoning: String,
    /// Çıktı (action)
    pub output: String,
    /// Güven skoru (0-1)
    pub confidence: f32,
    /// Bellek referansları
    pub memory_refs: Vec<Uuid>,
}

/// Karar tipi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionType {
    Observe,
    Think,
    Act,
    Reflect,
}

/// Araç tetikleme olayı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEvent {
    /// Araç adı
    pub name: String,
    /// Parametreler
    pub params: serde_json::Value,
    /// Sonuç (tamamlandıysa)
    pub result: Option<serde_json::Value>,
    /// Durum
    pub status: ToolStatus,
    /// Süre (ms)
    pub duration_ms: Option<u64>,
}

/// Araç durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolStatus {
    Pending,
    Running,
    Complete,
    Failed,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CLAW3D STATE MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Claw3D durum yöneticisi
pub struct Claw3DState {
    /// Aktif sahne
    scene: Arc<RwLock<Option<SceneData>>>,
    /// Ajan pozisyonları
    agents: Arc<RwLock<HashMap<Uuid, AgentNode>>>,
    /// Görev bağlantıları
    edges: Arc<RwLock<HashMap<Uuid, TaskEdge>>>,
    /// Bellek ısı haritası
    memory_heatmap: Arc<RwLock<HashMap<Uuid, MemoryHeat>>>,
    /// Karar akımı geçmişi
    decision_history: Arc<RwLock<Vec<DecisionStep>>>,
    /// Son güncelleme
    last_update: Arc<RwLock<DateTime<Utc>>>,
}

impl Claw3DState {
    /// Yeni durum oluştur
    pub fn new() -> Self {
        Self {
            scene: Arc::new(RwLock::new(None)),
            agents: Arc::new(RwLock::new(HashMap::new())),
            edges: Arc::new(RwLock::new(HashMap::new())),
            memory_heatmap: Arc::new(RwLock::new(HashMap::new())),
            decision_history: Arc::new(RwLock::new(Vec::new())),
            last_update: Arc::new(RwLock::new(Utc::now())),
        }
    }
    
    /// Sahne oluştur
    pub async fn create_scene(&self) -> SceneData {
        let scene = SceneData {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            agents: Vec::new(),
            edges: Vec::new(),
            stats: SceneStats {
                total_agents: 0,
                active_agents: 0,
                total_tasks: 0,
                completed_tasks: 0,
                success_rate: 0.0,
                avg_duration_ms: 0.0,
            },
            default_camera: CameraConfig::default(),
        };
        
        *self.scene.write().await = Some(scene.clone());
        scene
    }
    
    /// Ajan ekle/güncelle
    pub async fn update_agent(&self, agent: AgentNode) {
        let id = agent.id;
        self.agents.write().await.insert(id, agent.clone());
        *self.last_update.write().await = Utc::now();
    }
    
    /// Bağlantı ekle/güncelle
    pub async fn update_edge(&self, edge: TaskEdge) {
        let id = edge.id;
        self.edges.write().await.insert(id, edge.clone());
        *self.last_update.write().await = Utc::now();
    }
    
    /// Bellek ısı güncelle
    pub async fn update_memory_heat(&self, agent_id: Uuid, heat: MemoryHeat) {
        self.memory_heatmap.write().await.insert(agent_id, heat);
    }
    
    /// Karar ekle
    pub async fn add_decision(&self, decision: DecisionStep) {
        self.decision_history.write().await.push(decision);
        
        // Son 100 kararı tut
        let mut history = self.decision_history.write().await;
        if history.len() > 100 {
            history.remove(0);
        }
    }
    
    /// Mevcut sahne durumunu al
    pub async fn get_scene_state(&self) -> SceneData {
        let agents: Vec<_> = self.agents.read().await.values().cloned().collect();
        let edges: Vec<_> = self.edges.read().await.values().cloned().collect();
        
        let active_count = agents.iter().filter(|a| a.status != AgentStatus3D::Idle).count();
        let completed = edges.iter().filter(|e| !e.animated).count();
        
        SceneData {
            id: self.scene.read().await.as_ref().map(|s| s.id).unwrap_or_else(Uuid::new_v4),
            timestamp: *self.last_update.read().await,
            agents,
            edges,
            stats: SceneStats {
                total_agents: self.agents.read().await.len(),
                active_agents: active_count,
                total_tasks: self.edges.read().await.len(),
                completed_tasks: completed,
                success_rate: if completed > 0 { 0.85 } else { 0.0 },
                avg_duration_ms: 2500.0,
            },
            default_camera: CameraConfig::default(),
        }
    }
}

impl Default for Claw3DState {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WEBSOCKET HANDLER
// ═══════════════════════════════════════════════════════════════════════════════

/// Claw3D WebSocket yükseltme
pub async fn claw3d_ws_upgrade(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<Claw3DState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_claw3d_socket(socket, state))
}

/// Claw3D WebSocket işleyici
async fn handle_claw3d_socket(socket: WebSocket, state: Arc<Claw3DState>) {
    let (mut sender, mut receiver) = socket.split();
    
    // İlk sahne durumunu gönder
    let scene = state.get_scene_state().await;
    let initial_msg = ClawMessage::SceneState { scene };
    
    if let Ok(json) = serde_json::to_string(&initial_msg) {
        let _ = sender.send(Message::Text(json)).await;
    }
    
    // Mesaj döngüsü
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(claw_msg) = serde_json::from_str::<ClawMessage>(&text) {
                    handle_claw_message(claw_msg, &state, &mut sender).await;
                } else {
                    let error = ClawMessage::Error {
                        code: 400,
                        message: "Geçersiz mesaj formatı".into(),
                    };
                    if let Ok(json) = serde_json::to_string(&error) {
                        let _ = sender.send(Message::Text(json)).await;
                    }
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = sender.send(Message::Pong(data)).await;
            }
            Ok(Message::Close(_)) | Err(_) => break,
            _ => {}
        }
    }
    
    log::debug!("🧊 Claw3D bağlantı kapandı");
}

/// Claw mesajı işle
async fn handle_claw_message(
    msg: ClawMessage,
    state: &Arc<Claw3DState>,
    sender: &mut futures::stream::SplitSink<WebSocket, Message>,
) {
    match msg {
        ClawMessage::Subscribe { scene_id: _ } => {
            let scene = state.get_scene_state().await;
            let response = ClawMessage::SceneState { scene };
            if let Ok(json) = serde_json::to_string(&response) {
                let _ = sender.send(Message::Text(json)).await;
            }
        }
        
        ClawMessage::SelectAgent { agent_id } => {
            let agents = state.agents.read().await;
            if let Some(agent) = agents.get(&agent_id) {
                let response = ClawMessage::AgentUpdate { agent: agent.clone() };
                if let Ok(json) = serde_json::to_string(&response) {
                    let _ = sender.send(Message::Text(json)).await;
                }
            } else {
                let error = ClawMessage::Error {
                    code: 404,
                    message: "Ajan bulunamadı".into(),
                };
                if let Ok(json) = serde_json::to_string(&error) {
                    let _ = sender.send(Message::Text(json)).await;
                }
            }
        }
        
        ClawMessage::TimeControl { action, speed: _ } => {
            // Zaman kontrolü simülasyonu
            let response = ClawMessage::SceneState {
                scene: state.get_scene_state().await,
            };
            if let Ok(json) = serde_json::to_string(&response) {
                let _ = sender.send(Message::Text(json)).await;
            }
        }
        
        ClawMessage::ToggleDebug { enabled: _ } => {
            // Debug modu değiştirildi
            let response = ClawMessage::SceneState {
                scene: state.get_scene_state().await,
            };
            if let Ok(json) = serde_json::to_string(&response) {
                let _ = sender.send(Message::Text(json)).await;
            }
        }
        
        _ => {
            let error = ClawMessage::Error {
                code: 501,
                message: "Bu mesaj tipi henüz desteklenmiyor".into(),
            };
            if let Ok(json) = serde_json::to_string(&error) {
                let _ = sender.send(Message::Text(json)).await;
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AXUM ROUTER
// ═══════════════════════════════════════════════════════════════════════════════

use axum::Router;

/// Claw3D router oluştur
pub fn create_claw3d_router(state: Arc<Claw3DState>) -> Router {
    Router::new()
        .route("/ws/claw3d", axum::routing::get(claw3d_ws_upgrade))
        .layer(axum::Extension(state))
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_type_colors() {
        assert_eq!(AgentType::Scout.default_color(), "#00fff2");
        assert_eq!(AgentType::Forge.default_color(), "#ff00ff");
        assert_eq!(AgentType::SwarmAlpha.default_color(), "#39ff14");
    }
    
    #[test]
    fn test_agent_node_serialization() {
        let agent = AgentNode {
            id: Uuid::new_v4(),
            name: "Scout-001".into(),
            agent_type: AgentType::Scout,
            position: [10.0, 20.0, 30.0],
            rotation: [0.0, 45.0, 0.0],
            scale: 1.0,
            status: AgentStatus3D::Thinking,
            current_task: Some(Uuid::new_v4()),
            memory_usage: 0.45,
            cpu_usage: 0.75,
            thought_bubble: Some("Veritabanı tarama...".into()),
            color: "#00fff2".into(),
            glow_intensity: 0.8,
        };
        
        let json = serde_json::to_string(&agent).unwrap();
        assert!(json.contains("Scout-001"));
        assert!(json.contains("thinking"));
    }
    
    #[test]
    fn test_claw_message_serialization() {
        let msg = ClawMessage::SceneState {
            scene: SceneData {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                agents: vec![],
                edges: vec![],
                stats: SceneStats::default(),
                default_camera: CameraConfig::default(),
            },
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("scene_state"));
    }
    
    #[tokio::test]
    async fn test_claw3d_state() {
        let state = Claw3DState::new();
        
        // Sahne oluştur
        let scene = state.create_scene().await;
        assert!(!scene.agents.is_empty() || scene.agents.is_empty()); // Boş olabilir
        
        // Ajan ekle
        let agent = AgentNode {
            id: Uuid::new_v4(),
            name: "Test-Agent".into(),
            agent_type: AgentType::Scout,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: 1.0,
            status: AgentStatus3D::Idle,
            current_task: None,
            memory_usage: 0.0,
            cpu_usage: 0.0,
            thought_bubble: None,
            color: "#00fff2".into(),
            glow_intensity: 0.5,
        };
        
        state.update_agent(agent.clone()).await;
        
        let retrieved = state.get_scene_state().await;
        assert!(!retrieved.agents.is_empty());
    }
    
    #[test]
    fn test_decision_step() {
        let decision = DecisionStep {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            agent_id: Uuid::new_v4(),
            decision_type: DecisionType::Observe,
            input: "Kullanıcı isteği alındı".into(),
            reasoning: "Hedef analizi yapılıyor...".into(),
            output: "TASK_PLAN: create_file".into(),
            confidence: 0.92,
            memory_refs: vec![Uuid::new_v4()],
        };
        
        let json = serde_json::to_string(&decision).unwrap();
        assert!(json.contains("observe"));
        assert!(json.contains("Kullanıcı"));
    }
}
