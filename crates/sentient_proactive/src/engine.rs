//! ─── Proactive Engine Core ───
//!
//! Main engine coordinating all proactive subsystems

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::scheduler::Scheduler;
use crate::event::EventBus;
use crate::pattern::PatternMatcher;
use crate::action::ActionExecutor;
use crate::trigger::Trigger;

/// Main proactive engine
pub struct ProactiveEngine {
    scheduler: Scheduler,
    event_bus: EventBus,
    pattern_matcher: Arc<RwLock<PatternMatcher>>,
    action_executor: ActionExecutor,
    config: EngineConfig,
    stats: Arc<RwLock<EngineStats>>,
    running: Arc<RwLock<bool>>,
}

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Enable time-based triggers
    pub enable_time_triggers: bool,
    
    /// Enable event-based triggers
    pub enable_event_triggers: bool,
    
    /// Enable pattern matching
    pub enable_patterns: bool,
    
    /// Check interval in seconds
    pub check_interval_seconds: u64,
    
    /// Maximum concurrent actions
    pub max_concurrent_actions: usize,
    
    /// Enable notifications
    pub enable_notifications: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            enable_time_triggers: true,
            enable_event_triggers: true,
            enable_patterns: true,
            check_interval_seconds: 60,
            max_concurrent_actions: 5,
            enable_notifications: true,
        }
    }
}

/// Engine statistics
#[derive(Debug, Clone, Default)]
pub struct EngineStats {
    pub triggers_fired: u64,
    pub actions_executed: u64,
    pub patterns_matched: u64,
    pub errors: u64,
    pub uptime_seconds: u64,
}

impl ProactiveEngine {
    /// Create new proactive engine
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
            event_bus: EventBus::new(),
            pattern_matcher: Arc::new(RwLock::new(PatternMatcher::new())),
            action_executor: ActionExecutor::new(),
            config: EngineConfig::default(),
            stats: Arc::new(RwLock::new(EngineStats::default())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Create with custom config
    pub fn with_config(config: EngineConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }
    
    /// Add a trigger
    pub async fn add_trigger(&self, trigger: Trigger) {
        self.scheduler.add_trigger(trigger).await;
    }
    
    /// Publish an event
    pub async fn publish_event(&self, event: crate::event::Event) {
        self.event_bus.publish(event).await;
    }
    
    /// Add a pattern
    pub async fn add_pattern(&self, pattern: crate::pattern::Pattern) {
        let mut matcher = self.pattern_matcher.write().await;
        matcher.add_pattern(pattern);
    }
    
    /// Get statistics
    pub async fn stats(&self) -> EngineStats {
        self.stats.read().await.clone()
    }
    
    /// Start the engine
    pub async fn start(&self) {
        let mut running = self.running.write().await;
        if *running {
            return;
        }
        *running = true;
        drop(running);
        
        // Start scheduler
        self.scheduler.start().await;
        
        // Start main loop
        let stats = self.stats.clone();
        let running = self.running.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let start = std::time::Instant::now();
            
            loop {
                let is_running = *running.read().await;
                if !is_running {
                    break;
                }
                
                // Update uptime
                let mut stats_guard = stats.write().await;
                stats_guard.uptime_seconds = start.elapsed().as_secs();
                drop(stats_guard);
                
                tokio::time::sleep(
                    tokio::time::Duration::from_secs(config.check_interval_seconds)
                ).await;
            }
        });
        
        tracing::info!("Proactive Engine started");
    }
    
    /// Stop the engine
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        tracing::info!("Proactive Engine stopped");
    }
    
    /// Check if engine is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Default for ProactiveEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined proactive scenarios
pub struct ProactiveScenarios;

impl ProactiveScenarios {
    /// Morning briefing trigger
    pub fn morning_briefing() -> Trigger {
        Trigger::new(
            "morning-briefing",
            "Morning Briefing",
            crate::trigger::TriggerType::weekdays("08:30"),
        )
        .with_action("generate_briefing")
        .with_description("Generate daily morning briefing")
    }
    
    /// Email check trigger
    pub fn email_check() -> Trigger {
        Trigger::new(
            "email-check",
            "Email Monitor",
            crate::trigger::TriggerType::Interval { seconds: 300 },
        )
        .with_action("summarize_emails")
        .with_description("Check and summarize new emails")
    }
    
    /// Friday report trigger
    pub fn friday_report() -> Trigger {
        Trigger::new(
            "friday-report",
            "Weekly Report",
            crate::trigger::TriggerType::Cron {
                expression: "0 17 * * 5".into(), // Friday 17:00
            },
        )
        .with_action("weekly_report")
        .with_description("Generate weekly summary report")
    }
    
    /// Calendar reminder trigger
    pub fn calendar_reminder() -> Trigger {
        Trigger::new(
            "calendar-check",
            "Calendar Check",
            crate::trigger::TriggerType::Interval { seconds: 600 },
        )
        .with_action("check_calendar")
        .with_description("Check upcoming calendar events")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_engine_creation() {
        let engine = ProactiveEngine::new();
        assert!(!engine.is_running().await);
    }
    
    #[test]
    fn test_scenarios() {
        let briefing = ProactiveScenarios::morning_briefing();
        assert_eq!(briefing.id, "morning-briefing");
        
        let report = ProactiveScenarios::friday_report();
        assert_eq!(report.id, "friday-report");
    }
}
