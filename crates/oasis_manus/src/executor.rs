//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS EXECUTOR - Kod Çalıştırma Motoru
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Docker container içinde kod çalıştırma.

use crate::error::{ManusError, ManusResult, translate_error};
use crate::container::ContainerPool;
use crate::Language;
use bollard::exec::{CreateExecOptions, StartExecResults};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// ─── CODE EXECUTOR ───

pub struct CodeExecutor {
    /// Varsayılan timeout (saniye)
    default_timeout: u64,
    /// Simülasyon modu
    simulation: bool,
}

/// Çalıştırma sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Container ID
    pub container_id: String,
    /// Başarılı mı?
    pub success: bool,
    /// Çıkış kodu
    pub exit_code: Option<i32>,
    /// Stdout
    pub stdout: String,
    /// Stderr
    pub stderr: String,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Hata mesajı
    pub error: Option<String>,
    /// Dil
    pub language: ExecutionLanguage,
}

/// Çalıştırma dili
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionLanguage {
    Python,
    JavaScript,
    TypeScript,
    Bash,
    Rust,
    Go,
}

impl From<Language> for ExecutionLanguage {
    fn from(lang: Language) -> Self {
        match lang {
            Language::Python => ExecutionLanguage::Python,
            Language::JavaScript => ExecutionLanguage::JavaScript,
            Language::TypeScript => ExecutionLanguage::TypeScript,
            Language::Bash => ExecutionLanguage::Bash,
            Language::Rust => ExecutionLanguage::Rust,
            Language::Go => ExecutionLanguage::Go,
        }
    }
}

/// Kod çalıştırma isteği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// Kod
    pub code: String,
    /// Dil
    pub language: Language,
    /// Timeout (saniye)
    pub timeout_secs: Option<u64>,
    /// Giriş verisi (stdin)
    pub input: Option<String>,
    /// Ortam değişkenleri
    pub env: std::collections::HashMap<String, String>,
}

impl CodeExecutor {
    /// Yeni executor oluştur
    pub fn new(pool: &ContainerPool) -> Self {
        Self {
            default_timeout: 60,
            simulation: pool.is_simulation(),
        }
    }
    
    /// Kod çalıştır
    pub async fn execute(
        &self,
        container_id: &str,
        code: &str,
        language: Language,
    ) -> ManusResult<ExecutionResult> {
        self.execute_with_timeout(container_id, code, language, self.default_timeout).await
    }
    
    /// Timeout ile kod çalıştır
    pub async fn execute_with_timeout(
        &self,
        container_id: &str,
        code: &str,
        language: Language,
        timeout_secs: u64,
    ) -> ManusResult<ExecutionResult> {
        let start = Instant::now();
        
        log::info!("⚡  MANUS-EXEC: Kod çalıştırılıyor ({:?})...", language);
        
        // Simülasyon modu
        if container_id.starts_with("sim_") || self.simulation {
            return self.execute_simulated(container_id, code, language, start).await;
        }
        
        // Docker execution - şu an için simülasyon kullan
        // Gerçek Docker execution için bollard exec API'si gerekli
        self.execute_simulated(container_id, code, language, start).await
    }
    
    /// Simülasyon çalıştırma
    async fn execute_simulated(
        &self,
        container_id: &str,
        code: &str,
        language: Language,
        start: Instant,
    ) -> ManusResult<ExecutionResult> {
        // Basit Python simülasyonu
        let stdout = match language {
            Language::Python => {
                if code.contains("print(") {
                    // print() içeriğini çıkar
                    if let Some(start_idx) = code.find("print(") {
                        let rest = &code[start_idx + 6..];
                        if let Some(end_idx) = rest.find(')') {
                            let content = &rest[..end_idx];
                            content.trim_matches('"').trim_matches('\'').to_string()
                        } else {
                            "[SIM] Python çıktısı".into()
                        }
                    } else {
                        "[SIM] Python çıktısı".into()
                    }
                } else {
                    "[SIM] Python kodu çalıştırıldı".into()
                }
            }
            Language::JavaScript => "[SIM] JavaScript kodu çalıştırıldı".into(),
            Language::Bash => "[SIM] Bash komutu çalıştırıldı".into(),
            _ => format!("[SIM] {:?} kodu çalıştırıldı", language),
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        Ok(ExecutionResult {
            container_id: container_id.into(),
            success: true,
            exit_code: Some(0),
            stdout,
            stderr: String::new(),
            duration_ms,
            error: None,
            language: ExecutionLanguage::from(language),
        })
    }
    
    /// Çoklu kod çalıştır
    pub async fn execute_batch(
        &self,
        container_id: &str,
        requests: Vec<ExecutionRequest>,
    ) -> ManusResult<Vec<ExecutionResult>> {
        let mut results = Vec::new();
        
        for req in requests {
            let timeout = req.timeout_secs.unwrap_or(self.default_timeout);
            let result = self.execute_with_timeout(
                container_id,
                &req.code,
                req.language,
                timeout,
            ).await;
            
            match result {
                Ok(r) => results.push(r),
                Err(e) => {
                    results.push(ExecutionResult {
                        container_id: container_id.into(),
                        success: false,
                        exit_code: None,
                        stdout: String::new(),
                        stderr: e.to_string(),
                        duration_ms: 0,
                        error: Some(e.to_string()),
                        language: ExecutionLanguage::from(req.language),
                    });
                }
            }
        }
        
        Ok(results)
    }
    
    /// Varsayılan timeout'u ayarla
    pub fn set_default_timeout(&mut self, timeout: u64) {
        self.default_timeout = timeout;
    }
    
    /// Varsayılan timeout'u al
    pub fn default_timeout(&self) -> u64 {
        self.default_timeout
    }
}

impl Clone for CodeExecutor {
    fn clone(&self) -> Self {
        Self {
            default_timeout: self.default_timeout,
            simulation: self.simulation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result() {
        let result = ExecutionResult {
            container_id: "test".into(),
            success: true,
            exit_code: Some(0),
            stdout: "Hello".into(),
            stderr: String::new(),
            duration_ms: 100,
            error: None,
            language: ExecutionLanguage::Python,
        };
        
        assert!(result.success);
        assert_eq!(result.stdout, "Hello");
    }

    #[test]
    fn test_execution_language_from() {
        assert_eq!(ExecutionLanguage::from(Language::Python), ExecutionLanguage::Python);
        assert_eq!(ExecutionLanguage::from(Language::JavaScript), ExecutionLanguage::JavaScript);
    }

    #[test]
    fn test_execution_request() {
        let req = ExecutionRequest {
            code: "print('test')".into(),
            language: Language::Python,
            timeout_secs: Some(30),
            input: None,
            env: std::collections::HashMap::new(),
        };
        
        assert_eq!(req.language, Language::Python);
    }
}
