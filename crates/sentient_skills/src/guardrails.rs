//! Guardrails Middleware - DeerFlow'dan esinlenilmiş güvenlik katmanı
//!
//! Tool call'ları değerlendirir ve policy-based erişim kontrolü sağlar

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use async_trait::async_trait;

/// Tool Call Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    /// Tool adı
    pub tool_name: String,
    
    /// Tool argümanları
    pub args: HashMap<String, serde_json::Value>,
    
    /// Agent ID (kim çağırıyor)
    pub agent_id: Option<String>,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Guardrail Decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailDecision {
    /// İzin verildi mi
    pub allow: bool,
    
    /// Policy ID (hangi policy tetiklendi)
    pub policy_id: Option<String>,
    
    /// Red sebepleri
    pub reasons: Vec<GuardrailReason>,
}

/// Guardrail Reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailReason {
    /// Hata kodu
    pub code: String,
    
    /// Mesaj
    pub message: String,
}

impl GuardrailDecision {
    /// İzin ver
    pub fn allowed() -> Self {
        Self {
            allow: true,
            policy_id: None,
            reasons: Vec::new(),
        }
    }
    
    /// Reddet
    pub fn denied(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            allow: false,
            policy_id: None,
            reasons: vec![GuardrailReason {
                code: code.into(),
                message: message.into(),
            }],
        }
    }
    
    /// Policy ile reddet
    pub fn denied_by_policy(policy_id: impl Into<String>, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            allow: false,
            policy_id: Some(policy_id.into()),
            reasons: vec![GuardrailReason {
                code: code.into(),
                message: message.into(),
            }],
        }
    }
}

/// Guardrail Provider Trait
#[async_trait]
pub trait GuardrailProvider: Send + Sync {
    /// Request'i değerlendir (sync)
    fn evaluate(&self, request: &ToolCallRequest) -> GuardrailDecision;
    
    /// Request'i değerlendir (async)
    async fn evaluate_async(&self, request: &ToolCallRequest) -> GuardrailDecision {
        self.evaluate(request)
    }
}

/// Basit Rule-based Guardrail Provider
pub struct RuleBasedGuardrail {
    /// İzin verilen tool'lar
    allowed_tools: Vec<String>,
    
    /// Yasaklanan tool'lar
    blocked_tools: Vec<String>,
    
    /// Yasaklanan argüman pattern'leri
    blocked_patterns: Vec<String>,
    
    /// Maksimum argüman uzunluğu
    max_arg_length: usize,
}

impl RuleBasedGuardrail {
    /// Yeni provider oluştur
    pub fn new() -> Self {
        Self {
            allowed_tools: Vec::new(),
            blocked_tools: vec![
                "eval".to_string(),
                "exec".to_string(),
                "system".to_string(),
                "subprocess".to_string(),
            ],
            blocked_patterns: vec![
                "rm -rf".to_string(),
                "sudo".to_string(),
                "chmod 777".to_string(),
                "> /dev/".to_string(),
            ],
            max_arg_length: 100000,
        }
    }
    
    /// İzin verilen tool ekle
    pub fn allow_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.push(tool.into());
        self
    }
    
    /// Yasaklanan tool ekle
    pub fn block_tool(mut self, tool: impl Into<String>) -> Self {
        self.blocked_tools.push(tool.into());
        self
    }
    
    /// Yasaklanan pattern ekle
    pub fn block_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.blocked_patterns.push(pattern.into());
        self
    }
    
    /// Argümanları kontrol et
    fn check_args(&self, args: &HashMap<String, serde_json::Value>) -> Option<GuardrailReason> {
        for (key, value) in args {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            
            // Uzunluk kontrolü
            if value_str.len() > self.max_arg_length {
                return Some(GuardrailReason {
                    code: "arg.too_long".to_string(),
                    message: format!("Argument '{}' exceeds maximum length", key),
                });
            }
            
            // Pattern kontrolü
            for pattern in &self.blocked_patterns {
                if value_str.contains(pattern) {
                    return Some(GuardrailReason {
                        code: "arg.blocked_pattern".to_string(),
                        message: format!("Argument contains blocked pattern: {}", pattern),
                    });
                }
            }
        }
        
        None
    }
}

