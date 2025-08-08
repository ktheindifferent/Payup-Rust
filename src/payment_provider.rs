use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::Result;

// Common payment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64, // Amount in smallest currency unit (e.g., cents)
    pub currency: String, // ISO 4217 currency code
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: Option<String>,
    pub method_type: PaymentMethodType,
    pub card: Option<CardDetails>,
    pub bank_account: Option<BankAccountDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    Card,
    BankAccount,
    PayPal,
    ApplePay,
    GooglePay,
    Cryptocurrency,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub number: Option<String>,
    pub exp_month: String,
    pub exp_year: String,
    pub cvv: Option<String>,
    pub brand: Option<String>,
    pub last4: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountDetails {
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
    pub account_type: Option<String>,
    pub bank_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub id: Option<String>,
    pub amount: Money,
    pub customer_id: Option<String>,
    pub payment_method_id: Option<String>,
    pub status: ChargeStatus,
    pub description: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub created_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChargeStatus {
    Pending,
    Processing,
    Succeeded,
    Failed,
    Canceled,
    RequiresAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Option<String>,
    pub charge_id: String,
    pub amount: Option<Money>,
    pub reason: Option<RefundReason>,
    pub status: RefundStatus,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Option<String>,
    pub customer_id: String,
    pub plan_id: Option<String>,
    pub price_id: Option<String>,
    pub status: SubscriptionStatus,
    pub current_period_start: Option<i64>,
    pub current_period_end: Option<i64>,
    pub cancel_at_period_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Active,
    PastDue,
    Canceled,
    Incomplete,
    IncompleteExpired,
    Trialing,
    Unpaid,
}

// Unified Payment Provider Trait
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    // Provider information
    fn name(&self) -> &str;
    fn supported_currencies(&self) -> Vec<String>;
    fn supported_features(&self) -> Vec<PaymentFeature>;
    
    // Customer operations
    async fn create_customer(&self, customer: &Customer) -> Result<Customer>;
    async fn get_customer(&self, customer_id: &str) -> Result<Customer>;
    async fn update_customer(&self, customer: &Customer) -> Result<Customer>;
    async fn delete_customer(&self, customer_id: &str) -> Result<bool>;
    async fn list_customers(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Customer>>;
    
    // Payment method operations
    async fn create_payment_method(&self, payment_method: &PaymentMethod) -> Result<PaymentMethod>;
    async fn get_payment_method(&self, payment_method_id: &str) -> Result<PaymentMethod>;
    async fn attach_payment_method(&self, payment_method_id: &str, customer_id: &str) -> Result<PaymentMethod>;
    async fn detach_payment_method(&self, payment_method_id: &str) -> Result<PaymentMethod>;
    
    // Charge operations
    async fn create_charge(&self, charge: &Charge) -> Result<Charge>;
    async fn get_charge(&self, charge_id: &str) -> Result<Charge>;
    async fn capture_charge(&self, charge_id: &str, amount: Option<Money>) -> Result<Charge>;
    async fn list_charges(&self, customer_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Charge>>;
    
    // Refund operations
    async fn create_refund(&self, refund: &Refund) -> Result<Refund>;
    async fn get_refund(&self, refund_id: &str) -> Result<Refund>;
    async fn list_refunds(&self, charge_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Refund>>;
    
    // Subscription operations
    async fn create_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    async fn get_subscription(&self, subscription_id: &str) -> Result<Subscription>;
    async fn update_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    async fn cancel_subscription(&self, subscription_id: &str, at_period_end: bool) -> Result<Subscription>;
    async fn list_subscriptions(&self, customer_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Subscription>>;
    
    // Webhook operations
    async fn verify_webhook(&self, payload: &[u8], signature: &str, secret: &str) -> Result<bool>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentFeature {
    OneTimePayments,
    RecurringPayments,
    Refunds,
    PartialRefunds,
    PaymentMethods,
    Webhooks,
    Disputes,
    ThreeDSecure,
    Cryptocurrency,
    BankTransfers,
    DigitalWallets,
}

// Synchronous wrapper for providers that don't have async implementations
pub trait SyncPaymentProvider: Send + Sync {
    fn name(&self) -> &str;
    fn supported_currencies(&self) -> Vec<String>;
    fn supported_features(&self) -> Vec<PaymentFeature>;
    
    fn create_customer(&self, customer: &Customer) -> Result<Customer>;
    fn get_customer(&self, customer_id: &str) -> Result<Customer>;
    fn update_customer(&self, customer: &Customer) -> Result<Customer>;
    fn delete_customer(&self, customer_id: &str) -> Result<bool>;
    fn list_customers(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Customer>>;
    
    fn create_payment_method(&self, payment_method: &PaymentMethod) -> Result<PaymentMethod>;
    fn get_payment_method(&self, payment_method_id: &str) -> Result<PaymentMethod>;
    fn attach_payment_method(&self, payment_method_id: &str, customer_id: &str) -> Result<PaymentMethod>;
    fn detach_payment_method(&self, payment_method_id: &str) -> Result<PaymentMethod>;
    
    fn create_charge(&self, charge: &Charge) -> Result<Charge>;
    fn get_charge(&self, charge_id: &str) -> Result<Charge>;
    fn capture_charge(&self, charge_id: &str, amount: Option<Money>) -> Result<Charge>;
    fn list_charges(&self, customer_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Charge>>;
    
    fn create_refund(&self, refund: &Refund) -> Result<Refund>;
    fn get_refund(&self, refund_id: &str) -> Result<Refund>;
    fn list_refunds(&self, charge_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Refund>>;
    
    fn create_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    fn get_subscription(&self, subscription_id: &str) -> Result<Subscription>;
    fn update_subscription(&self, subscription: &Subscription) -> Result<Subscription>;
    fn cancel_subscription(&self, subscription_id: &str, at_period_end: bool) -> Result<Subscription>;
    fn list_subscriptions(&self, customer_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Subscription>>;
    
    fn verify_webhook(&self, payload: &[u8], signature: &str, secret: &str) -> Result<bool>;
}