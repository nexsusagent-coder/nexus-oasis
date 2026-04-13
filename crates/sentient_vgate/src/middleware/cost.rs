//! ═════════════════════════════════════════════════════════════════
//!  COST TRACKING MODULE - API Maliyet Takibi
//! ═════════════════════════════════════════════════════════════════
//!
//! Her provider ve model için maliyet takibi.
//! Günlük/aylık bütçe sınırları.

use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

/// Model fiyat bilgisi (1K token başına USD cent)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelPricing {
    pub model_id: String,
    pub provider: String,
    pub prompt_cost_per_1k: f64,   // cent / 1K token
    pub completion_cost_per_1k: f64,
}

/// Maliyet kaydı
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CostEntry {
    pub model: String,
    pub provider: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub cost_cents: f64,
    pub timestamp: DateTime<Utc>,
}

/// Bütçe yapılandırması
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BudgetConfig {
    /// Günlük bütçe (USD cent)
    pub daily_budget_cents: f64,
    /// Aylık bütçe (USD cent)
    pub monthly_budget_cents: f64,
    /// Bütçe aşıldığında uyarı ver
    pub warn_threshold_pct: f64,
    /// Bütçe aşıldığında istekleri reddet
    pub hard_limit: bool,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            daily_budget_cents: 1000.0,  // $10/gün
            monthly_budget_cents: 20000.0, // $200/ay
            warn_threshold_pct: 80.0,
            hard_limit: true,
        }
    }
}

/// Maliyet takip yöneticisi
pub struct CostTracker {
    pricing: HashMap<String, ModelPricing>,
    entries: Mutex<Vec<CostEntry>>,
    config: BudgetConfig,
}

impl CostTracker {
    pub fn new(config: BudgetConfig) -> Self {
        let mut tracker = Self {
            pricing: HashMap::new(),
            entries: Mutex::new(Vec::new()),
            config,
        };
        tracker.register_default_pricing();
        tracker
    }

    pub fn default_tracker() -> Self {
        Self::new(BudgetConfig::default())
    }

    /// Varsayılan model fiyatlarını kaydet
    fn register_default_pricing(&mut self) {
        let prices = [
            ("gpt-4-turbo", "openai", 1.0, 3.0),
            ("gpt-4", "openai", 3.0, 6.0),
            ("gpt-3.5-turbo", "openai", 0.05, 0.15),
            ("claude-3-opus", "anthropic", 1.5, 7.5),
            ("claude-3-sonnet", "anthropic", 0.3, 1.5),
            ("claude-3-haiku", "anthropic", 0.025, 0.125),
            ("llama-3-70b", "groq", 0.07, 0.07),
            ("mixtral-8x7b", "groq", 0.027, 0.027),
            ("gemini-pro", "google", 0.125, 0.375),
            ("gemini-2.0-flash", "google", 0.018, 0.07),
        ];

        for (model, provider, prompt, completion) in prices {
            self.pricing.insert(model.into(), ModelPricing {
                model_id: model.into(),
                provider: provider.into(),
                prompt_cost_per_1k: prompt,
                completion_cost_per_1k: completion,
            });
        }
    }

    /// Model fiyatı kaydet/güncelle
    pub fn set_pricing(&mut self, pricing: ModelPricing) {
        self.pricing.insert(pricing.model_id.clone(), pricing);
    }

    /// Maliyet hesapla ve kaydet
    pub fn record_usage(&self, model: &str, prompt_tokens: u64, completion_tokens: u64) -> f64 {
        let pricing = self.pricing.get(model);
        let cost_cents = if let Some(p) = pricing {
            (prompt_tokens as f64 / 1000.0 * p.prompt_cost_per_1k)
                + (completion_tokens as f64 / 1000.0 * p.completion_cost_per_1k)
        } else {
            // Bilinmeyen model: ortalama $0.50/1K token
            ((prompt_tokens + completion_tokens) as f64 / 1000.0) * 0.5
        };

        let provider = pricing.map(|p| p.provider.as_str()).unwrap_or("unknown");

        let entry = CostEntry {
            model: model.into(),
            provider: provider.into(),
            prompt_tokens,
            completion_tokens,
            cost_cents,
            timestamp: Utc::now(),
        };

        self.entries.lock().unwrap().push(entry);
        cost_cents
    }

