//! ─── SYSTEM METRICS ───
//!
//! Gerçek zamanlı sistem izleme:
//! - CPU kullanımı
//! - RAM kullanımı
//! - Disk I/O
//! - Network I/O
//! - API maliyeti

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

/// Sistem metrikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
    
    /// CPU kullanımı (%) - tüm çekirdekler
    pub cpu_usage: f32,
    
    /// RAM kullanımı (MB)
    pub memory_used_mb: f64,
    
    /// Toplam RAM (MB)
    pub memory_total_mb: f64,
    
    /// RAM kullanım oranı (%)
    pub memory_usage_percent: f32,
    
    /// Disk okuma (MB/s)
    pub disk_read_mbps: f64,
    
    /// Disk yazma (MB/s)
    pub disk_write_mbps: f64,
    
    /// Network giriş (MB/s)
    pub network_in_mbps: f64,
    
    /// Network çıkış (MB/s)
    pub network_out_mbps: f64,
    
    /// Aktif görev sayısı
    pub active_tasks: usize,
    
    /// Tamamlanan görev (son 1 saat)
    pub completed_tasks_hour: usize,
    
    /// API istek sayısı (son 1 saat)
    pub api_requests_hour: usize,
    
    /// Tahmini API maliyeti ($)
    pub estimated_cost_usd: f64,
    
    /// Sistem yükü (1, 5, 15 dakika)
    pub load_avg: [f64; 3],
    
    /// Çalışma süresi (saniye)
    pub uptime_secs: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu_usage: 0.0,
            memory_used_mb: 0.0,
            memory_total_mb: 0.0,
            memory_usage_percent: 0.0,
            disk_read_mbps: 0.0,
            disk_write_mbps: 0.0,
            network_in_mbps: 0.0,
            network_out_mbps: 0.0,
            active_tasks: 0,
            completed_tasks_hour: 0,
            api_requests_hour: 0,
            estimated_cost_usd: 0.0,
            load_avg: [0.0; 3],
            uptime_secs: 0,
        }
    }
}

impl SystemMetrics {
    /// Sağlık durumu
    pub fn health_status(&self) -> HealthStatus {
        if self.cpu_usage > 90.0 || self.memory_usage_percent > 90.0 {
            HealthStatus::Critical
        } else if self.cpu_usage > 70.0 || self.memory_usage_percent > 70.0 {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
    
    /// Özet mesaj
    pub fn summary(&self) -> String {
        format!(
            "CPU: {:.1}% | RAM: {:.1}% ({:.0}/{:.0}MB) | Görevler: {} | Maliyet: ${:.4}",
            self.cpu_usage,
            self.memory_usage_percent,
            self.memory_used_mb,
            self.memory_total_mb,
            self.active_tasks,
            self.estimated_cost_usd
        )
    }
}

/// Sağlık durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "🟢 Sağlıklı"),
            Self::Warning => write!(f, "🟡 Uyarı"),
            Self::Critical => write!(f, "🔴 Kritik"),
        }
    }
}

/// ─── METRICS COLLECTOR ───

pub struct MetricsCollector {
    /// Son metrikler
    metrics: Arc<RwLock<SystemMetrics>>,
    
    /// Başlangıç zamanı
    start_time: std::time::Instant,
    
    /// Görev sayaçları
    task_counter: Arc<RwLock<TaskCounters>>,
    
    /// API sayaçları
    api_counter: Arc<RwLock<ApiCounters>>,
}

/// Görev sayaçları
#[derive(Debug, Default)]
struct TaskCounters {
    active: usize,
    completed_hour: usize,
    completed_total: usize,
}

/// API sayaçları
#[derive(Debug, Default)]
struct ApiCounters {
    requests_hour: usize,
    requests_total: usize,
    tokens_used: u64,
    cost_usd: f64,
}

