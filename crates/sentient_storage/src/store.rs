//! ─── TASK STORE ───
//!
//! Görev veritabanı işlemleri (rusqlite)

use crate::models::*;
use chrono::Utc;
use rusqlite::{Connection, params, OptionalExtension};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub const DB_PATH: &str = "data/sentient.db";

/// Görev deposu
#[derive(Clone)]
pub struct TaskStore {
    conn: Arc<RwLock<Connection>>,
}

impl TaskStore {
    /// Yeni görev deposu oluştur
    pub async fn new() -> StorageResult<Self> {
        // Veri dizini oluştur
        std::fs::create_dir_all("data").ok();
        
        let path = Path::new(DB_PATH);
        let conn = Connection::open(path)
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;
        
        let store = Self {
            conn: Arc::new(RwLock::new(conn)),
        };
        
        store.init_schema().await?;
        log::info!("📊 TaskStore başlatıldı: {}", DB_PATH);
        
        Ok(store)
    }
    
    /// Şemayı oluştur
    async fn init_schema(&self) -> StorageResult<()> {
        let conn = self.conn.write().await;
        
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                request_id TEXT NOT NULL,
                goal TEXT NOT NULL,
                model TEXT NOT NULL DEFAULT 'qwen/qwen3-1.7b:free',
                status TEXT NOT NULL DEFAULT 'queued',
                priority INTEGER NOT NULL DEFAULT 1,
                assigned_agent TEXT,
                current_step INTEGER,
                total_steps INTEGER NOT NULL DEFAULT 0,
                progress REAL NOT NULL DEFAULT 0.0,
                started_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                completed_at TEXT,
                result TEXT,
                error TEXT,
                source TEXT NOT NULL DEFAULT 'api',
                user_id TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0,
                checkpoint TEXT
            );
            
            CREATE TABLE IF NOT EXISTS task_steps (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                step_number INTEGER NOT NULL,
                agent TEXT NOT NULL,
                action TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                input TEXT,
                output TEXT,
                started_at TEXT NOT NULL,
                completed_at TEXT,
                tokens_used INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
            );
            
            CREATE TABLE IF NOT EXISTS task_logs (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL DEFAULT 'info',
                source TEXT NOT NULL DEFAULT 'system',
                message TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
            );
            
            CREATE TABLE IF NOT EXISTS workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                workflow_type TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                current_task_id TEXT,
                dependencies TEXT,
                params TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS checkpoints (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                checkpoint_type TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
            );
            
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_tasks_updated ON tasks(updated_at);
            CREATE INDEX IF NOT EXISTS idx_task_steps_task ON task_steps(task_id);
            CREATE INDEX IF NOT EXISTS idx_logs_task ON task_logs(task_id);
            CREATE INDEX IF NOT EXISTS idx_logs_time ON task_logs(timestamp);
            CREATE INDEX IF NOT EXISTS idx_checkpoints_task ON checkpoints(task_id);
        "#).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::info!("✅ Veritabanı şeması oluşturuldu");
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════
    // TASK OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    /// Yeni görev kaydet
    pub async fn insert_task(&self, task: &PersistedTask) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let result_json = task.result.as_ref().and_then(|r| serde_json::to_string(r).ok());
        let checkpoint_json = task.checkpoint.as_ref().and_then(|c| serde_json::to_string(c).ok());
        
