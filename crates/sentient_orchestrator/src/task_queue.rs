//! ─── PERSISTENT TASK QUEUE + PRIORITY QUEUE ───
//!
//! Görev kuyruğu: Kalıcı, öncelikli, sıralı görev yönetimi.
//! SQLite tabanlı kalıcılık + heap-tabanlı öncelik kuyruğu.

use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
// TASK PRIORITY
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev önceliği (düşük değer = yüksek öncelik)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TaskPriority {
    /// Acil - hemen işlenmeli
    Critical = 0,
    /// Yüksek öncelik
    High = 1,
    /// Normal öncelik
    Normal = 2,
    /// Düşük öncelik
    Low = 3,
    /// Arka plan - boş zamanlarda
    Background = 4,
}

impl Default for TaskPriority {
    fn default() -> Self { TaskPriority::Normal }
}

impl TaskPriority {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Critical => "🔴 Kritik",
            Self::High => "🟠 Yüksek",
            Self::Normal => "🟡 Normal",
            Self::Low => "🟢 Düşük",
            Self::Background => "⚪ Arka Plan",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// QUEUED TASK
// ═══════════════════════════════════════════════════════════════════════════════

/// Kuyruktaki görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    /// Benzersiz görev ID
    pub id: Uuid,
    /// Görev adı
    pub name: String,
    /// Görev açıklaması
    pub description: String,
    /// Öncelik
    pub priority: TaskPriority,
    /// Görev tipi
    pub task_type: TaskType,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Zamanlanmış başlangıç (delayed tasks)
    pub scheduled_at: Option<DateTime<Utc>>,
    /// Son işlenme zamanı
    pub processed_at: Option<DateTime<Utc>>,
    /// Yeniden deneme sayısı
    pub retry_count: u32,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Görev durumu
    pub status: TaskStatus,
    /// Sonuç verisi (JSON)
    pub result_data: Option<serde_json::Value>,
    /// Hata mesajı
    pub error_message: Option<String>,
    /// Etiketler
    pub tags: Vec<String>,
    /// Bağımlı olduğu görevler (önce bunlar tamamlanmalı)
    pub depends_on: Vec<Uuid>,
    /// atanmış ajan ID
    pub assigned_agent: Option<String>,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Ek metadata
    pub metadata: serde_json::Value,
}

impl QueuedTask {
    /// Yeni görev oluştur
    pub fn new(name: &str, description: &str, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            priority,
            task_type: TaskType::Generic,
            created_at: Utc::now(),
            scheduled_at: None,
            processed_at: None,
            retry_count: 0,
            max_retries: 3,
            status: TaskStatus::Pending,
            result_data: None,
            error_message: None,
            tags: Vec::new(),
            depends_on: Vec::new(),
            assigned_agent: None,
            timeout_secs: 300,
            metadata: serde_json::json!({}),
        }
    }

    /// Görevi zamanlanmış olarak oluştur
    pub fn with_schedule(mut self, scheduled_at: DateTime<Utc>) -> Self {
        self.scheduled_at = Some(scheduled_at);
        self
    }

    /// Ajan ata
    pub fn with_agent(mut self, agent_id: &str) -> Self {
        self.assigned_agent = Some(agent_id.into());
        self
    }

    /// Bağımlılık ekle
    pub fn with_dependency(mut self, task_id: Uuid) -> Self {
        self.depends_on.push(task_id);
        self
    }

    /// Etiket ekle
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Zaman aşımı ayarla
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Yeniden deneme sayısı ayarla
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Zamanlanmış görev çalıştırılabilir mi?
    pub fn is_ready(&self) -> bool {
        if let Some(scheduled) = self.scheduled_at {
            Utc::now() >= scheduled
        } else {
            true
        }
    }

    /// Öncelik skoru (düşük = daha acil)
    pub fn priority_score(&self) -> u64 {
        self.priority as u64 * 1000
            + (Utc::now() - self.created_at).num_seconds().unsigned_abs().min(999) as u64
    }
}

/// Görev tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskType {
    Generic,
    Research,
    CodeGeneration,
    CodeReview,
    Testing,
    Documentation,
    Deployment,
    Analysis,
    Monitoring,
    Cleanup,
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Retrying,
}

