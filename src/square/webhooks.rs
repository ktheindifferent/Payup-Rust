use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{PayupError, Result};
use std::collections::HashMap;

/// Square webhook handler for signature verification and event processing
pub struct SquareWebhookHandler {
    /// The webhook signature key from Square Dashboard
    signature_key: String,
    /// Tolerance for request timestamp verification in seconds (default: 60)
    tolerance: i64,
    /// Webhook notification URL (for validation)
    notification_url: Option<String>,
}

/// Square webhook event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub merchant_id: String,
    pub location_id: Option<String>,
    pub entity_id: Option<String>,
    #[serde(rename = "type")]
    pub event_type: String,
    pub event_id: String,
    pub created_at: String,
    pub data: WebhookEventData,
}

/// Webhook event data wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    pub object: Value,
}

/// Square webhook notification structure (wrapper for events)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotification {
    pub merchant_id: String,
    pub location_id: Option<String>,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub event_id: String,
    pub created_at: String,
    pub data: WebhookNotificationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookNotificationData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    #[serde(flatten)]
    pub details: Value,
}

impl SquareWebhookHandler {
    /// Create a new webhook handler with the signature key
    pub fn new(signature_key: String) -> Self {
        Self {
            signature_key,
            tolerance: 60, // 60 seconds default
            notification_url: None,
        }
    }

    /// Create a webhook handler with custom settings
    pub fn with_settings(
        signature_key: String,
        tolerance: i64,
        notification_url: Option<String>,
    ) -> Self {
        Self {
            signature_key,
            tolerance,
            notification_url,
        }
    }

    /// Verify webhook signature and parse the event
    pub fn construct_event(
        &self,
        payload: &str,
        signature: &str,
        request_url: &str,
    ) -> Result<WebhookEvent> {
        // Verify the signature
        self.verify_signature(payload, signature, request_url)?;
        
        // Parse the event
        let event: WebhookEvent = serde_json::from_str(payload)
            .map_err(|e| PayupError::SerializationError(e))?;
        
        Ok(event)
    }

    /// Verify webhook signature using Square's HMAC-SHA256 algorithm
    pub fn verify_signature(
        &self,
        payload: &str,
        signature: &str,
        request_url: &str,
    ) -> Result<()> {
        // Validate notification URL if configured
        if let Some(ref expected_url) = self.notification_url {
            if request_url != expected_url {
                return Err(PayupError::ValidationError(
                    format!("Request URL mismatch: expected {}, got {}", expected_url, request_url)
                ));
            }
        }

        // Parse the notification to check timestamp
        let notification: WebhookNotification = serde_json::from_str(payload)
            .map_err(|e| PayupError::SerializationError(e))?;
        
        // Verify timestamp is within tolerance
        if let Err(e) = self.verify_timestamp(&notification.created_at) {
            return Err(e);
        }

        // Compute expected signature
        let expected_signature = self.compute_signature(request_url, payload)?;
        
        // Verify signature matches (constant-time comparison)
        if !self.secure_compare(&expected_signature, signature) {
            return Err(PayupError::ValidationError(
                "Invalid webhook signature".to_string()
            ));
        }
        
        Ok(())
    }

    /// Verify the webhook timestamp is within tolerance
    fn verify_timestamp(&self, timestamp: &str) -> Result<()> {
        use chrono::{DateTime, Utc};
        
        let event_time = DateTime::parse_from_rfc3339(timestamp)
            .map_err(|e| PayupError::ValidationError(format!("Invalid timestamp format: {}", e)))?
            .with_timezone(&Utc);
        
        let now = Utc::now();
        let diff = now.signed_duration_since(event_time).num_seconds().abs();
        
        if diff > self.tolerance {
            return Err(PayupError::ValidationError(
                format!("Webhook timestamp outside tolerance window ({} seconds)", diff)
            ));
        }
        
        Ok(())
    }