        conn.execute(r#"
            INSERT INTO tasks (
                id, request_id, goal, model, status, priority, assigned_agent,
                current_step, total_steps, progress, started_at, updated_at,
                completed_at, result, error, source, user_id, retry_count, checkpoint
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
        "#, params![
            task.id.to_string(),
            task.request_id.to_string(),
            &task.goal,
            &task.model,
            task.status.as_str(),
            task.priority as i32,
            &task.assigned_agent,
            task.current_step.map(|s| s as i32),
            task.total_steps as i32,
            task.progress,
            &task.started_at,
            &task.updated_at,
            &task.completed_at,
            result_json,
            &task.error,
            &task.source,
            &task.user_id,
            task.retry_count as i32,
            checkpoint_json,
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::debug!("💾 Görev kaydedildi: {}", task.id);
        Ok(())
    }
    
    /// Görev durumunu güncelle
    pub async fn update_task_status(&self, task_id: Uuid, status: PersistedStatus) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        let completed_at = if status.is_terminal() { Some(&now) } else { None };
        
        conn.execute(r#"
            UPDATE tasks SET status = ?1, updated_at = ?2, completed_at = COALESCE(?3, completed_at)
            WHERE id = ?4
        "#, params![
            status.as_str(),
            &now,
            completed_at,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::debug!("📝 Görev durumu güncellendi: {} → {}", task_id, status);
        Ok(())
    }
    
    /// İlerlemeyi güncelle
    pub async fn update_progress(&self, task_id: Uuid, progress: f32, current_step: Option<u32>) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(r#"
            UPDATE tasks SET progress = ?1, current_step = ?2, updated_at = ?3
            WHERE id = ?4
        "#, params![
            progress,
            current_step.map(|s| s as i32),
            &now,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Sonuç kaydet
    pub async fn set_task_result(&self, task_id: Uuid, result: &serde_json::Value, status: PersistedStatus) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        let result_str = serde_json::to_string(result).map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        conn.execute(r#"
            UPDATE tasks SET result = ?1, status = ?2, updated_at = ?3, completed_at = ?3
            WHERE id = ?4
        "#, params![
            &result_str,
            status.as_str(),
            &now,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Hata kaydet
    pub async fn set_task_error(&self, task_id: Uuid, error: &str) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(r#"
            UPDATE tasks SET error = ?1, status = 'failed', updated_at = ?2, completed_at = ?2
            WHERE id = ?3
        "#, params![
            error,
            &now,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Checkpoint kaydet
    pub async fn save_checkpoint(&self, task_id: Uuid, checkpoint: &serde_json::Value) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        let checkpoint_str = serde_json::to_string(checkpoint).map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        conn.execute(r#"
            UPDATE tasks SET checkpoint = ?1, updated_at = ?2
            WHERE id = ?3
        "#, params![
            &checkpoint_str,
            &now,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let cp_id = Uuid::new_v4();
        conn.execute(r#"
            INSERT INTO checkpoints (id, task_id, checkpoint_type, data, created_at)
            VALUES (?1, ?2, 'auto', ?3, ?4)
        "#, params![
            cp_id.to_string(),
            task_id.to_string(),
            &checkpoint_str,
            &now,
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::debug!("💾 Checkpoint kaydedildi: {}", task_id);
        Ok(())
    }
    
    /// Ajan ata
    pub async fn assign_agent(&self, task_id: Uuid, agent: &str) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        
        conn.execute(r#"
            UPDATE tasks SET assigned_agent = ?1, updated_at = ?2
            WHERE id = ?3
        "#, params![
            agent,
            &now,
            task_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Görev getir
    pub async fn get_task(&self, task_id: Uuid) -> StorageResult<Option<PersistedTask>> {
        let conn = self.conn.read().await;
        let result = conn.query_row(
            "SELECT * FROM tasks WHERE id = ?1",
            params![task_id.to_string()],
            |row| PersistedTask::from_row(row)
        ).optional().map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(result)
    }
    
    /// Aktif görevleri getir (yeniden başlatma için)
    pub async fn get_active_tasks(&self) -> StorageResult<Vec<PersistedTask>> {
        let conn = self.conn.read().await;
        let mut stmt = conn.prepare(
            r#"SELECT * FROM tasks WHERE status IN ('queued', 'starting', 'running', 'paused', 'waiting') ORDER BY priority DESC, started_at ASC"#
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let tasks = stmt.query_map([], |row| PersistedTask::from_row(row))
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::info!("📋 {} aktif görev bulundu", tasks.len());
        Ok(tasks)
    }
    
    /// Son N görevi getir
    pub async fn get_recent_tasks(&self, limit: usize) -> StorageResult<Vec<PersistedTask>> {
        let conn = self.conn.read().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM tasks ORDER BY updated_at DESC LIMIT ?1"
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let tasks = stmt.query_map(params![limit as i32], |row| PersistedTask::from_row(row))
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(tasks)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // STEP OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    /// Adım ekle
    pub async fn insert_step(&self, step: &PersistedStep) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let input_json = step.input.as_ref().and_then(|i| serde_json::to_string(i).ok());
        let output_json = step.output.as_ref().and_then(|o| serde_json::to_string(o).ok());
        
        conn.execute(r#"
            INSERT INTO task_steps (
                id, task_id, step_number, agent, action, status,
                input, output, started_at, completed_at, tokens_used
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        "#, params![
            step.id.to_string(),
            step.task_id.to_string(),
            step.step_number as i32,
            &step.agent,
            &step.action,
            &step.status,
            input_json,
            output_json,
            &step.started_at,
            &step.completed_at,
            step.tokens_used as i64,
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Görev adımlarını getir
    pub async fn get_task_steps(&self, task_id: Uuid) -> StorageResult<Vec<PersistedStep>> {
        let conn = self.conn.read().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM task_steps WHERE task_id = ?1 ORDER BY step_number ASC"
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let steps = stmt.query_map(params![task_id.to_string()], |row| PersistedStep::from_row(row))
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(steps)
    }
    
    /// Adımı tamamla
    pub async fn complete_step(&self, step_id: Uuid, output: &serde_json::Value, tokens: u64) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let now = Utc::now().to_rfc3339();
        let output_str = serde_json::to_string(output).map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        conn.execute(r#"
            UPDATE task_steps SET status = 'completed', output = ?1, completed_at = ?2, tokens_used = ?3
            WHERE id = ?4
        "#, params![
            &output_str,
            &now,
            tokens as i64,
            step_id.to_string(),
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════
    // LOG OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    /// Log ekle
    pub async fn add_log(&self, task_id: Uuid, level: &str, source: &str, message: &str) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let log_id = Uuid::new_v4();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(r#"
            INSERT INTO task_logs (id, task_id, timestamp, level, source, message)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#, params![
            log_id.to_string(),
            task_id.to_string(),
            &now,
            level,
            source,
            message,
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Görev loglarını getir
    pub async fn get_task_logs(&self, task_id: Uuid, limit: usize) -> StorageResult<Vec<TaskLogEntry>> {
        let conn = self.conn.read().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM task_logs WHERE task_id = ?1 ORDER BY timestamp DESC LIMIT ?2"
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let logs = stmt.query_map(params![task_id.to_string(), limit as i32], |row| TaskLogEntry::from_row(row))
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(logs)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // WORKFLOW OPERATIONS
    // ═══════════════════════════════════════════════════════════════
    
    /// Workflow kaydet
    pub async fn insert_workflow(&self, workflow: &WorkflowState) -> StorageResult<()> {
        let conn = self.conn.read().await;
        let deps = serde_json::to_string(&workflow.dependencies).unwrap_or_else(|_| "[]".into());
        let params = serde_json::to_string(&workflow.params).unwrap_or_else(|_| "{}".into());
        
        conn.execute(r#"
            INSERT INTO workflows (id, name, workflow_type, status, current_task_id, dependencies, params, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#, params![
            workflow.id.to_string(),
            &workflow.name,
            &workflow.workflow_type,
            &workflow.status,
            workflow.current_task_id.map(|id| id.to_string()),
            &deps,
            &params,
            &workflow.created_at,
            &workflow.updated_at,
        ]).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Aktif workflow'ları getir
    pub async fn get_active_workflows(&self) -> StorageResult<Vec<WorkflowState>> {
        let conn = self.conn.read().await;
        let mut stmt = conn.prepare(
            r#"SELECT * FROM workflows WHERE status NOT IN ('completed', 'failed', 'cancelled')"#
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let workflows = stmt.query_map([], |row| WorkflowState::from_row(row))
            .map_err(|e| StorageError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(workflows)
    }
    
    // ═══════════════════════════════════════════════════════════════
    // STATS & UTILITIES
    // ═══════════════════════════════════════════════════════════════
    
    /// İstatistikleri getir
    pub async fn get_stats(&self) -> StorageResult<TaskStats> {
        let conn = self.conn.read().await;
        
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks", [], |row| row.get(0)
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let active: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE status IN ('queued', 'starting', 'running', 'paused', 'waiting')", [], |row| row.get(0)
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let completed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE status = 'completed'", [], |row| row.get(0)
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        let failed: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE status = 'failed'", [], |row| row.get(0)
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        Ok(TaskStats { total, active, completed, failed })
    }
    
    /// Eski kayıtları temizle
    pub async fn cleanup(&self, days: i64) -> StorageResult<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        let cutoff_str = cutoff.to_rfc3339();
        
        let conn = self.conn.read().await;
        let result = conn.execute(
            "DELETE FROM tasks WHERE status IN ('completed', 'failed', 'cancelled', 'timeout') AND completed_at < ?1",
            params![&cutoff_str]
        ).map_err(|e| StorageError::QueryError(e.to_string()))?;
        
        log::info!("🗑️  {} günden eski {} kayıt temizlendi", days, result);
        Ok(result as u64)
    }
}

/// Görev istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub total: i64,
    pub active: i64,
    pub completed: i64,
    pub failed: i64,
}
