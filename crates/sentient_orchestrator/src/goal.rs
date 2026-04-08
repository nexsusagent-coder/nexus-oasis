//! ─── GOL VE GÖREV TANIMLARI ───
//!
//! Kullanıcı hedeflerini ve alt görevleri temsil eden veri yapıları.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ─── GOL (HEDEF) ───
/// 
/// Kullanıcının verdiği ana hedef. Bu hedef otomatik olarak
/// alt görevlere bölünür.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    /// Benzersiz tanımlayıcı
    pub id: Uuid,
    /// Hedef açıklaması (kullanıcı girdisi)
    pub description: String,
    /// Başarı kriterleri
    pub success_criteria: Vec<String>,
    /// Bağlam/Kısıtlar
    pub constraints: Vec<String>,
    /// Öncelik
    pub priority: TaskPriority,
    /// Üst görev (varsa)
    pub parent_id: Option<Uuid>,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Goal {
    /// Yeni hedef oluştur
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            success_criteria: Vec::new(),
            constraints: Vec::new(),
            priority: TaskPriority::Normal,
            parent_id: None,
            created_at: chrono::Utc::now(),
        }
    }
    
    /// Başarı kriteri ekle
    pub fn with_success_criteria(mut self, criteria: Vec<String>) -> Self {
        self.success_criteria = criteria;
        self
    }
    
    /// Kısıt ekle
    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints = constraints;
        self
    }
    
    /// Öncelik ayarla
    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }
    
    /// Araştırma hedefi oluştur
    pub fn research(topic: impl Into<String>) -> Self {
        let topic = topic.into();
        Self::new(format!("{} hakkında araştırma yap ve raporla", topic))
            .with_success_criteria(vec![
                format!("{} hakkında güvenilir kaynaklardan bilgi bul", topic),
                "Bulguları özetle".into(),
                "Kaynakları listele".into(),
            ])
    }
    
    /// Kod yazma hedefi oluştur
    pub fn code(specification: impl Into<String>) -> Self {
        let spec = specification.into();
        Self::new(format!("Şu spesifikasyona göre kod yaz: {}", spec))
            .with_success_criteria(vec![
                "Kod çalışmalı".into(),
                "Kod okunabilir olmalı".into(),
                "Hata durumları ele alınmalı".into(),
            ])
            .with_constraints(vec![
                "Güvenlik açıklarından kaçın".into(),
                "Performans optimizasyonu yap".into(),
            ])
    }
    
    /// Web görevi oluştur
    pub fn web_action(description: impl Into<String>, url: Option<String>) -> Self {
        let desc = description.into();
        let mut goal = Self::new(desc);
        if let Some(u) = url {
            goal.constraints.push(format!("Hedef URL: {}", u));
        }
        goal
    }
}

/// ─── TASK (ALT GÖREV) ───
/// 
/// Ana hedefin küçük, yönetilebilir parçası.
/// Her görev bir araç (tool) kullanılarak gerçekleştirilir.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Benzersiz tanımlayıcı
    pub id: Uuid,
    /// Görev açıklaması
    pub description: String,
    /// Kullanılacak araç
    pub tool: ToolType,
    /// Araç girdileri
    pub input: serde_json::Value,
    /// Durum
    pub status: TaskStatus,
    /// Sonuç (tamamlandıktan sonra)
    pub result: Option<TaskResult>,
    /// Bağımlı görevler (önce bunlar tamamlanmalı)
    pub dependencies: Vec<Uuid>,
    /// Yeniden deneme sayısı
    pub retry_count: u32,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Tamamlanma zamanı
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Task {
    /// Yeni görev oluştur
    pub fn new(description: impl Into<String>, tool: ToolType) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            tool,
            input: serde_json::Value::Null,
            status: TaskStatus::Pending,
            result: None,
            dependencies: Vec::new(),
            retry_count: 0,
            created_at: chrono::Utc::now(),
            completed_at: None,
        }
    }
    
    /// Girdi ayarla
    pub fn with_input(mut self, input: serde_json::Value) -> Self {
        self.input = input;
        self
    }
    
    /// Bağımlılık ekle
    pub fn depends_on(mut self, task_id: Uuid) -> Self {
        self.dependencies.push(task_id);
        self
    }
    
    /// Görevi başlat
    pub fn start(&mut self) {
        self.status = TaskStatus::Running;
    }
    
    /// Görevi tamamla
    pub fn complete(&mut self, result: TaskResult) {
        self.status = TaskStatus::Completed;
        self.result = Some(result);
        self.completed_at = Some(chrono::Utc::now());
    }
    
    /// Görevi başarısız
    pub fn fail(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.result = Some(TaskResult::Error(error));
        self.completed_at = Some(chrono::Utc::now());
    }
    
    /// Süre (tamamlandıysa)
    pub fn duration(&self) -> Option<chrono::Duration> {
        self.completed_at.map(|end| {
            end.signed_duration_since(self.created_at)
        })
    }
}

/// ─── TASK RESULT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResult {
    /// Başarılı sonuç
    Success(serde_json::Value),
    /// Hata
    Error(String),
    /// Kısmi başarı
    Partial {
        result: serde_json::Value,
        warnings: Vec<String>,
    },
}

/// ─── TASK STATUS ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Beklemede
    Pending,
    /// Çalışıyor
    Running,
    /// Tamamlandı
    Completed,
    /// Başarısız
    Failed,
    /// Atlandı (bağımlılık hatası)
    Skipped,
}