    /// Compute HMAC-SHA256 signature for Square webhooks
    fn compute_signature(&self, url: &str, payload: &str) -> Result<String> {
        // Square's signature format: HMAC-SHA256(url + body, signature_key)
        let data_to_sign = format!("{}{}", url, payload);
        
        type HmacSha256 = Hmac<Sha256>;
        
        let mut mac = HmacSha256::new_from_slice(self.signature_key.as_bytes())
            .map_err(|e| PayupError::GenericError(format!("HMAC error: {}", e)))?;
        
        mac.update(data_to_sign.as_bytes());
        let result = mac.finalize();
        
        // Square uses base64 encoding for signatures
        use base64::prelude::*;
        Ok(BASE64_STANDARD.encode(result.into_bytes()))
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

/// Common Square webhook event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventType {
    // Payment Events
    PaymentCreated,
    PaymentUpdated,
    
    // Refund Events
    RefundCreated,
    RefundUpdated,
    
    // Order Events
    OrderCreated,
    OrderUpdated,
    OrderFulfillmentUpdated,
    
    // Customer Events
    CustomerCreated,
    CustomerUpdated,
    CustomerDeleted,
    
    // Card Events
    CardCreated,
    CardUpdated,
    CardDeleted,
    CardDisabled,
    
    // Invoice Events
    InvoiceCreated,
    InvoiceSent,
    InvoiceScheduledChargeStarted,
    InvoiceScheduledChargeFailed,
    InvoicePaymentMade,
    InvoiceUpdated,
    InvoiceDeleted,
    
    // Subscription Events
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionCanceled,
    SubscriptionPaused,
    SubscriptionResumed,
    
    // Catalog Events
    CatalogVersionUpdated,
    
    // Inventory Events
    InventoryCountUpdated,
    
    // Location Events
    LocationCreated,
    LocationUpdated,
    
    // Team Member Events
    TeamMemberCreated,
    TeamMemberUpdated,
    TeamMemberWageSettingUpdated,
    
    // Booking Events
    BookingCreated,
    BookingUpdated,
    BookingCanceled,
    
    // Loyalty Events
    LoyaltyAccountCreated,
    LoyaltyAccountUpdated,
    LoyaltyProgramCreated,
    LoyaltyProgramUpdated,
    LoyaltyPromotionCreated,
    LoyaltyPromotionUpdated,
    
    // Gift Card Events
    GiftCardCreated,
    GiftCardUpdated,
    GiftCardActivityCreated,
    
    // OAuth Events
    OAuthAuthorizationRevoked,
    
    // Dispute Events
    DisputeCreated,
    DisputeEvidenceAdded,
    DisputeEvidenceRemoved,
    DisputeStateChanged,
    
    // Payout Events
    PayoutSent,
    PayoutFailed,
    
    // Terminal Events
    TerminalCheckoutCreated,
    TerminalCheckoutUpdated,
    TerminalRefundCreated,
    TerminalRefundUpdated,
    
    // Bank Account Events
    BankAccountCreated,
    BankAccountVerified,
    BankAccountDisabled,
    
    // Vendor Events
    VendorCreated,
    VendorUpdated,
    
    // Other Events
    Other(String),
}

impl From<&str> for WebhookEventType {
    fn from(event_type: &str) -> Self {
        match event_type {
            // Payment Events
            "payment.created" => Self::PaymentCreated,
            "payment.updated" => Self::PaymentUpdated,
            
            // Refund Events
            "refund.created" => Self::RefundCreated,
            "refund.updated" => Self::RefundUpdated,
            
            // Order Events
            "order.created" => Self::OrderCreated,
            "order.updated" => Self::OrderUpdated,
            "order.fulfillment.updated" => Self::OrderFulfillmentUpdated,
            
            // Customer Events
            "customer.created" => Self::CustomerCreated,
            "customer.updated" => Self::CustomerUpdated,
            "customer.deleted" => Self::CustomerDeleted,
            
            // Card Events
            "card.created" => Self::CardCreated,
            "card.updated" => Self::CardUpdated,
            "card.deleted" => Self::CardDeleted,
            "card.disabled" => Self::CardDisabled,
            
            // Invoice Events
            "invoice.created" => Self::InvoiceCreated,
            "invoice.sent" => Self::InvoiceSent,
            "invoice.scheduled_charge_started" => Self::InvoiceScheduledChargeStarted,
            "invoice.scheduled_charge_failed" => Self::InvoiceScheduledChargeFailed,
            "invoice.payment_made" => Self::InvoicePaymentMade,
            "invoice.updated" => Self::InvoiceUpdated,
            "invoice.deleted" => Self::InvoiceDeleted,
            
            // Subscription Events
            "subscription.created" => Self::SubscriptionCreated,
            "subscription.updated" => Self::SubscriptionUpdated,
            "subscription.canceled" => Self::SubscriptionCanceled,
            "subscription.paused" => Self::SubscriptionPaused,
            "subscription.resumed" => Self::SubscriptionResumed,
            
            // Catalog Events
            "catalog.version.updated" => Self::CatalogVersionUpdated,
            
            // Inventory Events
            "inventory.count.updated" => Self::InventoryCountUpdated,
            
            // Location Events
            "location.created" => Self::LocationCreated,
            "location.updated" => Self::LocationUpdated,
            
            // Team Member Events
            "team_member.created" => Self::TeamMemberCreated,
            "team_member.updated" => Self::TeamMemberUpdated,
            "team_member.wage_setting.updated" => Self::TeamMemberWageSettingUpdated,
            
            // Booking Events
            "booking.created" => Self::BookingCreated,
            "booking.updated" => Self::BookingUpdated,
            "booking.canceled" => Self::BookingCanceled,
            
            // Loyalty Events
            "loyalty.account.created" => Self::LoyaltyAccountCreated,
            "loyalty.account.updated" => Self::LoyaltyAccountUpdated,
            "loyalty.program.created" => Self::LoyaltyProgramCreated,
            "loyalty.program.updated" => Self::LoyaltyProgramUpdated,
            "loyalty.promotion.created" => Self::LoyaltyPromotionCreated,
            "loyalty.promotion.updated" => Self::LoyaltyPromotionUpdated,
            
            // Gift Card Events
            "gift_card.created" => Self::GiftCardCreated,
            "gift_card.updated" => Self::GiftCardUpdated,
            "gift_card_activity.created" => Self::GiftCardActivityCreated,
            
            // OAuth Events
            "oauth.authorization.revoked" => Self::OAuthAuthorizationRevoked,
            
            // Dispute Events
            "dispute.created" => Self::DisputeCreated,
            "dispute.evidence.added" => Self::DisputeEvidenceAdded,
            "dispute.evidence.removed" => Self::DisputeEvidenceRemoved,
            "dispute.state.changed" => Self::DisputeStateChanged,
            
            // Payout Events
            "payout.sent" => Self::PayoutSent,
            "payout.failed" => Self::PayoutFailed,
            
            // Terminal Events
            "terminal.checkout.created" => Self::TerminalCheckoutCreated,
            "terminal.checkout.updated" => Self::TerminalCheckoutUpdated,
            "terminal.refund.created" => Self::TerminalRefundCreated,
            "terminal.refund.updated" => Self::TerminalRefundUpdated,
            
            // Bank Account Events
            "bank_account.created" => Self::BankAccountCreated,
            "bank_account.verified" => Self::BankAccountVerified,
            "bank_account.disabled" => Self::BankAccountDisabled,
            
            // Vendor Events
            "vendor.created" => Self::VendorCreated,
            "vendor.updated" => Self::VendorUpdated,
            
            other => Self::Other(other.to_string()),
        }
    }
}

impl WebhookEvent {
    /// Parse a webhook event from JSON payload
    pub fn parse(payload: &str) -> Result<Self> {
        serde_json::from_str(payload)
            .map_err(|e| PayupError::SerializationError(e))
    }
    
