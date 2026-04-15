//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT LLM Cost Tracker - Usage & Cost Analytics
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Comprehensive cost tracking for LLM usage:
//!  - Per-model pricing (input/output tokens)
//!  - Usage analytics by provider, model, app
//!  - Budget limits and alerts
//!  - Cost optimization recommendations
//!  - Historical cost trends

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};

// ═══════════════════════════════════════════════════════════════════════════════
//  PRICING DATA
// ═══════════════════════════════════════════════════════════════════════════════

/// Model pricing information (per 1M tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    /// Model identifier
    pub model: String,
    /// Provider name
    pub provider: String,
    /// Input token price (per 1M tokens)
    pub input_price_per_million: f64,
    /// Output token price (per 1M tokens)
    pub output_price_per_million: f64,
    /// Cache read price (if supported)
    pub cache_read_price_per_million: Option<f64>,
    /// Cache write price (if supported)
    pub cache_write_price_per_million: Option<f64>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

impl ModelPricing {
    /// Calculate cost for tokens used
    pub fn calculate_cost(&self, input_tokens: u64, output_tokens: u64) -> f64 {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * self.input_price_per_million;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * self.output_price_per_million;
        input_cost + output_cost
    }
    
    /// Calculate cost with caching
    pub fn calculate_cost_with_cache(
        &self,
        input_tokens: u64,
        output_tokens: u64,
        cache_read_tokens: u64,
        cache_write_tokens: u64,
    ) -> f64 {
        let base_cost = self.calculate_cost(input_tokens, output_tokens);
        
        let cache_read_cost = self.cache_read_price_per_million.map(|p| {
            (cache_read_tokens as f64 / 1_000_000.0) * p
        }).unwrap_or(0.0);
        
        let cache_write_cost = self.cache_write_price_per_million.map(|p| {
            (cache_write_tokens as f64 / 1_000_000.0) * p
        }).unwrap_or(0.0);
        
        base_cost + cache_read_cost + cache_write_cost
    }
}

