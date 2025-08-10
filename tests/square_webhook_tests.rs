use payup::square::webhooks::{
    SquareWebhookHandler, WebhookEvent, WebhookEventType, WebhookEventHandler,
};
use serde_json::json;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::{Arc, Mutex};

const TEST_SIGNATURE_KEY: &str = "test_signature_key_12345";
const TEST_WEBHOOK_URL: &str = "https://example.com/webhooks/square";

fn create_test_handler() -> SquareWebhookHandler {
    SquareWebhookHandler::with_settings(
        TEST_SIGNATURE_KEY.to_string(),
        300, // 5 minutes tolerance
        Some(TEST_WEBHOOK_URL.to_string()),
    )
}

fn generate_signature(url: &str, payload: &str, key: &str) -> String {
    let data_to_sign = format!("{}{}", url, payload);
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data_to_sign.as_bytes());
    let result = mac.finalize();
    
    use base64::prelude::*;
    BASE64_STANDARD.encode(result.into_bytes())
}

#[test]
fn test_webhook_signature_verification_success() {
    let handler = create_test_handler();
    
    let payload = json!({
        "merchant_id": "MERCHANT_123",
        "type": "webhook.notification",
        "event_id": "event_123",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "data": {
            "type": "payment",
            "id": "PAYMENT_456"
        }
    });
    
    let payload_str = payload.to_string();
    let signature = generate_signature(TEST_WEBHOOK_URL, &payload_str, TEST_SIGNATURE_KEY);
    
    let result = handler.verify_signature(&payload_str, &signature, TEST_WEBHOOK_URL);
    assert!(result.is_ok());
}

#[test]
fn test_webhook_signature_verification_invalid_signature() {
    let handler = create_test_handler();
    
    let payload = json!({
        "merchant_id": "MERCHANT_123",
        "type": "webhook.notification",
        "event_id": "event_123",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "data": {
            "type": "payment",
            "id": "PAYMENT_456"
        }
    });
    
    let payload_str = payload.to_string();
    let invalid_signature = "invalid_signature_base64";
    
    let result = handler.verify_signature(&payload_str, invalid_signature, TEST_WEBHOOK_URL);
    assert!(result.is_err());
}

#[test]
fn test_webhook_signature_verification_wrong_url() {
    let handler = create_test_handler();
    
    let payload = json!({
        "merchant_id": "MERCHANT_123",
        "type": "webhook.notification",
        "event_id": "event_123",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "data": {
            "type": "payment",
            "id": "PAYMENT_456"
        }
    });
    
    let payload_str = payload.to_string();
    let signature = generate_signature("https://wrong.com/webhook", &payload_str, TEST_SIGNATURE_KEY);
    
    let result = handler.verify_signature(&payload_str, &signature, "https://wrong.com/webhook");
    assert!(result.is_err());
}

#[test]
fn test_webhook_timestamp_validation() {
    let handler = create_test_handler();
    
    // Create a payload with old timestamp (2 hours ago)
    let old_time = chrono::Utc::now() - chrono::Duration::hours(2);
    let payload = json!({
        "merchant_id": "MERCHANT_123",
        "type": "webhook.notification",
        "event_id": "event_123",
        "created_at": old_time.to_rfc3339(),
        "data": {
            "type": "payment",
            "id": "PAYMENT_456"
        }
    });
    
    let payload_str = payload.to_string();
    let signature = generate_signature(TEST_WEBHOOK_URL, &payload_str, TEST_SIGNATURE_KEY);
    
    let result = handler.verify_signature(&payload_str, &signature, TEST_WEBHOOK_URL);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("tolerance window"));
}

#[test]
fn test_webhook_event_parsing() {
    let event_json = r#"{
        "merchant_id": "MERCHANT_ABC",
        "location_id": "LOC_123",
        "entity_id": "PAYMENT_XYZ",
        "type": "payment.created",
        "event_id": "evt_unique",
        "created_at": "2024-01-15T12:00:00Z",
        "data": {
            "type": "payment",
            "id": "PAYMENT_XYZ",
            "object": {
                "id": "PAYMENT_XYZ",
                "amount_money": {
                    "amount": 2500,
                    "currency": "USD"
                },
                "status": "COMPLETED",
                "source_type": "CARD",
                "card_details": {
                    "status": "CAPTURED",
                    "card": {
                        "card_brand": "VISA",
                        "last_4": "1234"
                    }
                }
            }
        }
    }"#;
    
    let event = WebhookEvent::parse(event_json).unwrap();
    
    assert_eq!(event.get_merchant_id(), "MERCHANT_ABC");
    assert_eq!(event.get_location_id(), Some("LOC_123"));
    assert_eq!(event.get_entity_id(), Some("PAYMENT_XYZ"));
    assert_eq!(event.event_type, "payment.created");
    assert_eq!(event.event_type_enum(), WebhookEventType::PaymentCreated);
    
    // Test resource extraction
    let resource: serde_json::Value = event.get_resource().unwrap();
    assert_eq!(resource["id"], "PAYMENT_XYZ");
    assert_eq!(resource["amount_money"]["amount"], 2500);
}

