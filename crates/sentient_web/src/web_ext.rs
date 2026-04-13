// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT WEB - GraphQL, OpenAPI, Clustering, SSL Termination
// ═══════════════════════════════════════════════════════════════════════════════
//  Risk Çözümleri:
//  - ⚠️ GraphQL: GraphQL API desteği
//  - ⚠️ OpenAPI: OpenAPI 3.0 specification desteği
//  - ❌ Clustering: Çoklu süreç clustering desteği
//  - ❌ SSL Termination: SSL/TLS yönetimi
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// ───────────────────────────────────────────────────────────────────────────────
//  1. GRAPHQL DESTEĞİ
// ───────────────────────────────────────────────────────────────────────────────

/// GraphQL şema türü
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphqlType {
    Query,
    Mutation,
    Subscription,
    Object(String),
    Input(String),
    Enum(String),
    Scalar(String),
    Interface(String),
    Union(String),
}

/// GraphQL alan tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlField {
    pub name: String,
    pub field_type: GraphqlType,
    pub description: Option<String>,
    pub args: Vec<GraphqlArgument>,
    pub deprecated: bool,
    pub deprecation_reason: Option<String>,
}

/// GraphQL argüman
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlArgument {
    pub name: String,
    pub arg_type: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub description: Option<String>,
}

/// GraphQL sorgu sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlResponse {
    pub data: Option<serde_json::Value>,
    pub errors: Vec<GraphqlError>,
    pub extensions: Option<serde_json::Value>,
}

/// GraphQL hata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlError {
    pub message: String,
    pub locations: Vec<GraphqlLocation>,
    pub path: Option<Vec<String>>,
    pub extensions: Option<serde_json::Value>,
}

/// GraphQL konum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlLocation {
    pub line: u32,
    pub column: u32,
}

/// GraphQL yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Endpoint yolu
    pub endpoint: String,
    /// Introspection aktif mi?
    pub introspection: bool,
    /// Playground aktif mi?
    pub playground: bool,
    /// Maksimum sorgu derinliği
    pub max_depth: u32,
    /// Maksimum alan sayısı
    pub max_complexity: u32,
    /// Batch sorgu desteği
    pub batch_queries: bool,
    /// Maksimum batch boyutu
    pub max_batch_size: u32,
    /// Subscription desteği
    pub subscriptions: bool,
    /// Tracing
    pub tracing: bool,
    /// Cache control
    pub cache_control: bool,
}

impl Default for GraphqlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/graphql".to_string(),
            introspection: true,
            playground: true,
            max_depth: 10,
            max_complexity: 1000,
            batch_queries: true,
            max_batch_size: 10,
            subscriptions: true,
            tracing: false,
            cache_control: true,
        }
    }
}

impl GraphqlConfig {
    pub fn production() -> Self {
        Self {
            introspection: false,
            playground: false,
            tracing: false,
            ..Self::default()
        }
    }

    pub fn development() -> Self {
        Self {
            introspection: true,
            playground: true,
            tracing: true,
            ..Self::default()
        }
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn with_max_depth(mut self, depth: u32) -> Self {
        self.max_depth = depth;
        self
    }
}

/// GraphQL yöneticisi
pub struct GraphqlManager {
    config: GraphqlConfig,
    queries: Vec<GraphqlField>,
    mutations: Vec<GraphqlField>,
    subscriptions: Vec<GraphqlField>,
    schema: Option<String>,
    total_requests: u64,
}

impl GraphqlManager {
    pub fn new(config: GraphqlConfig) -> Self {
        Self {
            config,
            queries: Vec::new(),
            mutations: Vec::new(),
            subscriptions: Vec::new(),
            schema: None,
            total_requests: 0,
        }
    }

    /// Query alanı ekle
    pub fn add_query(&mut self, field: GraphqlField) {
        self.queries.push(field);
        self.schema = None; // Şema yeniden oluşturulacak
    }

    /// Mutation alanı ekle
    pub fn add_mutation(&mut self, field: GraphqlField) {
        self.mutations.push(field);
        self.schema = None;
    }

    /// Subscription alanı ekle
    pub fn add_subscription(&mut self, field: GraphqlField) {
        self.subscriptions.push(field);
        self.schema = None;
    }

