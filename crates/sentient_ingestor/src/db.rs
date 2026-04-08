//! ═══════════════════════════════════════════════════════════════════════════════
//!  SKILL DATABASE - SQLite Skill Storage
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{IngestorError, IngestorResult};
use crate::unified_yaml::UnifiedSkill;
use rusqlite::{Connection, params, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Skill veritabanı kaydı
#[derive(Debug, Clone)]
pub struct SkillRecord {
    pub id: i64,
    pub skill_id: String,
    pub name: String,
    pub slug: String,
    pub category: String,
    pub description: String,
    pub yaml_content: String,
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Skill Database
pub struct SkillDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl SkillDatabase {
    /// Yeni veritabanı oluştur
    pub fn new<P: AsRef<Path>>(path: P) -> IngestorResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.initialize()?;
        Ok(db)
    }
    
    /// In-memory veritabanı oluştur
    pub fn in_memory() -> IngestorResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.initialize()?;
        Ok(db)
    }
    
    /// Tabloları oluştur
    fn initialize(&self) -> IngestorResult<()> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS skills (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                skill_id TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                slug TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT NOT NULL,
                yaml_content TEXT NOT NULL,
                hash TEXT UNIQUE NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_skills_category ON skills(category);
            CREATE INDEX IF NOT EXISTS idx_skills_name ON skills(name);
            CREATE INDEX IF NOT EXISTS idx_skills_slug ON skills(slug);
            
            CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                skill_count INTEGER DEFAULT 0,
                last_updated TEXT
            );
            
            CREATE TABLE IF NOT EXISTS ingest_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                total_skills INTEGER DEFAULT 0,
                new_skills INTEGER DEFAULT 0,
                updated_skills INTEGER DEFAULT 0,
                status TEXT DEFAULT 'running'
            );
        "#)?;
        
        Ok(())
    }
    
    /// Skill ekle veya güncelle
    pub fn upsert_skill(&self, skill: &UnifiedSkill) -> IngestorResult<bool> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let yaml_content = skill.to_yaml()
            .map_err(|e| IngestorError::YamlError(e))?;
        
        // Önce var mı kontrol et
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM skills WHERE hash = ?1",
            params![&skill.hash],
            |row| row.get(0)
        ).unwrap_or(false);
        
        if exists {
            // Güncelle
            conn.execute(
                r#"UPDATE skills SET 
                   name = ?1, slug = ?2, category = ?3, description = ?4,
                   yaml_content = ?5, updated_at = ?6
                   WHERE hash = ?7"#,
                params![
                    &skill.name,
                    &skill.slug,
                    &skill.category,
                    &skill.description,
                    &yaml_content,
                    &skill.updated_at,
                    &skill.hash,
                ],
            )?;
            Ok(false) // Güncellendi, yeni değil
        } else {
            // Yeni ekle
            conn.execute(
                r#"INSERT INTO skills 
                   (skill_id, name, slug, category, description, yaml_content, hash, created_at, updated_at)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#,
                params![
                    &skill.id,
                    &skill.name,
                    &skill.slug,
                    &skill.category,
                    &skill.description,
                    &yaml_content,
                    &skill.hash,
                    &skill.created_at,
                    &skill.updated_at,
                ],
            )?;
            Ok(true) // Yeni eklendi
        }
    }
    
    /// Kategoriye göre skill'leri getir
    pub fn get_by_category(&self, category: &str) -> IngestorResult<Vec<SkillRecord>> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, skill_id, name, slug, category, description, yaml_content, hash, created_at, updated_at 
             FROM skills WHERE category = ?1 ORDER BY name"
        )?;
        
        let records = stmt.query_map(params![category], |row| {
            Ok(SkillRecord {
                id: row.get(0)?,
                skill_id: row.get(1)?,
                name: row.get(2)?,
                slug: row.get(3)?,
                category: row.get(4)?,
                description: row.get(5)?,
                yaml_content: row.get(6)?,
                hash: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        Ok(records)
    }
    
    /// Skill ara
    pub fn search(&self, query: &str) -> IngestorResult<Vec<SkillRecord>> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let pattern = format!("%{}%", query.to_lowercase());
        
        let mut stmt = conn.prepare(
            r#"SELECT id, skill_id, name, slug, category, description, yaml_content, hash, created_at, updated_at 
               FROM skills 
               WHERE LOWER(name) LIKE ?1 OR LOWER(description) LIKE ?1
               ORDER BY name
               LIMIT 100"#
        )?;
        
        let records = stmt.query_map(params![&pattern], |row| {
            Ok(SkillRecord {
                id: row.get(0)?,
                skill_id: row.get(1)?,
                name: row.get(2)?,
                slug: row.get(3)?,
                category: row.get(4)?,
                description: row.get(5)?,
                yaml_content: row.get(6)?,
                hash: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        Ok(records)
    }
    
    /// Toplam skill sayısı
    pub fn count(&self) -> IngestorResult<i64> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM skills",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        Ok(count)
    }
    
    /// Kategori istatistikleri
    pub fn category_stats(&self) -> IngestorResult<Vec<(String, i64)>> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let mut stmt = conn.prepare(
            "SELECT category, COUNT(*) as count FROM skills GROUP BY category ORDER BY count DESC"
        )?;
        
        let stats = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        Ok(stats)
    }
    
    /// Ingest history başlat
    pub fn start_ingest(&self) -> IngestorResult<i64> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO ingest_history (started_at, status) VALUES (?1, 'running')",
            params![&now],
        )?;
        
        Ok(conn.last_insert_rowid())
    }
    
    /// Ingest history bitir
    pub fn finish_ingest(&self, id: i64, total: i64, new: i64, updated: i64) -> IngestorResult<()> {
        let conn = self.conn.lock().map_err(|e| IngestorError::DatabaseError(e.to_string()))?;
        
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            r#"UPDATE ingest_history SET 
               finished_at = ?1, total_skills = ?2, new_skills = ?3, updated_skills = ?4, status = 'completed'
               WHERE id = ?5"#,
            params![&now, total, new, updated, id],
        )?;
        
        Ok(())
    }
}