#[test]
fn test_webhook_event_types() {
    let test_cases = vec![
        ("payment.created", WebhookEventType::PaymentCreated),
        ("payment.updated", WebhookEventType::PaymentUpdated),
        ("refund.created", WebhookEventType::RefundCreated),
        ("refund.updated", WebhookEventType::RefundUpdated),
        ("order.created", WebhookEventType::OrderCreated),
        ("order.updated", WebhookEventType::OrderUpdated),
        ("customer.created", WebhookEventType::CustomerCreated),
        ("customer.updated", WebhookEventType::CustomerUpdated),
        ("customer.deleted", WebhookEventType::CustomerDeleted),
        ("invoice.sent", WebhookEventType::InvoiceSent),
        ("subscription.created", WebhookEventType::SubscriptionCreated),
        ("subscription.canceled", WebhookEventType::SubscriptionCanceled),
        ("dispute.created", WebhookEventType::DisputeCreated),
        ("unknown.event.type", WebhookEventType::Other("unknown.event.type".to_string())),
    ];
    
    for (event_str, expected_type) in test_cases {
        let event_type = WebhookEventType::from(event_str);
        assert_eq!(event_type, expected_type, "Failed for event: {}", event_str);
    }
}

#[test]
fn test_webhook_event_handler() {
    use std::sync::{Arc, Mutex};
    
    let mut handler = WebhookEventHandler::new();
    
    // Track which handlers were called
    let payment_created = Arc::new(Mutex::new(false));
    let refund_created = Arc::new(Mutex::new(false));
    let default_called = Arc::new(Mutex::new(false));
    
    let payment_created_clone = payment_created.clone();
    handler.on("payment.created", move |event| {
        *payment_created_clone.lock().unwrap() = true;
        assert_eq!(event.event_type, "payment.created");
        Ok(())
    });
    
    let refund_created_clone = refund_created.clone();
    handler.on("refund.created", move |event| {
        *refund_created_clone.lock().unwrap() = true;
        assert_eq!(event.event_type, "refund.created");
        Ok(())
    });
    
    let default_called_clone = default_called.clone();
    handler.default(move |event| {
        *default_called_clone.lock().unwrap() = true;
        println!("Default handler called for event: {}", event.event_type);
        Ok(())
    });
    
    // Test payment.created event
    let payment_event = WebhookEvent {
        merchant_id: "MERCHANT_123".to_string(),
        location_id: Some("LOC_456".to_string()),
        entity_id: Some("PAYMENT_789".to_string()),
        event_type: "payment.created".to_string(),
        event_id: "evt_001".to_string(),
        created_at: "2024-01-15T10:00:00Z".to_string(),
        data: payup::square::webhooks::WebhookEventData {
            data_type: "payment".to_string(),
            id: "PAYMENT_789".to_string(),
            object: json!({
                "amount_money": {
                    "amount": 1500,
                    "currency": "USD"
                }
            }),
        },
    };
    
    handler.handle(&payment_event).unwrap();
    assert!(*payment_created.lock().unwrap());
    assert!(!*refund_created.lock().unwrap());
    assert!(!*default_called.lock().unwrap());
    
    // Test refund.created event
    let refund_event = WebhookEvent {
        merchant_id: "MERCHANT_123".to_string(),
        location_id: None,
        entity_id: Some("REFUND_456".to_string()),
        event_type: "refund.created".to_string(),
        event_id: "evt_002".to_string(),
        created_at: "2024-01-15T11:00:00Z".to_string(),
        data: payup::square::webhooks::WebhookEventData {
            data_type: "refund".to_string(),
            id: "REFUND_456".to_string(),
            object: json!({
                "amount_money": {
                    "amount": 500,
                    "currency": "USD"
                }
            }),
        },
    };
    
    handler.handle(&refund_event).unwrap();
    assert!(*refund_created.lock().unwrap());
    
    // Reset flags
    *payment_created.lock().unwrap() = false;
    *refund_created.lock().unwrap() = false;
    *default_called.lock().unwrap() = false;
    
    // Test unknown event (should trigger default handler)
    let unknown_event = WebhookEvent {
        merchant_id: "MERCHANT_123".to_string(),
        location_id: None,
        entity_id: None,
        event_type: "custom.event".to_string(),
        event_id: "evt_003".to_string(),
        created_at: "2024-01-15T12:00:00Z".to_string(),
        data: payup::square::webhooks::WebhookEventData {
            data_type: "custom".to_string(),
            id: "CUSTOM_123".to_string(),
            object: json!({}),
        },
    };
    
    handler.handle(&unknown_event).unwrap();
    assert!(!*payment_created.lock().unwrap());
    assert!(!*refund_created.lock().unwrap());
    assert!(*default_called.lock().unwrap());
}