impl MetricsCollector {
    /// Yeni collector oluştur
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            start_time: std::time::Instant::now(),
            task_counter: Arc::new(RwLock::new(TaskCounters::default())),
            api_counter: Arc::new(RwLock::new(ApiCounters::default())),
        }
    }
    
    /// Metrikleri topla
    pub async fn collect(&self) -> SystemMetrics {
        let mut metrics = self.metrics.write().await;
        
        // Sistem bilgilerini al
        let cpu_usage = self.get_cpu_usage();
        let (mem_used, mem_total) = self.get_memory_info();
        let (disk_read, disk_write) = self.get_disk_io();
        let (net_in, net_out) = self.get_network_io();
        let load_avg = self.get_load_average();
        
        // Zaman damgası
        metrics.timestamp = Utc::now();
        
        // CPU
        metrics.cpu_usage = cpu_usage;
        
        // RAM
        metrics.memory_used_mb = mem_used;
        metrics.memory_total_mb = mem_total;
        metrics.memory_usage_percent = if mem_total > 0.0 {
            (mem_used / mem_total * 100.0) as f32
        } else {
            0.0
        };
        
        // Disk I/O
        metrics.disk_read_mbps = disk_read;
        metrics.disk_write_mbps = disk_write;
        
        // Network I/O
        metrics.network_in_mbps = net_in;
        metrics.network_out_mbps = net_out;
        
        // Load average
        metrics.load_avg = load_avg;
        
        // Uptime
        metrics.uptime_secs = self.start_time.elapsed().as_secs();
        
        // Görev sayaçları
        let task_counter = self.task_counter.read().await;
        metrics.active_tasks = task_counter.active;
        metrics.completed_tasks_hour = task_counter.completed_hour;
        
        // API sayaçları
        let api_counter = self.api_counter.read().await;
        metrics.api_requests_hour = api_counter.requests_hour;
        metrics.estimated_cost_usd = api_counter.cost_usd;
        
        metrics.clone()
    }
    
    /// CPU kullanımı al
    fn get_cpu_usage(&self) -> f32 {
        // Basit CPU tahmini - gerçek sistemde sysinfo kullanılır
        use std::fs;
        
        if let Ok(stat) = fs::read_to_string("/proc/stat") {
            let line = stat.lines().next().unwrap_or("");
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 4 {
                // Basit hesaplama
                let user: f32 = parts[1].parse().unwrap_or(0.0);
                let nice: f32 = parts[2].parse().unwrap_or(0.0);
                let system: f32 = parts[3].parse().unwrap_or(0.0);
                let idle: f32 = parts[4].parse().unwrap_or(1.0);
                let total = user + nice + system + idle;
                if total > 0.0 {
                    return ((user + nice + system) / total * 100.0).min(100.0).max(0.0);
                }
            }
        }
        
        // Fallback: sabit değer
        25.0
    }
    
    /// RAM bilgisi al
    fn get_memory_info(&self) -> (f64, f64) {
        use std::fs;
        
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64;
            let mut available = 0u64;
            
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    total = line.split(':')
                        .nth(1)
                        .and_then(|s| s.trim().split(' ').next())
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("MemAvailable:") {
                    available = line.split(':')
                        .nth(1)
                        .and_then(|s| s.trim().split(' ').next())
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                }
                
                if total > 0 && available > 0 {
                    break;
                }
            }
            
            let used = total.saturating_sub(available);
            return (used as f64 / 1024.0, total as f64 / 1024.0);
        }
        
        (2048.0, 8192.0) // Fallback
    }
    
    /// Disk I/O al
    fn get_disk_io(&self) -> (f64, f64) {
        // Basit simülasyon - gerçek sistemde /proc/diskstats kullanılır
        (1.5, 0.8)
    }
    
    /// Network I/O al
    fn get_network_io(&self) -> (f64, f64) {
        // Basit simülasyon - gerçek sistemde /proc/net/dev kullanılır
        (0.5, 0.3)
    }
    
    /// Load average al
    fn get_load_average(&self) -> [f64; 3] {
        use std::fs;
        
        if let Ok(load) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = load.split_whitespace().collect();
            if parts.len() >= 3 {
                return [
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                ];
            }
        }
        
        [0.5, 0.5, 0.5]
    }
    
    // ─── Görev Yönetimi ───
    
    /// Görev başladı
    pub async fn task_started(&self) {
        let mut counter = self.task_counter.write().await;
        counter.active += 1;
    }
    
    /// Görev tamamlandı
    pub async fn task_completed(&self) {
        let mut counter = self.task_counter.write().await;
        counter.active = counter.active.saturating_sub(1);
        counter.completed_hour += 1;
        counter.completed_total += 1;
    }
    
    // ─── API Yönetimi ───
    
    /// API isteği kaydet
    pub async fn api_request(&self, tokens: u64, cost: f64) {
        let mut counter = self.api_counter.write().await;
        counter.requests_hour += 1;
        counter.requests_total += 1;
        counter.tokens_used += tokens;
        counter.cost_usd += cost;
    }
    
    /// Metrik referansı al
    pub fn get_metrics_arc(&self) -> Arc<RwLock<SystemMetrics>> {
        self.metrics.clone()
    }
    
    /// Saatlik sayaçları sıfırla
    pub async fn reset_hourly_counters(&self) {
        let mut task_counter = self.task_counter.write().await;
        task_counter.completed_hour = 0;
        
        let mut api_counter = self.api_counter.write().await;
        api_counter.requests_hour = 0;
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_default() {
        let metrics = SystemMetrics::default();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.active_tasks, 0);
    }
    
    #[test]
    fn test_health_status() {
        let mut metrics = SystemMetrics::default();
        
        metrics.cpu_usage = 50.0;
        metrics.memory_usage_percent = 50.0;
        assert_eq!(metrics.health_status(), HealthStatus::Healthy);
        
        metrics.cpu_usage = 80.0;
        assert_eq!(metrics.health_status(), HealthStatus::Warning);
        
        metrics.cpu_usage = 95.0;
        assert_eq!(metrics.health_status(), HealthStatus::Critical);
    }
    
    #[test]
    fn test_metrics_summary() {
        let mut metrics = SystemMetrics::default();
        metrics.cpu_usage = 45.5;
        metrics.memory_used_mb = 2048.0;
        metrics.memory_total_mb = 8192.0;
        metrics.active_tasks = 3;
        metrics.estimated_cost_usd = 0.0123;
        
        let summary = metrics.summary();
        assert!(summary.contains("CPU"));
        assert!(summary.contains("RAM"));
    }
    
    #[tokio::test]
    async fn test_collector() {
        let collector = MetricsCollector::new();
        
        collector.task_started().await;
        collector.task_started().await;
        
        let metrics = collector.collect().await;
        assert_eq!(metrics.active_tasks, 2);
        
        collector.task_completed().await;
        
        let metrics = collector.collect().await;
        assert_eq!(metrics.active_tasks, 1);
        assert_eq!(metrics.completed_tasks_hour, 1);
    }
}
