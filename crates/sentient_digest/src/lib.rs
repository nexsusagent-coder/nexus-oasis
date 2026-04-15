//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Digest - Morning Briefing System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Sprint 3: Personal AI - Günlük kişiselleştirilmiş bülten sistemi
//!  
//!  Özellikler:
//!  - Hava durumu, haberler, takvim, email özeti
//!  - LLM ile özetleme
//!  - Çoklu dil desteği
//!  - Sesli bülten (TTS)
//!  - Zamanlanmış gönderim

pub mod types;
pub mod error;
pub mod engine;
pub mod collector;
pub mod composer;
pub mod scheduler;
pub mod sections;
pub mod templates;

pub use types::*;
pub use error::{DigestError, DigestResult};
pub use engine::DigestEngine;
pub use collector::{Collector, CollectorRegistry, WeatherCollector, CalendarCollector, EmailCollector, NewsCollector, GreetingCollector, TasksCollector};
pub use composer::Composer;
pub use scheduler::{DigestScheduler, ScheduleConfig};
pub use sections::Section;
pub use templates::{DigestTemplate, TemplateRegistry};
