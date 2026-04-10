//! ═══════════════════════════════════════════════════════════════════════════════
//!  TASK PLANNER - Görev Planlama Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Görevleri alt görevlere bölme, planlama ve execution.
//!
//! PLANLAMA SEVİYELERİ:
//! ────────────────────
//! 1. Goal Level    → Kullanıcı hedefi ("Rapor hazırla")
//! 2. Task Level    → Ana görevler ("Veri topla", "PDF oluştur")
//! 3. Step Level    → Adımlar ("Excel'i aç", "Veriyi kopyala")
//! 4. Action Level  → Temel aksiyonlar ("Click", "Type")
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        PLANNING HIERARCHY                               │
//! │                                                                          │
//! │   GOAL: "Rapor hazırla ve email at"                                     │
//! │     │                                                                    │
//! │     ├── TASK 1: Veri topla                                              │
//! │     │     ├── Step 1: Excel'i aç                                        │
//! │     │     │     └── Action: Click("Excel icon")                         │
//! │     │     ├── Step 2: Veriyi çek                                        │
//! │     │     │     └── Action: Type("=QUERY(...)")                         │
//! │     │     └── Step 3: Kaydet                                            │
//! │     │           └── Action: Shortcut(Ctrl+S)                            │
//! │     │                                                                    │
//! │     ├── TASK 2: PDF oluştur                                            │
//! │     │     └── ...                                                        │
//! │     │                                                                    │
//! │     └── TASK 3: Email at                                                │
//! │           └── ...                                                        │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousError, AutonomousResult};
use crate::{Action};
use crate::vision::Observation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  TARGET
// ═══════════════════════════════════════════════════════════════════════════════

/// Hedef belirleme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Target {
    /// Koordinat bazlı
    Position { x: i32, y: i32 },
    
    /// Görsel template matching
    Image { 
        template: String, 
        confidence: f32,
        #[serde(default)]
        timeout_ms: u64,
    },
    
    /// Metin arama
    Text { 
        text: String, 
        #[serde(default)]
        fuzzy: bool,
        #[serde(default)]
        case_sensitive: bool,
    },
    
    /// UI element selector
    Element { 
        selector: String,
        #[serde(default)]
        selector_type: SelectorType,
    },
    
    /// Relatif konum
    Relative { 
        base: Box<Target>, 
        offset: (i32, i32),
    },
    
    /// AI açıklama bazlı
    Description { 
        description: String,
    },
    
    /// Son bilinen pozisyon
    LastKnown { 
        id: String,
    },
}

