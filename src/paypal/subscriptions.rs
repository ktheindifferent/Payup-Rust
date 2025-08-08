use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{PayPalClient, PayPalMoney, PayPalLink};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: Option<String>,
    pub product_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<PlanStatus>,
    pub billing_cycles: Vec<BillingCycle>,
    pub payment_preferences: Option<PaymentPreferences>,
    pub taxes: Option<Taxes>,
    pub quantity_supported: Option<bool>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub links: Option<Vec<PayPalLink>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlanStatus {
    Created,
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCycle {
    pub frequency: Frequency,
    pub tenure_type: TenureType,
    pub sequence: i32,
    pub total_cycles: Option<i32>,
    pub pricing_scheme: Option<PricingScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frequency {
    pub interval_unit: IntervalUnit,
    pub interval_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntervalUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenureType {
    Regular,
    Trial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingScheme {
    pub fixed_price: Option<PayPalMoney>,
    pub tiers: Option<Vec<Tier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tier {
    pub starting_quantity: String,
    pub ending_quantity: Option<String>,
    pub amount: PayPalMoney,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPreferences {
    pub auto_bill_outstanding: Option<bool>,
    pub setup_fee: Option<PayPalMoney>,
    pub setup_fee_failure_action: Option<SetupFeeFailureAction>,
    pub payment_failure_threshold: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SetupFeeFailureAction {
    Continue,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taxes {
    pub percentage: String,
    pub inclusive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Option<String>,
    pub plan_id: String,
    pub status: Option<SubscriptionStatus>,
    pub status_change_note: Option<String>,
    pub status_update_time: Option<String>,
    pub subscriber: Option<Subscriber>,
    pub billing_info: Option<BillingInfo>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub links: Option<Vec<PayPalLink>>,
    pub start_time: Option<String>,
    pub quantity: Option<String>,
    pub auto_renewal: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    ApprovalPending,
    Approved,
    Active,
    Suspended,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub name: Option<SubscriberName>,
    pub email_address: Option<String>,
    pub payer_id: Option<String>,
    pub phone: Option<Phone>,
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberName {
    pub given_name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phone {
    pub phone_type: Option<String>,
    pub phone_number: PhoneNumber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub national_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub name: Option<SubscriberName>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub admin_area_2: Option<String>,
    pub admin_area_1: Option<String>,
    pub postal_code: Option<String>,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub outstanding_balance: Option<PayPalMoney>,
    pub cycle_executions: Option<Vec<CycleExecution>>,
    pub last_payment: Option<LastPayment>,
    pub next_billing_time: Option<String>,
    pub failed_payments_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleExecution {
    pub tenure_type: TenureType,
    pub sequence: i32,
    pub cycles_completed: i32,
    pub cycles_remaining: Option<i32>,
    pub current_pricing_scheme_version: Option<i32>,
    pub total_cycles: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastPayment {
    pub amount: PayPalMoney,
    pub time: String,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            id: None,
            product_id: String::new(),
            name: String::new(),
            description: None,
            status: None,
            billing_cycles: Vec::new(),
            payment_preferences: None,
            taxes: None,
            quantity_supported: None,
            create_time: None,
            update_time: None,
            links: None,
        }
    }

    pub fn create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.post("/v1/billing/plans", self)
    }

    pub async fn async_create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.async_post("/v1/billing/plans", self).await
    }

    pub fn get(client: &mut PayPalClient, plan_id: &str) -> Result<Self> {
        let endpoint = format!("/v1/billing/plans/{}", plan_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, plan_id: &str) -> Result<Self> {
        let endpoint = format!("/v1/billing/plans/{}", plan_id);
        client.async_get(&endpoint).await
    }

    pub fn list(client: &mut PayPalClient, page_size: Option<i32>, page: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v1/billing/plans?");
        if let Some(size) = page_size {
            endpoint.push_str(&format!("page_size={}&", size));
        }
        if let Some(p) = page {
            endpoint.push_str(&format!("page={}", p));
        }
        client.get(&endpoint)
    }

    pub fn update(&self, client: &mut PayPalClient) -> Result<bool> {
        if let Some(id) = &self.id {
            let endpoint = format!("/v1/billing/plans/{}", id);
            // PayPal uses PATCH with specific operations
            let operations = vec![
                serde_json::json!({
                    "op": "replace",
                    "path": "/description",
                    "value": self.description
                })
            ];
            client.patch(&endpoint, &operations)?;
            Ok(true)
        } else {
            Err(crate::error::PayupError::ValidationError(
                "Plan ID is required for update".to_string()
            ))
        }
    }

    pub fn activate(client: &mut PayPalClient, plan_id: &str) -> Result<bool> {
        let endpoint = format!("/v1/billing/plans/{}/activate", plan_id);
        client.post::<serde_json::Value, _>(&endpoint, &serde_json::json!({})).map(|_| true)
    }

    pub fn deactivate(client: &mut PayPalClient, plan_id: &str) -> Result<bool> {
        let endpoint = format!("/v1/billing/plans/{}/deactivate", plan_id);
        client.post::<serde_json::Value, _>(&endpoint, &serde_json::json!({})).map(|_| true)
    }
}

impl Subscription {
    pub fn new() -> Self {
        Self {
            id: None,
            plan_id: String::new(),
            status: None,
            status_change_note: None,
            status_update_time: None,
            subscriber: None,
            billing_info: None,
            create_time: None,
            update_time: None,
            links: None,
            start_time: None,
            quantity: None,
            auto_renewal: None,
        }
    }

    pub fn create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.post("/v1/billing/subscriptions", self)
    }

    pub async fn async_create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.async_post("/v1/billing/subscriptions", self).await
    }

    pub fn get(client: &mut PayPalClient, subscription_id: &str) -> Result<Self> {
        let endpoint = format!("/v1/billing/subscriptions/{}", subscription_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, subscription_id: &str) -> Result<Self> {
        let endpoint = format!("/v1/billing/subscriptions/{}", subscription_id);
        client.async_get(&endpoint).await
    }

    pub fn cancel(client: &mut PayPalClient, subscription_id: &str, reason: Option<String>) -> Result<bool> {
        let endpoint = format!("/v1/billing/subscriptions/{}/cancel", subscription_id);
        let body = serde_json::json!({
            "reason": reason.unwrap_or_else(|| "Customer requested cancellation".to_string())
        });
        client.post::<serde_json::Value, _>(&endpoint, &body).map(|_| true)
    }

    pub fn suspend(client: &mut PayPalClient, subscription_id: &str, reason: Option<String>) -> Result<bool> {
        let endpoint = format!("/v1/billing/subscriptions/{}/suspend", subscription_id);
        let body = serde_json::json!({
            "reason": reason.unwrap_or_else(|| "Suspended by admin".to_string())
        });
        client.post::<serde_json::Value, _>(&endpoint, &body).map(|_| true)
    }

    pub fn activate(client: &mut PayPalClient, subscription_id: &str, reason: Option<String>) -> Result<bool> {
        let endpoint = format!("/v1/billing/subscriptions/{}/activate", subscription_id);
        let body = serde_json::json!({
            "reason": reason.unwrap_or_else(|| "Reactivated by admin".to_string())
        });
        client.post::<serde_json::Value, _>(&endpoint, &body).map(|_| true)
    }
}