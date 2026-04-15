//! ─── ARAÇ TANIMLARI VE YÖNETİMİ ───
//!
//! SENTIENT'nın kullanabileceği tüm araçların tanımları ve yönetimi.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ─── TOOL TRAIT ───
/// 
/// Tüm araçların uygulaması gereken arayüz.

#[async_trait]
pub trait Tool: Send + Sync {
    /// Araç adı
    fn name(&self) -> &str;
    
    /// Araç açıklaması
    fn description(&self) -> &str;
    
    /// Parametre şeması (JSON Schema)
    fn parameters_schema(&self) -> serde_json::Value;
    
    /// Aracı çalıştır
    async fn execute(&self, params: serde_json::Value) -> ToolResult;
}

/// ─── TOOL RESULT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub duration_ms: u64,
}

impl ToolResult {
    pub fn success(output: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            success: true,
            output,
            error: None,
            duration_ms,
        }
    }
    
    pub fn error(error: String, duration_ms: u64) -> Self {
        Self {
            success: false,
            output: serde_json::Value::Null,
            error: Some(error),
            duration_ms,
        }
    }
}

/// ─── TOOLBOX ───
/// 
/// Tüm araçları yöneten container.

pub struct Toolbox {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }
    
    /// Araç ekle
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().into(), Box::new(tool));
    }
    
    /// Araç al
    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }
    
    /// Araç çalıştır
    pub async fn execute(&self, name: &str, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        
        match self.tools.get(name) {
            Some(tool) => {
                log::debug!("🔧  TOOL: {} çalıştırılıyor...", name);
                let result = tool.execute(params).await;
                log::debug!("🔧  TOOL: {} → {}ms", name, start.elapsed().as_millis());
                result
            }
            None => {
                ToolResult::error(
                    format!("Bilinmeyen araç: {}", name),
                    start.elapsed().as_millis() as u64
                )
            }
        }
    }
    
    /// Tüm araç listesi
    pub fn list(&self) -> Vec<ToolInfo> {
        self.tools.values().map(|t| ToolInfo {
            name: t.name().into(),
            description: t.description().into(),
            parameters: t.parameters_schema(),
        }).collect()
    }
    
    /// LLM için araç tanımları (OpenAI format)
    pub fn to_openai_tools(&self) -> Vec<serde_json::Value> {
        self.tools.values().map(|t| {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name(),
                    "description": t.description(),
                    "parameters": t.parameters_schema()
                }
            })
        }).collect()
    }
}

impl Default for Toolbox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

// ─── BUILT-IN TOOLS ───

/// LLM Sorgu Aracı
pub struct LlmQueryTool;

#[async_trait]
impl Tool for LlmQueryTool {
    fn name(&self) -> &str { "llm_query" }
    
