//! Agent Definitions and Types
//!
//! Core agent types used across all frameworks

use serde::{Deserialize, Serialize};

/// Agent Role Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentRole {
    /// Research agent - searches and gathers information
    Researcher,
    /// Writer agent - creates content
    Writer,
    /// Reviewer agent - quality control
    Reviewer,
    /// Developer agent - writes code
    Developer,
    /// Analyst agent - analyzes data
    Analyst,
    /// Coordinator agent - manages other agents
    Coordinator,
    /// Custom role with name
    Custom(String),
}

/// Agent State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentState {
    Idle,
    Working,
    WaitingForInput,
    Completed,
    Error,
}

/// Agent Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent role
    pub role: AgentRole,
    /// Agent goal/purpose
    pub goal: String,
    /// Agent backstory (for CrewAI style)
    pub backstory: Option<String>,
    /// Current state
    pub state: AgentState,
    /// Tools available to this agent
    pub tools: Vec<String>,
    /// LLM model to use
    pub model: String,
}

impl Agent {
    pub fn new(name: &str, role: AgentRole, goal: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            role,
            goal: goal.to_string(),
            backstory: None,
            state: AgentState::Idle,
            tools: vec![],
            model: "vgate://default".to_string(),
        }
    }
    
    pub fn with_backstory(mut self, backstory: &str) -> Self {
        self.backstory = Some(backstory.to_string());
        self
    }
    
    pub fn with_tools(mut self, tools: Vec<&str>) -> Self {
        self.tools = tools.iter().map(|s| s.to_string()).collect();
        self
    }
}

/// CrewAI-style Crew (group of agents)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crew {
    pub name: String,
    pub agents: Vec<Agent>,
    pub tasks: Vec<String>,
    pub verbose: bool,
}

/// AutoGen-style Conversation Group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationGroup {
    pub name: String,
    pub agents: Vec<Agent>,
    pub max_rounds: u32,
}

/// Swarm-style Agent Handoff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandoff {
    pub from_agent: String,
    pub to_agent: String,
    pub context: String,
}