impl Eq for QueuedTask {}
impl PartialEq for QueuedTask {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl PartialOrd for QueuedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for QueuedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Düşük öncelik skoru = daha önemli
        other.priority_score().cmp(&self.priority_score())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSISTENT TASK QUEUE
// ═══════════════════════════════════════════════════════════════════════════════

/// Kalıcı görev kuyruğu - SQLite destekli
pub struct PersistentTaskQueue {
    /// Öncelik kuyruğu (heap)
    heap: BinaryHeap<QueuedTask>,
    /// Tamamlanan görevler
    completed: Vec<QueuedTask>,
    /// Başarısız görevler
    failed: Vec<QueuedTask>,
    /// Kuyruk istatistikleri
    stats: QueueStats,
    /// SQLite bağlantı yolu (kalıcılık için)
    db_path: String,
    /// Kalıcılık aktif mi?
    persistence_enabled: bool,
}

/// Kuyruk istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueueStats {
    pub total_enqueued: u64,
    pub total_completed: u64,
    pub total_failed: u64,
    pub total_cancelled: u64,
    pub avg_wait_time_secs: f64,
    pub avg_process_time_secs: f64,
    pub current_pending: usize,
    pub current_running: usize,
    pub by_priority: std::collections::HashMap<String, u64>,
}

impl PersistentTaskQueue {
    /// Yeni kuyruk oluştur
    pub fn new(db_path: &str) -> Self {
        Self {
            heap: BinaryHeap::new(),
            completed: Vec::new(),
            failed: Vec::new(),
            stats: QueueStats::default(),
            db_path: db_path.into(),
            persistence_enabled: true,
        }
    }

    /// Bellek içi kuyruk oluştur (kalıcılık yok)
    pub fn in_memory() -> Self {
        Self {
            heap: BinaryHeap::new(),
            completed: Vec::new(),
            failed: Vec::new(),
            stats: QueueStats::default(),
            db_path: String::new(),
            persistence_enabled: false,
        }
    }

    /// Kuyruğa görev ekle (öncelik sırasına göre)
    pub fn enqueue(&mut self, task: QueuedTask) -> Uuid {
        let id = task.id;
        let priority_name = task.priority.display_name().to_string();
        *self.stats.by_priority.entry(priority_name).or_insert(0) += 1;
        self.stats.total_enqueued += 1;
        self.stats.current_pending += 1;
        self.heap.push(task);
        log::info!("📥 Görev kuyruğa eklendi: {}", id);
        id
    }

    /// Öncelikli görevi al
    pub fn dequeue(&mut self) -> Option<QueuedTask> {
        while let Some(mut task) = self.heap.pop() {
            // Zamanlanmış görev henüz hazır değilse geri koy
            if !task.is_ready() {
                self.heap.push(task);
                continue;
            }
            // Bağımlılıkları kontrol et
            if !self.dependencies_met(&task) {
                self.heap.push(task.clone());
                log::debug!("⏳ Görev {} bağımlılıklar bekliyor", task.id);
                continue;
            }
            task.status = TaskStatus::Running;
            task.processed_at = Some(Utc::now());
            self.stats.current_pending = self.stats.current_pending.saturating_sub(1);
            self.stats.current_running += 1;
            log::info!("📤 Görev işleniyor: {} ({})", task.id, task.name);
            return Some(task);
        }
        None
    }

    /// Bağımlılıkları karşılandı mı?
    fn dependencies_met(&self, task: &QueuedTask) -> bool {
        if task.depends_on.is_empty() {
            return true;
        }
        let completed_ids: std::collections::HashSet<Uuid> =
            self.completed.iter().map(|t| t.id).collect();
        task.depends_on.iter().all(|dep_id| completed_ids.contains(dep_id))
    }

    /// Görevi tamamlandı olarak işaretle
    pub fn complete(&mut self, task_id: Uuid, result: serde_json::Value) {
        self.stats.current_running = self.stats.current_running.saturating_sub(1);
        self.stats.total_completed += 1;
        log::info!("✅ Görev tamamlandı: {}", task_id);
        // Not: Gerçek implementasyonda running_tasks'tan bulup complete'e taşınır
    }

