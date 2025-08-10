use payup::stripe::{StripeWebhookHandler, WebhookEvent, WebhookEventType};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Helper function to generate a valid signature for testing
fn generate_test_signature(secret: &str, timestamp: i64, payload: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    
    let signed_payload = format!("{}.{}", timestamp, payload);
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(signed_payload.as_bytes());
    let result = mac.finalize();
    
    hex::encode(result.into_bytes())
}

#[test]
fn test_webhook_signature_verification_success() {
    let secret = "whsec_test_secret";
    let handler = StripeWebhookHandler::new(secret.to_string());
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    let timestamp = chrono::Utc::now().timestamp();
    let signature = generate_test_signature(secret, timestamp, payload);
    let header = format!("t={} v1={}", timestamp, signature);
    
    // Should succeed with valid signature
    assert!(handler.verify_signature(payload, &header).is_ok());
}

#[test]
fn test_webhook_signature_verification_invalid_signature() {
    let handler = StripeWebhookHandler::new("whsec_test_secret".to_string());
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    let timestamp = chrono::Utc::now().timestamp();
    let header = format!("t={} v1=invalid_signature", timestamp);
    
    // Should fail with invalid signature
    assert!(handler.verify_signature(payload, &header).is_err());
}

#[test]
fn test_webhook_signature_verification_expired_timestamp() {
    let secret = "whsec_test_secret";
    let handler = StripeWebhookHandler::new(secret.to_string());
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    let old_timestamp = chrono::Utc::now().timestamp() - 400; // 400 seconds ago (outside 5 min window)
    let signature = generate_test_signature(secret, old_timestamp, payload);
    let header = format!("t={} v1={}", old_timestamp, signature);
    
    // Should fail with expired timestamp
    let result = handler.verify_signature(payload, &header);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("tolerance window"));
}

#[test]
fn test_webhook_event_parsing() {
    let secret = "whsec_test_secret";
    let handler = StripeWebhookHandler::new(secret.to_string());
    
    let payload = r#"{
        "id": "evt_1234567890",
        "object": "event",
        "type": "payment_intent.succeeded",
        "created": 1614556800,
        "livemode": false,
        "pending_webhooks": 1,
        "data": {
            "object": {
                "id": "pi_1234567890",
                "object": "payment_intent",
                "amount": 2000,
                "currency": "usd",
                "status": "succeeded"
            }
        }
    }"#;
    
    let timestamp = chrono::Utc::now().timestamp();
    let signature = generate_test_signature(secret, timestamp, payload);
    let header = format!("t={} v1={}", timestamp, signature);
    
    let event = handler.construct_event(payload, &header).unwrap();
    
    assert_eq!(event.id, "evt_1234567890");
    assert_eq!(event.event_type, "payment_intent.succeeded");
    assert_eq!(event.event_type_enum(), WebhookEventType::PaymentIntentSucceeded);
    assert_eq!(event.object_id(), Some("pi_1234567890".to_string()));
    assert!(!event.is_live());
}

#[test]
fn test_webhook_event_type_mapping() {
    // Test various event type conversions
    assert_eq!(
        WebhookEventType::from("payment_intent.succeeded"),
        WebhookEventType::PaymentIntentSucceeded
    );
    assert_eq!(
        WebhookEventType::from("charge.succeeded"),
        WebhookEventType::ChargeSucceeded
    );
    assert_eq!(
        WebhookEventType::from("customer.created"),
        WebhookEventType::CustomerCreated
    );
    assert_eq!(
        WebhookEventType::from("invoice.paid"),
        WebhookEventType::InvoicePaid
    );
    assert_eq!(
        WebhookEventType::from("unknown.event.type"),
        WebhookEventType::Other("unknown.event.type".to_string())
    );
}

#[test]
fn test_webhook_with_custom_tolerance() {
    let secret = "whsec_test_secret";
    let handler = StripeWebhookHandler::with_tolerance(secret.to_string(), 60); // 1 minute tolerance
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    let timestamp = chrono::Utc::now().timestamp() - 30; // 30 seconds ago
    let signature = generate_test_signature(secret, timestamp, payload);
    let header = format!("t={} v1={}", timestamp, signature);
    
    // Should succeed within 1 minute tolerance
    assert!(handler.verify_signature(payload, &header).is_ok());
    
    // Test with timestamp outside tolerance
    let old_timestamp = chrono::Utc::now().timestamp() - 120; // 2 minutes ago
    let old_signature = generate_test_signature(secret, old_timestamp, payload);
    let old_header = format!("t={} v1={}", old_timestamp, old_signature);
    
    // Should fail outside 1 minute tolerance
    assert!(handler.verify_signature(payload, &old_header).is_err());
}

#[test]
fn test_webhook_multiple_signatures() {
    let secret = "whsec_test_secret";
    let handler = StripeWebhookHandler::new(secret.to_string());
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    let timestamp = chrono::Utc::now().timestamp();
    let valid_signature = generate_test_signature(secret, timestamp, payload);
    
    // Header with multiple signatures (one valid, one invalid)
    let header = format!("t={} v1=invalid_sig v1={}", timestamp, valid_signature);
    
    // Should succeed if at least one signature is valid
    assert!(handler.verify_signature(payload, &header).is_ok());
}

#[test]
fn test_webhook_malformed_header() {
    let handler = StripeWebhookHandler::new("whsec_test_secret".to_string());
    
    let payload = r#"{"id":"evt_test","type":"payment_intent.succeeded"}"#;
    
    // Test missing timestamp
    let header = "v1=some_signature";
    assert!(handler.verify_signature(payload, header).is_err());
    
    // Test missing signature
    let header = "t=1614556800";
    assert!(handler.verify_signature(payload, header).is_err());
    
    // Test completely malformed header
    let header = "malformed";
    assert!(handler.verify_signature(payload, header).is_err());
}

#[test]
fn test_webhook_event_data_extraction() {
    let event_json = r#"{
        "id": "evt_test",
        "object": "event",
        "type": "customer.subscription.updated",
        "created": 1614556800,
        "livemode": true,
        "pending_webhooks": 2,
        "api_version": "2020-08-27",
        "data": {
            "object": {
                "id": "sub_1234567890",
                "object": "subscription",
                "customer": "cus_1234567890",
                "status": "active"
            },
            "previous_attributes": {
                "status": "trialing"
            }
        },
        "request": {
            "id": "req_test123",
            "idempotency_key": "idem_key_123"
        }
    }"#;
    
    let event: WebhookEvent = serde_json::from_str(event_json).unwrap();
    
    assert_eq!(event.id, "evt_test");
    assert_eq!(event.event_type_enum(), WebhookEventType::CustomerSubscriptionUpdated);
    assert!(event.is_live());
    assert_eq!(event.pending_webhooks, 2);
    assert_eq!(event.api_version, Some("2020-08-27".to_string()));
    
    // Check data extraction first (before moving request)
    assert_eq!(event.object_id(), Some("sub_1234567890".to_string()));
    
    // Check previous attributes exist
    assert!(event.data.previous_attributes.is_some());
    
    // Check request details (move this last since it consumes event.request)
    assert!(event.request.is_some());
    let request = event.request.unwrap();
    assert_eq!(request.id, Some("req_test123".to_string()));
    assert_eq!(request.idempotency_key, Some("idem_key_123".to_string()));
}