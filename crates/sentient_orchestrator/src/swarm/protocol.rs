//! ─── SWARM PROTOKOLÜ ───
//!
//! Ajanlar arası iletişim protokolü - el sıkışma, müzakere, senkronizasyon.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::{SwarmAgentId, SwarmTask};
use super::agent_type::{AgentType, AgentCapability, AgentPersona};
use super::message::{SwarmMessage, MessageType, MessagePriority};

/// ─── SWARM PROTOCOL ───
/// 
/// Ajanlar arası iletişim protokolü.

pub struct SwarmProtocol {
    /// Aktif el sıkışmalar
    handshakes: HashMap<String, Handshake>,
    /// Müzakere geçmişi
    negotiations: Vec<Negotiation>,
    /// Protokol versiyonu
    version: String,
}

impl SwarmProtocol {
    pub fn new() -> Self {
        Self {
            handshakes: HashMap::new(),
            negotiations: Vec::new(),
            version: "1.0.0".into(),
        }
    }
    
    /// El sıkışma başlat
    pub fn initiate_handshake(
        &mut self, 
        from: SwarmAgentId, 
        from_type: AgentType,
        to: SwarmAgentId,
        capabilities: Vec<AgentCapability>
    ) -> Handshake {
        let handshake_id = format!("hs_{}", Uuid::new_v4());
        
        let handshake = Handshake {
            id: handshake_id.clone(),
            initiator: from,
            initiator_type: from_type,
            target: to,
            capabilities_offered: capabilities,
            status: HandshakeStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
        };
        
        self.handshakes.insert(handshake_id, handshake.clone());
        handshake
    }
    
    /// El sıkışmayı kabul et
    pub fn accept_handshake(&mut self, handshake_id: &str) -> Option<Handshake> {
        if let Some(handshake) = self.handshakes.get_mut(handshake_id) {
            handshake.status = HandshakeStatus::Accepted;
            handshake.completed_at = Some(Utc::now());
            return Some(handshake.clone());
        }
        None
    }
    
    /// El sıkışmayı reddet
    pub fn reject_handshake(&mut self, handshake_id: &str, reason: String) -> Option<Handshake> {
        if let Some(handshake) = self.handshakes.get_mut(handshake_id) {
            handshake.status = HandshakeStatus::Rejected { reason };
            handshake.completed_at = Some(Utc::now());
            return Some(handshake.clone());
        }
        None
    }
    
    /// Görev müzakeresi başlat
    pub fn negotiate_task(
        &mut self,
        task: &SwarmTask,
        available_agents: Vec<(SwarmAgentId, AgentPersona)>,
    ) -> Negotiation {
        let negotiation_id = format!("neg_{}", Uuid::new_v4());
        
        // Her ajan için uygunluk skoru hesapla
        let mut proposals: Vec<AgentProposal> = available_agents.iter()
            .map(|(id, persona)| {
                let score = self.calculate_suitability(task, persona);
                AgentProposal {
                    agent_id: id.clone(),
                    agent_type: persona.agent_type,
                    score,
                    rationale: format!("{:?} için uygunluk", persona.agent_type),
                }
            })
            .collect();
        
        // Skorlara göre sırala
        proposals.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        let negotiation = Negotiation {
            id: negotiation_id.clone(),
            task_id: task.id,
            task_description: task.description.clone(),
            proposals,
            selected_agent: None,
            status: NegotiationStatus::Pending,
            created_at: Utc::now(),
            resolved_at: None,
        };
        
        self.negotiations.push(negotiation.clone());
        negotiation
    }
    
    /// Uygunluk skoru hesapla
    fn calculate_suitability(&self, task: &SwarmTask, persona: &AgentPersona) -> f32 {
        let mut score = 0.0f32;
        
        // Yetenek eşleşmesi
        let agent_caps: std::collections::HashSet<_> = persona.all_capabilities().into_iter().collect();
        for required in &task.required_capabilities {
            if agent_caps.contains(required) {
                score += 10.0;
            }
        }
        
        // Öncelik ağırlığı
        score *= persona.priority_weight;
        
        // Öncelik bonusu
        score += match task.priority {
            MessagePriority::Critical => 20.0,
            MessagePriority::Urgent => 15.0,
            MessagePriority::High => 10.0,
            MessagePriority::Normal => 5.0,
            MessagePriority::Low => 0.0,
        };
        
        score
    }
    
    /// Müzakerede ajan seç
    pub fn select_agent(&mut self, negotiation_id: &str, agent_id: SwarmAgentId) -> Option<Negotiation> {
        if let Some(neg) = self.negotiations.iter_mut().find(|n| n.id == negotiation_id) {
            neg.selected_agent = Some(agent_id);
            neg.status = NegotiationStatus::Resolved;
            neg.resolved_at = Some(Utc::now());
            return Some(neg.clone());
        }
        None
    }
    
