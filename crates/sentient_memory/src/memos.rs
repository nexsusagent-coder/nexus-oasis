//! ─── MEMOS - BELLEK İŞLETIM SISTEMI ───
//!
//! Kullanıcı verisini izole bellek küplerinde (Multi-Cube) tutan,
//! FTS5 tam metin araması ile vektör aramasını birleştiren ve
//! MemScheduler ile asenkron görev kuyruğu oluşturan MemOS mimarisi.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                         MEMOS                                   │
//! ├─────────────────────────────────────────────────────────────────┤
//! │                                                                 │
//! │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐              │
//! │  │  CUBE A     │ │  CUBE B     │ │  CUBE C     │              │
//! │  │  (User 1)   │ │  (User 2)   │ │  (Agent)    │              │
//! │  └──────┬──────┘ └──────┬──────┘ └──────┬──────┘              │
//! │         │               │               │                     │
//! │         ▼               ▼               ▼                     │
//! │  ┌─────────────────────────────────────────────────┐         │
//! │  │              HYBRID SEARCH ENGINE                │         │
//! │  │  ┌──────────┐    ┌──────────────┐               │         │
//! │  │  │   FTS5   │ +  │   VECTOR     │ = HYBRID     │         │
//! │  │  │  Search  │    │   Search     │               │         │
//! │  │  └──────────┘    └──────────────┘               │         │
//! │  └─────────────────────────────────────────────────┘         │
//! │                          │                                    │
//! │                          ▼                                    │
//! │  ┌─────────────────────────────────────────────────┐         │
//! │  │              MEMSCHEDULER                       │         │
//! │  │  ┌──────────────────────────────────────────┐  │         │
//! │  │  │  Priority Queue: Critical|High|Normal|Low│  │         │
//! │  │  └──────────────────────────────────────────┘  │         │
//! │  │  Tasks: Store | Index | Consolidate | Decay   │         │
//! │  └─────────────────────────────────────────────────┘         │
//! │                                                                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{
    MemoryEntry, MemoryInput, MemoryType, Importance,
    SearchResult, SearchType, RagContext, MemoryStats,
};
use super::cube::MemoryCube;
use super::scheduler::{MemScheduler, SchedulerConfig, TaskPriority};
use super::fts::{HybridWeights, HybridResult};

// ─────────────────────────────────────────────────────────────────────────────
// CUBE YÖNETİMİ
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek küpü meta verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeMeta {
    /// Küp ID
    pub id: Uuid,
    /// Sahip (kullanıcı/ajan)
    pub owner: String,
    /// Küp tipi
    pub cube_type: CubeType,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son erişim
    pub last_accessed: DateTime<Utc>,
    /// Bellek sayısı
    pub memory_count: u64,
    /// Toplam boyut (bytes)
    pub size_bytes: u64,
    /// Etiketler
    pub tags: Vec<String>,
    /// Aktif mi?
    pub is_active: bool,
}

/// Küp tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CubeType {
    /// Kullanıcıya özel
    User,
    /// Ajan belleği
    Agent,
    /// Paylaşılan bellek
    Shared,
    /// Sistem belleği
    System,
    /// Geçici bellek (oturum)
    Session,
}

impl std::fmt::Display for CubeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "Kullanıcı"),
            Self::Agent => write!(f, "Ajan"),
            Self::Shared => write!(f, "Paylaşılan"),
            Self::System => write!(f, "Sistem"),
            Self::Session => write!(f, "Oturum"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMOS YAPIŞANDIRMA
// ─────────────────────────────────────────────────────────────────────────────

/// MemOS yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemOSConfig {
    /// Veri dizini
    pub data_dir: PathBuf,
    /// Maksimum küp sayısı
    pub max_cubes: usize,
    /// Maksimum bellek boyutu (MB)
    pub max_memory_mb: usize,
    /// Zamanlayıcı yapılandırması
    pub scheduler: SchedulerConfig,
    /// Otomatik konsolidasyon (saniye)
    pub auto_consolidation_secs: u64,
    /// Otomatik decay (saniye)
    pub auto_decay_secs: u64,
}

impl Default for MemOSConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data/memory"),
            max_cubes: 100,
            max_memory_mb: 1024,
            scheduler: SchedulerConfig::default(),
            auto_consolidation_secs: 3600,
            auto_decay_secs: 7200,
        }
    }
}

