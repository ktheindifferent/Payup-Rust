use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::rate_limiter::get_rate_limiter;
use super::Auth;

/// Status of a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded,
}

/// Confirmation method for payment intents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    Automatic,
    Manual,
}

/// Capture method for payment intents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

/// Setup future usage for payment intents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupFutureUsage {
    OnSession,
    OffSession,
}

/// Automatic payment methods configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticPaymentMethods {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_redirects: Option<String>,
}

/// Shipping information for payment intents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
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

/// Next action for payment intents requiring additional steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<RedirectToUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits: Option<serde_json::Value>,
}

/// Redirect to URL action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectToUrl {
    pub return_url: String,
    pub url: String,
}

/// Payment method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UsBankAccountOptions>,
}

/// Card payment method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
}

/// US Bank Account payment method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsBankAccountOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
}

/// Represents a Stripe Payment Intent object for handling payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub amount_capturable: Option<i64>,
    pub amount_details: Option<AmountDetails>,
    pub amount_received: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    pub capture_method: CaptureMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub confirmation_method: ConfirmationMethod,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_charge: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<NextAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    pub status: PaymentIntentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// Amount details breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<Tip>,
}

/// Tip details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tip {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

/// Transfer data for connected accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    pub destination: String,
}

/// Parameters for creating a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentIntentParams {
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<AutomaticPaymentMethods>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_method: Option<ConfirmationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// Parameters for updating a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentIntentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// Parameters for confirming a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmPaymentIntentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PaymentMethodOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<SetupFutureUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
}

/// Parameters for capturing a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePaymentIntentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
}

/// Parameters for canceling a payment intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPaymentIntentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
}