    /// Aktif el sıkışmaları
    pub fn active_handshakes(&self) -> Vec<&Handshake> {
        self.handshakes.values()
            .filter(|h| h.status == HandshakeStatus::Pending)
            .collect()
    }
    
    /// Aktif müzakereler
    pub fn active_negotiations(&self) -> Vec<&Negotiation> {
        self.negotiations.iter()
            .filter(|n| n.status == NegotiationStatus::Pending)
            .collect()
    }
}

impl Default for SwarmProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── HANDSHAKE ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handshake {
    /// El sıkışma ID
    pub id: String,
    /// Başlatan ajan
    pub initiator: SwarmAgentId,
    /// Başlatan tip
    pub initiator_type: AgentType,
    /// Hedef ajan
    pub target: SwarmAgentId,
    /// Sunulan yetenekler
    pub capabilities_offered: Vec<AgentCapability>,
    /// Durum
    pub status: HandshakeStatus,
    /// Oluşturulma
    pub created_at: DateTime<Utc>,
    /// Tamamlanma
    pub completed_at: Option<DateTime<Utc>>,
}

impl Handshake {
    pub fn duration_ms(&self) -> Option<i64> {
        self.completed_at.map(|end| {
            (end - self.created_at).num_milliseconds()
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HandshakeStatus {
    Pending,
    Accepted,
    Rejected { reason: String },
}

/// ─── NEGOTIATION ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Negotiation {
    /// Müzakere ID
    pub id: String,
    /// Görev ID
    pub task_id: Uuid,
    /// Görev açıklaması
    pub task_description: String,
    /// Ajan önerileri
    pub proposals: Vec<AgentProposal>,
    /// Seçilen ajan
    pub selected_agent: Option<SwarmAgentId>,
    /// Durum
    pub status: NegotiationStatus,
    /// Oluşturulma
    pub created_at: DateTime<Utc>,
    /// Çözüm
    pub resolved_at: Option<DateTime<Utc>>,
}

impl Negotiation {
    pub fn best_proposal(&self) -> Option<&AgentProposal> {
        self.proposals.first()
    }
    
    pub fn duration_ms(&self) -> Option<i64> {
        self.resolved_at.map(|end| {
            (end - self.created_at).num_milliseconds()
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NegotiationStatus {
    Pending,
    Resolved,
    Failed,
}

/// ─── AGENT PROPOSAL ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProposal {
    pub agent_id: SwarmAgentId,
    pub agent_type: AgentType,
    pub score: f32,
    pub rationale: String,
}

/// ─── SYNCHRONIZATION ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMessage {
    pub sender: SwarmAgentId,
    pub sync_type: SyncType,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncType {
    /// Tam durum senkronizasyonu
    FullState,
    /// Artımlı güncelleme
    Delta,
    /// Kalp atışı
    Heartbeat,
    /// Yankı (ping-pong)
    Echo,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_handshake_creation() {
        let mut protocol = SwarmProtocol::new();
        let from = SwarmAgentId::new();
        let to = SwarmAgentId::new();
        
        let handshake = protocol.initiate_handshake(
            from, 
            AgentType::Coordinator,
            to, 
            vec![AgentCapability::Orchestration]
        );
        
        assert_eq!(handshake.status, HandshakeStatus::Pending);
    }
    
    #[test]
    fn test_handshake_acceptance() {
        let mut protocol = SwarmProtocol::new();
        let from = SwarmAgentId::new();
        let to = SwarmAgentId::new();
        
        let handshake = protocol.initiate_handshake(
            from, AgentType::Coordinator, to, vec![]
        );
        
        let accepted = protocol.accept_handshake(&handshake.id).unwrap();
        assert_eq!(accepted.status, HandshakeStatus::Accepted);
    }
    
    #[test]
    fn test_negotiation_best_proposal() {
        let mut protocol = SwarmProtocol::new();
        
        let task = SwarmTask::new("Test görevi")
            .require(AgentCapability::WebSearch);
        
        let persona1 = AgentPersona::new(AgentType::Researcher);
        let persona2 = AgentPersona::new(AgentType::Coder);
        
        let agent1 = SwarmAgentId::new();
        let agent2 = SwarmAgentId::new();
        
        let neg = protocol.negotiate_task(&task, vec![
            (agent1.clone(), persona1),
            (agent2, persona2),
        ]);
        
        // Researcher daha yüksek skor almalı
        assert!(neg.best_proposal().unwrap().agent_type == AgentType::Researcher);
    }
    
    #[test]
    fn test_negotiation_agent_selection() {
        let mut protocol = SwarmProtocol::new();
        
        let task = SwarmTask::new("Test");
        let agent = SwarmAgentId::new();
        let persona = AgentPersona::new(AgentType::Researcher);
        
        let neg = protocol.negotiate_task(&task, vec![
            (agent.clone(), persona)
        ]);
        
        let resolved = protocol.select_agent(&neg.id, agent).unwrap();
        assert_eq!(resolved.status, NegotiationStatus::Resolved);
    }
}