/// Default pricing for common models (as of 2026)
pub fn get_default_pricing() -> Vec<ModelPricing> {
    let now = Utc::now();
    vec![
        // OpenAI
        ModelPricing {
            model: "gpt-4o".into(),
            provider: "openai".into(),
            input_price_per_million: 2.50,
            output_price_per_million: 10.00,
            cache_read_price_per_million: Some(1.25),
            cache_write_price_per_million: Some(3.75),
            updated_at: now,
        },
        ModelPricing {
            model: "gpt-4o-mini".into(),
            provider: "openai".into(),
            input_price_per_million: 0.15,
            output_price_per_million: 0.60,
            cache_read_price_per_million: Some(0.075),
            cache_write_price_per_million: Some(0.30),
            updated_at: now,
        },
        ModelPricing {
            model: "gpt-4-turbo".into(),
            provider: "openai".into(),
            input_price_per_million: 10.00,
            output_price_per_million: 30.00,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        ModelPricing {
            model: "o1".into(),
            provider: "openai".into(),
            input_price_per_million: 15.00,
            output_price_per_million: 60.00,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        ModelPricing {
            model: "o3-mini".into(),
            provider: "openai".into(),
            input_price_per_million: 1.10,
            output_price_per_million: 4.40,
            cache_read_price_per_million: Some(0.55),
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // Anthropic
        ModelPricing {
            model: "claude-4-opus".into(),
            provider: "anthropic".into(),
            input_price_per_million: 15.00,
            output_price_per_million: 75.00,
            cache_read_price_per_million: Some(1.50),
            cache_write_price_per_million: Some(18.75),
            updated_at: now,
        },
        ModelPricing {
            model: "claude-4-sonnet".into(),
            provider: "anthropic".into(),
            input_price_per_million: 3.00,
            output_price_per_million: 15.00,
            cache_read_price_per_million: Some(0.30),
            cache_write_price_per_million: Some(3.75),
            updated_at: now,
        },
        ModelPricing {
            model: "claude-3.5-sonnet".into(),
            provider: "anthropic".into(),
            input_price_per_million: 3.00,
            output_price_per_million: 15.00,
            cache_read_price_per_million: Some(0.30),
            cache_write_price_per_million: Some(3.75),
            updated_at: now,
        },
        ModelPricing {
            model: "claude-3.5-haiku".into(),
            provider: "anthropic".into(),
            input_price_per_million: 0.80,
            output_price_per_million: 4.00,
            cache_read_price_per_million: Some(0.08),
            cache_write_price_per_million: Some(1.00),
            updated_at: now,
        },
        // Google
        ModelPricing {
            model: "gemini-2.0-flash".into(),
            provider: "google".into(),
            input_price_per_million: 0.10,
            output_price_per_million: 0.40,
            cache_read_price_per_million: Some(0.025),
            cache_write_price_per_million: Some(0.10),
            updated_at: now,
        },
        ModelPricing {
            model: "gemini-1.5-pro".into(),
            provider: "google".into(),
            input_price_per_million: 1.25,
            output_price_per_million: 10.00,
            cache_read_price_per_million: Some(0.3125),
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // DeepSeek (cheapest!)
        ModelPricing {
            model: "deepseek-chat".into(),
            provider: "deepseek".into(),
            input_price_per_million: 0.14,
            output_price_per_million: 0.28,
            cache_read_price_per_million: Some(0.014),
            cache_write_price_per_million: None,
            updated_at: now,
        },
        ModelPricing {
            model: "deepseek-reasoner".into(),
            provider: "deepseek".into(),
            input_price_per_million: 0.55,
            output_price_per_million: 2.19,
            cache_read_price_per_million: Some(0.14),
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // Mistral
        ModelPricing {
            model: "mistral-large".into(),
            provider: "mistral".into(),
            input_price_per_million: 2.00,
            output_price_per_million: 6.00,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        ModelPricing {
            model: "codestral".into(),
            provider: "mistral".into(),
            input_price_per_million: 0.30,
            output_price_per_million: 0.90,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // xAI
        ModelPricing {
            model: "grok-2".into(),
            provider: "xai".into(),
            input_price_per_million: 2.00,
            output_price_per_million: 10.00,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // Groq (fastest)
        ModelPricing {
            model: "llama-3.3-70b".into(),
            provider: "groq".into(),
            input_price_per_million: 0.59,
            output_price_per_million: 0.79,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
        // Ollama (free - local)
        ModelPricing {
            model: "llama3.2".into(),
            provider: "ollama".into(),
            input_price_per_million: 0.00,
            output_price_per_million: 0.00,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: now,
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════════
//  USAGE RECORD
// ═══════════════════════════════════════════════════════════════════════════════

/// Usage record for a single request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    /// Record ID
    pub id: String,
    /// Model used
    pub model: String,
    /// Provider
    pub provider: String,
    /// Application name
    pub app_name: String,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Request ID
    pub request_id: String,
    /// Input tokens
    pub input_tokens: u64,
    /// Output tokens
    pub output_tokens: u64,
    /// Cache read tokens
    pub cache_read_tokens: u64,
    /// Cache write tokens
    pub cache_write_tokens: u64,
    /// Total tokens
    pub total_tokens: u64,
    /// Cost in USD
    pub cost: f64,
    /// Latency in ms
    pub latency_ms: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Success
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl UsageRecord {
    pub fn new(
        model: String,
        provider: String,
        app_name: String,
        input_tokens: u64,
        output_tokens: u64,
        latency_ms: u64,
    ) -> Self {
        let total_tokens = input_tokens + output_tokens;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            model,
            provider,
            app_name,
            user_id: None,
            request_id: uuid::Uuid::new_v4().to_string(),
            input_tokens,
            output_tokens,
            cache_read_tokens: 0,
            cache_write_tokens: 0,
            total_tokens,
            cost: 0.0, // Will be calculated by tracker
            latency_ms,
            timestamp: Utc::now(),
            success: true,
            error: None,
        }
    }
    
    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn with_cache(mut self, read: u64, write: u64) -> Self {
        self.cache_read_tokens = read;
        self.cache_write_tokens = write;
        self
    }
    
    pub fn with_error(mut self, error: String) -> Self {
        self.success = false;
        self.error = Some(error);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COST TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Cost tracker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostTrackerConfig {
    /// Daily budget limit (USD)
    pub daily_budget: Option<f64>,
    /// Monthly budget limit (USD)
    pub monthly_budget: Option<f64>,
    /// Per-app budget limits
    pub app_budgets: HashMap<String, f64>,
    /// Alert threshold (0.0 - 1.0)
    pub alert_threshold: f32,
    /// Enable cost optimization suggestions
    pub enable_optimization: bool,
    /// Retention days for records
    pub retention_days: u32,
}

impl Default for CostTrackerConfig {
    fn default() -> Self {
        Self {
            daily_budget: Some(50.0),
            monthly_budget: Some(1000.0),
            app_budgets: HashMap::new(),
            alert_threshold: 0.8,
            enable_optimization: true,
            retention_days: 90,
        }
    }
}

/// LLM Cost Tracker
pub struct CostTracker {
    /// Configuration
    config: CostTrackerConfig,
    /// Model pricing database
    pricing: HashMap<String, ModelPricing>,
    /// Usage records
    records: Arc<RwLock<Vec<UsageRecord>>>,
    /// Daily totals cache
    daily_totals: Arc<RwLock<HashMap<String, DailyTotal>>>,
    /// Monthly totals cache
    monthly_totals: Arc<RwLock<HashMap<String, MonthlyTotal>>>,
}

impl CostTracker {
    pub fn new(config: CostTrackerConfig) -> Self {
        let pricing_map: HashMap<String, ModelPricing> = get_default_pricing()
            .into_iter()
            .map(|p| (p.model.clone(), p))
            .collect();
        
        Self {
            config,
            pricing: pricing_map,
            records: Arc::new(RwLock::new(Vec::new())),
            daily_totals: Arc::new(RwLock::new(HashMap::new())),
            monthly_totals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Record usage
    pub async fn record(&self, mut record: UsageRecord) -> f64 {
        // Calculate cost
        let cost = if let Some(pricing) = self.pricing.get(&record.model) {
            pricing.calculate_cost_with_cache(
                record.input_tokens,
                record.output_tokens,
                record.cache_read_tokens,
                record.cache_write_tokens,
            )
        } else {
            // Default pricing if model not found ($1 per 1M tokens)
            (record.total_tokens as f64 / 1_000_000.0) * 1.0
        };
        
        record.cost = cost;
        
        // Update daily totals
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let mut daily = self.daily_totals.write().await;
        let day_total = daily.entry(today).or_insert(DailyTotal::default());
        day_total.total_cost += cost;
        day_total.total_tokens += record.total_tokens;
        day_total.request_count += 1;
        
        // Update model breakdown
        *day_total.by_model.entry(record.model.clone()).or_insert(0.0) += cost;
        *day_total.by_provider.entry(record.provider.clone()).or_insert(0.0) += cost;
        *day_total.by_app.entry(record.app_name.clone()).or_insert(0.0) += cost;
        
        // Store record
        let mut records = self.records.write().await;
        records.push(record.clone());
        
        // Check budgets
        self.check_budgets(&day_total.total_cost).await;
        
        cost
    }
    
    /// Check if budget alerts should fire
    async fn check_budgets(&self, daily_cost: &f64) {
        if let Some(daily_budget) = self.config.daily_budget {
            let threshold = daily_budget * self.config.alert_threshold as f64;
            if daily_cost >= &threshold {
                log::warn!("💰 Budget Alert: Daily cost ${:.2} exceeds {:.0}% of ${:.2} budget",
                    daily_cost, self.config.alert_threshold * 100.0, daily_budget);
            }
        }
    }
    
    /// Get daily statistics
    pub async fn get_daily_stats(&self, date: &str) -> Option<DailyTotal> {
        let daily = self.daily_totals.read().await;
        daily.get(date).cloned()
    }
    
    /// Get today's statistics
    pub async fn get_today_stats(&self) -> DailyTotal {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        self.get_daily_stats(&today).await.unwrap_or_default()
    }
    
    /// Get monthly statistics
    pub async fn get_monthly_stats(&self, year_month: &str) -> Option<MonthlyTotal> {
        let monthly = self.monthly_totals.read().await;
        monthly.get(year_month).cloned()
    }
    
    /// Get cost optimization suggestions
    pub async fn get_optimization_suggestions(&self) -> Vec<CostOptimization> {
        if !self.config.enable_optimization {
            return vec![];
        }
        
        let mut suggestions = Vec::new();
        let records = self.records.read().await;
        
        // Analyze by model
        let mut model_costs: HashMap<String, f64> = HashMap::new();
        let mut model_tokens: HashMap<String, u64> = HashMap::new();
        
        for record in records.iter() {
            *model_costs.entry(record.model.clone()).or_insert(0.0) += record.cost;
            *model_tokens.entry(record.model.clone()).or_insert(0) += record.total_tokens;
        }
        
        // Find expensive models
        for (model, cost) in model_costs.iter() {
            if *cost > 10.0 {
                // Check if there's a cheaper alternative
                if model.starts_with("gpt-4-turbo") || model.starts_with("gpt-4-") {
                    suggestions.push(CostOptimization {
                        recommendation: format!("Switch {} to gpt-4o for similar performance at 75% lower cost", model),
                        potential_savings: cost * 0.75,
                        priority: OptimizationPriority::High,
                    });
                }
                if model.starts_with("claude-3-opus") {
                    suggestions.push(CostOptimization {
                        recommendation: format!("Switch {} to claude-4-sonnet for similar performance at 80% lower cost", model),
                        potential_savings: cost * 0.80,
                        priority: OptimizationPriority::High,
                    });
                }
            }
        }
        
        // Suggest caching if not used
        let total_input: u64 = records.iter().map(|r| r.input_tokens).sum();
        let cache_used: u64 = records.iter().map(|r| r.cache_read_tokens).sum();
        
        if total_input > 1_000_000 && cache_used == 0 {
            suggestions.push(CostOptimization {
                recommendation: "Enable prompt caching for repeated context (50-90% savings on cached tokens)".into(),
                potential_savings: records.iter().map(|r| r.cost * 0.3).sum(),
                priority: OptimizationPriority::Medium,
            });
        }
        
        // Sort by potential savings
        suggestions.sort_by(|a, b| b.potential_savings.partial_cmp(&a.potential_savings).unwrap());
        suggestions
    }
    
    /// Get usage by model
    pub async fn get_usage_by_model(&self) -> HashMap<String, ModelUsageStats> {
        let records = self.records.read().await;
        let mut stats: HashMap<String, ModelUsageStats> = HashMap::new();
        
        for record in records.iter() {
            let entry = stats.entry(record.model.clone()).or_default();
            entry.total_requests += 1;
            entry.total_tokens += record.total_tokens;
            entry.total_cost += record.cost;
            entry.avg_latency_ms = 
                (entry.avg_latency_ms * (entry.total_requests - 1) as f64 + record.latency_ms as f64) 
                / entry.total_requests as f64;
        }
        
        stats
    }
    
    /// Export to CSV
    pub async fn export_csv(&self) -> Result<String, CostTrackerError> {
        let records = self.records.read().await;
        let mut csv = String::from("timestamp,model,provider,app,input_tokens,output_tokens,cost,latency_ms\n");
        
        for record in records.iter() {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{:.6},{}\n",
                record.timestamp.to_rfc3339(),
                record.model,
                record.provider,
                record.app_name,
                record.input_tokens,
                record.output_tokens,
                record.cost,
                record.latency_ms
            ));
        }
        
        Ok(csv)
    }
    
    /// Update pricing
    pub fn update_pricing(&mut self, model: String, pricing: ModelPricing) {
        self.pricing.insert(model, pricing);
    }
}

/// Daily total statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DailyTotal {
    pub total_cost: f64,
    pub total_tokens: u64,
    pub request_count: u64,
    pub by_model: HashMap<String, f64>,
    pub by_provider: HashMap<String, f64>,
    pub by_app: HashMap<String, f64>,
}

/// Monthly total statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonthlyTotal {
    pub total_cost: f64,
    pub total_tokens: u64,
    pub request_count: u64,
    pub by_model: HashMap<String, f64>,
    pub by_provider: HashMap<String, f64>,
    pub by_app: HashMap<String, f64>,
}

/// Model usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelUsageStats {
    pub total_requests: u64,
    pub total_tokens: u64,
    pub total_cost: f64,
    pub avg_latency_ms: f64,
}

/// Cost optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostOptimization {
    pub recommendation: String,
    pub potential_savings: f64,
    pub priority: OptimizationPriority,
}

/// Optimization priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationPriority {
    High,
    Medium,
    Low,
}

/// Cost tracker error
#[derive(Debug, Clone)]
pub enum CostTrackerError {
    ExportError(String),
    InvalidDate(String),
}

impl std::fmt::Display for CostTrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExportError(e) => write!(f, "Export error: {}", e),
            Self::InvalidDate(e) => write!(f, "Invalid date: {}", e),
        }
    }
}

impl std::error::Error for CostTrackerError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pricing_calculation() {
        let pricing = ModelPricing {
            model: "test".into(),
            provider: "test".into(),
            input_price_per_million: 10.0,
            output_price_per_million: 30.0,
            cache_read_price_per_million: None,
            cache_write_price_per_million: None,
            updated_at: Utc::now(),
        };
        
        // 1M input + 500K output = $10 + $15 = $25
        let cost = pricing.calculate_cost(1_000_000, 500_000);
        assert!((cost - 25.0).abs() < 0.001);
    }
    
    #[test]
    fn test_pricing_with_cache() {
        let pricing = ModelPricing {
            model: "test".into(),
            provider: "test".into(),
            input_price_per_million: 10.0,
            output_price_per_million: 30.0,
            cache_read_price_per_million: Some(1.0),
            cache_write_price_per_million: Some(5.0),
            updated_at: Utc::now(),
        };
        
        // 1M input + 500K output + 500K cache read + 100K cache write
        // = $10 + $15 + $0.50 + $0.50 = $26
        let cost = pricing.calculate_cost_with_cache(1_000_000, 500_000, 500_000, 100_000);
        assert!((cost - 26.0).abs() < 0.001);
    }
    
    #[tokio::test]
    async fn test_cost_tracker() {
        let tracker = CostTracker::new(CostTrackerConfig::default());
        
        let record = UsageRecord::new(
            "gpt-4o".into(),
            "openai".into(),
            "test-app".into(),
            1000,
            500,
            500,
        );
        
        let cost = tracker.record(record).await;
        assert!(cost > 0.0);
        
        let stats = tracker.get_today_stats().await;
        assert_eq!(stats.request_count, 1);
    }
}
