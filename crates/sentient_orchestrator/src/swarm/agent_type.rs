//! ─── AJAN TİPLERİ VE PERSONALAR ───
//!
//! Swarm içindeki uzmanlaşmış ajan tipleri - her biri belirli yeteneklere sahip.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// ─── AGENT TYPE ───
/// 
/// Swarm içindeki ajan tipleri - her biri farklı uzmanlığa sahip.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentType {
    /// Koordinatör - Görev dağıtımı ve yönetim
    Coordinator,
    
    /// Araştırmacı - Web araştırması, bilgi toplama
    Researcher,
    
    /// Yazılımcı - Kod yazma, debug etme
    Coder,
    
    /// Eleştirmen - Sonuç değerlendirme, kalite kontrol
    Critic,
    
    /// Planlayıcı - Görev bölme, strateji oluşturma
    Planner,
    
    /// Yürütücü - Genel görev yürütme
    Executor,
    
    /// Bellek Uzmanı - Bilgi saklama ve erişim
    MemoryKeeper,
    
    /// Web Uzmanı - Tarayıcı işlemleri
    WebSurfer,
}

impl AgentType {
    /// Kısa kod
    pub fn short_code(&self) -> &'static str {
        match self {
            Self::Coordinator => "coord",
            Self::Researcher => "res",
            Self::Coder => "code",
            Self::Critic => "crit",
            Self::Planner => "plan",
            Self::Executor => "exec",
            Self::MemoryKeeper => "mem",
            Self::WebSurfer => "web",
        }
    }
    
    /// Gösterge emoji
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Coordinator => "🎯",
            Self::Researcher => "🔍",
            Self::Coder => "💻",
            Self::Critic => "🧐",
            Self::Planner => "📋",
            Self::Executor => "⚡",
            Self::MemoryKeeper => "💾",
            Self::WebSurfer => "🌐",
        }
    }
    
    /// Açıklama
    pub fn description(&self) -> &'static str {
        match self {
            Self::Coordinator => "Swarm koordinasyonu ve görev dağıtımı",
            Self::Researcher => "Bilgi araştırması ve toplanması",
            Self::Coder => "Kod yazma ve hata ayıklama",
            Self::Critic => "Sonuç değerlendirme ve kalite kontrol",
            Self::Planner => "Strateji planlama ve görev bölme",
            Self::Executor => "Görev yürütme ve araç kullanma",
            Self::MemoryKeeper => "Bilgi saklama ve erişim yönetimi",
            Self::WebSurfer => "Web sayfası gezintisi ve veri çıkarma",
        }
    }
    
    /// Varsayılan yetenekler
    pub fn default_capabilities(&self) -> Vec<AgentCapability> {
        match self {
            Self::Coordinator => vec![
                AgentCapability::TaskRouting,
                AgentCapability::Orchestration,
                AgentCapability::DecisionMaking,
            ],
            Self::Researcher => vec![
                AgentCapability::WebSearch,
                AgentCapability::InformationSynthesis,
                AgentCapability::SourceVerification,
            ],
            Self::Coder => vec![
                AgentCapability::CodeGeneration,
                AgentCapability::CodeReview,
                AgentCapability::Testing,
                AgentCapability::Debugging,
            ],
            Self::Critic => vec![
                AgentCapability::Evaluation,
                AgentCapability::QualityAssurance,
                AgentCapability::ErrorDetection,
            ],
            Self::Planner => vec![
                AgentCapability::Planning,
                AgentCapability::TaskDecomposition,
                AgentCapability::Strategy,
            ],
            Self::Executor => vec![
                AgentCapability::Execution,
                AgentCapability::ToolUsage,
                AgentCapability::ProblemSolving,
            ],
            Self::MemoryKeeper => vec![
                AgentCapability::MemoryStorage,
                AgentCapability::MemoryRetrieval,
                AgentCapability::KnowledgeManagement,
            ],
            Self::WebSurfer => vec![
                AgentCapability::WebBrowsing,
                AgentCapability::DataExtraction,
                AgentCapability::FormInteraction,
            ],
        }
    }
    
    /// Sistem promptu
    pub fn system_prompt(&self) -> &'static str {
        match self {
            Self::Coordinator => include_str!("prompts/coordinator.txt"),
            Self::Researcher => include_str!("prompts/researcher.txt"),
            Self::Coder => include_str!("prompts/coder.txt"),
            Self::Critic => include_str!("prompts/critic.txt"),
            Self::Planner => include_str!("prompts/planner.txt"),
            Self::Executor => include_str!("prompts/executor.txt"),
            Self::MemoryKeeper => include_str!("prompts/memory_keeper.txt"),
            Self::WebSurfer => include_str!("prompts/web_surfer.txt"),
        }
    }
    
    /// Bu ajan tipi verilen görevi yapabilir mi?
    pub fn can_handle(&self, capabilities: &[AgentCapability]) -> bool {
        let my_caps: HashSet<_> = self.default_capabilities().into_iter().collect();
        capabilities.iter().any(|c| my_caps.contains(c))
    }
}

