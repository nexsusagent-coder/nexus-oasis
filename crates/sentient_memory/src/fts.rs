//! ─── FTS5 - TAM METİN ARAMA ───
//!
//! SQLite FTS5 tabanlı tam metin arama motoru.
//! Vektör araması ile hibrit sorgulama desteği.
//!
//! Özellikler:
//! - FTS5 virtual table ile full-text search
//! - Türkçe karakter duyarsız arama
//! - BM25 ranking algoritması
//! - Hibrit arama (FTS + Vector)
//! - Snippet ve highlight desteği

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{MemoryType, SearchResult, SearchType};

// ─────────────────────────────────────────────────────────────────────────────
// FTS5 ARAMA SONUCU
// ─────────────────────────────────────────────────────────────────────────────

/// FTS5 arama sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsResult {
    /// Bellek ID
    pub memory_id: Uuid,
    /// Eşleşen içerik
    pub content: String,
    /// BM25 skoru
    pub bm25_score: f32,
    /// Highlight edilmiş snippet
    pub snippet: Option<String>,
    /// Başlık (varsa)
    pub title: Option<String>,
    /// Bellek tipi
    pub memory_type: MemoryType,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Etiketler
    pub tags: Vec<String>,
}

/// FTS5 arama seçenekleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsOptions {
    /// Arama sorgusu
    pub query: String,
    /// Maksimum sonuç
    pub limit: usize,
    /// Minimum BM25 skoru
    pub min_score: f32,
    /// Bellek tipi filtresi
    pub memory_types: Option<Vec<MemoryType>>,
    /// Etiket filtresi
    pub tags: Option<Vec<String>>,
    /// Zaman aralığı
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Snippet uzunluğu
    pub snippet_length: usize,
    /// Highlight marker
    pub highlight_marker: String,
    /// Önek arama (ör: "test*" -> test ile başlayan)
    pub prefix_search: bool,
}

impl Default for FtsOptions {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 10,
            min_score: -10.0,
            memory_types: None,
            tags: None,
            time_range: None,
            snippet_length: 100,
            highlight_marker: "**".into(),
            prefix_search: true,
        }
    }
}

impl FtsOptions {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            ..Default::default()
        }
    }
    
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    pub fn with_types(mut self, types: Vec<MemoryType>) -> Self {
        self.memory_types = Some(types);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
    
    pub fn with_snippet_length(mut self, length: usize) -> Self {
        self.snippet_length = length;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HİBRİT ARAMA SONUCU
// ─────────────────────────────────────────────────────────────────────────────

/// Hibrit arama sonucu (FTS + Vector)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridResult {
    /// Bellek ID
    pub memory_id: Uuid,
    /// İçerik
    pub content: String,
    /// FTS skoru (normalize edilmiş 0-1)
    pub fts_score: f32,
    /// Vektör benzerlik skoru (0-1)
    pub vector_score: f32,
    /// Birleştirilmiş skor
    pub combined_score: f32,
    /// Arama tipleri
    pub search_types: Vec<SearchType>,
    /// Snippet
    pub snippet: Option<String>,
    /// Bellek tipi
    pub memory_type: MemoryType,
    /// Etiketler
    pub tags: Vec<String>,
}

/// Hibrit arama ağırlıkları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridWeights {
    /// FTS ağırlığı (varsayılan: 0.4)
    pub fts: f32,
    /// Vektör ağırlığı (varsayılan: 0.6)
    pub vector: f32,
}

impl Default for HybridWeights {
    fn default() -> Self {
        Self {
            fts: 0.4,
            vector: 0.6,
        }
    }
}

impl HybridWeights {
    pub fn new(fts: f32, vector: f32) -> Self {
        Self {
            fts: fts.clamp(0.0, 1.0),
            vector: vector.clamp(0.0, 1.0),
        }
    }
    
