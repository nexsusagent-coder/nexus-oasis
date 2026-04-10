//! ─── MEMORY CUBE (BİLGİ KÜPÜ) ───
//!
//! Ana bellek yönetim sistemi:
//! - SQLite kalıcılığı
//! - Vektör indeksleme
//! - Bilgi grafiği entegrasyonu
//! - Bellek konsolidasyonu

use crate::{
    MemoryType, MemoryEntry, MemoryInput, MemorySource,
    Importance, SearchResult, SearchType,
    MemoryError, MemoryResult, MemoryStats, RelationType,
};
use crate::embeddings::{EmbeddingEngine, EmbeddingConfig};
use crate::vector_index::{VectorIndex, InMemoryVectorIndex};
use crate::knowledge_graph::KnowledgeGraph;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY CUBE CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek küpü yapılandırması
#[derive(Debug, Clone)]
pub struct MemoryCubeConfig {
    /// Veritabanı yolu
    pub db_path: String,
    /// Vektör indeks yolu
    pub vector_db_path: String,
    /// Graf veritabanı yolu
    pub graph_db_path: String,
    /// Vektör boyutu
    pub embedding_dimension: usize,
    /// Otomatik konsolidasyon
    pub auto_consolidation: bool,
    /// Konsolidasyon aralığı (saniye)
    pub consolidation_interval: u64,
}

impl Default for MemoryCubeConfig {
    fn default() -> Self {
        Self {
            db_path: "data/sentient_memory.db".into(),
            vector_db_path: "data/sentient_vectors.db".into(),
            graph_db_path: "data/sentient_graph.db".into(),
            embedding_dimension: 1536,
            auto_consolidation: true,
            consolidation_interval: 3600,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY CUBE
// ─────────────────────────────────────────────────────────────────────────────

/// Ana Bellek Küpü
pub struct MemoryCube {
    /// SQLite bağlantısı
    conn: Connection,
    /// Vektör indeksi
    vector_index: Option<VectorIndex>,
    /// Bellek içi vektör önbelleği
    vector_cache: InMemoryVectorIndex,
    /// Bilgi grafiği
    knowledge_graph: Option<KnowledgeGraph>,
    /// Yapılandırma
    config: MemoryCubeConfig,
}

// SQL select template
const MEMORY_SELECT: &str = "
    SELECT id, content, entry_type, source, metadata, importance,
           access_count, tags, created_at, last_accessed, updated_at,
           ttl_seconds, last_validated, confidence
    FROM memories
";

impl MemoryCube {
    /// Yeni Bellek Küpü oluştur
    pub fn new(db_path: &str) -> MemoryResult<Self> {
        Self::with_config(MemoryCubeConfig {
            db_path: db_path.into(),
            ..Default::default()
        })
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: MemoryCubeConfig) -> MemoryResult<Self> {
        // Dizin oluştur
        if let Some(parent) = Path::new(&config.db_path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    MemoryError::DatabaseError(format!("Dizin oluşturulamadı: {}", e))
                })?;
            }
        }
        
        // SQLite bağlantısı
        let conn = Connection::open(&config.db_path).map_err(|e| {
            MemoryError::DatabaseError(format!("SQLite bağlantısı kurulamadı: {}", e))
        })?;
        
        let mut cube = Self {
            conn,
            vector_index: None,
            vector_cache: InMemoryVectorIndex::new(config.embedding_dimension),
            knowledge_graph: None,
            config,
        };
        
        cube.initialize_schema()?;
        
        // Vektör indeksi başlat
        if let Ok(index) = VectorIndex::new(&cube.config.vector_db_path, cube.config.embedding_dimension) {
            cube.vector_index = Some(index);
        }
        
        // Bilgi grafiği başlat
        if let Ok(graph) = KnowledgeGraph::new(&cube.config.graph_db_path) {
            cube.knowledge_graph = Some(graph);
        }
        
        log::info!("🧠  BELLEK: Küp başlatıldı: {}", cube.config.db_path);
        Ok(cube)
    }
    