/// Selector türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SelectorType {
    #[default]
    Css,
    Xpath,
    AccessibilityId,
    TestId,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TASK STEP
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStep {
    // ─────────────────────────────────────────────────────────────────────────
    //  TEMEL AKSİYONLAR
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Tıklama
    Click { 
        target: Target,
        #[serde(default)]
        double: bool,
        #[serde(default)]
        button: crate::MouseButton,
    },
    
    /// Metin yazma
    Type { 
        text: String,
        #[serde(default)]
        target: Option<Target>,
        #[serde(default)]
        human_like: bool,
        #[serde(default)]
        press_enter: bool,
    },
    
    /// Kaydırma
    Scroll {
        direction: ScrollDirection,
        amount: u32,
    },
    
    /// Sürükle bırak
    DragDrop {
        from: Target,
        to: Target,
    },
    
    /// Tuş basma
    KeyPress {
        key: crate::Key,
    },
    
    /// Kısayol
    Shortcut {
        modifiers: Vec<crate::Key>,
        key: crate::Key,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  KOMPOZİT AKSİYONLAR
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Form doldur
    FillForm {
        fields: Vec<FormField>,
    },
    
    /// URL'ye git
    Navigate {
        url: String,
    },
    
    /// Uygulama aç
    LaunchApp {
        app_name: String,
    },
    
    /// Dosya aç
    OpenFile {
        path: String,
    },
    
    /// Dosya kaydet
    SaveFile {
        path: String,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  KOŞULLU ADIMLAR
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Koşullu execution
    If {
        condition: Condition,
        then_steps: Vec<TaskStep>,
        else_steps: Vec<TaskStep>,
    },
    
    /// Döngü
    Repeat {
        count: usize,
        steps: Vec<TaskStep>,
    },
    
    /// While döngüsü
    While {
        condition: Condition,
        steps: Vec<TaskStep>,
        #[serde(default)]
        max_iterations: usize,
    },
    
    /// Her element için
    ForEach {
        selector: String,
        steps: Vec<TaskStep>,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  BEKLEME ADIMLARI
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Sabit bekleme
    Wait {
        duration_ms: u64,
    },
    
    /// Koşul bekleme
    WaitFor {
        condition: WaitCondition,
        timeout_ms: u64,
    },
    
    /// Element bekleme
    WaitForElement {
        target: Target,
        timeout_ms: u64,
    },
    
    /// Sayfa yüklenme bekleme
    WaitForPageLoad {
        timeout_ms: u64,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  VERİ ADIMLARI
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Değişken ata
    SetVariable {
        name: String,
        value: VariableValue,
    },
    
    /// Değişken oku
    GetVariable {
        name: String,
        store_as: String,
    },
    
    /// Metin oku
    ReadText {
        target: Target,
        store_as: String,
    },
    
    /// Veri kaydet
    StoreResult {
        key: String,
        value: String,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  ALT GÖREV
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Alt görev çalıştır
    Subtask {
        name: String,
        params: HashMap<String, serde_json::Value>,
    },
    
    // ─────────────────────────────────────────────────────────────────────────
    //  ÖZEL
    // ─────────────────────────────────────────────────────────────────────────
    
    /// Özel adım
    Custom {
        name: String,
        params: HashMap<String, serde_json::Value>,
    },
    
    /// Hiçbir şey
    NoOp,
}

/// Form alanı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub target: Target,
    pub value: String,
    #[serde(default)]
    pub clear_first: bool,
    #[serde(default)]
    pub press_tab: bool,
}

/// Kaydırma yönü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
    ToElement { target: Target },
}

/// Koşul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Element var mı?
    ElementExists { target: Target },
    /// Element görünür mü?
    ElementVisible { target: Target },
    /// Metin içeriyor mu?
    TextContains { target: Target, text: String },
    /// Değişken eşit mi?
    VariableEquals { name: String, value: String },
    /// Değişken var mı?
    VariableExists { name: String },
    /// URL eşit mi?
    UrlEquals { url: String },
    /// URL içeriyor mu?
    UrlContains { text: String },
    /// Mantıksal VE
    And { conditions: Vec<Condition> },
    /// Mantıksal VEYA
    Or { conditions: Vec<Condition> },
    /// Mantıksal DEĞİL
    Not { condition: Box<Condition> },
    /// Özel koşul
    Custom { expression: String },
}

/// Bekleme koşulu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaitCondition {
    ElementExists { target: Target },
    ElementVisible { target: Target },
    ElementClickable { target: Target },
    TextAppears { text: String },
    UrlChanges { from: String },
    PageLoadComplete,
    NetworkIdle,
    Custom { condition: Condition },
}

/// Değişken değeri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    Static(String),
    FromVariable(String),
    FromExpression(String),
    FromInput(String),
    Generated { template: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TASK
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Görev ID
    pub id: String,
    /// Görev adı
    pub name: String,
    /// Görev açıklaması
    pub description: String,
    /// Görev adımları
    pub steps: Vec<TaskStep>,
    /// Öncelik (1-10)
    pub priority: u8,
    /// Timeout (ms)
    pub timeout_ms: u64,
    /// Retry sayısı
    pub max_retries: u32,
    /// Bağımlılıklar
    pub dependencies: Vec<String>,
    /// Etiketler
    pub tags: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Task {
    /// Yeni görev oluştur
    pub fn new(name: &str, steps: Vec<TaskStep>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            steps,
            priority: 5,
            timeout_ms: 60000,
            max_retries: 3,
            dependencies: vec![],
            tags: vec![],
            metadata: HashMap::new(),
        }
    }
    
    /// Açıklama ekle
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }
    
    /// Öncelik ayarla
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }
    
    /// Timeout ayarla
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TASK PLAN
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev planı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Plan ID
    pub id: String,
    /// Hedef açıklaması
    pub goal: String,
    /// Görevler
    pub tasks: Vec<Task>,
    /// Mevcut görev indeksi
    pub current_task_index: usize,
    /// Mevcut adım indeksi
    pub current_step_index: usize,
    /// Durum
    pub status: PlanStatus,
    /// Değişkenler
    pub variables: HashMap<String, serde_json::Value>,
    /// Sonuçlar
    pub results: HashMap<String, serde_json::Value>,
}

