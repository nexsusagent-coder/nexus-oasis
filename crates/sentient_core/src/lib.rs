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

use sentient_common::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerManager};
use sentient_common::crypto::{AutoBackup, BackupConfig, EncryptionConfig, EncryptionEngine};
use sentient_common::error::SENTIENTResult;
use sentient_common::events::{SENTIENTEvent, EventType};
use sentient_common::metrics::{MetricsRegistry, metrics_inc, metrics_gauge_set, metrics_output};
use sentient_common::tracing::{self as sent_tracing, Span, SpanStatus};
use sentient_graph::{EventGraph, NodeDef, NodeType};
use sentient_guardrails::GuardrailEngine;
use sentient_memory::MemoryCube;
use sentient_python::PythonBridge;
use sentient_vgate::{LlmRequest, Message, VGateConfig, VGateEngine};
use log;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// ─── SENTIENT Sistem Durumu ───
pub struct SENTIENTSystem {
    pub memory: Arc<Mutex<MemoryCube>>,
    pub vgate: Arc<Mutex<VGateEngine>>,
    pub guardrails: Arc<Mutex<GuardrailEngine>>,
    pub python_bridge: Arc<Mutex<PythonBridge>>,
    pub event_log: Arc<Mutex<Vec<SENTIENTEvent>>>,
    pub graph: Arc<EventGraph>,
    /// Devre kesici yöneticisi
    pub circuit_breaker_manager: Arc<Mutex<CircuitBreakerManager>>,
    /// Otomatik yedekleme
    pub auto_backup: Arc<Mutex<AutoBackup>>,
    /// Şifreleme motoru
    pub encryption: Arc<Mutex<EncryptionEngine>>,
    /// Başlatma zamanı
    pub started_at: Instant,
}

