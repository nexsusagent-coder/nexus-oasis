//! ═══════════════════════════════════════════════════════════════════════════════
//!  ANOMALY DETECTION ENGINE - Enterprise Grade 2026
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Gerçek zamanlı davranışsal anomali tespiti.
//! Sonsuz döngü, kaynak tüketimi ve davranış sapması tespiti.

use crate::{Anomaly, AnomalyConfig, AnomalySeverity, AnomalyType};
use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  ANOMALY ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Anomaly Detection Engine
/// 
/// # Features:
/// - Loop detection with pattern analysis
/// - Resource anomaly detection
/// - Behavioral deviation tracking
/// - Real-time Z-score analysis
pub struct AnomalyEngine {
    config: AnomalyConfig,
    agent_state: HashMap<String, AgentState>,
    anomalies: Vec<Anomaly>,
    global_metrics: GlobalMetrics,
}

/// Per-agent state for tracking
#[derive(Debug, Clone)]
struct AgentState {
    actions: VecDeque<String>,
    action_timestamps: VecDeque<chrono::DateTime<chrono::Utc>>,
    metrics: MetricHistory,
    profile: BehaviorProfile,
    last_activity: chrono::DateTime<chrono::Utc>,
}

/// Metric history for statistical analysis
#[derive(Debug, Clone, Default)]
struct MetricHistory {
    response_times: VecDeque<f64>,
    cpu_usage: VecDeque<f64>,
    memory_usage: VecDeque<f64>,
    action_counts: VecDeque<u64>,
}