    /// Şema oluştur
    fn initialize_schema(&mut self) -> MemoryResult<()> {
        self.conn.execute_batch(
            "
            -- Ana bellek tablosu
            CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                entry_type TEXT NOT NULL,
                source TEXT,
                metadata TEXT DEFAULT '{}',
                importance REAL DEFAULT 0.5,
                access_count INTEGER DEFAULT 0,
                tags TEXT DEFAULT '[]',
                created_at TEXT NOT NULL,
                last_accessed TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                ttl_seconds INTEGER,
                expires_at TEXT,
                last_validated TEXT,
                confidence REAL DEFAULT 1.0
            );
            
            -- İndeksler
            CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(entry_type);
            CREATE INDEX IF NOT EXISTS idx_memories_created ON memories(created_at);
            CREATE INDEX IF NOT EXISTS idx_memories_importance ON memories(importance);
            CREATE INDEX IF NOT EXISTS idx_memories_expires ON memories(expires_at);
            
            -- İlişki tablosu
            CREATE TABLE IF NOT EXISTS memory_relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                created_at TEXT NOT NULL,
                UNIQUE(source_id, target_id, relation_type)
            );
            
            CREATE INDEX IF NOT EXISTS idx_relations_source ON memory_relations(source_id);
            CREATE INDEX IF NOT EXISTS idx_relations_target ON memory_relations(target_id);
            "
        ).map_err(|e| MemoryError::DatabaseError(format!("Şema hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// Bellek kaydet
    pub fn store(&mut self, entry: MemoryEntry) -> MemoryResult<Uuid> {
        let id = entry.id;
        let now = Utc::now();
        
        let expires_at = entry.ttl_seconds.map(|ttl| {
            (now + chrono::Duration::seconds(ttl)).to_rfc3339()
        });
        
        self.conn.execute(
            "INSERT INTO memories (
                id, content, entry_type, source, metadata, importance,
                access_count, tags, created_at, last_accessed, updated_at,
                ttl_seconds, expires_at, last_validated, confidence
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                id.to_string(),
                entry.content,
                serde_json::to_string(&entry.memory_type).unwrap_or_default(),
                serde_json::to_string(&entry.source).unwrap_or_default(),
                serde_json::to_string(&entry.metadata).unwrap_or("{}".into()),
                entry.importance.value(),
                entry.access_count as i32,
                serde_json::to_string(&entry.tags).unwrap_or("[]".into()),
                entry.created_at.to_rfc3339(),
                entry.last_accessed.to_rfc3339(),
                entry.updated_at.to_rfc3339(),
                entry.ttl_seconds,
                expires_at,
                entry.last_validated.map(|d| d.to_rfc3339()),
                entry.confidence,
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("Kaydetme hatası: {}", e)))?;
        
        // Vektör varsa ekle
        if let Some(ref embedding) = entry.embedding {
            if let Some(ref index) = self.vector_index {
                index.insert(id, embedding.clone())?;
            }
            self.vector_cache.insert(id, embedding.clone())?;
        }
        
        log::debug!("📝  Kaydedildi: {} [{}]", id, entry.memory_type);
        Ok(id)
    }
    
    /// Yeni bellek oluştur
    pub fn create(
        &mut self,
        content: String,
        memory_type: MemoryType,
        metadata: Option<serde_json::Value>,
        ttl_seconds: Option<i64>,
    ) -> MemoryResult<Uuid> {
        let input = MemoryInput::new(&content)
            .with_type(memory_type);
        
        let entry = MemoryEntry::from_input(input);
        self.store(entry)
    }
    
    /// Embedding ile oluştur
    pub fn create_with_embedding(
        &mut self,
        input: MemoryInput,
        embedding: Option<Vec<f32>>,
    ) -> MemoryResult<Uuid> {
        let mut entry = MemoryEntry::from_input(input);
        entry.embedding = embedding;
        self.store(entry)
    }
    
    /// Bellek getir
    pub fn recall(&self, id: Uuid) -> MemoryResult<Option<MemoryEntry>> {
        let mut stmt = self.conn.prepare(&format!("{} WHERE id = ?1", MEMORY_SELECT))
            .map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let result = stmt.query_row(
            params![id.to_string()],
            Self::row_to_entry
        );
        
        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(MemoryError::DatabaseError(format!("Okuma hatası: {}", e))),
        }
    }
    
    /// Metin ara
    pub fn search(&self, query: &str, memory_type: Option<MemoryType>) -> MemoryResult<Vec<MemoryEntry>> {
        let sql = match memory_type {
            Some(t) => format!("{} WHERE content LIKE ?1 AND entry_type = ?2 ORDER BY importance DESC, created_at DESC", MEMORY_SELECT),
            None => format!("{} WHERE content LIKE ?1 ORDER BY importance DESC, created_at DESC", MEMORY_SELECT),
        };
        
        let mut stmt = self.conn.prepare(&sql)
            .map_err(|e| MemoryError::DatabaseError(format!("Arama sorgusu hatası: {}", e)))?;
        
        let pattern = format!("%{}%", query);
        
        let rows = match memory_type {
            Some(t) => stmt.query_map(
                params![pattern, serde_json::to_string(&t).unwrap_or_default()],
                Self::row_to_entry
            ).map_err(|e| MemoryError::DatabaseError(format!("Arama hatası: {}", e)))?,
            None => stmt.query_map(
                params![pattern],
                Self::row_to_entry
            ).map_err(|e| MemoryError::DatabaseError(format!("Arama hatası: {}", e)))?,
        };
        
        let entries: Vec<_> = rows
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(entries)
    }
    
    /// Vektör ara
    pub fn search_vector(&self, query: &[f32], limit: usize) -> MemoryResult<Vec<SearchResult>> {
        // Önce bellek içi önbellekte ara
        let candidates = self.vector_cache.search(query, limit * 2, 0.3)?;
        
        let mut results = Vec::new();
        for (id, similarity) in candidates {
            if let Some(entry) = self.recall(id)? {
                results.push(SearchResult {
                    memory: entry,
                    similarity,
                    search_type: SearchType::VectorSimilarity,
                });
            }
        }
        
        results.truncate(limit);
        Ok(results)
    }
    
    /// Benzer içerik bul
    pub fn find_similar(&self, content: &str, threshold: f32) -> MemoryResult<Vec<MemoryEntry>> {
        // Basit metin benzerliği
        let all = self.list_all()?;
        let similar: Vec<_> = all
            .into_iter()
            .filter(|e| {
                let similarity = self.text_similarity(content, &e.content);
                similarity >= threshold
            })
            .collect();
        
        Ok(similar)
    }
    
    /// Tip ile getir
    pub fn get_by_type(&self, memory_type: MemoryType) -> MemoryResult<Vec<MemoryEntry>> {
        let sql = format!("{} WHERE entry_type = ?1 ORDER BY created_at DESC", MEMORY_SELECT);
        
        let mut stmt = self.conn.prepare(&sql)
            .map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let entries = stmt.query_map(
            params![serde_json::to_string(&memory_type).unwrap_or_default()],
            Self::row_to_entry
        ).map_err(|e| MemoryError::DatabaseError(format!("Okuma hatası: {}", e)))?
        .filter_map(|r| r.ok())
        .collect();
        
        Ok(entries)
    }
    
    /// Bellek sil
    pub fn delete(&mut self, id: Uuid) -> MemoryResult<bool> {
        let affected = self.conn.execute(
            "DELETE FROM memories WHERE id = ?1",
            params![id.to_string()]
        ).map_err(|e| MemoryError::DatabaseError(format!("Silme hatası: {}", e)))?;
        
        if affected > 0 {
            if let Some(ref index) = self.vector_index {
                index.delete(id)?;
            }
            log::debug!("🗑️  Silindi: {}", id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Bellek güncelle
    pub fn update(&mut self, entry: MemoryEntry) -> MemoryResult<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE memories SET content = ?1, updated_at = ?2, importance = ?3, metadata = ?4
             WHERE id = ?5",
            params![
                entry.content,
                now.to_rfc3339(),
                entry.importance.value(),
                serde_json::to_string(&entry.metadata).unwrap_or("{}".into()),
                entry.id.to_string(),
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("Güncelleme hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// Önem güncelle
    pub fn update_importance(&mut self, id: Uuid, importance: Importance) -> MemoryResult<()> {
        self.conn.execute(
            "UPDATE memories SET importance = ?1, updated_at = ?2 WHERE id = ?3",
            params![importance.value(), Utc::now().to_rfc3339(), id.to_string()]
        ).map_err(|e| MemoryError::DatabaseError(format!("Önem güncelleme hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// Güçlendir
    pub fn reinforce_memory(&mut self, id: Uuid, delta: f32) -> MemoryResult<()> {
        if let Some(mut entry) = self.recall(id)? {
            entry.reinforce(delta);
            self.update(entry)?;
        }
        Ok(())
    }
    
    /// Süresi dolanları temizle
    pub fn cleanup_expired(&mut self) -> MemoryResult<usize> {
        let now = Utc::now().to_rfc3339();
        let affected = self.conn.execute(
            "DELETE FROM memories WHERE expires_at IS NOT NULL AND expires_at < ?1",
            params![now]
        ).map_err(|e| MemoryError::DatabaseError(format!("Temizleme hatası: {}", e)))?;
        
        if affected > 0 {
            log::info!("🧹  {} süresi dolmuş bellek temizlendi", affected);
        }
        Ok(affected)
    }
    
    /// Toplam sayı
    pub fn count(&self) -> MemoryResult<u64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM memories",
            [],
            |row| row.get(0)
        ).map_err(|e| MemoryError::DatabaseError(format!("Sayım hatası: {}", e)))?;
        
        Ok(count as u64)
    }
    
    /// Tümünü listele
    pub fn list_all(&self) -> MemoryResult<Vec<MemoryEntry>> {
        let sql = format!("{} ORDER BY created_at DESC", MEMORY_SELECT);
        let mut stmt = self.conn.prepare(&sql)
            .map_err(|e| MemoryError::DatabaseError(format!("Liste sorgusu hatası: {}", e)))?;
        
        let entries = stmt.query_map([], Self::row_to_entry)
            .map_err(|e| MemoryError::DatabaseError(format!("Liste hatası: {}", e)))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(entries)
    }
    
    /// İstatistikler
    pub fn stats(&self) -> MemoryResult<MemoryStats> {
        let entries = self.list_all()?;
        
        let mut stats = MemoryStats::default();
        stats.total_memories = entries.len() as u64;
        
        for entry in &entries {
            *stats.by_type.entry(entry.memory_type).or_default() += 1;
            if entry.embedding.is_some() {
                stats.total_embeddings += 1;
            }
            if entry.is_expired() {
                stats.expired_count += 1;
            }
        }
        
        stats.avg_importance = if entries.is_empty() {
            0.0
        } else {
            entries.iter().map(|e| e.importance.value()).sum::<f32>() / entries.len() as f32
        };
        
        stats.avg_access_count = if entries.is_empty() {
            0.0
        } else {
            entries.iter().map(|e| e.access_count as f32).sum::<f32>() / entries.len() as f32
        };
        
        if let Some(first) = entries.iter().min_by_key(|e| e.created_at) {
            stats.oldest_memory = Some(first.created_at);
        }
        if let Some(last) = entries.iter().max_by_key(|e| e.created_at) {
            stats.newest_memory = Some(last.created_at);
        }
        
        Ok(stats)
    }
    
    /// İlişki ekle
    pub fn link(
        &mut self,
        source: Uuid,
        target: Uuid,
        relation: RelationType,
        weight: Option<f32>,
    ) -> MemoryResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memory_relations (source_id, target_id, relation_type, weight, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                source.to_string(),
                target.to_string(),
                serde_json::to_string(&relation).unwrap_or_default(),
                weight.unwrap_or(1.0),
                Utc::now().to_rfc3339(),
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("İlişki ekleme hatası: {}", e)))?;
        
        // Bilgi grafiğine de ekle
        if let Some(ref graph) = self.knowledge_graph {
            let _ = graph.add_edge(source, target, relation, weight);
        }
        
        log::debug!("🔗  İlişki: {} → {} ({})", source, target, relation);
        Ok(())
    }
    
    /// İlişkili bellekleri getir
    pub fn get_related(&self, id: Uuid) -> MemoryResult<Vec<MemoryEntry>> {
        let sql = "
            SELECT m.id, m.content, m.entry_type, m.source, m.metadata, m.importance,
                   m.access_count, m.tags, m.created_at, m.last_accessed, m.updated_at,
                   m.ttl_seconds, m.last_validated, m.confidence
            FROM memories m
            INNER JOIN memory_relations r ON m.id = r.target_id
            WHERE r.source_id = ?1
        ";
        
        let mut stmt = self.conn.prepare(sql)
            .map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let entries = stmt.query_map(params![id.to_string()], Self::row_to_entry)
            .map_err(|e| MemoryError::DatabaseError(format!("Okuma hatası: {}", e)))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(entries)
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // HELPERS
    // ─────────────────────────────────────────────────────────────────────────
    
    fn row_to_entry(row: &rusqlite::Row) -> Result<MemoryEntry, rusqlite::Error> {
        Ok(MemoryEntry {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap_or_default(),
            content: row.get(1)?,
            memory_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or(MemoryType::Semantic),
            source: serde_json::from_str(&row.get::<_, Option<String>>(3)?.unwrap_or_default()).unwrap_or(MemorySource::Internal),
            embedding: None, // Not stored in main table
            metadata: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
            importance: Importance::new(row.get(5)?),
            access_count: row.get::<_, i32>(6)? as u32,
            tags: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
            last_accessed: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                .map(|d| d.with_timezone(&Utc)).unwrap_or(Utc::now()),
            ttl_seconds: row.get(11)?,
            last_validated: row.get::<_, Option<String>>(12)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|d| d.with_timezone(&Utc)),
            confidence: row.get(13)?,
        })
    }
    
    fn text_similarity(&self, a: &str, b: &str) -> f32 {
        // Basit Jaccard benzerliği
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();
        let a_words: std::collections::HashSet<_> = a_lower.split_whitespace().collect();
        let b_words: std::collections::HashSet<_> = b_lower.split_whitespace().collect();
        
        if a_words.is_empty() || b_words.is_empty() {
            return 0.0;
        }
        
        let intersection = a_words.intersection(&b_words).count();
        let union = a_words.union(&b_words).count();
        
        intersection as f32 / union as f32
    }
    
    /// Bellek oluştur (backward compat)
    pub fn create_with_metadata(
        &mut self,
        content: String,
        memory_type: MemoryType,
        metadata: Option<serde_json::Value>,
        ttl_seconds: Option<i64>,
    ) -> MemoryResult<Uuid> {
        self.create(content, memory_type, metadata, ttl_seconds)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    fn test_db(name: &str) -> String {
        format!("/tmp/test_cube_{}.db", name)
    }
    
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(path.replace(".db", "_vectors.db"));
        let _ = fs::remove_file(path.replace(".db", "_graph.db"));
        // Default graph DB
        let _ = fs::remove_file("data/sentient_graph.db");
    }
    
    #[test]
    fn test_cube_creation() {
        let db = test_db("create");
        cleanup(&db);
        {
            let cube = MemoryCube::new(&db).expect("operation failed");
            assert_eq!(cube.count().expect("operation failed"), 0);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_store_and_recall() {
        let db = test_db("recall");
        cleanup(&db);
        {
            let mut cube = MemoryCube::new(&db).expect("operation failed");
            let id = cube.create(
                "SENTIENT bir AI OS'tur".into(),
                MemoryType::Semantic,
                None,
                None,
            ).expect("operation failed");
            
            let entry = cube.recall(id).expect("operation failed").expect("operation failed");
            assert_eq!(entry.content, "SENTIENT bir AI OS'tur");
            assert_eq!(entry.memory_type, MemoryType::Semantic);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_search() {
        let db = test_db("search");
        cleanup(&db);
        {
            let mut cube = MemoryCube::new(&db).expect("operation failed");
            cube.create("Rust güvenli bir dildir".into(), MemoryType::Semantic, None, None).expect("operation failed");
            cube.create("Python kolay bir dildir".into(), MemoryType::Semantic, None, None).expect("operation failed");
            
            let results = cube.search("Rust", None).expect("operation failed");
            assert_eq!(results.len(), 1);
            assert!(results[0].content.contains("Rust"));
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_link_and_get_related() {
        let db = test_db("link");
        cleanup(&db);
        {
            let mut cube = MemoryCube::new(&db).expect("operation failed");
            let id1 = cube.create("Makine öğrenimi".into(), MemoryType::Semantic, None, None).expect("operation failed");
            let id2 = cube.create("Derin öğrenme".into(), MemoryType::Semantic, None, None).expect("operation failed");
            
            cube.link(id1, id2, RelationType::RelatedTo, None).expect("operation failed");
            
            let related = cube.get_related(id1).expect("operation failed");
            assert_eq!(related.len(), 1);
            assert!(related[0].content.contains("Derin"));
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_stats() {
        let db = test_db("stats");
        cleanup(&db);
        {
            let mut cube = MemoryCube::new(&db).expect("operation failed");
            cube.create("Test 1".into(), MemoryType::Semantic, None, None).expect("operation failed");
            cube.create("Test 2".into(), MemoryType::Episodic, None, None).expect("operation failed");
            
            let stats = cube.stats().expect("operation failed");
            assert_eq!(stats.total_memories, 2);
            assert_eq!(stats.by_type.get(&MemoryType::Semantic), Some(&1));
        }
        cleanup(&db);
    }
}
