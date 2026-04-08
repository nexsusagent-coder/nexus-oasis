//! ═══════════════════════════════════════════════════════════════════════════════
//!  MEMORY BRIDGE - BELLEK-ORKESTRATOR KÖPRÜSÜ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! L7: Memory-Orchestrator entegrasyonu.
//!
//! Bu modül, L3: MEMORY katmanındaki MemOS ile L4: BRAIN katmanındaki
//! Orchestrator'ü birbirine bağlar.
//!
//! Özellikler:
//! - ReAct döngüsü için bağlam (context) retrieval
//! - FTS5 + Vektör hibrit araması
//! - Kısa vadeli çalışma belleği (Working Memory)
//! - Oturumlar arası kalıcı bellek (Cross-Thread Long-term Memory)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                         MEMORY BRIDGE                                       │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │   ┌──────────────────────────────────────────────────────────────────┐     │
//! │   │                      ReAct CONTEXT FLOW                          │     │
//! │   │                                                                  │     │
//! │   │   USER INPUT ──► GOAL ANALYSIS ──► CONTEXT RETRIEVAL            │     │
//! │   │         │              │                 │                       │     │
//! │   │         │              ▼                 ▼                       │     │
//! │   │         │    ┌─────────────────────────────────────┐            │     │
//! │   │         │    │     HYBRID SEARCH ENGINE            │            │     │
//! │   │         │    │  ┌─────────┐   ┌─────────────┐     │            │     │
//! │   │         │    │  │  FTS5   │ + │   VECTOR    │     │            │     │
//! │   │         │    │  │  SEARCH │   │   SEARCH    │     │            │     │
//! │   │         │    │  └─────────┘   └─────────────┘     │            │     │
//! │   │         │    └─────────────────────────────────────┘            │     │
//! │   │         │                          │                             │     │
//! │   │         ▼                          ▼                             │     │
//! │   │   ┌─────────────────────────────────────────────────┐          │     │
//! │   │   │              WORKING MEMORY                     │          │     │
//! │   │   │  (Thread-local, geçici, oturum boyunca)        │          │     │
//! │   │   │  ┌─────────┐ ┌─────────┐ ┌─────────┐          │          │     │
//! │   │   │  │ Current │ │ Recent  │ │ Scratch │          │          │     │
//! │   │   │  │  Goal   │ │ Results │ │  Pad    │          │          │     │
//! │   │   │  └─────────┘ └─────────┘ └─────────┘          │          │     │
//! │   │   └─────────────────────────────────────────────────┘          │     │
//! │   │                              │                                  │     │
//! │   │                              ▼                                  │     │
//! │   │   ┌─────────────────────────────────────────────────┐          │     │
//! │   │   │          CROSS-THREAD MEMORY                    │          │     │
//! │   │   │  (Kalıcı, oturumlar arası, uzun vadeli)        │          │     │
//! │   │   │  ┌─────────┐ ┌─────────┐ ┌─────────┐          │          │     │
//! │   │   │  │ Episodic│ │Semantic │ │Procedural│          │          │     │
//! │   │   │  │ Memory  │ │ Memory  │ │ Memory  │          │          │     │
//! │   │   │  └─────────┘ └─────────┘ └─────────┘          │          │     │
//! │   │   └─────────────────────────────────────────────────┘          │     │
//! │   │                                                                  │     │
//! │   └──────────────────────────────────────────────────────────────────┘     │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//! ```

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_memory::{
    MemOS, MemOSConfig, CubeType,
    MemoryType, MemoryInput, MemorySource, Importance,
    HybridWeights, HybridResult, FtsOptions,
    SearchResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::goal::Goal;
use crate::state::AgentContext;

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY BRIDGE - ANA KÖPRÜ YAPISI
// ═══════════════════════════════════════════════════════════════════════════════

/// Memory-Orchestrator köprüsü
/// 
/// Bu yapı, ReAct döngüsü sırasında bellek erişimini yönetir.
pub struct MemoryBridge {
    /// MemOS referansı (çoklu küp yönetimi)
    memos: Arc<RwLock<MemOS>>,
    /// Aktif küp ID'si
    active_cube_id: RwLock<Option<Uuid>>,
    /// Çalışma belleği (thread-local)
    working_memory: RwLock<WorkingMemory>,
    /// Oturum ID'si
    session_id: Uuid,
    /// Yapılandırma
    config: BridgeConfig,
}

/// Köprü yapılandırması
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Hibrit arama ağırlıkları
    pub hybrid_weights: HybridWeights,
    /// Maksimum bağlam boyutu
    pub max_context_items: usize,
    /// Çalışma belleği kapasitesi
    pub working_memory_capacity: usize,
    /// Otomatik konsolidasyon aralığı (iterasyon)
    pub auto_consolidation_interval: u32,
    /// Epizodik bellek aktif mi?
    pub enable_episodic: bool,
    /// Semantik bellek aktif mi?
    pub enable_semantic: bool,
    /// Debug modu
    pub debug: bool,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            hybrid_weights: HybridWeights::default(),
            max_context_items: 10,
            working_memory_capacity: 100,
            auto_consolidation_interval: 10,
            enable_episodic: true,
            enable_semantic: true,
            debug: false,
        }
    }
}

