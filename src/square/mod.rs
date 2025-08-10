pub mod auth;
pub mod client;
pub mod payments;
pub mod customers;
pub mod catalog;
pub mod provider;
pub mod webhooks;

use serde::{Deserialize, Serialize};

pub use auth::SquareAuth;
pub use client::SquareClient;
pub use provider::SquareProvider;
pub use webhooks::{
    SquareWebhookHandler, WebhookEvent, WebhookEventType, 
    WebhookEventHandler, WebhookEventData, WebhookNotification
};

// Square API endpoints
pub const SQUARE_SANDBOX_URL: &str = "https://connect.squareupsandbox.com";
pub const SQUARE_PRODUCTION_URL: &str = "https://connect.squareup.com";

// Square common types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub address_line_3: Option<String>,
    pub locality: Option<String>,
    pub sublocality: Option<String>,
    pub administrative_district_level_1: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub category: String,
    pub code: String,
    pub detail: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<Error>>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &str {
        match self {
            Environment::Sandbox => SQUARE_SANDBOX_URL,
            Environment::Production => SQUARE_PRODUCTION_URL,
        }
    }
}

// Square configuration
#[derive(Debug, Clone)]
pub struct SquareConfig {
    pub access_token: String,
    pub environment: Environment,
    pub location_id: Option<String>,
}