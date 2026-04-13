//! ═════════════════════════════════════════════════════════════════
//!  MIGRATION MODULE - Veritabanı Migrasyon Yöneticisi
//! ═════════════════════════════════════════════════════════════════
//!
//! SQLite şema değişikliklerini güvenli şekilde yönetir.
//! Versiyon takibi, geri alma (rollback) ve doğrulama.

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Migrasyon kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
    pub applied_at: Option<DateTime<Utc>>,
}

/// Migrasyon sonucu
#[derive(Debug)]
pub enum MigrationResult {
    Success { version: u32, name: String },
    AlreadyApplied { version: u32 },
    RolledBack { version: u32 },
    Error { version: u32, message: String },
}

/// Migrasyon yöneticisi
pub struct MigrationManager {
    migrations: Vec<Migration>,
}

impl MigrationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            migrations: Vec::new(),
        };
        manager.register_default_migrations();
        manager
    }

    /// Varsayılan migrasyonları kaydet
    fn register_default_migrations(&mut self) {
        // V1: Temel memories tablosu (zaten var, boş bırak)
        self.add_migration(1, "initial_schema",
            "CREATE TABLE IF NOT EXISTS memories (id TEXT PRIMARY KEY);",
            "DROP TABLE IF EXISTS memories;"
        );

        // V2: İlişki tablosu
        self.add_migration(2, "add_relations",
            "CREATE TABLE IF NOT EXISTS memory_relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                created_at TEXT NOT NULL,
                UNIQUE(source_id, target_id, relation_type)
            );",
            "DROP TABLE IF NOT EXISTS memory_relations;"
        );

        // V3: Meta veri tablosu
        self.add_migration(3, "add_metadata_table",
            "CREATE TABLE IF NOT EXISTS memory_metadata (
                memory_id TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY(memory_id, key)
            );",
            "DROP TABLE IF NOT EXISTS memory_metadata;"
        );

        // V4: Arama indeksleri
        self.add_migration(4, "add_search_indexes",
            "CREATE INDEX IF NOT EXISTS idx_memories_type ON memories(entry_type);
             CREATE INDEX IF NOT EXISTS idx_memories_created ON memories(created_at);
             CREATE INDEX IF NOT EXISTS idx_memories_importance ON memories(importance);",
            "DROP INDEX IF EXISTS idx_memories_type;
             DROP INDEX IF EXISTS idx_memories_created;
             DROP INDEX IF EXISTS idx_memories_importance;"
        );

        // V5: Sıkıştırma desteği
        self.add_migration(5, "add_compression_flag",
            "ALTER TABLE memories ADD COLUMN is_compressed INTEGER DEFAULT 0;
             ALTER TABLE memories ADD COLUMN original_size INTEGER DEFAULT 0;",
            // SQLite ALTER TABLE DROP COLUMN desteklenmeyebilir - boş bırak
            "SELECT 1;"
        );

        // V6: Versiyon takip
        self.add_migration(6, "add_version_tracking",
            "ALTER TABLE memories ADD COLUMN schema_version INTEGER DEFAULT 1;",
            "SELECT 1;"
        );
    }

    /// Yeni migrasyon ekle
    pub fn add_migration(&mut self, version: u32, name: &str, up_sql: &str, down_sql: &str) {
        self.migrations.push(Migration {
            version,
            name: name.into(),
            up_sql: up_sql.into(),
            down_sql: down_sql.into(),
            applied_at: None,
        });
        self.migrations.sort_by_key(|m| m.version);
    }

    /// Migrasyon tablosunu oluştur
    fn ensure_migration_table(&self, conn: &Connection) -> Result<(), String> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL
            );"
        ).map_err(|e| format!("Migration tablo hatası: {}", e))
    }

    /// Mevcut versiyonu al
    pub fn current_version(&self, conn: &Connection) -> u32 {
        self.ensure_migration_table(conn).ok();
        let mut stmt = conn.prepare("SELECT MAX(version) FROM _migrations").unwrap();
        stmt.query_row([], |row| row.get::<_, Option<u32>>(0))
            .ok()
            .flatten()
            .unwrap_or(0)
    }

    /// Tüm migrasyonları uygula
    pub fn migrate_up(&self, conn: &Connection) -> Vec<MigrationResult> {
        self.ensure_migration_table(conn).ok();
        let current = self.current_version(conn);
        let mut results = Vec::new();

        for migration in &self.migrations {
            if migration.version <= current {
                results.push(MigrationResult::AlreadyApplied { version: migration.version });
                continue;
            }

            match conn.execute_batch(&migration.up_sql) {
                Ok(_) => {
                    let now = Utc::now().to_rfc3339();
                    let _ = conn.execute(
                        "INSERT INTO _migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
                        params![migration.version, migration.name, now],
                    );
                    log::info!("🧠  MIGRATION: v{} {} uygulandı", migration.version, migration.name);
                    results.push(MigrationResult::Success {
                        version: migration.version,
                        name: migration.name.clone(),
                    });
                }
                Err(e) => {
                    log::error!("🧠  MIGRATION: v{} {} HATA: {}", migration.version, migration.name, e);
                    results.push(MigrationResult::Error {
                        version: migration.version,
                        message: e.to_string(),
                    });
                    break; // Hata durumunda dur
                }
            }
        }

        results
    }

    /// Son migrasyonu geri al
    pub fn migrate_down(&self, conn: &Connection) -> MigrationResult {
        self.ensure_migration_table(conn).ok();
        let current = self.current_version(conn);

        if current == 0 {
            return MigrationResult::Error { version: 0, message: "Geri alınacak migrasyon yok".into() };
        }

        if let Some(migration) = self.migrations.iter().find(|m| m.version == current) {
            match conn.execute_batch(&migration.down_sql) {
                Ok(_) => {
                    let _ = conn.execute(
                        "DELETE FROM _migrations WHERE version = ?1",
                        params![current],
                    );
                    log::info!("🧠  MIGRATION: v{} {} geri alındı", current, migration.name);
                    MigrationResult::RolledBack { version: current }
                }
                Err(e) => {
                    log::error!("🧠  MIGRATION: v{} geri alma HATA: {}", current, e);
                    MigrationResult::Error { version: current, message: e.to_string() }
                }
            }
        } else {
            MigrationResult::Error { version: current, message: "Migrasyon tanımı bulunamadı".into() }
        }
    }

    /// Migrasyon durumunu listele
    pub fn status(&self, conn: &Connection) -> Vec<(u32, String, bool)> {
        self.ensure_migration_table(conn).ok();
        let current = self.current_version(conn);
        self.migrations.iter().map(|m| {
            (m.version, m.name.clone(), m.version <= current)
        }).collect()
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_manager() {
        let manager = MigrationManager::new();
        assert!(!manager.migrations.is_empty());
    }

    #[test]
    fn test_migrations_sorted() {
        let manager = MigrationManager::new();
        for i in 1..manager.migrations.len() {
            assert!(manager.migrations[i-1].version < manager.migrations[i].version);
        }
    }
}
