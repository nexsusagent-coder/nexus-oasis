//! ═══════════════════════════════════════════════════════════════════════════════
//!  ADVANCED MEMORY - Öğrenen Bellek Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Episodic, Semantic ve Procedural memory tipleri.
//!
//! BELLEK TİPLERİ:
//! ──────────────
//! 1. EPISODIC    → Yaşanan olaylar ("Dün login oldum, şifre X idi")
//! 2. SEMANTIC    → Genel bilgiler ("Login butonu genelde sağ üstte")
//! 3. PROCEDURAL  → Nasıl yapılır ("Login olmak için: 1. Siteye git...")
//! 4. WORKING     → Aktif görev verileri

use crate::error::AutonomousResult;
use crate::{Action, ActionResult, Observation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY TYPE
// ═══════════════════════════════════════════════════════════════════════════════

/// Bellek türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    /// Epizodik - yaşanan olaylar
    Episodic,
    /// Semantik - genel bilgiler
    Semantic,
    /// Prosedürel - nasıl yapılır
    Procedural,
    /// Çalışma - aktif görev
    Working,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EPISODE
// ═══════════════════════════════════════════════════════════════════════════════

/// Epizodik bellek kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Kayıt ID
    pub id: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Görev adı
    pub task_name: String,
    /// Gözlem (özet)
    pub observation_summary: String,
    /// Aksiyon
    pub action: String,
    /// Sonuç
    pub result: String,
    /// Başarılı mı?
    pub success: bool,
    /// Süre (ms)
    pub duration_ms: u64,
    /// Bağlam
    pub context: HashMap<String, String>,
    /// Öğrenilen ders
    pub lesson: Option<String>,
}

impl Episode {
    pub fn new(task_name: &str, observation: &Observation, action: &Action, result: &ActionResult) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            task_name: task_name.into(),
            observation_summary: format!("{} elements, {} text", 
                observation.elements.len(),
                observation.text_content.len()
            ),
            action: action.describe(),
            result: result.message.clone(),
            success: result.success,
            duration_ms: 0,
            context: HashMap::new(),
            lesson: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SEMANTIC KNOWLEDGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Semantik bilgi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKnowledge {
    /// Konu
    pub subject: String,
    /// Predikat
    pub predicate: String,
    /// Nesne
    pub object: String,
    /// Güven skoru
    pub confidence: f32,
    /// Kaynak sayısı
    pub source_count: usize,
    /// Son güncelleme
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl SemanticKnowledge {
    pub fn new(subject: &str, predicate: &str, object: &str) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            confidence: 0.5,
            source_count: 1,
            last_updated: chrono::Utc::now(),
        }
    }
    
    /// Bilgiyi güçlendir
    pub fn reinforce(&mut self) {
        self.source_count += 1;
        self.confidence = (self.confidence + 0.1).min(1.0);
        self.last_updated = chrono::Utc::now();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROCEDURAL SKILL
// ═══════════════════════════════════════════════════════════════════════════════

/// Prosedürel beceri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralSkill {
    /// Beceri adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Adımlar
    pub steps: Vec<SkillStep>,
    /// Başarı oranı
    pub success_rate: f32,
    /// Kullanım sayısı
    pub usage_count: usize,
    /// Son kullanım
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

/// Beceri adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub description: String,
    pub action_pattern: String,
    pub success_condition: Option<String>,
}

impl ProceduralSkill {
    pub fn new(name: &str, description: &str, steps: Vec<SkillStep>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            steps,
            success_rate: 0.0,
            usage_count: 0,
            last_used: None,
        }
    }
    
