//! ─── AGENT (ANA DÖNGÜ) ───
//!
//! SENTIENT'nın otonom görev döngüsü - beyni, elleri ve gözleri
//! tek bir merkezi orkestrasyon sisteminde birleştirir.

use crate::goal::{Goal, Task};
use crate::planner::{ExecutionPlan, Planner};
use crate::state::{AgentContext, AgentState, ToolCall, ToolResult};
use crate::tools::Toolbox;
use crate::execution::{ExecutionResult, StepResult};
use sentient_common::error::{SENTIENTError, SENTIENTResult};

/// SENTIENT Sistem Promptu
pub const SYSTEM_PROMPT: &str = r#"
Sen SENTIENT'sın — NEXUS OASIS Yapay Zeka İşletim Sistemi.

Kimliğin:
- Otonom bir yapay zeka asistanısın
- Kullanıcıya yardım etmek için tasarlandın
- Araçlarını akıllıca kullanarak görevleri yerine getirirsin

Araçların:
1. llm_query - Basit sorular için
2. llm_reason - Karmaşık problemler için
3. web_search - Web'de arama yapmak için
4. browser_navigate - Web sayfasına gitmek için
5. browser_click - Sayfada tıklamak için
6. browser_extract - Sayfadan veri çıkarmak için
7. sandbox_execute - Kod çalıştırmak için
8. memory_store - Bilgi kaydetmek için
9. memory_recall - Bilgi hatırlamak için
10. calculator - Matematik hesaplamak için

Kurallar:
1. Her zaman mantıklı ve adım adım düşün
2. Belirsiz durumda kullanıcıya sor
3. Hata yaparsan yeniden dene
4. Sonuçları doğrula
5. Önemli bilgileri belleğe kaydet
"#;

/// ─── AGENT ───
/// 
/// SENTIENT'nın otonom ajanı. Verilen bir hedefi kendi kendine
/// planlayıp, araçları kullanarak gerçekleştirir.

pub struct Agent {
    /// Hedef
    goal: Goal,
    /// Yapılandırma
    config: AgentConfig,
    /// Bağlam (çalışma zamanı durumu)
    context: AgentContext,
    /// Planlayıcı
    planner: Planner,
    /// Araç kutusu
    toolbox: Toolbox,
    /// Durum
    state: AgentState,
}

/// Agent yapılandırması
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Kullanılacak LLM modeli
    pub model: String,
    /// V-GATE sunucu adresi
    pub vgate_url: String,
    /// Maksimum iterasyon sayısı
    pub max_iterations: u32,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Sistem promptu
    pub system_prompt: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model: "qwen/qwen3-1.7b:free".into(),
            vgate_url: "http://127.0.0.1:1071".into(),
            max_iterations: 50,
            timeout_secs: 300,
            system_prompt: SYSTEM_PROMPT.into(),
        }
    }
}

impl Agent {
    /// Yeni ajan oluştur
    pub fn new(goal: Goal, config: AgentConfig) -> Self {
        let mut context = AgentContext::new();
        context.set_goal(goal.clone());
        context.system_message(config.system_prompt.clone());
        
        let mut toolbox = Toolbox::new();
        Self::register_default_tools(&mut toolbox);
        
        Self {
            goal,
            config,
            context,
            planner: Planner::new(),
            toolbox,
            state: AgentState::Idle,
        }
    }
    
    /// Varsayılan araçları kaydet
    fn register_default_tools(toolbox: &mut Toolbox) {
        use crate::tools::*;
        
        toolbox.register(LlmQueryTool);
        toolbox.register(WebSearchTool);
        toolbox.register(BrowserNavigateTool);
        toolbox.register(SandboxExecuteTool);
        toolbox.register(MemoryStoreTool);
        toolbox.register(MemoryRecallTool);
        toolbox.register(CalculatorTool);
    }
    
    /// ─── ANA DÖNGÜ ───
    /// 
    /// SENTIENT'nın kalbi - hedefe ulaşana kadar çalışan döngü.
    
