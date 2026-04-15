//! Observability module for SENTIENT
//!
//! Provides distributed tracing, metrics, and logging.

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod tracing_setup;
pub mod metrics;
pub mod logging;
pub mod health;
pub mod spans;
pub mod telemetry;

pub use tracing_setup::*;
pub use metrics::*;
pub use health::*;
pub use telemetry::*;
