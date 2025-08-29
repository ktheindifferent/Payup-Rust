use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::PayPalClient;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub event_type: String,
    pub resource_type: Option<String>,
    pub summary: Option<String>,
    pub resource: serde_json::Value,
    pub create_time: String,
    pub event_version: String,
    pub links: Option<Vec<super::PayPalLink>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerification {
    pub auth_algo: String,
    pub cert_url: String,
    pub transmission_id: String,
    pub transmission_sig: String,
    pub transmission_time: String,
    pub webhook_id: String,
    pub webhook_event: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookVerificationResponse {
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    Success,
    Failure,
}

// Common webhook event types
pub mod event_types {
    pub const PAYMENT_CAPTURE_COMPLETED: &str = "PAYMENT.CAPTURE.COMPLETED";
    pub const PAYMENT_CAPTURE_DENIED: &str = "PAYMENT.CAPTURE.DENIED";
    pub const PAYMENT_CAPTURE_REFUNDED: &str = "PAYMENT.CAPTURE.REFUNDED";
    pub const PAYMENT_CAPTURE_PENDING: &str = "PAYMENT.CAPTURE.PENDING";
    
    pub const CHECKOUT_ORDER_COMPLETED: &str = "CHECKOUT.ORDER.COMPLETED";
    pub const CHECKOUT_ORDER_APPROVED: &str = "CHECKOUT.ORDER.APPROVED";
    pub const CHECKOUT_ORDER_SAVED: &str = "CHECKOUT.ORDER.SAVED";
    
    pub const BILLING_SUBSCRIPTION_CREATED: &str = "BILLING.SUBSCRIPTION.CREATED";
    pub const BILLING_SUBSCRIPTION_ACTIVATED: &str = "BILLING.SUBSCRIPTION.ACTIVATED";
    pub const BILLING_SUBSCRIPTION_UPDATED: &str = "BILLING.SUBSCRIPTION.UPDATED";
    pub const BILLING_SUBSCRIPTION_EXPIRED: &str = "BILLING.SUBSCRIPTION.EXPIRED";
    pub const BILLING_SUBSCRIPTION_CANCELLED: &str = "BILLING.SUBSCRIPTION.CANCELLED";
    pub const BILLING_SUBSCRIPTION_SUSPENDED: &str = "BILLING.SUBSCRIPTION.SUSPENDED";
    pub const BILLING_SUBSCRIPTION_PAYMENT_FAILED: &str = "BILLING.SUBSCRIPTION.PAYMENT.FAILED";
    
    pub const BILLING_PLAN_CREATED: &str = "BILLING.PLAN.CREATED";
    pub const BILLING_PLAN_UPDATED: &str = "BILLING.PLAN.UPDATED";
    pub const BILLING_PLAN_ACTIVATED: &str = "BILLING.PLAN.ACTIVATED";
    pub const BILLING_PLAN_DEACTIVATED: &str = "BILLING.PLAN.DEACTIVATED";
}

impl WebhookEvent {
    pub fn parse(payload: &str) -> Result<Self> {
        serde_json::from_str(payload).map_err(PayupError::from)
    }

    pub fn verify(
        client: &mut PayPalClient,
        headers: HashMap<String, String>,
        body: &str,
        webhook_id: &str,
    ) -> Result<bool> {
        // Extract required headers
        let auth_algo = headers.get("paypal-auth-algo")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-auth-algo header".to_string()))?;
        let cert_url = headers.get("paypal-cert-url")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-cert-url header".to_string()))?;
        let transmission_id = headers.get("paypal-transmission-id")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-id header".to_string()))?;
        let transmission_sig = headers.get("paypal-transmission-sig")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-sig header".to_string()))?;
        let transmission_time = headers.get("paypal-transmission-time")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-time header".to_string()))?;

        let verification = WebhookVerification {
            auth_algo: auth_algo.clone(),
            cert_url: cert_url.clone(),
            transmission_id: transmission_id.clone(),
            transmission_sig: transmission_sig.clone(),
            transmission_time: transmission_time.clone(),
            webhook_id: webhook_id.to_string(),
            webhook_event: serde_json::from_str(body).map_err(PayupError::from)?,
        };

        let response: WebhookVerificationResponse = client.post(
            "/v1/notifications/verify-webhook-signature",
            &verification
        )?;

        Ok(matches!(response.verification_status, VerificationStatus::Success))
    }

    pub async fn async_verify(
        client: &mut PayPalClient,
        headers: HashMap<String, String>,
        body: &str,
        webhook_id: &str,
    ) -> Result<bool> {
        // Extract required headers
        let auth_algo = headers.get("paypal-auth-algo")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-auth-algo header".to_string()))?;
        let cert_url = headers.get("paypal-cert-url")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-cert-url header".to_string()))?;
        let transmission_id = headers.get("paypal-transmission-id")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-id header".to_string()))?;
        let transmission_sig = headers.get("paypal-transmission-sig")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-sig header".to_string()))?;
        let transmission_time = headers.get("paypal-transmission-time")
            .ok_or_else(|| PayupError::ValidationError("Missing paypal-transmission-time header".to_string()))?;

        let verification = WebhookVerification {
            auth_algo: auth_algo.clone(),
            cert_url: cert_url.clone(),
            transmission_id: transmission_id.clone(),
            transmission_sig: transmission_sig.clone(),
            transmission_time: transmission_time.clone(),
            webhook_id: webhook_id.to_string(),
            webhook_event: serde_json::from_str(body).map_err(PayupError::from)?,
        };

        let response: WebhookVerificationResponse = client.async_post(
            "/v1/notifications/verify-webhook-signature",
            &verification
        ).await?;

        Ok(matches!(response.verification_status, VerificationStatus::Success))
    }

    pub fn get_resource<T>(&self) -> Result<T> 
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_value(self.resource.clone()).map_err(PayupError::from)
    }
}

