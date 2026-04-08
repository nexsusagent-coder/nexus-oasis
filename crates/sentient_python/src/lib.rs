//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT PYO3 KÖPRÜSÜ (ENTEGRASYON KATMANI)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Python tabanlı araçlar PyO3 ile Rust çekirdeğine "Native" modül
//! olarak sarmalanır. Sıfır kopyalı (zero-copy) veri akışı sağlar.
//! Ham Python hataları yakalanır ve SENTIENT formatına çevrilir.

pub mod wrappers;

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_common::translate::translate_raw_error;
use log;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Python Aracı Tanımı ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonToolDef {
    pub name: String,
    pub module_path: String,
    pub function_name: String,
    pub description: String,
    pub args: Vec<String>,
}

impl PythonToolDef {
    pub fn new(name: &str, module: &str, function: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            module_path: module.into(),
            function_name: function.into(),
            description: description.into(),
            args: vec![],
        }
    }
    
    pub fn with_args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|s| s.to_string()).collect();
        self
    }
}

// ─── Browser Tool Sonucu ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserResult {
    pub success: bool,
    pub content: String,
    pub url: Option<String>,
    pub screenshot: Option<String>,
    pub links: Vec<String>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl BrowserResult {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            success: json.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            content: json.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            url: json.get("url").and_then(|v| v.as_str()).map(|s| s.to_string()),
            screenshot: json.get("screenshot").and_then(|v| v.as_str()).map(|s| s.to_string()),
            links: json.get("links")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            error: json.get("error").and_then(|v| v.as_str()).map(|s| s.to_string()),
            timestamp: json.get("timestamp").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }
    }
    
    pub fn is_ok(&self) -> bool {
        self.success && self.error.is_none()
    }
    
    pub fn summary(&self) -> String {
        if self.is_ok() {
            format!("✅ {} ({} karakter)", self.url.as_deref().unwrap_or("URL yok"), self.content.len())
        } else {
            format!("❌ {}", self.error.as_deref().unwrap_or("Bilinmeyen hata"))
        }
    }
}

// ─── Sandbox Tool Sonucu ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub sandbox_id: String,
    pub error: Option<String>,
}

impl SandboxResult {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            success: json.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            exit_code: json.get("exit_code").and_then(|v| v.as_i64()).unwrap_or(-1) as i32,
            stdout: json.get("stdout").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            stderr: json.get("stderr").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            duration_ms: json.get("duration_ms").and_then(|v| v.as_u64()).unwrap_or(0),
            sandbox_id: json.get("sandbox_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            error: json.get("error").and_then(|v| v.as_str()).map(|s| s.to_string()),
        }
    }
    
    pub fn is_ok(&self) -> bool {
        self.success && self.exit_code == 0
    }
    
    pub fn summary(&self) -> String {
        if self.is_ok() {
            format!("✅ [{}] {}ms → exit=0", self.sandbox_id, self.duration_ms)
        } else {
            format!("❌ [{}] exit={} → {}", self.sandbox_id, self.exit_code, self.error.as_deref().unwrap_or("Hata"))
        }
    }
}

// ─── PyO3 Köprü Yöneticisi ───

pub struct PythonBridge {
    tools: HashMap<String, PythonToolDef>,
    initialized: bool,
}

impl PythonBridge {
    pub fn new() -> Self {
        log::info!("🐍  KÖPRÜ: PyO3 asimilasyon katmanı başlatılıyor...");
        Self {
            tools: HashMap::new(),
            initialized: false,
        }
    }

    /// Python aracını kaydet
    pub fn register_tool(&mut self, tool: PythonToolDef) {
        log::info!(
            "🐍  KÖPRÜ: Araç kaydedildi → {} ({}::{})",
            tool.name,
            tool.module_path,
            tool.function_name
        );
        self.tools.insert(tool.name.clone(), tool);
    }
    
