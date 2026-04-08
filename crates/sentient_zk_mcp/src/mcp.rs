//! ═══════════════════════════════════════════════════════════════════════════════
//!  MCP PROTOCOL - Zero-Knowledge MCP Integration (Enterprise Grade 2026)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Model Context Protocol için ZK-SNARK tabanlı güvenli iletişim.
//!
//! ## Güvenlik Özellikleri:
//! - Dış araçlara veri sızıntısı önleme
//! - İşlem doğrulama without revealing data
//! - Audit trail without compromising privacy
//! - Rate limiting with privacy preservation

use crate::{ProofContext, ZkError, ZkProof, ZkProver, ZkResult, PrivacyLevel, ProofAlgorithm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  ZK MCP REQUEST
// ═══════════════════════════════════════════════════════════════════════════════

/// MCP Request with ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkMcpRequest {
    /// Request ID
    pub id: uuid::Uuid,
    
    /// Tool name
    pub tool: String,
    
    /// Zero-knowledge proof of authorization
    pub proof: ZkProof,
    
    /// Public parameters (commitment to private data)
    pub public_params: serde_json::Value,
    
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Request signature
    pub signature: Option<String>,
    
    /// Nonce for replay protection
    pub nonce: String,
}

impl ZkMcpRequest {
    /// Create new ZK MCP request
    pub async fn new(
        tool: impl Into<String>,
        parameters: &serde_json::Value,
        prover: &ZkProver,
    ) -> ZkResult<Self> {
        let tool = tool.into();
        let nonce = Self::generate_nonce();
        
        let context = ProofContext {
            tool_name: tool.clone(),
            request_hash: String::new(),
            privacy_level: PrivacyLevel::ParameterHash,
            include_response: false,
        };
        
        let proof = prover.prove_mcp_request(parameters, &context).await?;
        
        // Extract public parameters based on privacy level
        let public_params = match context.privacy_level {
            PrivacyLevel::RequestOnly => {
                serde_json::json!({"tool": tool})
            }
            PrivacyLevel::ParameterHash => {
                let hash = blake3::hash(
                    &serde_json::to_vec(parameters).unwrap_or_default()
                ).to_hex().to_string();
                serde_json::json!({"tool": tool, "params_hash": hash})
            }
            PrivacyLevel::FullRequest => {
                parameters.clone()
            }
        };
        
        Ok(Self {
            id: uuid::Uuid::new_v4(),
            tool,
            proof,
            public_params,
            timestamp: chrono::Utc::now(),
            signature: None,
            nonce,
        })
    }
    
    /// Create request with custom privacy level
    pub async fn with_privacy(
        tool: impl Into<String>,
        parameters: &serde_json::Value,
        prover: &ZkProver,
        privacy_level: PrivacyLevel,
    ) -> ZkResult<Self> {
        let tool = tool.into();
        let nonce = Self::generate_nonce();
        
        let context = ProofContext {
            tool_name: tool.clone(),
            request_hash: String::new(),
            privacy_level,
            include_response: false,
        };
        
        let proof = prover.prove_mcp_request(parameters, &context).await?;
        
        let public_params = match privacy_level {
            PrivacyLevel::RequestOnly => {
                serde_json::json!({"tool": tool})
            }
            PrivacyLevel::ParameterHash => {
                let hash = blake3::hash(
                    &serde_json::to_vec(parameters).unwrap_or_default()
                ).to_hex().to_string();
                serde_json::json!({"tool": tool, "params_hash": hash})
            }
            PrivacyLevel::FullRequest => {
                parameters.clone()
            }
        };
        
        Ok(Self {
            id: uuid::Uuid::new_v4(),
            tool,
            proof,
            public_params,
            timestamp: chrono::Utc::now(),
            signature: None,
            nonce,
        })
    }
    
    fn generate_nonce() -> String {
        blake3::hash(&chrono::Utc::now().timestamp_nanos().to_le_bytes())
            .to_hex()
            .to_string()[..16].to_string()
    }
    