/// ─── AGENT PERSONA ───
/// 
/// Bir ajanın kişiliği ve uzmanlık alanları.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersona {
    /// Ajan tipi
    pub agent_type: AgentType,
    
    /// Kişiselleştirilmiş isim (opsiyonel)
    pub name: Option<String>,
    
    /// Uzmanlık alanları
    pub specializations: Vec<String>,
    
    /// Ek yetenekler
    pub extra_capabilities: Vec<AgentCapability>,
    
    /// Öncelik ağırlığı (yüksek = daha sık görev alır)
    pub priority_weight: f32,
    
    /// Maksimum eşzamanlı görev
    pub max_concurrent_tasks: usize,
}

impl AgentPersona {
    pub fn new(agent_type: AgentType) -> Self {
        Self {
            agent_type,
            name: None,
            specializations: Vec::new(),
            extra_capabilities: Vec::new(),
            priority_weight: 1.0,
            max_concurrent_tasks: 3,
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn specialize(mut self, area: impl Into<String>) -> Self {
        self.specializations.push(area.into());
        self
    }
    
    pub fn add_capability(mut self, capability: AgentCapability) -> Self {
        self.extra_capabilities.push(capability);
        self
    }
    
    pub fn with_priority(mut self, weight: f32) -> Self {
        self.priority_weight = weight;
        self
    }
    
    pub fn with_max_tasks(mut self, max: usize) -> Self {
        self.max_concurrent_tasks = max;
        self
    }
    
    /// Tüm yetenekler
    pub fn all_capabilities(&self) -> Vec<AgentCapability> {
        let mut caps = self.agent_type.default_capabilities();
        caps.extend(self.extra_capabilities.clone());
        caps
    }
    
    /// Görünür isim
    pub fn display_name(&self) -> String {
        match &self.name {
            Some(n) => format!("{} {}", self.agent_type.emoji(), n),
            None => format!("{} {:?}", self.agent_type.emoji(), self.agent_type),
        }
    }
}

/// ─── AGENT CAPABILITY ───
/// 
/// Bir ajanın sahip olabileceği yetenekler.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgentCapability {
    // Görev yönetimi
    TaskRouting,
    Orchestration,
    DecisionMaking,
    Planning,
    TaskDecomposition,
    Strategy,
    
    // Bilgi işleme
    WebSearch,
    InformationSynthesis,
    SourceVerification,
    KnowledgeManagement,
    
    // Kod işlemleri
    CodeGeneration,
    CodeReview,
    Testing,
    Debugging,
    
    // Değerlendirme
    Evaluation,
    QualityAssurance,
    ErrorDetection,
    
    // Yürütme
    Execution,
    ToolUsage,
    ProblemSolving,
    
    // Bellek
    MemoryStorage,
    MemoryRetrieval,
    
    // Web
    WebBrowsing,
    DataExtraction,
    FormInteraction,
}

impl AgentCapability {
    /// Kategori
    pub fn category(&self) -> CapabilityCategory {
        match self {
            // Görev yönetimi
            Self::TaskRouting | Self::Orchestration | Self::DecisionMaking |
            Self::Planning | Self::TaskDecomposition | Self::Strategy => {
                CapabilityCategory::Management
            }
            
            // Bilgi işleme
            Self::WebSearch | Self::InformationSynthesis | Self::SourceVerification |
            Self::KnowledgeManagement => {
                CapabilityCategory::Information
            }
            
            // Kod işlemleri
            Self::CodeGeneration | Self::CodeReview | Self::Testing | Self::Debugging => {
                CapabilityCategory::Code
            }
            
            // Değerlendirme
            Self::Evaluation | Self::QualityAssurance | Self::ErrorDetection => {
                CapabilityCategory::Evaluation
            }
            
            // Yürütme
            Self::Execution | Self::ToolUsage | Self::ProblemSolving => {
                CapabilityCategory::Execution
            }
            
            // Bellek
            Self::MemoryStorage | Self::MemoryRetrieval => {
                CapabilityCategory::Memory
            }
            
            // Web
            Self::WebBrowsing | Self::DataExtraction | Self::FormInteraction => {
                CapabilityCategory::Web
            }
        }
    }
    
