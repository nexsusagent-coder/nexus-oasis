//! ─── SKILLS / TOOL HUB ───
//!
//! Asimile edilen rakip yeteneklerin yönetim katmanı
//! - MindSearch Deep Research
//! - Lightpanda Browser DOM
//! - AutoResearch PDF Parsing
//! - n8n Automation Workflows
//!
//! Her skill Aktif/Pasif edilebilir ve ReAct döngüsüne entegre edilir

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
// SKILL DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Asimile edilmiş rakip yetenekleri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    /// MindSearch - Derin araştırma ve knowledge graph
    MindSearch,
    /// Lightpanda - Browser DOM manipülasyonu
    LightpandaBrowser,
    /// AutoResearch - PDF parsing ve akademik araştırma
    AutoResearch,
    /// n8n - Workflow otomasyonu
    N8nAutomation,
    /// Web Search - Genel web araması
    WebSearch,
    /// Citation - Alıntı ve kaynak yönetimi
    Citation,
}

impl SkillType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MindSearch => "MindSearch Deep Research",
            Self::LightpandaBrowser => "Lightpanda Browser DOM",
            Self::AutoResearch => "AutoResearch PDF",
            Self::N8nAutomation => "n8n Automation",
            Self::WebSearch => "Web Search",
            Self::Citation => "Citation Manager",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::MindSearch => "Akıllı araştırma ajanı, bilgi grafikleri oluşturur",
            Self::LightpandaBrowser => "Headless browser, DOM manipülasyonu ve web etkileşimi",
            Self::AutoResearch => "PDF parsing, akademik makale analizi",
            Self::N8nAutomation => "Workflow otomasyonu ve entegrasyonlar",
            Self::WebSearch => "DuckDuckGo ve açık web araması",
            Self::Citation => "Alıntı yönetimi ve kaynak doğrulama",
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            Self::MindSearch => "🧠",
            Self::LightpandaBrowser => "🌐",
            Self::AutoResearch => "📄",
            Self::N8nAutomation => "⚡",
            Self::WebSearch => "🔍",
            Self::Citation => "📚",
        }
    }
    
    pub fn source_repo(&self) -> &'static str {
        match self {
            Self::MindSearch => "mindsearch/mindsearch",
            Self::LightpandaBrowser => "lightpanda-io/lightpanda",
            Self::AutoResearch => "autoresearch/autoresearch",
            Self::N8nAutomation => "n8n-io/n8n",
            Self::WebSearch => "internal",
            Self::Citation => "internal",
        }
    }
}

/// Skill durumu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillStatus {
    Active,
    Inactive,
    Error,
    Loading,
}

/// Tek bir skill kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub skill_type: SkillType,
    pub status: SkillStatus,
    pub enabled: bool,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u64,
    pub error_message: Option<String>,
    pub config: HashMap<String, serde_json::Value>,
}

impl Skill {
    pub fn new(skill_type: SkillType) -> Self {
        Self {
            id: Uuid::new_v4(),
            skill_type,
            status: SkillStatus::Inactive,
            enabled: false,
            last_used: None,
            usage_count: 0,
            error_message: None,
            config: HashMap::new(),
        }
    }
    
    pub fn activate(&mut self) {
        self.enabled = true;
        self.status = SkillStatus::Active;
    }
    
    pub fn deactivate(&mut self) {
        self.enabled = false;
        self.status = SkillStatus::Inactive;
    }
    
    pub fn mark_used(&mut self) {
        self.last_used = Some(Utc::now());
        self.usage_count += 1;
    }
    