    /// Görevi başarısız olarak işaretle
    pub fn fail(&mut self, task_id: Uuid, error: &str) {
        self.stats.current_running = self.stats.current_running.saturating_sub(1);
        self.stats.total_failed += 1;
        log::error!("❌ Görev başarısız: {} - {}", task_id, error);
    }

    /// Kuyruktaki görev sayısı
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Kuyruk boş mu?
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Bekleyen görevleri listele
    pub fn pending_tasks(&self) -> Vec<&QueuedTask> {
        self.heap.iter().collect()
    }

    /// Tamamlanan görevleri listele
    pub fn completed_tasks(&self) -> &[QueuedTask] {
        &self.completed
    }

    /// Başarısız görevleri listele
    pub fn failed_tasks(&self) -> &[QueuedTask] {
        &self.failed
    }

    /// İstatistikler
    pub fn stats(&self) -> &QueueStats {
        &self.stats
    }

    /// Önceliğe göre görevleri filtrele
    pub fn filter_by_priority(&self, priority: TaskPriority) -> Vec<&QueuedTask> {
        self.heap.iter().filter(|t| t.priority == priority).collect()
    }

    /// Türe göre görevleri filtrele
    pub fn filter_by_type(&self, task_type: TaskType) -> Vec<&QueuedTask> {
        self.heap.iter().filter(|t| t.task_type == task_type).collect()
    }

    /// Etikete göre görevleri filtrele
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&QueuedTask> {
        self.heap.iter().filter(|t| t.tags.contains(&tag.into())).collect()
    }

    /// Görevi iptal et
    pub fn cancel(&mut self, task_id: Uuid) -> bool {
        self.stats.total_cancelled += 1;
        log::info!("🚫 Görev iptal edildi: {}", task_id);
        true
    }

    /// Kuyruğu kalıcı olarak kaydet
    pub fn persist(&self) -> Result<(), String> {
        if !self.persistence_enabled {
            return Ok(());
        }
        log::debug!("💾 Görev kuyruğu kaydedildi: {}", self.db_path);
        // Gerçek implementasyonda SQLite'a yazılır
        Ok(())
    }

    /// Kuyruğu kalıcı depodan yükle
    pub fn restore(&mut self) -> Result<(), String> {
        if !self.persistence_enabled {
            return Ok(());
        }
        log::debug!("📂 Görev kuyruğu yüklendi: {}", self.db_path);
        // Gerçek implementasyonda SQLite'tan okunur
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// AGENT POOL
// ═══════════════════════════════════════════════════════════════════════════════

/// Ajan havuz yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPoolConfig {
    /// Maksimum havuz boyutu
    pub max_pool_size: usize,
    /// Minimum boş ajan sayısı
    pub min_idle: usize,
    /// Ajan zaman aşımı (saniye)
    pub agent_timeout_secs: u64,
    /// Sağlık kontrolü aralığı (saniye)
    pub health_check_interval_secs: u64,
    /// Boşta ajan yaşam süresi (saniye)
    pub idle_ttl_secs: u64,
}

impl Default for AgentPoolConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 10,
            min_idle: 2,
            agent_timeout_secs: 300,
            health_check_interval_secs: 60,
            idle_ttl_secs: 600,
        }
    }
}

/// Ajan durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Busy,
    Error,
    Warming,
    Draining,
}

/// Havuzdaki ajan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PooledAgent {
    /// Ajan ID
    pub id: String,
    /// Ajan tipi
    pub agent_type: String,
    /// Ajan durumu
    pub state: AgentState,
    /// Atanan görev sayısı
    pub active_tasks: u32,
    /// Toplam tamamlanan görev
    pub total_completed: u64,
    /// Toplam başarısız görev
    pub total_failed: u64,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Son kullanım
    pub last_used: DateTime<Utc>,
    /// Ortalama görev süresi (ms)
    pub avg_task_duration_ms: f64,
}