    /// Açıklama
    pub fn description(&self) -> &'static str {
        match self {
            Self::TaskRouting => "Görevleri uygun ajanlara yönlendirme",
            Self::Orchestration => "Çoklu ajan koordinasyonu",
            Self::DecisionMaking => "Kritik kararlar alma",
            Self::Planning => "Eylem planları oluşturma",
            Self::TaskDecomposition => "Karmaşık görevleri alt görevlere bölme",
            Self::Strategy => "Stratejik yaklaşım geliştirme",
            
            Self::WebSearch => "Web'de arama yapma",
            Self::InformationSynthesis => "Bilgi sentezi yapma",
            Self::SourceVerification => "Kaynak doğrulama",
            Self::KnowledgeManagement => "Bilgi yönetimi",
            
            Self::CodeGeneration => "Kod üretme",
            Self::CodeReview => "Kod inceleme",
            Self::Testing => "Test yazma ve çalıştırma",
            Self::Debugging => "Hata ayıklama",
            
            Self::Evaluation => "Sonuç değerlendirme",
            Self::QualityAssurance => "Kalite güvencesi",
            Self::ErrorDetection => "Hata tespiti",
            
            Self::Execution => "Görev yürütme",
            Self::ToolUsage => "Araç kullanma",
            Self::ProblemSolving => "Problem çözme",
            
            Self::MemoryStorage => "Bilgi saklama",
            Self::MemoryRetrieval => "Bilgi erişimi",
            
            Self::WebBrowsing => "Web sayfası gezintisi",
            Self::DataExtraction => "Veri çıkarma",
            Self::FormInteraction => "Form doldurma",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityCategory {
    Management,
    Information,
    Code,
    Evaluation,
    Execution,
    Memory,
    Web,
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_code())
    }
}

impl Default for AgentType {
    fn default() -> Self {
        Self::Executor
    }
}

impl std::fmt::Display for AgentCapability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_type_short_codes() {
        assert_eq!(AgentType::Coordinator.short_code(), "coord");
        assert_eq!(AgentType::Researcher.short_code(), "res");
        assert_eq!(AgentType::Coder.short_code(), "code");
    }
    
    #[test]
    fn test_agent_type_capabilities() {
        let research_caps = AgentType::Researcher.default_capabilities();
        assert!(research_caps.contains(&AgentCapability::WebSearch));
        assert!(!research_caps.contains(&AgentCapability::CodeGeneration));
    }
    
    #[test]
    fn test_agent_persona_creation() {
        let persona = AgentPersona::new(AgentType::Researcher)
            .with_name("Araştırmacı Ali")
            .specialize("Yapay Zeka")
            .with_priority(1.5);
        
        assert!(persona.name.is_some());
        assert!(!persona.specializations.is_empty());
        assert_eq!(persona.priority_weight, 1.5);
    }
    
    #[test]
    fn test_agent_type_can_handle() {
        assert!(AgentType::Researcher.can_handle(&[AgentCapability::WebSearch]));
        assert!(AgentType::Coder.can_handle(&[AgentCapability::CodeGeneration]));
        assert!(!AgentType::Coder.can_handle(&[AgentCapability::WebSearch]));
    }
    
    #[test]
    fn test_capability_categories() {
        assert_eq!(AgentCapability::CodeGeneration.category(), CapabilityCategory::Code);
        assert_eq!(AgentCapability::WebSearch.category(), CapabilityCategory::Information);
        assert_eq!(AgentCapability::Planning.category(), CapabilityCategory::Management);
    }
    
    #[test]
    fn test_persona_display_name() {
        let named = AgentPersona::new(AgentType::Coder).with_name("Python Uzmanı");
        assert!(named.display_name().contains("Python"));
        
        let unnamed = AgentPersona::new(AgentType::Coder);
        assert!(unnamed.display_name().contains("Coder"));
    }
}
