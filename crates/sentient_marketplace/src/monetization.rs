//! ─── Skill Monetization System ───
//!
//!  Features:
//!  - Multiple pricing models (Free, One-time, Subscription, Usage-based)
//!  - Revenue sharing with developers
//!  - Payment processing integration
//!  - License key management
//!  - Developer earnings tracking
//!  - Refund handling

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Skill pricing model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum PricingModel {
    /// Free to use
    Free {
        /// Optional donation link
        donation_url: Option<String>,
    },
    
    /// One-time purchase
    OneTime {
        /// Price in USD
        price: f64,
        /// Original price (for discounts)
        original_price: Option<f64>,
        /// Currency (default: USD)
        currency: String,
    },
    
    /// Subscription
    Subscription {
        /// Monthly price in USD
        monthly_price: f64,
        /// Yearly price (optional discount)
        yearly_price: Option<f64>,
        /// Trial days
        trial_days: u32,
        /// Currency
        currency: String,
    },
    
    /// Usage-based pricing
    UsageBased {
        /// Price per 1000 API calls
        price_per_1k: f64,
        /// Free tier (calls per month)
        free_tier: u32,
        /// Currency
        currency: String,
    },
    
    /// Freemium (free tier + premium)
    Freemium {
        /// Free features
        free_features: Vec<String>,
        /// Premium pricing
        premium: Box<PricingModel>,
    },
}

impl PricingModel {
    /// Check if free
    pub fn is_free(&self) -> bool {
        matches!(self, PricingModel::Free { .. })
    }
    
    /// Get price display string
    pub fn display_price(&self) -> String {
        match self {
            PricingModel::Free { .. } => "Free".to_string(),
            PricingModel::OneTime { price, currency, .. } => {
                format!("${:.2} {}", price, currency)
            }
            PricingModel::Subscription { monthly_price, yearly_price, currency, .. } => {
                if let Some(yearly) = yearly_price {
                    format!("${:.2}/mo or ${:.2}/yr {}", monthly_price, yearly, currency)
                } else {
                    format!("${:.2}/mo {}", monthly_price, currency)
                }
            }
            PricingModel::UsageBased { price_per_1k, free_tier, .. } => {
                format!("${:.3}/1K calls ({} free)", price_per_1k, free_tier)
            }
            PricingModel::Freemium { premium, .. } => {
                format!("Free + {}", premium.display_price())
            }
        }
    }
}

impl Default for PricingModel {
    fn default() -> Self {
        PricingModel::Free { donation_url: None }
    }
}

/// Skill license
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLicense {
    /// License ID
    pub id: Uuid,
    
    /// Skill ID
    pub skill_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// License key
    pub license_key: String,
    
    /// License type
    pub license_type: LicenseType,
    
    /// Purchase date
    pub purchased_at: DateTime<Utc>,
    
    /// Expiration date (for subscriptions)
    pub expires_at: Option<DateTime<Utc>>,
    
    /// Is active
    pub active: bool,
    
    /// Usage count (for usage-based)
    pub usage_count: u64,
    
    /// Usage limit (for usage-based)
    pub usage_limit: Option<u64>,
}

impl SkillLicense {
    /// Check if license is valid
    pub fn is_valid(&self) -> bool {
        if !self.active {
            return false;
        }
        
        if let Some(expires) = self.expires_at {
            if expires < Utc::now() {
                return false;
            }
        }
        
        if let Some(limit) = self.usage_limit {
            if self.usage_count >= limit {
                return false;
            }
        }
        
        true
    }
}

/// License type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    /// Free license
    Free,
    
    /// Personal license
    Personal,
    
    /// Team license (N seats)
    Team { seats: u32 },
    
    /// Enterprise license
    Enterprise { 
        seats: u32,
        custom_terms: bool,
    },
    
    /// Trial license
    Trial { days: u32 },
}