impl MemoryBridge {
    /// Yeni köprü oluştur
    pub async fn new(db_path: &str) -> SENTIENTResult<Self> {
        let config = MemOSConfig {
            data_dir: PathBuf::from(db_path),
            ..Default::default()
        };
        
        let memos = MemOS::new(config).await
            .map_err(|e| SENTIENTError::Memory(format!("MemOS başlatılamadı: {}", e)))?;
        
        Ok(Self {
            memos: Arc::new(RwLock::new(memos)),
            active_cube_id: RwLock::new(None),
            working_memory: RwLock::new(WorkingMemory::new(100)),
            session_id: Uuid::new_v4(),
            config: BridgeConfig::default(),
        })
    }
    
    /// Yapılandırma ile oluştur
    pub async fn with_config(db_path: &str, config: BridgeConfig) -> SENTIENTResult<Self> {
        let memos_config = MemOSConfig {
            data_dir: PathBuf::from(db_path),
            ..Default::default()
        };
        
        let memos = MemOS::new(memos_config).await
            .map_err(|e| SENTIENTError::Memory(format!("MemOS başlatılamadı: {}", e)))?;
        
        Ok(Self {
            memos: Arc::new(RwLock::new(memos)),
            active_cube_id: RwLock::new(None),
            working_memory: RwLock::new(WorkingMemory::new(config.working_memory_capacity)),
            session_id: Uuid::new_v4(),
            config,
        })
    }
    
    /// Mevcut MemOS ile oluştur (shared)
    pub fn from_memos(memos: Arc<RwLock<MemOS>>, config: BridgeConfig) -> Self {
        Self {
            memos,
            active_cube_id: RwLock::new(None),
            working_memory: RwLock::new(WorkingMemory::new(config.working_memory_capacity)),
            session_id: Uuid::new_v4(),
            config,
        }
    }
    
    /// Kullanıcı için küp oluştur/aktif et
    pub async fn activate_user_cube(&self, user_id: &str) -> SENTIENTResult<Uuid> {
        let mut memos = self.memos.write().await;
        
        // Mevcut küpü ara
        let cubes = memos.list_cubes().await
            .map_err(|e| SENTIENTError::Memory(format!("Küp listesi alınamadı: {}", e)))?;
        
        for cube in cubes {
            if cube.owner == user_id && cube.cube_type == CubeType::User {
                *self.active_cube_id.write().await = Some(cube.id);
                return Ok(cube.id);
            }
        }
        
        // Yeni küp oluştur
        let cube_id = memos.create_cube(user_id, CubeType::User).await
            .map_err(|e| SENTIENTError::Memory(format!("Küp oluşturulamadı: {}", e)))?;
        
        *self.active_cube_id.write().await = Some(cube_id);
        
        log::info!("🧠  MEMORY-BRIDGE: Kullanıcı küpü aktif → {} ({})", user_id, cube_id);
        
        Ok(cube_id)
    }
    