    /// Kullanımı kaydet
    pub fn record_usage(&mut self, success: bool) {
        self.usage_count += 1;
        
        // Running average
        let n = self.usage_count as f32;
        self.success_rate = self.success_rate * (n - 1.0) / n + if success { 1.0 } else { 0.0 } / n;
        
        self.last_used = Some(chrono::Utc::now());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  WORKING MEMORY
// ═══════════════════════════════════════════════════════════════════════════════

/// Çalışma belleği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    /// Aktif görev
    pub current_task: Option<String>,
    /// Mevcut hedef
    pub current_goal: Option<String>,
    /// Değişkenler
    pub variables: HashMap<String, serde_json::Value>,
    /// Son N aksiyon
    pub recent_actions: Vec<(String, bool)>,
    /// Aktif elementler
    pub active_elements: HashMap<String, String>,
    /// Context
    pub context: HashMap<String, String>,
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self {
            current_task: None,
            current_goal: None,
            variables: HashMap::new(),
            recent_actions: Vec::with_capacity(100),
            active_elements: HashMap::new(),
            context: HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ADVANCED MEMORY ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Gelişmiş bellek motoru
pub struct AdvancedMemory {
    /// Epizodik bellek
    episodic: Vec<Episode>,
    /// Semantik bellek
    semantic: Vec<SemanticKnowledge>,
    /// Prosedürel bellek
    procedural: HashMap<String, ProceduralSkill>,
    /// Çalışma belleği
    working: WorkingMemory,
    /// Maksimum epizodik kayıt
    max_episodic: usize,
}

impl AdvancedMemory {
    pub fn new() -> Self {
        log::info!("🧠 MEMORY: Gelişmiş bellek motoru başlatılıyor...");
        
        Self {
            episodic: Vec::with_capacity(1000),
            semantic: Vec::new(),
            procedural: HashMap::new(),
            working: WorkingMemory::default(),
            max_episodic: 10000,
        }
    }
    
    /// Episode kaydet
    pub async fn record_episode(&mut self, observation: &Observation, action: &Action, result: &ActionResult) -> AutonomousResult<()> {
        let task_name = self.working.current_task.clone().unwrap_or_else(|| "unknown".into());
        
        let episode = Episode::new(&task_name, observation, action, result);
        
        // Çalışma belleğine ekle
        self.working.recent_actions.push((action.describe(), result.success));
        if self.working.recent_actions.len() > 100 {
            self.working.recent_actions.remove(0);
        }
        
        // Epizodik belleğe ekle
        self.episodic.push(episode);
        
        // Limite göre temizle
        if self.episodic.len() > self.max_episodic {
            self.episodic.drain(0..self.episodic.len() - self.max_episodic);
        }
        
        log::debug!("🧠 MEMORY: Episode recorded (total: {})", self.episodic.len());
        
        Ok(())
    }
    
    /// Semantik bilgi ekle
    pub fn add_knowledge(&mut self, subject: &str, predicate: &str, object: &str) {
        // Aynı bilgi var mı kontrol et
        if let Some(existing) = self.semantic.iter_mut().find(|k| 
            k.subject == subject && k.predicate == predicate && k.object == object
        ) {
            existing.reinforce();
        } else {
            self.semantic.push(SemanticKnowledge::new(subject, predicate, object));
        }
        
        log::debug!("🧠 MEMORY: Knowledge added: {} -> {} -> {}", subject, predicate, object);
    }
    
    /// Bilgi sorgula
    pub fn query_knowledge(&self, subject: &str, predicate: &str) -> Vec<&SemanticKnowledge> {
        self.semantic.iter()
            .filter(|k| k.subject == subject && k.predicate == predicate)
            .collect()
    }
    
    /// Beceri kaydet
    pub fn register_skill(&mut self, skill: ProceduralSkill) {
        self.procedural.insert(skill.name.clone(), skill);
        log::info!("🧠 MEMORY: Skill registered: {}", self.procedural.len());
    }
    
    /// Beceri kullan
    pub fn use_skill(&mut self, name: &str, success: bool) -> Option<&ProceduralSkill> {
        if let Some(skill) = self.procedural.get_mut(name) {
            skill.record_usage(success);
            return Some(skill);
        }
        None
    }
    
    /// Benzer durumlardan öğren
    pub fn learn_from_similar(&self, context: &str) -> Option<String> {
        // En son benzer durumu bul
        for episode in self.episodic.iter().rev() {
            for (_, value) in &episode.context {
                if value.contains(context) {
                    if let Some(lesson) = &episode.lesson {
                        return Some(lesson.clone());
                    }
                }
            }
        }
        None
    }
    
    /// Başarılı stratejileri getir
    pub fn get_successful_strategies(&self, task_type: &str) -> Vec<&Episode> {
        self.episodic.iter()
            .filter(|e| e.task_name.contains(task_type) && e.success)
            .take(10)
            .collect()
    }
    
    /// Değişken ayarla
    pub fn set_variable(&mut self, name: &str, value: serde_json::Value) {
        self.working.variables.insert(name.into(), value);
    }
    
    /// Değişken al
    pub fn get_variable(&self, name: &str) -> Option<&serde_json::Value> {
        self.working.variables.get(name)
    }
    
    /// Aktif görev ayarla
    pub fn set_current_task(&mut self, task: &str) {
        self.working.current_task = Some(task.into());
    }
    
    /// Aktif hedef ayarla
    pub fn set_current_goal(&mut self, goal: &str) {
        self.working.current_goal = Some(goal.into());
    }
    
    /// Son aksiyonları al
    pub fn recent_actions(&self, n: usize) -> Vec<&(String, bool)> {
        self.working.recent_actions.iter().rev().take(n).collect()
    }
    
    /// Başarı oranı al
    pub fn success_rate(&self) -> f32 {
        if self.working.recent_actions.is_empty() {
            return 0.0;
        }
        
        let success_count = self.working.recent_actions.iter().filter(|(_, s)| *s).count();
        success_count as f32 / self.working.recent_actions.len() as f32
    }
    
    /// İstatistikler
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            episodic_count: self.episodic.len(),
            semantic_count: self.semantic.len(),
            procedural_count: self.procedural.len(),
            recent_success_rate: self.success_rate(),
        }
    }
    
    /// Belleği temizle
    pub fn clear(&mut self) {
        self.episodic.clear();
        self.working = WorkingMemory::default();
    }
}

impl Default for AdvancedMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Bellek istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub episodic_count: usize,
    pub semantic_count: usize,
    pub procedural_count: usize,
    pub recent_success_rate: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_semantic_knowledge() {
        let mut k = SemanticKnowledge::new("button", "location", "top-right");
        assert_eq!(k.confidence, 0.5);
        
        k.reinforce();
        assert!(k.confidence > 0.5);
    }
    
    #[test]
    fn test_procedural_skill() {
        let skill = ProceduralSkill::new(
            "login",
            "Login işlemi",
            vec![
                SkillStep {
                    description: "Username gir".into(),
                    action_pattern: "type_username".into(),
                    success_condition: Some("input_filled".into()),
                },
            ],
        );
        
        assert_eq!(skill.steps.len(), 1);
    }
    
    #[tokio::test]
    async fn test_memory_creation() {
        let memory = AdvancedMemory::new();
        assert!(memory.episodic.is_empty());
    }
    
    #[tokio::test]
    async fn test_record_episode() {
        let mut memory = AdvancedMemory::new();
        memory.set_current_task("test");
        
        let obs = Observation::default();
        let action = Action::NoOp;
        let result = ActionResult::success("test");
        
        memory.record_episode(&obs, &action, &result).await.expect("operation failed");
        
        assert_eq!(memory.episodic.len(), 1);
    }
    
    #[test]
    fn test_add_knowledge() {
        let mut memory = AdvancedMemory::new();
        memory.add_knowledge("app", "has", "button");
        
        assert_eq!(memory.semantic.len(), 1);
    }
}
