//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SYSTEM - Merkez Sistem Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::traits::*;
use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_common::events::{SENTIENTEvent, EventType};
use sentient_graph::{EventGraph, NodeDef, NodeType};
use sentient_guardrails::GuardrailEngine;
use sentient_memory::MemoryCube;
use sentient_python::PythonBridge;
use sentient_vgate::{VGateConfig, VGateEngine};

use log;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use chrono::Utc;

/// ─── SENTIENT SİSTEM YÖNETİCİSİ ───
pub struct SENTIENTSystem {
    pub id: Uuid,
    pub memory: Arc<RwLock<MemoryCube>>,
    pub vgate: Arc<Mutex<VGateEngine>>,
    pub guardrails: Arc<RwLock<GuardrailEngine>>,
    pub python_bridge: Arc<Mutex<PythonBridge>>,
    pub event_log: Arc<Mutex<Vec<SENTIENTEvent>>>,
    pub graph: Arc<EventGraph>,
    state: Arc<RwLock<ComponentState>>,
}

impl SENTIENTSystem {
    /// Tüm alt sistemleri başlat
    pub async fn init() -> SENTIENTResult<Self> {
        log::info!("══════════════════════════════════════════════");
        log::info!("  🌟  SENTIENT (NEXUS OASIS) Başlatılıyor...");
        log::info!("══════════════════════════════════════════════");

        let system_id = Uuid::new_v4();
        
        // 1) Bellek (HİPOKAMPÜS)
        let memory_path = "data/sentient_memory.db";
        std::fs::create_dir_all("data").map_err(|e| {
            SENTIENTError::Memory(format!("Veri dizini oluşturulamadı: {}", e))
        })?;

        let memory = Arc::new(RwLock::new(
            MemoryCube::new(memory_path)
                .map_err(|e| SENTIENTError::Memory(e.to_string()))?,
        ));
        log::info!("  ✅  BELLEK: Bilgi Küpü hazır.");

        // 2) Guardrails (GÜVENLİK SİSTEMİ)
        let guardrails = Arc::new(RwLock::new(GuardrailEngine::new()));
        log::info!("  ✅  GUARDRAILS: Güvenlik duvarı aktif.");

        // 3) V-GATE (VEKİL SUNUCU)
        let config = VGateConfig::default();
        let vgate = Arc::new(Mutex::new(VGateEngine::new(config)));
        log::info!("  ✅  V-GATE: Vekil sunucu katmanı hazır.");

        // 4) Python Köprüsü (ENTRASYON KATMANI)
        let python_bridge = Arc::new(Mutex::new(PythonBridge::new()));
        log::info!("  ✅  KÖPRÜ: PyO3 entegrasyon katmanı hazır.");

        // 5) Event Graph (MERKEZİ SİNİR SİSTEMİ)
        let graph = Arc::new(EventGraph::new("sentient_main"));
        
        // Ana düğümleri oluştur
        Self::setup_event_graph(&graph)?;

        // 6) Olay günlüğü
        let event_log = Arc::new(Mutex::new(Vec::new()));

        // 7) Durum yönetimi
        let state = Arc::new(RwLock::new(ComponentState::Ready));

        // Başlangıç olayını kaydet
        let start_event = SENTIENTEvent::new(
            EventType::SystemStart, 
            "sentient_system", 
            serde_json::json!({
                "version": env!("CARGO_PKG_VERSION"),
                "system_id": system_id.to_string(),
            })
        );
        event_log.lock().await.push(start_event);

        log::info!("══════════════════════════════════════════════");
        log::info!("  🚀  SENTIENT tüm modülleriyle hazır!");
        log::info!("══════════════════════════════════════════════");

        Ok(Self {
            id: system_id,
            memory,
            vgate,
            guardrails,
            python_bridge,
            event_log,
            graph,
            state,
        })
    }