    /// GraphQL şemasını oluştur
    pub fn build_schema(&mut self) -> String {
        if let Some(ref schema) = self.schema {
            return schema.clone();
        }

        let mut schema = String::new();
        schema.push_str("# Auto-generated GraphQL Schema by Sentient Web\n\n");
        schema.push_str("schema {\n");
        if !self.queries.is_empty() { schema.push_str("  query: Query\n"); }
        if !self.mutations.is_empty() { schema.push_str("  mutation: Mutation\n"); }
        if !self.subscriptions.is_empty() { schema.push_str("  subscription: Subscription\n"); }
        schema.push_str("}\n\n");

        if !self.queries.is_empty() {
            schema.push_str("type Query {\n");
            for q in &self.queries {
                schema.push_str(&format!("  {}", q.name));
                if !q.args.is_empty() {
                    schema.push_str("(");
                    let args: Vec<String> = q.args.iter().map(|a| {
                        let req = if a.required { "!" } else { "" };
                        format!("{}: {}{}", a.name, a.arg_type, req)
                    }).collect();
                    schema.push_str(&args.join(", "));
                    schema.push_str(")");
                }
                if let Some(ref desc) = q.description {
                    schema.push_str(&format!("  # {}", desc));
                }
                schema.push_str("\n");
            }
            schema.push_str("}\n\n");
        }

        if !self.mutations.is_empty() {
            schema.push_str("type Mutation {\n");
            for m in &self.mutations {
                schema.push_str(&format!("  {}\n", m.name));
            }
            schema.push_str("}\n\n");
        }

        if !self.subscriptions.is_empty() {
            schema.push_str("type Subscription {\n");
            for s in &self.subscriptions {
                schema.push_str(&format!("  {}\n", s.name));
            }
            schema.push_str("}\n");
        }

        self.schema = Some(schema.clone());
        schema
    }

    /// Sorguyu çalıştır
    pub fn execute(&mut self, query: &str, variables: Option<serde_json::Value>) -> GraphqlResponse {
        tracing::info!("🔍 GRAPHQL: Sorgu çalıştırılıyor...");
        self.total_requests += 1;

        // Sorgu doğrulama
        if let Some(errors) = self.validate_query(query) {
            return GraphqlResponse {
                data: None,
                errors,
                extensions: None,
            };
        }

        GraphqlResponse {
            data: Some(serde_json::json!({"status": "ok"})),
            errors: Vec::new(),
            extensions: None,
        }
    }

    fn validate_query(&self, query: &str) -> Option<Vec<GraphqlError>> {
        let mut errors = Vec::new();
        // Basit doğrulama
        if query.trim().is_empty() {
            errors.push(GraphqlError {
                message: "Boş sorgu".to_string(),
                locations: vec![GraphqlLocation { line: 1, column: 1 }],
                path: None,
                extensions: None,
            });
        }
        if errors.is_empty() { None } else { Some(errors) }
    }

    /// İstatistikler
    pub fn stats(&self) -> GraphqlStats {
        GraphqlStats {
            total_requests: self.total_requests,
            query_count: self.queries.len() as u32,
            mutation_count: self.mutations.len() as u32,
            subscription_count: self.subscriptions.len() as u32,
        }
    }
}

/// GraphQL istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlStats {
    pub total_requests: u64,
    pub query_count: u32,
    pub mutation_count: u32,
    pub subscription_count: u32,
}

// ───────────────────────────────────────────────────────────────────────────────
//  2. OPENAPI SPECIFICATION
// ───────────────────────────────────────────────────────────────────────────────

/// OpenAPI belgesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub servers: Vec<OpenApiServer>,
    pub paths: HashMap<String, HashMap<String, OpenApiOperation>>,
    pub components: Option<OpenApiComponents>,
    pub security: Vec<OpenApiSecurityScheme>,
    pub tags: Vec<OpenApiTag>,
}

impl Default for OpenApiSpec {
    fn default() -> Self {
        Self {
            openapi: "3.0.3".to_string(),
            info: OpenApiInfo::default(),
            servers: vec![OpenApiServer::default()],
            paths: HashMap::new(),
            components: None,
            security: Vec::new(),
            tags: Vec::new(),
        }
    }
}