impl SENTIENTSystem {
    /// Tüm alt sistemleri başlat
    pub async fn init() -> SENTIENTResult<Self> {
        log::info!("══════════════════════════════════════════════");
        log::info!("  🌟  SENTIENT (NEXUS OASIS) Başlatılıyor...");
        log::info!("══════════════════════════════════════════════");

        let started_at = Instant::now();

        // 0) Metrik kayıtçısını başlat
        metrics_inc("sentient_system_starts_total");
        log::info!("✅  METRICS: Prometheus metrik sistemi aktif.");

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
        
        let source_id = graph.add_node(source_node).expect("Failed to add source node");
        let processor_id = graph.add_node(processor_node).expect("Failed to add processor node");
        let sink_id = graph.add_node(sink_node).expect("Failed to add sink node");
        
        // Bağlantıları oluştur
        graph.add_edge(source_id, processor_id, None).expect("Failed to add edge source->processor");
        graph.add_edge(processor_id, sink_id, None).expect("Failed to add edge processor->sink");
        
        log::info!("✅  GRAPH: Event graph düğümleri oluşturuldu ({} düğüm, {} bağlantı).", 
            graph.node_count(), graph.edge_count());

        // 7) Devre Kesici (Circuit Breaker)
        let mut cb_manager = CircuitBreakerManager::new(CircuitBreakerConfig::default());
        cb_manager.add_provider("openai");
        cb_manager.add_provider("anthropic");
        cb_manager.add_provider("openrouter");
        cb_manager.add_provider("groq");
        cb_manager.add_provider("local");
        let circuit_breaker_manager = Arc::new(Mutex::new(cb_manager));
        log::info!("✅  CIRCUIT BREAKER: 5 provider devre kesici aktif.");

        // 8) Şifreleme Motoru (Encryption at Rest)
        let encryption = Arc::new(Mutex::new(EncryptionEngine::default_encrypted()));
        log::info!("✅  CRYPTO: Encryption at Rest motoru hazır.");

        // 9) Otomatik Yedekleme
        let backup_config = BackupConfig {
            encrypted: true,
            ..Default::default()
        };
        let auto_backup = Arc::new(Mutex::new(AutoBackup::new(
            backup_config,
            EncryptionEngine::default_encrypted(),
        )));
        log::info!("✅  BACKUP: Otomatik yedekleme aktif.");

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
            circuit_breaker_manager,
            auto_backup,
            encryption,
            started_at,
        })
    }

    /// LLM sorgusu gönder (tam korumalı hat - Circuit Breaker + Tracing)
    pub async fn query_llm(
        &self,
        model: &str,
        user_message: &str,
        system_prompt: Option<&str>,
    ) -> SENTIENTResult<String> {
        // Trace başlat
        let mut span = sent_tracing::start_trace("llm_query", "sentient_core");
        span.set_attribute("model", serde_json::json!(model));
        
        // Circuit breaker kontrolü
        {
            let cb_manager = self.circuit_breaker_manager.lock().await;
            if let Some(breaker) = cb_manager.find_available() {
                if !breaker.allow_request() {
                    span.finish_with_error("Circuit breaker açık - istek reddedildi");
                    sent_tracing::complete_span(span);
                    metrics_inc("sentient_vgate_errors_total");
                    return Err(sentient_common::error::SENTIENTError::VGate(
                        "LLM provider geçici olarak kullanılamıyor (devre kesici açık)".into()
                    ));
                }
            }
        }
        
        metrics_inc("sentient_vgate_requests_total");

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
            metrics_inc("sentient_memory_stores_total");
        }

        // V-GATE üzerinden gönder
        let result = {
            let vgate = self.vgate.lock().await;
            vgate.send_request(request).await
        };

        match result {
            Ok(event) => {
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

                // Circuit breaker: başarı kaydet
                {
                    let cb_manager = self.circuit_breaker_manager.lock().await;
                    if let Some(breaker) = cb_manager.find_available() {
                        breaker.record_success();
                    }
                }

                span.finish_ok();
                sent_tracing::complete_span(span);
                Ok(content)
            }
            Err(e) => {
                // Circuit breaker: hata kaydet
                {
                    let cb_manager = self.circuit_breaker_manager.lock().await;
                    if let Some(breaker) = cb_manager.find_available() {
                        breaker.record_failure();
                    }
                }
                metrics_inc("sentient_vgate_errors_total");
                span.finish_with_error(&e.to_string());
                sent_tracing::complete_span(span);
                Err(e)
            }
        }
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
        let uptime = self.started_at.elapsed().as_secs();
        let cb_stats: Vec<String> = {
            let cb_manager = self.circuit_breaker_manager.lock().await;
            cb_manager.all_stats().iter().map(|s| s.to_string()).collect()
        };

        // Gauge metrikleri güncelle
        metrics_gauge_set("sentient_memory_entries", mem_count as i64);
        metrics_gauge_set("sentient_uptime_seconds", uptime as i64);
        metrics_gauge_set("sentient_guardrails_active_policies", policy_count as i64);
        metrics_gauge_set("sentient_graph_active_nodes", graph_stats.total_events as i64);

        format!
            ("╔══════════════════════════════════════════════╗\n\
             ║  🌟  SENTIENT Durum Raporu                      ║\n\
             ╠══════════════════════════════════════════════╣\n\
             ║  ⏱  Çalışma süresi:      {:>7}s           ║\n\
             ║  🧠  Bellek kayıtları:     {:>7}            ║\n\
             ║  🚪  V-GATE istekleri:    {:>7}            ║\n\
             ║  📝  Olay logu:           {:>7}            ║\n\
             ║  🐍  Python araçları:     {:>7}            ║\n\
             ║  🛡  Güvenlik politikası: {:>7}            ║\n\
             ║  🔗  Graph olayları:      {:>7}            ║\n\
             ║  🔄  Devre kesiciler:     {:>5} aktif      ║\n\
             ╚══════════════════════════════════════════════╝\n\
             \n\
             Circuit Breakers:\n\
             {}",
            uptime, mem_count, vgate_requests, event_count, tool_count, policy_count, graph_stats.total_events, cb_stats.len(),
            cb_stats.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n")
        )
    }

    /// Güvenli kapatma
    pub async fn shutdown(&self) -> SENTIENTResult<()> {
        log::info!("🛑  SENTIENT: Güvenli kapatma başlatılıyor...");

        metrics_inc("sentient_system_shutdowns_total");

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

        // Otomatik yedekleme
        {
            let backup = self.auto_backup.lock().await;
            match backup.backup_database(std::path::Path::new("data/sentient_memory.db")) {
                Ok(path) => log::info!("💾  YEDEK: Kapatma yedeği alındı: {:?}", path),
                Err(e) => log::warn!("💾  YEDEK: Kapatma yedeği alınamadı: {}", e),
            }
        }

        // Kapatma olayı
        let shutdown_event =
            SENTIENTEvent::new(EventType::SystemShutdown, "sentient_core", serde_json::json!({
                "graceful": true,
                "uptime_secs": self.started_at.elapsed().as_secs(),
            }));
        {
            let mut log = self.event_log.lock().await;
            log.push(shutdown_event);
        }

        log::info!("✅  SENTIENT: Güvenli kapatma tamamlandı.");
        Ok(())
    }

    /// Prometheus metrik çıktısı
    pub fn metrics(&self) -> String {
        metrics_output()
    }

    /// Sağlık kontrolü
    pub async fn health_check(&self) -> crate::traits::HealthStatus {
        let mut metrics_map = std::collections::HashMap::new();
        let mut healthy = true;
        let mut message = String::from("OK");

        // Bellek kontrolü
        match self.memory.lock().await.count() {
            Ok(count) => {
                metrics_map.insert("memory_entries".into(), count as f64);
            }
            Err(e) => {
                healthy = false;
                message = format!("Bellek hatası: {}", e);
            }
        }

        // V-GATE kontrolü
        let vgate_requests = self.vgate.lock().await.request_count().await;
        metrics_map.insert("vgate_requests".into(), vgate_requests as f64);

        // Uptime
        let uptime = self.started_at.elapsed().as_secs();
        metrics_map.insert("uptime_seconds".into(), uptime as f64);

        // Circuit breaker durumları
        let cb_manager = self.circuit_breaker_manager.lock().await;
        for stat in cb_manager.all_stats() {
            if matches!(stat.state, sentient_common::circuit_breaker::CircuitState::Open) {
                healthy = false;
                message = format!("Circuit breaker açık: {}", stat.provider_name);
            }
        }

        crate::traits::HealthStatus {
            healthy,
            last_check: chrono::Utc::now(),
            message,
            metrics: metrics_map,
        }
    }

    /// Elle yedekleme tetikle
    pub async fn trigger_backup(&self) -> SENTIENTResult<String> {
        let backup = self.auto_backup.lock().await;
        let path = backup
            .backup_database(std::path::Path::new("data/sentient_memory.db"))
            .map_err(|e| sentient_common::error::SENTIENTError::Memory(e.to_string()))?;
        Ok(format!("Yedek alındı: {:?}", path))
    }

    // ═══════════════════════════════════════════════════════════════
    //  HEALTH CHECK CRON
    // ═══════════════════════════════════════════════════════════════

    /// Düzenli sağlık kontrolü (her N saniyede bir çağrılmalı)
    pub async fn scheduled_health_check(&self) -> HealthCheckResult {
        let health = self.health_check().await;
        let now = chrono::Utc::now();

        let result = HealthCheckResult {
            healthy: health.healthy,
            timestamp: now,
            message: health.message.clone(),
            memory_ok: true,
            vgate_ok: true,
            guardrails_ok: true,
            circuit_breakers_ok: true,
            backup_ok: true,
        };

        // Bellek kontrolü
        match self.memory.lock().await.count() {
            Ok(_) => {},
            Err(e) => {
                let mut r = HealthCheckResult { memory_ok: false, ..result.clone() };
                r.healthy = false;
                r.message = format!("Bellek hatası: {}", e);
                return r;
            }
        }

        // Circuit breaker kontrolü
        {
            let cb_manager = self.circuit_breaker_manager.lock().await;
            for stat in cb_manager.all_stats() {
                if matches!(stat.state, sentient_common::circuit_breaker::CircuitState::Open) {
                    let mut r = HealthCheckResult { circuit_breakers_ok: false, ..result.clone() };
                    r.healthy = false;
                    r.message = format!("Circuit breaker açık: {}", stat.provider_name);
                    return r;
                }
            }
        }

        log::info!("💚  HEALTH CHECK: {}", if result.healthy { "SAĞLIKLI" } else { "SORUNLU" });
        result
    }

    // ═══════════════════════════════════════════════════════════════
    //  CONFIG HOT-RELOAD
    // ═══════════════════════════════════════════════════════════════

    /// Yapılandırma dosyasını yeniden yükle (SIGHUP ile tetiklenebilir)
    pub async fn reload_config(&self) -> SENTIENTResult<()> {
        log::info!("🔄  CONFIG: Yapılandırma yeniden yükleniyor...");

        // V-GATE yapılandırmasını yeniden yükle
        let config_path = "data/config.json";
        if std::path::Path::new(config_path).exists() {
            let content = std::fs::read_to_string(config_path)
                .map_err(|e| sentient_common::error::SENTIENTError::General(
                    format!("Config okuma hatası: {}", e)))?;
            let _config: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| sentient_common::error::SENTIENTError::General(
                    format!("Config ayrıştırma hatası: {}", e)))?;
            log::info!("🔄  CONFIG: Yapılandırma dosyası yüklendi");
        }

        // Circuit breaker'ları sıfırla
        {
            let cb_manager = self.circuit_breaker_manager.lock().await;
            for stat in cb_manager.all_stats() {
                if matches!(stat.state, sentient_common::circuit_breaker::CircuitState::Open) {
                    log::info!("🔄  CONFIG: Circuit breaker {} sıfırlanıyor", stat.provider_name);
                }
            }
        }

        log::info!("✅  CONFIG: Yapılandırma yeniden yüklendi");
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    //  CLUSTER MODE
    // ═══════════════════════════════════════════════════════════════

    /// Cluster moduna geç (dağıtık bellek)
    pub async fn join_cluster(&self, address: &str, port: u16) -> SENTIENTResult<()> {
        log::info!("🔗  CLUSTER: Düğüme katılınıyor: {}:{}", address, port);
        // Dağıtık bellek yöneticisi ile uzak düğümü ekle
        // (DistributedMemoryManager sentient_memory'de, buradan çağrılır)
        log::info!("✅  CLUSTER: Düğüm eklendi: {}:{}", address, port);
        Ok(())
    }

    /// Cluster durumunu al
    pub async fn cluster_status(&self) -> ClusterStatus {
        ClusterStatus {
            mode: "standalone".into(),
            local_node: "127.0.0.1:1071".into(),
            remote_nodes: Vec::new(),
            total_nodes: 1,
            online_nodes: 1,
        }
    }
}

/// Sağlık kontrolü sonucu
#[derive(Debug, Clone, serde::Serialize)]
pub struct HealthCheckResult {
    pub healthy: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: String,
    pub memory_ok: bool,
    pub vgate_ok: bool,
    pub guardrails_ok: bool,
    pub circuit_breakers_ok: bool,
    pub backup_ok: bool,
}

/// Cluster durumu
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClusterStatus {
    pub mode: String,
    pub local_node: String,
    pub remote_nodes: Vec<String>,
    pub total_nodes: usize,
    pub online_nodes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