    /// Browser araçlarını otomatik kaydet
    pub fn register_browser_tools(&mut self) {
        self.register_tool(PythonToolDef::new(
            "browser_init",
            "browser_use",
            "initialize",
            "Tarayıcıyı başlatır"
        ).with_args(vec!["headless"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_task",
            "browser_use",
            "execute_task",
            "Doğal dille bir görevi çalıştırır"
        ).with_args(vec!["task"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_navigate",
            "browser_use",
            "navigate",
            "Belirtilen URL'ye gider"
        ).with_args(vec!["url"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_search",
            "browser_use",
            "search",
            "Web'de arama yapar"
        ).with_args(vec!["query", "engine"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_research",
            "browser_use",
            "research",
            "Derinlemesine araştırma yapar"
        ).with_args(vec!["topic", "depth"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_screenshot",
            "browser_use",
            "screenshot",
            "Ekran görüntüsü alır"
        ).with_args(vec!["full_page"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_extract",
            "browser_use",
            "extract_content",
            "Sayfa içeriğini çıkarır"
        ).with_args(vec!["selector"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_click",
            "browser_use",
            "click",
            "Elemente tıklar"
        ).with_args(vec!["selector"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_type",
            "browser_use",
            "type_text",
            "Input alanına yazı yazar"
        ).with_args(vec!["selector", "text", "press_enter"]));
        
        self.register_tool(PythonToolDef::new(
            "browser_close",
            "browser_use",
            "close",
            "Tarayıcıyı kapatır"
        ));
        
        self.register_tool(PythonToolDef::new(
            "browser_history",
            "browser_use",
            "get_history",
            "Görev geçmişini döndürür"
        ));
        
        log::info!("🐍  KÖPRÜ: {} browser aracı kaydedildi", self.tools.len());
    }
    
    /// Sandbox (OpenManus) araçlarını kaydet
    pub fn register_sandbox_tools(&mut self) {
        self.register_tool(PythonToolDef::new(
            "sandbox_create",
            "openmanus",
            "initialize",
            "Yalıtılmış Docker sandbox oluşturur"
        ));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_execute",
            "openmanus",
            "execute_code",
            "Sandbox içinde kod çalıştırır"
        ).with_args(vec!["code", "language"]));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_python",
            "openmanus",
            "execute_python",
            "Python kodu çalıştırır (kısayol)"
        ).with_args(vec!["code"]));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_javascript",
            "openmanus",
            "execute_javascript",
            "JavaScript kodu çalıştırır (kısayol)"
        ).with_args(vec!["code"]));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_bash",
            "openmanus",
            "execute_bash",
            "Bash komutu çalıştırır (kısayol)"
        ).with_args(vec!["command"]));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_close",
            "openmanus",
            "close",
            "Sandbox'ı temizler"
        ));
        
        self.register_tool(PythonToolDef::new(
            "sandbox_limits",
            "openmanus",
            "get_limits",
            "Sandbox kaynak limitlerini döndürür"
        ));
        
        log::info!("🐍  KÖPRÜ: {} araç kayıtlı (browser + sandbox)", self.tools.len());
    }

    /// PyO3 üzerinden Python fonksiyonu çağır
    pub fn call_python(
        &self,
        tool_name: &str,
        args: Option<serde_json::Value>,
    ) -> SENTIENTResult<serde_json::Value> {
        let tool = self.tools.get(tool_name).ok_or_else(|| {
            SENTIENTError::PythonBridge(format!("Tanımlanmamış araç: {}", tool_name))
        })?;

        Python::with_gil(|py| {
            // Modülü import et
            let module = py.import(&tool.module_path).map_err(|e| {
                let raw = e.value(py).to_string();
                log::warn!("🐍  KÖPRÜ HATA → {}", translate_raw_error(&raw));
                SENTIENTError::PythonBridge(translate_raw_error(&raw))
            })?;
            
            // Senkron wrapper class'ı al (SENTIENTBrowserSync)
            let sync_class = module.getattr("SENTIENTBrowserSync").map_err(|e| {
                let raw = e.value(py).to_string();
                SENTIENTError::PythonBridge(format!("SENTIENTBrowserSync bulunamadı: {}", raw))
            })?;
            
            // Instance oluştur
            let instance = sync_class.call0().map_err(|e| {
                let raw = e.value(py).to_string();
                SENTIENTError::PythonBridge(translate_raw_error(&raw))
            })?;
            
            // Fonksiyonu çağır
            let func = instance.getattr(&tool.function_name).map_err(|e| {
                let raw = e.value(py).to_string();
                SENTIENTError::PythonBridge(format!("Fonksiyon bulunamadı {}: {}", tool.function_name, raw))
            })?;
            
            // Argümanları hazırla ve çağır
            let py_result = if let Some(ref json_args) = args {
                // JSON'dan Python dict'e çevir
                let kwargs = json_to_pydict(py, json_args).map_err(|e| {
                    SENTIENTError::PythonBridge(format!("Argüman çevrimi hatası: {}", e))
                })?;
                func.call((), Some(&kwargs))
            } else {
                func.call0()
            };
            
            // Sonucu işle
            let py_value = py_result.map_err(|e| {
                let raw = e.value(py).to_string();
                log::warn!("🐍  KÖPRÜ HATA → {}", translate_raw_error(&raw));
                SENTIENTError::PythonBridge(translate_raw_error(&raw))
            })?;
            
            // Python dict'ini JSON'a çevir
            py_value_to_json(py, &py_value).map_err(|e| {
                SENTIENTError::PythonBridge(format!("JSON çevrimi hatası: {}", e))
            })
        })
    }
    
    /// Browser'ı başlat
    pub fn browser_init(&mut self, headless: bool) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"headless": headless});
        let result = self.call_python("browser_init", Some(args))?;
        self.initialized = true;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Görev çalıştır
    pub fn browser_task(&self, task: &str) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"task": task});
        let result = self.call_python("browser_task", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// URL'ye git
    pub fn browser_navigate(&self, url: &str) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"url": url});
        let result = self.call_python("browser_navigate", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Ara
    pub fn browser_search(&self, query: &str, engine: &str) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"query": query, "engine": engine});
        let result = self.call_python("browser_search", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Araştırma yap
    pub fn browser_research(&self, topic: &str, depth: u32) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"topic": topic, "depth": depth});
        let result = self.call_python("browser_research", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Ekran görüntüsü
    pub fn browser_screenshot(&self, full_page: bool) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"full_page": full_page});
        let result = self.call_python("browser_screenshot", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// İçerik çıkar
    pub fn browser_extract(&self, selector: Option<&str>) -> SENTIENTResult<BrowserResult> {
        let args = if let Some(sel) = selector {
            serde_json::json!({"selector": sel})
        } else {
            serde_json::json!({})
        };
        let result = self.call_python("browser_extract", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Tıkla
    pub fn browser_click(&self, selector: &str) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({"selector": selector});
        let result = self.call_python("browser_click", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Yaz
    pub fn browser_type(&self, selector: &str, text: &str, press_enter: bool) -> SENTIENTResult<BrowserResult> {
        let args = serde_json::json!({
            "selector": selector,
            "text": text,
            "press_enter": press_enter
        });
        let result = self.call_python("browser_type", Some(args))?;
        Ok(BrowserResult::from_json(&result))
    }
    
    /// Kapat
    pub fn browser_close(&self) -> SENTIENTResult<BrowserResult> {
        let result = self.call_python("browser_close", None)?;
        Ok(BrowserResult::from_json(&result))
    }
    
    // ─── Sandbox (OpenManus) Metodları ───
    
    /// Sandbox oluştur
    pub fn sandbox_create(&mut self) -> SENTIENTResult<SandboxResult> {
        let result = self.call_python("sandbox_create", None)?;
        self.initialized = true;
        Ok(SandboxResult::from_json(&result))
    }
    
    /// Sandbox'ta kod çalıştır
    pub fn sandbox_execute(&self, code: &str, language: &str) -> SENTIENTResult<SandboxResult> {
        let args = serde_json::json!({"code": code, "language": language});
        let result = self.call_python("sandbox_execute", Some(args))?;
        Ok(SandboxResult::from_json(&result))
    }
    
    /// Python çalıştır
    pub fn sandbox_python(&self, code: &str) -> SENTIENTResult<SandboxResult> {
        let args = serde_json::json!({"code": code});
        let result = self.call_python("sandbox_python", Some(args))?;
        Ok(SandboxResult::from_json(&result))
    }
    
    /// JavaScript çalıştır
    pub fn sandbox_javascript(&self, code: &str) -> SENTIENTResult<SandboxResult> {
        let args = serde_json::json!({"code": code});
        let result = self.call_python("sandbox_javascript", Some(args))?;
        Ok(SandboxResult::from_json(&result))
    }
    
    /// Bash çalıştır
    pub fn sandbox_bash(&self, command: &str) -> SENTIENTResult<SandboxResult> {
        let args = serde_json::json!({"command": command});
        let result = self.call_python("sandbox_bash", Some(args))?;
        Ok(SandboxResult::from_json(&result))
    }
    
    /// Sandbox kapat
    pub fn sandbox_close(&self) -> SENTIENTResult<SandboxResult> {
        let result = self.call_python("sandbox_close", None)?;
        Ok(SandboxResult::from_json(&result))
    }

    /// Kayıtlı araçların listesini döndür
    pub fn list_tools(&self) -> Vec<&PythonToolDef> {
        self.tools.values().collect()
    }

    /// Araç kayıtlı mı?
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
    
    /// Başlatıldı mı?
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Default for PythonBridge {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Yardımcı Fonksiyonlar ───

fn json_to_pydict<'py>(py: Python<'py>, json: &serde_json::Value) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    
    if let serde_json::Value::Object(map) = json {
        for (k, v) in map {
            let py_val = json_value_to_py(py, v)?;
            dict.set_item(k, py_val)?;
        }
    }
    
    Ok(dict)
}

fn json_value_to_py<'py>(py: Python<'py>, json: &serde_json::Value) -> PyResult<Bound<'py, PyAny>> {
    match json {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => {
            let obj: Bound<'_, PyAny> = pyo3::types::PyBool::new(py, *b).as_any().clone();
            Ok(obj)
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any())
            } else {
                let zero: i32 = 0;
                Ok(zero.into_pyobject(py)?.into_any())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any()),
        serde_json::Value::Array(arr) => {
            let items: Vec<_> = arr.iter().map(|v| json_value_to_py(py, v)).collect::<Result<Vec<_>, _>>()?;
            let list: Bound<'_, PyList> = PyList::new(py, &items)?;
            Ok(list.into_any())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}

fn py_value_to_json<'py>(py: Python<'py>, value: &Bound<'py, PyAny>) -> SENTIENTResult<serde_json::Value> {
    if value.is_none() {
        return Ok(serde_json::Value::Null);
    }
    
    // Dict ise
    if let Ok(dict) = value.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract().map_err(|e| SENTIENTError::PythonBridge(format!("Dict key hatası: {}", e)))?;
            let val = py_value_to_json(py, &v)?;
            map.insert(key, val);
        }
        return Ok(serde_json::Value::Object(map));
    }
    
    // List ise
    if let Ok(list) = value.downcast::<PyList>() {
        let arr: Vec<serde_json::Value> = list.iter()
            .map(|v| py_value_to_json(py, &v))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(serde_json::Value::Array(arr));
    }
    
    // String ise
    if let Ok(s) = value.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    
    // Bool ise
    if let Ok(b) = value.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    
    // Int ise
    if let Ok(i) = value.extract::<i64>() {
        return Ok(serde_json::Value::Number(i.into()));
    }
    
    // Float ise
    if let Ok(f) = value.extract::<f64>() {
        return Ok(serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null));
    }
    
    // Bilinmeyen tip - string'e çevir
    Ok(serde_json::Value::String(value.to_string()))
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_creation() {
        let bridge = PythonBridge::new();
        assert!(bridge.list_tools().is_empty());
    }

    #[test]
    fn test_register_tool() {
        let mut bridge = PythonBridge::new();
        let tool = PythonToolDef::new("test_tool", "test_module", "test_func", "Test aracı");
        bridge.register_tool(tool);
        assert!(bridge.has_tool("test_tool"));
        assert!(!bridge.has_tool("nonexistent"));
    }
    
    #[test]
    fn test_register_browser_tools() {
        let mut bridge = PythonBridge::new();
        bridge.register_browser_tools();
        assert!(bridge.has_tool("browser_init"));
        assert!(bridge.has_tool("browser_navigate"));
        assert!(bridge.has_tool("browser_search"));
        assert!(bridge.has_tool("browser_research"));
        assert!(bridge.has_tool("browser_screenshot"));
        assert!(bridge.has_tool("browser_close"));
    }
    
    #[test]
    fn test_browser_result_from_json() {
        let json = serde_json::json!({
            "success": true,
            "content": "Test içeriği",
            "url": "https://example.com",
            "error": null,
            "timestamp": "2024-01-01T00:00:00"
        });
        
        let result = BrowserResult::from_json(&json);
        assert!(result.is_ok());
        assert_eq!(result.content, "Test içeriği");
        assert_eq!(result.url, Some("https://example.com".to_string()));
    }
    
    #[test]
    fn test_browser_result_error() {
        let json = serde_json::json!({
            "success": false,
            "content": "",
            "error": "Bağlantı hatası"
        });
        
        let result = BrowserResult::from_json(&json);
        assert!(!result.is_ok());
        assert_eq!(result.error, Some("Bağlantı hatası".to_string()));
    }

    #[test]
    fn test_call_unknown_tool() {
        let bridge = PythonBridge::new();
        let result = bridge.call_python("unknown", None);
        assert!(result.is_err());
    }
}