/// Plan durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TASK PLANNER
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev planlayıcı
pub struct TaskPlanner {
    /// Mevcut plan
    current_plan: Option<TaskPlan>,
    /// Kayıtlı görev şablonları
    task_templates: HashMap<String, Task>,
    /// Execution geçmişi
    history: Vec<ExecutionRecord>,
    /// Debug modu
    debug_mode: bool,
}

/// Execution kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExecutionRecord {
    timestamp: chrono::DateTime<chrono::Utc>,
    step: TaskStep,
    success: bool,
    duration_ms: u64,
    error: Option<String>,
}

impl TaskPlanner {
    pub fn new() -> Self {
        log::info!("📋 PLANNER: Görev planlayıcı başlatılıyor...");
        
        let mut planner = Self {
            current_plan: None,
            task_templates: HashMap::new(),
            history: Vec::new(),
            debug_mode: false,
        };
        
        // Varsayılan şablonları ekle
        planner.register_default_templates();
        
        planner
    }
    
    /// Varsayılan şablonları kaydet
    fn register_default_templates(&mut self) {
        // Login şablonu
        self.task_templates.insert("login".into(), Task::new(
            "Login",
            vec![
                TaskStep::Click { 
                    target: Target::Description { description: "username input".into() },
                    double: false,
                    button: crate::MouseButton::Left,
                },
                TaskStep::Type { 
                    text: "${username}".into(),
                    target: None,
                    human_like: true,
                    press_enter: false,
                },
                TaskStep::Click { 
                    target: Target::Description { description: "password input".into() },
                    double: false,
                    button: crate::MouseButton::Left,
                },
                TaskStep::Type { 
                    text: "${password}".into(),
                    target: None,
                    human_like: true,
                    press_enter: false,
                },
                TaskStep::Click { 
                    target: Target::Description { description: "login button".into() },
                    double: false,
                    button: crate::MouseButton::Left,
                },
            ],
        ).with_description("Login form doldur"));
        
        // Email gönder şablonu
        self.task_templates.insert("send_email".into(), Task::new(
            "Send Email",
            vec![
                TaskStep::Shortcut { 
                    modifiers: vec![crate::Key::Ctrl],
                    key: crate::Key::Char('n'),
                },
                TaskStep::Type { 
                    text: "${to}".into(),
                    target: Some(Target::Description { description: "to field".into() }),
                    human_like: true,
                    press_enter: true,
                },
                TaskStep::Type { 
                    text: "${subject}".into(),
                    target: Some(Target::Description { description: "subject field".into() }),
                    human_like: true,
                    press_enter: false,
                },
                TaskStep::Type { 
                    text: "${body}".into(),
                    target: Some(Target::Description { description: "body field".into() }),
                    human_like: true,
                    press_enter: false,
                },
                TaskStep::Shortcut { 
                    modifiers: vec![crate::Key::Ctrl],
                    key: crate::Key::Char('d'),
                },
            ],
        ).with_description("Email gönder"));
    }
    
