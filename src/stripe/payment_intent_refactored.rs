use serde::{Deserialize, Serialize};
use crate::error::{Result, PayupError};
use crate::http_client::{HttpClient, RequestBuilder};
use crate::config::api::STRIPE_BASE_URL;
use crate::builders::{ParameterBuilder, PageRequest};
use super::Auth;
use std::collections::HashMap;

// Re-export existing types from the original file
pub use super::payment_intent::{
    PaymentIntentStatus, ConfirmationMethod, CaptureMethod, SetupFutureUsage,
    AutomaticPaymentMethods, ShippingDetails, Address, NextAction, RedirectToUrl,
    PaymentMethodOptions, CardOptions, UsBankAccountOptions, AmountDetails, Tip,
    TransferData
};

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
    pub metadata: Option<HashMap<String, String>>,
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

/// Builder for PaymentIntent operations
pub struct PaymentIntentBuilder {
    client: HttpClient,
}

impl PaymentIntentBuilder {
    pub fn new(auth: &Auth) -> Self {
        let client = HttpClient::new(STRIPE_BASE_URL)
            .with_bearer_auth(&auth.secret);
        Self { client }
    }

    pub async fn create(&self, params: CreateParams) -> Result<PaymentIntent> {
        RequestBuilder::post("payment_intents")
            .form(params.to_form_params())
            .send(&self.client)
            .await
    }

    pub async fn retrieve(&self, id: &str) -> Result<PaymentIntent> {
        RequestBuilder::get(&format!("payment_intents/{}", id))
            .send(&self.client)
            .await
    }

    pub async fn update(&self, id: &str, params: UpdateParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}", id))
            .form(params.to_form_params())
            .send(&self.client)
            .await
    }

    pub async fn confirm(&self, id: &str, params: ConfirmParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/confirm", id))
            .form(params.to_form_params())
            .send(&self.client)
            .await
    }

    pub async fn capture(&self, id: &str, params: CaptureParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/capture", id))
            .form(params.to_form_params())
            .send(&self.client)
            .await
    }

    pub async fn cancel(&self, id: &str, params: CancelParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/cancel", id))
            .form(params.to_form_params())
            .send(&self.client)
            .await
    }

    pub async fn list(&self, params: ListParams) -> Result<PaymentIntentList> {
        RequestBuilder::get("payment_intents")
            .query("limit", params.limit.unwrap_or(10).to_string())
            .query_opt("starting_after", params.starting_after)
            .query_opt("ending_before", params.ending_before)
            .query_opt("customer", params.customer)
            .send(&self.client)
            .await
    }

    pub fn create_blocking(&self, params: CreateParams) -> Result<PaymentIntent> {
        RequestBuilder::post("payment_intents")
            .form(params.to_form_params())
            .send_blocking(&self.client)
    }

    pub fn retrieve_blocking(&self, id: &str) -> Result<PaymentIntent> {
        RequestBuilder::get(&format!("payment_intents/{}", id))
            .send_blocking(&self.client)
    }

    pub fn update_blocking(&self, id: &str, params: UpdateParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}", id))
            .form(params.to_form_params())
            .send_blocking(&self.client)
    }

    pub fn confirm_blocking(&self, id: &str, params: ConfirmParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/confirm", id))
            .form(params.to_form_params())
            .send_blocking(&self.client)
    }

    pub fn capture_blocking(&self, id: &str, params: CaptureParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/capture", id))
            .form(params.to_form_params())
            .send_blocking(&self.client)
    }

    pub fn cancel_blocking(&self, id: &str, params: CancelParams) -> Result<PaymentIntent> {
        RequestBuilder::post(&format!("payment_intents/{}/cancel", id))
            .form(params.to_form_params())
            .send_blocking(&self.client)
    }
}

/// Fluent builder for create parameters
#[derive(Debug, Clone, Default)]
pub struct CreateParams {
    amount: i64,
    currency: String,
    automatic_payment_methods: Option<AutomaticPaymentMethods>,
    capture_method: Option<CaptureMethod>,
    confirmation_method: Option<ConfirmationMethod>,
    customer: Option<String>,
    description: Option<String>,
    metadata: Option<HashMap<String, String>>,
    payment_method: Option<String>,
    payment_method_options: Option<PaymentMethodOptions>,
    payment_method_types: Option<Vec<String>>,
    receipt_email: Option<String>,
    setup_future_usage: Option<SetupFutureUsage>,
    shipping: Option<ShippingDetails>,
    statement_descriptor: Option<String>,
    statement_descriptor_suffix: Option<String>,
    transfer_data: Option<TransferData>,
    transfer_group: Option<String>,
}