    /// Bütçe kontrolü: istek izin veriliyor mu?
    pub fn check_budget(&self) -> BudgetStatus {
        let entries = self.entries.lock().unwrap();
        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();

        let daily_spent: f64 = entries.iter()
            .filter(|e| e.timestamp.naive_utc() >= today_start)
            .map(|e| e.cost_cents)
            .sum();

        let month_start = {
            let month_date = chrono::NaiveDate::parse_from_str(
                &format!("{}-{}-01", now.format("%Y"), now.format("%m")),
                "%Y-%m-%d"
            ).unwrap_or(now.date_naive());
            month_date.and_hms_opt(0, 0, 0)
                .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, *now.offset()))
                .unwrap_or(now)
        };
        let monthly_spent: f64 = entries.iter()
            .filter(|e| e.timestamp >= month_start)
            .map(|e| e.cost_cents)
            .sum();

        let daily_pct = (daily_spent / self.config.daily_budget_cents) * 100.0;
        let monthly_pct = (monthly_spent / self.config.monthly_budget_cents) * 100.0;

        BudgetStatus {
            daily_spent_cents: daily_spent,
            monthly_spent_cents: monthly_spent,
            daily_budget_cents: self.config.daily_budget_cents,
            monthly_budget_cents: self.config.monthly_budget_cents,
            daily_pct,
            monthly_pct,
            over_budget: daily_spent > self.config.daily_budget_cents ||
                        monthly_spent > self.config.monthly_budget_cents,
            should_block: self.config.hard_limit && 
                         (daily_spent > self.config.daily_budget_cents ||
                          monthly_spent > self.config.monthly_budget_cents),
        }
    }

    /// Model bazlı maliyet özeti
    pub fn cost_by_model(&self) -> HashMap<String, f64> {
        let entries = self.entries.lock().unwrap();
        let mut costs: HashMap<String, f64> = HashMap::new();
        for entry in entries.iter() {
            *costs.entry(entry.model.clone()).or_insert(0.0) += entry.cost_cents;
        }
        costs
    }

    /// Tüm kayıtları temizle
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

/// Bütçe durumu
#[derive(Debug, Clone, serde::Serialize)]
pub struct BudgetStatus {
    pub daily_spent_cents: f64,
    pub monthly_spent_cents: f64,
    pub daily_budget_cents: f64,
    pub monthly_budget_cents: f64,
    pub daily_pct: f64,
    pub monthly_pct: f64,
    pub over_budget: bool,
    pub should_block: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_usage() {
        let tracker = CostTracker::default_tracker();
        let cost = tracker.record_usage("gpt-4", 1000, 500);
        assert!(cost > 0.0);
    }

    #[test]
    fn test_unknown_model() {
        let tracker = CostTracker::default_tracker();
        let cost = tracker.record_usage("unknown-model", 100, 100);
        assert!(cost > 0.0);
    }

    #[test]
    fn test_budget_check() {
        let tracker = CostTracker::default_tracker();
        let status = tracker.check_budget();
        assert!(!status.over_budget);
        assert!(!status.should_block);
    }

    #[test]
    fn test_cost_by_model() {
        let tracker = CostTracker::default_tracker();
        tracker.record_usage("gpt-4", 1000, 500);
        tracker.record_usage("gpt-4", 500, 250);
        tracker.record_usage("claude-3-sonnet", 1000, 1000);
        let costs = tracker.cost_by_model();
        assert!(costs.contains_key("gpt-4"));
        assert!(costs.contains_key("claude-3-sonnet"));
    }
}