    /// Ajan için küp oluştur/aktif et
    pub async fn activate_agent_cube(&self, agent_id: &str) -> SENTIENTResult<Uuid> {
        let mut memos = self.memos.write().await;
        
        let cubes = memos.list_cubes().await
            .map_err(|e| SENTIENTError::Memory(format!("Küp listesi alınamadı: {}", e)))?;
        
        for cube in cubes {
            if cube.owner == agent_id && cube.cube_type == CubeType::Agent {
                *self.active_cube_id.write().await = Some(cube.id);
                return Ok(cube.id);
            }
        }
        
        let cube_id = memos.create_cube(agent_id, CubeType::Agent).await
            .map_err(|e| SENTIENTError::Memory(format!("Küp oluşturulamadı: {}", e)))?;
        
        *self.active_cube_id.write().await = Some(cube_id);
        
        log::info!("🧠  MEMORY-BRIDGE: Ajan küpü aktif → {} ({})", agent_id, cube_id);
        
        Ok(cube_id)
    }
    
    /// Aktif küp ID'si
    pub async fn active_cube(&self) -> Option<Uuid> {
        *self.active_cube_id.read().await
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  ReAct CONTEXT RETRIEVAL
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// ReAct döngüsü için bağlam al
    /// 
    /// Bu metod, ajanın karar vermesi için gerekli tüm bağlamı toplar:
    /// 1. Hedef ile ilgili geçmiş deneyimler (episodik)
    /// 2. İlgili gerçekler ve bilgiler (semantik)
    /// 3. Önceki oturumlardan öğrenilen prosedürler
    /// 4. Çalışma belleğindeki son durum
    pub async fn retrieve_context(&self, goal: &Goal, context: &AgentContext) -> SENTIENTResult<ReActContext> {
        let start = std::time::Instant::now();
        
        // 1. Hedef anahtar kelimelerini çıkar
        let keywords = self.extract_keywords(&goal.description);
        
        // 2. Hibrit arama ile ilgili bellekleri bul
        let hybrid_results = self.hybrid_search(&goal.description).await?;
        
        // 3. Bellek tipine göre ayır
        let mut episodic = Vec::new();
        let mut semantic = Vec::new();
        let mut procedural = Vec::new();
        
        for result in hybrid_results {
            match result.memory_type {
                MemoryType::Episodic => episodic.push(result),
                MemoryType::Semantic => semantic.push(result),
                MemoryType::Procedural => procedural.push(result),
                _ => {}
            }
        }
        
        // 4. Çalışma belleğinden son durumu al
        let working_state = self.get_working_state().await;
        
        // 5. Bağlam oluştur
        let react_context = ReActContext {
            goal_summary: goal.description.chars().take(200).collect(),
            keywords: keywords.clone(),
            episodic_memories: episodic.into_iter().take(5).collect(),
            semantic_memories: semantic.into_iter().take(5).collect(),
            procedural_memories: procedural.into_iter().take(3).collect(),
            working_memory: working_state,
            iteration: context.iteration,
            completed_tasks: context.completed_tasks.len(),
            failed_tasks: context.failed_tasks.len(),
            retrieved_at: Utc::now(),
            retrieval_duration_ms: start.elapsed().as_millis() as u64,
        };
        
        if self.config.debug {
            log::debug!("🧠  MEMORY-BRIDGE: Bağlam alındı ({}ms)", start.elapsed().as_millis());
            log::debug!("    Episodik: {}, Semantik: {}, Prosedürel: {}", 
                react_context.episodic_memories.len(),
                react_context.semantic_memories.len(),
                react_context.procedural_memories.len());
        }
        
        Ok(react_context)
    }
    
    /// Hızlı bağlam al (önbellekten)
    pub async fn quick_context(&self, query: &str) -> SENTIENTResult<Vec<String>> {
        let mut working = self.working_memory.write().await;
        
        // Önce çalışma belleğinde ara
        if let Some(cached) = working.cache.get(query) {
            return Ok(cached.clone());
        }
        
        // Hibrit arama yap
        let results = self.hybrid_search(query).await?;
        
        // Sonuçları string'e çevir
        let contexts: Vec<String> = results.iter()
            .take(self.config.max_context_items)
            .map(|r| format!("[{}] {}", r.memory_type, r.content.chars().take(500).collect::<String>()))
            .collect();
        
        // Önbelleğe al
        working.cache.insert(query.to_string(), contexts.clone());
        
        Ok(contexts)
    }
    
    /// Hibrit arama (FTS5 + Vektör)
    async fn hybrid_search(&self, query: &str) -> SENTIENTResult<Vec<HybridResult>> {
        let memos = self.memos.read().await;
        let cube_id = self.active_cube_id.read().await;
        
        if let Some(cube_id) = *cube_id {
            memos.hybrid_search(cube_id, query, self.config.hybrid_weights.clone(), self.config.max_context_items).await
                .map_err(|e| SENTIENTError::Memory(format!("Hibrit arama hatası: {}", e)))
        } else {
            // Aktif küp yoksa boş dön
            Ok(Vec::new())
        }
    }
    
    /// Anahtar kelime çıkarma
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .filter(|w| !["için", "olan", "ile", "ve", "veya", "bir", "bu", "şu", "gibi", "kadar"].contains(w))
            .take(10)
            .map(String::from)
            .collect()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  WORKING MEMORY (KISA VADELİ)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Çalışma belleğine ekle
    pub async fn add_to_working_memory(&self, item: WorkingMemoryItem) {
        let mut working = self.working_memory.write().await;
        working.add(item);
    }
    
    /// Çalışma belleğine görev sonucu ekle
    pub async fn record_task_result(&self, task_id: Uuid, description: &str, result: &str, success: bool) {
        let item = WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::TaskResult,
            content: format!("{}: {}", description, result),
            metadata: serde_json::json!({
                "task_id": task_id,
                "success": success,
                "timestamp": Utc::now().to_rfc3339(),
            }),
            created_at: Utc::now(),
            importance: if success { 0.5 } else { 0.8 },
        };
        
        self.add_to_working_memory(item).await;
    }
    
