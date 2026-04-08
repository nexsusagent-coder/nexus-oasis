//! Agent Profile

use serde::{Deserialize, Serialize};

/// Agent behavior profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub agent_id: String,
    pub action_count: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl AgentProfile {
    pub fn new(agent_id: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            agent_id: agent_id.to_string(),
            action_count: 0,
            created_at: now,
            last_activity: now,
        }
    }

    pub fn record_action(&mut self) {
        self.action_count += 1;
        self.last_activity = chrono::Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile() {
        let mut profile = AgentProfile::new("test");
        profile.record_action();
        assert_eq!(profile.action_count, 1);
    }
}