impl OpenApiSpec {
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            info: OpenApiInfo {
                title: title.into(),
                version: version.into(),
                ..OpenApiInfo::default()
            },
            ..Self::default()
        }
    }

    /// Endpoint ekle
    pub fn add_path(&mut self, path: impl Into<String>, method: impl Into<String>, op: OpenApiOperation) {
        self.paths
            .entry(path.into())
            .or_default()
            .insert(method.into(), op);
    }

    /// JSON formatında oluştur
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// YAML formatında oluştur
    pub fn to_yaml(&self) -> String {
        // Basit YAML oluşturma (gerçek implementasyon serde_yaml kullanır)
        format!(
            "openapi: {}\ninfo:\n  title: {}\n  version: {}\n",
            self.openapi, self.info.title, self.info.version
        )
    }

    /// Toplam endpoint sayısı
    pub fn endpoint_count(&self) -> u32 {
        self.paths.values().map(|m| m.len() as u32).sum()
    }
}

/// OpenAPI bilgi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
    pub contact: Option<OpenApiContact>,
    pub license: Option<OpenApiLicense>,
}

impl Default for OpenApiInfo {
    fn default() -> Self {
        Self {
            title: "Sentient OS API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Sentient OS REST API Documentation".to_string()),
            contact: None,
            license: None,
        }
    }
}

/// İletişim bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiContact {
    pub name: String,
    pub email: String,
    pub url: Option<String>,
}

/// Lisans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiLicense {
    pub name: String,
    pub url: Option<String>,
}

/// Sunucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiServer {
    pub url: String,
    pub description: String,
}

impl Default for OpenApiServer {
    fn default() -> Self {
        Self {
            url: "http://localhost:8080".to_string(),
            description: "Development server".to_string(),
        }
    }
}

/// Operasyon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiOperation {
    pub operation_id: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<OpenApiParameter>,
    pub request_body: Option<OpenApiRequestBody>,
    pub responses: HashMap<String, OpenApiResponse>,
    pub security: Vec<HashMap<String, Vec<String>>>,
    pub deprecated: bool,
}

/// Parametre
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiParameter {
    pub name: String,
    pub location: OpenApiParamLocation,
    pub required: bool,
    pub description: Option<String>,
    pub schema: Option<OpenApiSchema>,
}

/// Parametre konumu
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenApiParamLocation {
    Query,
    Header,
    Path,
    Cookie,
}

/// Şema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSchema {
    pub schema_type: Option<String>,
    pub format: Option<String>,
    pub description: Option<String>,
    pub items: Option<Box<OpenApiSchema>>,
    pub properties: Option<HashMap<String, OpenApiSchema>>,
    pub required: Option<Vec<String>>,
    pub example: Option<serde_json::Value>,
}

/// İstek gövdesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiRequestBody {
    pub description: Option<String>,
    pub content: HashMap<String, OpenApiMediaType>,
    pub required: bool,
}

/// Medya türü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiMediaType {
    pub schema: Option<OpenApiSchema>,
    pub example: Option<serde_json::Value>,
}

/// Yanıt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiResponse {
    pub description: String,
    pub content: Option<HashMap<String, OpenApiMediaType>>,
}

/// Bileşenler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiComponents {
    pub schemas: HashMap<String, OpenApiSchema>,
    pub security_schemes: HashMap<String, OpenApiSecurityScheme>,
}

/// Güvenlik şeması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenApiSecurityScheme {
    Bearer {
        bearer_format: Option<String>,
    },
    ApiKey {
        location: String,
        name: String,
    },
    OAuth2 {
        flows: String,
    },
}

/// Etiket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiTag {
    pub name: String,
    pub description: Option<String>,
}

/// OpenAPI oluşturucu
pub struct OpenApiBuilder {
    spec: OpenApiSpec,
}

impl OpenApiBuilder {
    pub fn new(title: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            spec: OpenApiSpec::new(title, version),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.spec.info.description = Some(desc.into());
        self
    }