impl PaymentIntent {
    /// Create a new payment intent
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent, CreatePaymentIntentParams, AutomaticPaymentMethods};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let params = CreatePaymentIntentParams {
    ///     amount: 2000, // $20.00 in cents
    ///     currency: "usd".to_string(),
    ///     automatic_payment_methods: Some(AutomaticPaymentMethods { 
    ///         enabled: true, 
    ///         allow_redirects: None 
    ///     }),
    ///     description: Some("Payment for services".to_string()),
    ///     customer: None,
    ///     metadata: None,
    ///     payment_method: None,
    ///     payment_method_options: None,
    ///     payment_method_types: None,
    ///     receipt_email: None,
    ///     setup_future_usage: None,
    ///     shipping: None,
    ///     statement_descriptor: None,
    ///     statement_descriptor_suffix: None,
    ///     transfer_data: None,
    ///     transfer_group: None,
    ///     capture_method: None,
    ///     confirmation_method: None,
    /// };
    /// let payment_intent = PaymentIntent::create(&auth, params)?;
    /// ```
    pub fn create(auth: &Auth, params: CreatePaymentIntentParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.stripe.com/v1/payment_intents")
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Create a new payment intent (async)
    pub async fn create_async(auth: &Auth, params: CreatePaymentIntentParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post("https://api.stripe.com/v1/payment_intents")
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// Retrieve a payment intent by ID
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let payment_intent = PaymentIntent::retrieve(&auth, "pi_1234567890")?;
    /// ```
    pub fn retrieve(auth: &Auth, payment_intent_id: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&format!("https://api.stripe.com/v1/payment_intents/{}", payment_intent_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Retrieve a payment intent by ID (async)
    pub async fn retrieve_async(auth: &Auth, payment_intent_id: &str) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_intent_id = payment_intent_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_intent_id = payment_intent_id.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .get(&format!("https://api.stripe.com/v1/payment_intents/{}", payment_intent_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// Update a payment intent
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent, UpdatePaymentIntentParams};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let params = UpdatePaymentIntentParams {
    ///     amount: Some(2500), // Update amount to $25.00
    ///     description: Some("Updated payment for services".to_string()),
    ///     ..Default::default()
    /// };
    /// let payment_intent = PaymentIntent::update(&auth, "pi_1234567890", params)?;
    /// ```
    pub fn update(auth: &Auth, payment_intent_id: &str, params: UpdatePaymentIntentParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_intents/{}", payment_intent_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Update a payment intent (async)
    pub async fn update_async(auth: &Auth, payment_intent_id: &str, params: UpdatePaymentIntentParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_intent_id = payment_intent_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_intent_id = payment_intent_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_intents/{}", payment_intent_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// Confirm a payment intent
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent, ConfirmPaymentIntentParams};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let params = ConfirmPaymentIntentParams {
    ///     payment_method: Some("pm_1234567890".to_string()),
    ///     return_url: Some("https://example.com/return".to_string()),
    ///     payment_method_options: None,
    ///     setup_future_usage: None,
    ///     shipping: None,
    /// };
    /// let payment_intent = PaymentIntent::confirm(&auth, "pi_1234567890", params)?;
    /// ```
    pub fn confirm(auth: &Auth, payment_intent_id: &str, params: ConfirmPaymentIntentParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_intents/{}/confirm", payment_intent_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Confirm a payment intent (async)
    pub async fn confirm_async(auth: &Auth, payment_intent_id: &str, params: ConfirmPaymentIntentParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_intent_id = payment_intent_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_intent_id = payment_intent_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_intents/{}/confirm", payment_intent_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// Capture a payment intent
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent, CapturePaymentIntentParams};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let params = CapturePaymentIntentParams {
    ///     amount_to_capture: Some(1500), // Capture $15.00 of a larger authorized amount
    ///     application_fee_amount: None,
    ///     statement_descriptor: None,
    ///     statement_descriptor_suffix: None,
    ///     transfer_data: None,
    /// };
    /// let payment_intent = PaymentIntent::capture(&auth, "pi_1234567890", params)?;
    /// ```
    pub fn capture(auth: &Auth, payment_intent_id: &str, params: CapturePaymentIntentParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_intents/{}/capture", payment_intent_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Capture a payment intent (async)
    pub async fn capture_async(auth: &Auth, payment_intent_id: &str, params: CapturePaymentIntentParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_intent_id = payment_intent_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_intent_id = payment_intent_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_intents/{}/capture", payment_intent_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// Cancel a payment intent
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent, CancelPaymentIntentParams};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let params = CancelPaymentIntentParams {
    ///     cancellation_reason: Some("requested_by_customer".to_string()),
    /// };
    /// let payment_intent = PaymentIntent::cancel(&auth, "pi_1234567890", params)?;
    /// ```
    pub fn cancel(auth: &Auth, payment_intent_id: &str, params: CancelPaymentIntentParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/payment_intents/{}/cancel", payment_intent_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let payment_intent: PaymentIntent = response.json()?;
        Ok(payment_intent)
    }

    /// Cancel a payment intent (async)
    pub async fn cancel_async(auth: &Auth, payment_intent_id: &str, params: CancelPaymentIntentParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let payment_intent_id = payment_intent_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let payment_intent_id = payment_intent_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/payment_intents/{}/cancel", payment_intent_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let payment_intent: PaymentIntent = response.json().await?;
                Ok(payment_intent)
            }
        }).await
    }

    /// List all payment intents
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, PaymentIntent};
    /// 
    /// let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
    /// let payment_intents = PaymentIntent::list(&auth, Some(10))?; // Get up to 10 payment intents
    /// ```
    pub fn list(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let client = reqwest::blocking::Client::new();
        let mut url = "https://api.stripe.com/v1/payment_intents".to_string();
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        #[derive(Deserialize)]
        struct PaymentIntentList {
            data: Vec<PaymentIntent>,
        }
        
        let list: PaymentIntentList = response.json()?;
        Ok(list.data)
    }

    /// List all payment intents (async)
    pub async fn list_async(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let rate_limiter = get_rate_limiter();
        
        rate_limiter.execute_with_retry_async("stripe", move || async move {
            let client = reqwest::Client::new();
            let mut url = "https://api.stripe.com/v1/payment_intents".to_string();
            if let Some(limit) = limit {
                url = format!("{}?limit={}", url, limit);
            }
            
            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", auth.secret))
                .send()
                .await?;
            
            #[derive(Deserialize)]
            struct PaymentIntentList {
                data: Vec<PaymentIntent>,
            }
            
            let list: PaymentIntentList = response.json().await?;
            Ok(list.data)
        }).await
    }
}

impl Default for UpdatePaymentIntentParams {
    fn default() -> Self {
        Self {
            amount: None,
            currency: None,
            customer: None,
            description: None,
            metadata: None,
            payment_method: None,
            payment_method_options: None,
            payment_method_types: None,
            receipt_email: None,
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        }
    }
}

impl Default for CreatePaymentIntentParams {
    fn default() -> Self {
        Self {
            amount: 0,
            currency: String::new(),
            automatic_payment_methods: None,
            capture_method: None,
            confirmation_method: None,
            customer: None,
            description: None,
            metadata: None,
            payment_method: None,
            payment_method_options: None,
            payment_method_types: None,
            receipt_email: None,
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        }
    }
}