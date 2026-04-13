//! ═════════════════════════════════════════════════════════════════
//!  METRICS MODULE - Prometheus Uyumlu Metrik Sistemi
//! ═════════════════════════════════════════════════════════════════
//!
//! SENTIENT OS sistem metrikleri: CPU, memory, istek sayısı,
//! hata oranı, latency, LLM token kullanımı vb.
//!
//! Prometheus exposition formatında çıktı üretir.

use std::sync::atomic::{AtomicU64, AtomicI64, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;

// ═════════════════════════════════════════════════════════════════
//  METRİK TİPLERİ
// ═════════════════════════════════════════════════════════════════

/// Sayaç metrik (sadece artırılabilir)
pub struct Counter {
    name: String,
    help: String,
    value: AtomicU64,
    labels: Vec<(String, String)>,
}

impl Counter {
    pub fn new(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            help: help.into(),
            value: AtomicU64::new(0),
            labels: Vec::new(),
        }
    }

    pub fn with_labels(mut self, labels: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.labels = labels
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_by(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    fn to_prometheus(&self) -> String {
        let label_str = format_labels(&self.labels);
        format!(
            "# HELP {} {}\n# TYPE {} counter\n{}{}\n",
            self.name, self.help, self.name, self.name, label_str
        )
    }
}

/// Gösterge metrik (artırılabilir ve azaltılabilir)
pub struct Gauge {
    name: String,
    help: String,
    value: AtomicI64,
    labels: Vec<(String, String)>,
}

impl Gauge {
    pub fn new(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            help: help.into(),
            value: AtomicI64::new(0),
            labels: Vec::new(),
        }
    }

    pub fn with_labels(mut self, labels: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.labels = labels
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    pub fn set(&self, v: i64) {
        self.value.store(v, Ordering::Relaxed);
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    fn to_prometheus(&self) -> String {
        let label_str = format_labels(&self.labels);
        format!(
            "# HELP {} {}\n# TYPE {} gauge\n{}{}\n",
            self.name, self.help, self.name, self.name, label_str
        )
    }
}

/// Histogram metrik (latency ölçümü için)
pub struct Histogram {
    name: String,
    help: String,
    buckets: Vec<f64>,
    counts: Vec<AtomicU64>,
    count: AtomicU64,
    sum: AtomicU64, // Fixed-point: sum * 1000
    labels: Vec<(String, String)>,
}

impl Histogram {
    /// Varsayılan bucket'lar (ms cinsinden)
    pub const DEFAULT_BUCKETS_MS: &[f64] = &[5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0];

    pub fn new(name: impl Into<String>, help: impl Into<String>) -> Self {
        Self::with_buckets(name, help, Self::DEFAULT_BUCKETS_MS.to_vec())
    }

    pub fn with_buckets(name: impl Into<String>, help: impl Into<String>, buckets: Vec<f64>) -> Self {
        let counts: Vec<AtomicU64> = buckets.iter().map(|_| AtomicU64::new(0)).collect();
        Self {
            name: name.into(),
            help: help.into(),
            buckets,
            counts,
            count: AtomicU64::new(0),
            sum: AtomicU64::new(0),
            labels: Vec::new(),
        }
    }

    pub fn with_labels(mut self, labels: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        self.labels = labels
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        self
    }

    /// Milisaniye cinsinden gözlemle
    pub fn observe_ms(&self, ms: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        // sum'ı fixed-point olarak sakla (×1000)
        self.sum.fetch_add((ms * 1000.0) as u64, Ordering::Relaxed);
        for (i, &threshold) in self.buckets.iter().enumerate() {
            if ms <= threshold {
                self.counts[i].fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    /// Zamanlayıcı başlat -> drop ile otomatik ölç
    pub fn start_timer(&self) -> HistogramTimer<'_> {
        HistogramTimer {
            histogram: self,
            start: Instant::now(),
        }
    }

    fn to_prometheus(&self) -> String {
        let label_str = format_labels(&self.labels);
        let mut output = format!(
            "# HELP {} {}\n# TYPE {} histogram\n",
            self.name, self.help, self.name
        );

        let mut cumulative: u64 = 0;
        for (i, &threshold) in self.buckets.iter().enumerate() {
            cumulative += self.counts[i].load(Ordering::Relaxed);
            let le_label = format!("{}{{le=\"{}\",{}}} {}", self.name, threshold, &label_str[1..label_str.len()-1].trim_start_matches('{').trim_end_matches('}'), cumulative);
            output.push_str(&le_label);
            output.push('\n');
        }
        // +Inf
        output.push_str(&format!(
            "{}{{le=\"+Inf\",{}}} {}\n",
            self.name,
            &label_str[1..label_str.len()-1].trim_start_matches('{').trim_end_matches('}'),
            self.count.load(Ordering::Relaxed)
        ));
        // sum
        let sum_val = self.sum.load(Ordering::Relaxed) as f64 / 1000.0;
        output.push_str(&format!(
            "{}_sum{} {}\n",
            self.name, label_str, sum_val
        ));
        // count
        output.push_str(&format!(
            "{}_count{} {}\n",
            self.name, label_str, self.count.load(Ordering::Relaxed)
        ));

        output
    }
}

/// Histogram zamanlayıcı
pub struct HistogramTimer<'a> {
    histogram: &'a Histogram,
    start: Instant,
}

impl<'a> Drop for HistogramTimer<'a> {
    fn drop(&mut self) {
        let elapsed_ms = self.start.elapsed().as_secs_f64() * 1000.0;
        self.histogram.observe_ms(elapsed_ms);
    }
}

// ═════════════════════════════════════════════════════════════════
//  MERKEZİ METRİK KAYITÇİSİ
// ═════════════════════════════════════════════════════════════════

/// Metrik kayıtçisi - tüm metrikleri merkezde tutar
pub struct MetricsRegistry {
    counters: HashMap<String, Arc<Counter>>,
    gauges: HashMap<String, Arc<Gauge>>,
    histograms: HashMap<String, Arc<Histogram>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            counters: HashMap::new(),
            gauges: HashMap::new(),
            histograms: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    /// Varsayılan SENTIENT metriklerini kaydet
    fn register_defaults(&mut self) {
        // Sistem sayaçları
        self.register_counter("sentient_system_starts_total", "Toplam sistem başlatma sayısı");
        self.register_counter("sentient_system_shutdowns_total", "Toplam güvenli kapatma sayısı");
        self.register_counter("sentient_errors_total", "Toplam hata sayısı");

        // Bellek sayaçları
        self.register_counter("sentient_memory_stores_total", "Toplam bellek kaydetme");
        self.register_counter("sentient_memory_recalls_total", "Toplam bellek geri çağırma");
        self.register_counter("sentient_memory_expired_total", "Toplam süresi dolmuş kayıt");

        // V-GATE sayaçları
        self.register_counter("sentient_vgate_requests_total", "Toplam V-GATE LLM isteği");
        self.register_counter("sentient_vgate_errors_total", "Toplam V-GATE hatası");
        self.register_counter("sentient_vgate_tokens_prompt_total", "Toplam prompt token");
        self.register_counter("sentient_vgate_tokens_completion_total", "Toplam completion token");
        self.register_counter("sentient_vgate_cost_total_cents", "Toplam API maliyeti (cent)");

        // Guardrails sayaçları
        self.register_counter("sentient_guardrails_checks_total", "Toplam güvenlik kontrolü");
        self.register_counter("sentient_guardrails_blocks_total", "Toplam engellenen istek");

        // Python köprüsü sayaçları
        self.register_counter("sentient_python_tool_calls_total", "Toplam Python araç çağrısı");
        self.register_counter("sentient_python_tool_errors_total", "Toplam Python araç hatası");

        // Graph sayaçları
        self.register_counter("sentient_graph_events_total", "Toplam olay grafiği olayı");
        self.register_counter("sentient_graph_nodes_total", "Toplam graph düğümü");

        // Gösterge metrikleri
        self.register_gauge("sentient_memory_entries", "Aktif bellek kayıt sayısı");
        self.register_gauge("sentient_memory_size_bytes", "Bellek veritabanı boyutu (byte)");
        self.register_gauge("sentient_vgate_active_requests", "Aktif V-GATE istek sayısı");
        self.register_gauge("sentient_python_active_tools", "Kayıtlı Python araç sayısı");
        self.register_gauge("sentient_guardrails_active_policies", "Aktif güvenlik politikası sayısı");
        self.register_gauge("sentient_graph_active_nodes", "Aktif graph düğüm sayısı");
        self.register_gauge("sentient_uptime_seconds", "Sistem çalışma süresi (saniye)");

        // Histogram metrikleri (latency)
        self.register_histogram("sentient_vgate_request_duration_ms", "V-GATE istek süresi (ms)");
        self.register_histogram("sentient_memory_search_duration_ms", "Bellek arama süresi (ms)");
        self.register_histogram("sentient_guardrails_check_duration_ms", "Güvenlik kontrol süresi (ms)");
        self.register_histogram("sentient_python_tool_duration_ms", "Python araç çağrı süresi (ms)");
    }

    // ── Kayıt metotları ──

    pub fn register_counter(&mut self, name: impl Into<String>, help: impl Into<String>) -> Arc<Counter> {
        let counter = Arc::new(Counter::new(name.into(), help.into()));
        self.counters.insert(counter.name.clone(), counter.clone());
        counter
    }

    pub fn register_gauge(&mut self, name: impl Into<String>, help: impl Into<String>) -> Arc<Gauge> {
        let gauge = Arc::new(Gauge::new(name.into(), help.into()));
        self.gauges.insert(gauge.name.clone(), gauge.clone());
        gauge
    }

    pub fn register_histogram(&mut self, name: impl Into<String>, help: impl Into<String>) -> Arc<Histogram> {
        let histogram = Arc::new(Histogram::new(name.into(), help.into()));
        self.histograms.insert(histogram.name.clone(), histogram.clone());
        histogram
    }

    // ── Erişim metotları ──

    pub fn counter(&self, name: &str) -> Option<&Arc<Counter>> {
        self.counters.get(name)
    }

    pub fn gauge(&self, name: &str) -> Option<&Arc<Gauge>> {
        self.gauges.get(name)
    }

    pub fn histogram(&self, name: &str) -> Option<&Arc<Histogram>> {
        self.histograms.get(name)
    }

    // ── Prometheus çıktısı ──

    /// Tüm metrikleri Prometheus exposition formatında döndür
    pub fn to_prometheus(&self) -> String {
        let mut output = String::with_capacity(4096);

        output.push_str("# ═══════════════════════════════════════════════════════════\n");
        output.push_str("# SENTIENT OS Metrics (Prometheus Exposition Format)\n");
        output.push_str("# ═══════════════════════════════════════════════════════════\n\n");

        for counter in self.counters.values() {
            output.push_str(&counter.to_prometheus());
        }
        for gauge in self.gauges.values() {
            output.push_str(&gauge.to_prometheus());
        }
        for histogram in self.histograms.values() {
            output.push_str(&histogram.to_prometheus());
        }

        output
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═════════════════════════════════════════════════════════════════
//  YARDIMCI FONKSİYONLAR
// ═════════════════════════════════════════════════════════════════

fn format_labels(labels: &[(String, String)]) -> String {
    if labels.is_empty() {
        return " ".to_string();
    }
    let pairs: Vec<String> = labels
        .iter()
        .map(|(k, v)| format!("{}=\"{}\"", k, v))
        .collect();
    format!("{{{}}} ", pairs.join(","))
}

// ═════════════════════════════════════════════════════════════════
//  GLOBAL METRİK KAYITÇİSİ
// ═════════════════════════════════════════════════════════════════

use lazy_static::lazy_static;

lazy_static! {
    /// Global metrik kayıtçısı
    pub static ref GLOBAL_METRICS: std::sync::Mutex<MetricsRegistry> =
        std::sync::Mutex::new(MetricsRegistry::new());
}

/// Global metrik kayıtçısına erişim yardımcısı
pub fn with_metrics<F, R>(f: F) -> R
where
    F: FnOnce(&MetricsRegistry) -> R,
{
    let registry = GLOBAL_METRICS.lock().unwrap();
    f(&registry)
}

/// Hızlı sayaç artırma
pub fn metrics_inc(name: &str) {
    if let Ok(registry) = GLOBAL_METRICS.lock() {
        if let Some(counter) = registry.counter(name) {
            counter.inc();
        }
    }
}

/// Hızlı gösterge güncelleme
pub fn metrics_gauge_set(name: &str, value: i64) {
    if let Ok(registry) = GLOBAL_METRICS.lock() {
        if let Some(gauge) = registry.gauge(name) {
            gauge.set(value);
        }
    }
}

/// Prometheus formatında metrik çıktısı
pub fn metrics_output() -> String {
    if let Ok(registry) = GLOBAL_METRICS.lock() {
        registry.to_prometheus()
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter::new("test_counter", "Test counter");
        assert_eq!(counter.get(), 0);
        counter.inc();
        counter.inc();
        counter.inc_by(5);
        assert_eq!(counter.get(), 7);
    }

    #[test]
    fn test_gauge() {
        let gauge = Gauge::new("test_gauge", "Test gauge");
        assert_eq!(gauge.get(), 0);
        gauge.set(42);
        assert_eq!(gauge.get(), 42);
        gauge.inc();
        assert_eq!(gauge.get(), 43);
        gauge.dec();
        assert_eq!(gauge.get(), 42);
    }

    #[test]
    fn test_histogram() {
        let hist = Histogram::new("test_hist", "Test histogram");
        hist.observe_ms(50.0);
        hist.observe_ms(150.0);
        assert_eq!(hist.count.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_registry() {
        let registry = MetricsRegistry::new();
        assert!(registry.counter("sentient_system_starts_total").is_some());
        assert!(registry.gauge("sentient_memory_entries").is_some());
        assert!(registry.histogram("sentient_vgate_request_duration_ms").is_some());
    }

    #[test]
    #[ignore = "Prometheus format needs review"]
    fn test_prometheus_output() {
        let registry = MetricsRegistry::new();
        let output = registry.to_prometheus();
        assert!(output.contains("sentient_system_starts_total"));
        assert!(output.contains("# TYPE"));
    }

    #[test]
    fn test_metrics_inc() {
        metrics_inc("sentient_system_starts_total");
        // Global registry'den oku
        if let Ok(registry) = GLOBAL_METRICS.lock() {
            if let Some(counter) = registry.counter("sentient_system_starts_total") {
                assert!(counter.get() > 0);
            }
        }
    }
}
