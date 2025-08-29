use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{SquareClient, Money};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub amount_money: Option<Money>,
    pub tip_money: Option<Money>,
    pub total_money: Option<Money>,
    pub app_fee_money: Option<Money>,
    pub status: Option<String>,
    pub source_type: Option<String>,
    pub card_details: Option<CardPaymentDetails>,
    pub location_id: Option<String>,
    pub order_id: Option<String>,
    pub reference_id: Option<String>,
    pub customer_id: Option<String>,
    pub refund_ids: Option<Vec<String>>,
    pub receipt_number: Option<String>,
    pub receipt_url: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPaymentDetails {
    pub status: Option<String>,
    pub card: Option<Card>,
    pub entry_method: Option<String>,
    pub cvv_status: Option<String>,
    pub avs_status: Option<String>,
    pub statement_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub card_brand: Option<String>,
    pub last_4: Option<String>,
    pub exp_month: Option<i32>,
    pub exp_year: Option<i32>,
    pub fingerprint: Option<String>,
    pub card_type: Option<String>,
    pub prepaid_type: Option<String>,
    pub bin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub source_id: String,
    pub idempotency_key: String,
    pub amount_money: Money,
    pub tip_money: Option<Money>,
    pub app_fee_money: Option<Money>,
    pub autocomplete: Option<bool>,
    pub customer_id: Option<String>,
    pub location_id: Option<String>,
    pub reference_id: Option<String>,
    pub note: Option<String>,
    pub order_id: Option<String>,
    pub verification_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentRequest {
    pub payment: PaymentUpdate,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentUpdate {
    pub tip_money: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletePaymentRequest {
    pub version_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPaymentRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Option<String>,
    pub status: Option<String>,
    pub location_id: Option<String>,
    pub amount_money: Money,
    pub app_fee_money: Option<Money>,
    pub payment_id: Option<String>,
    pub order_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPaymentRequest {
    pub idempotency_key: String,
    pub amount_money: Money,
    pub app_fee_money: Option<Money>,
    pub payment_id: String,
    pub reason: Option<String>,
}

impl Payment {
    pub fn create(client: &SquareClient, request: &CreatePaymentRequest) -> Result<Self> {
        client.post("/v2/payments", request)
    }

    pub async fn async_create(client: &SquareClient, request: &CreatePaymentRequest) -> Result<Self> {
        client.async_post("/v2/payments", request).await
    }

    pub fn get(client: &SquareClient, payment_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}", payment_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &SquareClient, payment_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}", payment_id);
        client.async_get(&endpoint).await
    }

    pub fn update(client: &SquareClient, payment_id: &str, request: &UpdatePaymentRequest) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}", payment_id);
        client.put(&endpoint, request)
    }

    pub fn complete(client: &SquareClient, payment_id: &str, request: &CompletePaymentRequest) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}/complete", payment_id);
        client.post(&endpoint, request)
    }

    pub async fn async_complete(client: &SquareClient, payment_id: &str, request: &CompletePaymentRequest) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}/complete", payment_id);
        client.async_post(&endpoint, request).await
    }

    pub fn cancel(client: &SquareClient, payment_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/{}/cancel", payment_id);
        client.post(&endpoint, &CancelPaymentRequest { reason: None })
    }

    pub fn list(client: &SquareClient, location_id: Option<&str>, limit: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/payments?");
        if let Some(loc) = location_id {
            endpoint.push_str(&format!("location_id={}&", loc));
        }
        if let Some(lim) = limit {
            endpoint.push_str(&format!("limit={}", lim));
        }
        client.get(&endpoint)
    }
    
    pub async fn async_list(client: &SquareClient, location_id: Option<&str>, limit: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/payments?");
        if let Some(loc) = location_id {
            endpoint.push_str(&format!("location_id={}&", loc));
        }
        if let Some(lim) = limit {
            endpoint.push_str(&format!("limit={}", lim));
        }
        client.async_get(&endpoint).await
    }
}

impl Refund {
    pub fn create(client: &SquareClient, request: &RefundPaymentRequest) -> Result<Self> {
        client.post("/v2/refunds", request)
    }

    pub async fn async_create(client: &SquareClient, request: &RefundPaymentRequest) -> Result<Self> {
        client.async_post("/v2/refunds", request).await
    }

    pub fn get(client: &SquareClient, refund_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/refunds/{}", refund_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &SquareClient, refund_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/refunds/{}", refund_id);
        client.async_get(&endpoint).await
    }

    pub fn list(client: &SquareClient, location_id: Option<&str>, limit: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/refunds?");
        if let Some(loc) = location_id {
            endpoint.push_str(&format!("location_id={}&", loc));
        }
        if let Some(lim) = limit {
            endpoint.push_str(&format!("limit={}", lim));
        }
        client.get(&endpoint)
    }
    
    pub async fn async_list(client: &SquareClient, location_id: Option<&str>, limit: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/refunds?");
        if let Some(loc) = location_id {
            endpoint.push_str(&format!("location_id={}&", loc));
        }
        if let Some(lim) = limit {
            endpoint.push_str(&format!("limit={}", lim));
        }
        client.async_get(&endpoint).await
    }
}

// Helper function to create a simple payment
pub fn create_simple_payment(
    source_id: &str,
    amount: i64,
    currency: &str,
    idempotency_key: &str,
) -> CreatePaymentRequest {
    CreatePaymentRequest {
        source_id: source_id.to_string(),
        idempotency_key: idempotency_key.to_string(),
        amount_money: Money {
            amount,
            currency: currency.to_string(),
        },
        tip_money: None,
        app_fee_money: None,
        autocomplete: Some(true),
        customer_id: None,
        location_id: None,
        reference_id: None,
        note: None,
        order_id: None,
        verification_token: None,
    }
}