/// MemOS istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemOSStats {
    pub total_cubes: usize,
    pub active_cubes: usize,
    pub total_memories: u64,
    pub total_vectors: u64,
    pub storage_used_mb: f64,
    pub pending_tasks: usize,
    pub last_consolidation: Option<DateTime<Utc>>,
    pub last_decay: Option<DateTime<Utc>>,
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMOS ANA YAPI
// ─────────────────────────────────────────────────────────────────────────────

/// MemOS - Bellek İşletim Sistemi
pub struct MemOS {
    /// Yapılandırma
    config: MemOSConfig,
    /// Aktif küpler
    cubes: Arc<RwLock<HashMap<Uuid, Arc<tokio::sync::Mutex<MemoryCube>>>>>,
    /// Küp meta verileri
    cube_metas: Arc<RwLock<HashMap<Uuid, CubeMeta>>>,
    /// Küp -> Owner eşlemesi
    owner_cubes: Arc<RwLock<HashMap<String, Vec<Uuid>>>>,
    /// Zamanlayıcı
    scheduler: Arc<MemScheduler>,
    /// Ana veritabanı
    db: Arc<RwLock<Connection>>,
}

impl MemOS {
    /// Yeni MemOS oluştur
    pub async fn new(config: MemOSConfig) -> Result<Self, String> {
        // Dizin oluştur
        tokio::fs::create_dir_all(&config.data_dir).await
            .map_err(|e| format!("Dizin oluşturulamadı: {}", e))?;
        
        // Ana veritabanını aç
        let db_path = config.data_dir.join("memos.db");
        let conn = Connection::open(db_path)
            .map_err(|e| format!("Veritabanı açılamadı: {}", e))?;
        
        // Tabloları oluştur
        Self::create_tables(&conn)?;
        
        let scheduler = MemScheduler::new(config.scheduler.clone());
        scheduler.start().await;
        
        let memos = Self {
            config,
            cubes: Arc::new(RwLock::new(HashMap::new())),
            cube_metas: Arc::new(RwLock::new(HashMap::new())),
            owner_cubes: Arc::new(RwLock::new(HashMap::new())),
            scheduler: Arc::new(scheduler),
            db: Arc::new(RwLock::new(conn)),
        };
        
        // Mevcut küpleri yükle
        memos.load_existing_cubes().await
            .map_err(|e| format!("Küpler yüklenemedi: {}", e))?;
        
        log::info!("🧠 MemOS başlatıldı");
        Ok(memos)
    }
    
    /// Varsayılan yapılandırma ile oluştur
    pub async fn default_memos() -> Result<Self, String> {
        Self::new(MemOSConfig::default()).await
    }
    
    /// Tabloları oluştur
    fn create_tables(conn: &Connection) -> Result<(), String> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS cubes (
                id TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                cube_type TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_accessed TEXT NOT NULL,
                memory_count INTEGER DEFAULT 0,
                size_bytes INTEGER DEFAULT 0,
                tags TEXT,
                is_active INTEGER DEFAULT 1
            );
            
            CREATE TABLE IF NOT EXISTS cube_registry (
                owner TEXT NOT NULL,
                cube_id TEXT NOT NULL,
                PRIMARY KEY (owner, cube_id)
            );
            