    /// Sonraki aksiyonu planla
    pub async fn plan_next(&mut self, observation: &Observation, goal: &str) -> AutonomousResult<Action> {
        log::debug!("📋 PLANNER: Planning next action for goal: {}", goal);
        
        // Plan yoksa oluştur
        if self.current_plan.is_none() {
            self.current_plan = Some(self.create_plan(goal, observation).await?);
        }
        
        // Loop instead of recursion to avoid boxing
        loop {
            let plan = self.current_plan.as_mut().expect("operation failed");
            
            // Plan tamamlandı mı?
            if plan.status == PlanStatus::Completed || plan.status == PlanStatus::Failed {
                return Ok(Action::Stop { reason: "Plan completed".into() });
            }
            
            // Mevcut görevi al
            let task_index = plan.current_task_index;
            let step_index = plan.current_step_index;
            
            if let Some(task) = plan.tasks.get(task_index) {
                // Mevcut adımı al
                if step_index < task.steps.len() {
                    let step = task.steps[step_index].clone();
                    let variables = plan.variables.clone();
                    let steps_len = task.steps.len();
                    
                    // Adımı aksiyona çevir
                    let action = self.step_to_action(&step, observation, &variables)?;
                    
                    log::info!("📋 PLANNER: Step {}/{} → {:?}", 
                        step_index + 1, 
                        steps_len,
                        action.describe()
                    );
                    
                    return Ok(action);
                } else {
                    // Görev tamamlandı, sonraki göreve geç
                    plan.current_task_index += 1;
                    plan.current_step_index = 0;
                    
                    if plan.current_task_index >= plan.tasks.len() {
                        plan.status = PlanStatus::Completed;
                        return Ok(Action::Stop { reason: "All tasks completed".into() });
                    }
                    // Continue loop for next task
                    continue;
                }
            }
            
            // Plan bulunamadı, AI ile tahmin et
            return self.ai_plan(observation, goal).await;
        }
    }
    
    /// Plan oluştur
    async fn create_plan(&self, goal: &str, observation: &Observation) -> AutonomousResult<TaskPlan> {
        log::info!("📋 PLANNER: Creating plan for: {}", goal);
        
        // LLM ile görev ayrıştırma (önce dene)
        let tasks = match self.llm_decompose(goal, observation).await {
            Ok(t) => t,
            Err(_) => self.decompose_goal(goal, observation), // Fallback
        };
        
        Ok(TaskPlan {
            id: uuid::Uuid::new_v4().to_string(),
            goal: goal.into(),
            tasks,
            current_task_index: 0,
            current_step_index: 0,
            status: PlanStatus::Pending,
            variables: HashMap::new(),
            results: HashMap::new(),
        })
    }
    