/// Behavior profile for anomaly baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorProfile {
    pub agent_id: String,
    pub baseline_response_time_ms: f64,
    pub baseline_cpu_percent: f64,
    pub baseline_memory_percent: f64,
    pub typical_actions: Vec<String>,
    pub action_frequency: HashMap<String, f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for BehaviorProfile {
    fn default() -> Self {
        Self {
            agent_id: String::new(),
            baseline_response_time_ms: 100.0,
            baseline_cpu_percent: 20.0,
            baseline_memory_percent: 30.0,
            typical_actions: Vec::new(),
            action_frequency: HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

/// Global metrics across all agents
#[derive(Debug, Clone, Default)]
struct GlobalMetrics {
    total_actions: u64,
    total_anomalies: u64,
    active_agents: u64,
    system_load: f64,
}

impl AnomalyEngine {
    pub fn new(config: AnomalyConfig) -> Self {
        Self {
            config,
            agent_state: HashMap::new(),
            anomalies: Vec::new(),
            global_metrics: GlobalMetrics::default(),
        }
    }

    /// Register agent for monitoring
    pub fn register_agent(&mut self, agent_id: &str) {
        self.agent_state.insert(agent_id.to_string(), AgentState {
            actions: VecDeque::with_capacity(1000),
            action_timestamps: VecDeque::with_capacity(1000),
            metrics: MetricHistory::default(),
            profile: BehaviorProfile {
                agent_id: agent_id.to_string(),
                ..Default::default()
            },
            last_activity: chrono::Utc::now(),
        });
        self.global_metrics.active_agents += 1;
    }
    
    /// Unregister agent
    pub fn unregister_agent(&mut self, agent_id: &str) {
        if self.agent_state.remove(agent_id).is_some() {
            self.global_metrics.active_agents = self.global_metrics.active_agents.saturating_sub(1);
        }
    }

    /// Record agent action
    /// 
    /// Returns any detected anomalies
    pub fn record_action(&mut self, agent_id: &str, action: &str) -> Vec<Anomaly> {
        let mut detected = Vec::new();
        
        // First, update state
        let actions_snapshot = {
            if let Some(state) = self.agent_state.get_mut(agent_id) {
                // Add action to history
                state.actions.push_back(action.to_string());
                state.action_timestamps.push_back(chrono::Utc::now());
                state.last_activity = chrono::Utc::now();
                
                // Limit history size
                if state.actions.len() > 1000 {
                    state.actions.pop_front();
                    state.action_timestamps.pop_front();
                }
                
                // Update action frequency profile
                *state.profile.action_frequency.entry(action.to_string()).or_insert(0.0) += 1.0;
                
                // Clone actions for loop detection
                Some(state.actions.clone())
            } else {
                None
            }
        };
        
        // Check for loop patterns (after mutable borrow ends)
        if self.config.loop_detection {
            if let Some(actions) = actions_snapshot {
                if let Some(anomaly) = self.check_loop(agent_id, &actions) {
                    detected.push(anomaly);
                }
            }
        }
        
        self.global_metrics.total_actions += 1;
        self.anomalies.extend(detected.clone());
        detected
    }
    
    /// Record metrics for an agent
    pub fn record_metrics(
        &mut self,
        agent_id: &str,
        response_time_ms: f64,
        cpu_percent: f64,
        memory_percent: f64,
    ) -> Vec<Anomaly> {
        let mut detected = Vec::new();
        
        // Collect data and snapshot metrics
        let metrics_snapshot = {
            if let Some(state) = self.agent_state.get_mut(agent_id) {
                // Add to history
                state.metrics.response_times.push_back(response_time_ms);
                state.metrics.cpu_usage.push_back(cpu_percent);
                state.metrics.memory_usage.push_back(memory_percent);
                
                // Limit history
                if state.metrics.response_times.len() > self.config.window_size {
                    state.metrics.response_times.pop_front();
                    state.metrics.cpu_usage.pop_front();
                    state.metrics.memory_usage.pop_front();
                }
                
                Some(state.metrics.clone())
            } else {
                None
            }
        };
        
        // Check for resource anomalies (after mutable borrow ends)
        if self.config.resource_monitoring {
            if let Some(anomaly) = self.check_resource_anomaly(agent_id, cpu_percent, memory_percent) {
                detected.push(anomaly);
            }
            
            if let Some(ref metrics) = metrics_snapshot {
                if let Some(anomaly) = self.check_latency_anomaly(agent_id, response_time_ms, metrics) {
                    detected.push(anomaly);
                }
            }
        }
        
        self.anomalies.extend(detected.clone());
        detected
    }

    /// Check for loop patterns
    fn check_loop(&self, agent_id: &str, actions: &VecDeque<String>) -> Option<Anomaly> {
        let window = self.config.max_repetition as usize;
        
        if actions.len() < window * 2 {
            return None;
        }
        
        // Check for exact repetition
        let seq1: Vec<_> = actions.iter().rev().take(window).collect();
        let seq2: Vec<_> = actions.iter().rev().skip(window).take(window).collect();
        
        if seq1 == seq2 {
            return Some(Anomaly::new(
                AnomalyType::LoopPattern,
                AnomalySeverity::High,
                "action_repetition",
                actions.len() as f64,
                (0.0, self.config.max_repetition as f64),
                self.calculate_z_score(actions.len() as f64, 10.0, 5.0),
            ).with_agent(agent_id)
             .with_description(format!("Exact loop detected: {} actions repeated", window))
             .with_action("Break loop or increase max_repetition threshold"));
        }
        
        // Check for pattern loops (longer cycles)
        for pattern_len in 2..=window/2 {
            if actions.len() < pattern_len * 4 {
                continue;
            }
            
            let pattern: Vec<_> = actions.iter().rev().take(pattern_len).collect();
            let mut matches = 0;
            
            for i in (0..actions.len()).step_by(pattern_len) {
                let segment: Vec<_> = actions.iter().rev().skip(i).take(pattern_len).collect();
                if segment == pattern {
                    matches += 1;
                }
            }
            
            if matches >= 3 {
                return Some(Anomaly::new(
                    AnomalyType::LoopPattern,
                    AnomalySeverity::Warning,
                    "pattern_loop",
                    matches as f64,
                    (0.0, 2.0),
                    3.0,
                ).with_agent(agent_id)
                 .with_description(format!("Pattern loop detected: {:?}", pattern))
                 .with_action("Review agent logic for repeated patterns"));
            }
        }
        
        None
    }
    
    /// Check for resource anomalies
    fn check_resource_anomaly(
        &self,
        agent_id: &str,
        cpu_percent: f64,
        memory_percent: f64,
    ) -> Option<Anomaly> {
        if cpu_percent > self.config.max_cpu_percent {
            return Some(Anomaly::new(
                AnomalyType::CpuSpike,
                AnomalySeverity::High,
                "cpu_usage",
                cpu_percent,
                (0.0, self.config.max_cpu_percent),
                self.calculate_z_score(cpu_percent, 20.0, 10.0),
            ).with_agent(agent_id)
             .with_description(format!("CPU usage {}% exceeds limit {}%", cpu_percent, self.config.max_cpu_percent))
             .with_action("Check for runaway processes or optimize algorithm"));
        }
        
        if memory_percent > self.config.max_memory_percent {
            return Some(Anomaly::new(
                AnomalyType::MemoryLeak,
                AnomalySeverity::High,
                "memory_usage",
                memory_percent,
                (0.0, self.config.max_memory_percent),
                self.calculate_z_score(memory_percent, 30.0, 15.0),
            ).with_agent(agent_id)
             .with_description(format!("Memory usage {}% exceeds limit {}%", memory_percent, self.config.max_memory_percent))
             .with_action("Check for memory leaks or increase memory limit"));
        }
        
        None
    }
    
    /// Check for latency anomalies using statistical analysis
    fn check_latency_anomaly(
        &self,
        agent_id: &str,
        response_time: f64,
        metrics: &MetricHistory,
    ) -> Option<Anomaly> {
        if metrics.response_times.len() < self.config.min_samples {
            return None;
        }
        
        // Calculate mean and std deviation
        let mean: f64 = metrics.response_times.iter().sum::<f64>() / metrics.response_times.len() as f64;
        let variance: f64 = metrics.response_times.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / metrics.response_times.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev < 1.0 {
            return None; // Not enough variance to detect anomalies
        }
        
        let z_score = (response_time - mean) / std_dev;
        
        if z_score.abs() > self.config.z_threshold {
            let severity = if z_score.abs() > self.config.z_threshold * 2.0 {
                AnomalySeverity::Critical
            } else {
                AnomalySeverity::Warning
            };
            
            return Some(Anomaly::new(
                AnomalyType::LatencyAnomaly,
                severity,
                "response_time",
                response_time,
                (mean - std_dev * self.config.z_threshold, mean + std_dev * self.config.z_threshold),
                z_score,
            ).with_agent(agent_id)
             .with_description(format!("Response time {}ms is {} std devs from mean {}ms", 
                response_time, z_score.abs(), mean))
             .with_action("Investigate slow operations or optimize performance"));
        }
        
        None
    }
    
    /// Calculate Z-score for anomaly detection
    fn calculate_z_score(&self, value: f64, mean: f64, std_dev: f64) -> f64 {
        if std_dev < 0.001 {
            return 0.0;
        }
        (value - mean) / std_dev
    }
    
    /// Detect behavioral deviation from profile
    pub fn detect_behavioral_deviation(
        &mut self,
        agent_id: &str,
        action: &str,
    ) -> Option<Anomaly> {
        let state = self.agent_state.get(agent_id)?;
        
        // Check if action is unusual
        let total_actions: f64 = state.profile.action_frequency.values().sum();
        let action_freq = state.profile.action_frequency.get(action).copied().unwrap_or(0.0);
        let probability = if total_actions > 0.0 {
            action_freq / total_actions
        } else {
            1.0
        };
        
        // Low probability action might indicate deviation
        if probability < 0.05 && total_actions > 100.0 {
            return Some(Anomaly::new(
                AnomalyType::BehaviorDeviation,
                AnomalySeverity::Warning,
                "unusual_action",
                probability * 100.0,
                (5.0, 100.0), // Expected probability > 5%
                (0.05 - probability) / 0.05, // Z-score approximation
            ).with_agent(agent_id)
             .with_description(format!("Unusual action '{}' detected (probability: {:.2}%)", action, probability * 100.0))
             .with_action("Review if action is intentional or caused by error"));
        }
        
        None
    }
    
    /// Check for output anomalies (repeated outputs)
    pub fn check_output_anomaly(
        &mut self,
        agent_id: &str,
        output_hash: &str,
    ) -> Option<Anomaly> {
        // Track output hashes
        if let Some(state) = self.agent_state.get_mut(agent_id) {
            let recent_count = state.actions.iter()
                .filter(|a| a.contains(output_hash))
                .count();
            
            if recent_count > 5 {
                return Some(Anomaly::new(
                    AnomalyType::OutputAnomaly,
                    AnomalySeverity::Warning,
                    "repeated_output",
                    recent_count as f64,
                    (0.0, 5.0),
                    recent_count as f64 / 5.0,
                ).with_agent(agent_id)
                 .with_description("Agent producing identical outputs repeatedly")
                 .with_action("Check for stuck state or infinite loop"));
            }
        }
        
        None
    }

    /// Get detected anomalies
    pub fn anomalies(&self) -> &[Anomaly] {
        &self.anomalies
    }
    
    /// Get anomalies for specific agent
    pub fn agent_anomalies(&self, agent_id: &str) -> Vec<&Anomaly> {
        self.anomalies.iter()
            .filter(|a| a.agent_id.as_deref() == Some(agent_id))
            .collect()
    }
    
    /// Get anomalies by severity
    pub fn anomalies_by_severity(&self, severity: AnomalySeverity) -> Vec<&Anomaly> {
        self.anomalies.iter()
            .filter(|a| a.severity == severity)
            .collect()
    }

    /// Clear anomalies
    pub fn clear(&mut self) {
        self.anomalies.clear();
    }
    
    /// Clear old anomalies (older than duration)
    pub fn clear_old(&mut self, older_than: chrono::Duration) {
        let cutoff = chrono::Utc::now() - older_than;
        self.anomalies.retain(|a| a.timestamp > cutoff);
    }
    
    /// Get engine statistics
    pub fn stats(&self) -> EngineStats {
        EngineStats {
            monitored_agents: self.agent_state.len(),
            total_actions: self.global_metrics.total_actions,
            total_anomalies: self.anomalies.len() as u64,
            critical_anomalies: self.anomalies.iter().filter(|a| a.severity == AnomalySeverity::Critical).count(),
            high_anomalies: self.anomalies.iter().filter(|a| a.severity == AnomalySeverity::High).count(),
        }
    }
    
    /// Update behavior profile for an agent
    pub fn update_profile(&mut self, agent_id: &str) {
        if let Some(state) = self.agent_state.get_mut(agent_id) {
            if !state.metrics.response_times.is_empty() {
                state.profile.baseline_response_time_ms = 
                    state.metrics.response_times.iter().sum::<f64>() / state.metrics.response_times.len() as f64;
            }
            if !state.metrics.cpu_usage.is_empty() {
                state.profile.baseline_cpu_percent = 
                    state.metrics.cpu_usage.iter().sum::<f64>() / state.metrics.cpu_usage.len() as f64;
            }
            if !state.metrics.memory_usage.is_empty() {
                state.profile.baseline_memory_percent = 
                    state.metrics.memory_usage.iter().sum::<f64>() / state.metrics.memory_usage.len() as f64;
            }
            state.profile.updated_at = chrono::Utc::now();
        }
    }
}

impl Default for AnomalyEngine {
    fn default() -> Self {
        Self::new(AnomalyConfig::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStats {
    pub monitored_agents: usize,
    pub total_actions: u64,
    pub total_anomalies: u64,
    pub critical_anomalies: usize,
    pub high_anomalies: usize,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANOMALY HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

impl Anomaly {
    pub fn with_agent(mut self, agent_id: &str) -> Self {
        self.agent_id = Some(agent_id.to_string());
        self
    }
    
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.recommended_action = Some(action.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_registration() {
        let mut engine = AnomalyEngine::new(AnomalyConfig::default());
        engine.register_agent("test_agent");
        assert!(engine.agent_state.contains_key("test_agent"));
    }

    #[test]
    fn test_loop_detection() {
        let mut engine = AnomalyEngine::new(AnomalyConfig::default());
        engine.register_agent("test_agent");
        
        // Create a loop pattern
        for _ in 0..2 {
            engine.record_action("test_agent", "action1");
            engine.record_action("test_agent", "action2");
            engine.record_action("test_agent", "action3");
            engine.record_action("test_agent", "action4");
            engine.record_action("test_agent", "action5");
        }
        
        assert!(!engine.anomalies().is_empty());
        assert!(engine.anomalies().iter().any(|a| matches!(a.anomaly_type, AnomalyType::LoopPattern)));
    }
    
    #[test]
    fn test_resource_anomaly() {
        let mut engine = AnomalyEngine::new(AnomalyConfig {
            max_cpu_percent: 80.0,
            max_memory_percent: 80.0,
            ..Default::default()
        });
        engine.register_agent("test_agent");
        
        let anomalies = engine.record_metrics("test_agent", 100.0, 95.0, 50.0);
        assert!(anomalies.iter().any(|a| matches!(a.anomaly_type, AnomalyType::CpuSpike)));
    }
    
    #[test]
    fn test_latency_anomaly() {
        let mut engine = AnomalyEngine::new(AnomalyConfig {
            z_threshold: 2.0,
            min_samples: 5,
            ..Default::default()
        });
        engine.register_agent("test_agent");
        
        // Build baseline
        for _ in 0..10 {
            engine.record_metrics("test_agent", 100.0, 20.0, 30.0);
        }
        
        // Add anomaly
        let anomalies = engine.record_metrics("test_agent", 500.0, 20.0, 30.0);
        assert!(anomalies.iter().any(|a| matches!(a.anomaly_type, AnomalyType::LatencyAnomaly)));
    }
    
    #[test]
    fn test_stats() {
        let mut engine = AnomalyEngine::new(AnomalyConfig::default());
        engine.register_agent("agent1");
        engine.register_agent("agent2");
        engine.record_action("agent1", "test");
        
        let stats = engine.stats();
        assert_eq!(stats.monitored_agents, 2);
        assert_eq!(stats.total_actions, 1);
    }
}
