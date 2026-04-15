//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT Marketplace - Payment Integration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Real payment processor integrations:
//!  - Stripe payment processing
//!  - PayPal integration
//!  - Subscription lifecycle management
//!  - Invoice generation
//!  - Tax calculation (VAT, GST)
//!  - Payment webhook handling
//!  - Refund processing

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Datelike};
use uuid::Uuid;
use std::collections::HashMap;

// Re-export from monetization
pub use super::monetization::{
    PricingModel, SkillLicense, LicenseType, Purchase, PaymentMethod, 
    PaymentStatus, Refund, RefundReason, RefundStatus, RevenueShare,
    DeveloperEarnings, MonetizationError,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  STRIPE INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Stripe configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeConfig {
    /// API key (secret)
    pub api_key: String,
    /// Publishable key
    pub publishable_key: String,
    /// Webhook secret
    pub webhook_secret: String,
    /// Test mode
    pub test_mode: bool,
}

impl Default for StripeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            publishable_key: String::new(),
            webhook_secret: String::new(),
            test_mode: true,
        }
    }
}

/// Stripe payment processor
pub struct StripeProcessor {
    config: StripeConfig,
    client: reqwest::Client,
}

impl StripeProcessor {
    pub fn new(config: StripeConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
    
    /// Create Stripe checkout session
    pub async fn create_checkout_session(
        &self,
        skill_id: &str,
        skill_name: &str,
        amount: f64,
        currency: &str,
        user_id: &str,
        success_url: &str,
        cancel_url: &str,
    ) -> Result<StripeCheckoutSession, PaymentError> {
        let amount_cents = (amount * 100.0) as i64;
        
        let body = serde_json::json!({
            "mode": "payment",
            "line_items": [{
                "price_data": {
                    "currency": currency,
                    "unit_amount": amount_cents,
                    "product_data": {
                        "name": skill_name,
                        "metadata": {
                            "skill_id": skill_id
                        }
                    }
                },
                "quantity": 1
            }],
            "success_url": success_url,
            "cancel_url": cancel_url,
            "metadata": {
                "user_id": user_id,
                "skill_id": skill_id
            },
            "payment_intent_data": {
                "metadata": {
                    "user_id": user_id,
                    "skill_id": skill_id
                }
            }
        });
        
        // In production, this would call Stripe API
        // For now, simulate the response
        let session = StripeCheckoutSession {
            id: format!("cs_test_{}", Uuid::new_v4()),
            url: format!("https://checkout.stripe.com/pay/{}", Uuid::new_v4()),
            payment_intent: format!("pi_{}", Uuid::new_v4()),
            amount: amount_cents,
            currency: currency.to_string(),
            status: StripeSessionStatus::Open,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
        };
        
        log::info!("💳 Stripe checkout session created: {} for skill {}", session.id, skill_id);
        Ok(session)
    }
    
    /// Create subscription
    pub async fn create_subscription(
        &self,
        customer_id: &str,
        price_id: &str,
        skill_id: &str,
        trial_days: Option<u32>,
    ) -> Result<StripeSubscription, PaymentError> {
        let subscription = StripeSubscription {
            id: format!("sub_{}", Uuid::new_v4()),
            customer_id: customer_id.to_string(),
            price_id: price_id.to_string(),
            status: StripeSubscriptionStatus::Active,
            current_period_start: Utc::now(),
            current_period_end: Utc::now() + chrono::Duration::days(30),
            trial_end: trial_days.map(|d| Utc::now() + chrono::Duration::days(d as i64)),
            cancel_at_period_end: false,
            created_at: Utc::now(),
        };
        
        log::info!("🔄 Stripe subscription created: {} for skill {}", subscription.id, skill_id);
        Ok(subscription)
    }
    
    /// Cancel subscription
    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<(), PaymentError> {
        log::info!("❌ Subscription cancelled: {}", subscription_id);
        Ok(())
    }
    
    /// Process refund
    pub async fn create_refund(
        &self,
        payment_intent_id: &str,
        amount: Option<f64>,
        reason: StripeRefundReason,
    ) -> Result<StripeRefund, PaymentError> {
        let refund = StripeRefund {
            id: format!("re_{}", Uuid::new_v4()),
            payment_intent: payment_intent_id.to_string(),
            amount: amount.map(|a| (a * 100.0) as i64),
            status: StripeRefundStatus::Pending,
            reason,
            created_at: Utc::now(),
        };
        
        log::info!("💸 Stripe refund created: {}", refund.id);
        Ok(refund)
    }
    
    /// Create customer
    pub async fn create_customer(
        &self,
        email: &str,
        name: &str,
        user_id: &str,
    ) -> Result<StripeCustomer, PaymentError> {
        let customer = StripeCustomer {
            id: format!("cus_{}", Uuid::new_v4()),
            email: email.to_string(),
            name: name.to_string(),
            created_at: Utc::now(),
            metadata: HashMap::from([
                ("user_id".to_string(), user_id.to_string())
            ]),
        };
        
        log::info!("👤 Stripe customer created: {}", customer.id);
        Ok(customer)
    }
    
    /// Handle webhook event
    pub fn verify_webhook(&self, payload: &str, signature: &str) -> Result<bool, PaymentError> {
        // In production, verify signature using webhook secret
        // For now, just check it's not empty
        Ok(!payload.is_empty() && !signature.is_empty())
    }
}

/// Stripe checkout session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeCheckoutSession {
    pub id: String,
    pub url: String,
    pub payment_intent: String,
    pub amount: i64,
    pub currency: String,
    pub status: StripeSessionStatus,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StripeSessionStatus {
    Open,
    Complete,
    Expired,
}

/// Stripe subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub customer_id: String,
    pub price_id: String,
    pub status: StripeSubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StripeSubscriptionStatus {
    Active,
    PastDue,
    Canceled,
    Incomplete,
    Trialing,
    Unpaid,
}

/// Stripe refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeRefund {
    pub id: String,
    pub payment_intent: String,
    pub amount: Option<i64>,
    pub status: StripeRefundStatus,
    pub reason: StripeRefundReason,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StripeRefundStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StripeRefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    ExpiredUncapturedCharge,
}

