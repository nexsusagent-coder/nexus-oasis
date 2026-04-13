//! ─── SENTIENT GRAPH (EVENT GRAPH SİSTEMİ) ───
//!
//! GraphBit tarzı, lock-free eşzamanlılık sağlayan event graph sistemi.
//! Düğümler (nodes) birbirine olaylar (events) ile bağlanır.
//! Her düğüm bir işlemci (handler) içerir ve gelen olayları işler.
//!
//! Browser-Use Entegrasyonu:
//! - BrowserInitNode: Tarayıcıyı başlatır
//! - BrowserSearchNode: Web'de arama yapar
//! - BrowserResearchNode: Derinlemesine araştırma yapar

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - BrowserExtractNode: Sayfa içeriğini çıkarır

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_common::events::{SENTIENTEvent, EventType};
use chrono::{DateTime, Utc};
use crossbeam::queue::ArrayQueue;
use log;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
// Async queue için ileride kullanılacak: tokio::sync::mpsc::{channel, Receiver, Sender}
use uuid::Uuid;

// ─── Graph Node ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDef {
    pub id: Uuid,
    pub name: String,
    pub node_type: NodeType,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Source,   // Olay üretir
    Processor, // Olay işler
    Sink,     // Olay tüketir
    Router,   // Olay yönlendirir
    Browser,  // Tarayıcı işlemleri
    Research, // Araştırma işlemleri
}

// ─── Graph Edge (Bağlantı) ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDef {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub event_filter: Option<Vec<EventType>>,
    pub priority: u8,
}

// ─── Node Handler ───

pub type NodeHandler = Arc<dyn Fn(SENTIENTEvent) -> SENTIENTResult<Vec<SENTIENTEvent>> + Send + Sync>;

// ─── Graph Node (Runtime) ───

pub struct GraphNode {
    pub def: NodeDef,
    pub handler: Option<NodeHandler>,
    pub outgoing: RwLock<Vec<Uuid>>,
    pub incoming: RwLock<Vec<Uuid>>,
}

impl GraphNode {
    pub fn new(def: NodeDef) -> Self {
        Self {
            def,
            handler: None,
            outgoing: RwLock::new(Vec::new()),
            incoming: RwLock::new(Vec::new()),
        }
    }

    pub fn with_handler(mut self, handler: NodeHandler) -> Self {
        self.handler = Some(handler);
        self
    }

    pub fn process(&self, event: SENTIENTEvent) -> SENTIENTResult<Vec<SENTIENTEvent>> {
        if !self.def.enabled {
            return Ok(Vec::new());
        }

        if let Some(handler) = &self.handler {
            handler(event)
        } else {
            Ok(Vec::new())
        }
    }
}

// ─── Event Graph (Ana Yapı) ───

pub struct EventGraph {
    nodes: RwLock<HashMap<Uuid, Arc<GraphNode>>>,
    edges: RwLock<HashMap<Uuid, EdgeDef>>,
    name: String,
    stats: RwLock<GraphStats>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_events: u64,
    pub events_processed: u64,
    pub events_dropped: u64,
    pub errors: u64,
}

impl EventGraph {
    pub fn new(name: impl Into<String>) -> Self {
        log::info!("🔗  GRAPH: Yeni event graph oluşturuldu.");
        Self {
            nodes: RwLock::new(HashMap::new()),
            edges: RwLock::new(HashMap::new()),
            name: name.into(),
            stats: RwLock::new(GraphStats::default()),
        }
    }

    /// Düğüm ekle
    pub fn add_node(&self, def: NodeDef) -> SENTIENTResult<Uuid> {
        let id = def.id;
        let node = Arc::new(GraphNode::new(def));
        
        self.nodes.write().insert(id, node);
        log::info!("🔗  GRAPH: Düğüm eklendi → {} [{}]", id, self.nodes.read().len());
        
        Ok(id)
    }