    /// Get the event type as an enum
    pub fn event_type_enum(&self) -> WebhookEventType {
        WebhookEventType::from(self.event_type.as_str())
    }
    
    /// Extract the entity ID from the event
    pub fn get_entity_id(&self) -> Option<&str> {
        self.entity_id.as_deref()
    }
    
    /// Get the merchant ID
    pub fn get_merchant_id(&self) -> &str {
        &self.merchant_id
    }
    
    /// Get the location ID
    pub fn get_location_id(&self) -> Option<&str> {
        self.location_id.as_deref()
    }
    
    /// Extract the resource object from the event data
    pub fn get_resource<T>(&self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_value(self.data.object.clone())
            .map_err(|e| PayupError::SerializationError(e))
    }
    
    /// Get the event creation timestamp
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
}

/// Helper to handle webhook events with registered handlers
pub struct WebhookEventHandler {
    handlers: HashMap<String, Box<dyn Fn(&WebhookEvent) -> Result<()> + Send + Sync>>,
    default_handler: Option<Box<dyn Fn(&WebhookEvent) -> Result<()> + Send + Sync>>,
}

impl WebhookEventHandler {
    /// Create a new event handler
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            default_handler: None,
        }
    }
    
    /// Register a handler for a specific event type
    pub fn on<F>(&mut self, event_type: &str, handler: F) -> &mut Self
    where
        F: Fn(&WebhookEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.handlers.insert(event_type.to_string(), Box::new(handler));
        self
    }
    
    /// Register a handler for specific event type enum
    pub fn on_event<F>(&mut self, event_type: WebhookEventType, handler: F) -> &mut Self
    where
        F: Fn(&WebhookEvent) -> Result<()> + Send + Sync + 'static,
    {
        let event_str = match event_type {
            WebhookEventType::PaymentCreated => "payment.created",
            WebhookEventType::PaymentUpdated => "payment.updated",
            WebhookEventType::RefundCreated => "refund.created",
            WebhookEventType::RefundUpdated => "refund.updated",
            WebhookEventType::OrderCreated => "order.created",
            WebhookEventType::OrderUpdated => "order.updated",
            WebhookEventType::CustomerCreated => "customer.created",
            WebhookEventType::CustomerUpdated => "customer.updated",
            WebhookEventType::CustomerDeleted => "customer.deleted",
            WebhookEventType::Other(ref s) => s,
            _ => return self, // Skip if no string mapping
        };
        
        self.handlers.insert(event_str.to_string(), Box::new(handler));
        self
    }
    
    /// Set a default handler for unregistered event types
    pub fn default<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(&WebhookEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.default_handler = Some(Box::new(handler));
        self
    }
    
    /// Handle an incoming webhook event
    pub fn handle(&self, event: &WebhookEvent) -> Result<()> {
        if let Some(handler) = self.handlers.get(&event.event_type) {
            handler(event)
        } else if let Some(ref default_handler) = self.default_handler {
            default_handler(event)
        } else {
            // No handler registered for this event type
            Ok(())
        }
    }
    
    /// Handle an event asynchronously
    pub async fn handle_async(&self, event: &WebhookEvent) -> Result<()> {
        self.handle(event)
    }
}