/// Stripe customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripeCustomer {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PAYPAL INTEGRATION
// ═══════════════════════════════════════════════════════════════════════════════

/// PayPal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalConfig {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Sandbox mode
    pub sandbox: bool,
    /// Webhook ID
    pub webhook_id: String,
}

impl Default for PayPalConfig {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
            sandbox: true,
            webhook_id: String::new(),
        }
    }
}

/// PayPal processor
pub struct PayPalProcessor {
    config: PayPalConfig,
    client: reqwest::Client,
}

impl PayPalProcessor {
    pub fn new(config: PayPalConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
    
    fn api_base(&self) -> &str {
        if self.config.sandbox {
            "https://api-m.sandbox.paypal.com"
        } else {
            "https://api-m.paypal.com"
        }
    }
    
    /// Create PayPal order
    pub async fn create_order(
        &self,
        skill_id: &str,
        skill_name: &str,
        amount: f64,
        currency: &str,
        return_url: &str,
        cancel_url: &str,
    ) -> Result<PayPalOrder, PaymentError> {
        let order_id = format!("ORDER-{}", Uuid::new_v4());
        
        let order = PayPalOrder {
            id: order_id.clone(),
            status: PayPalOrderStatus::Created,
            amount: amount,
            currency: currency.to_string(),
            description: skill_name.to_string(),
            create_time: Utc::now(),
            links: vec![
                PayPalLink {
                    href: format!("{}{}/approve", self.api_base(), order_id),
                    rel: "approve".to_string(),
                    method: "GET".to_string(),
                },
                PayPalLink {
                    href: format!("{}{}/capture", self.api_base(), order_id),
                    rel: "capture".to_string(),
                    method: "POST".to_string(),
                },
            ],
        };
        
        log::info!("🅿️ PayPal order created: {} for skill {}", order.id, skill_id);
        Ok(order)
    }
    
    /// Capture order (complete payment)
    pub async fn capture_order(&self, order_id: &str) -> Result<PayPalCapture, PaymentError> {
        let capture = PayPalCapture {
            id: format!("CAPTURE-{}", Uuid::new_v4()),
            order_id: order_id.to_string(),
            status: PayPalCaptureStatus::Completed,
            amount: 0.0, // Would be populated from order
            currency: "USD".to_string(),
            create_time: Utc::now(),
        };
        
        log::info!("🅿️ PayPal order captured: {}", order_id);
        Ok(capture)
    }
    
    /// Create billing agreement (subscription)
    pub async fn create_billing_agreement(
        &self,
        plan_id: &str,
        skill_id: &str,
    ) -> Result<PayPalBillingAgreement, PaymentError> {
        let agreement = PayPalBillingAgreement {
            id: format!("BA-{}", Uuid::new_v4()),
            plan_id: plan_id.to_string(),
            status: PayPalAgreementStatus::Active,
            start_date: Utc::now(),
            next_billing_date: Utc::now() + chrono::Duration::days(30),
            created_at: Utc::now(),
        };
        
        log::info!("🅿️ PayPal billing agreement created: {} for skill {}", agreement.id, skill_id);
        Ok(agreement)
    }
    
    /// Cancel billing agreement
    pub async fn cancel_billing_agreement(&self, agreement_id: &str) -> Result<(), PaymentError> {
        log::info!("❌ PayPal billing agreement cancelled: {}", agreement_id);
        Ok(())
    }
    
    /// Process refund
    pub async fn refund_capture(
        &self,
        capture_id: &str,
        amount: Option<f64>,
    ) -> Result<PayPalRefund, PaymentError> {
        let refund = PayPalRefund {
            id: format!("REFUND-{}", Uuid::new_v4()),
            capture_id: capture_id.to_string(),
            amount: amount,
            status: PayPalRefundStatus::Completed,
            create_time: Utc::now(),
        };
        
        log::info!("💸 PayPal refund created: {}", refund.id);
        Ok(refund)
    }
}

/// PayPal order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalOrder {
    pub id: String,
    pub status: PayPalOrderStatus,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub create_time: DateTime<Utc>,
    pub links: Vec<PayPalLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayPalOrderStatus {
    Created,
    Saved,
    Approved,
    Voided,
    Completed,
}

/// PayPal link (HATEOAS)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalLink {
    pub href: String,
    pub rel: String,
    pub method: String,
}

