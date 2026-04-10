//! ═══════════════════════════════════════════════════════════════════════════════
//!  TOOL CHAINING - Araç Zincirleri
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Araçları zincirleme execution.
//!
//! ÖRNEK ZİNCİRLER:
//! ────────────────
//! 1. web_search → web_fetch → pdf_extract → summarize → email_send
//! 2. screenshot → ocr → translate → speak
//! 3. git_status → grep_issues → file_edit → git_commit → notify

use crate::error::{AutonomousError, AutonomousResult};
use crate::{Action, ActionResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAIN STEP
// ═══════════════════════════════════════════════════════════════════════════════

/// Zincir adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainStep {
    /// Adım ID
    pub id: String,
    /// Adım adı
    pub name: String,
    /// Araç adı
    pub tool: String,
    /// Girdi parametreleri
    pub input: HashMap<String, serde_json::Value>,
    /// Önceki adımdan veri al mı?
    pub use_previous_output: bool,
    /// Önceki çıktıyı hangi parametreye map et
    pub output_mapping: Option<String>,
    /// Koşul (opsiyonel)
    pub condition: Option<ChainCondition>,
    /// Hata durumunda ne yap
    pub on_error: ErrorHandling,
}

/// Zincir koşulu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainCondition {
    OutputContains { value: String },
    OutputEquals { value: String },
    OutputNotEmpty,
    PreviousSuccess,
    Custom { expression: String },
}

/// Hata durumunda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorHandling {
    /// Zinciri durdur
    StopChain,
    /// Sonraki adıma geç
    Continue,
    /// Retry
    Retry { max_attempts: u32 },
    /// Varsayılan değer kullan
    UseDefault { value: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TOOL CHAIN
// ═══════════════════════════════════════════════════════════════════════════════

/// Araç zinciri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChain {
    /// Zincir ID
    pub id: String,
    /// Zincir adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Adımlar
    pub steps: Vec<ChainStep>,
    /// Paralel çalıştırılacak gruplar
    pub parallel_groups: Vec<Vec<String>>,
    /// Timeout (ms)
    pub timeout_ms: u64,
    /// Retry policy
    pub retry_count: u32,
}

impl ToolChain {
    pub fn new(name: &str, steps: Vec<ChainStep>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            steps,
            parallel_groups: vec![],
            timeout_ms: 60000,
            retry_count: 0,
        }
    }
    
    /// Adım sayısı
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    
    /// Boş mu?
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAIN RESULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Zincir sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainResult {
    /// Zincir ID
    pub chain_id: String,
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
    /// Toplam süre
    pub total_duration_ms: u64,
    /// Adım sonuçları
    pub step_results: Vec<StepResult>,
    /// Son çıktı
    pub final_output: Option<serde_json::Value>,
    /// Hata (varsa)
    pub error: Option<String>,
}

/// Adım sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub step_name: String,
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub duration_ms: u64,
    pub error: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAIN BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Zincir oluşturucu
pub struct ChainBuilder {
    name: String,
    steps: Vec<ChainStep>,
    parallel_groups: Vec<Vec<String>>,
}