/// ─── TASK PRIORITY ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// ─── TOOL TYPE ───
/// 
/// SENTIENT'nın kullanabileceği araçlar.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolType {
    // LLM araçları
    LlmQuery,
    LlmReason,
    
    // Browser araçları
    BrowserNavigate,
    BrowserClick,
    BrowserType,
    BrowserScroll,
    BrowserExtract,
    BrowserSearch,
    BrowserScreenshot,
    BrowserWait,
    
    // Sandbox araçları
    SandboxExecute,
    SandboxInstall,
    SandboxTest,
    
    // Bellek araçları
    MemoryStore,
    MemoryRecall,
    MemorySearch,
    
    // Sistem araçları
    WebSearch,
    Calculator,
    FileRead,
    FileWrite,
}

impl std::fmt::Display for ToolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LlmQuery => write!(f, "llm_query"),
            Self::LlmReason => write!(f, "llm_reason"),
            Self::BrowserNavigate => write!(f, "browser_navigate"),
            Self::BrowserClick => write!(f, "browser_click"),
            Self::BrowserType => write!(f, "browser_type"),
            Self::BrowserScroll => write!(f, "browser_scroll"),
            Self::BrowserExtract => write!(f, "browser_extract"),
            Self::BrowserSearch => write!(f, "browser_search"),
            Self::BrowserScreenshot => write!(f, "browser_screenshot"),
            Self::BrowserWait => write!(f, "browser_wait"),
            Self::SandboxExecute => write!(f, "sandbox_execute"),
            Self::SandboxInstall => write!(f, "sandbox_install"),
            Self::SandboxTest => write!(f, "sandbox_test"),
            Self::MemoryStore => write!(f, "memory_store"),
            Self::MemoryRecall => write!(f, "memory_recall"),
            Self::MemorySearch => write!(f, "memory_search"),
            Self::WebSearch => write!(f, "web_search"),
            Self::Calculator => write!(f, "calculator"),
            Self::FileRead => write!(f, "file_read"),
            Self::FileWrite => write!(f, "file_write"),
        }
    }
}

impl ToolType {
    /// Araç açıklaması
    pub fn description(&self) -> &'static str {
        match self {
            Self::LlmQuery => "LLM'e soru sor ve yanıt al",
            Self::LlmReason => "Karmaşık bir problemi mantıksal olarak çözümle",
            
            Self::BrowserNavigate => "Tarayıcıda bir URL'ye git",
            Self::BrowserClick => "Sayfadaki bir elemente tıkla",
            Self::BrowserType => "Metin kutusuna yazı yaz",
            Self::BrowserScroll => "Sayfayı kaydır",
            Self::BrowserExtract => "Sayfadan veri çıkar",
            Self::BrowserSearch => "Web'de arama yap",
            Self::BrowserScreenshot => "Ekran görüntüsü al",
            Self::BrowserWait => "Bir elementi veya koşulu bekle",
            
            Self::SandboxExecute => "İzole ortamda kod çalıştır",
            Self::SandboxInstall => "Paket yükle",
            Self::SandboxTest => "Kod test et",
            
            Self::MemoryStore => "Bilgiyi belleğe kaydet",
            Self::MemoryRecall => "Bellekten bilgi hatırla",
            Self::MemorySearch => "Bellekte ara",
            
            Self::WebSearch => "Web'de arama yap (API)",
            Self::Calculator => "Matematiksel hesaplama yap",
            Self::FileRead => "Dosya oku",
            Self::FileWrite => "Dosya yaz",
        }
    }
    
    /// Kategori
    pub fn category(&self) -> ToolCategory {
        match self {
            Self::LlmQuery | Self::LlmReason => ToolCategory::Llm,
            Self::BrowserNavigate | Self::BrowserClick | Self::BrowserType | 
            Self::BrowserScroll | Self::BrowserExtract | Self::BrowserSearch |
            Self::BrowserScreenshot | Self::BrowserWait => ToolCategory::Browser,
            Self::SandboxExecute | Self::SandboxInstall | Self::SandboxTest => ToolCategory::Sandbox,
            Self::MemoryStore | Self::MemoryRecall | Self::MemorySearch => ToolCategory::Memory,
            Self::WebSearch | Self::Calculator | Self::FileRead | Self::FileWrite => ToolCategory::System,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCategory {
    Llm,
    Browser,
    Sandbox,
    Memory,
    System,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("Test hedefi");
        assert!(goal.id != Uuid::nil());
        assert!(!goal.description.is_empty());
    }
    
    #[test]
    fn test_goal_research() {
        let goal = Goal::research("Yapay zeka");
        assert!(goal.description.contains("araştırma"));
        assert!(!goal.success_criteria.is_empty());
    }
    
    #[test]
    fn test_task_creation() {
        let task = Task::new("Test görevi", ToolType::BrowserNavigate);
        assert!(task.id != Uuid::nil());
        assert_eq!(task.status, TaskStatus::Pending);
    }
    
    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new("Test", ToolType::Calculator);
        task.start();
        assert_eq!(task.status, TaskStatus::Running);
        
        task.complete(TaskResult::Success(serde_json::json!(42)));
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.completed_at.is_some());
    }
    
    #[test]
    fn test_tool_type_descriptions() {
        assert!(!ToolType::BrowserNavigate.description().is_empty());
        assert!(!ToolType::SandboxExecute.description().is_empty());
    }
    
    #[test]
    fn test_tool_categories() {
        assert_eq!(ToolType::BrowserNavigate.category(), ToolCategory::Browser);
        assert_eq!(ToolType::SandboxExecute.category(), ToolCategory::Sandbox);
        assert_eq!(ToolType::LlmQuery.category(), ToolCategory::Llm);
    }
}
