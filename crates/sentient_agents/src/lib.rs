//! SENTIENT Multi-Agent Orchestration Module
//! 
//! Integrates CrewAI, AutoGen, Swarm, MetaGPT patterns into SENTIENT
//! 
//! ## Supported Frameworks:
//! - **CrewAI**: Role-based multi-agent collaboration
//! - **AutoGen**: Conversation-based multi-agent (Microsoft)
//! - **Swarm**: Lightweight orchestration (OpenAI)
//! - **MetaGPT**: Company-like agent organization
//! - **Agent-S**: Desktop automation agents

use serde::{Deserialize, Serialize};
use tracing::info;

pub mod error;
pub mod orchestrator;
pub mod agents;

// Re-exports
pub use error::{AgentError, AgentResult};
pub use orchestrator::{AgentOrchestrator, TaskResult, OrchestratorStatus};
pub use agents::{Agent, AgentRole, AgentState, Crew, ConversationGroup, AgentHandoff};

/// Multi-Agent Framework Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentFramework {
    /// CrewAI - Role-based agents
    CrewAI,
    /// AutoGen - Microsoft conversation agents
    AutoGen,
    /// Swarm - OpenAI lightweight orchestration
    Swarm,
    /// MetaGPT - Company-style organization
    MetaGPT,
    /// Agent-S - Desktop automation
    AgentS,
    /// Custom SENTIENT native agents
    SENTIENTNative,
}

/// Agent Task Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Unique task ID
    pub id: String,
    /// Task description
    pub description: String,
    /// Expected output format
    pub expected_output: String,
    /// Assigned agent
    pub assigned_agent: Option<String>,
    /// Task priority (1-10)
    pub priority: u8,
    /// Dependencies (other task IDs)
    pub dependencies: Vec<String>,
    /// Task status
    pub status: TaskStatus,
}

/// Task Execution Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Multi-Agent Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAgentConfig {
    /// Framework to use
    pub framework: AgentFramework,
    /// Maximum concurrent agents
    pub max_agents: usize,
    /// Task timeout (seconds)
    pub timeout_secs: u64,
    /// Enable verbose logging
    pub verbose: bool,
    /// Memory sharing between agents
    pub shared_memory: bool,
    /// V-GATE endpoint for LLM calls
    pub vgate_endpoint: String,
}

impl Default for MultiAgentConfig {
    fn default() -> Self {
        Self {
            framework: AgentFramework::SENTIENTNative,
            max_agents: 5,
            timeout_secs: 300,
            verbose: false,
            shared_memory: true,
            vgate_endpoint: "http://localhost:8080/vgate".to_string(),
        }
    }
}

/// Initialize the multi-agent system
pub async fn initialize(config: MultiAgentConfig) -> AgentResult<AgentOrchestrator> {
    info!("🤖 Initializing SENTIENT Multi-Agent System with {:?}", config.framework);
    
    let orchestrator = AgentOrchestrator::new(config.clone());
    
    // Log framework patterns
    match config.framework {
        AgentFramework::CrewAI => {
            info!("Loading CrewAI patterns from integrations/agents/crewai");
        }
        AgentFramework::AutoGen => {
            info!("Loading AutoGen patterns from integrations/agents/autogen");
        }
        AgentFramework::Swarm => {
            info!("Loading Swarm patterns from integrations/agents/swarm");
        }
        AgentFramework::MetaGPT => {
            info!("Loading MetaGPT patterns from integrations/agents/metagpt");
        }
        AgentFramework::AgentS => {
            info!("Loading Agent-S patterns from integrations/agents/agent-s");
        }
        AgentFramework::SENTIENTNative => {
            info!("Using native SENTIENT agent orchestration");
        }
    }
    
    Ok(orchestrator)
}