    fn description(&self) -> &str {
        "LLM'e bir soru sor ve yanıt al. Mantıksal akıl yürütme gerektirmeyen basit sorgular için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Sorulacak soru"
                },
                "context": {
                    "type": "string",
                    "description": "İsteğe bağlı bağlam bilgisi"
                }
            },
            "required": ["query"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("🧠  LLM QUERY: {}", query.chars().take(100).collect::<String>());
        
        // LLM call via V-GATE or local model
        // Production: Use sentient_cevahir or V-GATE API
        let response = format!("LLM response for: {}", query.chars().take(50).collect::<String>());
        
        ToolResult::success(
            serde_json::json!({
                "response": response,
                "query": query
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Web Arama Aracı
pub struct WebSearchTool;

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str { "web_search" }
    
    fn description(&self) -> &str {
        "Web'de arama yap ve sonuçları getir. Güncel bilgi gerektiren görevler için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Arama sorgusu"
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maksimum sonuç sayısı",
                    "default": 5
                }
            },
            "required": ["query"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let max_results = params.get("max_results").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
        
        log::info!("🔍  WEB SEARCH: {}", query);
        
        // Simülasyon sonuçları
        let results = (0..max_results.min(3)).map(|i| {
            serde_json::json!({
                "title": format!("Sonuç {} - {}", i + 1, query),
                "url": format!("https://example.com/result/{}", i + 1),
                "snippet": format!("Bu, '{}' için simüle edilmiş bir sonuçtur.", query)
            })
        }).collect::<Vec<_>>();
        
        ToolResult::success(
            serde_json::json!({
                "query": query,
                "results": results,
                "total": results.len()
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Tarayıcı Gezinme Aracı
pub struct BrowserNavigateTool;

#[async_trait]
impl Tool for BrowserNavigateTool {
    fn name(&self) -> &str { "browser_navigate" }
    
    fn description(&self) -> &str {
        "Tarayıcıda belirtilen URL'ye git. Web sayfalarını görüntülemek için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "Gidilecek URL"
                },
                "wait_for": {
                    "type": "string",
                    "description": "Beklenecek element (CSS selector)",
                    "default": "body"
                }
            },
            "required": ["url"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let url = params.get("url").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("🌐  BROWSER NAVIGATE: {}", url);
        
        // Simülasyon
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        ToolResult::success(
            serde_json::json!({
                "url": url,
                "status": "loaded",
                "title": format!("Simüle edilmiş sayfa: {}", url)
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Sandbox Kod Çalıştırma Aracı
pub struct SandboxExecuteTool;

#[async_trait]
impl Tool for SandboxExecuteTool {
    fn name(&self) -> &str { "sandbox_execute" }
    
    fn description(&self) -> &str {
        "İzole Docker ortamında kod çalıştır. Python, JavaScript, Bash desteklenir."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "language": {
                    "type": "string",
                    "enum": ["python", "javascript", "bash"],
                    "description": "Programlama dili"
                },
                "code": {
                    "type": "string",
                    "description": "Çalıştırılacak kod"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Zaman aşımı (saniye)",
                    "default": 30
                }
            },
            "required": ["language", "code"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let language = params.get("language").and_then(|v| v.as_str()).unwrap_or("python");
        let code = params.get("code").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("📦  SANDBOX EXECUTE: {} ({} bytes)", language, code.len());
        
        // Basit simülasyon - gerçekte sentient_sandbox kullanılır
        let output = match language {
            "python" => "Python kodu simüle edildi".into(),
            "javascript" => "JavaScript kodu simüle edildi".into(),
            "bash" => "Bash komutu simüle edildi".into(),
            _ => format!("Bilinmeyen dil: {}", language),
        };
        
        ToolResult::success(
            serde_json::json!({
                "language": language,
                "output": output,
                "exit_code": 0,
                "executed": true
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Bellek Kaydetme Aracı
pub struct MemoryStoreTool;

#[async_trait]
impl Tool for MemoryStoreTool {
    fn name(&self) -> &str { "memory_store" }
    
    fn description(&self) -> &str {
        "Bilgiyi uzun süreli belleğe kaydet. Önemli bulgular ve sonuçlar için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "key": {
                    "type": "string",
                    "description": "Bellek anahtarı"
                },
                "value": {
                    "type": "string",
                    "description": "Kaydedilecek değer"
                },
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Etiketler"
                }
            },
            "required": ["key", "value"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let key = params.get("key").and_then(|v| v.as_str()).unwrap_or("");
        let value = params.get("value").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("💾  MEMORY STORE: {} = {}", key, value.chars().take(50).collect::<String>());
        
        ToolResult::success(
            serde_json::json!({
                "key": key,
                "stored": true,
                "size": value.len()
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// LLM Akıl Yürütme Aracı
pub struct LlmReasonTool;

#[async_trait]
impl Tool for LlmReasonTool {
    fn name(&self) -> &str { "llm_reason" }
    
    fn description(&self) -> &str {
        "Karmaşık bir problemi mantıksal olarak çözümle. Adım adım düşünme gerektiren görevler için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "problem": {
                    "type": "string",
                    "description": "Çözülecek problem"
                },
                "context": {
                    "type": "string",
                    "description": "İsteğe bağlı bağlam bilgisi"
                },
                "steps": {
                    "type": "boolean",
                    "description": "Adım adım düşün",
                    "default": true
                }
            },
            "required": ["problem"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let problem = params.get("problem").and_then(|v| v.as_str()).unwrap_or("");
        let show_steps = params.get("steps").and_then(|v| v.as_bool()).unwrap_or(true);
        
        log::info!("🧠  LLM REASON: {}", problem.chars().take(100).collect::<String>());
        
        // Gerçek LLM çağrısı için V-GATE kullanılmalı
        // Şimdilik mantıksal çıkarım simülasyonu
        let reasoning = if show_steps {
            format!(
                "Problem analizi:\n1. Sorunu anla: {}\n2. Kavramları tanımla\n3. Mantıksal çıkarım yap\n4. Sonuçlandır",
                problem.chars().take(50).collect::<String>()
            )
        } else {
            format!("Analiz edildi: {}", problem.chars().take(50).collect::<String>())
        };
        
        ToolResult::success(
            serde_json::json!({
                "problem": problem,
                "reasoning": reasoning,
                "confidence": 0.85,
                "method": "logical_inference"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Bellek Hatırlama Aracı
pub struct MemoryRecallTool;

#[async_trait]
impl Tool for MemoryRecallTool {
    fn name(&self) -> &str { "memory_recall" }
    
    fn description(&self) -> &str {
        "Bellekten bilgi hatırla. Daha önce kaydedilen bilgilere erişmek için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Arama sorgusu"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maksimum sonuç sayısı",
                    "default": 5
                }
            },
            "required": ["query"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("📖  MEMORY RECALL: {}", query);
        
        // Simülasyon
        ToolResult::success(
            serde_json::json!({
                "query": query,
                "results": [],
                "found": false,
                "message": "Bellekte eşleşme bulunamadı (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Hesap Makinesi Aracı
pub struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str { "calculator" }
    
    fn description(&self) -> &str {
        "Matematiksel ifadeyi hesapla. Aritmetik ve basit matematik işlemleri için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Matematiksel ifade (örn: '2 + 2 * 3')"
                }
            },
            "required": ["expression"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let expr = params.get("expression").and_then(|v| v.as_str()).unwrap_or("");
        
        // Basit hesap makinesi simülasyonu
        let result = self.evaluate_simple(expr);
        
        log::info!("🔢  CALCULATOR: {} = {:?}", expr, result);
        
        match result {
            Ok(value) => ToolResult::success(
                serde_json::json!({
                    "expression": expr,
                    "result": value
                }),
                start.elapsed().as_millis() as u64
            ),
            Err(e) => ToolResult::error(
                format!("Hesaplama hatası: {}", e),
                start.elapsed().as_millis() as u64
            )
        }
    }
}

impl CalculatorTool {
    fn evaluate_simple(&self, expr: &str) -> Result<f64, String> {
        // Basit ifade değerlendirme
        // Gerçek uygulamada daha güvenli bir parser kullanılmalı
        
        let expr = expr.replace(" ", "");
        
        // Toplama
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Geçersiz sayı")?;
                let b: f64 = parts[1].parse().map_err(|_| "Geçersiz sayı")?;
                return Ok(a + b);
            }
        }
        
        // Çıkarma
        if expr.contains('-') && !expr.starts_with('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Geçersiz sayı")?;
                let b: f64 = parts[1].parse().map_err(|_| "Geçersiz sayı")?;
                return Ok(a - b);
            }
        }
        
        // Çarpma
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Geçersiz sayı")?;
                let b: f64 = parts[1].parse().map_err(|_| "Geçersiz sayı")?;
                return Ok(a * b);
            }
        }
        
        // Bölme
        if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].parse().map_err(|_| "Geçersiz sayı")?;
                let b: f64 = parts[1].parse().map_err(|_| "Geçersiz sayı")?;
                if b == 0.0 {
                    return Err("Sıfıra bölme hatası".into());
                }
                return Ok(a / b);
            }
        }
        
        // Sadece sayı
        expr.parse::<f64>().map_err(|_| "Geçersiz ifade".into())
    }
}

/// Tarayıcı Tıklama Aracı
pub struct BrowserClickTool;

#[async_trait]
impl Tool for BrowserClickTool {
    fn name(&self) -> &str { "browser_click" }
    
    fn description(&self) -> &str {
        "Sayfadaki bir elemente tıkla. CSS selector veya element ID kullanılabilir."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector"
                },
                "wait_after": {
                    "type": "integer",
                    "description": "Tıklamadan sonra bekleme (ms)",
                    "default": 500
                }
            },
            "required": ["selector"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let selector = params.get("selector").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("🖱️  BROWSER CLICK: {}", selector);
        
        ToolResult::success(
            serde_json::json!({
                "selector": selector,
                "clicked": true,
                "message": "Elemente tıklandı (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Tarayıcı Yazma Aracı
pub struct BrowserTypeTool;

#[async_trait]
impl Tool for BrowserTypeTool {
    fn name(&self) -> &str { "browser_type" }
    
    fn description(&self) -> &str {
        "Metin kutusuna yazı yaz. Input veya textarea elementleri için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector"
                },
                "text": {
                    "type": "string",
                    "description": "Yazılacak metin"
                },
                "clear_first": {
                    "type": "boolean",
                    "description": "Önce içeriği temizle",
                    "default": true
                }
            },
            "required": ["selector", "text"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let selector = params.get("selector").and_then(|v| v.as_str()).unwrap_or("");
        let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("⌨️  BROWSER TYPE: {} -> {}", selector, text.chars().take(30).collect::<String>());
        
        ToolResult::success(
            serde_json::json!({
                "selector": selector,
                "text": text,
                "typed": true,
                "message": "Metin yazıldı (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Tarayıcı Veri Çıkarma Aracı
pub struct BrowserExtractTool;

#[async_trait]
impl Tool for BrowserExtractTool {
    fn name(&self) -> &str { "browser_extract" }
    
    fn description(&self) -> &str {
        "Sayfadan veri çıkar. CSS selector ile element seç ve içeriğini al."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector"
                },
                "attribute": {
                    "type": "string",
                    "description": "Alınacak öznitelik (href, src, text)",
                    "default": "text"
                },
                "multiple": {
                    "type": "boolean",
                    "description": "Tüm eşleşen elementleri al",
                    "default": false
                }
            },
            "required": ["selector"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let selector = params.get("selector").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("📄  BROWSER EXTRACT: {}", selector);
        
        ToolResult::success(
            serde_json::json!({
                "selector": selector,
                "data": "Simüle edilmiş içerik",
                "count": 1
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Sandbox Paket Yükleme Aracı
pub struct SandboxInstallTool;

#[async_trait]
impl Tool for SandboxInstallTool {
    fn name(&self) -> &str { "sandbox_install" }
    
    fn description(&self) -> &str {
        "İzole ortamda paket yükle. pip, npm, cargo desteklenir."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "package_manager": {
                    "type": "string",
                    "enum": ["pip", "npm", "cargo"],
                    "description": "Paket yöneticisi"
                },
                "package": {
                    "type": "string",
                    "description": "Yüklenecek paket"
                }
            },
            "required": ["package_manager", "package"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let pm = params.get("package_manager").and_then(|v| v.as_str()).unwrap_or("pip");
        let pkg = params.get("package").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("📦  SANDBOX INSTALL: {} {}", pm, pkg);
        
        ToolResult::success(
            serde_json::json!({
                "package_manager": pm,
                "package": pkg,
                "installed": true,
                "message": "Paket yüklendi (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Sandbox Test Aracı
pub struct SandboxTestTool;

#[async_trait]
impl Tool for SandboxTestTool {
    fn name(&self) -> &str { "sandbox_test" }
    
    fn description(&self) -> &str {
        "İzole ortamda test çalıştır. pytest, jest, cargo test desteklenir."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "framework": {
                    "type": "string",
                    "enum": ["pytest", "jest", "cargo"],
                    "description": "Test framework"
                },
                "path": {
                    "type": "string",
                    "description": "Test dosyası yolu"
                }
            },
            "required": ["framework", "path"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let framework = params.get("framework").and_then(|v| v.as_str()).unwrap_or("pytest");
        let path = params.get("path").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("🧪  SANDBOX TEST: {} {}", framework, path);
        
        ToolResult::success(
            serde_json::json!({
                "framework": framework,
                "path": path,
                "passed": 5,
                "failed": 0,
                "message": "Tüm testler geçti (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Bellek Arama Aracı
pub struct MemorySearchTool;

#[async_trait]
impl Tool for MemorySearchTool {
    fn name(&self) -> &str { "memory_search" }
    
    fn description(&self) -> &str {
        "Bellekte semantik arama yap. Benzerlik tabanlı bilgi bulma için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Arama sorgusu"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maksimum sonuç sayısı",
                    "default": 5
                },
                "threshold": {
                    "type": "number",
                    "description": "Benzerlik eşiği (0-1)",
                    "default": 0.7
                }
            },
            "required": ["query"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("🔍  MEMORY SEARCH: {}", query);
        
        ToolResult::success(
            serde_json::json!({
                "query": query,
                "results": [],
                "message": "Bellek arandı (simülasyon)"
            }),
            start.elapsed().as_millis() as u64
        )
    }
}

/// Dosya Okuma Aracı
pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str { "file_read" }
    
    fn description(&self) -> &str {
        "Dosya oku. Proje dosyalarını incelemek için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Dosya yolu"
                },
                "max_lines": {
                    "type": "integer",
                    "description": "Maksimum satır sayısı",
                    "default": 1000
                }
            },
            "required": ["path"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let path = params.get("path").and_then(|v| v.as_str()).unwrap_or("");
        
        log::info!("📂  FILE READ: {}", path);
        
        // Gerçek dosya okuma işlemi
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                let max_lines = params.get("max_lines").and_then(|v| v.as_u64()).unwrap_or(1000) as usize;
                let truncated: Vec<&str> = lines.iter().take(max_lines).copied().collect();
                
                ToolResult::success(
                    serde_json::json!({
                        "path": path,
                        "content": truncated.join("\n"),
                        "lines": lines.len(),
                        "truncated": lines.len() > max_lines
                    }),
                    start.elapsed().as_millis() as u64
                )
            }
            Err(e) => ToolResult::error(
                format!("Dosya okunamadı: {}", e),
                start.elapsed().as_millis() as u64
            )
        }
    }
}

/// Dosya Yazma Aracı
pub struct FileWriteTool;

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str { "file_write" }
    
    fn description(&self) -> &str {
        "Dosyaya yaz. Kod veya veri oluşturmak için kullanılır."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Dosya yolu"
                },
                "content": {
                    "type": "string",
                    "description": "Yazılacak içerik"
                },
                "append": {
                    "type": "boolean",
                    "description": "Dosyaya ekle",
                    "default": false
                }
            },
            "required": ["path", "content"]
        })
    }
    
    async fn execute(&self, params: serde_json::Value) -> ToolResult {
        let start = std::time::Instant::now();
        let path = params.get("path").and_then(|v| v.as_str()).unwrap_or("");
        let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let append = params.get("append").and_then(|v| v.as_bool()).unwrap_or(false);
        
        log::info!("📝  FILE WRITE: {} ({} bytes)", path, content.len());
        
        // Gerçek dosya yazma işlemi
        use std::io::Write;
        let result = if append {
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut f| f.write_all(content.as_bytes()))
        } else {
            std::fs::write(path, content)
        };
        
        match result {
            Ok(()) => ToolResult::success(
                serde_json::json!({
                    "path": path,
                    "bytes_written": content.len(),
                    "appended": append
                }),
                start.elapsed().as_millis() as u64
            ),
            Err(e) => ToolResult::error(
                format!("Dosya yazılamadı: {}", e),
                start.elapsed().as_millis() as u64
            )
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_toolbox_creation() {
        let toolbox = Toolbox::new();
        assert!(toolbox.tools.is_empty());
    }
    
    #[tokio::test]
    async fn test_tool_registration() {
        let mut toolbox = Toolbox::new();
        toolbox.register(LlmQueryTool);
        toolbox.register(WebSearchTool);
        
        assert!(toolbox.get("llm_query").is_some());
        assert!(toolbox.get("web_search").is_some());
        assert!(toolbox.get("unknown").is_none());
    }
    
    #[tokio::test]
    async fn test_tool_execution() {
        let mut toolbox = Toolbox::new();
        toolbox.register(CalculatorTool);
        
        let result = toolbox.execute("calculator", serde_json::json!({"expression": "2+2"})).await;
        assert!(result.success);
        assert_eq!(result.output.get("result").expect("operation failed").as_f64().expect("operation failed"), 4.0);
    }
    
    #[tokio::test]
    async fn test_unknown_tool() {
        let toolbox = Toolbox::new();
        let result = toolbox.execute("unknown", serde_json::Value::Null).await;
        assert!(!result.success);
        assert!(result.error.is_some());
    }
    
    #[tokio::test]
    async fn test_calculator_tool() {
        let tool = CalculatorTool;
        
        let result = tool.execute(serde_json::json!({"expression": "10 + 5"})).await;
        assert!(result.success);
        assert_eq!(result.output.get("result").expect("operation failed").as_f64().expect("operation failed"), 15.0);
        
        let result = tool.execute(serde_json::json!({"expression": "20 * 3"})).await;
        assert!(result.success);
        assert_eq!(result.output.get("result").expect("operation failed").as_f64().expect("operation failed"), 60.0);
    }
    
    #[tokio::test]
    async fn test_calculator_division_by_zero() {
        let tool = CalculatorTool;
        let result = tool.execute(serde_json::json!({"expression": "10 / 0"})).await;
        assert!(!result.success);
    }
    
    #[tokio::test]
    async fn test_web_search_tool() {
        let tool = WebSearchTool;
        let result = tool.execute(serde_json::json!({"query": "test query"})).await;
        assert!(result.success);
        assert!(result.output.get("results").expect("operation failed").is_array());
    }
    
    #[test]
    fn test_openai_tools_format() {
        let mut toolbox = Toolbox::new();
        toolbox.register(LlmQueryTool);
        toolbox.register(CalculatorTool);
        
        let tools = toolbox.to_openai_tools();
        assert!(!tools.is_empty());
        assert!(tools[0].get("type").expect("operation failed").as_str().expect("operation failed") == "function");
    }
}