/// Purchase record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    /// Purchase ID
    pub id: Uuid,
    
    /// Skill ID
    pub skill_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Amount paid
    pub amount: f64,
    
    /// Currency
    pub currency: String,
    
    /// Payment method
    pub payment_method: PaymentMethod,
    
    /// Payment status
    pub status: PaymentStatus,
    
    /// Purchase date
    pub purchased_at: DateTime<Utc>,
    
    /// Generated license
    pub license: Option<SkillLicense>,
    
    /// Refund info
    pub refund: Option<Refund>,
}

/// Payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard { last4: String, brand: String },
    PayPal { email: String },
    Stripe { customer_id: String },
    Crypto { wallet: String, chain: String },
    ApplePay,
    GooglePay,
}

/// Payment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Disputed,
}

/// Refund record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    /// Refund ID
    pub id: Uuid,
    
    /// Reason
    pub reason: RefundReason,
    
    /// Amount refunded
    pub amount: f64,
    
    /// Refund date
    pub refunded_at: DateTime<Utc>,
    
    /// Status
    pub status: RefundStatus,
}

/// Refund reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundReason {
    NotAsDescribed,
    DoesntWork,
    FoundAlternative,
    ChangedMind,
    Other(String),
}

/// Refund status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefundStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

/// Developer earnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperEarnings {
    /// Developer ID
    pub developer_id: String,
    
    /// Total earnings
    pub total_earnings: f64,
    
    /// Pending earnings
    pub pending_earnings: f64,
    
    /// Withdrawn earnings
    pub withdrawn: f64,
    
    /// Earnings by skill
    pub by_skill: HashMap<String, SkillEarnings>,
    
    /// Earnings history
    pub history: Vec<EarningsRecord>,
    
    /// Payout method
    pub payout_method: Option<PayoutMethod>,
}

/// Skill earnings breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEarnings {
    pub skill_id: String,
    pub skill_name: String,
    pub total_sales: u64,
    pub total_earnings: f64,
    pub monthly_earnings: Vec<MonthlyEarning>,
}

/// Monthly earning record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyEarning {
    pub month: String,
    pub sales: u64,
    pub amount: f64,
}

/// Earnings record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarningsRecord {
    pub id: Uuid,
    pub amount: f64,
    pub source: EarningsSource,
    pub created_at: DateTime<Utc>,
}

/// Earnings source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EarningsSource {
    Sale { purchase_id: Uuid, skill_id: String },
    Subscription { subscription_id: Uuid, skill_id: String },
    Usage { skill_id: String, calls: u64 },
    Refund { refund_id: Uuid, skill_id: String },
}

/// Payout method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayoutMethod {
    BankTransfer {
        account_holder: String,
        bank_name: String,
        account_number: String, // Last 4 digits visible
        routing_number: String, // Last 4 digits visible
    },
    PayPal {
        email: String,
    },
    Stripe {
        account_id: String,
    },
    Crypto {
        wallet: String,
        chain: String,
    },
}

/// Revenue share configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueShare {
    /// Platform fee percentage
    pub platform_fee: f64,
    
    /// Payment processor fee percentage
    pub processor_fee: f64,
    
    /// Developer share percentage
    pub developer_share: f64,
    
    /// Minimum payout amount
    pub min_payout: f64,
    
    /// Payout schedule (days)
    pub payout_delay_days: u32,
}

impl Default for RevenueShare {
    fn default() -> Self {
        Self {
            platform_fee: 20.0,      // 20% platform fee
            processor_fee: 2.9,      // 2.9% + $0.30 per transaction
            developer_share: 77.1,   // Remaining goes to developer
            min_payout: 50.0,        // $50 minimum payout
            payout_delay_days: 30,   // 30-day hold
        }
    }
}

impl RevenueShare {
    /// Calculate developer earnings from sale
    pub fn calculate_earnings(&self, amount: f64) -> f64 {
        let after_platform = amount * (1.0 - self.platform_fee / 100.0);
        let after_processor = after_platform * (1.0 - self.processor_fee / 100.0);
        after_processor - 0.30 // Stripe flat fee
    }
}