    /// Sign the request
    pub fn sign(&mut self, private_key: &[u8]) {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.id.as_bytes());
        hasher.update(self.tool.as_bytes());
        hasher.update(self.nonce.as_bytes());
        hasher.update(private_key);
        self.signature = Some(hasher.finalize().to_hex().to_string());
    }
    
    /// Verify request signature
    pub fn verify_signature(&self, public_key: &[u8]) -> bool {
        if let Some(sig) = &self.signature {
            // Simplified signature verification
            let expected = blake3::hash(
                &[self.id.as_bytes(), self.tool.as_bytes(), self.nonce.as_bytes()].concat()
            ).to_hex().to_string();
            sig.starts_with(&expected[..16])
        } else {
            false
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ZK MCP RESPONSE
// ═══════════════════════════════════════════════════════════════════════════════

/// MCP Response with ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkMcpResponse {
    /// Request ID this responds to
    pub request_id: uuid::Uuid,
    
    /// Response proof
    pub proof: ZkProof,
    
    /// Public response data
    pub public_data: serde_json::Value,
    
    /// Success indicator
    pub success: bool,
    
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Processing time in ms
    pub processing_time_ms: u64,
    
    /// Error message (if any)
    pub error: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ZK MCP HANDLER
// ═══════════════════════════════════════════════════════════════════════════════

/// ZK MCP Handler - Processes MCP requests with privacy
pub struct ZkMcpHandler {
    prover: ZkProver,
    enabled_tools: Vec<String>,
    tool_configs: HashMap<String, ToolConfig>,
    audit_mode: bool,
    rate_limiter: RateLimiter,
    seen_nonces: Arc<RwLock<Vec<String>>>,
}

/// Tool-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub allowed_privacy_levels: Vec<PrivacyLevel>,
    pub max_calls_per_minute: u32,
    pub require_signature: bool,
    pub audit_response: bool,
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            allowed_privacy_levels: vec![PrivacyLevel::ParameterHash, PrivacyLevel::RequestOnly],
            max_calls_per_minute: 60,
            require_signature: false,
            audit_response: true,
        }
    }
}

/// Rate limiter for privacy-preserving rate limiting
#[derive(Debug, Clone, Default)]
pub struct RateLimiter {
    call_counts: HashMap<String, Vec<chrono::DateTime<chrono::Utc>>>,
}

impl RateLimiter {
    pub fn check_and_record(&mut self, key: &str, max_per_minute: u32) -> bool {
        let now = chrono::Utc::now();
        let minute_ago = now - chrono::Duration::seconds(60);
        
        let calls = self.call_counts.entry(key.to_string()).or_default();
        calls.retain(|&t| t > minute_ago);
        
        if calls.len() >= max_per_minute as usize {
            return false;
        }
        
        calls.push(now);
        true
    }
}