            CREATE INDEX IF NOT EXISTS idx_cubes_owner ON cubes(owner);
            CREATE INDEX IF NOT EXISTS idx_cubes_type ON cubes(cube_type);
            "#
        ).map_err(|e| format!("Tablo oluşturulamadı: {}", e))?;
        Ok(())
    }
    
    /// Mevcut küpleri yükle
    async fn load_existing_cubes(&self) -> Result<(), String> {
        let conn = self.db.read().await;
        
        let mut stmt = conn.prepare(
            "SELECT id, owner, cube_type, created_at, last_accessed, memory_count, size_bytes, tags, is_active FROM cubes WHERE is_active = 1"
        ).map_err(|e| format!("Sorgu hazırlanamadı: {}", e))?;
        
        let cube_rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, String>(7).unwrap_or_default(),
                row.get::<_, i32>(8)?
            ))
        }).map_err(|e| format!("Sorgu çalıştırılamadı: {}", e))?
        .filter_map(|r| r.ok()).collect::<Vec<_>>();
        
        drop(stmt);
        drop(conn);
        
        for (id_str, owner, type_str, created_str, accessed_str, memory_count, size_bytes, tags_str, _is_active) in cube_rows {
            let id = Uuid::parse_str(&id_str).unwrap_or(Uuid::nil());
            let cube_type = match type_str.as_str() {
                "User" => CubeType::User,
                "Agent" => CubeType::Agent,
                "Shared" => CubeType::Shared,
                "System" => CubeType::System,
                "Session" => CubeType::Session,
                _ => CubeType::User,
            };
            
            let tags: Vec<String> = if tags_str.is_empty() {
                vec![]
            } else {
                tags_str.split(',').map(|s| s.to_string()).collect()
            };
            
            let meta = CubeMeta {
                id,
                owner: owner.clone(),
                cube_type,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or(Utc::now()),
                last_accessed: chrono::DateTime::parse_from_rfc3339(&accessed_str)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or(Utc::now()),
                memory_count: memory_count as u64,
                size_bytes: size_bytes as u64,
                tags,
                is_active: true,
            };
            
            self.cube_metas.write().await.insert(id, meta);
            
            let mut owner_cubes = self.owner_cubes.write().await;
            owner_cubes.entry(owner).or_default().push(id);
        }
        
        log::info!("📦 {} aktif küp yüklendi", self.cube_metas.read().await.len());
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // KÜP YÖNETİMİ
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Yeni küp oluştur
    pub async fn create_cube(&self, owner: &str, cube_type: CubeType) -> Result<Uuid, String> {
        let cube_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Küp veritabanı yolu
        let cube_path = self.config.data_dir.join(format!("cube_{}.db", cube_id.simple()));
        
        // MemoryCube oluştur (senkron işlemi async'e al)
        let cube_path_str = cube_path.to_string_lossy().to_string();
        let cube = tokio::task::spawn_blocking(move || {
            MemoryCube::new(&cube_path_str)
                .map_err(|e| format!("Küp oluşturulamadı: {}", e))
        }).await
            .map_err(|e| format!("Task hatası: {}", e))??;
        
        // Meta veri
        let meta = CubeMeta {
            id: cube_id,
            owner: owner.to_string(),
            cube_type,
            created_at: now,
            last_accessed: now,
            memory_count: 0,
            size_bytes: 0,
            tags: vec![],
            is_active: true,
        };
        
        // Veritabanına kaydet
        let conn = self.db.write().await;
        conn.execute(
            "INSERT INTO cubes (id, owner, cube_type, created_at, last_accessed, memory_count, size_bytes, tags, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, '', 1)",
            params![
                cube_id.to_string(),
                owner,
                format!("{:?}", cube_type),
                now.to_rfc3339(),
                now.to_rfc3339(),
                0i64,
                0i64,
            ]
        ).map_err(|e| format!("Veritabanı hatası: {}", e))?;
        
        conn.execute(
            "INSERT INTO cube_registry (owner, cube_id) VALUES (?, ?)",
            params![owner, cube_id.to_string()]
        ).map_err(|e| format!("Registry hatası: {}", e))?;
        
        // Belleğe ekle
        drop(conn);
        self.cubes.write().await.insert(cube_id, Arc::new(tokio::sync::Mutex::new(cube)));
        self.cube_metas.write().await.insert(cube_id, meta);
        self.owner_cubes.write().await.entry(owner.to_string()).or_default().push(cube_id);
        
        log::info!("📦 Yeni küp oluşturuldu: {} → {} ({})", cube_id, owner, cube_type);
        Ok(cube_id)
    }
    
    /// Kullanıcı için küp al veya oluştur
    pub async fn get_or_create_user_cube(&self, user_id: &str) -> Result<Uuid, String> {
        let owner_cubes = self.owner_cubes.read().await;
        
        if let Some(cube_ids) = owner_cubes.get(user_id) {
            if let Some(&cube_id) = cube_ids.first() {
                return Ok(cube_id);
            }
        }
        
        drop(owner_cubes);
        self.create_cube(user_id, CubeType::User).await
    }
    
    /// Küpü getir
    pub async fn get_cube(&self, cube_id: Uuid) -> Option<Arc<tokio::sync::Mutex<MemoryCube>>> {
        let cubes = self.cubes.read().await;
        
        if let Some(cube) = cubes.get(&cube_id) {
            let cube_clone = cube.clone();
            drop(cubes);
            self.touch_cube(cube_id).await;
            return Some(cube_clone);
        }
        
        None
    }
    
    /// Küpe erişildi işaretle
    async fn touch_cube(&self, cube_id: Uuid) {
        let now = Utc::now();
        if let Some(meta) = self.cube_metas.write().await.get_mut(&cube_id) {
            meta.last_accessed = now;
        }
        
        let _ = self.db.write().await.execute(
            "UPDATE cubes SET last_accessed = ? WHERE id = ?",
            params![now.to_rfc3339(), cube_id.to_string()]
        );
    }
    
    /// Kullanıcının tüm küplerini getir
    pub async fn get_user_cubes(&self, user_id: &str) -> Vec<CubeMeta> {
        let owner_cubes = self.owner_cubes.read().await;
        let cube_metas = self.cube_metas.read().await;
        
        owner_cubes
            .get(user_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| cube_metas.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Küpü sil
    pub async fn delete_cube(&self, cube_id: Uuid) -> Result<(), String> {
        let conn = self.db.write().await;
        
        conn.execute(
            "UPDATE cubes SET is_active = 0 WHERE id = ?",
            params![cube_id.to_string()]
        ).map_err(|e| format!("Silme hatası: {}", e))?;
        
        self.cubes.write().await.remove(&cube_id);
        self.cube_metas.write().await.remove(&cube_id);
        
        // Owner listesinden kaldır
        let mut owner_cubes = self.owner_cubes.write().await;
        for cubes in owner_cubes.values_mut() {
            cubes.retain(|id| *id != cube_id);
        }
        
        log::info!("🗑️ Küp silindi: {}", cube_id);
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // BELLEK İŞLEMLERİ
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Belleğe kaydet (asenkron)
    pub async fn memorize(&self, cube_id: Uuid, input: MemoryInput) -> Result<Uuid, String> {
        let cubes = self.cubes.read().await;
        
        if let Some(cube_arc) = cubes.get(&cube_id) {
            let entry = MemoryEntry::from_input(input);
            let memory_id = entry.id;
            
            // Zamanlayıcıya görev ekle
            self.scheduler.schedule_store(cube_id, entry.clone()).await;
            
            // Küpü kilitle ve kaydet
            {
                let mut cube = cube_arc.lock().await;
                let _ = cube.store(entry);
            }
            
            drop(cubes);
            self.touch_cube(cube_id).await;
            
            return Ok(memory_id);
        }
        
        Err("Küp bulunamadı".into())
    }
    
    /// Hatırla - Vektör araması
    pub async fn recall(&self, cube_id: Uuid, _query: &str, limit: usize) -> Result<Vec<SearchResult>, String> {
        let cubes = self.cubes.read().await;
        
        if let Some(cube_arc) = cubes.get(&cube_id) {
            let cube = cube_arc.lock().await;
            let results = cube.search_vector(&[], limit)
                .map_err(|e| format!("Arama hatası: {}", e))?;
            
            drop(cube);
            drop(cubes);
            self.touch_cube(cube_id).await;
            
            return Ok(results);
        }
        
        Err("Küp bulunamadı".into())
    }
    
    /// RAG Context hazırla
    pub async fn prepare_rag_context(
        &self,
        cube_id: Uuid,
        query: &str,
        _max_tokens: usize,
    ) -> Result<Option<RagContext>, String> {
        let cubes = self.cubes.read().await;
        
        if let Some(cube_arc) = cubes.get(&cube_id) {
            let cube = cube_arc.lock().await;
            
            // Search for memories
            let memories = cube.search(query, None)
                .map_err(|e| format!("Arama hatası: {}", e))?;
            
            if memories.is_empty() {
                return Ok(None);
            }
            
            // Build RAG context
            let search_results: Vec<SearchResult> = memories.into_iter().map(|m| SearchResult {
                memory: m,
                similarity: 0.5, // Default similarity
                search_type: SearchType::Hybrid,
            }).take(10).collect();
            
            let context_text = search_results.iter()
                .map(|r| r.memory.content.clone())
                .collect::<Vec<_>>()
                .join("\n\n");
            
            let context = RagContext {
                query: query.to_string(),
                retrieved_memories: search_results,
                estimated_tokens: context_text.len() / 4, // Rough estimate
                context_text,
                source_types: vec![MemoryType::Semantic],
            };
            
            drop(cube);
            drop(cubes);
            self.touch_cube(cube_id).await;
            
            return Ok(Some(context));
        }
        
        Ok(None)
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // İSTATİSTİKLER
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Sistem istatistikleri
    pub async fn stats(&self) -> MemOSStats {
        let cubes = self.cubes.read().await;
        let cube_metas = self.cube_metas.read().await;
        let scheduler_stats = self.scheduler.stats().await;
        
        let total_memories: u64 = cube_metas.values().map(|m| m.memory_count).sum();
        
        MemOSStats {
            total_cubes: cube_metas.len(),
            active_cubes: cubes.len(),
            total_memories,
            total_vectors: 0, // Would need to query each cube
            storage_used_mb: cube_metas.values().map(|m| m.size_bytes as f64).sum::<f64>() / 1024.0 / 1024.0,
            pending_tasks: scheduler_stats.pending_tasks,
            last_consolidation: None,
            last_decay: None,
        }
    }
    
    /// Zamanlayıcıyı durdur
    pub async fn shutdown(&self) {
        self.scheduler.stop().await;
        log::info!("🛑 MemOS kapatıldı");
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // MEMORY BRIDGE İÇİN EK METODLAR
    // ═══════════════════════════════════════════════════════════════════════
    
    /// Tüm aktif küpleri listele
    pub async fn list_cubes(&self) -> Result<Vec<CubeMeta>, String> {
        let cube_metas = self.cube_metas.read().await;
        Ok(cube_metas.values().cloned().collect())
    }
    
    /// Bellek kaydet (store_memory alias)
    pub async fn store_memory(&self, cube_id: Uuid, input: MemoryInput) -> Result<Uuid, String> {
        self.memorize(cube_id, input).await
    }
    
    /// Hibrit arama (FTS + Vector)
    pub async fn hybrid_search(
        &self,
        cube_id: Uuid,
        query: &str,
        weights: HybridWeights,
        limit: usize,
    ) -> Result<Vec<HybridResult>, String> {
        let cubes = self.cubes.read().await;
        
        if let Some(cube_arc) = cubes.get(&cube_id) {
            let cube = cube_arc.lock().await;
            
            // Vektör araması
            let vector_results = cube.search(query, None)
                .map_err(|e| format!("Vektör arama hatası: {}", e))?;
            
            let search_results: Vec<SearchResult> = vector_results.into_iter().map(|m| SearchResult {
                memory: m,
                similarity: 0.5,
                search_type: SearchType::Hybrid,
            }).take(limit * 2).collect();
            
            // FTS için boş liste (şimdilik vektör ağırlıklı)
            let fts_results = Vec::new();
            
            // Hibrit motor oluştur
            let fts = Arc::new(crate::fts::FtsEngine::new(
                Arc::new(tokio::sync::RwLock::new(rusqlite::Connection::open_in_memory().map_err(|e| e.to_string())?)),
                cube_id,
            ));
            let hybrid_engine = crate::fts::HybridSearchEngine::new(fts, weights);
            
            let results = hybrid_engine.combine_results(fts_results, search_results, limit).await;
            
            drop(cube);
            drop(cubes);
            self.touch_cube(cube_id).await;
            
            return Ok(results);
        }
        
        Err("Küp bulunamadı".into())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    
    fn unique_config() -> MemOSConfig {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let uuid = uuid::Uuid::new_v4();
        MemOSConfig {
            data_dir: PathBuf::from(format!("/tmp/memos_test_{}_{}", uuid, id)),
            ..Default::default()
        }
    }
    
    #[tokio::test]
    async fn test_memos_creation() {
        let memos = MemOS::new(unique_config()).await.unwrap();
        let stats = memos.stats().await;
        
        assert_eq!(stats.total_cubes, 0);
        assert_eq!(stats.active_cubes, 0);
    }
    
    #[tokio::test]
    async fn test_create_cube() {
        let memos = MemOS::new(unique_config()).await.unwrap();
        let cube_id = memos.create_cube("test_user", CubeType::User).await.unwrap();
        
        assert!(!cube_id.is_nil());
        
        let stats = memos.stats().await;
        assert_eq!(stats.total_cubes, 1);
        assert_eq!(stats.active_cubes, 1);
    }
    
    #[tokio::test]
    async fn test_get_or_create_user_cube() {
        let memos = MemOS::new(unique_config()).await.unwrap();
        
        let cube1 = memos.get_or_create_user_cube("user1").await.unwrap();
        let cube2 = memos.get_or_create_user_cube("user1").await.unwrap();
        
        assert_eq!(cube1, cube2);
        
        let cube3 = memos.get_or_create_user_cube("user2").await.unwrap();
        assert_ne!(cube1, cube3);
    }
    
    #[tokio::test]
    async fn test_memorize() {
        let memos = MemOS::new(unique_config()).await.unwrap();
        let cube_id = memos.create_cube("test_user", CubeType::User).await.unwrap();
        
        let input = MemoryInput::new("Test bellek içeriği")
            .with_type(MemoryType::Semantic)
            .with_importance(Importance::high());
        
        let memory_id = memos.memorize(cube_id, input).await.unwrap();
        
        assert!(!memory_id.is_nil());
        
        let stats = memos.scheduler.stats().await;
        assert!(stats.total_tasks >= 1);
    }
}