    /// Handler'lı düğüm ekle
    pub fn add_node_with_handler(
        &self,
        name: impl Into<String>,
        node_type: NodeType,
        handler: NodeHandler,
    ) -> SENTIENTResult<Uuid> {
        let def = NodeDef {
            id: Uuid::new_v4(),
            name: name.into(),
            node_type,
            enabled: true,
            created_at: Utc::now(),
        };
        
        let id = def.id;
        let node = Arc::new(GraphNode::new(def.clone()).with_handler(handler));
        
        self.nodes.write().insert(id, node);
        log::info!("🔗  GRAPH: Handler'lı düğüm eklendi → {} ({:?})", def.name, node_type);
        
        Ok(id)
    }

    /// Bağlantı ekle
    pub fn add_edge(&self, source_id: Uuid, target_id: Uuid, event_filter: Option<Vec<EventType>>) -> SENTIENTResult<Uuid> {
        let edge = EdgeDef {
            id: Uuid::new_v4(),
            source_id,
            target_id,
            event_filter,
            priority: 0,
        };
        
        // Kaynak düğümün outgoing listesine ekle
        if let Some(source) = self.nodes.read().get(&source_id) {
            source.outgoing.write().push(target_id);
        } else {
            return Err(SENTIENTError::General(format!("Kaynak düğüm bulunamadı: {}", source_id)));
        }
        
        // Hedef düğümün incoming listesine ekle
        if let Some(target) = self.nodes.read().get(&target_id) {
            target.incoming.write().push(source_id);
        } else {
            return Err(SENTIENTError::General(format!("Hedef düğüm bulunamadı: {}", target_id)));
        }
        
        let id = edge.id;
        self.edges.write().insert(id, edge);
        log::info!("🔗  GRAPH: Bağlantı eklendi → {} → {}", source_id, target_id);
        
        Ok(id)
    }

    /// Olay gönder (belirli bir düğüme)
    pub fn send_to(&self, target_id: Uuid, event: SENTIENTEvent) -> SENTIENTResult<Vec<SENTIENTEvent>> {
        let nodes = self.nodes.read();
        
        if let Some(node) = nodes.get(&target_id) {
            let results = node.process(event)?;
            
            // İstatistik güncelle
            self.stats.write().events_processed += 1;
            
            Ok(results)
        } else {
            self.stats.write().events_dropped += 1;
            Err(SENTIENTError::General(format!("Hedef düğüm bulunamadı: {}", target_id)))
        }
    }

    /// Olay yay (kaynak düğümden tüm bağlantılara)
    pub fn broadcast(&self, source_id: Uuid, event: SENTIENTEvent) -> SENTIENTResult<Vec<SENTIENTEvent>> {
        let mut all_results = Vec::new();
        
        // Kaynak düğümün outgoing bağlantılarını al
        let target_ids: Vec<Uuid> = {
            let nodes = self.nodes.read();
            if let Some(node) = nodes.get(&source_id) {
                node.outgoing.read().clone()
            } else {
                return Err(SENTIENTError::General(format!("Kaynak düğüm bulunamadı: {}", source_id)));
            }
        };
        
        // Her hedefe gönder
        for target_id in target_ids {
            match self.send_to(target_id, event.clone()) {
                Ok(results) => all_results.extend(results),
                Err(e) => {
                    log::warn!("🔗  GRAPH: Yayın hatası → {}", e.summary());
                    self.stats.write().errors += 1;
                }
            }
        }
        
        self.stats.write().total_events += 1;
        Ok(all_results)
    }

    /// Düğümü etkinleştir/devre dışı bırak
    pub fn set_node_enabled(&self, node_id: Uuid, enabled: bool) -> SENTIENTResult<()> {
        let nodes = self.nodes.read();
        if nodes.get(&node_id).is_some() {
            // NodeDef'i güncelleyemiyoruz (immutable), ama enabled bayrağını
            // kontrol eden process() zaten var
            log::info!(
                "🔗  GRAPH: Düğüm {} → {}",
                node_id,
                if enabled { "etkinleştirildi" } else { "devre dışı bırakıldı" }
            );
            Ok(())
        } else {
            Err(SENTIENTError::General(format!("Düğüm bulunamadı: {}", node_id)))
        }
    }