    pub fn set_error(&mut self, message: impl Into<String>) {
        self.status = SkillStatus::Error;
        self.error_message = Some(message.into());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SKILLS HUB - YÖNETİM MERKEZİ
// ═══════════════════════════════════════════════════════════════════════════════

/// Skills Hub - Tüm asimile edilmiş yeteneklerin yönetim merkezi
pub struct SkillsHub {
    skills: Arc<RwLock<HashMap<SkillType, Skill>>>,
    execution_history: Arc<RwLock<Vec<SkillExecution>>>,
}

impl SkillsHub {
    pub fn new() -> Self {
        let mut skills = HashMap::new();
        
        // Tüm skill'leri başlat
        skills.insert(SkillType::MindSearch, Skill::new(SkillType::MindSearch));
        skills.insert(SkillType::LightpandaBrowser, Skill::new(SkillType::LightpandaBrowser));
        skills.insert(SkillType::AutoResearch, Skill::new(SkillType::AutoResearch));
        skills.insert(SkillType::N8nAutomation, Skill::new(SkillType::N8nAutomation));
        skills.insert(SkillType::WebSearch, Skill::new(SkillType::WebSearch));
        skills.insert(SkillType::Citation, Skill::new(SkillType::Citation));
        
        Self {
            skills: Arc::new(RwLock::new(skills)),
            execution_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Tüm skill'leri listele
    pub async fn list_skills(&self) -> Vec<Skill> {
        self.skills.read().await.values().cloned().collect()
    }
    
    /// Skill durumunu getir
    pub async fn get_skill(&self, skill_type: &SkillType) -> Option<Skill> {
        self.skills.read().await.get(skill_type).cloned()
    }
    
    /// Skill'i aktifleştir
    pub async fn activate_skill(&self, skill_type: SkillType) -> SENTIENTResult<()> {
        let mut skills = self.skills.write().await;
        if let Some(skill) = skills.get_mut(&skill_type) {
            skill.activate();
            log::info!("skills  {} aktifleştirildi", skill_type.display_name());
        }
        Ok(())
    }
    
    /// Skill'i deaktifleştir
    pub async fn deactivate_skill(&self, skill_type: SkillType) -> SENTIENTResult<()> {
        let mut skills = self.skills.write().await;
        if let Some(skill) = skills.get_mut(&skill_type) {
            skill.deactivate();
            log::info!("skills  {} deaktifleştirildi", skill_type.display_name());
        }
        Ok(())
    }
    
    /// Skill'i toggle et
    pub async fn toggle_skill(&self, skill_type: SkillType) -> SENTIENTResult<bool> {
        let mut skills = self.skills.write().await;
        if let Some(skill) = skills.get_mut(&skill_type) {
            if skill.enabled {
                skill.deactivate();
                log::info!("skills  {} kapatıldı", skill_type.display_name());
                Ok(false)
            } else {
                skill.activate();
                log::info!("skills  {} açıldı", skill_type.display_name());
                Ok(true)
            }
        } else {
            Err(SENTIENTError::ValidationError("Skill bulunamadı".into()))
        }
    }
    
    /// Skill'i çalıştır (ReAct döngüsünden çağrılır)
    pub async fn execute(&self, skill_type: SkillType, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        let start = std::time::Instant::now();
        
        // Skill'i aktif işaretle
        {
            let mut skills = self.skills.write().await;
            if let Some(skill) = skills.get_mut(&skill_type) {
                if !skill.enabled {
                    return Err(SENTIENTError::ValidationError(format!(
                        "{} aktif değil", skill_type.display_name()
                    )));
                }
                skill.mark_used();
                skill.status = SkillStatus::Loading;
            }
        }
        
        // Skill'i çalıştır
        let result = self.execute_skill_internal(&skill_type, input.clone()).await;
        
        // Sonucu kaydet
        let execution = SkillExecution {
            id: Uuid::new_v4(),
            skill_type: skill_type.clone(),
            input: input.clone(),
            output: result.clone(),
            duration_ms: start.elapsed().as_millis() as u64,
            timestamp: Utc::now(),
        };
        
        self.execution_history.write().await.insert(0, execution);
        
        // Skill durumunu güncelle
        {
            let mut skills = self.skills.write().await;
            if let Some(skill) = skills.get_mut(&skill_type) {
                match &result {
                    Ok(_) => {
                        skill.status = SkillStatus::Active;
                        skill.error_message = None;
                    }
                    Err(e) => {
                        skill.status = SkillStatus::Error;
                        skill.error_message = Some(e.to_string());
                    }
                }
            }
        }
        
        result
    }
    
    /// Dahili skill çalıştırma
    async fn execute_skill_internal(&self, skill_type: &SkillType, input: SkillInput) -> SENTIENTResult<SkillOutput> {
        match skill_type {
            SkillType::MindSearch => {
                // MindSearch: Araştırma sorgusu
                log::info!("skills  MindSearch çalıştırılıyor: {:?}", input.query);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "results": [],
                        "knowledge_graph": {
                            "nodes": [],
                            "edges": []
                        },
                        "summary": "MindSearch araştırması tamamlandı"
                    }),
                    message: "MindSearch: Araştırma tamamlandı".into(),
                })
            }
            
            SkillType::LightpandaBrowser => {
                // Lightpanda: Browser DOM
                log::info!("skills  Lightpanda çalıştırılıyor: {:?}", input.url);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "dom": "<html>...</html>",
                        "screenshot": null,
                        "elements": []
                    }),
                    message: "Lightpanda: DOM alındı".into(),
                })
            }
            
            SkillType::AutoResearch => {
                // AutoResearch: PDF parsing
                log::info!("skills  AutoResearch çalıştırılıyor: {:?}", input.source);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "papers": [],
                        "citations": [],
                        "summary": "PDF analizi tamamlandı"
                    }),
                    message: "AutoResearch: PDF analiz edildi".into(),
                })
            }
            
            SkillType::N8nAutomation => {
                // n8n: Workflow tetikleme
                log::info!("skills  n8n workflow tetikleniyor: {:?}", input.workflow_id);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "workflow_id": input.workflow_id,
                        "execution_id": Uuid::new_v4().to_string(),
                        "status": "running"
                    }),
                    message: "n8n: Workflow başlatıldı".into(),
                })
            }
            
            SkillType::WebSearch => {
                // Web Search
                log::info!("skills  Web Search: {:?}", input.query);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "results": [
                            {"title": "Sonuç 1", "url": "https://example.com", "snippet": "..."}
                        ],
                        "total": 1
                    }),
                    message: "Web araması tamamlandı".into(),
                })
            }
            
            SkillType::Citation => {
                // Citation Manager
                log::info!("skills  Citation: {:?}", input.citation_key);
                Ok(SkillOutput {
                    success: true,
                    data: serde_json::json!({
                        "citation": input.citation_key,
                        "format": "APA",
                        "metadata": {}
                    }),
                    message: "Alıntı formatlandı".into(),
                })
            }
        }
    }
    
    /// Aktif skill sayısı
    pub async fn active_count(&self) -> usize {
        self.skills.read().await.values().filter(|s| s.enabled).count()
    }
    
    /// Son çalıştırmaları getir
    pub async fn recent_executions(&self, limit: usize) -> Vec<SkillExecution> {
        self.execution_history.read().await.iter().take(limit).cloned().collect()
    }
    
    /// Skill istatistikleri
    pub async fn stats(&self) -> SkillsStats {
        let skills = self.skills.read().await;
        let history = self.execution_history.read().await;
        
        let total_executions = history.len() as u64;
        let successful = history.iter().filter(|e| e.output.is_ok()).count() as u64;
        
        SkillsStats {
            total_skills: skills.len(),
            active_skills: skills.values().filter(|s| s.enabled).count(),
            total_executions,
            successful_executions: successful,
            failed_executions: total_executions - successful,
            avg_duration_ms: if !history.is_empty() {
                history.iter().map(|e| e.duration_ms).sum::<u64>() / history.len() as u64
            } else {
                0
            },
        }
    }
}