    /// LLM-based task decomposition
    async fn llm_decompose(&self, goal: &str, _observation: &Observation) -> AutonomousResult<Vec<Task>> {
        #[derive(serde::Serialize)]
        struct DecomposeRequest {
            model: String,
            messages: Vec<Message>,
            max_tokens: u32,
            temperature: f32,
        }
        
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Message {
            role: String,
            content: String,
        }
        
        #[derive(serde::Deserialize)]
        struct DecomposeResponse {
            choices: Vec<DecomposeChoice>,
        }
        
        #[derive(serde::Deserialize)]
        struct DecomposeChoice {
            message: Message,
        }
        
        let request = DecomposeRequest {
            model: "llama3".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Break down the goal into steps. Each step on a new line. Format: STEP_NAME: description".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Goal: {}", goal),
                },
            ],
            max_tokens: 500,
            temperature: 0.3,
        };
        
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:11434/v1/chat/completions")
            .json(&request)
            .timeout(std::time::Duration::from_secs(15))
            .send()
            .await
            .map_err(|_| AutonomousError::PlanningFailed("LLM unavailable".into()))?;
        
        if !response.status().is_success() {
            return Err(AutonomousError::PlanningFailed("LLM request failed".into()));
        }
        
        let data: DecomposeResponse = response
            .json()
            .await
            .map_err(|_| AutonomousError::PlanningFailed("Parse error".into()))?;
        
        // Parse steps from response
        let steps: Vec<TaskStep> = data.choices
            .first()
            .map(|c| c.message.content.lines()
                .filter(|l| l.contains(":"))
                .map(|line| {
                    let parts: Vec<&str> = line.splitn(2, ':').collect();
                    TaskStep::Custom {
                        name: parts.get(0).unwrap_or(&"").trim().to_string(),
                        params: vec![("desc".into(), serde_json::json!(parts.get(1).unwrap_or(&"").trim()))].into_iter().collect(),
                    }
                })
                .collect())
            .unwrap_or_default();
        
        if steps.is_empty() {
            return Err(AutonomousError::PlanningFailed("No steps generated".into()));
        }
        
        Ok(vec![Task::new("LLM Generated Task", steps)])
    }
    
    /// LLM-based planning
    async fn llm_plan(&self, observation: &Observation, goal: &str) -> AutonomousResult<Action> {
        #[derive(serde::Serialize)]
        struct PlanRequest {
            model: String,
            messages: Vec<PlanMessage>,
            max_tokens: u32,
            temperature: f32,
        }
        
        #[derive(serde::Serialize, serde::Deserialize)]
        struct PlanMessage {
            role: String,
            content: String,
        }
        
        #[derive(serde::Deserialize)]
        struct PlanResponse {
            choices: Vec<PlanChoice>,
        }
        
        #[derive(serde::Deserialize)]
        struct PlanChoice {
            message: PlanMessage,
        }
        
        // Build context from observation
        let context = format!(
            "Current screen state:\n\
            - Interactive elements: {}\n\
            - Visible text: {:?}\n\n\
            Goal: {}\n\n\
            Decide the next action. Respond with one of:\n\
            - CLICK x y\n\
            - TYPE text\n\
            - SCROLL direction\n\
            - WAIT\n\
            - DONE",
            observation.elements.iter().filter(|e| e.is_interactive).count(),
            observation.text_content.chars().take(200).collect::<String>(),
            goal
        );
        
        let request = PlanRequest {
            model: "llama3".to_string(),
            messages: vec![
                PlanMessage {
                    role: "system".to_string(),
                    content: "You are a desktop automation AI. Decide the next action based on the screen state.".to_string(),
                },
                PlanMessage {
                    role: "user".to_string(),
                    content: context,
                },
            ],
            max_tokens: 100,
            temperature: 0.3,
        };
        
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:11434/v1/chat/completions")
            .json(&request)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(data) = resp.json::<PlanResponse>().await {
                    if let Some(choice) = data.choices.first() {
                        return self.parse_action(&choice.message.content);
                    }
                }
            }
            _ => {}
        }
        
        Err(AutonomousError::PlanningFailed("LLM planning failed".into()))
    }
    
    /// Parse LLM response into action
    fn parse_action(&self, response: &str) -> AutonomousResult<Action> {
        let parts: Vec<&str> = response.trim().split_whitespace().collect();
        
        match parts.first() {
            Some(&"CLICK") if parts.len() >= 3 => {
                let x = parts[1].parse().unwrap_or(0);
                let y = parts[2].parse().unwrap_or(0);
                Ok(Action::MouseClick {
                    button: crate::MouseButton::Left,
                    x,
                    y,
                })
            }
            Some(&"TYPE") if parts.len() >= 2 => {
                let text = parts[1..].join(" ");
                Ok(Action::TypeText { text, human_like: true })
            }
            Some(&"SCROLL") if parts.len() >= 2 => {
                let (dx, dy) = match parts[1] {
                    "up" => (0, -100),
                    "down" => (0, 100),
                    "left" => (-100, 0),
                    "right" => (100, 0),
                    _ => (0, 100),
                };
                Ok(Action::MouseScroll { delta_x: dx, delta_y: dy })
            }
            Some(&"WAIT") => Ok(Action::NoOp), // Wait = NoOp, agent handles timing
            Some(&"DONE") => Ok(Action::Stop { reason: "Task completed".into() }),
            _ => Err(AutonomousError::PlanningFailed(format!("Invalid action: {}", response))),
        }
    }
    fn decompose_goal(&self, goal: &str, _observation: &Observation) -> Vec<Task> {
        let goal_lower = goal.to_lowercase();
        
        // Basit anahtar kelime bazlı ayrıştırma
        if goal_lower.contains("login") || goal_lower.contains("giriş") {
            if let Some(template) = self.task_templates.get("login") {
                return vec![template.clone()];
            }
        }
        
        if goal_lower.contains("email") || goal_lower.contains("mail") {
            if let Some(template) = self.task_templates.get("send_email") {
                return vec![template.clone()];
            }
        }
        
        // Varsayılan: Tek görev olarak ele al
        vec![Task::new(
            "Main Task",
            vec![
                TaskStep::Custom { 
                    name: "execute_goal".into(),
                    params: vec![("goal".into(), serde_json::json!(goal))].into_iter().collect(),
                },
            ],
        )]
    }
    
    /// AI ile planlama
    async fn ai_plan(&self, observation: &Observation, goal: &str) -> AutonomousResult<Action> {
        // V-GATE ile Gemma 4 kullanarak aksiyon kararı
        log::debug!("📋 PLANNER: AI planning for: {}", goal);
        
        // Try LLM-based planning first
        if let Ok(action) = self.llm_plan(observation, goal).await {
            return Ok(action);
        }
        
        // Fallback: Basit strateji - Aktif elementleri tara ve uygun aksiyon üret
        for element in &observation.elements {
            if element.is_interactive {
                // İlk interaktif elemente tıkla (placeholder logic)
                return Ok(Action::MouseClick {
                    button: crate::MouseButton::Left,
                    x: element.x + element.width as i32 / 2,
                    y: element.y + element.height as i32 / 2,
                });
            }
        }
        
        // Element bulunamadı, NoOp dön
        Ok(Action::NoOp)
    }
    
    /// Adımı aksiyona çevir
    fn step_to_action(
        &self, 
        step: &TaskStep, 
        observation: &Observation,
        variables: &HashMap<String, serde_json::Value>,
    ) -> AutonomousResult<Action> {
        match step {
            TaskStep::Click { target, button, .. } => {
                let (x, y) = self.resolve_target_position(target, variables, &observation.elements, &observation.text_content)?;
                Ok(Action::MouseClick { button: *button, x, y })
            }
            
            TaskStep::Type { text, human_like, .. } => {
                let resolved_text = self.resolve_variables(text, variables);
                Ok(Action::TypeText { 
                    text: resolved_text, 
                    human_like: *human_like 
                })
            }
            
            TaskStep::KeyPress { key } => {
                Ok(Action::KeyPress { key: *key })
            }
            
            TaskStep::Shortcut { modifiers, key } => {
                Ok(Action::KeyShortcut { 
                    modifiers: modifiers.clone(), 
                    key: *key 
                })
            }
            
            TaskStep::Scroll { direction, amount } => {
                let (dx, dy) = match direction {
                    ScrollDirection::Up => (0, -(*amount as i32)),
                    ScrollDirection::Down => (0, *amount as i32),
                    ScrollDirection::Left => (-(*amount as i32), 0),
                    ScrollDirection::Right => (*amount as i32, 0),
                    ScrollDirection::ToElement { .. } => (0, 100), // Placeholder
                };
                Ok(Action::MouseScroll { delta_x: dx, delta_y: dy })
            }
            
            TaskStep::Wait { duration_ms } => {
                // Wait için NoOp dön, agent'ın kendisi handle etmesi gerek
                log::info!("📋 PLANNER: Waiting {}ms", duration_ms);
                Ok(Action::NoOp)
            }
            
            TaskStep::NoOp => Ok(Action::NoOp),
            
            TaskStep::Custom { name, params } => {
                Ok(Action::Custom { 
                    name: name.clone(), 
                    params: params.clone() 
                })
            }
            
            // Diğer adımlar için placeholder
            _ => Ok(Action::NoOp),
        }
    }
    
    /// Target pozisyonunu çöz
    fn resolve_target_position(
        &self,
        target: &Target,
        _variables: &HashMap<String, serde_json::Value>,
        elements: &[crate::vision::UIElement],
        text_content: &str,
    ) -> AutonomousResult<(i32, i32)> {
        match target {
            Target::Position { x, y } => Ok((*x, *y)),
            Target::Relative { base, offset } => {
                let (bx, by) = self.resolve_target_position(base, _variables, elements, text_content)?;
                Ok((bx + offset.0, by + offset.1))
            }
            Target::Description { description } => {
                // AI ile element bul - interaktif elementleri ara
                for element in elements {
                    if element.is_interactive && 
                       format!("{:?}", element.element_type).to_lowercase().contains(&description.to_lowercase()) {
                        return Ok((element.x + element.width as i32 / 2, element.y + element.height as i32 / 2));
                    }
                }
                log::warn!("📋 PLANNER: Description target not found: {}", description);
                Err(AutonomousError::ElementNotFound(description.clone()))
            }
            Target::Element { selector, .. } => {
                // DOM'dan element bul - selector ile eşleşen elementi bul
                for element in elements {
                    if element.id.contains(selector) ||
                       element.label.as_ref().map(|l| l.contains(selector)).unwrap_or(false) ||
                       element.text.as_ref().map(|t| t.contains(selector)).unwrap_or(false) {
                        return Ok((element.x + element.width as i32 / 2, element.y + element.height as i32 / 2));
                    }
                }
                log::warn!("📋 PLANNER: Element target not found: {}", selector);
                Err(AutonomousError::ElementNotFound(selector.clone()))
            }
            Target::Text { text, .. } => {
                // Metin bul - elementlerin text içeriğinde ara
                for element in elements {
                    if let Some(ref elem_text) = element.text {
                        if elem_text.to_lowercase().contains(&text.to_lowercase()) {
                            return Ok((element.x + element.width as i32 / 2, element.y + element.height as i32 / 2));
                        }
                    }
                }
                // Genel metin içeriğinde de ara
                if text_content.to_lowercase().contains(&text.to_lowercase()) {
                    // Metin bulundu ama pozisyon bilinmiyor - ekran merkezini döndür
                    return Ok((640, 360)); // Placeholder
                }
                log::warn!("📋 PLANNER: Text target not found: {}", text);
                Err(AutonomousError::ElementNotFound(text.clone()))
            }
            Target::Image { template, .. } => {
                // Template matching - OpenCV gerektirir
                log::warn!("📋 PLANNER: Image template matching requires OpenCV: {}", template);
                Err(AutonomousError::ElementNotFound(template.clone()))
            }
            Target::LastKnown { id } => {
                // Cache'den son bilinen pozisyonu al
                for element in elements {
                    if element.id == *id {
                        return Ok((element.x + element.width as i32 / 2, element.y + element.height as i32 / 2));
                    }
                }
                Err(AutonomousError::ElementNotFound(id.clone()))
            }
        }
    }
    
    /// Değişkenleri çöz
    fn resolve_variables(&self, text: &str, variables: &HashMap<String, serde_json::Value>) -> String {
        let mut result = text.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("${{{}}}", key);
            if result.contains(&placeholder) {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string(),
                };
                result = result.replace(&placeholder, &value_str);
            }
        }
        
        result
    }
    
    /// Adımı tamamlandı olarak işaretle
    pub fn advance_step(&mut self) {
        if let Some(plan) = self.current_plan.as_mut() {
            plan.current_step_index += 1;
            
            // Görev tamamlandı mı kontrol et
            if let Some(task) = plan.tasks.get(plan.current_task_index) {
                if plan.current_step_index >= task.steps.len() {
                    plan.current_task_index += 1;
                    plan.current_step_index = 0;
                    
                    if plan.current_task_index >= plan.tasks.len() {
                        plan.status = PlanStatus::Completed;
                    }
                }
            }
        }
    }
    
    /// Değişken ayarla
    pub fn set_variable(&mut self, name: &str, value: serde_json::Value) {
        if let Some(plan) = self.current_plan.as_mut() {
            plan.variables.insert(name.into(), value);
        }
    }
    
    /// Sonuç kaydet
    pub fn record_result(&mut self, key: &str, value: serde_json::Value) {
        if let Some(plan) = self.current_plan.as_mut() {
            plan.results.insert(key.into(), value);
        }
    }
    
    /// Mevcut planı al
    pub fn current_plan(&self) -> Option<&TaskPlan> {
        self.current_plan.as_ref()
    }
    
    /// Planı sıfırla
    pub fn reset(&mut self) {
        self.current_plan = None;
    }
    
    /// Görev şablonu kaydet
    pub fn register_template(&mut self, name: &str, task: Task) {
        self.task_templates.insert(name.into(), task);
        log::info!("📋 PLANNER: Template registered: {}", name);
    }
    
    /// Şablondan görev oluştur
    pub fn from_template(&self, name: &str, params: HashMap<String, serde_json::Value>) -> Option<Task> {
        if let Some(template) = self.task_templates.get(name) {
            let mut task = template.clone();
            // Parametreleri uygula
            for (key, value) in params {
                task.metadata.insert(key, value.to_string());
            }
            Some(task)
        } else {
            None
        }
    }
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = Task::new("Test", vec![TaskStep::NoOp]);
        assert_eq!(task.name, "Test");
        assert_eq!(task.steps.len(), 1);
    }
    
    #[test]
    fn test_task_with_priority() {
        let task = Task::new("Test", vec![TaskStep::NoOp])
            .with_priority(8);
        assert_eq!(task.priority, 8);
    }
    
    #[tokio::test]
    async fn test_planner_creation() {
        let planner = TaskPlanner::new();
        assert!(planner.current_plan.is_none());
    }
    
    #[tokio::test]
    async fn test_plan_next_no_plan() {
        let mut planner = TaskPlanner::new();
        let obs = Observation::default();
        
        let action = planner.plan_next(&obs, "test goal").await;
        assert!(action.is_ok());
    }
    
    #[test]
    fn test_resolve_variables() {
        let planner = TaskPlanner::new();
        let vars = vec![
            ("name".into(), serde_json::json!("John")),
            ("age".into(), serde_json::json!(30)),
        ].into_iter().collect();
        
        let result = planner.resolve_variables("Hello ${name}, you are ${age}", &vars);
        assert_eq!(result, "Hello John, you are 30");
    }
    
    #[test]
    fn test_step_to_action_click() {
        let planner = TaskPlanner::new();
        let obs = Observation::default();
        let vars = HashMap::new();
        
        let step = TaskStep::Click {
            target: Target::Position { x: 100, y: 200 },
            double: false,
            button: crate::MouseButton::Left,
        };
        
        let action = planner.step_to_action(&step, &obs, &vars).expect("operation failed");
        
        match action {
            Action::MouseClick { x, y, .. } => {
                assert_eq!(x, 100);
                assert_eq!(y, 200);
            }
            _ => panic!("Wrong action type"),
        }
    }
    
    #[test]
    fn test_template_registration() {
        let mut planner = TaskPlanner::new();
        let task = Task::new("Custom", vec![TaskStep::NoOp]);
        
        planner.register_template("custom", task);
        
        assert!(planner.task_templates.contains_key("custom"));
    }
}
