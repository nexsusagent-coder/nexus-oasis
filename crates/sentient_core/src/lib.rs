//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT CORE - Nexus Oasis OS Merkez Çekirdek Kütüphanesi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Ana sistem kütüphanesi — tüm modülleri orkestre eder.
//! CLI binary'den (main.rs) ve diğer kütüphanelerden kullanılabilir.
//!
//! Modüller:
//! - `traits`: Ortak arayüz tanımları
//! - `system`: SENTIENT sistem yönetimi
//! - `llm_test`: LLM test yardımcıları

pub mod traits;
pub mod llm_test;
pub mod system;

// Trait re-exports
pub use traits::*;

pub use sentient_common;
pub use sentient_guardrails;
pub use sentient_graph;
pub use sentient_memory;
pub use sentient_python;
pub use sentient_vgate;

// Re-export LLM types for convenience
// V-GATE Provider re-export
pub use sentient_vgate::auth::Provider;

use sentient_common::error::SENTIENTResult;
use sentient_common::events::{SENTIENTEvent, EventType};
use sentient_graph::{EventGraph, NodeDef, NodeType};
use sentient_guardrails::GuardrailEngine;
use sentient_memory::MemoryCube;
use sentient_python::PythonBridge;
use sentient_vgate::{LlmRequest, Message, VGateConfig, VGateEngine};
use log;
use std::sync::Arc;
use tokio::sync::Mutex;

/// ─── SENTIENT Sistem Durumu ───
pub struct SENTIENTSystem {
    pub memory: Arc<Mutex<MemoryCube>>,
    pub vgate: Arc<Mutex<VGateEngine>>,
    pub guardrails: Arc<Mutex<GuardrailEngine>>,
    pub python_bridge: Arc<Mutex<PythonBridge>>,
    pub event_log: Arc<Mutex<Vec<SENTIENTEvent>>>,
    pub graph: Arc<EventGraph>,
}