/// Monetization manager
pub struct MonetizationManager {
    /// Revenue share config
    revenue_share: RevenueShare,
    
    /// Active licenses
    licenses: HashMap<String, Vec<SkillLicense>>,
    
    /// Purchases
    purchases: Vec<Purchase>,
    
    /// Developer earnings
    developer_earnings: HashMap<String, DeveloperEarnings>,
}

impl MonetizationManager {
    pub fn new() -> Self {
        Self {
            revenue_share: RevenueShare::default(),
            licenses: HashMap::new(),
            purchases: Vec::new(),
            developer_earnings: HashMap::new(),
        }
    }
    
    /// Create purchase
    pub fn create_purchase(
        &mut self,
        skill_id: String,
        user_id: String,
        pricing: &PricingModel,
        payment_method: PaymentMethod,
    ) -> Result<Purchase, MonetizationError> {
        let amount = match pricing {
            PricingModel::Free { .. } => 0.0,
            PricingModel::OneTime { price, .. } => *price,
            PricingModel::Subscription { monthly_price, .. } => *monthly_price,
            PricingModel::UsageBased { .. } => 0.0, // Charged on usage
            PricingModel::Freemium { .. } => 0.0, // Freemium purchase handled separately
        };
        
        let purchase = Purchase {
            id: Uuid::new_v4(),
            skill_id: skill_id.clone(),
            user_id: user_id.clone(),
            amount,
            currency: "USD".to_string(),
            payment_method,
            status: PaymentStatus::Pending,
            purchased_at: Utc::now(),
            license: None,
            refund: None,
        };
        
        self.purchases.push(purchase.clone());
        Ok(purchase)
    }
    
    /// Complete purchase and generate license
    pub fn complete_purchase(
        &mut self,
        purchase_id: Uuid,
        developer_id: &str,
    ) -> Result<SkillLicense, MonetizationError> {
        let purchase = self.purchases.iter_mut()
            .find(|p| p.id == purchase_id)
            .ok_or_else(|| MonetizationError::PurchaseNotFound(purchase_id.to_string()))?;
        
        purchase.status = PaymentStatus::Completed;
        
        // Generate license
        let license = SkillLicense {
            id: Uuid::new_v4(),
            skill_id: purchase.skill_id.clone(),
            user_id: purchase.user_id.clone(),
            license_key: generate_license_key(),
            license_type: LicenseType::Personal,
            purchased_at: purchase.purchased_at,
            expires_at: None,
            active: true,
            usage_count: 0,
            usage_limit: None,
        };
        
        purchase.license = Some(license.clone());
        
        // Update developer earnings
        let earnings = self.developer_earnings
            .entry(developer_id.to_string())
            .or_insert_with(|| DeveloperEarnings {
                developer_id: developer_id.to_string(),
                total_earnings: 0.0,
                pending_earnings: 0.0,
                withdrawn: 0.0,
                by_skill: HashMap::new(),
                history: Vec::new(),
                payout_method: None,
            });
        
        let dev_earnings = self.revenue_share.calculate_earnings(purchase.amount);
        earnings.pending_earnings += dev_earnings;
        earnings.total_earnings += dev_earnings;
        
        // Add to skill earnings
        let skill_earnings = earnings.by_skill
            .entry(purchase.skill_id.clone())
            .or_insert_with(|| SkillEarnings {
                skill_id: purchase.skill_id.clone(),
                skill_name: purchase.skill_id.clone(),
                total_sales: 0,
                total_earnings: 0.0,
                monthly_earnings: Vec::new(),
            });
        
        skill_earnings.total_sales += 1;
        skill_earnings.total_earnings += dev_earnings;
        
        // Store license
        self.licenses
            .entry(purchase.user_id.clone())
            .or_default()
            .push(license.clone());
        
        Ok(license)
    }
    
