//! Digest engine - main orchestrator

use crate::{
    Digest, DigestConfig, DigestError, DigestResult, DigestSection,
    SectionType, CollectorRegistry, Composer,
};
use std::time::Instant;

/// Main digest engine
pub struct DigestEngine {
    config: DigestConfig,
    collectors: CollectorRegistry,
    composer: Composer,
}

impl DigestEngine {
    pub fn new(config: DigestConfig) -> Self {
        Self {
            config,
            collectors: CollectorRegistry::new(),
            composer: Composer::new(),
        }
    }

    pub fn with_collector<C: crate::Collector + 'static>(mut self, collector: C) -> Self {
        self.collectors.register(Box::new(collector));
        self
    }

    /// Generate a complete digest
    pub async fn generate(&self) -> DigestResult<Digest> {
        let start = Instant::now();
        
        // Create digest
        let mut digest = Digest::new(
            &self.format_title(),
            &self.config.language,
        );

        // Collect data for each section
        let mut sources_used = Vec::new();
        
        for section_type in &self.config.include_sections {
            if self.config.exclude_sections.contains(section_type) {
                continue;
            }

            match self.collect_section(section_type).await {
                Ok(Some(section)) => {
                    if !section.items.is_empty() || !section.content.is_empty() {
                        digest = digest.with_section(section);
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    log::warn!("Failed to collect {:?}: {}", section_type, e);
                }
            }
        }

        // Build full text
        digest.build_full_text();

        // Update metadata
        digest.metadata.assistant_name = self.config.assistant_name.clone();
        digest.metadata.user_name = self.config.user_name.clone();
        digest.metadata.location = self.config.location.clone();
        digest.metadata.timezone = self.config.timezone.clone();
        digest.metadata.total_items = digest.sections.iter().map(|s| s.items.len()).sum();
        digest.metadata.generation_time_ms = start.elapsed().as_millis() as u64;
        digest.metadata.sources_used = sources_used;

        Ok(digest)
    }

    async fn collect_section(&self, section_type: &SectionType) -> DigestResult<Option<DigestSection>> {
        self.collectors.collect(section_type, &self.config).await
    }

    fn format_title(&self) -> String {
        let greeting = match self.config.time_of_day {
            crate::TimeOfDay::Morning => "Günaydın",
            crate::TimeOfDay::Afternoon => "Tünaydın",
            crate::TimeOfDay::Evening => "İyi akşamlar",
        };

        if let Some(ref user) = self.config.user_name {
            format!("{} {}, işte bugünkü özetiniz", greeting, user)
        } else {
            format!("{}! İşte bugünkü özetiniz", greeting)
        }
    }

    pub fn config(&self) -> &DigestConfig {
        &self.config
    }
}