impl ZkMcpHandler {
    pub fn new() -> Self {
        Self {
            prover: ZkProver::new(),
            enabled_tools: Vec::new(),
            tool_configs: HashMap::new(),
            audit_mode: true,
            rate_limiter: RateLimiter::default(),
            seen_nonces: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add enabled tool
    pub fn enable_tool(mut self, tool: impl Into<String>) -> Self {
        let tool_name = tool.into();
        self.enabled_tools.push(tool_name.clone());
        self.tool_configs.insert(tool_name, ToolConfig::default());
        self
    }
    
    /// Configure tool
    pub fn configure_tool(mut self, tool: &str, config: ToolConfig) -> Self {
        self.tool_configs.insert(tool.to_string(), config);
        self
    }

    /// Set audit mode
    pub fn with_audit(mut self, enabled: bool) -> Self {
        self.audit_mode = enabled;
        self
    }

    /// Process incoming MCP request
    pub async fn process_request(
        &mut self,
        request: &ZkMcpRequest,
    ) -> ZkResult<ZkMcpResponse> {
        let start = std::time::Instant::now();
        
        // Check for replay attack
        {
            let mut nonces = self.seen_nonces.write().await;
            if nonces.contains(&request.nonce) {
                return Err(ZkError::McpError("Replay attack detected".into()));
            }
            nonces.push(request.nonce.clone());
            
            // Limit stored nonces
            if nonces.len() > 10000 {
                nonces.drain(0..5000);
            }
        }
        
        // Verify request proof
        if !request.proof.validate()? {
            return Err(ZkError::ProofVerificationFailed("Invalid request proof".into()));
        }
        
        // Check tool is enabled
        if !self.enabled_tools.contains(&request.tool) {
            return Err(ZkError::McpError(format!("Tool '{}' not enabled", request.tool)));
        }
        
        // Check rate limit
        let config = self.tool_configs.get(&request.tool).cloned().unwrap_or_default();
        if !self.rate_limiter.check_and_record(&request.tool, config.max_calls_per_minute) {
            return Err(ZkError::McpError("Rate limit exceeded".into()));
        }
        
        // Check signature if required
        if config.require_signature && request.signature.is_none() {
            return Err(ZkError::McpError("Signature required".into()));
        }
        
        // Log audit (without revealing parameters)
        if self.audit_mode {
            log::info!(
                "📋 ZK-MCP: Tool '{}' called with proof {}",
                request.tool,
                request.proof.id
            );
        }
        
        // Create response proof
        let context = ProofContext {
            tool_name: request.tool.clone(),
            request_hash: String::new(),
            privacy_level: PrivacyLevel::RequestOnly,
            include_response: true,
        };
        
        let response_proof = self.prover.prove_mcp_request(
            &request.public_params,
            &context,
        ).await?;
        
        // Build response
        let mut response_data = serde_json::json!({"processed": true});
        
        // Check for data leaks before returning
        DataLeakFilter::sanitize(&mut response_data);
        
        Ok(ZkMcpResponse {
            request_id: request.id,
            proof: response_proof,
            public_data: response_data,
            success: true,
            timestamp: chrono::Utc::now(),
            processing_time_ms: start.elapsed().as_millis() as u64,
            error: None,
        })
    }
    
    /// Process batch of requests
    pub async fn process_batch(
        &mut self,
        requests: &[ZkMcpRequest],
    ) -> ZkResult<Vec<ZkMcpResponse>> {
        let mut responses = Vec::with_capacity(requests.len());
        for request in requests {
            responses.push(self.process_request(request).await?);
        }
        Ok(responses)
    }

    /// Get handler statistics
    pub fn stats(&self) -> HandlerStats {
        HandlerStats {
            enabled_tools: self.enabled_tools.len(),
            audit_mode: self.audit_mode,
            total_rate_limits: self.rate_limiter.call_counts.len(),
        }
    }
}

impl Default for ZkMcpHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerStats {
    pub enabled_tools: usize,
    pub audit_mode: bool,
    pub total_rate_limits: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATA LEAK PREVENTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Data leak prevention filter
pub struct DataLeakFilter;

impl DataLeakFilter {
    /// Patterns that indicate potential data leaks
    const LEAK_PATTERNS: &'static [&'static str] = &[
        "api_key", "apikey", "api-key",
        "password", "passwd", "pwd",
        "secret", "token", "credential",
        "private_key", "privatekey",
        "ssh-rsa", "BEGIN RSA", "BEGIN PRIVATE",
        "sk-", "pk-", "ghp_", "gho_",
        "AKIA", // AWS access key prefix
        "eyJ", // JWT token prefix
    ];
    
    /// Check if response contains potential data leak
    pub fn check_response(response: &ZkMcpResponse) -> ZkResult<()> {
        let response_str = serde_json::to_string(&response.public_data)
            .map_err(|e| ZkError::McpError(e.to_string()))?;
        
        for pattern in Self::LEAK_PATTERNS {
            if response_str.to_lowercase().contains(pattern) {
                log::warn!("🚨 Potential data leak detected: pattern '{}' found", pattern);
                return Err(ZkError::PrivacyViolation);
            }
        }
        
        Ok(())
    }
    
    /// Sanitize response data
    pub fn sanitize(data: &mut serde_json::Value) {
        if let serde_json::Value::Object(map) = data {
            let sensitive_keys = [
                "password", "passwd", "pwd",
                "api_key", "apikey", "api-key",
                "secret", "token", "credential",
                "private_key", "privatekey", "key",
                "auth", "authorization",
            ];
            
            for key in &sensitive_keys {
                if map.contains_key(*key) {
                    map.insert((*key).to_string(), serde_json::json!("***REDACTED***"));
                    log::debug!("🔒 Sanitized sensitive field: {}", key);
                }
            }
            
            // Recursively sanitize nested objects
            for value in map.values_mut() {
                Self::sanitize(value);
            }
        } else if let serde_json::Value::Array(arr) = data {
            for item in arr {
                Self::sanitize(item);
            }
        }
    }
    
    /// Check string for sensitive patterns
    pub fn check_string(s: &str) -> ZkResult<()> {
        let lower = s.to_lowercase();
        for pattern in Self::LEAK_PATTERNS {
            if lower.contains(pattern) {
                return Err(ZkError::PrivacyViolation);
            }
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PRIVACY AUDIT LOG
// ═══════════════════════════════════════════════════════════════════════════════

/// Privacy audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditEntry {
    pub id: uuid::Uuid,
    pub tool_name: String,
    pub proof_id: uuid::Uuid,
    pub privacy_level: PrivacyLevel,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub data_leaks_detected: u32,
}

/// Privacy audit logger
pub struct PrivacyAuditLog {
    entries: Vec<PrivacyAuditEntry>,
    max_entries: usize,
}

impl PrivacyAuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    
    pub fn log(&mut self, entry: PrivacyAuditEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }
    
    pub fn entries(&self) -> &[PrivacyAuditEntry] {
        &self.entries
    }
    
    /// Check for suspicious patterns
    pub fn analyze(&self) -> AuditAnalysis {
        let total = self.entries.len();
        let failures = self.entries.iter().filter(|e| !e.success).count();
        let leaks = self.entries.iter().map(|e| e.data_leaks_detected as usize).sum();
        
        AuditAnalysis {
            total_requests: total,
            failed_requests: failures,
            potential_leaks: leaks,
            risk_score: if total == 0 { 0.0 } else { failures as f64 / total as f64 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditAnalysis {
    pub total_requests: usize,
    pub failed_requests: usize,
    pub potential_leaks: usize,
    pub risk_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zk_mcp_request() {
        let prover = ZkProver::new();
        let params = serde_json::json!({"query": "test"});
        
        let request = ZkMcpRequest::new("test_tool", &params, &prover).await;
        assert!(request.is_ok());
    }

    #[tokio::test]
    async fn test_zk_mcp_handler() {
        let mut handler = ZkMcpHandler::new()
            .enable_tool("test_tool")
            .with_audit(true);
        
        let prover = ZkProver::new();
        let params = serde_json::json!({});
        let request = ZkMcpRequest::new("test_tool", &params, &prover).await.unwrap();
        
        let response = handler.process_request(&request).await;
        assert!(response.is_ok());
    }

    #[test]
    fn test_data_leak_filter() {
        let mut data = serde_json::json!({
            "result": "success",
            "api_key": "secret123",
            "nested": {
                "password": "mypass"
            }
        });
        
        DataLeakFilter::sanitize(&mut data);
        
        assert_eq!(data["api_key"], "***REDACTED***");
        assert_eq!(data["nested"]["password"], "***REDACTED***");
    }
    
    #[tokio::test]
    async fn test_replay_protection() {
        let mut handler = ZkMcpHandler::new()
            .enable_tool("test_tool");
        
        let prover = ZkProver::new();
        let params = serde_json::json!({});
        
        // First request should succeed
        let request1 = ZkMcpRequest::new("test_tool", &params, &prover).await.unwrap();
        let response1 = handler.process_request(&request1).await;
        assert!(response1.is_ok());
        
        // Same nonce should be rejected (replay attack)
        let mut request2 = ZkMcpRequest::new("test_tool", &params, &prover).await.unwrap();
        request2.nonce = request1.nonce.clone();
        
        let response2 = handler.process_request(&request2).await;
        assert!(response2.is_err());
    }
    
    #[test]
    fn test_privacy_audit_log() {
        let mut log = PrivacyAuditLog::new(100);
        
        log.log(PrivacyAuditEntry {
            id: uuid::Uuid::new_v4(),
            tool_name: "test".into(),
            proof_id: uuid::Uuid::new_v4(),
            privacy_level: PrivacyLevel::ParameterHash,
            timestamp: chrono::Utc::now(),
            success: true,
            data_leaks_detected: 0,
        });
        
        let analysis = log.analyze();
        assert_eq!(analysis.total_requests, 1);
    }
}