#[test]
fn test_construct_event() {
    let handler = create_test_handler();
    
    let event_json = json!({
        "merchant_id": "MERCHANT_TEST",
        "location_id": "LOC_TEST",
        "entity_id": "ORDER_123",
        "type": "order.created",
        "event_id": "evt_test",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "data": {
            "type": "order",
            "id": "ORDER_123",
            "object": {
                "id": "ORDER_123",
                "state": "OPEN",
                "total_money": {
                    "amount": 3000,
                    "currency": "USD"
                },
                "line_items": [
                    {
                        "name": "Coffee",
                        "quantity": "2",
                        "base_price_money": {
                            "amount": 1500,
                            "currency": "USD"
                        }
                    }
                ]
            }
        }
    });
    
    let payload_str = event_json.to_string();
    let signature = generate_signature(TEST_WEBHOOK_URL, &payload_str, TEST_SIGNATURE_KEY);
    
    let event = handler.construct_event(&payload_str, &signature, TEST_WEBHOOK_URL).unwrap();
    
    assert_eq!(event.get_merchant_id(), "MERCHANT_TEST");
    assert_eq!(event.get_location_id(), Some("LOC_TEST"));
    assert_eq!(event.event_type_enum(), WebhookEventType::OrderCreated);
    
    let resource: serde_json::Value = event.get_resource().unwrap();
    assert_eq!(resource["state"], "OPEN");
    assert_eq!(resource["total_money"]["amount"], 3000);
}

#[tokio::test]
async fn test_async_event_handling() {
    let mut handler = WebhookEventHandler::new();
    
    let handled = Arc::new(Mutex::new(false));
    let handled_clone = handled.clone();
    
    handler.on("payment.created", move |_event| {
        *handled_clone.lock().unwrap() = true;
        Ok(())
    });
    
    let event = WebhookEvent {
        merchant_id: "MERCHANT_ASYNC".to_string(),
        location_id: None,
        entity_id: Some("PAYMENT_ASYNC".to_string()),
        event_type: "payment.created".to_string(),
        event_id: "evt_async".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        data: payup::square::webhooks::WebhookEventData {
            data_type: "payment".to_string(),
            id: "PAYMENT_ASYNC".to_string(),
            object: json!({}),
        },
    };
    
    handler.handle_async(&event).await.unwrap();
    assert!(*handled.lock().unwrap());
}

#[test]
fn test_multiple_event_types_registration() {
    use payup::square::webhooks::WebhookEventType;
    
    let mut handler = WebhookEventHandler::new();
    
    let events_handled = Arc::new(Mutex::new(Vec::new()));
    
    // Register multiple event types
    let events = vec![
        WebhookEventType::PaymentCreated,
        WebhookEventType::RefundCreated,
        WebhookEventType::CustomerCreated,
    ];
    
    for event_type in events {
        let events_handled_clone = events_handled.clone();
        handler.on_event(event_type.clone(), move |event| {
            events_handled_clone.lock().unwrap().push(event.event_type.clone());
            Ok(())
        });
    }
    
    // Test that handlers are called correctly
    let test_events = vec![
        ("payment.created", "PAYMENT_TEST"),
        ("refund.created", "REFUND_TEST"),
        ("customer.created", "CUSTOMER_TEST"),
    ];
    
    for (event_type, entity_id) in test_events {
        let event = WebhookEvent {
            merchant_id: "MERCHANT_MULTI".to_string(),
            location_id: None,
            entity_id: Some(entity_id.to_string()),
            event_type: event_type.to_string(),
            event_id: format!("evt_{}", entity_id),
            created_at: chrono::Utc::now().to_rfc3339(),
            data: payup::square::webhooks::WebhookEventData {
                data_type: event_type.split('.').next().unwrap().to_string(),
                id: entity_id.to_string(),
                object: json!({}),
            },
        };
        
        handler.handle(&event).unwrap();
    }
    
    let handled = events_handled.lock().unwrap();
    assert_eq!(handled.len(), 3);
    assert!(handled.contains(&"payment.created".to_string()));
    assert!(handled.contains(&"refund.created".to_string()));
    assert!(handled.contains(&"customer.created".to_string()));
}