    pub async fn run(&mut self) -> SENTIENTResult<ExecutionResult> {
        let start_time = std::time::Instant::now();
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🐺  SENTIENT AJAN BAŞLATILIYOR");
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🎯  Hedef: {}", self.goal.description.chars().take(60).collect::<String>());
        log::info!("🧠  Model: {}", self.config.model);
        log::info!("🔄  Maks iterasyon: {}", self.config.max_iterations);
        log::info!("════════════════════════════════════════════════════════════");
        
        // 1. Hedefi analiz et
        self.state = AgentState::Analyzing;
        self.analyze_goal().await?;
        
        // 2. Plan oluştur
        self.state = AgentState::Planning;
        let plan = self.create_plan();
        self.context.set_plan(plan);
        
        log::info!("📋  Plan oluşturuldu: {} görev", self.context.current_plan.as_ref().expect("operation failed").tasks.len());
        
        // 3. Ana döngü
        self.state = AgentState::Acting;
        let mut iteration = 0;
        let mut step_results: Vec<StepResult> = Vec::new();
        let mut errors: Vec<String> = Vec::new();
        
        'main_loop: while iteration < self.config.max_iterations {
            iteration += 1;
            self.context.increment_iteration();
            
            log::debug!("🔄  İterasyon {}", iteration);
            
            // Timeout kontrolü
            if start_time.elapsed().as_secs() > self.config.timeout_secs {
                log::warn!("⏰  Zaman aşımı!");
                self.state = AgentState::Timeout;
                break 'main_loop;
            }
            
            // Sonraki görevi al
            let next_task_opt = {
                let plan = self.context.current_plan.as_ref().expect("operation failed");
                plan.next_task().cloned()
            };
            
            match next_task_opt {
                Some(task) => {
                    let task_id = task.id;
                    let task_desc = task.description.clone();
                    
                    // Görevi çalıştır
                    let result = self.execute_task(&task).await;
                    
                    match result {
                        Ok(step_result) => {
                            self.context.complete_task(task_id);
                            step_results.push(step_result);
                            
                            log::info!(
                                "  ✓ {}",
                                task_desc.chars().take(40).collect::<String>()
                            );
                        }
                        Err(e) => {
                            self.context.fail_task(task_id);
                            errors.push(e.to_sentient_message());
                            
                            log::warn!(
                                "  ✗ {} → {}",
                                task_desc.chars().take(30).collect::<String>(),
                                e.to_sentient_message()
                            );
                            
                            // Maksimum hata kontrolü
                            if errors.len() >= 5 {
                                log::error!("❌  Çok fazla hata, iptal ediliyor");
                                self.state = AgentState::Error;
                                break 'main_loop;
                            }
                        }
                    }
                }
                None => {
                    // Tüm görevler tamamlandı
                    log::info!("✅  Tüm görevler tamamlandı");
                    self.state = AgentState::Completed;
                    break 'main_loop;
                }
            }
            
            // Ara değerlendirme
            if iteration % 5 == 0 {
                self.evaluate_progress().await?;
            }
        }
        
        // Sonuç oluştur
        let duration_ms = start_time.elapsed().as_millis() as u64;
        
        let result = if self.state == AgentState::Completed {
            ExecutionResult::success(
                &self.goal,
                step_results,
                iteration,
                self.context.total_tokens,
                duration_ms,
            )
        } else {
            ExecutionResult::failure(
                &self.goal,
                errors.join("; "),
                step_results.clone(),
                step_results.into_iter().filter(|r| !r.success).collect(),
                iteration,
                self.context.total_tokens,
                duration_ms,
                self.state,
            )
        };
        
        log::info!("{}", result.report());
        