    /// Birleştirilmiş skor hesapla
    pub fn combine(&self, fts_score: f32, vector_score: f32) -> f32 {
        (fts_score * self.fts) + (vector_score * self.vector)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FTS5 MOTORU
// ─────────────────────────────────────────────────────────────────────────────

/// FTS5 arama motoru
pub struct FtsEngine {
    /// Veritabanı bağlantısı
    conn: Arc<RwLock<Connection>>,
    /// Cube ID
    cube_id: Uuid,
    /// Tablo adı
    table_name: String,
}

impl FtsEngine {
    /// Yeni FTS motoru oluştur
    pub fn new(conn: Arc<RwLock<Connection>>, cube_id: Uuid) -> Self {
        let table_name = format!("memory_fts_{}", cube_id.simple());
        Self {
            conn,
            cube_id,
            table_name,
        }
    }
    
    /// FTS5 tablosunu oluştur
    pub async fn initialize(&self) -> rusqlite::Result<()> {
        let conn = self.conn.write().await;
        
        // FTS5 virtual table oluştur
        conn.execute(&format!(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS {table} USING fts5(
                memory_id UNINDEXED,
                content,
                title,
                tags,
                memory_type,
                tokenize = 'unicode61 remove_diacritics 2'
            );
            "#,
            table = self.table_name
        ), [])?;
        
        log::info!("📝 FTS5 tablosu oluşturuldu: {}", self.table_name);
        Ok(())
    }
    
    /// Bellek ekle
    pub async fn insert(
        &self,
        memory_id: Uuid,
        content: &str,
        title: Option<&str>,
        tags: &[String],
        memory_type: MemoryType,
    ) -> rusqlite::Result<()> {
        let conn = self.conn.write().await;
        
        let title_str = title.unwrap_or("");
        let tags_str = tags.join(" ");
        let type_str = serde_json::to_string(&memory_type).unwrap_or_default();
        
        conn.execute(&format!(
            "INSERT INTO {} (memory_id, content, title, tags, memory_type) VALUES (?, ?, ?, ?, ?)",
            self.table_name
        ), params![
            memory_id.to_string(),
            content,
            title_str,
            tags_str,
            type_str
        ])?;
        
        Ok(())
    }
    
    /// Bellek güncelle
    pub async fn update(
        &self,
        memory_id: Uuid,
        content: &str,
        title: Option<&str>,
        tags: &[String],
    ) -> rusqlite::Result<()> {
        let conn = self.conn.write().await;
        
        // Önce sil
        conn.execute(&format!(
            "DELETE FROM {} WHERE memory_id = ?",
            self.table_name
        ), params![memory_id.to_string()])?;
        
        // Sonra ekle
        drop(conn);
        self.insert(memory_id, content, title, tags, MemoryType::Semantic).await
    }
    
    /// Bellek sil
    pub async fn delete(&self, memory_id: Uuid) -> rusqlite::Result<()> {
        let conn = self.conn.write().await;
        
        conn.execute(&format!(
            "DELETE FROM {} WHERE memory_id = ?",
            self.table_name
        ), params![memory_id.to_string()])?;
        
        Ok(())
    }
    
    /// FTS5 arama
    pub async fn search(&self, options: &FtsOptions) -> rusqlite::Result<Vec<FtsResult>> {
        let conn = self.conn.read().await;
        
        // Sorguyu FTS5 formatına çevir
        let fts_query = self.build_fts_query(&options.query, options.prefix_search);
        
        // BM25 ile sıralı arama
        let sql = format!(
            r#"
            SELECT 
                memory_id,
                content,
                bm25({table}) as score,
                snippet({table}, 0, '{hl}', '{hl}', '...', {len}) as snippet,
                title,
                memory_type,
                tags
            FROM {table}
            WHERE {table} MATCH ?
            ORDER BY bm25({table})
            LIMIT ?
            "#,
            table = self.table_name,
            hl = options.highlight_marker,
            len = options.snippet_length
        );
        
        let mut stmt = conn.prepare(&sql)?;
        
        let results = stmt.query_map(params![fts_query, options.limit as i32], |row| {
            Ok(self.row_to_fts_result(row))
        })?
        .filter_map(|r| r.ok())
        .filter(|r| r.bm25_score >= options.min_score)
        .collect();
        
        Ok(results)
    }
    
    /// Satırdan FTS sonucu oluştur
    fn row_to_fts_result(&self, row: &Row) -> FtsResult {
        let memory_id_str: String = row.get(0).unwrap_or_default();
        let memory_id = Uuid::parse_str(&memory_id_str).unwrap_or(Uuid::nil());
        
        let memory_type_str: String = row.get(5).unwrap_or_default();
        let memory_type: MemoryType = serde_json::from_str(&memory_type_str).unwrap_or_default();
        
        let tags_str: String = row.get(6).unwrap_or_default();
        let tags: Vec<String> = tags_str.split_whitespace().map(|s| s.to_string()).collect();
        
        FtsResult {
            memory_id,
            content: row.get(1).unwrap_or_default(),
            bm25_score: row.get(2).unwrap_or(0.0),
            snippet: row.get(3).ok(),
            title: row.get(4).ok(),
            memory_type,
            created_at: Utc::now(), // Not stored in FTS
            tags,
        }
    }
    
    /// FTS5 sorgusu oluştur
    fn build_fts_query(&self, query: &str, prefix_search: bool) -> String {
        // Türkçe karakter normalizasyonu
        let normalized = self.normalize_turkish(query);
        
        // Kelimelere ayır
        let words: Vec<&str> = normalized.split_whitespace().collect();
        
        if words.is_empty() {
            return String::new();
        }
        
        // FTS5 sorgusu oluştur
        let fts_terms: Vec<String> = words.iter().map(|word| {
            if prefix_search {
                format!("{}*", word)
            } else {
                word.to_string()
            }
        }).collect();
        
        fts_terms.join(" ")
    }
    
    /// Türkçe karakter normalizasyonu
    fn normalize_turkish(&self, text: &str) -> String {
        // Unicode Normalizasyon Form D (NFD) - Aksanları ayır
        // Sonra Türkçe karakterleri ASCII'ye çevir
        text.to_lowercase()
            .replace('ı', "i")
            .replace('ğ', "g")
            .replace('ü', "u")
            .replace('ş', "s")
            .replace('ö', "o")
            .replace('ç', "c")
            .replace('İ', "i")
            .replace('Ğ', "g")
            .replace('Ü', "u")
            .replace('Ş', "s")
            .replace('Ö', "o")
            .replace('Ç', "c")
            // Aksanlı karakterleri temizle
            .replace('̇', "")  // Combining dot above
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ')
            .collect()
    }
    
    /// Tabloyu temizle
    pub async fn clear(&self) -> rusqlite::Result<()> {
        let conn = self.conn.write().await;
        conn.execute(&format!("DELETE FROM {}", self.table_name), [])?;
        Ok(())
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> rusqlite::Result<FtsStats> {
        let conn = self.conn.read().await;
        
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", self.table_name),
            [],
            |row| row.get(0)
        )?;
        
        Ok(FtsStats {
            total_documents: count as u64,
            table_name: self.table_name.clone(),
        })
    }
}

/// FTS istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtsStats {
    pub total_documents: u64,
    pub table_name: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// HİBRİT ARAMA MOTORU
// ─────────────────────────────────────────────────────────────────────────────

/// Hibrit arama motoru (FTS + Vector)
pub struct HybridSearchEngine {
    /// FTS motoru
    fts: Arc<FtsEngine>,
    /// Ağırlıklar
    weights: HybridWeights,
}

impl HybridSearchEngine {
    pub fn new(fts: Arc<FtsEngine>, weights: HybridWeights) -> Self {
        Self { fts, weights }
    }
    
    /// Hibrit arama yap
    /// Not: Vektör sonuçları dışarıdan sağlanmalı (VectorIndex'ten)
    pub async fn combine_results(
        &self,
        fts_results: Vec<FtsResult>,
        vector_results: Vec<SearchResult>,
        limit: usize,
    ) -> Vec<HybridResult> {
        // FTS skorlarını normalize et (BM25 negatif olabilir, 0-1 arasına çek)
        let max_fts_score = fts_results.iter()
            .map(|r| r.bm25_score)
            .fold(0.0f32, f32::max)
            .max(1.0);
        
        // Sonuçları ID ile birleştir
        let mut combined: HashMap<Uuid, HybridResult> = HashMap::new();
        
        // FTS sonuçlarını ekle
        for fts_r in fts_results {
            let normalized_fts = (fts_r.bm25_score + 10.0) / (max_fts_score + 10.0);
            
            combined.entry(fts_r.memory_id).or_insert(HybridResult {
                memory_id: fts_r.memory_id,
                content: fts_r.content.clone(),
                fts_score: normalized_fts.clamp(0.0, 1.0),
                vector_score: 0.0,
                combined_score: normalized_fts * self.weights.fts,
                search_types: vec![SearchType::KeywordMatch],
                snippet: fts_r.snippet,
                memory_type: fts_r.memory_type,
                tags: fts_r.tags,
            });
        }
        
        // Vektör sonuçlarını ekle/güncelle
        for vec_r in vector_results {
            let entry = combined.entry(vec_r.memory.id).or_insert(HybridResult {
                memory_id: vec_r.memory.id,
                content: vec_r.memory.content.clone(),
                fts_score: 0.0,
                vector_score: vec_r.similarity,
                combined_score: 0.0,
                search_types: vec![],
                snippet: None,
                memory_type: vec_r.memory.memory_type,
                tags: vec_r.memory.tags.clone(),
            });
            
            // Vektör skorunu güncelle
            entry.vector_score = vec_r.similarity;
            entry.search_types.push(SearchType::VectorSimilarity);
            
            // Birleştirilmiş skoru yeniden hesapla
            entry.combined_score = self.weights.combine(entry.fts_score, entry.vector_score);
            
            // Her iki arama tipi varsa hibrit olarak işaretle
            if entry.search_types.len() == 2 {
                entry.search_types.clear();
                entry.search_types.push(SearchType::Hybrid);
            }
        }
        
        // Birleştirilmiş skora göre sırala
        let mut results: Vec<HybridResult> = combined.into_values().collect();
        results.sort_by(|a, b| {
            b.combined_score.partial_cmp(&a.combined_score).expect("operation failed")
        });
        
        results.truncate(limit);
        results
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hybrid_weights() {
        let weights = HybridWeights::default();
        let combined = weights.combine(0.8, 0.6);
        
        // 0.8 * 0.4 + 0.6 * 0.6 = 0.32 + 0.36 = 0.68
        assert!((combined - 0.68).abs() < 0.01);
    }
    
    #[test]
    fn test_fts_options() {
        let opts = FtsOptions::new("test query")
            .with_limit(5)
            .with_snippet_length(150);
        
        assert_eq!(opts.query, "test query");
        assert_eq!(opts.limit, 5);
        assert_eq!(opts.snippet_length, 150);
    }
    
    #[test]
    fn test_normalize_turkish() {
        let fts = FtsEngine::new(
            Arc::new(RwLock::new(Connection::open_in_memory().expect("operation failed"))),
            Uuid::new_v4()
        );
        
        let normalized = fts.normalize_turkish("İŞĞÜŞİÖÇ");
        assert_eq!(normalized, "isgusioc");
    }
}