    /// Event graph ana düğümlerini kur
    fn setup_event_graph(graph: &Arc<EventGraph>) -> SENTIENTResult<()> {
        let source_node = NodeDef {
            id: Uuid::new_v4(),
            name: "input_source".into(),
            node_type: NodeType::Source,
            enabled: true,
            created_at: Utc::now(),
        };
        let processor_node = NodeDef {
            id: Uuid::new_v4(),
            name: "core_processor".into(),
            node_type: NodeType::Processor,
            enabled: true,
            created_at: Utc::now(),
        };
        let sink_node = NodeDef {
            id: Uuid::new_v4(),
            name: "output_sink".into(),
            node_type: NodeType::Sink,
            enabled: true,
            created_at: Utc::now(),
        };
        
        let source_id = graph.add_node(source_node)?;
        let processor_id = graph.add_node(processor_node)?;
        let sink_id = graph.add_node(sink_node)?;
        
        graph.add_edge(source_id, processor_id, None)?;
        graph.add_edge(processor_id, sink_id, None)?;
        
        log::info!(
            "  ✅  GRAPH: Event graph düğümleri oluşturuldu ({} düğüm, {} bağlantı).",
            graph.node_count(), 
            graph.edge_count()
        );

        Ok(())
    }

    /// Sistem kimliği
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// LLM sorgusu gönder (tam korumalı hat)
    pub async fn query_llm(
        &self,
        model: &str,
        user_message: &str,
        system_prompt: Option<&str>,
    ) -> SENTIENTResult<String> {
        // 1. Giriş güvenlik kontrolü
        {
            let guardrails = self.guardrails.read().await;
            let verdict = guardrails.check_input(user_message);
            if !verdict.is_clean() {
                log::warn!("GUARDRAILS: Giriş engellendi");
                return Err(SENTIENTError::Guardrails(
                    "Güvenlik politikası tarafından engellendi".into()
                ));
            }
        }

        // 2. Belleğe kaydet
        {
            let mut mem = self.memory.write().await;
            let _ = mem.create(
                format!("Soru: {}", user_message),
                sentient_memory::MemoryType::Working,
                Some(serde_json::json!({ "model": model })),
                Some(3600),
            );
        }

        // 3. V-GATE üzerinden gönder
        let content = {
            let vgate = self.vgate.lock().await;
            let request = sentient_vgate::LlmRequest {
                model: model.into(),
                messages: self.build_messages(system_prompt, user_message),
                max_tokens: Some(4096),
                temperature: Some(0.7),
                stream: Some(false),
            };
            let event = vgate.send_request(request).await?;
            
            event.payload
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        };

        // 4. Çıkış güvenlik kontrolü
        {
            let guardrails = self.guardrails.read().await;
            let verdict = guardrails.check_output(&content);
            if !verdict.is_clean() {
                log::warn!("GUARDRAILS: Çıkış engellendi");
                return Err(SENTIENTError::Guardrails(
                    "Yanıt güvenlik politikasını ihlal ediyor".into()
                ));
            }
        }

        // 5. Yanıtı belleğe kaydet
        {
            let mut mem = self.memory.write().await;
            let _ = mem.create(
                format!("Yanıt: {}", &content[..content.len().min(200)]),
                sentient_memory::MemoryType::Working,
                Some(serde_json::json!({ "model": model })),
                Some(3600),
            );
        }

        Ok(content)
    }

    /// Mesaj listesi oluştur
    fn build_messages(&self, system_prompt: Option<&str>, user_message: &str) -> Vec<sentient_vgate::Message> {
        let mut messages = Vec::new();
        
        if let Some(prompt) = system_prompt {
            messages.push(sentient_vgate::Message {
                role: "system".into(),
                content: prompt.into(),
            });
        }
        
        messages.push(sentient_vgate::Message {
            role: "user".into(),
            content: user_message.into(),
        });
        
        messages
    }