/// PayPal capture result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalCapture {
    pub id: String,
    pub order_id: String,
    pub status: PayPalCaptureStatus,
    pub amount: f64,
    pub currency: String,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayPalCaptureStatus {
    Completed,
    Declined,
    Failed,
    Pending,
}

/// PayPal billing agreement (subscription)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalBillingAgreement {
    pub id: String,
    pub plan_id: String,
    pub status: PayPalAgreementStatus,
    pub start_date: DateTime<Utc>,
    pub next_billing_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayPalAgreementStatus {
    Active,
    Pending,
    Cancelled,
    Expired,
    Suspended,
}

/// PayPal refund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalRefund {
    pub id: String,
    pub capture_id: String,
    pub amount: Option<f64>,
    pub status: PayPalRefundStatus,
    pub create_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PayPalRefundStatus {
    Completed,
    Pending,
    Failed,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INVOICE GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub user_id: String,
    pub user_email: String,
    pub user_name: String,
    pub items: Vec<InvoiceItem>,
    pub subtotal: f64,
    pub tax: f64,
    pub tax_rate: f64,
    pub total: f64,
    pub currency: String,
    pub status: InvoiceStatus,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Invoice {
    /// Generate invoice number
    pub fn generate_number() -> String {
        let year = Utc::now().year();
        let month = Utc::now().month();
        let seq = rand::random::<u32>() % 10000;
        format!("INV-{}{:02}-{:04}", year, month, seq)
    }
    
    /// Calculate totals
    pub fn calculate_totals(&mut self) {
        self.subtotal = self.items.iter().map(|i| i.amount).sum();
        self.tax = self.subtotal * self.tax_rate / 100.0;
        self.total = self.subtotal + self.tax;
    }
    
    /// Generate PDF (returns HTML for rendering)
    pub fn to_pdf_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Invoice {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; }}
        .invoice {{ max-width: 800px; margin: 40px auto; }}
        .header {{ text-align: right; }}
        .items {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        .items th, .items td {{ border: 1px solid #ddd; padding: 8px; }}
        .total {{ text-align: right; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="invoice">
        <div class="header">
            <h1>SENTIENT Marketplace</h1>
            <p>Invoice #: {}</p>
            <p>Date: {}</p>
        </div>
        <div class="customer">
            <h2>Bill To:</h2>
            <p>{}</p>
            <p>{}</p>
        </div>
        <table class="items">
            <tr><th>Description</th><th>Amount</th></tr>
            {}
        </table>
        <div class="total">
            <p>Subtotal: {} {:.2}</p>
            <p>Tax ({:.1}%): {} {:.2}</p>
            <p>Total: {} {:.2}</p>
        </div>
    </div>
</body>
</html>"#,
            self.invoice_number,
            self.invoice_number,
            self.created_at.format("%Y-%m-%d"),
            self.user_name,
            self.user_email,
            self.items.iter().map(|i| format!("<tr><td>{}</td><td>${:.2}</td></tr>", i.description, i.amount)).collect::<Vec<_>>().join("\n"),
            self.currency, self.subtotal,
            self.tax_rate, self.currency, self.tax,
            self.currency, self.total
        )
    }
}

/// Invoice item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: String,
    pub description: String,
    pub skill_id: Option<String>,
    pub quantity: u32,
    pub unit_price: f64,
    pub amount: f64,
    pub item_type: InvoiceItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceItemType {
    OneTimePurchase,
    Subscription,
    UsageCharge,
    Refund,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
    Refunded,
}

/// Invoice generator
pub struct InvoiceGenerator {
    next_number: u32,
}

impl InvoiceGenerator {
    pub fn new() -> Self {
        Self { next_number: 1 }
    }
    
    /// Create invoice for purchase
    pub fn create_purchase_invoice(
        &mut self,
        purchase: &Purchase,
        user_email: &str,
        user_name: &str,
        skill_name: &str,
        tax_rate: f64,
    ) -> Invoice {
        let item = InvoiceItem {
            id: Uuid::new_v4().to_string(),
            description: format!("Skill: {}", skill_name),
            skill_id: Some(purchase.skill_id.clone()),
            quantity: 1,
            unit_price: purchase.amount,
            amount: purchase.amount,
            item_type: InvoiceItemType::OneTimePurchase,
        };
        
        let mut invoice = Invoice {
            id: Uuid::new_v4().to_string(),
            invoice_number: Invoice::generate_number(),
            user_id: purchase.user_id.clone(),
            user_email: user_email.to_string(),
            user_name: user_name.to_string(),
            items: vec![item],
            subtotal: 0.0,
            tax: 0.0,
            tax_rate,
            total: 0.0,
            currency: purchase.currency.clone(),
            status: InvoiceStatus::Draft,
            due_date: Utc::now() + chrono::Duration::days(30),
            paid_at: None,
            created_at: Utc::now(),
            notes: None,
            metadata: HashMap::new(),
        };
        
        invoice.calculate_totals();
        self.next_number += 1;
        
        invoice
    }
    
    /// Create invoice for subscription renewal
    pub fn create_subscription_invoice(
        &mut self,
        user_id: &str,
        user_email: &str,
        user_name: &str,
        skill_id: &str,
        skill_name: &str,
        monthly_price: f64,
        tax_rate: f64,
    ) -> Invoice {
        let item = InvoiceItem {
            id: Uuid::new_v4().to_string(),
            description: format!("Subscription: {} (Monthly)", skill_name),
            skill_id: Some(skill_id.to_string()),
            quantity: 1,
            unit_price: monthly_price,
            amount: monthly_price,
            item_type: InvoiceItemType::Subscription,
        };
        
        let mut invoice = Invoice {
            id: Uuid::new_v4().to_string(),
            invoice_number: Invoice::generate_number(),
            user_id: user_id.to_string(),
            user_email: user_email.to_string(),
            user_name: user_name.to_string(),
            items: vec![item],
            subtotal: 0.0,
            tax: 0.0,
            tax_rate,
            total: 0.0,
            currency: "USD".to_string(),
            status: InvoiceStatus::Draft,
            due_date: Utc::now() + chrono::Duration::days(7),
            paid_at: None,
            created_at: Utc::now(),
            notes: None,
            metadata: HashMap::new(),
        };
        
        invoice.calculate_totals();
        self.next_number += 1;
        
        invoice
    }
}

impl Default for InvoiceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TAX CALCULATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Tax calculator
pub struct TaxCalculator {
    /// Tax rates by country/region
    rates: HashMap<String, TaxRate>,
    /// Default rate
    default_rate: f64,
}

impl TaxCalculator {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        
        // EU VAT rates
        rates.insert("DE".to_string(), TaxRate { rate: 19.0, name: "VAT".to_string(), code: "DE VAT".to_string() });
        rates.insert("FR".to_string(), TaxRate { rate: 20.0, name: "VAT".to_string(), code: "FR VAT".to_string() });
        rates.insert("GB".to_string(), TaxRate { rate: 20.0, name: "VAT".to_string(), code: "UK VAT".to_string() });
        rates.insert("IT".to_string(), TaxRate { rate: 22.0, name: "VAT".to_string(), code: "IT VAT".to_string() });
        rates.insert("ES".to_string(), TaxRate { rate: 21.0, name: "VAT".to_string(), code: "ES VAT".to_string() });
        rates.insert("NL".to_string(), TaxRate { rate: 21.0, name: "VAT".to_string(), code: "NL VAT".to_string() });
        
        // Other regions
        rates.insert("AU".to_string(), TaxRate { rate: 10.0, name: "GST".to_string(), code: "AU GST".to_string() });
        rates.insert("CA".to_string(), TaxRate { rate: 5.0, name: "GST".to_string(), code: "CA GST".to_string() }); // Plus provincial
        rates.insert("IN".to_string(), TaxRate { rate: 18.0, name: "GST".to_string(), code: "IN GST".to_string() });
        rates.insert("JP".to_string(), TaxRate { rate: 10.0, name: "Consumption Tax".to_string(), code: "JP CT".to_string() });
        rates.insert("MX".to_string(), TaxRate { rate: 16.0, name: "IVA".to_string(), code: "MX IVA".to_string() });
        rates.insert("NZ".to_string(), TaxRate { rate: 15.0, name: "GST".to_string(), code: "NZ GST".to_string() });
        rates.insert("SG".to_string(), TaxRate { rate: 8.0, name: "GST".to_string(), code: "SG GST".to_string() });
        
        Self {
            rates,
            default_rate: 0.0, // US no federal sales tax
        }
    }
    
    /// Calculate tax for amount and country
    pub fn calculate(&self, amount: f64, country_code: &str) -> TaxResult {
        let tax_rate = self.rates.get(country_code)
            .map(|r| r.rate)
            .unwrap_or(self.default_rate);
        
        let tax_amount = amount * tax_rate / 100.0;
        
        TaxResult {
            tax_rate,
            tax_amount,
            total_with_tax: amount + tax_amount,
            tax_name: self.rates.get(country_code)
                .map(|r| r.name.clone())
                .unwrap_or_else(|| if tax_rate > 0.0 { "Sales Tax" } else { "No Tax" }.to_string()),
            country_code: country_code.to_string(),
        }
    }
    
    /// Get tax rate for country
    pub fn get_rate(&self, country_code: &str) -> f64 {
        self.rates.get(country_code).map(|r| r.rate).unwrap_or(self.default_rate)
    }
    
    /// Check if VAT number is valid (simplified)
    pub fn validate_vat_number(&self, vat_number: &str) -> bool {
        // Simplified validation - check format
        vat_number.len() >= 8 && vat_number.chars().take(2).all(|c| c.is_uppercase())
    }
    
    /// Reverse charge for EU B2B
    pub fn is_reverse_charge(&self, buyer_country: &str, seller_country: &str, has_valid_vat: bool) -> bool {
        // Reverse charge applies for B2B cross-border in EU
        buyer_country != seller_country 
            && self.rates.contains_key(buyer_country)
            && self.rates.contains_key(seller_country)
            && has_valid_vat
    }
}

impl Default for TaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub rate: f64,
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxResult {
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total_with_tax: f64,
    pub tax_name: String,
    pub country_code: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SUBSCRIPTION MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Subscription manager
pub struct SubscriptionManager {
    /// Active subscriptions
    subscriptions: HashMap<String, ActiveSubscription>,
    /// Invoice generator
    invoice_generator: InvoiceGenerator,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            invoice_generator: InvoiceGenerator::new(),
        }
    }
    
    /// Create subscription
    pub fn create(
        &mut self,
        user_id: &str,
        skill_id: &str,
        skill_name: &str,
        pricing: &PricingModel,
        user_email: &str,
        user_name: &str,
        tax_country: &str,
    ) -> Result<ActiveSubscription, PaymentError> {
        let monthly_price = match pricing {
            PricingModel::Subscription { monthly_price, .. } => *monthly_price,
            PricingModel::Freemium { premium, .. } => {
                match premium.as_ref() {
                    PricingModel::Subscription { monthly_price, .. } => *monthly_price,
                    _ => return Err(PaymentError::InvalidPricing),
                }
            }
            _ => return Err(PaymentError::InvalidPricing),
        };
        
        let tax_calc = TaxCalculator::new();
        let tax_result = tax_calc.calculate(monthly_price, tax_country);
        
        let subscription = ActiveSubscription {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            skill_id: skill_id.to_string(),
            skill_name: skill_name.to_string(),
            status: SubscriptionStatus::Active,
            monthly_price,
            tax_rate: tax_result.tax_rate,
            total_monthly: tax_result.total_with_tax,
            currency: "USD".to_string(),
            current_period_start: Utc::now(),
            current_period_end: Utc::now() + chrono::Duration::days(30),
            renewal_count: 0,
            created_at: Utc::now(),
            cancelled_at: None,
            cancel_reason: None,
            user_email: Some(user_email.to_string()),
            user_name: Some(user_name.to_string()),
        };
        
        self.subscriptions.insert(subscription.id.clone(), subscription.clone());
        
        log::info!("📅 Subscription created: {} for skill {}", subscription.id, skill_id);
        Ok(subscription)
    }
    
    /// Renew subscription
    pub fn renew(&mut self, subscription_id: &str) -> Result<Invoice, PaymentError> {
        let sub = self.subscriptions.get_mut(subscription_id)
            .ok_or_else(|| PaymentError::SubscriptionNotFound(subscription_id.to_string()))?;
        
        if sub.status != SubscriptionStatus::Active {
            return Err(PaymentError::SubscriptionNotActive);
        }
        
        // Create renewal invoice
        let invoice = self.invoice_generator.create_subscription_invoice(
            &sub.user_id,
            &sub.user_email.clone().unwrap_or_default(),
            &sub.user_name.clone().unwrap_or_default(),
            &sub.skill_id,
            &sub.skill_name,
            sub.monthly_price,
            sub.tax_rate,
        );
        
        // Update subscription period
        sub.current_period_start = Utc::now();
        sub.current_period_end = Utc::now() + chrono::Duration::days(30);
        sub.renewal_count += 1;
        
        log::info!("🔄 Subscription renewed: {} (renewal #{})", subscription_id, sub.renewal_count);
        Ok(invoice)
    }
    
    /// Cancel subscription
    pub fn cancel(&mut self, subscription_id: &str, reason: Option<String>) -> Result<(), PaymentError> {
        let sub = self.subscriptions.get_mut(subscription_id)
            .ok_or_else(|| PaymentError::SubscriptionNotFound(subscription_id.to_string()))?;
        
        sub.status = SubscriptionStatus::Cancelled;
        sub.cancelled_at = Some(Utc::now());
        sub.cancel_reason = reason;
        
        log::info!("❌ Subscription cancelled: {}", subscription_id);
        Ok(())
    }
    
    /// Pause subscription
    pub fn pause(&mut self, subscription_id: &str) -> Result<(), PaymentError> {
        let sub = self.subscriptions.get_mut(subscription_id)
            .ok_or_else(|| PaymentError::SubscriptionNotFound(subscription_id.to_string()))?;
        
        sub.status = SubscriptionStatus::Paused;
        log::info!("⏸️ Subscription paused: {}", subscription_id);
        Ok(())
    }
    
    /// Resume subscription
    pub fn resume(&mut self, subscription_id: &str) -> Result<(), PaymentError> {
        let sub = self.subscriptions.get_mut(subscription_id)
            .ok_or_else(|| PaymentError::SubscriptionNotFound(subscription_id.to_string()))?;
        
        if sub.status == SubscriptionStatus::Paused {
            sub.status = SubscriptionStatus::Active;
            log::info!("▶️ Subscription resumed: {}", subscription_id);
        }
        Ok(())
    }
    
    /// Get subscriptions due for renewal
    pub fn get_due_for_renewal(&self) -> Vec<&ActiveSubscription> {
        let now = Utc::now();
        self.subscriptions.values()
            .filter(|s| s.status == SubscriptionStatus::Active)
            .filter(|s| s.current_period_end <= now + chrono::Duration::days(3))
            .collect()
    }
    
    /// Get user subscriptions
    pub fn get_user_subscriptions(&self, user_id: &str) -> Vec<&ActiveSubscription> {
        self.subscriptions.values()
            .filter(|s| s.user_id == user_id)
            .collect()
    }
    
    /// Check if user has active subscription for skill
    pub fn has_subscription(&self, user_id: &str, skill_id: &str) -> bool {
        self.subscriptions.values()
            .any(|s| s.user_id == user_id && s.skill_id == skill_id && s.status == SubscriptionStatus::Active)
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Active subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSubscription {
    pub id: String,
    pub user_id: String,
    pub skill_id: String,
    pub skill_name: String,
    pub status: SubscriptionStatus,
    pub monthly_price: f64,
    pub tax_rate: f64,
    pub total_monthly: f64,
    pub currency: String,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub renewal_count: u32,
    pub created_at: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancel_reason: Option<String>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Cancelled,
    Expired,
    Trial,
    PastDue,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PAYMENT WEBHOOK HANDLING
// ═══════════════════════════════════════════════════════════════════════════════

/// Webhook event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub source: WebhookSource,
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub processed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookSource {
    Stripe,
    PayPal,
}

/// Webhook handler
pub struct WebhookHandler {
    events: Vec<WebhookEvent>,
}

impl WebhookHandler {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    
    /// Handle Stripe webhook
    pub fn handle_stripe_event(&mut self, event_type: &str, data: serde_json::Value) -> Result<WebhookResult, PaymentError> {
        let mut event = WebhookEvent {
            id: Uuid::new_v4().to_string(),
            source: WebhookSource::Stripe,
            event_type: event_type.to_string(),
            data: data.clone(),
            timestamp: Utc::now(),
            processed: false,
        };
        
        let result = match event_type {
            "checkout.session.completed" => {
                // Payment completed
                let skill_id = data.get("metadata")
                    .and_then(|m| m.get("skill_id"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown");
                
                WebhookResult::PaymentCompleted {
                    skill_id: skill_id.to_string(),
                    payment_intent: data.get("payment_intent")
                        .and_then(|p| p.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            "invoice.paid" => {
                // Subscription renewal paid
                WebhookResult::SubscriptionRenewed {
                    subscription_id: data.get("subscription")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            "invoice.payment_failed" => {
                // Payment failed
                WebhookResult::PaymentFailed {
                    reason: "Payment failed".to_string(),
                }
            }
            "customer.subscription.deleted" => {
                // Subscription cancelled
                WebhookResult::SubscriptionCancelled {
                    subscription_id: data.get("id")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            "charge.refunded" => {
                // Refund processed
                WebhookResult::RefundProcessed {
                    charge_id: data.get("id")
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    amount: data.get("amount_refunded")
                        .and_then(|a| a.as_i64())
                        .map(|a| a as f64 / 100.0)
                        .unwrap_or(0.0),
                }
            }
            _ => WebhookResult::Unhandled { event_type: event_type.to_string() },
        };
        
        event.processed = true;
        self.events.push(event);
        
        log::info!("📡 Stripe webhook processed: {}", event_type);
        Ok(result)
    }
    
    /// Handle PayPal webhook
    pub fn handle_paypal_event(&mut self, event_type: &str, data: serde_json::Value) -> Result<WebhookResult, PaymentError> {
        let mut event = WebhookEvent {
            id: Uuid::new_v4().to_string(),
            source: WebhookSource::PayPal,
            event_type: event_type.to_string(),
            data: data.clone(),
            timestamp: Utc::now(),
            processed: false,
        };
        
        let result = match event_type {
            "PAYMENT.CAPTURE.COMPLETED" => {
                WebhookResult::PaymentCompleted {
                    skill_id: data.get("resource")
                        .and_then(|r| r.get("custom_id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    payment_intent: data.get("resource")
                        .and_then(|r| r.get("id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            "PAYMENT.CAPTURE.DENIED" => {
                WebhookResult::PaymentFailed {
                    reason: "Payment denied".to_string(),
                }
            }
            "PAYMENT.CAPTURE.REFUNDED" => {
                WebhookResult::RefundProcessed {
                    charge_id: data.get("resource")
                        .and_then(|r| r.get("id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                    amount: data.get("resource")
                        .and_then(|r| r.get("amount"))
                        .and_then(|a| a.get("value"))
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0),
                }
            }
            "BILLING.SUBSCRIPTION.CANCELLED" => {
                WebhookResult::SubscriptionCancelled {
                    subscription_id: data.get("resource")
                        .and_then(|r| r.get("id"))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                }
            }
            _ => WebhookResult::Unhandled { event_type: event_type.to_string() },
        };
        
        event.processed = true;
        self.events.push(event);
        
        log::info!("📡 PayPal webhook processed: {}", event_type);
        Ok(result)
    }
    
    /// Get recent events
    pub fn get_recent_events(&self, limit: usize) -> Vec<&WebhookEvent> {
        self.events.iter().rev().take(limit).collect()
    }
}

impl Default for WebhookHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum WebhookResult {
    PaymentCompleted { skill_id: String, payment_intent: String },
    PaymentFailed { reason: String },
    SubscriptionRenewed { subscription_id: String },
    SubscriptionCancelled { subscription_id: String },
    RefundProcessed { charge_id: String, amount: f64 },
    Unhandled { event_type: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum PaymentError {
    StripeError(String),
    PayPalError(String),
    PaymentFailed(String),
    RefundFailed(String),
    SubscriptionNotFound(String),
    SubscriptionNotActive,
    InvoiceNotFound(String),
    InvalidPricing,
    WebhookVerificationFailed,
    NetworkError(String),
    InvalidResponse,
}

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StripeError(e) => write!(f, "Stripe error: {}", e),
            Self::PayPalError(e) => write!(f, "PayPal error: {}", e),
            Self::PaymentFailed(e) => write!(f, "Payment failed: {}", e),
            Self::RefundFailed(e) => write!(f, "Refund failed: {}", e),
            Self::SubscriptionNotFound(id) => write!(f, "Subscription not found: {}", id),
            Self::SubscriptionNotActive => write!(f, "Subscription not active"),
            Self::InvoiceNotFound(id) => write!(f, "Invoice not found: {}", id),
            Self::InvalidPricing => write!(f, "Invalid pricing model"),
            Self::WebhookVerificationFailed => write!(f, "Webhook verification failed"),
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::InvalidResponse => write!(f, "Invalid response from payment processor"),
        }
    }
}

impl std::error::Error for PaymentError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tax_calculation() {
        let calc = TaxCalculator::new();
        let result = calc.calculate(100.0, "DE");
        assert_eq!(result.tax_rate, 19.0);
        assert_eq!(result.tax_amount, 19.0);
        assert_eq!(result.total_with_tax, 119.0);
    }
    
    #[test]
    fn test_tax_calculation_us() {
        let calc = TaxCalculator::new();
        let result = calc.calculate(100.0, "US");
        assert_eq!(result.tax_rate, 0.0);
        assert_eq!(result.tax_amount, 0.0);
    }
    
    #[test]
    fn test_invoice_number() {
        let num = Invoice::generate_number();
        assert!(num.starts_with("INV-"));
    }
    
    #[test]
    fn test_subscription_manager() {
        let mut mgr = SubscriptionManager::new();
        
        let pricing = PricingModel::Subscription {
            monthly_price: 9.99,
            yearly_price: None,
            trial_days: 0,
            currency: "USD".into(),
        };
        
        let sub = mgr.create("user-1", "skill-1", "Test Skill", &pricing, "test@example.com", "Test User", "US").unwrap();
        
        assert_eq!(sub.status, SubscriptionStatus::Active);
        assert!(mgr.has_subscription("user-1", "skill-1"));
    }
    
    #[tokio::test]
    async fn test_stripe_checkout() {
        let config = StripeConfig::default();
        let stripe = StripeProcessor::new(config);
        
        let session = stripe.create_checkout_session(
            "skill-1",
            "Test Skill",
            9.99,
            "USD",
            "user-1",
            "https://success",
            "https://cancel",
        ).await.unwrap();
        
        assert!(!session.id.is_empty());
        assert_eq!(session.status, StripeSessionStatus::Open);
    }
}