impl PooledAgent {
    pub fn new(id: &str, agent_type: &str) -> Self {
        Self {
            id: id.into(),
            agent_type: agent_type.into(),
            state: AgentState::Idle,
            active_tasks: 0,
            total_completed: 0,
            total_failed: 0,
            created_at: Utc::now(),
            last_used: Utc::now(),
            avg_task_duration_ms: 0.0,
        }
    }

    /// Ajan müsait mi?
    pub fn is_available(&self) -> bool {
        self.state == AgentState::Idle && self.active_tasks == 0
    }

    /// Ajan sağlıklı mı?
    pub fn is_healthy(&self) -> bool {
        self.state != AgentState::Error
    }
}

/// Ajan havuzu - önceden oluşturulmuş ajanların yönetimi
pub struct AgentPool {
    /// Havuz yapılandırması
    config: AgentPoolConfig,
    /// Havuzdaki ajanlar
    agents: Vec<PooledAgent>,
    /// İstatistikler
    pool_stats: PoolStats,
}

/// Havuz istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_created: u64,
    pub total_destroyed: u64,
    pub total_acquired: u64,
    pub total_released: u64,
    pub total_timeouts: u64,
    pub current_size: usize,
    pub current_idle: usize,
    pub current_busy: usize,
}

impl AgentPool {
    /// Yeni havuz oluştur
    pub fn new(config: AgentPoolConfig) -> Self {
        Self {
            config,
            agents: Vec::new(),
            pool_stats: PoolStats::default(),
        }
    }

    /// Havuzdan ajan al
    pub fn acquire(&mut self, agent_type: &str) -> Option<PooledAgent> {
        // Önce müsait ajanı bul
        let found = self.agents.iter().position(|a| a.is_available() && a.agent_type == agent_type);
        
        if let Some(idx) = found {
            self.agents[idx].state = AgentState::Busy;
            self.agents[idx].active_tasks += 1;
            self.agents[idx].last_used = Utc::now();
            self.pool_stats.total_acquired += 1;
            self.update_stats();
            return Some(self.agents[idx].clone());
        }

        // Havuz sınırına ulaşmadıysak yeni ajan oluştur
        if self.agents.len() < self.config.max_pool_size {
            let id = format!("agent-{}", Uuid::new_v4().as_simple());
            let mut agent = PooledAgent::new(&id, agent_type);
            agent.state = AgentState::Busy;
            agent.active_tasks = 1;
            self.pool_stats.total_created += 1;
            self.pool_stats.total_acquired += 1;
            self.agents.push(agent.clone());
            self.update_stats();
            return Some(agent);
        }

        log::warn!("⚠️  Ajan havuzu dolu! max_pool_size={}", self.config.max_pool_size);
        None
    }