    /// Çalışma belleğine karar ekle
    pub async fn record_decision(&self, decision: &str, reasoning: &str) {
        let item = WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::Decision,
            content: format!("KARAR: {} → {}", decision, reasoning),
            metadata: serde_json::json!({
                "timestamp": Utc::now().to_rfc3339(),
            }),
            created_at: Utc::now(),
            importance: 0.7,
        };
        
        self.add_to_working_memory(item).await;
    }
    
    /// Çalışma belleğine gözlem ekle
    pub async fn record_observation(&self, observation: &str, source: &str) {
        let item = WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::Observation,
            content: format!("[{}] {}", source, observation),
            metadata: serde_json::json!({
                "source": source,
                "timestamp": Utc::now().to_rfc3339(),
            }),
            created_at: Utc::now(),
            importance: 0.4,
        };
        
        self.add_to_working_memory(item).await;
    }
    
    /// Çalışma belleği durumunu al
    async fn get_working_state(&self) -> WorkingMemoryState {
        let working = self.working_memory.read().await;
        WorkingMemoryState {
            recent_items: working.recent_items(10),
            decisions: working.by_type(WorkingItemType::Decision),
            observations: working.by_type(WorkingItemType::Observation),
            failed_tasks: working.failed_tasks(),
            scratch_pad: working.scratch_pad.clone(),
        }
    }
    
    /// Karalama defterine yaz
    pub async fn write_scratch(&self, content: &str) {
        let mut working = self.working_memory.write().await;
        working.scratch_pad.push_str(content);
        working.scratch_pad.push('\n');
    }
    
    /// Karalama defterini oku
    pub async fn read_scratch(&self) -> String {
        let working = self.working_memory.read().await;
        working.scratch_pad.clone()
    }
    
    /// Karalama defterini temizle
    pub async fn clear_scratch(&self) {
        let mut working = self.working_memory.write().await;
        working.scratch_pad.clear();
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  CROSS-THREAD MEMORY (UZUN VADELİ)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Uzun vadeli belleğe kaydet
    pub async fn store_long_term(&self, content: &str, memory_type: MemoryType, importance: f32) -> SENTIENTResult<Uuid> {
        let cube_id = self.active_cube_id.read().await;
        
        if let Some(cube_id) = *cube_id {
            let memos = self.memos.read().await;
            
            let input = MemoryInput::new(content)
                .with_type(memory_type)
                .with_importance(Importance::new(importance));
            
            memos.store_memory(cube_id, input).await
                .map_err(|e| SENTIENTError::Memory(format!("Bellek kaydedilemedi: {}", e)))
        } else {
            Err(SENTIENTError::Memory("Aktif bellek küpü yok".into()))
        }
    }
    
    /// Epizodik belleğe deneyim kaydet
    pub async fn store_experience(&self, experience: &Experience) -> SENTIENTResult<Uuid> {
        let content = serde_json::to_string(experience)
            .map_err(|e| SENTIENTError::ValidationError(format!("Deneyim serialize edilemedi: {}", e)))?;
        
        self.store_long_term(&content, MemoryType::Episodic, experience.importance).await
    }
    
    /// Semantik belleğe bilgi kaydet
    pub async fn store_fact(&self, fact: &str, source: &str, confidence: f32) -> SENTIENTResult<Uuid> {
        let content = format!("[{}] {}", source, fact);
        let importance = 0.3 + (confidence * 0.4); // 0.3 - 0.7 arası
        self.store_long_term(&content, MemoryType::Semantic, importance).await
    }
    
    /// Prosedürel belleğe yöntem kaydet
    pub async fn store_procedure(&self, procedure: &Procedure) -> SENTIENTResult<Uuid> {
        let content = serde_json::to_string(procedure)
            .map_err(|e| SENTIENTError::ValidationError(format!("Prosedür serialize edilemedi: {}", e)))?;
        
        self.store_long_term(&content, MemoryType::Procedural, 0.7).await
    }
    
    /// Konsolidasyon çalıştır
    /// 
    /// Çalışma belleğini uzun vadeli belleğe aktarır
    pub async fn consolidate(&self) -> SENTIENTResult<ConsolidationResult> {
        let start = std::time::Instant::now();
        let mut stats = ConsolidationResult::default();
        
        let working = self.working_memory.read().await;
        
        // Önemli öğeleri uzun vadeli belleğe aktar
        for item in working.items.iter() {
            if item.importance >= 0.6 {
                let memory_type = match item.item_type {
                    WorkingItemType::TaskResult => MemoryType::Episodic,
                    WorkingItemType::Decision => MemoryType::Episodic,
                    WorkingItemType::Observation => MemoryType::Semantic,
                    WorkingItemType::Error => MemoryType::Episodic,
                };
                
                match self.store_long_term(&item.content, memory_type, item.importance).await {
                    Ok(_) => stats.transferred += 1,
                    Err(e) => {
                        log::warn!("🧠  MEMORY-BRIDGE: Konsolidasyon hatası: {}", e);
                        stats.failed += 1;
                    }
                }
            }
        }
        
        stats.duration_ms = start.elapsed().as_millis() as u64;
        
        log::info!("🧠  MEMORY-BRIDGE: Konsolidasyon tamamlandı → {} aktarıldı, {} başarısız ({}ms)",
            stats.transferred, stats.failed, stats.duration_ms);
        
        Ok(stats)
    }
    
    /// Oturum sonlandır
    pub async fn end_session(&self) -> SENTIENTResult<()> {
        log::info!("🧠  MEMORY-BRIDGE: Oturum sonlandırılıyor → {}", self.session_id);
        
        // Son konsolidasyon
        self.consolidate().await?;
        
        // Çalışma belleğini temizle
        let mut working = self.working_memory.write().await;
        working.clear();
        
        Ok(())
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> BridgeStats {
        let working = self.working_memory.read().await;
        let cube_id = self.active_cube_id.read().await;
        
        BridgeStats {
            session_id: self.session_id,
            active_cube: *cube_id,
            working_memory_items: working.items.len(),
            working_memory_capacity: working.capacity,
            scratch_pad_lines: working.scratch_pad.lines().count(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ReAct CONTEXT - REACT DÖNGÜSÜ BAĞLAMI
// ═══════════════════════════════════════════════════════════════════════════════

/// ReAct döngüsü için bağlam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReActContext {
    /// Hedef özeti
    pub goal_summary: String,
    /// Anahtar kelimeler
    pub keywords: Vec<String>,
    /// Epizodik bellekler (deneyimler)
    pub episodic_memories: Vec<HybridResult>,
    /// Semantik bellekler (bilgiler)
    pub semantic_memories: Vec<HybridResult>,
    /// Prosedürel bellekler (yöntemler)
    pub procedural_memories: Vec<HybridResult>,
    /// Çalışma belleği durumu
    pub working_memory: WorkingMemoryState,
    /// İterasyon sayısı
    pub iteration: u32,
    /// Tamamlanan görev sayısı
    pub completed_tasks: usize,
    /// Başarısız görev sayısı
    pub failed_tasks: usize,
    /// Alınma zamanı
    pub retrieved_at: DateTime<Utc>,
    /// Alma süresi (ms)
    pub retrieval_duration_ms: u64,
}

impl ReActContext {
    /// LLM için formatlanmış bağlam
    pub fn to_llm_context(&self) -> String {
        let mut ctx = String::new();
        
        ctx.push_str("════════════════════════════════════════════════════════════\n");
        ctx.push_str("BAĞLAM (Memory Bridge)\n");
        ctx.push_str("════════════════════════════════════════════════════════════\n\n");
        
        // Hedef
        ctx.push_str(&format!("🎯 HEDEF: {}\n\n", self.goal_summary));
        
        // İlgili geçmiş deneyimler
        if !self.episodic_memories.is_empty() {
            ctx.push_str("📚 İLGİLİ DENEYİMLER:\n");
            for (i, mem) in self.episodic_memories.iter().enumerate() {
                ctx.push_str(&format!("  {}. {}\n", i + 1, 
                    mem.content.chars().take(200).collect::<String>()));
            }
            ctx.push('\n');
        }
        
        // İlgili bilgiler
        if !self.semantic_memories.is_empty() {
            ctx.push_str("📖 İLGİLİ BİLGİLER:\n");
            for (i, mem) in self.semantic_memories.iter().enumerate() {
                ctx.push_str(&format!("  {}. {}\n", i + 1,
                    mem.content.chars().take(200).collect::<String>()));
            }
            ctx.push('\n');
        }
        
        // Öğrenilen yöntemler
        if !self.procedural_memories.is_empty() {
            ctx.push_str("🔧 ÖĞRENİLEN YÖNTEMLER:\n");
            for (i, mem) in self.procedural_memories.iter().enumerate() {
                ctx.push_str(&format!("  {}. {}\n", i + 1,
                    mem.content.chars().take(200).collect::<String>()));
            }
            ctx.push('\n');
        }
        
        // Çalışma belleği
        if !self.working_memory.recent_items.is_empty() {
            ctx.push_str("📝 SON İŞLEMLER:\n");
            for item in &self.working_memory.recent_items {
                ctx.push_str(&format!("  • {}\n", item.chars().take(100).collect::<String>()));
            }
            ctx.push('\n');
        }
        
        // Karalama defteri
        if !self.working_memory.scratch_pad.is_empty() {
            ctx.push_str("✏️ KARALAMA DEFTERİ:\n");
            ctx.push_str(&self.working_memory.scratch_pad);
            ctx.push_str("\n\n");
        }
        
        ctx.push_str(&format!("📊 İLERLEME: {} tamamlandı, {} başarısız, iterasyon {}\n",
            self.completed_tasks, self.failed_tasks, self.iteration));
        
        ctx
    }
    
    /// Kısa özet
    pub fn summary(&self) -> String {
        format!(
            "ReActContext: {} episodik, {} semantik, {} prosedürel, {} çalışma belleği ({}ms)",
            self.episodic_memories.len(),
            self.semantic_memories.len(),
            self.procedural_memories.len(),
            self.working_memory.recent_items.len(),
            self.retrieval_duration_ms
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WORKING MEMORY - KISA VADELİ ÇALIŞMA BELLEĞİ
// ═══════════════════════════════════════════════════════════════════════════════

/// Çalışma belleği
#[derive(Debug, Clone)]
pub struct WorkingMemory {
    /// Öğeler
    items: Vec<WorkingMemoryItem>,
    /// Kapasite
    capacity: usize,
    /// Karalama defteri
    scratch_pad: String,
    /// Önbellek
    cache: HashMap<String, Vec<String>>,
}

/// Çalışma belleği öğesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    /// ID
    pub id: Uuid,
    /// Tip
    pub item_type: WorkingItemType,
    /// İçerik
    pub content: String,
    /// Meta veri
    pub metadata: serde_json::Value,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Önemi (0.0-1.0)
    pub importance: f32,
}

/// Çalışma belleği öğe tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkingItemType {
    /// Görev sonucu
    TaskResult,
    /// Karar
    Decision,
    /// Gözlem
    Observation,
    /// Hata
    Error,
}

/// Çalışma belleği durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryState {
    /// Son öğeler
    pub recent_items: Vec<String>,
    /// Kararlar
    pub decisions: Vec<String>,
    /// Gözlemler
    pub observations: Vec<String>,
    /// Başarısız görevler
    pub failed_tasks: Vec<String>,
    /// Karalama defteri
    pub scratch_pad: String,
}

impl WorkingMemory {
    /// Yeni çalışma belleği
    fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            scratch_pad: String::new(),
            cache: HashMap::new(),
        }
    }
    
    /// Öğe ekle
    fn add(&mut self, item: WorkingMemoryItem) {
        self.items.push(item);
        
        // Kapasite kontrolü
        if self.items.len() > self.capacity {
            // En az önemli öğeyi çıkar
            self.items.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
            self.items.truncate(self.capacity);
        }
    }
    
    /// Son N öğeyi al
    fn recent_items(&self, n: usize) -> Vec<String> {
        self.items.iter()
            .rev()
            .take(n)
            .map(|i| i.content.clone())
            .collect()
    }
    
    /// Tipe göre öğeleri al
    fn by_type(&self, item_type: WorkingItemType) -> Vec<String> {
        self.items.iter()
            .filter(|i| i.item_type == item_type)
            .map(|i| i.content.clone())
            .collect()
    }
    
    /// Başarısız görevleri al
    fn failed_tasks(&self) -> Vec<String> {
        self.items.iter()
            .filter(|i| i.item_type == WorkingItemType::Error || 
                        (i.item_type == WorkingItemType::TaskResult && 
                         i.metadata.get("success").and_then(|v| v.as_bool()) == Some(false)))
            .map(|i| i.content.clone())
            .collect()
    }
    
    /// Temizle
    fn clear(&mut self) {
        self.items.clear();
        self.scratch_pad.clear();
        self.cache.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DENeyİM VE PROSEDÜR YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

/// Deneyim (epizodik bellek)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    /// Deneyim ID
    pub id: Uuid,
    /// Hedef
    pub goal: String,
    /// Yapılan eylemler
    pub actions: Vec<String>,
    /// Sonuç
    pub outcome: String,
    /// Başarılı mı?
    pub success: bool,
    /// Öğrenilen ders
    pub lesson: Option<String>,
    /// Önemi
    pub importance: f32,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

impl Experience {
    /// Yeni deneyim oluştur
    pub fn new(goal: &str, actions: Vec<String>, outcome: &str, success: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal: goal.into(),
            actions,
            outcome: outcome.into(),
            success,
            lesson: None,
            importance: if success { 0.5 } else { 0.8 },
            timestamp: Utc::now(),
        }
    }
    
    /// Ders ekle
    pub fn with_lesson(mut self, lesson: &str) -> Self {
        self.lesson = Some(lesson.into());
        self.importance = 0.9;
        self
    }
}

/// Prosedür (prosedürel bellek)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    /// Prosedür ID
    pub id: Uuid,
    /// İsim
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Adımlar
    pub steps: Vec<String>,
    /// Ön koşullar
    pub preconditions: Vec<String>,
    /// Son koşullar
    pub postconditions: Vec<String>,
    /// Başarı oranı
    pub success_rate: f32,
    /// Kullanım sayısı
    pub usage_count: u32,
}

impl Procedure {
    /// Yeni prosedür
    pub fn new(name: &str, steps: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: String::new(),
            steps,
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            success_rate: 0.5,
            usage_count: 0,
        }
    }
    
    /// Açıklama ekle
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.into();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SONUÇ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

/// Konsolidasyon sonucu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsolidationResult {
    /// Aktarılan öğe sayısı
    pub transferred: usize,
    /// Başarısız aktarım
    pub failed: usize,
    /// Süre (ms)
    pub duration_ms: u64,
}

/// Köprü istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStats {
    /// Oturum ID
    pub session_id: Uuid,
    /// Aktif küp
    pub active_cube: Option<Uuid>,
    /// Çalışma belleği öğe sayısı
    pub working_memory_items: usize,
    /// Çalışma belleği kapasitesi
    pub working_memory_capacity: usize,
    /// Karalama defteri satır sayısı
    pub scratch_pad_lines: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTLER
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bridge_config_default() {
        let config = BridgeConfig::default();
        assert_eq!(config.max_context_items, 10);
        assert_eq!(config.working_memory_capacity, 100);
    }
    
    #[test]
    fn test_working_memory_creation() {
        let wm = WorkingMemory::new(50);
        assert_eq!(wm.capacity, 50);
        assert!(wm.items.is_empty());
    }
    
    #[test]
    fn test_working_memory_add() {
        let mut wm = WorkingMemory::new(10);
        
        let item = WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::Decision,
            content: "Test karar".into(),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            importance: 0.5,
        };
        
        wm.add(item);
        assert_eq!(wm.items.len(), 1);
    }
    
    #[test]
    fn test_working_memory_capacity() {
        let mut wm = WorkingMemory::new(5);
        
        for i in 0..10 {
            let item = WorkingMemoryItem {
                id: Uuid::new_v4(),
                item_type: WorkingItemType::Observation,
                content: format!("Gözlem {}", i),
                metadata: serde_json::json!({}),
                created_at: Utc::now(),
                importance: 0.5,
            };
            wm.add(item);
        }
        
        assert!(wm.items.len() <= wm.capacity);
    }
    
    #[test]
    fn test_working_memory_by_type() {
        let mut wm = WorkingMemory::new(10);
        
        wm.add(WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::Decision,
            content: "Karar 1".into(),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            importance: 0.5,
        });
        
        wm.add(WorkingMemoryItem {
            id: Uuid::new_v4(),
            item_type: WorkingItemType::Observation,
            content: "Gözlem 1".into(),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            importance: 0.5,
        });
        
        let decisions = wm.by_type(WorkingItemType::Decision);
        assert_eq!(decisions.len(), 1);
        
        let observations = wm.by_type(WorkingItemType::Observation);
        assert_eq!(observations.len(), 1);
    }
    
    #[test]
    fn test_experience_creation() {
        let exp = Experience::new(
            "Test hedefi",
            vec!["Adım 1".into(), "Adım 2".into()],
            "Başarılı sonuç",
            true
        );
        
        assert!(exp.success);
        assert_eq!(exp.actions.len(), 2);
    }
    
    #[test]
    fn test_experience_with_lesson() {
        let exp = Experience::new(
            "Test hedefi",
            vec!["Adım 1".into()],
            "Başarısız",
            false
        ).with_lesson("Bu şekilde yapma");
        
        assert!(exp.lesson.is_some());
        assert_eq!(exp.importance, 0.9);
    }
    
    #[test]
    fn test_procedure_creation() {
        let proc = Procedure::new(
            "Test prosedürü",
            vec!["Adım 1".into(), "Adım 2".into()]
        ).with_description("Bu bir test prosedürüdür");
        
        assert_eq!(proc.steps.len(), 2);
        assert!(!proc.description.is_empty());
    }
    
    #[test]
    fn test_react_context_summary() {
        let ctx = ReActContext {
            goal_summary: "Test hedefi".into(),
            keywords: vec!["test".into()],
            episodic_memories: vec![],
            semantic_memories: vec![],
            procedural_memories: vec![],
            working_memory: WorkingMemoryState {
                recent_items: vec!["Son öğe".into()],
                decisions: vec![],
                observations: vec![],
                failed_tasks: vec![],
                scratch_pad: String::new(),
            },
            iteration: 1,
            completed_tasks: 0,
            failed_tasks: 0,
            retrieved_at: Utc::now(),
            retrieval_duration_ms: 10,
        };
        
        let summary = ctx.summary();
        assert!(summary.contains("ReActContext"));
    }
    
    #[test]
    fn test_consolidation_result() {
        let result = ConsolidationResult {
            transferred: 5,
            failed: 1,
            duration_ms: 100,
        };
        
        assert_eq!(result.transferred, 5);
        assert_eq!(result.failed, 1);
    }
}