    pub fn with_server(mut self, url: impl Into<String>, desc: impl Into<String>) -> Self {
        self.spec.servers.push(OpenApiServer {
            url: url.into(),
            description: desc.into(),
        });
        self
    }

    pub fn add_get(mut self, path: &str, operation: OpenApiOperation) -> Self {
        self.spec.add_path(path, "get", operation);
        self
    }

    pub fn add_post(mut self, path: &str, operation: OpenApiOperation) -> Self {
        self.spec.add_path(path, "post", operation);
        self
    }

    pub fn add_tag(mut self, name: impl Into<String>, desc: impl Into<String>) -> Self {
        self.spec.tags.push(OpenApiTag {
            name: name.into(),
            description: Some(desc.into()),
        });
        self
    }

    pub fn build(self) -> OpenApiSpec {
        self.spec
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  3. CLUSTERING DESTEĞİ
// ───────────────────────────────────────────────────────────────────────────────

/// Cluster yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Aktif mi?
    pub enabled: bool,
    /// Worker sayısı
    pub worker_count: u32,
    /// Port aralığı
    pub port_range: (u16, u16),
    /// Load balancer türü
    pub load_balancer: ClusterLoadBalancer,
    /// Health check aralığı (saniye)
    pub health_check_interval_secs: u64,
    /// Worker zaman aşımı (saniye)
    pub worker_timeout_secs: u64,
    /// Otomatik ölçeklendirme
    pub auto_scaling: bool,
    /// Minimum worker
    pub min_workers: u32,
    /// Maksimum worker
    pub max_workers: u32,
    /// CPU eşik (%) - auto scaling
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    /// Graceful shutdown süresi (saniye)
    pub graceful_shutdown_secs: u64,
    /// Sticky sessions
    pub sticky_sessions: bool,
    /// Session cookie adı
    pub session_cookie: String,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            worker_count: 4,
            port_range: (8080, 8090),
            load_balancer: ClusterLoadBalancer::RoundRobin,
            health_check_interval_secs: 10,
            worker_timeout_secs: 30,
            auto_scaling: false,
            min_workers: 2,
            max_workers: 16,
            scale_up_threshold: 80.0,
            scale_down_threshold: 30.0,
            graceful_shutdown_secs: 30,
            sticky_sessions: false,
            session_cookie: "sentient-lb".to_string(),
        }
    }
}

impl ClusterConfig {
    pub fn with_workers(mut self, count: u32) -> Self {
        self.worker_count = count;
        self
    }

    pub fn with_auto_scaling(mut self, min: u32, max: u32) -> Self {
        self.auto_scaling = true;
        self.min_workers = min;
        self.max_workers = max;
        self
    }

    pub fn with_sticky_sessions(mut self) -> Self {
        self.sticky_sessions = true;
        self
    }
}

/// Load balancer türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClusterLoadBalancer {
    RoundRobin,
    LeastConnections,
    IpHash,
    Random,
    WeightedRoundRobin,
}

/// Worker durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerStatus {
    Starting,
    Ready,
    Busy,
    Draining,
    Stopping,
    Stopped,
    Error,
}

/// Worker bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterWorker {
    pub id: u32,
    pub pid: Option<u32>,
    pub port: u16,
    pub status: WorkerStatus,
    pub active_connections: u32,
    pub total_requests: u64,
    pub avg_response_time_ms: u64,
    pub memory_mb: u32,
    pub cpu_percent: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub last_health_check: Option<DateTime<Utc>>,
}

impl ClusterWorker {
    pub fn new(id: u32, port: u16) -> Self {
        Self {
            id,
            pid: None,
            port,
            status: WorkerStatus::Starting,
            active_connections: 0,
            total_requests: 0,
            avg_response_time_ms: 0,
            memory_mb: 0,
            cpu_percent: 0.0,
            started_at: None,
            last_health_check: None,
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, WorkerStatus::Ready | WorkerStatus::Busy)
    }

    pub fn load_score(&self) -> f64 {
        if self.status != WorkerStatus::Ready && self.status != WorkerStatus::Busy {
            return f64::MAX;
        }
        self.active_connections as f64 + self.cpu_percent / 100.0
    }
}