    /// Check license
    pub fn check_license(&self, user_id: &str, skill_id: &str) -> Option<&SkillLicense> {
        self.licenses.get(user_id)?.iter()
            .find(|l| l.skill_id == skill_id && l.is_valid())
    }
    
    /// Record usage (for usage-based pricing)
    pub fn record_usage(&mut self, user_id: &str, skill_id: &str, calls: u64) {
        if let Some(licenses) = self.licenses.get_mut(user_id) {
            if let Some(license) = licenses.iter_mut().find(|l| l.skill_id == skill_id) {
                license.usage_count += calls;
            }
        }
    }
    
    /// Request refund
    pub fn request_refund(
        &mut self,
        purchase_id: Uuid,
        reason: RefundReason,
    ) -> Result<Refund, MonetizationError> {
        let purchase = self.purchases.iter_mut()
            .find(|p| p.id == purchase_id)
            .ok_or_else(|| MonetizationError::PurchaseNotFound(purchase_id.to_string()))?;
        
        if purchase.status == PaymentStatus::Refunded {
            return Err(MonetizationError::AlreadyRefunded);
        }
        
        let refund = Refund {
            id: Uuid::new_v4(),
            reason,
            amount: purchase.amount,
            refunded_at: Utc::now(),
            status: RefundStatus::Pending,
        };
        
        purchase.status = PaymentStatus::Refunded;
        purchase.refund = Some(refund.clone());
        
        // Deactivate license
        if let Some(license) = &mut purchase.license {
            license.active = false;
        }
        
        Ok(refund)
    }
    
    /// Get developer earnings
    pub fn get_earnings(&self, developer_id: &str) -> Option<&DeveloperEarnings> {
        self.developer_earnings.get(developer_id)
    }
    
    /// Get user purchases
    pub fn get_user_purchases(&self, user_id: &str) -> Vec<&Purchase> {
        self.purchases.iter()
            .filter(|p| p.user_id == user_id)
            .collect()
    }
    
    /// Get user licenses
    pub fn get_user_licenses(&self, user_id: &str) -> Vec<&SkillLicense> {
        self.licenses.get(user_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}

impl Default for MonetizationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate license key
fn generate_license_key() -> String {
    let segments: Vec<String> = (0..4)
        .map(|_| {
            (0..4)
                .map(|_| {
                    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                    let idx = rand::random::<usize>() % CHARSET.len();
                    CHARSET[idx] as char
                })
                .collect()
        })
        .collect();
    
    format!("SENTIENT-{}", segments.join("-"))
}

/// Monetization errors
#[derive(Debug, thiserror::Error)]
pub enum MonetizationError {
    #[error("Purchase not found: {0}")]
    PurchaseNotFound(String),
    
    #[error("License not found")]
    LicenseNotFound,
    
    #[error("Already refunded")]
    AlreadyRefunded,
    
    #[error("Payment failed: {0}")]
    PaymentFailed(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Invalid pricing model")]
    InvalidPricing,
    
    #[error("Developer not found: {0}")]
    DeveloperNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pricing_display() {
        let free = PricingModel::Free { donation_url: None };
        assert_eq!(free.display_price(), "Free");
        
        let onetime = PricingModel::OneTime {
            price: 9.99,
            original_price: None,
            currency: "USD".into(),
        };
        assert!(onetime.display_price().contains("9.99"));
    }
    
    #[test]
    fn test_revenue_share() {
        let share = RevenueShare::default();
        let earnings = share.calculate_earnings(10.0);
        assert!(earnings > 0.0 && earnings < 10.0);
    }
    
    #[test]
    fn test_license_validity() {
        let license = SkillLicense {
            id: Uuid::new_v4(),
            skill_id: "test-skill".into(),
            user_id: "user-1".into(),
            license_key: "KEY".into(),
            license_type: LicenseType::Personal,
            purchased_at: Utc::now(),
            expires_at: None,
            active: true,
            usage_count: 0,
            usage_limit: None,
        };
        assert!(license.is_valid());
    }
}
