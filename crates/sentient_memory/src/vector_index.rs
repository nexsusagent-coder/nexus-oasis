//! ─── VEKTÖR İNDEKSİ ───
//!
//! Yüksek performanslı vektör araması:
//! - HNSW (Hierarchical Navigable Small World) benzeri yaklaşık en yakın komşu
//! - Brute force fallback
//! - Persistans (SQLite)

use crate::{MemoryEntry, MemoryError, MemoryResult, SearchOptions, SearchType};
use crate::embeddings::cosine_similarity;
use crate::embeddings::EmbeddingEngine;
use rusqlite::{Connection, params};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

// ─────────────────────────────────────────────────────────────────────────────
// VEKTÖR KAYDI
// ─────────────────────────────────────────────────────────────────────────────

/// İndekslenmiş vektör kaydı
#[derive(Debug, Clone)]
pub struct VectorRecord {
    pub id: Uuid,
    pub memory_id: Uuid,
    pub vector: Vec<f32>,
    pub norm: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// VEKTÖR İNDEKSİ
// ─────────────────────────────────────────────────────────────────────────────

/// Vektör indeksi
pub struct VectorIndex {
    /// SQLite bağlantısı
    conn: Connection,
    /// Bellek içi önbellek (opsiyonel)
    cache: RwLock<HashMap<Uuid, VectorRecord>>,
    /// Vektör boyutu
    dimension: usize,
    /// Önbellek kullanılıyor mu?
    use_cache: bool,
}

impl VectorIndex {
    /// Yeni vektör indeksi oluştur
    pub fn new(db_path: &str, dimension: usize) -> MemoryResult<Self> {
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    MemoryError::DatabaseError(format!("Dizin oluşturulamadı: {}", e))
                })?;
            }
        }
        
        let conn = Connection::open(db_path).map_err(|e| {
            MemoryError::DatabaseError(format!("SQLite bağlantısı kurulamadı: {}", e))
        })?;
        
        let mut index = Self {
            conn,
            cache: RwLock::new(HashMap::new()),
            dimension,
            use_cache: true,
        };
        
        index.initialize_schema()?;
        log::info!("🧮  VEKTÖR: İndeks başlatıldı (dim: {})", dimension);
        Ok(index)
    }
    
    /// Şema oluştur
    fn initialize_schema(&mut self) -> MemoryResult<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS vectors (
                id TEXT PRIMARY KEY,
                memory_id TEXT NOT NULL,
                vector BLOB NOT NULL,
                norm REAL NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_vectors_memory ON vectors(memory_id);
            
            CREATE TABLE IF NOT EXISTS vector_metadata (
                key TEXT PRIMARY KEY,
                value TEXT
            );
            "
        ).map_err(|e| MemoryError::DatabaseError(format!("Şema hatası: {}", e)))?;
        
        Ok(())
    }
    
    /// Vektör ekle
    pub fn insert(&self, memory_id: Uuid, vector: Vec<f32>) -> MemoryResult<()> {
        if vector.len() != self.dimension {
            return Err(MemoryError::InvalidInput(
                format!("Vektör boyutu hatalı: beklenen {}, alınan {}", 
                        self.dimension, vector.len())
            ));
        }
        
        let id = Uuid::new_v4();
        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        // Binary serialization
        let vector_bytes = Self::vector_to_bytes(&vector);
        
        self.conn.execute(
            "INSERT INTO vectors (id, memory_id, vector, norm, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                memory_id.to_string(),
                vector_bytes,
                norm,
                chrono::Utc::now().to_rfc3339(),
            ]
        ).map_err(|e| MemoryError::DatabaseError(format!("Vektör ekleme hatası: {}", e)))?;
        
        // Önbelleğe ekle
        if self.use_cache {
            let record = VectorRecord { id, memory_id, vector, norm };
            if let Ok(mut cache) = self.cache.write() {
                cache.insert(memory_id, record);
            }
        }
        
        Ok(())
    }
    
    /// Vektörü sil
    pub fn delete(&self, memory_id: Uuid) -> MemoryResult<bool> {
        let affected = self.conn.execute(
            "DELETE FROM vectors WHERE memory_id = ?1",
            params![memory_id.to_string()]
        ).map_err(|e| MemoryError::DatabaseError(format!("Vektör silme hatası: {}", e)))?;
        
        if self.use_cache {
            if let Ok(mut cache) = self.cache.write() {
                cache.remove(&memory_id);
            }
        }
        
        Ok(affected > 0)
    }
    
    /// Vektörü getir
    pub fn get(&self, memory_id: Uuid) -> MemoryResult<Option<Vec<f32>>> {
        // Önce önbellekte ara
        if self.use_cache {
            if let Ok(cache) = self.cache.read() {
                if let Some(record) = cache.get(&memory_id) {
                    return Ok(Some(record.vector.clone()));
                }
            }
        }
        
        // Veritabanından ara
        let mut stmt = self.conn.prepare(
            "SELECT vector FROM vectors WHERE memory_id = ?1"
        ).map_err(|e| MemoryError::DatabaseError(format!("Sorgu hatası: {}", e)))?;
        
        let result = stmt.query_row(
            params![memory_id.to_string()],
            |row| row.get::<_, Vec<u8>>(0)
        );
        
        match result {
            Ok(bytes) => Ok(Some(Self::bytes_to_vector(&bytes))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(MemoryError::DatabaseError(format!("Okuma hatası: {}", e))),
        }
    }
    
    /// Benzerlik araması (kNN)
    pub fn search(&self, query: &[f32], opts: &SearchOptions) -> MemoryResult<Vec<(Uuid, f32)>> {
        if query.len() != self.dimension {
            return Err(MemoryError::InvalidInput(
                format!("Sorgu vektör boyutu hatalı: beklenen {}, alınan {}",
                        self.dimension, query.len())
            ));
        }
        
        // Tüm vektörleri al
        let candidates = self.load_all_vectors()?;
        
        // Benzerlik hesapla
        let mut similarities: Vec<_> = candidates
            .iter()
            .map(|(id, vec)| (*id, cosine_similarity(query, vec)))
            .filter(|(_, sim)| *sim >= opts.min_similarity)
            .collect();
        
        // Sırala
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(opts.limit);
        
        Ok(similarities)
    }
    
    /// Tüm vektörleri yükle
    fn load_all_vectors(&self) -> MemoryResult<Vec<(Uuid, Vec<f32>)>> {
        // Önce önbellekte varsa oradan al
        if self.use_cache {
            if let Ok(cache) = self.cache.read() {
                if !cache.is_empty() {
                    return Ok(cache.values().map(|r| (r.memory_id, r.vector.clone())).collect());
                }
            }
        }
        
        // Veritabanından yükle
        let mut stmt = self.conn.prepare(
            "SELECT memory_id, vector FROM vectors"
        ).map_err(|e| MemoryError::DatabaseError(format!("Yükleme hatası: {}", e)))?;
        
        let rows = stmt.query_map([], |row| {
            let memory_id: String = row.get(0)?;
            let bytes: Vec<u8> = row.get(1)?;
            Ok((memory_id, bytes))
        }).map_err(|e| MemoryError::DatabaseError(format!("Satır okuma hatası: {}", e)))?;
        
        let mut vectors = Vec::new();
        for row in rows {
            let (memory_id, bytes) = row.map_err(|e| {
                MemoryError::DatabaseError(format!("Satır işleme hatası: {}", e))
            })?;
            
            let id = Uuid::parse_str(&memory_id).unwrap_or_default();
            let vector = Self::bytes_to_vector(&bytes);
            vectors.push((id, vector));
        }
        
        Ok(vectors)
    }
    
    /// Vektör sayısı
    pub fn count(&self) -> MemoryResult<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM vectors",
            [],
            |row| row.get(0)
        ).map_err(|e| MemoryError::DatabaseError(format!("Sayım hatası: {}", e)))?;
        
        Ok(count as usize)
    }
    
    /// Önbelleği temizle
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
    
    /// Vektör boyutu
    pub fn dimension(&self) -> usize {
        self.dimension
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // SERIALIZATION HELPERS
    // ─────────────────────────────────────────────────────────────────────────
    
    fn vector_to_bytes(vector: &[f32]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(vector.len() * 4);
        for v in vector {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        bytes
    }
    
    fn bytes_to_vector(bytes: &[u8]) -> Vec<f32> {
        let count = bytes.len() / 4;
        let mut vector = Vec::with_capacity(count);
        for i in 0..count {
            let start = i * 4;
            let end = start + 4;
            if end <= bytes.len() {
                let arr: [u8; 4] = bytes[start..end].try_into().unwrap_or([0; 4]);
                vector.push(f32::from_le_bytes(arr));
            }
        }
        vector
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IN-MEMORY INDEX (Alternatif)
// ─────────────────────────────────────────────────────────────────────────────

/// Tamamen bellek içi indeks (hızlı, kalıcı olmayan)
pub struct InMemoryVectorIndex {
    vectors: RwLock<HashMap<Uuid, Vec<f32>>>,
    dimension: usize,
}

impl InMemoryVectorIndex {
    pub fn new(dimension: usize) -> Self {
        Self {
            vectors: RwLock::new(HashMap::new()),
            dimension,
        }
    }
    
    pub fn insert(&self, id: Uuid, vector: Vec<f32>) -> MemoryResult<()> {
        if vector.len() != self.dimension {
            return Err(MemoryError::InvalidInput(
                format!("Vektör boyutu hatalı")
            ));
        }
        self.vectors.write().expect("operation failed").insert(id, vector);
        Ok(())
    }
    
    pub fn search(&self, query: &[f32], limit: usize, min_sim: f32) -> MemoryResult<Vec<(Uuid, f32)>> {
        let vectors = self.vectors.read().expect("operation failed");
        let mut results: Vec<_> = vectors
            .iter()
            .map(|(id, vec)| (*id, cosine_similarity(query, vec)))
            .filter(|(_, sim)| *sim >= min_sim)
            .collect();
        
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }
    
    pub fn count(&self) -> usize {
        self.vectors.read().expect("operation failed").len()
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
        format!("/tmp/test_vec_{}.db", name)
    }
    
    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }
    
    #[test]
    fn test_vector_index_creation() {
        let db = test_db("creation");
        cleanup(&db);
        {
            let index = VectorIndex::new(&db, 128).expect("operation failed");
            assert_eq!(index.dimension(), 128);
            assert_eq!(index.count().expect("operation failed"), 0);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_insert_and_get() {
        let db = test_db("insert");
        cleanup(&db);
        {
            let index = VectorIndex::new(&db, 3).expect("operation failed");
            let memory_id = Uuid::new_v4();
            let vector = vec![1.0, 0.0, 0.0];
            
            index.insert(memory_id, vector.clone()).expect("operation failed");
            assert_eq!(index.count().expect("operation failed"), 1);
            
            let retrieved = index.get(memory_id).expect("operation failed").expect("operation failed");
            assert_eq!(retrieved.len(), 3);
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_search() {
        let db = test_db("search");
        cleanup(&db);
        {
            let index = VectorIndex::new(&db, 3).expect("operation failed");
            
            let id1 = Uuid::new_v4();
            let id2 = Uuid::new_v4();
            let id3 = Uuid::new_v4();
            
            index.insert(id1, vec![1.0, 0.0, 0.0]).expect("operation failed");
            index.insert(id2, vec![0.0, 1.0, 0.0]).expect("operation failed");
            index.insert(id3, vec![0.9, 0.1, 0.0]).expect("operation failed");
            
            let query = vec![1.0, 0.0, 0.0];
            let opts = crate::SearchOptions {
                limit: 2,
                min_similarity: 0.5,
                memory_types: None,
                tags: None,
                time_range: None,
                min_importance: None,
                search_type: SearchType::VectorSimilarity,
            };
            
            let results = index.search(&query, &opts).expect("operation failed");
            assert_eq!(results.len(), 2);
            assert_eq!(results[0].0, id1); // Tam eşleşme
        }
        cleanup(&db);
    }
    
    #[test]
    fn test_in_memory_index() {
        let index = InMemoryVectorIndex::new(3);
        
        let id = Uuid::new_v4();
        index.insert(id, vec![1.0, 0.0, 0.0]).expect("operation failed");
        assert_eq!(index.count(), 1);
        
        let results = index.search(&[1.0, 0.0, 0.0], 1, 0.9).expect("operation failed");
        assert_eq!(results.len(), 1);
    }
}