/// Available frameworks from integrations
pub fn available_frameworks() -> Vec<AgentFrameworkInfo> {
    vec![
        AgentFrameworkInfo {
            framework: AgentFramework::CrewAI,
            name: "CrewAI".to_string(),
            description: "Role-based multi-agent collaboration".to_string(),
            source: "integrations/agents/crewai".to_string(),
            status: "READY".to_string(),
        },
        AgentFrameworkInfo {
            framework: AgentFramework::AutoGen,
            name: "AutoGen".to_string(),
            description: "Microsoft conversation-based agents".to_string(),
            source: "integrations/agents/autogen".to_string(),
            status: "READY".to_string(),
        },
        AgentFrameworkInfo {
            framework: AgentFramework::Swarm,
            name: "Swarm".to_string(),
            description: "OpenAI lightweight orchestration".to_string(),
            source: "integrations/agents/swarm".to_string(),
            status: "READY".to_string(),
        },
        AgentFrameworkInfo {
            framework: AgentFramework::MetaGPT,
            name: "MetaGPT".to_string(),
            description: "Company-style agent organization".to_string(),
            source: "integrations/agents/metagpt".to_string(),
            status: "READY".to_string(),
        },
        AgentFrameworkInfo {
            framework: AgentFramework::AgentS,
            name: "Agent-S".to_string(),
            description: "Desktop automation agents".to_string(),
            source: "integrations/agents/agent-s".to_string(),
            status: "READY".to_string(),
        },
        AgentFrameworkInfo {
            framework: AgentFramework::SENTIENTNative,
            name: "SENTIENT Native".to_string(),
            description: "Built-in SENTIENT orchestration".to_string(),
            source: "crates/sentient_agents".to_string(),
            status: "ACTIVE".to_string(),
        },
    ]
}

/// Framework Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFrameworkInfo {
    pub framework: AgentFramework,
    pub name: String,
    pub description: String,
    pub source: String,
    pub status: String,
}

/// CrewAI Integration Helper
pub mod crewai {
    use crate::agents::{Agent, AgentRole, Crew};

    /// Create a research crew
    pub fn create_research_crew() -> Crew {
        let researcher = Agent::new(
            "Researcher",
            AgentRole::Researcher,
            "Find and compile information on the given topic"
        ).with_backstory("Expert researcher with attention to detail");
        
        let writer = Agent::new(
            "Writer",
            AgentRole::Writer,
            "Create engaging content based on research findings"
        ).with_backstory("Professional writer with SEO expertise");
        
        Crew {
            name: "Research & Writing Crew".to_string(),
            agents: vec![researcher, writer],
            tasks: vec![],
            verbose: true,
        }
    }
}

/// AutoGen Integration Helper
pub mod autogen {
    use crate::agents::{Agent, ConversationGroup};

    /// Create a group chat
    pub fn create_group_chat(name: &str, agents: Vec<Agent>) -> ConversationGroup {
        ConversationGroup {
            name: name.to_string(),
            agents,
            max_rounds: 10,
        }
    }
}

/// Swarm Integration Helper  
pub mod swarm {
    use crate::agents::AgentHandoff;

    /// Create a simple agent handoff
    pub fn create_handoff(from: &str, to: &str, context: &str) -> AgentHandoff {
        AgentHandoff {
            from_agent: from.to_string(),
            to_agent: to.to_string(),
            context: context.to_string(),
        }
    }
}

/// MetaGPT Integration Helper
pub mod metagpt {
    use crate::agents::{Agent, AgentRole};

    /// Create software team
    pub fn create_software_team() -> Vec<Agent> {
        vec![
            Agent::new(
                "Product Manager",
                AgentRole::Coordinator,
                "Analyze requirements and create PRD"
            ),
            Agent::new(
                "Architect",
                AgentRole::Analyst,
                "Design system architecture"
            ),
            Agent::new(
                "Engineer",
                AgentRole::Developer,
                "Implement the system"
            ),
            Agent::new(
                "QA Engineer",
                AgentRole::Reviewer,
                "Test and validate the system"
            ),
        ]
    }
}