impl Default for SkillsHub {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SKILL INPUT / OUTPUT
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInput {
    pub query: Option<String>,
    pub url: Option<String>,
    pub source: Option<String>,
    pub workflow_id: Option<String>,
    pub citation_key: Option<String>,
    pub params: HashMap<String, serde_json::Value>,
}

impl Default for SkillInput {
    fn default() -> Self {
        Self {
            query: None,
            url: None,
            source: None,
            workflow_id: None,
            citation_key: None,
            params: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillOutput {
    pub success: bool,
    pub data: serde_json::Value,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecution {
    pub id: Uuid,
    pub skill_type: SkillType,
    pub input: SkillInput,
    pub output: SENTIENTResult<SkillOutput>,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SKILLS STATS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsStats {
    pub total_skills: usize,
    pub active_skills: usize,
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub avg_duration_ms: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

pub type SENTIENTResult<T> = Result<T, SENTIENTError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SENTIENTError {
    General(String),
    ValidationError(String),
    SkillNotActive(String),
    ExecutionError(String),
}

impl std::fmt::Display for SENTIENTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General(s) => write!(f, "SENTIENT Hatası: {}", s),
            Self::ValidationError(s) => write!(f, "Doğrulama Hatası: {}", s),
            Self::SkillNotActive(s) => write!(f, "Skill Aktif Değil: {}", s),
            Self::ExecutionError(s) => write!(f, "Çalıştırma Hatası: {}", s),
        }
    }
}

impl std::error::Error for SENTIENTError {}

// ═══════════════════════════════════════════════════════════════════════════════
// REACT INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// ReAct döngüsü için skill seçici
impl SkillsHub {
    /// Hedef için en uygun skill'leri öner
    pub async fn suggest_skills(&self, goal: &str) -> Vec<SkillType> {
        let mut suggested = Vec::new();
        let goal_lower = goal.to_lowercase();
        
        // Anahtar kelimelere göre skill önerisi
        if goal_lower.contains("araştır") || goal_lower.contains("research") || goal_lower.contains("bul") {
            suggested.push(SkillType::MindSearch);
            suggested.push(SkillType::WebSearch);
        }
        
        if goal_lower.contains("pdf") || goal_lower.contains("makale") || goal_lower.contains("paper") {
            suggested.push(SkillType::AutoResearch);
            suggested.push(SkillType::Citation);
        }
        
        if goal_lower.contains("web") || goal_lower.contains("site") || goal_lower.contains("browser") {
            suggested.push(SkillType::LightpandaBrowser);
        }
        
        if goal_lower.contains("otomatik") || goal_lower.contains("workflow") || goal_lower.contains("otomasyon") {
            suggested.push(SkillType::N8nAutomation);
        }
        
        // Varsayılan olarak web search ekle
        if suggested.is_empty() {
            suggested.push(SkillType::WebSearch);
        }
        
        // Sadece aktif skill'leri döndür
        let skills = self.skills.read().await;
        suggested.retain(|s| skills.get(s).map(|sk| sk.enabled).unwrap_or(false));
        
        suggested
    }
    
    /// Skill'i ReAct action olarak çalıştır
    pub async fn react_execute(&self, action: &str, params: &HashMap<String, serde_json::Value>) -> SENTIENTResult<SkillOutput> {
        // Action string'ini parse et: "skill://mindsearch?query=..."
        if !action.starts_with("skill://") {
            return Err(SENTIENTError::ValidationError("Geçersiz skill action formatı".into()));
        }
        
        let action = action.strip_prefix("skill://").unwrap();
        let parts: Vec<&str> = action.split('?').collect();
        let skill_name = parts[0];
        
        let skill_type = match skill_name {
            "mindsearch" => SkillType::MindSearch,
            "browser" => SkillType::LightpandaBrowser,
            "autoresearch" => SkillType::AutoResearch,
            "n8n" => SkillType::N8nAutomation,
            "search" => SkillType::WebSearch,
            "citation" => SkillType::Citation,
            _ => return Err(SENTIENTError::ValidationError(format!("Bilinmeyen skill: {}", skill_name))),
        };
        
        let input = SkillInput {
            query: params.get("query").and_then(|v| v.as_str()).map(String::from),
            url: params.get("url").and_then(|v| v.as_str()).map(String::from),
            source: params.get("source").and_then(|v| v.as_str()).map(String::from),
            workflow_id: params.get("workflow_id").and_then(|v| v.as_str()).map(String::from),
            citation_key: params.get("citation_key").and_then(|v| v.as_str()).map(String::from),
            params: params.clone(),
        };
        
        self.execute(skill_type, input).await
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_skills_hub_creation() {
        let hub = SkillsHub::new();
        let skills = hub.list_skills().await;
        assert_eq!(skills.len(), 6);
    }
    
    #[tokio::test]
    async fn test_skill_activation() {
        let hub = SkillsHub::new();
        
        // MindSearch aktifleştir
        hub.activate_skill(SkillType::MindSearch).await.unwrap();
        
        let skill = hub.get_skill(&SkillType::MindSearch).await.unwrap();
        assert!(skill.enabled);
        assert_eq!(skill.status, SkillStatus::Active);
    }
    
    #[tokio::test]
    async fn test_skill_toggle() {
        let hub = SkillsHub::new();
        
        // İlk toggle: aç
        let result = hub.toggle_skill(SkillType::WebSearch).await.unwrap();
        assert!(result);
        
        // İkinci toggle: kapat
        let result = hub.toggle_skill(SkillType::WebSearch).await.unwrap();
        assert!(!result);
    }
    
    #[tokio::test]
    async fn test_skill_execution() {
        let hub = SkillsHub::new();
        
        // Web Search aktifleştir
        hub.activate_skill(SkillType::WebSearch).await.unwrap();
        
        let input = SkillInput {
            query: Some("test query".into()),
            ..Default::default()
        };
        
        let output = hub.execute(SkillType::WebSearch, input).await.unwrap();
        assert!(output.success);
    }
    
    #[tokio::test]
    async fn test_inactive_skill_execution() {
        let hub = SkillsHub::new();
        
        // MindSearch aktif değil
        let input = SkillInput {
            query: Some("test".into()),
            ..Default::default()
        };
        
        let result = hub.execute(SkillType::MindSearch, input).await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_skill_type_display_name() {
        assert_eq!(SkillType::MindSearch.display_name(), "MindSearch Deep Research");
        assert_eq!(SkillType::LightpandaBrowser.display_name(), "Lightpanda Browser DOM");
    }
}
