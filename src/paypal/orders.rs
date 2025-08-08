use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{PayPalClient, PayPalMoney, PayPalPayer, PayPalLink, PayPalAddress};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub intent: OrderIntent,
    pub purchase_units: Vec<PurchaseUnit>,
    pub payer: Option<PayPalPayer>,
    pub status: Option<OrderStatus>,
    pub links: Option<Vec<PayPalLink>>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderIntent {
    Capture,
    Authorize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Created,
    Saved,
    Approved,
    Voided,
    Completed,
    PayerActionRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseUnit {
    pub reference_id: Option<String>,
    pub amount: PayPalMoney,
    pub payee: Option<Payee>,
    pub description: Option<String>,
    pub custom_id: Option<String>,
    pub invoice_id: Option<String>,
    pub items: Option<Vec<Item>>,
    pub shipping: Option<Shipping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub email_address: Option<String>,
    pub merchant_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub unit_amount: PayPalMoney,
    pub quantity: String,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub category: Option<ItemCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategory {
    DigitalGoods,
    PhysicalGoods,
    Donation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipping {
    pub name: Option<ShippingName>,
    pub address: Option<PayPalAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingName {
    pub full_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub amount: Option<PayPalMoney>,
    pub final_capture: Option<bool>,
    pub note_to_payer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    pub id: String,
    pub status: String,
    pub amount: PayPalMoney,
    pub final_capture: bool,
    pub create_time: String,
    pub update_time: String,
}

impl Order {
    pub fn new() -> Self {
        Self {
            id: None,
            intent: OrderIntent::Capture,
            purchase_units: Vec::new(),
            payer: None,
            status: None,
            links: None,
            create_time: None,
            update_time: None,
        }
    }

    pub fn create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.post("/v2/checkout/orders", self)
    }

    pub async fn async_create(&self, client: &mut PayPalClient) -> Result<Self> {
        client.async_post("/v2/checkout/orders", self).await
    }

    pub fn get(client: &mut PayPalClient, order_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/checkout/orders/{}", order_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, order_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/checkout/orders/{}", order_id);
        client.async_get(&endpoint).await
    }

    pub fn update(&self, client: &mut PayPalClient) -> Result<Self> {
        if let Some(id) = &self.id {
            let endpoint = format!("/v2/checkout/orders/{}", id);
            client.patch(&endpoint, self)
        } else {
            Err(crate::error::PayupError::ValidationError(
                "Order ID is required for update".to_string()
            ))
        }
    }

    pub fn capture(
        client: &mut PayPalClient,
        order_id: &str,
        capture_request: Option<CaptureRequest>,
    ) -> Result<CaptureResponse> {
        let endpoint = format!("/v2/checkout/orders/{}/capture", order_id);
        let body = capture_request.unwrap_or_else(|| CaptureRequest {
            amount: None,
            final_capture: None,
            note_to_payer: None,
        });
        client.post(&endpoint, &body)
    }

    pub async fn async_capture(
        client: &mut PayPalClient,
        order_id: &str,
        capture_request: Option<CaptureRequest>,
    ) -> Result<CaptureResponse> {
        let endpoint = format!("/v2/checkout/orders/{}/capture", order_id);
        let body = capture_request.unwrap_or_else(|| CaptureRequest {
            amount: None,
            final_capture: None,
            note_to_payer: None,
        });
        client.async_post(&endpoint, &body).await
    }

    pub fn authorize(
        client: &mut PayPalClient,
        order_id: &str,
    ) -> Result<Self> {
        let endpoint = format!("/v2/checkout/orders/{}/authorize", order_id);
        client.post(&endpoint, &serde_json::json!({}))
    }

    pub fn void(
        client: &mut PayPalClient,
        order_id: &str,
    ) -> Result<bool> {
        let endpoint = format!("/v2/checkout/orders/{}/void", order_id);
        client.delete(&endpoint)
    }
}

// Helper function to create a simple order
pub fn create_simple_order(amount: f64, currency: &str, description: Option<String>) -> Order {
    let mut order = Order::new();
    order.intent = OrderIntent::Capture;
    
    let purchase_unit = PurchaseUnit {
        reference_id: None,
        amount: PayPalMoney {
            currency_code: currency.to_string(),
            value: format!("{:.2}", amount),
        },
        payee: None,
        description,
        custom_id: None,
        invoice_id: None,
        items: None,
        shipping: None,
    };
    
    order.purchase_units.push(purchase_unit);
    order
}