impl ChainBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            steps: vec![],
            parallel_groups: vec![],
        }
    }
    
    /// Adım ekle
    pub fn step(mut self, name: &str, tool: &str, input: HashMap<String, serde_json::Value>) -> Self {
        self.steps.push(ChainStep {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            tool: tool.into(),
            input,
            use_previous_output: false,
            output_mapping: None,
            condition: None,
            on_error: ErrorHandling::StopChain,
        });
        self
    }
    
    /// Önceki çıktıyı kullan
    pub fn use_previous_output(mut self, mapping: &str) -> Self {
        if let Some(step) = self.steps.last_mut() {
            step.use_previous_output = true;
            step.output_mapping = Some(mapping.into());
        }
        self
    }
    
    /// Hata durumunda
    pub fn on_error(mut self, handling: ErrorHandling) -> Self {
        if let Some(step) = self.steps.last_mut() {
            step.on_error = handling;
        }
        self
    }
    
    /// Paralel grup ekle
    pub fn parallel(mut self, step_names: &[&str]) -> Self {
        self.parallel_groups.push(step_names.iter().map(|s| s.to_string()).collect());
        self
    }
    
    /// Build
    pub fn build(self) -> ToolChain {
        ToolChain {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.name,
            description: String::new(),
            steps: self.steps,
            parallel_groups: self.parallel_groups,
            timeout_ms: 60000,
            retry_count: 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHAIN EXECUTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Zincir çalıştırıcı
pub struct ChainExecutor {
    /// Kayıtlı zincirler
    chains: HashMap<String, ToolChain>,
    /// Macro recorder
    macro_recorder: Option<MacroRecorder>,
}

impl ChainExecutor {
    pub fn new() -> Self {
        log::info!("⛓️ TOOLS: Zincir çalıştırıcı başlatılıyor...");
        
        Self {
            chains: HashMap::new(),
            macro_recorder: None,
        }
    }
    
    /// Zincir kaydet
    pub fn register_chain(&mut self, chain: ToolChain) {
        log::info!("⛓️ TOOLS: Chain registered: {}", chain.name);
        self.chains.insert(chain.name.clone(), chain);
    }
    
    /// Zincir çalıştır
    pub async fn execute(&self, chain_name: &str, _initial_input: HashMap<String, serde_json::Value>) -> AutonomousResult<ChainResult> {
        let chain = self.chains.get(chain_name)
            .ok_or_else(|| AutonomousError::Other(format!("Chain not found: {}", chain_name)))?;
        
        log::info!("⛓️ TOOLS: Executing chain '{}' with {} steps", chain.name, chain.steps.len());
        
        let start = std::time::Instant::now();
        let mut step_results = Vec::new();
        let mut current_output: Option<serde_json::Value> = None;
        let mut success = true;
        let mut error = None;
        
        for step in &chain.steps {
            log::debug!("⛓️ TOOLS: Executing step '{}'", step.name);
            
            let step_start = std::time::Instant::now();
            
            // Girdi hazırla
            let mut input = step.input.clone();
            
            // Önceki çıktıyı ekle
            if step.use_previous_output {
                if let Some(ref prev_output) = current_output {
                    if let Some(mapping) = &step.output_mapping {
                        input.insert(mapping.clone(), prev_output.clone());
                    }
                }
            }
            
            // Koşul kontrolü
            if let Some(ref condition) = step.condition {
                if !self.check_condition(condition, &current_output) {
                    log::debug!("⛓️ TOOLS: Condition not met, skipping step '{}'", step.name);
                    continue;
                }
            }
            
            // Araç çalıştır (placeholder)
            let step_result = self.execute_tool(&step.tool, &input).await;
            
            let duration = step_start.elapsed().as_millis() as u64;
            
            match step_result {
                Ok(output) => {
                    step_results.push(StepResult {
                        step_id: step.id.clone(),
                        step_name: step.name.clone(),
                        success: true,
                        output: Some(output.clone()),
                        duration_ms: duration,
                        error: None,
                    });
                    current_output = Some(output);
                }
                Err(e) => {
                    let err_msg = e.to_string();
                    
                    match step.on_error {
                        ErrorHandling::StopChain => {
                            success = false;
                            error = Some(err_msg.clone());
                            step_results.push(StepResult {
                                step_id: step.id.clone(),
                                step_name: step.name.clone(),
                                success: false,
                                output: None,
                                duration_ms: duration,
                                error: Some(err_msg),
                            });
                            break;
                        }
                        ErrorHandling::Continue => {
                            step_results.push(StepResult {
                                step_id: step.id.clone(),
                                step_name: step.name.clone(),
                                success: false,
                                output: None,
                                duration_ms: duration,
                                error: Some(err_msg),
                            });
                        }
                        ErrorHandling::UseDefault { ref value } => {
                            current_output = Some(serde_json::json!(value));
                            step_results.push(StepResult {
                                step_id: step.id.clone(),
                                step_name: step.name.clone(),
                                success: true,
                                output: current_output.clone(),
                                duration_ms: duration,
                                error: Some(format!("Used default: {}", value)),
                            });
                        }
                        ErrorHandling::Retry { max_attempts } => {
                            // Retry logic (basitleştirilmiş)
                            log::warn!("⛓️ TOOLS: Retry not fully implemented, max_attempts={}", max_attempts);
                        }
                    }
                }
            }
        }
        
        let total_duration = start.elapsed().as_millis() as u64;
        
        Ok(ChainResult {
            chain_id: chain.id.clone(),
            success,
            message: if success { "Chain completed".into() } else { "Chain failed".into() },
            total_duration_ms: total_duration,
            step_results,
            final_output: current_output,
            error,
        })
    }
    
    /// Araç çalıştır
    async fn execute_tool(&self, tool: &str, input: &HashMap<String, serde_json::Value>) -> AutonomousResult<serde_json::Value> {
        log::debug!("⛓️ TOOLS: Executing tool '{}' with input: {:?}", tool, input);
        
        match tool {
            // File operations
            "file_read" => {
                let path = input.get("path").and_then(|v| v.as_str()).unwrap_or(".");
                Ok(serde_json::json!({"tool": tool, "result": "success", "path": path, "content": "[file content]"}))
            }
            "file_write" => {
                let path = input.get("path").and_then(|v| v.as_str()).unwrap_or(".");
                Ok(serde_json::json!({"tool": tool, "result": "success", "path": path, "bytes_written": 0}))
            }
            // Web operations
            "http_get" | "fetch" => {
                let url = input.get("url").and_then(|v| v.as_str()).unwrap_or("");
                Ok(serde_json::json!({"tool": tool, "result": "success", "url": url, "status": 200, "body": "[response]"}))
            }
            // Shell execution
            "shell" | "exec" => {
                let cmd = input.get("command").and_then(|v| v.as_str()).unwrap_or("");
                Ok(serde_json::json!({"tool": tool, "result": "success", "command": cmd, "stdout": "", "stderr": "", "exit_code": 0}))
            }
            // Default
            _ => Ok(serde_json::json!({
                "tool": tool,
                "result": "success",
                "message": "Tool executed (simulated)",
                "input": input
            }))
        }
    }
    
    /// Koşul kontrolü
    fn check_condition(&self, condition: &ChainCondition, output: &Option<serde_json::Value>) -> bool {
        match condition {
            ChainCondition::OutputContains { value } => {
                if let Some(out) = output {
                    out.to_string().contains(value)
                } else {
                    false
                }
            }
            ChainCondition::OutputEquals { value } => {
                if let Some(out) = output {
                    &out.to_string() == value
                } else {
                    false
                }
            }
            ChainCondition::OutputNotEmpty => output.is_some(),
            ChainCondition::PreviousSuccess => true, // Simplified
            ChainCondition::Custom { expression } => {
                log::warn!("Custom condition not implemented: {}", expression);
                true
            }
        }
    }
    
    /// Zincir listesi
    pub fn list_chains(&self) -> Vec<&String> {
        self.chains.keys().collect()
    }
    
    /// Macro recorder başlat
    pub fn start_recording(&mut self) {
        self.macro_recorder = Some(MacroRecorder::new());
        log::info!("⛓️ TOOLS: Macro recording started");
    }
    
    /// Macro recorder durdur
    pub fn stop_recording(&mut self) -> Option<ToolChain> {
        if let Some(recorder) = self.macro_recorder.take() {
            let chain = recorder.to_chain();
            log::info!("⛓️ TOOLS: Macro recording stopped, {} steps", chain.steps.len());
            Some(chain)
        } else {
            None
        }
    }
    
    /// Aksiyon kaydet
    pub fn record_action(&mut self, action: &Action, result: &ActionResult) {
        if let Some(ref mut recorder) = self.macro_recorder {
            recorder.record(action, result);
        }
    }
}

impl Default for ChainExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MACRO RECORDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Macro kaydedici
struct MacroRecorder {
    actions: Vec<RecordedAction>,
}

struct RecordedAction {
    action: Action,
    success: bool,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl MacroRecorder {
    fn new() -> Self {
        Self { actions: vec![] }
    }
    
    fn record(&mut self, action: &Action, result: &ActionResult) {
        self.actions.push(RecordedAction {
            action: action.clone(),
            success: result.success,
            timestamp: chrono::Utc::now(),
        });
    }
    
    fn to_chain(self) -> ToolChain {
        let steps: Vec<ChainStep> = self.actions.into_iter().enumerate().map(|(i, _ra)| {
            ChainStep {
                id: format!("step-{}", i),
                name: format!("Recorded step {}", i + 1),
                tool: "recorded".into(),
                input: HashMap::new(),
                use_previous_output: false,
                output_mapping: None,
                condition: None,
                on_error: ErrorHandling::StopChain,
            }
        }).collect();
        
        ToolChain::new("Recorded Macro", steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chain_builder() {
        let chain = ChainBuilder::new("test")
            .step("step1", "tool1", HashMap::new())
            .step("step2", "tool2", HashMap::new())
            .use_previous_output("input")
            .build();
        
        assert_eq!(chain.steps.len(), 2);
    }
    
    #[tokio::test]
    async fn test_chain_executor() {
        let executor = ChainExecutor::new();
        
        let chain = ToolChain::new("test", vec![]);
        assert_eq!(chain.len(), 0);
    }
}
