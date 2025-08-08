pub mod auth;
pub mod client;
pub mod orders;
pub mod payments;
pub mod subscriptions;
pub mod webhooks;

use serde::{Deserialize, Serialize};
use crate::error::Result;

pub use auth::PayPalAuth;
pub use client::PayPalClient;

// PayPal API endpoints
pub const PAYPAL_SANDBOX_URL: &str = "https://api.sandbox.paypal.com";
pub const PAYPAL_LIVE_URL: &str = "https://api.paypal.com";

// PayPal common types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalMoney {
    pub currency_code: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalAddress {
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub admin_area_2: Option<String>, // City
    pub admin_area_1: Option<String>, // State
    pub postal_code: Option<String>,
    pub country_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalName {
    pub given_name: Option<String>,
    pub surname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalPhone {
    pub phone_type: Option<String>,
    pub phone_number: PayPalPhoneNumber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalPhoneNumber {
    pub national_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalPayer {
    pub name: Option<PayPalName>,
    pub email_address: Option<String>,
    pub payer_id: Option<String>,
    pub phone: Option<PayPalPhone>,
    pub birth_date: Option<String>,
    pub tax_info: Option<PayPalTaxInfo>,
    pub address: Option<PayPalAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalTaxInfo {
    pub tax_id: Option<String>,
    pub tax_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalLink {
    pub href: String,
    pub rel: String,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayPalEnvironment {
    Sandbox,
    Live,
}

impl PayPalEnvironment {
    pub fn base_url(&self) -> &str {
        match self {
            PayPalEnvironment::Sandbox => PAYPAL_SANDBOX_URL,
            PayPalEnvironment::Live => PAYPAL_LIVE_URL,
        }
    }
}

// PayPal configuration
#[derive(Debug, Clone)]
pub struct PayPalConfig {
    pub client_id: String,
    pub client_secret: String,
    pub environment: PayPalEnvironment,
    pub webhook_id: Option<String>,
}