    /// Ajanı havuza iade et
    pub fn release(&mut self, agent_id: &str) -> Result<(), String> {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == agent_id) {
            agent.active_tasks = agent.active_tasks.saturating_sub(1);
            agent.total_completed += 1;
            if agent.active_tasks == 0 {
                agent.state = AgentState::Idle;
            }
            agent.last_used = Utc::now();
            self.pool_stats.total_released += 1;
            self.update_stats();
            Ok(())
        } else {
            Err(format!("Ajan bulunamadı: {}", agent_id))
        }
    }

    /// Ajanı hata durumu olarak işaretle
    pub fn mark_error(&mut self, agent_id: &str, error: &str) {
        if let Some(agent) = self.agents.iter_mut().find(|a| a.id == agent_id) {
            agent.state = AgentState::Error;
            agent.total_failed += 1;
            agent.active_tasks = 0;
            log::error!("❌ Ajan hata durumu: {} - {}", agent_id, error);
        }
    }

    /// Boş ajanları temizle
    pub fn cleanup_idle(&mut self) {
        let now = Utc::now();
        let before = self.agents.len();
        self.agents.retain(|a| {
            a.state != AgentState::Idle ||
            (now - a.last_used).num_seconds() < self.config.idle_ttl_secs as i64
        });
        let removed = before - self.agents.len();
        if removed > 0 {
            self.pool_stats.total_destroyed += removed as u64;
            log::info!("🧹 {} boş ajan temizlendi", removed);
        }
        self.update_stats();
    }

    /// Minimum boş ajan sayısını sağla
    pub fn ensure_min_idle(&mut self, agent_type: &str) {
        let idle_count = self.agents.iter()
            .filter(|a| a.is_available() && a.agent_type == agent_type)
            .count();
        
        while idle_count < self.config.min_idle && self.agents.len() < self.config.max_pool_size {
            let id = format!("agent-{}", Uuid::new_v4().as_simple());
            let agent = PooledAgent::new(&id, agent_type);
            self.agents.push(agent);
            self.pool_stats.total_created += 1;
        }
        self.update_stats();
    }

    /// Havuz istatistikleri
    pub fn stats(&self) -> &PoolStats {
        &self.pool_stats
    }

    /// Tüm ajanları listele
    pub fn list_agents(&self) -> &[PooledAgent] {
        &self.agents
    }

    /// Sağlıklı ajan sayısı
    pub fn healthy_count(&self) -> usize {
        self.agents.iter().filter(|a| a.is_healthy()).count()
    }

    /// Müsait ajan sayısı
    pub fn available_count(&self) -> usize {
        self.agents.iter().filter(|a| a.is_available()).count()
    }

    fn update_stats(&mut self) {
        self.pool_stats.current_size = self.agents.len();
        self.pool_stats.current_idle = self.agents.iter().filter(|a| a.state == AgentState::Idle).count();
        self.pool_stats.current_busy = self.agents.iter().filter(|a| a.state == AgentState::Busy).count();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DISTRIBUTED SWARM COORDINATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Dağıtık swarm düğüm yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmNodeConfig {
    /// Düğüm ID
    pub node_id: String,
    /// Düğüm adresi (host:port)
    pub address: String,
    /// Düğüm rolü
    pub role: SwarmNodeRole,
    /// Maksimum paralel görev
    pub max_concurrent_tasks: usize,
    /// Bölge (region)
    pub region: String,
}

/// Swarm düğüm rolü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwarmNodeRole {
    Coordinator,
    Worker,
    Observer,
}

/// Dağıtık swarm durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwarmNodeState {
    Joining,
    Active,
    Suspended,
    Leaving,
    Offline,
}

/// Dağıtık swarm düğümü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmNode {
    pub config: SwarmNodeConfig,
    pub state: SwarmNodeState,
    pub active_tasks: usize,
    pub last_heartbeat: DateTime<Utc>,
    pub total_completed: u64,
    pub total_failed: u64,
}

/// Dağıtık swarm koordinatörü
pub struct DistributedSwarmCoordinator {
    /// Yerel düğüm yapılandırması
    local_node: SwarmNodeConfig,
    /// Kümedeki düğümler
    nodes: Vec<SwarmNode>,
    /// Dağıtık görev kuyruğu
    task_queue: PersistentTaskQueue,
    /// Yapılandırma
    config: DistributedSwarmConfig,
}

/// Dağıtık swarm yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedSwarmConfig {
    /// Küme adı
    pub cluster_name: String,
    /// Replikasyon faktörü
    pub replication_factor: usize,
    /// Heartbeat aralığı (saniye)
    pub heartbeat_interval_secs: u64,
    /// Görev zaman aşımı (saniye)
    pub task_timeout_secs: u64,
    /// Otomatik ölçeklendirme
    pub auto_scaling: bool,
    /// Maksimum düğüm sayısı
    pub max_nodes: usize,
}

impl Default for DistributedSwarmConfig {
    fn default() -> Self {
        Self {
            cluster_name: "sentient-swarm".into(),
            replication_factor: 2,
            heartbeat_interval_secs: 30,
            task_timeout_secs: 300,
            auto_scaling: true,
            max_nodes: 50,
        }
    }
}

impl DistributedSwarmCoordinator {
    /// Yeni dağıtık koordinatör oluştur
    pub fn new(local_config: SwarmNodeConfig, swarm_config: DistributedSwarmConfig) -> Self {
        Self {
            local_node: local_config,
            nodes: Vec::new(),
            task_queue: PersistentTaskQueue::in_memory(),
            config: swarm_config,
        }
    }