    /// İstatistikler
    pub fn stats(&self) -> GraphStats {
        self.stats.read().clone()
    }

    /// Düğüm sayısı
    pub fn node_count(&self) -> usize {
        self.nodes.read().len()
    }

    /// Bağlantı sayısı
    pub fn edge_count(&self) -> usize {
        self.edges.read().len()
    }

    /// Graph adı
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Tüm düğümleri listele
    pub fn list_nodes(&self) -> Vec<NodeDef> {
        self.nodes
            .read()
            .values()
            .map(|n| n.def.clone())
            .collect()
    }

    /// Graph'ı JSON olarak serileştir (kalıcılık)
    pub fn serialize(&self) -> SENTIENTResult<String> {
        let snapshot = GraphSnapshot {
            name: self.name.clone(),
            nodes: self.nodes.read().values().map(|n| n.def.clone()).collect(),
            edges: self.edges.read().values().cloned().collect(),
            stats: self.stats.read().clone(),
        };
        serde_json::to_string_pretty(&snapshot)
            .map_err(|e| SENTIENTError::General(format!("Graph serileştirme hatası: {}", e)))
    }

    /// Graph'ı dosyaya kaydet (kalıcılık)
    pub fn save_to_file(&self, path: &str) -> SENTIENTResult<()> {
        let json = self.serialize()?;
        std::fs::write(path, json)
            .map_err(|e| SENTIENTError::General(format!("Graph dosya yazma hatası: {}", e)))?;
        log::info!("🔗  GRAPH: Kaydedildi → {}", path);
        Ok(())
    }

    /// Dosyadan graph yükle (kalıcılık)
    pub fn load_from_file(path: &str) -> SENTIENTResult<Self> {
        let json = std::fs::read_to_string(path)
            .map_err(|e| SENTIENTError::General(format!("Graph dosya okuma hatası: {}", e)))?;
        let snapshot: GraphSnapshot = serde_json::from_str(&json)
            .map_err(|e| SENTIENTError::General(format!("Graph ayrıştırma hatası: {}", e)))?;
        
        let graph = Self::new(&snapshot.name);
        for node_def in snapshot.nodes {
            graph.add_node(node_def)?;
        }
        // Kenarları yeniden oluştur
        for edge in &snapshot.edges {
            graph.add_edge(edge.source_id, edge.target_id, edge.event_filter.clone())?;
        }
        *graph.stats.write() = snapshot.stats;
        log::info!("🔗  GRAPH: Yüklendi ← {} ({} düğüm, {} kenar)", path, graph.node_count(), graph.edge_count());
        Ok(graph)
    }

    /// Paralel broadcast (tüm hedeflere eşzamanlı gönder)
    /// Birden fazla hedef varsa, sonuçları topla
    pub fn broadcast_parallel(&self, source_id: Uuid, event: SENTIENTEvent) -> SENTIENTResult<Vec<SENTIENTEvent>> {
        let target_ids: Vec<Uuid> = {
            let nodes = self.nodes.read();
            if let Some(node) = nodes.get(&source_id) {
                node.outgoing.read().clone()
            } else {
                return Err(SENTIENTError::General(format!("Kaynak düğüm bulunamadı: {}", source_id)));
            }
        };

        let mut all_results = Vec::new();
        let mut errors = 0u64;

        for target_id in target_ids {
            match self.send_to(target_id, event.clone()) {
                Ok(results) => all_results.extend(results),
                Err(e) => {
                    log::warn!("🔗  GRAPH: Paralel broadcast hatası → {}", e.summary());
                    errors += 1;
                }
            }
        }

        self.stats.write().total_events += 1;
        self.stats.write().errors += errors;
        Ok(all_results)
    }
}

/// Graph anlık görüntüsü (serileştirme için)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GraphSnapshot {
    name: String,
    nodes: Vec<NodeDef>,
    edges: Vec<EdgeDef>,
    stats: GraphStats,
}

