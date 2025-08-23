use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{PayupError, Result};

/// Stripe webhook event handler
pub struct StripeWebhookHandler {
    /// The webhook signing secret from Stripe Dashboard
    signing_secret: String,
    /// Tolerance for timestamp verification (default: 5 minutes)
    tolerance: i64,
}

/// Stripe webhook event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub object: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub created: i64,
    pub data: WebhookEventData,
    pub livemode: bool,
    pub pending_webhooks: i32,
    pub request: Option<WebhookRequest>,
    pub api_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventData {
    pub object: Value,
    pub previous_attributes: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRequest {
    pub id: Option<String>,
    pub idempotency_key: Option<String>,
}

/// Webhook signature header components
#[derive(Debug)]
struct SignatureHeader {
    timestamp: i64,
    signatures: Vec<String>,
}

impl StripeWebhookHandler {
    /// Create a new webhook handler with the signing secret
    pub fn new(signing_secret: String) -> Self {
        Self {
            signing_secret,
            tolerance: 300, // 5 minutes default
        }
    }

    /// Create a webhook handler with custom tolerance
    pub fn with_tolerance(signing_secret: String, tolerance: i64) -> Self {
        Self {
            signing_secret,
            tolerance,
        }
    }

    /// Verify webhook signature and parse the event
    pub fn construct_event(
        &self,
        payload: &str,
        signature_header: &str,
    ) -> Result<WebhookEvent> {
        // Verify the signature
        self.verify_signature(payload, signature_header)?;
        
        // Parse the event
        let event: WebhookEvent = serde_json::from_str(payload)
            .map_err(|e| PayupError::SerializationError(e))?;
        
        Ok(event)
    }

    /// Verify the webhook signature
    pub fn verify_signature(
        &self,
        payload: &str,
        signature_header: &str,
    ) -> Result<()> {
        let header = self.parse_signature_header(signature_header)?;
        
        // Check timestamp to prevent replay attacks
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| PayupError::GenericError(format!("System time error: {}", e)))?
            .as_secs() as i64;
        
        if (current_timestamp - header.timestamp).abs() > self.tolerance {
            return Err(PayupError::ValidationError(
                "Webhook timestamp outside tolerance window".to_string()
            ));
        }
        
        // Compute expected signature
        let signed_payload = format!("{}.{}", header.timestamp, payload);
        let expected_signature = self.compute_signature(&signed_payload)?;
        
        // Verify at least one signature matches
        let signature_found = header.signatures.iter().any(|sig| {
            self.secure_compare(sig, &expected_signature)
        });
        
        if !signature_found {
            return Err(PayupError::ValidationError(
                "Invalid webhook signature".to_string()
            ));
        }
        
        Ok(())
    }

    /// Parse the Stripe-Signature header
    fn parse_signature_header(&self, header: &str) -> Result<SignatureHeader> {
        let mut timestamp = None;
        let mut signatures = Vec::new();
        
        for element in header.split(' ') {
            let parts: Vec<&str> = element.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            match parts[0] {
                "t" => {
                    timestamp = parts[1].parse::<i64>().ok();
                }
                "v1" => {
                    signatures.push(parts[1].to_string());
                }
                _ => {} // Ignore other versions
            }
        }
        
        let timestamp = timestamp.ok_or_else(|| {
            PayupError::ValidationError("Invalid signature header: missing timestamp".to_string())
        })?;
        
        if signatures.is_empty() {
            return Err(PayupError::ValidationError(
                "Invalid signature header: missing signatures".to_string()
            ));
        }
        
        Ok(SignatureHeader {
            timestamp,
            signatures,
        })
    }

    /// Compute HMAC-SHA256 signature
    fn compute_signature(&self, payload: &str) -> Result<String> {
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(self.signing_secret.as_bytes())
            .map_err(|e| PayupError::GenericError(format!("HMAC error: {}", e)))?;
        
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());
        
        Ok(signature)
    }

    /// Constant-time string comparison to prevent timing attacks
    fn secure_compare(&self, a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        
        let mut result = 0u8;
        for i in 0..a.len() {
            result |= a_bytes[i] ^ b_bytes[i];
        }
        
        result == 0
    }
}

/// Common Stripe webhook event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventType {
    // Payment Intent Events
    PaymentIntentSucceeded,
    PaymentIntentFailed,
    PaymentIntentCanceled,
    PaymentIntentProcessing,
    PaymentIntentRequiresAction,
    
    // Charge Events
    ChargeSucceeded,
    ChargeFailed,
    ChargeRefunded,
    ChargeDisputed,
    
    // Customer Events
    CustomerCreated,
    CustomerUpdated,
    CustomerDeleted,
    CustomerSubscriptionCreated,
    CustomerSubscriptionUpdated,
    CustomerSubscriptionDeleted,
    
    // Invoice Events
    InvoiceCreated,
    InvoicePaid,
    InvoicePaymentFailed,
    InvoiceFinalized,
    
    // Subscription Events
    SubscriptionScheduleCreated,
    SubscriptionScheduleUpdated,
    SubscriptionScheduleCanceled,
    
    // Payout Events
    PayoutCreated,
    PayoutPaid,
    PayoutFailed,
    
    // Other Events
    AccountUpdated,
    Other(String),
}