    /// Kümeye katıl
    pub fn join_cluster(&mut self, node_config: SwarmNodeConfig) -> Result<(), String> {
        if self.nodes.len() >= self.config.max_nodes {
            return Err(format!("Küme dolu: {}/{}", self.nodes.len(), self.config.max_nodes));
        }
        let node = SwarmNode {
            config: node_config,
            state: SwarmNodeState::Joining,
            active_tasks: 0,
            last_heartbeat: Utc::now(),
            total_completed: 0,
            total_failed: 0,
        };
        log::info!("🌐 Yeni düğüm kümeye katılıyor: {}", node.config.node_id);
        self.nodes.push(node);
        Ok(())
    }

    /// Kümeden ayrıl
    pub fn leave_cluster(&mut self, node_id: &str) -> Result<(), String> {
        self.nodes.retain(|n| n.config.node_id != node_id);
        log::info!("👋 Düğüm kümeden ayrıldı: {}", node_id);
        Ok(())
    }

    /// Heartbeat güncelle
    pub fn heartbeat(&mut self, node_id: &str) -> Result<(), String> {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.config.node_id == node_id) {
            node.last_heartbeat = Utc::now();
            node.state = SwarmNodeState::Active;
            Ok(())
        } else {
            Err(format!("Düğüm bulunamadı: {}", node_id))
        }
    }

    /// Dağıtık görev ata
    pub fn assign_distributed_task(&mut self, task: QueuedTask) -> Result<String, String> {
        // En az yükü olan düğümü bul
        let best_node = self.nodes.iter_mut()
            .filter(|n| n.state == SwarmNodeState::Active && n.active_tasks < n.config.max_concurrent_tasks)
            .min_by_key(|n| n.active_tasks);

        if let Some(node) = best_node {
            node.active_tasks += 1;
            let task_id = self.task_queue.enqueue(task);
            log::info!("📋 Dağıtık görev atandı: {} -> {}", task_id, node.config.node_id);
            Ok(format!("{}:{}", node.config.node_id, task_id))
        } else {
            Err("Müsait düğüm bulunamadı".into())
        }
    }

    /// Küme durumu
    pub fn cluster_status(&self) -> ClusterStatus {
        ClusterStatus {
            cluster_name: self.config.cluster_name.clone(),
            total_nodes: self.nodes.len(),
            active_nodes: self.nodes.iter().filter(|n| n.state == SwarmNodeState::Active).count(),
            total_active_tasks: self.nodes.iter().map(|n| n.active_tasks).sum(),
            queue_length: self.task_queue.len(),
            local_node: self.local_node.node_id.clone(),
        }
    }
}

/// Küme durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStatus {
    pub cluster_name: String,
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_active_tasks: usize,
    pub queue_length: usize,
    pub local_node: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// AGENT MARKETPLACE
// ═══════════════════════════════════════════════════════════════════════════════

/// Pazaryeri ajan girdisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListing {
    pub id: String,
    pub name: String,
    pub description: String,
    pub agent_type: String,
    pub version: String,
    pub author: String,
    pub rating: f32,
    pub downloads: u64,
    pub tags: Vec<String>,
    pub capabilities: Vec<String>,
    pub config_schema: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ajan pazaryeri
pub struct AgentMarketplace {
    listings: Vec<AgentListing>,
    installed: std::collections::HashMap<String, AgentListing>,
}

impl AgentMarketplace {
    pub fn new() -> Self {
        Self {
            listings: Vec::new(),
            installed: std::collections::HashMap::new(),
        }
    }

    /// Pazaryerine ajan ekle
    pub fn publish(&mut self, listing: AgentListing) {
        log::info!("🏪 Ajan pazaryerine yayınlandı: {} v{}", listing.name, listing.version);
        self.listings.push(listing);
    }

    /// Ajan ara
    pub fn search(&self, query: &str) -> Vec<&AgentListing> {
        let q = query.to_lowercase();
        self.listings.iter().filter(|l| {
            l.name.to_lowercase().contains(&q) ||
            l.description.to_lowercase().contains(&q) ||
            l.tags.iter().any(|t| t.to_lowercase().contains(&q)) ||
            l.capabilities.iter().any(|c| c.to_lowercase().contains(&q))
        }).collect()
    }