// Helper to handle webhook events
pub struct WebhookHandler {
    handlers: HashMap<String, Box<dyn Fn(&WebhookEvent) -> Result<()> + Send + Sync>>,
}

impl WebhookHandler {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on<F>(&mut self, event_type: &str, handler: F) -> &mut Self
    where
        F: Fn(&WebhookEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.handlers.insert(event_type.to_string(), Box::new(handler));
        self
    }

    pub fn handle(&self, event: &WebhookEvent) -> Result<()> {
        if let Some(handler) = self.handlers.get(&event.event_type) {
            handler(event)
        } else {
            // No handler registered for this event type
            Ok(())
        }
    }
}

// PayPal Webhook Handler with local signature verification capabilities
pub struct PayPalWebhookHandler {
    webhook_id: String,
}

impl PayPalWebhookHandler {
    pub fn new(webhook_id: String) -> Self {
        Self { webhook_id }
    }
    
    /// Extract PayPal webhook headers from HTTP headers
    pub fn extract_headers(http_headers: &[(String, String)]) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for (key, value) in http_headers {
            let lower_key = key.to_lowercase();
            if lower_key.starts_with("paypal-") {
                headers.insert(lower_key, value.clone());
            }
        }
        headers
    }
    
    /// Verify webhook using PayPal's API
    pub async fn verify_with_api(
        &self,
        client: &mut PayPalClient,
        headers: HashMap<String, String>,
        body: &str,
    ) -> Result<bool> {
        WebhookEvent::async_verify(client, headers, body, &self.webhook_id).await
    }
    
    /// Verify webhook using PayPal's API (synchronous)
    pub fn verify_with_api_sync(
        &self,
        client: &mut PayPalClient,
        headers: HashMap<String, String>,
        body: &str,
    ) -> Result<bool> {
        WebhookEvent::verify(client, headers, body, &self.webhook_id)
    }
    
    /// Parse and verify a webhook in one step
    pub async fn parse_and_verify(
        &self,
        client: &mut PayPalClient,
        headers: HashMap<String, String>,
        body: &str,
    ) -> Result<WebhookEvent> {
        // First verify the webhook
        let is_valid = self.verify_with_api(client, headers, body).await?;
        
        if !is_valid {
            return Err(PayupError::WebhookVerificationFailed(
                "PayPal webhook signature verification failed".to_string()
            ));
        }
        
        // Then parse the event
        WebhookEvent::parse(body)
    }
    
    /// Validate required headers are present
    pub fn validate_headers(headers: &HashMap<String, String>) -> Result<()> {
        const REQUIRED_HEADERS: &[&str] = &[
            "paypal-auth-algo",
            "paypal-cert-url",
            "paypal-transmission-id",
            "paypal-transmission-sig",
            "paypal-transmission-time",
        ];
        
        for header in REQUIRED_HEADERS {
            if !headers.contains_key(*header) {
                return Err(PayupError::ValidationError(
                    format!("Missing required header: {}", header)
                ));
            }
        }
        
        // Validate cert URL is from PayPal domain
        if let Some(cert_url) = headers.get("paypal-cert-url") {
            if !cert_url.contains("paypal.com") {
                return Err(PayupError::ValidationError(
                    "Invalid cert URL: must be from paypal.com domain".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Create headers HashMap from raw HTTP headers for convenience
    pub fn headers_from_slice(raw_headers: &[(&str, &str)]) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for (key, value) in raw_headers {
            headers.insert(key.to_lowercase(), value.to_string());
        }
        headers
    }
}