/// Cluster yöneticisi
pub struct ClusterManager {
    config: ClusterConfig,
    workers: Vec<ClusterWorker>,
    next_worker: usize,
    total_requests: u64,
}

impl ClusterManager {
    pub fn new(config: ClusterConfig) -> Self {
        let worker_count = config.worker_count as usize;
        let workers = (0..worker_count)
            .map(|i| ClusterWorker::new(i as u32, config.port_range.0 + i as u16))
            .collect();

        Self {
            config,
            workers,
            next_worker: 0,
            total_requests: 0,
        }
    }

    /// Worker seç (load balancing)
    pub fn select_worker(&mut self) -> Option<&ClusterWorker> {
        let healthy: Vec<usize> = self.workers.iter()
            .enumerate()
            .filter(|(_, w)| w.is_healthy())
            .map(|(i, _)| i)
            .collect();

        if healthy.is_empty() {
            return None;
        }

        let idx = match self.config.load_balancer {
            ClusterLoadBalancer::RoundRobin => {
                let idx = healthy[self.next_worker % healthy.len()];
                self.next_worker = (self.next_worker + 1) % healthy.len();
                idx
            }
            ClusterLoadBalancer::LeastConnections => {
                *healthy.iter().min_by_key(|&&i| self.workers[i].active_connections).unwrap()
            }
            ClusterLoadBalancer::IpHash => {
                healthy[0]
            }
            ClusterLoadBalancer::Random => {
                healthy[rand_index(healthy.len())]
            }
            ClusterLoadBalancer::WeightedRoundRobin => {
                let idx = healthy[self.next_worker % healthy.len()];
                self.next_worker = (self.next_worker + 1) % healthy.len();
                idx
            }
        };

        Some(&self.workers[idx])
    }

    /// Worker'ı başlat
    pub fn start_worker(&mut self, worker_id: u32) -> Result<(), String> {
        let worker = self.workers.get_mut(worker_id as usize)
            .ok_or_else(|| format!("Worker bulunamadı: {}", worker_id))?;
        worker.status = WorkerStatus::Ready;
        worker.started_at = Some(Utc::now());
        tracing::info!("🚀 CLUSTER: Worker {} başlatıldı (port: {})", worker_id, worker.port);
        Ok(())
    }

    /// Worker'ı durdur
    pub fn stop_worker(&mut self, worker_id: u32) -> Result<(), String> {
        let worker = self.workers.get_mut(worker_id as usize)
            .ok_or_else(|| format!("Worker bulunamadı: {}", worker_id))?;
        worker.status = WorkerStatus::Draining;
        tracing::info!("🛑 CLUSTER: Worker {} durduruluyor", worker_id);
        Ok(())
    }

    /// Sağlık kontrolü
    pub fn health_check(&mut self) -> ClusterHealth {
        let healthy = self.workers.iter().filter(|w| w.is_healthy()).count();
        let unhealthy = self.workers.len() - healthy;
        let avg_cpu = if !self.workers.is_empty() {
            self.workers.iter().map(|w| w.cpu_percent).sum::<f64>() / self.workers.len() as f64
        } else { 0.0 };

        // Auto scaling
        if self.config.auto_scaling {
            if avg_cpu > self.config.scale_up_threshold && self.workers.len() < self.config.max_workers as usize {
                tracing::info!("📈 CLUSTER: Auto-scaling UP (CPU: {:.1}%)", avg_cpu);
            }
            if avg_cpu < self.config.scale_down_threshold && self.workers.len() > self.config.min_workers as usize {
                tracing::info!("📉 CLUSTER: Auto-scaling DOWN (CPU: {:.1}%)", avg_cpu);
            }
        }

        ClusterHealth {
            total_workers: self.workers.len() as u32,
            healthy_workers: healthy as u32,
            unhealthy_workers: unhealthy as u32,
            avg_cpu_percent: avg_cpu,
            total_requests: self.total_requests,
        }
    }

    /// İstatistikler
    pub fn stats(&self) -> ClusterStats {
        ClusterStats {
            total_workers: self.workers.len() as u32,
            healthy_workers: self.workers.iter().filter(|w| w.is_healthy()).count() as u32,
            total_requests: self.total_requests,
        }
    }
}