impl SENTIENTSystem {
    /// Tüm alt sistemleri başlat
    pub async fn init() -> SENTIENTResult<Self> {
        log::info!("══════════════════════════════════════════════");
        log::info!("  🌟  SENTIENT (NEXUS OASIS) Başlatılıyor...");
        log::info!("══════════════════════════════════════════════");

        // 1) Bellek (HİPOKAMPÜS)
        let memory_path = "data/sentient_memory.db";
        std::fs::create_dir_all("data").map_err(|e| {
            sentient_common::error::SENTIENTError::Memory(format!(
                "Veri dizini oluşturulamadı: {}",
                e
            ))
        })?;

        let memory = Arc::new(Mutex::new(
            MemoryCube::new(memory_path)
                .map_err(|e| sentient_common::error::SENTIENTError::Memory(e.to_string()))?,
        ));
        log::info!("✅  BELLEK: Bilgi Küpü hazır.");

        // 2) Guardrails (BAĞIŞIKLIK SİSTEMİ)
        let guardrails = Arc::new(Mutex::new(GuardrailEngine::new()));
        log::info!("✅  GUARDRAILS: Güvenlik duvarı aktif.");

        // 3) V-GATE (VEKİL SUNUCU)
        let config = VGateConfig::default();
        let vgate = Arc::new(Mutex::new(VGateEngine::new(config)));
        log::info!("✅  V-GATE: Vekil sunucu katmanı hazır.");

        // 4) Python Köprüsü (ASİMİLASYON)
        let python_bridge = Arc::new(Mutex::new(PythonBridge::new()));
        log::info!("✅  KÖPRÜ: PyO3 asimilasyon katmanı hazır.");

        // 5) Olay günlüğü
        let event_log = Arc::new(Mutex::new(Vec::new()));

        // 6) Event Graph (MERKEZI SİNİR SİSTEMİ)
        let graph = Arc::new(EventGraph::new("sentient_main"));
        
        // Ana düğümleri oluştur
        let source_node = NodeDef {
            id: uuid::Uuid::new_v4(),
            name: "input_source".into(),
            node_type: NodeType::Source,
            enabled: true,
            created_at: chrono::Utc::now(),
        };
        let processor_node = NodeDef {
            id: uuid::Uuid::new_v4(),
            name: "core_processor".into(),
            node_type: NodeType::Processor,
            enabled: true,
            created_at: chrono::Utc::now(),
        };
        let sink_node = NodeDef {
            id: uuid::Uuid::new_v4(),
            name: "output_sink".into(),
            node_type: NodeType::Sink,
            enabled: true,
            created_at: chrono::Utc::now(),
        };
        
        let source_id = graph.add_node(source_node).unwrap();
        let processor_id = graph.add_node(processor_node).unwrap();
        let sink_id = graph.add_node(sink_node).unwrap();
        
        // Bağlantıları oluştur
        graph.add_edge(source_id, processor_id, None).unwrap();
        graph.add_edge(processor_id, sink_id, None).unwrap();
        
        log::info!("✅  GRAPH: Event graph düğümleri oluşturuldu ({} düğüm, {} bağlantı).", 
            graph.node_count(), graph.edge_count());

        log::info!("══════════════════════════════════════════════");
        log::info!("  🚀  SENTIENT tüm modülleriyle hazır!");
        log::info!("══════════════════════════════════════════════");

        // Başlangıç olayını kaydet
        let start_event = SENTIENTEvent::new(EventType::SystemStart, "sentient_core", serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
        }));

        {
            let mut log = event_log.lock().await;
            log.push(start_event);
        }

        Ok(Self {
            memory,
            vgate,
            guardrails,
            python_bridge,
            event_log,
            graph,
        })
    }

    /// LLM sorgusu gönder (tam korumalı hat)
    pub async fn query_llm(
        &self,
        model: &str,
        user_message: &str,
        system_prompt: Option<&str>,
    ) -> SENTIENTResult<String> {
        let mut messages = Vec::new();

        if let Some(prompt) = system_prompt {
            messages.push(Message {
                role: "system".into(),
                content: prompt.into(),
            });
        }

        messages.push(Message {
            role: "user".into(),
            content: user_message.into(),
        });

        let request = LlmRequest {
            model: model.into(),
            messages,
            max_tokens: Some(4096),
            temperature: Some(0.7),
            stream: Some(false),
        };

        // Belleğe kaydet
        {
            let mut mem = self.memory.lock().await;
            let _ = mem.create(
                format!("Soru: {}", user_message),
                sentient_memory::MemoryType::Working,
                Some(serde_json::json!({ "model": model })),
                Some(3600), // 1 saat TTL
            );
        }

        // V-GATE üzerinden gönder
        let vgate = self.vgate.lock().await;
        let event = vgate.send_request(request).await?;

        // Yanıtı çıkar
        let content = event
            .payload
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Belleğe yanıtı kaydet
        {
            let mut mem = self.memory.lock().await;
            let _ = mem.create(
                format!("Yanıt: {}", content),
                sentient_memory::MemoryType::Working,
                Some(serde_json::json!({ "model": model, "event_id": event.id.to_string() })),
                Some(3600),
            );
        }

        // Olayı kaydet
        {
            let mut log = self.event_log.lock().await;
            log.push(event);
        }

        Ok(content)
    }

    /// Sistem durum raporu
    pub async fn status(&self) -> String {
        let mem_count = match self.memory.lock().await.count() {
            Ok(c) => c,
            Err(_) => 0,
        };
        let vgate_requests = self.vgate.lock().await.request_count().await;
        let event_count = self.event_log.lock().await.len();
        let tool_count = self.python_bridge.lock().await.list_tools().len();
        let policy_count = self.guardrails.lock().await.list_policies().len();
        let graph_stats = self.graph.stats();

        format!
            ("╔══════════════════════════════════════════════╗\n\
             ║  🌟  SENTIENT Durum Raporu                      ║\n\
             ╠══════════════════════════════════════════════╣\n\
             ║  🧠  Bellek kayıtları:     {:>7}            ║\n\
             ║  🚪  V-GATE istekleri:    {:>7}            ║\n\
             ║  📝  Olay logu:           {:>7}            ║\n\
             ║  🐍  Python araçları:     {:>7}            ║\n\
             ║  🛡  Güvenlik politikası: {:>7}            ║\n\
             ║  🔗  Graph olayları:      {:>7}            ║\n\
             ╚══════════════════════════════════════════════╝",
            mem_count, vgate_requests, event_count, tool_count, policy_count, graph_stats.total_events
        )
    }

    /// Güvenli kapatma
    pub async fn shutdown(&self) -> SENTIENTResult<()> {
        log::info!("🛑  SENTIENT: Güvenli kapatma başlatılıyor...");

        // Bellek temizliği
        {
            let mut mem = self.memory.lock().await;
            match mem.cleanup_expired() {
                Ok(cleaned) => {
                    if cleaned > 0 {
                        log::info!(
                            "🧹  BELLEK: {} süresi dolmuş kayıt temizlendi.",
                            cleaned
                        );
                    }
                }
                Err(e) => log::warn!("🧹  BELLEK temizlik hatası: {}", e),
            }
        }

        // Kapatma olayı
        let shutdown_event =
            SENTIENTEvent::new(EventType::SystemShutdown, "sentient_core", serde_json::json!({}));
        {
            let mut log = self.event_log.lock().await;
            log.push(shutdown_event);
        }

        log::info!("✅  SENTIENT: Güvenli kapatma tamamlandı.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
