//! ─── COMPACTION ───
//!
//! Bağlam sıkıştırma motoru

use crate::{Session, SessionResult};
use serde::{Deserialize, Serialize};

/// Sıkıştırma yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionConfig {
    /// Hedef sıkıştırma oranı (0.0 - 1.0)
    pub target_ratio: f32,
    /// Minimum tutulacak mesaj
    pub min_messages: usize,
    /// Özet için maksimum token
    pub max_summary_tokens: usize,
    /// Sıkıştırma stratejisi
    pub strategy: CompactionStrategy,
}

impl Default for CompactionConfig {
    fn default() -> Self {
        Self {
            target_ratio: 0.3,
            min_messages: 4,
            max_summary_tokens: 500,
            strategy: CompactionStrategy::SlidingWindow,
        }
    }
}

/// Sıkıştırma stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactionStrategy {
    /// Kayan pencere - en son N mesajı tut
    SlidingWindow,
    /// Özetleme - eski mesajları özetle
    Summarization,
    /// Hibrit - her ikisi
    Hybrid,
    /// Token bazlı - token limitine göre
    TokenBased,
}

/// Sıkıştırma sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionResult {
    /// Özet
    pub summary: String,
    /// Tutulan mesaj sayısı
    pub kept_messages: usize,
    /// Sıkıştırılan mesaj sayısı
    pub compacted_messages: usize,
    /// Önceki token sayısı
    pub tokens_before: u64,
    /// Sonraki token sayısı
    pub tokens_after: u64,
    /// Sıkıştırma oranı
    pub compression_ratio: f32,
}

/// Sıkıştırma motoru
pub struct Compactor {
    config: CompactionConfig,
}

impl Compactor {
    pub fn new(config: CompactionConfig) -> Self {
        Self { config }
    }
    
    /// Oturumu sıkıştır
    pub fn compact(&self, session: &Session) -> SessionResult<CompactionResult> {
        if session.messages.len() <= self.config.min_messages {
            return Ok(CompactionResult {
                summary: "Sıkıştırma gerekli değil".into(),
                kept_messages: session.messages.len(),
                compacted_messages: 0,
                tokens_before: session.token_count,
                tokens_after: session.token_count,
                compression_ratio: 1.0,
            });
        }
        
        let tokens_before = session.token_count;
        
        match self.config.strategy {
            CompactionStrategy::SlidingWindow => self.sliding_window_compact(session, tokens_before),
            CompactionStrategy::Summarization => self.summarization_compact(session, tokens_before),
            CompactionStrategy::Hybrid => self.hybrid_compact(session, tokens_before),
            CompactionStrategy::TokenBased => self.token_based_compact(session, tokens_before),
        }
    }
    
    /// Kayan pencere sıkıştırması
    fn sliding_window_compact(&self, session: &Session, tokens_before: u64) -> SessionResult<CompactionResult> {
        let kept_count = self.config.min_messages;
        let total = session.messages.len();
        let compacted = total.saturating_sub(kept_count);
        
        // Özet oluştur
        let summary = self.create_summary(&session.messages[..compacted]);
        
        // Token tahmini
        let tokens_after = session.messages[compacted..]
            .iter()
            .map(|m| m.token_count as u64)
            .sum::<u64>() + summary.len() as u64 / 4; // Yaklaşık token
        
        Ok(CompactionResult {
            summary,
            kept_messages: kept_count,
            compacted_messages: compacted,
            tokens_before,
            tokens_after,
            compression_ratio: tokens_after as f32 / tokens_before as f32,
        })
    }
    
    /// Özetleme sıkıştırması
    fn summarization_compact(&self, session: &Session, tokens_before: u64) -> SessionResult<CompactionResult> {
        let split_point = session.messages.len() / 2;
        let summary = self.create_summary(&session.messages[..split_point]);
        
        let tokens_after = session.messages[split_point..]
            .iter()
            .map(|m| m.token_count as u64)
            .sum::<u64>() + summary.len() as u64 / 4;
        
        Ok(CompactionResult {
            summary,
            kept_messages: session.messages.len() - split_point,
            compacted_messages: split_point,
            tokens_before,
            tokens_after,
            compression_ratio: tokens_after as f32 / tokens_before as f32,
        })
    }
    