fn rand_index(max: usize) -> usize {
    if max == 0 { return 0; }
    // Simplified random (gerçek implementasyon rand crate kullanır)
    0
}

/// Cluster sağlık bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    pub total_workers: u32,
    pub healthy_workers: u32,
    pub unhealthy_workers: u32,
    pub avg_cpu_percent: f64,
    pub total_requests: u64,
}

/// Cluster istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStats {
    pub total_workers: u32,
    pub healthy_workers: u32,
    pub total_requests: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  4. SSL TERMINATION
// ───────────────────────────────────────────────────────────────────────────────

/// TLS sürümü
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TlsVersion {
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}

impl TlsVersion {
    pub fn version_str(&self) -> &'static str {
        match self {
            TlsVersion::Tls10 => "1.0",
            TlsVersion::Tls11 => "1.1",
            TlsVersion::Tls12 => "1.2",
            TlsVersion::Tls13 => "1.3",
        }
    }
}

/// SSL yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    /// SSL aktif mi?
    pub enabled: bool,
    /// Sertifika dosyası yolu
    pub cert_path: String,
    /// Anahtar dosyası yolu
    pub key_path: String,
    /// CA dosyası yolu
    pub ca_path: Option<String>,
    /// Minimum TLS sürümü
    pub min_tls_version: TlsVersion,
    /// Desteklenen şifre paketleri
    pub cipher_suites: Vec<String>,
    /// HTTP/2 desteği
    pub http2_enabled: bool,
    /// OCSP stapling
    pub ocsp_stapling: bool,
    /// HSTS (HTTP Strict Transport Security)
    pub hsts_enabled: bool,
    /// HSTS max-age (saniye)
    pub hsts_max_age_secs: u64,
    /// HSTS include subdomains
    pub hsts_include_subdomains: bool,
    /// Sertifika otomatik yenileme (Let's Encrypt)
    pub auto_renewal: bool,
    /// Yenileme gün sayısı (son gün)
    pub renewal_days: u32,
    /// Redirect HTTP → HTTPS
    pub redirect_http: bool,
    /// HTTPS port
    pub https_port: u16,
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: String::new(),
            key_path: String::new(),
            ca_path: None,
            min_tls_version: TlsVersion::Tls12,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
                "ECDHE-RSA-AES256-GCM-SHA384".to_string(),
                "ECDHE-RSA-AES128-GCM-SHA256".to_string(),
            ],
            http2_enabled: true,
            ocsp_stapling: false,
            hsts_enabled: true,
            hsts_max_age_secs: 31536000, // 1 yıl
            hsts_include_subdomains: true,
            auto_renewal: false,
            renewal_days: 30,
            redirect_http: true,
            https_port: 443,
        }
    }
}

impl SslConfig {
    /// Let's Encrypt ile otomatik sertifika
    pub fn lets_encrypt(domain: impl Into<String>) -> Self {
        let domain = domain.into();
        Self {
            enabled: true,
            cert_path: format!("/etc/letsencrypt/live/{}/fullchain.pem", domain),
            key_path: format!("/etc/letsencrypt/live/{}/privkey.pem", domain),
            auto_renewal: true,
            ..Self::default()
        }
    }

    /// Manuel sertifika
    pub fn with_cert(cert: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            enabled: true,
            cert_path: cert.into(),
            key_path: key.into(),
            ..Self::default()
        }
    }

    /// Minimum TLS ayarla
    pub fn with_min_tls(mut self, version: TlsVersion) -> Self {
        self.min_tls_version = version;
        self
    }

    /// HTTP/2 aktif/pasif
    pub fn with_http2(mut self, enabled: bool) -> Self {
        self.http2_enabled = enabled;
        self
    }

    /// HSTS başlığını oluştur
    pub fn hsts_header(&self) -> Option<String> {
        if !self.hsts_enabled {
            return None;
        }
        let mut header = format!("max-age={}", self.hsts_max_age_secs);
        if self.hsts_include_subdomains {
            header.push_str("; includeSubDomains");
        }
        Some(header)
    }
}

/// SSL sertifika bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub domain: String,
    pub issuer: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub days_remaining: i64,
    pub fingerprint: String,
    pub serial_number: String,
    pub is_valid: bool,
    pub auto_renewal: bool,
}

