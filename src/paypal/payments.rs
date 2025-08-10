use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{PayPalClient, PayPalMoney, PayPalLink};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Option<String>,
    pub status: Option<PaymentStatus>,
    pub amount: PayPalMoney,
    pub invoice_id: Option<String>,
    pub custom_id: Option<String>,
    pub final_capture: Option<bool>,
    pub seller_protection: Option<SellerProtection>,
    pub links: Option<Vec<PayPalLink>>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Created,
    Captured,
    Completed,
    Declined,
    Denied,
    PartiallyRefunded,
    Pending,
    Refunded,
    Failed,
    Canceled,
    Voided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerProtection {
    pub status: Option<String>,
    pub dispute_categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    pub id: Option<String>,
    pub amount: Option<PayPalMoney>,
    pub invoice_id: Option<String>,
    pub note_to_payer: Option<String>,
    pub status: Option<RefundStatus>,
    pub links: Option<Vec<PayPalLink>>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    Canceled,
    Cancelled,
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub amount: Option<PayPalMoney>,
    pub invoice_id: Option<String>,
    pub note_to_payer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub id: Option<String>,
    pub status: Option<AuthorizationStatus>,
    pub amount: PayPalMoney,
    pub invoice_id: Option<String>,
    pub custom_id: Option<String>,
    pub seller_protection: Option<SellerProtection>,
    pub expiration_time: Option<String>,
    pub links: Option<Vec<PayPalLink>>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatus {
    Created,
    Captured,
    Denied,
    Expired,
    PartialyCaptured,
    PartiallyCreated,
    Voided,
    Pending,
}

impl Payment {
    pub fn get(client: &mut PayPalClient, capture_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/captures/{}", capture_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, capture_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/captures/{}", capture_id);
        client.async_get(&endpoint).await
    }

    pub fn refund(
        client: &mut PayPalClient,
        capture_id: &str,
        refund_request: Option<RefundRequest>,
    ) -> Result<Refund> {
        let endpoint = format!("/v2/payments/captures/{}/refund", capture_id);
        let body = refund_request.unwrap_or_else(|| RefundRequest {
            amount: None,
            invoice_id: None,
            note_to_payer: None,
        });
        client.post(&endpoint, &body)
    }

    pub async fn async_refund(
        client: &mut PayPalClient,
        capture_id: &str,
        refund_request: Option<RefundRequest>,
    ) -> Result<Refund> {
        let endpoint = format!("/v2/payments/captures/{}/refund", capture_id);
        let body = refund_request.unwrap_or_else(|| RefundRequest {
            amount: None,
            invoice_id: None,
            note_to_payer: None,
        });
        client.async_post(&endpoint, &body).await
    }
}

impl Refund {
    pub fn get(client: &mut PayPalClient, refund_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/refunds/{}", refund_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, refund_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/refunds/{}", refund_id);
        client.async_get(&endpoint).await
    }
}

impl Authorization {
    pub fn get(client: &mut PayPalClient, authorization_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/authorizations/{}", authorization_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &mut PayPalClient, authorization_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/payments/authorizations/{}", authorization_id);
        client.async_get(&endpoint).await
    }

    pub fn capture(
        client: &mut PayPalClient,
        authorization_id: &str,
        amount: Option<PayPalMoney>,
    ) -> Result<Payment> {
        let endpoint = format!("/v2/payments/authorizations/{}/capture", authorization_id);
        let body = serde_json::json!({
            "amount": amount,
            "final_capture": false
        });
        client.post(&endpoint, &body)
    }

    pub async fn async_capture(
        client: &mut PayPalClient,
        authorization_id: &str,
        amount: Option<PayPalMoney>,
    ) -> Result<Payment> {
        let endpoint = format!("/v2/payments/authorizations/{}/capture", authorization_id);
        let body = serde_json::json!({
            "amount": amount,
            "final_capture": false
        });
        client.async_post(&endpoint, &body).await
    }

    pub fn void(client: &mut PayPalClient, authorization_id: &str) -> Result<bool> {
        let endpoint = format!("/v2/payments/authorizations/{}/void", authorization_id);
        client.delete(&endpoint)
    }
}