impl From<&str> for WebhookEventType {
    fn from(event_type: &str) -> Self {
        match event_type {
            "payment_intent.succeeded" => Self::PaymentIntentSucceeded,
            "payment_intent.payment_failed" => Self::PaymentIntentFailed,
            "payment_intent.canceled" => Self::PaymentIntentCanceled,
            "payment_intent.processing" => Self::PaymentIntentProcessing,
            "payment_intent.requires_action" => Self::PaymentIntentRequiresAction,
            
            "charge.succeeded" => Self::ChargeSucceeded,
            "charge.failed" => Self::ChargeFailed,
            "charge.refunded" => Self::ChargeRefunded,
            "charge.dispute.created" => Self::ChargeDisputed,
            
            "customer.created" => Self::CustomerCreated,
            "customer.updated" => Self::CustomerUpdated,
            "customer.deleted" => Self::CustomerDeleted,
            "customer.subscription.created" => Self::CustomerSubscriptionCreated,
            "customer.subscription.updated" => Self::CustomerSubscriptionUpdated,
            "customer.subscription.deleted" => Self::CustomerSubscriptionDeleted,
            
            "invoice.created" => Self::InvoiceCreated,
            "invoice.paid" => Self::InvoicePaid,
            "invoice.payment_failed" => Self::InvoicePaymentFailed,
            "invoice.finalized" => Self::InvoiceFinalized,
            
            "subscription_schedule.created" => Self::SubscriptionScheduleCreated,
            "subscription_schedule.updated" => Self::SubscriptionScheduleUpdated,
            "subscription_schedule.canceled" => Self::SubscriptionScheduleCanceled,
            
            "payout.created" => Self::PayoutCreated,
            "payout.paid" => Self::PayoutPaid,
            "payout.failed" => Self::PayoutFailed,
            
            "account.updated" => Self::AccountUpdated,
            
            other => Self::Other(other.to_string()),
        }
    }
}

impl WebhookEvent {
    /// Get the event type as an enum
    pub fn event_type_enum(&self) -> WebhookEventType {
        WebhookEventType::from(self.event_type.as_str())
    }
    
    /// Extract the object ID from the event data
    pub fn object_id(&self) -> Option<String> {
        self.data.object.get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
    
    /// Check if this is a live mode event
    pub fn is_live(&self) -> bool {
        self.livemode
    }
    
    /// Get the event creation timestamp
    pub fn created_at(&self) -> i64 {
        self.created
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signature_header_parsing() {
        let handler = StripeWebhookHandler::new("test_secret".to_string());
        let header = "t=1614556800 v1=5257a869e7ecb3f2e3c5a6d5e3b5a6d5e3b5a6d5e3b5a6d5e3b5a6d5e3b5a6d5";
        
        let parsed = handler.parse_signature_header(header)
            .expect("Failed to parse valid signature header");
        assert_eq!(parsed.timestamp, 1614556800);
        assert_eq!(parsed.signatures.len(), 1);
    }
    
    #[test]
    fn test_event_type_conversion() {
        assert_eq!(
            WebhookEventType::from("payment_intent.succeeded"),
            WebhookEventType::PaymentIntentSucceeded
        );
        assert_eq!(
            WebhookEventType::from("unknown.event"),
            WebhookEventType::Other("unknown.event".to_string())
        );
    }
    
    #[test]
    fn test_secure_compare() {
        let handler = StripeWebhookHandler::new("test_secret".to_string());
        
        assert!(handler.secure_compare("test", "test"));
        assert!(!handler.secure_compare("test", "test2"));
        assert!(!handler.secure_compare("test", "tes"));
    }
    
    #[test]
    fn test_webhook_event_helpers() {
        let event_json = r#"{
            "id": "evt_test123",
            "object": "event",
            "type": "payment_intent.succeeded",
            "created": 1614556800,
            "livemode": false,
            "pending_webhooks": 1,
            "data": {
                "object": {
                    "id": "pi_test456",
                    "object": "payment_intent"
                }
            }
        }"#;
        
        let event: WebhookEvent = serde_json::from_str(event_json)
            .expect("Failed to parse valid webhook event JSON");
        
        assert_eq!(event.event_type_enum(), WebhookEventType::PaymentIntentSucceeded);
        assert_eq!(event.object_id(), Some("pi_test456".to_string()));
        assert!(!event.is_live());
        assert_eq!(event.created_at(), 1614556800);
    }
}