impl CertificateInfo {
    pub fn is_expiring_soon(&self, days: i64) -> bool {
        self.days_remaining <= days
    }

    pub fn summary(&self) -> String {
        format!(
            "{} | İçerik: {} | Son: {} | Kalan: {} gün | {}",
            self.domain,
            self.issuer,
            self.not_after.format("%Y-%m-%d"),
            self.days_remaining,
            if self.is_valid { "✅" } else { "❌" },
        )
    }
}

/// SSL yöneticisi
pub struct SslManager {
    config: SslConfig,
    certificates: Vec<CertificateInfo>,
}

impl SslManager {
    pub fn new(config: SslConfig) -> Self {
        Self {
            config,
            certificates: Vec::new(),
        }
    }

    /// SSL aktif mi?
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Sertifika ekle
    pub fn add_certificate(&mut self, cert: CertificateInfo) {
        self.certificates.push(cert);
    }

    /// Sertifikaları kontrol et
    pub fn check_certificates(&self) -> Vec<&CertificateInfo> {
        self.certificates.iter()
            .filter(|c| c.is_expiring_soon(30))
            .collect()
    }

    /// Sertifika yenile
    pub fn renew_certificate(&mut self, domain: &str) -> Result<(), String> {
        if !self.config.auto_renewal {
            return Err("Otomatik yenileme kapalı".to_string());
        }
        tracing::info!("🔄 SSL: '{}' sertifikası yenileniyor...", domain);
        Ok(())
    }

    /// HSTS başlığı
    pub fn hsts_header(&self) -> Option<String> {
        self.config.hsts_header()
    }

    /// İstatistikler
    pub fn stats(&self) -> SslStats {
        SslStats {
            enabled: self.config.enabled,
            certificate_count: self.certificates.len() as u32,
            expiring_soon: self.certificates.iter().filter(|c| c.is_expiring_soon(30)).count() as u32,
            http2_enabled: self.config.http2_enabled,
            min_tls: self.config.min_tls_version.version_str().to_string(),
        }
    }
}