impl CreateParams {
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
            ..Default::default()
        }
    }

    pub fn automatic_payment_methods(mut self, apm: AutomaticPaymentMethods) -> Self {
        self.automatic_payment_methods = Some(apm);
        self
    }

    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn payment_method(mut self, pm: impl Into<String>) -> Self {
        self.payment_method = Some(pm.into());
        self
    }

    pub fn receipt_email(mut self, email: impl Into<String>) -> Self {
        self.receipt_email = Some(email.into());
        self
    }

    fn to_form_params(&self) -> Vec<(String, String)> {
        let mut builder = ParameterBuilder::new()
            .add_number("amount", self.amount)
            .add("currency", self.currency.clone());

        if let Some(apm) = &self.automatic_payment_methods {
            builder = builder
                .add_bool("automatic_payment_methods[enabled]", apm.enabled);
            if let Some(redirects) = &apm.allow_redirects {
                builder = builder.add("automatic_payment_methods[allow_redirects]", redirects.clone());
            }
        }

        builder = builder
            .add_opt("customer", self.customer.clone())
            .add_opt("description", self.description.clone())
            .add_opt("payment_method", self.payment_method.clone())
            .add_opt("receipt_email", self.receipt_email.clone())
            .add_opt("statement_descriptor", self.statement_descriptor.clone())
            .add_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.clone())
            .add_opt("transfer_group", self.transfer_group.clone());

        if let Some(metadata) = &self.metadata {
            builder = builder.add_metadata(metadata.clone());
        }

        if let Some(types) = &self.payment_method_types {
            for (i, pm_type) in types.iter().enumerate() {
                builder = builder.add(format!("payment_method_types[{}]", i), pm_type.clone());
            }
        }

        builder.build()
    }
}

/// Fluent builder for update parameters
#[derive(Debug, Clone, Default)]
pub struct UpdateParams {
    amount: Option<i64>,
    currency: Option<String>,
    customer: Option<String>,
    description: Option<String>,
    metadata: Option<HashMap<String, String>>,
    payment_method: Option<String>,
    payment_method_options: Option<PaymentMethodOptions>,
    payment_method_types: Option<Vec<String>>,
    receipt_email: Option<String>,
    setup_future_usage: Option<SetupFutureUsage>,
    shipping: Option<ShippingDetails>,
    statement_descriptor: Option<String>,
    statement_descriptor_suffix: Option<String>,
    transfer_data: Option<TransferData>,
    transfer_group: Option<String>,
}

impl UpdateParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    fn to_form_params(&self) -> Vec<(String, String)> {
        let mut builder = ParameterBuilder::new();
        
        builder = builder
            .add_opt_number("amount", self.amount)
            .add_opt("currency", self.currency.clone())
            .add_opt("customer", self.customer.clone())
            .add_opt("description", self.description.clone())
            .add_opt("payment_method", self.payment_method.clone())
            .add_opt("receipt_email", self.receipt_email.clone())
            .add_opt("statement_descriptor", self.statement_descriptor.clone())
            .add_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.clone())
            .add_opt("transfer_group", self.transfer_group.clone());

        if let Some(metadata) = &self.metadata {
            builder = builder.add_metadata(metadata.clone());
        }

        builder.build()
    }
}

/// Fluent builder for confirm parameters
#[derive(Debug, Clone, Default)]
pub struct ConfirmParams {
    payment_method: Option<String>,
    payment_method_options: Option<PaymentMethodOptions>,
    return_url: Option<String>,
    setup_future_usage: Option<SetupFutureUsage>,
    shipping: Option<ShippingDetails>,
}

impl ConfirmParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn payment_method(mut self, pm: impl Into<String>) -> Self {
        self.payment_method = Some(pm.into());
        self
    }

    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }

    fn to_form_params(&self) -> Vec<(String, String)> {
        ParameterBuilder::new()
            .add_opt("payment_method", self.payment_method.clone())
            .add_opt("return_url", self.return_url.clone())
            .build()
    }
}

/// Fluent builder for capture parameters
#[derive(Debug, Clone, Default)]
pub struct CaptureParams {
    amount_to_capture: Option<i64>,
    application_fee_amount: Option<i64>,
    statement_descriptor: Option<String>,
    statement_descriptor_suffix: Option<String>,
    transfer_data: Option<TransferData>,
}

impl CaptureParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn amount_to_capture(mut self, amount: i64) -> Self {
        self.amount_to_capture = Some(amount);
        self
    }

    fn to_form_params(&self) -> Vec<(String, String)> {
        ParameterBuilder::new()
            .add_opt_number("amount_to_capture", self.amount_to_capture)
            .add_opt_number("application_fee_amount", self.application_fee_amount)
            .add_opt("statement_descriptor", self.statement_descriptor.clone())
            .add_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.clone())
            .build()
    }
}

/// Fluent builder for cancel parameters
#[derive(Debug, Clone, Default)]
pub struct CancelParams {
    cancellation_reason: Option<String>,
}

impl CancelParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancellation_reason(mut self, reason: impl Into<String>) -> Self {
        self.cancellation_reason = Some(reason.into());
        self
    }

    fn to_form_params(&self) -> Vec<(String, String)> {
        ParameterBuilder::new()
            .add_opt("cancellation_reason", self.cancellation_reason.clone())
            .build()
    }
}

/// Parameters for listing payment intents
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    pub limit: Option<usize>,
    pub starting_after: Option<String>,
    pub ending_before: Option<String>,
    pub customer: Option<String>,
}

/// List response for payment intents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntentList {
    pub object: String,
    pub data: Vec<PaymentIntent>,
    pub has_more: bool,
    pub url: String,
}

impl PaymentIntent {
    /// Create a new builder for PaymentIntent operations
    pub fn builder(auth: &Auth) -> PaymentIntentBuilder {
        PaymentIntentBuilder::new(auth)
    }

    /// Convenience method for creating a payment intent
    pub async fn create(auth: &Auth, amount: i64, currency: impl Into<String>) -> Result<Self> {
        let params = CreateParams::new(amount, currency);
        Self::builder(auth).create(params).await
    }

    /// Convenience method for retrieving a payment intent
    pub async fn retrieve(auth: &Auth, id: &str) -> Result<Self> {
        Self::builder(auth).retrieve(id).await
    }
}