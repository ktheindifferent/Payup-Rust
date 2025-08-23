use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::rate_limiter::get_rate_limiter;
use super::Auth;
use std::collections::HashMap;

/// Type of payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    CardPresent,
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    CashApp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    Zip,
}

/// Billing details for a payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Card details for payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<CardChecks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CardNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<ThreeDSecureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Wallet>,
}

/// Card checks result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardChecks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
}

/// Card networks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardNetworks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}

/// 3D Secure usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<bool>,
}

/// Wallet type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<serde_json::Value>,
}

/// Stripe Payment Method object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: Option<String>,
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodType>,
    // Additional payment method type specific fields can be added here
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<serde_json::Value>,
}

/// Parameters for creating a payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodParams {
    #[serde(rename = "type")]
    pub payment_method_type: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCardParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    // Add other payment method type specific parameters as needed
}

/// Parameters for creating a card payment method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardParams {
    pub number: String,
    pub exp_month: i32,
    pub exp_year: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
}

/// Parameters for attaching a payment method to a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachPaymentMethodParams {
    pub customer: String,
}

impl PaymentMethod {
    pub fn new() -> Self {
        PaymentMethod {
            id: None,
            object: None,
            allow_redisplay: None,
            billing_details: None,
            card: None,
            created: None,
            customer: None,
            livemode: None,
            metadata: None,
            payment_method_type: None,
            acss_debit: None,
            affirm: None,
            afterpay_clearpay: None,
            alipay: None,
            au_becs_debit: None,
            bacs_debit: None,
            bancontact: None,
            blik: None,
            boleto: None,
            cashapp: None,
            customer_balance: None,
            eps: None,
            fpx: None,
            giropay: None,
            grabpay: None,
            ideal: None,
            interac_present: None,
            klarna: None,
            konbini: None,
            link: None,
            oxxo: None,
            p24: None,
            paynow: None,
            paypal: None,
            pix: None,
            promptpay: None,
            revolut_pay: None,
            sepa_debit: None,
            sofort: None,
            us_bank_account: None,
            wechat_pay: None,
            zip: None,
        }
    }

    /// Create a new payment method
    pub fn create(auth: &Auth, params: CreatePaymentMethodParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.stripe.com/v1/payment_methods")
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_method: PaymentMethod = response.json()?;
        Ok(payment_method)
    }

    /// Create a new payment method (async)
    pub async fn create_async(auth: &Auth, params: CreatePaymentMethodParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post("https://api.stripe.com/v1/payment_methods")
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_method: PaymentMethod = response.json().await?;
                Ok(payment_method)
            }
        }).await
    }

    /// Retrieve a payment method by ID
    pub fn retrieve(auth: &Auth, payment_method_id: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&format!("https://api.stripe.com/v1/payment_methods/{}", payment_method_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let payment_method: PaymentMethod = response.json()?;
        Ok(payment_method)
    }

    /// Retrieve a payment method by ID (async)
    pub async fn retrieve_async(auth: &Auth, payment_method_id: &str) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_method_id = payment_method_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_method_id = payment_method_id.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .get(&format!("https://api.stripe.com/v1/payment_methods/{}", payment_method_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                let payment_method: PaymentMethod = response.json().await?;
                Ok(payment_method)
            }
        }).await
    }

    /// Attach a payment method to a customer
    pub fn attach(auth: &Auth, payment_method_id: &str, customer_id: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let params = AttachPaymentMethodParams {
            customer: customer_id.to_string(),
        };
        
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_methods/{}/attach", payment_method_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_method: PaymentMethod = response.json()?;
        Ok(payment_method)
    }

    /// Attach a payment method to a customer (async)
    pub async fn attach_async(auth: &Auth, payment_method_id: &str, customer_id: &str) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_method_id = payment_method_id.to_string();
        let customer_id = customer_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_method_id = payment_method_id.clone();
            let customer_id = customer_id.clone();
            async move {
                let client = reqwest::Client::new();
                let params = AttachPaymentMethodParams {
                    customer: customer_id,
                };
                
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_methods/{}/attach", payment_method_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_method: PaymentMethod = response.json().await?;
                Ok(payment_method)
            }
        }).await
    }

    /// Detach a payment method from a customer
    pub fn detach(auth: &Auth, payment_method_id: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_methods/{}/detach", payment_method_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let payment_method: PaymentMethod = response.json()?;
        Ok(payment_method)
    }

    /// Detach a payment method from a customer (async)
    pub async fn detach_async(auth: &Auth, payment_method_id: &str) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_method_id = payment_method_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_method_id = payment_method_id.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_methods/{}/detach", payment_method_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                let payment_method: PaymentMethod = response.json().await?;
                Ok(payment_method)
            }
        }).await
    }
}