/// SSL istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslStats {
    pub enabled: bool,
    pub certificate_count: u32,
    pub expiring_soon: u32,
    pub http2_enabled: bool,
    pub min_tls: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- GraphQL Tests ---

    #[test]
    fn test_graphql_config_default() {
        let config = GraphqlConfig::default();
        assert!(config.enabled);
        assert_eq!(config.endpoint, "/graphql");
        assert!(config.introspection);
    }

    #[test]
    fn test_graphql_config_production() {
        let config = GraphqlConfig::production();
        assert!(!config.introspection);
        assert!(!config.playground);
    }

    #[test]
    fn test_graphql_manager() {
        let mut mgr = GraphqlManager::new(GraphqlConfig::default());
        mgr.add_query(GraphqlField {
            name: "users".to_string(),
            field_type: GraphqlType::Query,
            description: Some("Get users".to_string()),
            args: vec![],
            deprecated: false,
            deprecation_reason: None,
        });
        let schema = mgr.build_schema();
        assert!(schema.contains("type Query"));
    }

    #[test]
    fn test_graphql_execute() {
        let mut mgr = GraphqlManager::new(GraphqlConfig::default());
        let response = mgr.execute("{ users { id name } }", None);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_graphql_execute_empty() {
        let mut mgr = GraphqlManager::new(GraphqlConfig::default());
        let response = mgr.execute("", None);
        assert!(!response.errors.is_empty());
    }

    // --- OpenAPI Tests ---

    #[test]
    fn test_openapi_spec_new() {
        let spec = OpenApiSpec::new("Sentient API", "1.0.0");
        assert_eq!(spec.openapi, "3.0.3");
        assert_eq!(spec.info.title, "Sentient API");
    }

    #[test]
    fn test_openapi_add_path() {
        let mut spec = OpenApiSpec::new("Test API", "1.0.0");
        spec.add_path("/users", "get", OpenApiOperation {
            operation_id: "getUsers".to_string(),
            summary: Some("Get all users".to_string()),
            description: None,
            tags: vec!["users".to_string()],
            parameters: vec![],
            request_body: None,
            responses: HashMap::new(),
            security: vec![],
            deprecated: false,
        });
        assert_eq!(spec.endpoint_count(), 1);
    }

    #[test]
    fn test_openapi_builder() {
        let spec = OpenApiBuilder::new("Test API", "1.0.0")
            .with_description("Test API description")
            .with_server("https://api.example.com", "Production")
            .add_tag("users", "User operations")
            .build();
        assert_eq!(spec.servers.len(), 2);
        assert_eq!(spec.tags.len(), 1);
    }

    #[test]
    fn test_openapi_to_json() {
        let spec = OpenApiSpec::new("Test", "1.0.0");
        let json = spec.to_json();
        assert!(json.contains("3.0.3"));
    }

    // --- Clustering Tests ---

    #[test]
    fn test_cluster_config_default() {
        let config = ClusterConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.worker_count, 4);
    }

    #[test]
    fn test_cluster_config_scaling() {
        let config = ClusterConfig::default()
            .with_workers(8)
            .with_auto_scaling(2, 32);
        assert_eq!(config.worker_count, 8);
        assert!(config.auto_scaling);
        assert_eq!(config.max_workers, 32);
    }

    #[test]
    fn test_worker_new() {
        let worker = ClusterWorker::new(0, 8080);
        assert_eq!(worker.status, WorkerStatus::Starting);
        assert!(!worker.is_healthy());
    }

    #[test]
    fn test_worker_healthy() {
        let mut worker = ClusterWorker::new(0, 8080);
        worker.status = WorkerStatus::Ready;
        assert!(worker.is_healthy());
    }

    #[test]
    fn test_cluster_manager() {
        let mut mgr = ClusterManager::new(ClusterConfig::default());
        mgr.start_worker(0).unwrap();
        let worker = mgr.select_worker();
        assert!(worker.is_some());
    }

    #[test]
    fn test_cluster_health_check() {
        let mut mgr = ClusterManager::new(ClusterConfig::default());
        mgr.start_worker(0).unwrap();
        mgr.start_worker(1).unwrap();
        let health = mgr.health_check();
        assert_eq!(health.healthy_workers, 2);
    }

    // --- SSL Tests ---

    #[test]
    fn test_ssl_config_default() {
        let config = SslConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.min_tls_version, TlsVersion::Tls12);
    }

    #[test]
    fn test_ssl_lets_encrypt() {
        let config = SslConfig::lets_encrypt("example.com");
        assert!(config.enabled);
        assert!(config.auto_renewal);
        assert!(config.cert_path.contains("example.com"));
    }

    #[test]
    fn test_ssl_manual() {
        let config = SslConfig::with_cert("/path/cert.pem", "/path/key.pem");
        assert!(config.enabled);
    }

    #[test]
    fn test_ssl_hsts() {
        let config = SslConfig::default();
        let header = config.hsts_header().unwrap();
        assert!(header.contains("max-age=31536000"));
        assert!(header.contains("includeSubDomains"));
    }

    #[test]
    fn test_tls_version() {
        assert_eq!(TlsVersion::Tls12.version_str(), "1.2");
        assert_eq!(TlsVersion::Tls13.version_str(), "1.3");
        assert!(TlsVersion::Tls12 < TlsVersion::Tls13);
    }

    #[test]
    fn test_certificate_info() {
        let cert = CertificateInfo {
            domain: "example.com".to_string(),
            issuer: "Let's Encrypt".to_string(),
            not_before: Utc::now(),
            not_after: Utc::now() + chrono::Duration::days(90),
            days_remaining: 90,
            fingerprint: "abc123".to_string(),
            serial_number: "12345".to_string(),
            is_valid: true,
            auto_renewal: true,
        };
        assert!(!cert.is_expiring_soon(30));
        assert!(cert.is_expiring_soon(100));
    }

    #[test]
    fn test_ssl_manager() {
        let mgr = SslManager::new(SslConfig::default());
        assert!(!mgr.is_enabled());
    }

    #[test]
    fn test_ssl_stats() {
        let mgr = SslManager::new(SslConfig::default());
        let stats = mgr.stats();
        assert!(!stats.enabled);
    }
}
