//! ─── SENTIENT STORAGE - TASK PERSISTENCE LAYER ───
//!
//! Görev sürekliliği ve veritabanı kalıcılığı:
//! - SQLite ile görev durumları saklama
//! - Sunucu yeniden başlatıldığında otomatik görev devralma (Hydration)
//! - Aktif iş akışı takibi
//! - Ajan aşama durumları
//!
//! # Mimarİ
//! ```text
//! ┌─────────────────┐     ┌──────────────────┐
//! │   TaskManager    │────▶│   TaskStore      │
//! │  (Gateway)       │     │   (SQLite)       │
//! └─────────────────┘     └──────────────────┘
//!         │                        │
//!         ▼                        ▼
//! ┌─────────────────┐     ┌──────────────────┐
//! │   Hydration     │◀────│   Task Tables     │
//! │   (Restore)     │     │   - tasks         │
//! └─────────────────┘     │   - task_steps    │
//!                         │   - task_logs     │
//!                         │   - workflows     │
//!                         │   - checkpoints   │
//!                         └──────────────────┘
//! ```

pub mod store;
pub mod hydration;
pub mod models;

// Re-exports
pub use store::TaskStore;
pub use hydration::HydrationEngine;
pub use models::{
    PersistedTask, PersistedStep, PersistedStatus, TaskSnapshot, WorkflowState,
    TaskLogEntry, StorageError, StorageResult,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