        Ok(result)
    }
    
    /// Hedefi analiz et
    async fn analyze_goal(&self) -> SENTIENTResult<()> {
        log::info!("🔍  Hedef analiz ediliyor...");
        
        // Basit analiz - ileride LLM ile güçlendirilecek
        let keywords = self.extract_keywords(&self.goal.description);
        
        log::debug!("📝  Anahtar kelimeler: {:?}", keywords);
        
        Ok(())
    }
    
    /// Anahtar kelimeleri çıkar
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .filter(|w| !["için", "olan", "ile", "ve", "veya", "bir", "bu", "şu"].contains(w))
            .take(10)
            .map(String::from)
            .collect()
    }
    
    /// Plan oluştur
    fn create_plan(&self) -> ExecutionPlan {
        self.planner.plan(&self.goal)
    }
    
    /// Görevi çalıştır
    async fn execute_task(&mut self, task: &Task) -> SENTIENTResult<StepResult> {
        let start = std::time::Instant::now();
        
        log::debug!("🔧  Araç çalıştırılıyor: {:?}", task.tool);
        
        // Aracı çalıştır
        let result = self.toolbox.execute(
            &task.tool.to_string().to_lowercase(),
            task.input.clone()
        ).await;
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        // Sonucu bağlama kaydet
        self.context.last_tool_call = Some(ToolCall::new(
            task.tool.to_string(),
            task.input.clone()
        ));
        
        let step_result = if result.success {
            self.context.last_tool_result = Some(ToolResult::success(
                self.context.last_tool_call.as_ref().expect("operation failed").id.clone(),
                result.output.clone(),
                duration_ms
            ));
            
            StepResult::from_task_with_result(task, result.output, duration_ms, true, None)
        } else {
            self.context.last_tool_result = Some(ToolResult::error(
                self.context.last_tool_call.as_ref().expect("operation failed").id.clone(),
                result.error.clone().unwrap_or_default(),
                duration_ms
            ));
            
            StepResult::from_task_with_result(
                task,
                serde_json::Value::Null,
                duration_ms,
                false,
                result.error
            )
        };
        
        Ok(step_result)
    }
    
    /// İlerlemeyi değerlendir
    async fn evaluate_progress(&self) -> SENTIENTResult<()> {
        let summary = self.context.summary();
        
        log::debug!(
            "📊  İlerleme: {} görev tamamlandı, {} başarısız, {} bekliyor",
            summary.completed_tasks,
            summary.failed_tasks,
            summary.pending_tasks
        );
        
        Ok(())
    }
    
    /// Araç kullan (LLM kararı ile)
    pub async fn use_tool(&mut self, tool_name: &str, params: serde_json::Value) -> SENTIENTResult<serde_json::Value> {
        let result = self.toolbox.execute(tool_name, params).await;
        
        if result.success {
            Ok(result.output)
        } else {
            Err(SENTIENTError::VGate(result.error.unwrap_or_else(|| "Bilinmeyen hata".into())))
        }
    }
    
    /// Durum al
    pub fn state(&self) -> AgentState {
        self.state
    }
    
    /// Bağlam al
    pub fn context(&self) -> &AgentContext {
        &self.context
    }
}

impl StepResult {
    pub fn from_task_with_result(
        task: &Task,
        output: serde_json::Value,
        duration_ms: u64,
        success: bool,
        error: Option<String>
    ) -> Self {
        Self {
            task_id: task.id,
            description: task.description.clone(),
            tool: format!("{:?}", task.tool),
            input: task.input.clone(),
            output,
            success,
            error,
            duration_ms,
            tokens_used: None,
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.max_iterations, 50);
        assert!(!config.model.is_empty());
    }
    
    #[test]
    fn test_agent_creation() {
        let goal = Goal::new("Test hedefi");
        let config = AgentConfig::default();
        let agent = Agent::new(goal, config);
        
        assert_eq!(agent.state(), AgentState::Idle);
    }
    
    #[test]
    fn test_keyword_extraction() {
        let goal = Goal::new("Yapay zeka hakkında araştırma yap");
        let config = AgentConfig::default();
        let agent = Agent::new(goal, config);
        
        let keywords = agent.extract_keywords("Yapay zeka ve machine learning hakkında araştırma yap");
        assert!(keywords.contains(&"yapay".to_string()));
    }
    
    #[tokio::test]
    async fn test_tool_execution() {
        let goal = Goal::new("Test");
        let mut agent = Agent::new(goal, AgentConfig::default());
        
        let result = agent.use_tool("calculator", serde_json::json!({"expression": "2+2"})).await;
        
        // Calculator simülasyonu dönecek
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_unknown_tool() {
        let goal = Goal::new("Test");
        let mut agent = Agent::new(goal, AgentConfig::default());
        
        let result = agent.use_tool("unknown_tool", serde_json::Value::Null).await;
        
        assert!(result.is_err());
    }
}