impl Default for WebhookEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_type_conversion() {
        assert_eq!(
            WebhookEventType::from("payment.created"),
            WebhookEventType::PaymentCreated
        );
        assert_eq!(
            WebhookEventType::from("customer.updated"),
            WebhookEventType::CustomerUpdated
        );
        assert_eq!(
            WebhookEventType::from("unknown.event"),
            WebhookEventType::Other("unknown.event".to_string())
        );
    }
    
    #[test]
    fn test_secure_compare() {
        let handler = SquareWebhookHandler::new("test_key".to_string());
        
        assert!(handler.secure_compare("test123", "test123"));
        assert!(!handler.secure_compare("test123", "test124"));
        assert!(!handler.secure_compare("test", "test123"));
    }
    
    #[test]
    fn test_webhook_event_parsing() {
        let event_json = r#"{
            "merchant_id": "MERCHANT_123",
            "location_id": "LOC_456",
            "entity_id": "PAYMENT_789",
            "type": "payment.created",
            "event_id": "event_abc",
            "created_at": "2024-01-15T10:00:00Z",
            "data": {
                "type": "payment",
                "id": "PAYMENT_789",
                "object": {
                    "id": "PAYMENT_789",
                    "amount_money": {
                        "amount": 1000,
                        "currency": "USD"
                    },
                    "status": "COMPLETED"
                }
            }
        }"#;
        
        let event = WebhookEvent::parse(event_json).unwrap();
        
        assert_eq!(event.event_type_enum(), WebhookEventType::PaymentCreated);
        assert_eq!(event.get_merchant_id(), "MERCHANT_123");
        assert_eq!(event.get_location_id(), Some("LOC_456"));
        assert_eq!(event.get_entity_id(), Some("PAYMENT_789"));
    }
    
    #[test]
    fn test_event_handler_registration() {
        let mut handler = WebhookEventHandler::new();
        
        let payment_handled = std::sync::Arc::new(std::sync::Mutex::new(false));
        let payment_handled_clone = payment_handled.clone();
        
        handler.on("payment.created", move |_event| {
            *payment_handled_clone.lock().unwrap() = true;
            Ok(())
        });
        
        let event = WebhookEvent {
            merchant_id: "TEST".to_string(),
            location_id: None,
            entity_id: None,
            event_type: "payment.created".to_string(),
            event_id: "test".to_string(),
            created_at: "2024-01-15T10:00:00Z".to_string(),
            data: WebhookEventData {
                data_type: "payment".to_string(),
                id: "test".to_string(),
                object: serde_json::json!({}),
            },
        };
        
        handler.handle(&event).unwrap();
        assert!(*payment_handled.lock().unwrap());
    }
    
    #[test]
    fn test_timestamp_verification() {
        use chrono::{Duration, Utc};
        
        let handler = SquareWebhookHandler::new("test_key".to_string());
        
        // Current timestamp should pass
        let now = Utc::now();
        let timestamp = now.to_rfc3339();
        assert!(handler.verify_timestamp(&timestamp).is_ok());
        
        // Old timestamp should fail
        let old = now - Duration::seconds(120);
        let old_timestamp = old.to_rfc3339();
        assert!(handler.verify_timestamp(&old_timestamp).is_err());
    }
}