impl Default for RuleBasedGuardrail {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GuardrailProvider for RuleBasedGuardrail {
    fn evaluate(&self, request: &ToolCallRequest) -> GuardrailDecision {
        // Tool kontrolü
        if self.blocked_tools.contains(&request.tool_name) {
            return GuardrailDecision::denied_by_policy(
                "blocked_tool",
                "tool.blocked",
                format!("Tool '{}' is blocked by security policy", request.tool_name),
            );
        }
        
        // Eğer allowlist varsa, listede olmayanları reddet
        if !self.allowed_tools.is_empty() && !self.allowed_tools.contains(&request.tool_name) {
            return GuardrailDecision::denied_by_policy(
                "not_in_allowlist",
                "tool.not_allowed",
                format!("Tool '{}' is not in allowed list", request.tool_name),
            );
        }
        
        // Argüman kontrolü
        if let Some(reason) = self.check_args(&request.args) {
            return GuardrailDecision {
                allow: false,
                policy_id: Some("arg_validation".to_string()),
                reasons: vec![reason],
            };
        }
        
        GuardrailDecision::allowed()
    }
}

/// Guardrail Middleware
pub struct GuardrailMiddleware {
    /// Provider
    provider: Arc<dyn GuardrailProvider>,
    
    /// Fail-closed mode (hata durumunda engelle)
    fail_closed: bool,
    
    /// Passport (agent kimliği)
    passport: Option<String>,
    
    /// İstatistikler
    stats: Arc<RwLock<GuardrailStats>>,
}

/// İstatistikler
#[derive(Debug, Default, Clone)]
pub struct GuardrailStats {
    pub total_calls: u64,
    pub allowed: u64,
    pub denied: u64,
    pub errors: u64,
}

impl GuardrailMiddleware {
    /// Yeni middleware oluştur
    pub fn new(provider: Arc<dyn GuardrailProvider>) -> Self {
        Self {
            provider,
            fail_closed: true,
            passport: None,
            stats: Arc::new(RwLock::new(GuardrailStats::default())),
        }
    }
    
    /// Rule-based provider ile oluştur
    pub fn with_rules() -> Self {
        Self::new(Arc::new(RuleBasedGuardrail::new()))
    }
    
    /// Fail-closed mode ayarla
    pub fn fail_closed(mut self, value: bool) -> Self {
        self.fail_closed = value;
        self
    }
    
    /// Passport ayarla
    pub fn with_passport(mut self, passport: impl Into<String>) -> Self {
        self.passport = Some(passport.into());
        self
    }
    
    /// Tool call'u değerlendir
    pub fn evaluate(&self, request: &ToolCallRequest) -> GuardrailDecision {
        // İstatistik güncelle
        {
            let mut stats = self.stats.write();
            stats.total_calls += 1;
        }
        
        let decision = self.provider.evaluate(request);
        
        // İstatistik güncelle
        {
            let mut stats = self.stats.write();
            if decision.allow {
                stats.allowed += 1;
            } else {
                stats.denied += 1;
            }
        }
        
        decision
    }
    
    /// Async değerlendir
    pub async fn evaluate_async(&self, request: &ToolCallRequest) -> GuardrailDecision {
        self.provider.evaluate_async(request).await
    }
    
    /// İstatistikleri al
    pub fn stats(&self) -> GuardrailStats {
        self.stats.read().clone()
    }
    
    /// Red mesajı oluştur
    pub fn build_denied_message(request: &ToolCallRequest, decision: &GuardrailDecision) -> String {
        let reason_text = decision.reasons
            .first()
            .map(|r| &r.message)
            .map(|s| s.as_str())
            .unwrap_or("blocked by guardrail policy");
        
        let reason_code = decision.reasons
            .first()
            .map(|r| &r.code)
            .map(|s| s.as_str())
            .unwrap_or("denied");
        
        format!(
            "Guardrail denied: tool '{}' was blocked ({}). Reason: {}. Choose an alternative approach.",
            request.tool_name, reason_code, reason_text
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guardrail_decision() {
        let allowed = GuardrailDecision::allowed();
        assert!(allowed.allow);
        
        let denied = GuardrailDecision::denied("test.code", "Test message");
        assert!(!denied.allow);
        assert_eq!(denied.reasons.len(), 1);
    }
    
    #[test]
    fn test_rule_based_guardrail() {
        let guardrail = RuleBasedGuardrail::new()
            .allow_tool("web_search")
            .block_pattern("password");
        
        // İzin verilen tool
        let request = ToolCallRequest {
            tool_name: "web_search".to_string(),
            args: HashMap::new(),
            agent_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(guardrail.evaluate(&request).allow);
        
        // Yasaklanan tool
        let request = ToolCallRequest {
            tool_name: "exec".to_string(),
            args: HashMap::new(),
            agent_id: None,
            timestamp: chrono::Utc::now(),
        };
        assert!(!guardrail.evaluate(&request).allow);
    }
    
    #[test]
    fn test_middleware() {
        let middleware = GuardrailMiddleware::with_rules();
        
        let request = ToolCallRequest {
            tool_name: "test_tool".to_string(),
            args: HashMap::new(),
            agent_id: None,
            timestamp: chrono::Utc::now(),
        };
        
        let decision = middleware.evaluate(&request);
        assert!(decision.allow);
        
        let stats = middleware.stats();
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.allowed, 1);
    }
}