    /// Sistem durum raporu
    pub async fn status(&self) -> SystemStatus {
        let mem_count: usize = match self.memory.read().await.count() {
            Ok(c) => c as usize,
            Err(_) => 0,
        };
        let vgate_requests = self.vgate.lock().await.request_count().await;
        let event_count = self.event_log.lock().await.len();
        let policy_count = self.guardrails.read().await.list_policies().len();
        let current_state = *self.state.read().await;

        SystemStatus {
            system_id: self.id,
            state: current_state,
            memory_entries: mem_count,
            vgate_requests,
            events_logged: event_count,
            tools_registered: 0,
            security_policies: policy_count,
            graph_nodes: self.graph.node_count(),
            graph_edges: self.graph.edge_count(),
            active_agents: 0,
            uptime_seconds: 0,
        }
    }

    /// Durum raporu (metin formatında)
    pub async fn status_report(&self) -> String {
        let status = self.status().await;
        
        format!(
            "╔══════════════════════════════════════════════╗\n\
             ║  🌟  SENTIENT Durum Raporu                      ║\n\
             ╠══════════════════════════════════════════════╣\n\
             ║  🆔  Sistem ID:       {:>20}    ║\n\
             ║  📊  Durum:           {:>20?}    ║\n\
             ╠══════════════════════════════════════════════╣\n\
             ║  🧠  Bellek kayıtları: {:>7}                ║\n\
             ║  🚪  V-GATE istekleri: {:>7}                ║\n\
             ║  📝  Olay logu:        {:>7}                ║\n\
             ║  🔧  Kayıtlı araçlar:  {:>7}                ║\n\
             ║  🛡️  Güvenlik politikası: {:>5}             ║\n\
             ║  🤖  Aktif ajanlar:    {:>7}                ║\n\
             ╠══════════════════════════════════════════════╣\n\
             ║  🔗  Graph istatistikleri:                   ║\n\
             ║      Düğümler:        {:>7}                ║\n\
             ║      Bağlantılar:     {:>7}                ║\n\
             ╚══════════════════════════════════════════════╝",
            status.system_id.to_string().split('-').next().unwrap_or("?"),
            status.state,
            status.memory_entries,
            status.vgate_requests,
            status.events_logged,
            status.tools_registered,
            status.security_policies,
            status.active_agents,
            status.graph_nodes,
            status.graph_edges,
        )
    }

    /// Güvenli kapatma
    pub async fn shutdown(&self) -> SENTIENTResult<()> {
        log::info!("🛑  SENTIENT: Güvenli kapatma başlatılıyor...");
        
        // Durumu güncelle
        {
            let mut state = self.state.write().await;
            *state = ComponentState::ShuttingDown;
        }

        // Bellek temizliği
        {
            let mut mem = self.memory.write().await;
            match mem.cleanup_expired() {
                Ok(cleaned) => {
                    if cleaned > 0 {
                        log::info!("  🧹  BELLEK: {} süresi dolmuş kayıt temizlendi.", cleaned);
                    }
                }
                Err(e) => log::warn!("  🧹  BELLEK temizlik hatası: {}", e),
            }
        }

        // Kapatma olayı
        let shutdown_event = SENTIENTEvent::new(
            EventType::SystemShutdown, 
            "sentient_system", 
            serde_json::json!({
                "graceful": true,
                "timestamp": Utc::now().to_rfc3339(),
            })
        );
        self.event_log.lock().await.push(shutdown_event);

        // Durumu güncelle
        {
            let mut state = self.state.write().await;
            *state = ComponentState::Terminated;
        }

        log::info!("✅  SENTIENT: Güvenli kapatma tamamlandı.");
        Ok(())
    }
}

/// Sistem durumu yapısı
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemStatus {
    pub system_id: Uuid,
    pub state: ComponentState,
    pub memory_entries: usize,
    pub vgate_requests: u64,
    pub events_logged: usize,
    pub tools_registered: usize,
    pub security_policies: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub active_agents: usize,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_system_init() {
        // Bu test sadece derleme testi için
        // Gerçek init() için veritabanı gerekir
        assert!(true);
    }
}