    /// Hibrit sıkıştırma
    fn hybrid_compact(&self, session: &Session, tokens_before: u64) -> SessionResult<CompactionResult> {
        // İlk yarısını özetle, son kısmı kayan pencere ile tut
        let window_size = self.config.min_messages / 2;
        let summary_point = session.messages.len().saturating_sub(window_size);
        
        let summary = if summary_point > 0 {
            self.create_summary(&session.messages[..summary_point])
        } else {
            String::new()
        };
        
        let kept = session.messages.len().saturating_sub(summary_point);
        let compacted = summary_point;
        
        let tokens_after = session.messages[summary_point..]
            .iter()
            .map(|m| m.token_count as u64)
            .sum::<u64>() + summary.len() as u64 / 4;
        
        Ok(CompactionResult {
            summary,
            kept_messages: kept,
            compacted_messages: compacted,
            tokens_before,
            tokens_after,
            compression_ratio: tokens_after as f32 / tokens_before as f32,
        })
    }
    
    /// Token bazlı sıkıştırma
    fn token_based_compact(&self, session: &Session, tokens_before: u64) -> SessionResult<CompactionResult> {
        let target_tokens = (tokens_before as f32 * self.config.target_ratio) as u64;
        
        // Sondan başlayarak mesajları topla
        let mut current_tokens = 0u64;
        let mut kept_count = 0;
        
        for msg in session.messages.iter().rev() {
            if current_tokens + msg.token_count as u64 > target_tokens {
                break;
            }
            current_tokens += msg.token_count as u64;
            kept_count += 1;
        }
        
        let compacted = session.messages.len().saturating_sub(kept_count);
        let summary = if compacted > 0 {
            self.create_summary(&session.messages[..compacted])
        } else {
            String::new()
        };
        
        let tokens_after = current_tokens + summary.len() as u64 / 4;
        
        Ok(CompactionResult {
            summary,
            kept_messages: kept_count,
            compacted_messages: compacted,
            tokens_before,
            tokens_after,
            compression_ratio: tokens_after as f32 / tokens_before as f32,
        })
    }
    
    /// Özet oluştur
    fn create_summary(&self, messages: &[crate::session::Message]) -> String {
        if messages.is_empty() {
            return String::new();
        }
        
        // Basit özet - gerçekte LLM kullanılmalı
        let mut summary = String::from("[ÖZET] ");
        
        // Kullanıcı mesajlarını topla
        let user_messages: Vec<_> = messages.iter()
            .filter(|m| matches!(m.role, crate::session::MessageRole::User))
            .collect();
        
        // Asistan mesajlarını topla
        let assistant_messages: Vec<_> = messages.iter()
            .filter(|m| matches!(m.role, crate::session::MessageRole::Assistant))
            .collect();
        
        summary.push_str(&format!("{} kullanıcı girdisi, {} asistan yanıtı. ", 
            user_messages.len(), 
            assistant_messages.len()
        ));
        
        // İlk ve son kullanıcı mesajı
        if let Some(first) = user_messages.first() {
            let preview: String = first.content.chars().take(50).collect();
            summary.push_str(&format!("İlk: \"{}...\" ", preview));
        }
        
        if let Some(last) = user_messages.last() {
            let preview: String = last.content.chars().take(50).collect();
            summary.push_str(&format!("Son: \"{}...\"", preview));
        }
        
        summary
    }
}

impl Default for Compactor {
    fn default() -> Self {
        Self::new(CompactionConfig::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{SessionConfig, Message};
    
    #[test]
    fn test_compaction_small_session() {
        let compactor = Compactor::default();
        let mut session = Session::new(SessionConfig::default());
        session.start().expect("operation failed");
        
        // Az mesaj - sıkıştırma olmamalı
        session.add_message(Message::user("Merhaba", 10));
        session.add_message(Message::assistant("Merhaba!", 20));
        
        let result = compactor.compact(&session).expect("operation failed");
        assert_eq!(result.compacted_messages, 0);
    }
    
    #[test]
    fn test_sliding_window() {
        let config = CompactionConfig {
            min_messages: 2,
            ..Default::default()
        };
        let compactor = Compactor::new(config);
        let mut session = Session::new(SessionConfig::default());
        session.start().expect("operation failed");
        
        // 6 mesaj ekle
        for i in 0..6 {
            session.add_message(Message::user(&format!("Mesaj {}", i), 100));
        }
        
        let result = compactor.compact(&session).expect("operation failed");
        assert!(result.compression_ratio < 1.0);
    }
}
