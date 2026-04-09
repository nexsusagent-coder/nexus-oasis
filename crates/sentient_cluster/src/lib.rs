//! ─── SENTIENT Kubernetes Operator ───
//!
//! Deploy and manage distributed SENTIENT agents on Kubernetes.
//!
//! Features:
//! - Custom Resource Definitions (CRDs)
//! - Auto-scaling based on load
//! - Distributed task execution
//! - Multi-agent orchestration
//! - Health monitoring
//! - Metrics export
//!
//! Usage:
//! ```rust
//! // Apply CRDs
//! sentient_cluster::crds::apply_all().await?;
//!
//! // Create agent deployment
//! let agent = AgentDeployment::new("sentient-worker")
//!     .with_replicas(3)
//!     .with_resources("1Gi", "1")
//!     .with_channels(vec!["telegram", "discord"]);
//! 
//! agent.deploy().await?;
//! ```

pub mod crds;
pub mod operator;
pub mod agent;
pub mod task;
pub mod metrics;
pub mod health;

pub use crds::{SentientAgent, SentientTask};
pub use operator::Operator;
pub use agent::AgentDeployment;
pub use task::TaskDispatcher;