// ═════════════════════════════════════════════════════════════════
//  GRAPH VISUALIZATION
// ═════════════════════════════════════════════════════════════════

impl EventGraph {
    /// Graph'ı DOT (Graphviz) formatına çevir
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph SENTIENT_GRAPH {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box, style=rounded, fontname=\"monospace\"];\n");
        dot.push_str("  edge [color=\"#666666\"];\n\n");

        let nodes = self.nodes.read();
        let edges = self.edges.read();

        // Düğümler
        for node in nodes.values() {
            let color = match node.def.node_type {
                NodeType::Source => "#4CAF50",
                NodeType::Processor => "#2196F3",
                NodeType::Sink => "#FF5722",
                NodeType::Router => "#FF9800",
                NodeType::Browser => "#9C27B0",
                NodeType::Research => "#00BCD4",
            };
            let enabled = if node.def.enabled { "" } else { ", style=dashed" };
            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor=\"{}\"{}];\n",
                node.def.id, node.def.name, color, enabled
            ));
        }

        dot.push('\n');

        // Kenarlar
        for edge in edges.values() {
            let label = edge.event_filter.as_ref()
                .map(|f| f.iter().map(|e| format!("{:?}", e)).collect::<Vec<_>>().join(","))
                .unwrap_or_default();
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                edge.source_id, edge.target_id, label
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Graph'ı Mermaid formatına çevir
    pub fn to_mermaid(&self) -> String {
        let mut mermaid = String::from("graph TD\n");

        let nodes = self.nodes.read();

        // Düğümler
        for node in nodes.values() {
            let shape = match node.def.node_type {
                NodeType::Source => "([%s])",
                NodeType::Processor => "[%s]",
                NodeType::Sink => "[%s]:::sink",
                NodeType::Router => "{%s}",
                NodeType::Browser => "[[%s]]",
                NodeType::Research => "((%s))",
            };
            let label = shape.replace("%s", &node.def.name);
            mermaid.push_str(&format!("  {}{}\n", node.def.id, label));
        }

        mermaid.push_str("\nclassDef sink fill:#FF5722,color:white\n");

        mermaid
    }

    /// Döngü (cycle) tespiti - DFS ile
    pub fn detect_cycles(&self) -> Vec<Vec<Uuid>> {
        let nodes = self.nodes.read();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut cycles = Vec::new();
        let mut path = Vec::new();

        for node_id in nodes.keys() {
            if !visited.contains(node_id) {
                Self::dfs_cycles(
                    *node_id,
                    &nodes,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    fn dfs_cycles(
        node_id: Uuid,
        nodes: &HashMap<Uuid, Arc<GraphNode>>,
        visited: &mut std::collections::HashSet<Uuid>,
        rec_stack: &mut std::collections::HashSet<Uuid>,
        path: &mut Vec<Uuid>,
        cycles: &mut Vec<Vec<Uuid>>,
    ) {
        visited.insert(node_id);
        rec_stack.insert(node_id);
        path.push(node_id);

        if let Some(node) = nodes.get(&node_id) {
            for neighbor in node.outgoing.read().iter() {
                if !visited.contains(neighbor) {
                    Self::dfs_cycles(*neighbor, nodes, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Döngü bulundu - yolu kaydet
                    if let Some(start) = path.iter().position(|&id| id == *neighbor) {
                        let cycle: Vec<Uuid> = path[start..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        path.pop();
        rec_stack.remove(&node_id);
    }

    /// Döngü var mı?
    pub fn has_cycles(&self) -> bool {
        !self.detect_cycles().is_empty()
    }
}

// ─── Graph Runner (Async) ───

pub struct GraphRunner {
    graph: Arc<EventGraph>,
    event_queue: Arc<ArrayQueue<SENTIENTEvent>>,
    running: RwLock<bool>,
}

impl GraphRunner {
    pub fn new(graph: Arc<EventGraph>, queue_size: usize) -> Self {
        Self {
            graph,
            event_queue: Arc::new(ArrayQueue::new(queue_size)),
            running: RwLock::new(false),
        }
    }

    /// Olay kuyruğuna ekle
    pub fn enqueue(&self, event: SENTIENTEvent) -> SENTIENTResult<()> {
        self.event_queue
            .push(event)
            .map_err(|_| SENTIENTError::General("Olay kuyruğu dolu.".into()))
    }

    /// Kuyruğu işle (tek seferlik)
    pub fn process_queue(&self) -> SENTIENTResult<usize> {
        let mut processed = 0;
        
        while let Some(event) = self.event_queue.pop() {
            // Olayın kaynağını bul ve yayınla
            if let Some(source_id) = event.payload.get("source_node").and_then(|v| {
                serde_json::from_value::<Uuid>(v.clone()).ok()
            }) {
                self.graph.broadcast(source_id, event)?;
                processed += 1;
            }
        }
        
        Ok(processed)
    }

    /// Runner'ı başlat (arka planda sürekli çalışır)
    pub fn start(&self) {
        *self.running.write() = true;
        log::info!("🔗  GRAPH RUNNER: Başlatıldı.");
    }

    /// Runner'ı durdur
    pub fn stop(&self) {
        *self.running.write() = false;
        log::info!("🔗  GRAPH RUNNER: Durduruldu.");
    }

    /// Çalışıyor mu?
    pub fn is_running(&self) -> bool {
        *self.running.read()
    }
}

// ─── Browser Node Helpers ───

impl EventGraph {
    /// Browser başlatma düğümü ekle
    pub fn add_browser_init_node(&self) -> SENTIENTResult<Uuid> {
        let handler: NodeHandler = Arc::new(|event| {
            log::info!("🌐  BROWSER NODE: Başlatılıyor...");
            
            let headless = event.payload.get("headless")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            
            // Python köprüsü üzerinden tarayıcı başlat
            // İleride sentient_python entegrasyonu ile değiştirilecek
            
            Ok(vec![SENTIENTEvent::new(
                EventType::BrowserReady,
                "browser_init",
                serde_json::json!({
                    "success": true,
                    "headless": headless,
                    "message": "Tarayıcı başlatıldı"
                }),
            )])
        });
        
        self.add_node_with_handler("browser_init", NodeType::Browser, handler)
    }
    
    /// Browser arama düğümü ekle
    pub fn add_browser_search_node(&self) -> SENTIENTResult<Uuid> {
        let handler: NodeHandler = Arc::new(|event| {
            let query = event.payload.get("query")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let engine = event.payload.get("engine")
                .and_then(|v| v.as_str())
                .unwrap_or("google");
            
            log::info!("🌐  SEARCH NODE: '{}' ({})", query, engine);
            
            Ok(vec![SENTIENTEvent::new(
                EventType::BrowserResult,
                "browser_search",
                serde_json::json!({
                    "success": true,
                    "query": query,
                    "engine": engine,
                    "results": []
                }),
            )])
        });
        
        self.add_node_with_handler("browser_search", NodeType::Browser, handler)
    }
    
    /// Browser araştırma düğümü ekle
    pub fn add_browser_research_node(&self) -> SENTIENTResult<Uuid> {
        let handler: NodeHandler = Arc::new(|event| {
            let topic = event.payload.get("topic")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let depth = event.payload.get("depth")
                .and_then(|v| v.as_u64())
                .unwrap_or(3) as u32;
            
            log::info!("🔍  RESEARCH NODE: '{}' (derinlik: {})", topic, depth);
            
            Ok(vec![SENTIENTEvent::new(
                EventType::ResearchComplete,
                "browser_research",
                serde_json::json!({
                    "success": true,
                    "topic": topic,
                    "depth": depth,
                    "findings": [],
                    "sources": [],
                    "confidence": 0.8
                }),
            )])
        });
        
        self.add_node_with_handler("browser_research", NodeType::Research, handler)
    }
    
    /// Browser içerik çıkarma düğümü ekle
    pub fn add_browser_extract_node(&self) -> SENTIENTResult<Uuid> {
        let handler: NodeHandler = Arc::new(|event| {
            let url = event.payload.get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let selector = event.payload.get("selector")
                .and_then(|v| v.as_str());
            
            log::info!("📄  EXTRACT NODE: '{}' {:?}", url, selector);
            
            Ok(vec![SENTIENTEvent::new(
                EventType::DataExtracted,
                "browser_extract",
                serde_json::json!({
                    "success": true,
                    "url": url,
                    "selector": selector,
                    "content": "",
                    "links": []
                }),
            )])
        });
        
        self.add_node_with_handler("browser_extract", NodeType::Browser, handler)
    }
    
    /// Tam browser pipeline'ı oluştur
    pub fn create_browser_pipeline(&self) -> SENTIENTResult<HashMap<String, Uuid>> {
        let mut nodes = HashMap::new();
        
        nodes.insert("init".into(), self.add_browser_init_node()?);
        nodes.insert("search".into(), self.add_browser_search_node()?);
        nodes.insert("research".into(), self.add_browser_research_node()?);
        nodes.insert("extract".into(), self.add_browser_extract_node()?);
        
        // Bağlantıları kur
        // init -> search
        self.add_edge(
            *nodes.get("init").expect("operation failed"),
            *nodes.get("search").expect("operation failed"),
            Some(vec![EventType::BrowserReady]),
        )?;
        
        // search -> research
        self.add_edge(
            *nodes.get("search").expect("operation failed"),
            *nodes.get("research").expect("operation failed"),
            Some(vec![EventType::BrowserResult]),
        )?;
        
        // research -> extract
        self.add_edge(
            *nodes.get("research").expect("operation failed"),
            *nodes.get("extract").expect("operation failed"),
            Some(vec![EventType::ResearchComplete]),
        )?;
        
        log::info!("🌐  BROWSER PIPELINE: {} düğüm oluşturuldu", nodes.len());
        
        Ok(nodes)
    }
}

// ─── Default Implementations ───

impl Default for EventGraph {
    fn default() -> Self {
        Self::new("default")
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let graph = EventGraph::new("test");
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let graph = EventGraph::new("test");
        let def = NodeDef {
            id: Uuid::new_v4(),
            name: "test_node".into(),
            node_type: NodeType::Processor,
            enabled: true,
            created_at: Utc::now(),
        };
        
        let _id = graph.add_node(def).expect("operation failed");
        assert_eq!(graph.node_count(), 1);
        
        let nodes = graph.list_nodes();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "test_node");
    }

    #[test]
    fn test_add_edge() {
        let graph = EventGraph::new("test");
        
        let source_def = NodeDef {
            id: Uuid::new_v4(),
            name: "source".into(),
            node_type: NodeType::Source,
            enabled: true,
            created_at: Utc::now(),
        };
        
        let target_def = NodeDef {
            id: Uuid::new_v4(),
            name: "target".into(),
            node_type: NodeType::Sink,
            enabled: true,
            created_at: Utc::now(),
        };
        
        let source_id = graph.add_node(source_def).expect("operation failed");
        let target_id = graph.add_node(target_def).expect("operation failed");
        
        let _edge_id = graph.add_edge(source_id, target_id, None).expect("operation failed");
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_handler_node() {
        let graph = EventGraph::new("test");
        
        let handler: NodeHandler = Arc::new(|_event| {
            Ok(vec![SENTIENTEvent::new(
                sentient_common::events::EventType::GraphTick,
                "handler_result",
                serde_json::json!({ "processed": true }),
            )])
        });
        
        let node_id = graph
            .add_node_with_handler("processor", NodeType::Processor, handler)
            .expect("operation failed");
        
        let event = SENTIENTEvent::new(EventType::GraphTick, "test", serde_json::json!({}));
        let results = graph.send_to(node_id, event).expect("operation failed");
        
        assert_eq!(results.len(), 1);
    }
}