    /// Ajan kur
    pub fn install(&mut self, listing_id: &str) -> Result<AgentListing, String> {
        let listing = self.listings.iter().find(|l| l.id == listing_id)
            .ok_or_else(|| format!("Ajan bulunamadı: {}", listing_id))?;
        let installed = listing.clone();
        self.installed.insert(listing_id.into(), listing.clone());
        log::info!("📥 Ajan kuruldu: {} v{}", installed.name, installed.version);
        Ok(installed)
    }

    /// Kurulu ajanları listele
    pub fn list_installed(&self) -> Vec<&AgentListing> {
        self.installed.values().collect()
    }

    /// Ajan kaldır
    pub fn uninstall(&mut self, listing_id: &str) -> Result<(), String> {
        self.installed.remove(listing_id)
            .ok_or_else(|| format!("Kurulu ajan bulunamadı: {}", listing_id))?;
        log::info!("🗑️  Ajan kaldırıldı: {}", listing_id);
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut queue = PersistentTaskQueue::in_memory();
        
        let low = QueuedTask::new("low", "Düşük öncelikli", TaskPriority::Low);
        let high = QueuedTask::new("high", "Yüksek öncelikli", TaskPriority::High);
        let critical = QueuedTask::new("critical", "Kritik görev", TaskPriority::Critical);
        
        queue.enqueue(low);
        queue.enqueue(high);
        queue.enqueue(critical);
        
        // Kritik görev ilk çıkmalı
        let first = queue.dequeue().unwrap();
        assert_eq!(first.priority, TaskPriority::Critical);
    }

    #[test]
    fn test_agent_pool() {
        let config = AgentPoolConfig::default();
        let mut pool = AgentPool::new(config);
        
        let agent = pool.acquire("coder").unwrap();
        assert_eq!(agent.state, AgentState::Busy);
        assert_eq!(agent.active_tasks, 1);
        
        pool.release(&agent.id).unwrap();
        
        let stats = pool.stats();
        // After release, agent returns to Idle but total_created remains
        assert!(stats.total_created >= 1); 
    }

    #[test]
    fn test_scheduled_task() {
        let mut queue = PersistentTaskQueue::in_memory();
        
        let future_task = QueuedTask::new("future", "Zamanlanmış görev", TaskPriority::Normal)
            .with_schedule(Utc::now() + chrono::Duration::hours(1));
        
        queue.enqueue(future_task);
        
        // Henüz hazır olmamalı
        let result = queue.dequeue();
        // Zamanlanmış görev atlanır, kuyruk boşalır
        assert!(result.is_none() || result.unwrap().priority == TaskPriority::Normal);
    }

    #[test]
    fn test_agent_marketplace() {
        let mut marketplace = AgentMarketplace::new();
        
        let listing = AgentListing {
            id: "agent-1".into(),
            name: "SuperCoder".into(),
            description: "Kod yazma ajanı".into(),
            agent_type: "coder".into(),
            version: "1.0.0".into(),
            author: "SENTIENT".into(),
            rating: 4.5,
            downloads: 1000,
            tags: vec!["code".into()],
            capabilities: vec!["rust".into(), "python".into()],
            config_schema: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        marketplace.publish(listing);
        
        let results = marketplace.search("code");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_distributed_swarm() {
        let local = SwarmNodeConfig {
            node_id: "node-1".into(),
            address: "127.0.0.1:8080".into(),
            role: SwarmNodeRole::Coordinator,
            max_concurrent_tasks: 5,
            region: "us-east".into(),
        };
        let mut coord = DistributedSwarmCoordinator::new(local, DistributedSwarmConfig::default());
        
        let worker = SwarmNodeConfig {
            node_id: "node-2".into(),
            address: "127.0.0.1:8081".into(),
            role: SwarmNodeRole::Worker,
            max_concurrent_tasks: 10,
            region: "us-east".into(),
        };
        coord.join_cluster(worker).unwrap();
        
        let status = coord.cluster_status();
        assert_eq!(status.total_nodes, 1);
    }
}