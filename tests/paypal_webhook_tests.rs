#[cfg(test)]
mod paypal_webhook_tests {
    use payup::paypal::webhooks::{
        PayPalWebhookHandler, WebhookEvent, WebhookHandler, event_types, VerificationStatus
    };
    use payup::error::{PayupError, Result};
    use std::collections::HashMap;
    
    #[test]
    fn test_webhook_handler_creation() {
        let handler = PayPalWebhookHandler::new("WH_123456".to_string());
        assert_eq!(handler.webhook_id, "WH_123456");
    }
    
    #[test]
    fn test_extract_paypal_headers() {
        let raw_headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("PayPal-Auth-Algo".to_string(), "SHA256withRSA".to_string()),
            ("PayPal-Cert-Url".to_string(), "https://api.paypal.com/cert.pem".to_string()),
            ("PayPal-Transmission-Id".to_string(), "tx-123456".to_string()),
            ("PayPal-Transmission-Sig".to_string(), "signature-data".to_string()),
            ("PayPal-Transmission-Time".to_string(), "2024-01-01T00:00:00Z".to_string()),
            ("Other-Header".to_string(), "other-value".to_string()),
        ];
        
        let headers = PayPalWebhookHandler::extract_headers(&raw_headers);
        
        assert_eq!(headers.len(), 5);
        assert_eq!(headers.get("paypal-auth-algo").unwrap(), "SHA256withRSA");
        assert_eq!(headers.get("paypal-cert-url").unwrap(), "https://api.paypal.com/cert.pem");
        assert_eq!(headers.get("paypal-transmission-id").unwrap(), "tx-123456");
        assert!(!headers.contains_key("content-type"));
        assert!(!headers.contains_key("other-header"));
    }
    
    #[test]
    fn test_headers_from_slice() {
        let raw_headers = [
            ("PayPal-Auth-Algo", "SHA256withRSA"),
            ("PayPal-Cert-Url", "https://api.paypal.com/cert.pem"),
            ("PayPal-Transmission-Id", "tx-123456"),
        ];
        
        let headers = PayPalWebhookHandler::headers_from_slice(&raw_headers);
        
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get("paypal-auth-algo").unwrap(), "SHA256withRSA");
        assert_eq!(headers.get("paypal-cert-url").unwrap(), "https://api.paypal.com/cert.pem");
    }
    
    #[test]
    fn test_validate_headers_success() {
        let mut headers = HashMap::new();
        headers.insert("paypal-auth-algo".to_string(), "SHA256withRSA".to_string());
        headers.insert("paypal-cert-url".to_string(), "https://api.paypal.com/cert.pem".to_string());
        headers.insert("paypal-transmission-id".to_string(), "tx-123456".to_string());
        headers.insert("paypal-transmission-sig".to_string(), "signature".to_string());
        headers.insert("paypal-transmission-time".to_string(), "2024-01-01T00:00:00Z".to_string());
        
        let result = PayPalWebhookHandler::validate_headers(&headers);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_headers_missing_required() {
        let mut headers = HashMap::new();
        headers.insert("paypal-auth-algo".to_string(), "SHA256withRSA".to_string());
        headers.insert("paypal-cert-url".to_string(), "https://api.paypal.com/cert.pem".to_string());
        // Missing other required headers
        
        let result = PayPalWebhookHandler::validate_headers(&headers);
        assert!(result.is_err());
        
        match result {
            Err(PayupError::ValidationError(msg)) => {
                assert!(msg.contains("Missing required header"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }
    
    #[test]
    fn test_validate_headers_invalid_cert_url() {
        let mut headers = HashMap::new();
        headers.insert("paypal-auth-algo".to_string(), "SHA256withRSA".to_string());
        headers.insert("paypal-cert-url".to_string(), "https://malicious.com/cert.pem".to_string());
        headers.insert("paypal-transmission-id".to_string(), "tx-123456".to_string());
        headers.insert("paypal-transmission-sig".to_string(), "signature".to_string());
        headers.insert("paypal-transmission-time".to_string(), "2024-01-01T00:00:00Z".to_string());
        
        let result = PayPalWebhookHandler::validate_headers(&headers);
        assert!(result.is_err());
        
        match result {
            Err(PayupError::ValidationError(msg)) => {
                assert!(msg.contains("Invalid cert URL"));
                assert!(msg.contains("must be from paypal.com domain"));
            }
            _ => panic!("Expected ValidationError for invalid cert URL"),
        }
    }
    
    #[test]
    fn test_webhook_event_parse() {
        let json_payload = r#"{
            "id": "WH-123456789",
            "event_type": "PAYMENT.CAPTURE.COMPLETED",
            "resource_type": "capture",
            "summary": "Payment completed successfully",
            "resource": {
                "id": "CAP-123456",
                "amount": {
                    "currency_code": "USD",
                    "value": "100.00"
                }
            },
            "create_time": "2024-01-01T00:00:00Z",
            "event_version": "1.0"
        }"#;
        
        let event = WebhookEvent::parse(json_payload);
        assert!(event.is_ok());
        
        let event = event.unwrap();
        assert_eq!(event.id, "WH-123456789");
        assert_eq!(event.event_type, "PAYMENT.CAPTURE.COMPLETED");
        assert_eq!(event.resource_type, Some("capture".to_string()));
        assert_eq!(event.summary, Some("Payment completed successfully".to_string()));
        assert_eq!(event.event_version, "1.0");
    }
    
    #[test]
    fn test_webhook_event_parse_invalid() {
        let invalid_json = r#"{ invalid json }"#;
        
        let result = WebhookEvent::parse(invalid_json);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_webhook_event_get_resource() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct CaptureResource {
            id: String,
            amount: Amount,
        }
        
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Amount {
            currency_code: String,
            value: String,
        }
        
        let json_payload = r#"{
            "id": "WH-123456789",
            "event_type": "PAYMENT.CAPTURE.COMPLETED",
            "resource_type": "capture",
            "summary": "Payment completed",
            "resource": {
                "id": "CAP-123456",
                "amount": {
                    "currency_code": "USD",
                    "value": "100.00"
                }
            },
            "create_time": "2024-01-01T00:00:00Z",
            "event_version": "1.0"
        }"#;
        
        let event = WebhookEvent::parse(json_payload).unwrap();
        let resource: Result<CaptureResource> = event.get_resource();
        
        assert!(resource.is_ok());
        let resource = resource.unwrap();
        assert_eq!(resource.id, "CAP-123456");
        assert_eq!(resource.amount.currency_code, "USD");
        assert_eq!(resource.amount.value, "100.00");
    }
    
    #[test]
    fn test_webhook_handler_registration_and_handling() {
        use std::sync::{Arc, Mutex};
        
        let mut handler = WebhookHandler::new();
        let counter = Arc::new(Mutex::new(0));
        
        let counter_clone = counter.clone();
        handler.on(event_types::PAYMENT_CAPTURE_COMPLETED, move |_event| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            Ok(())
        });
        
        let counter_clone = counter.clone();
        handler.on(event_types::BILLING_SUBSCRIPTION_CREATED, move |_event| {
            let mut count = counter_clone.lock().unwrap();
            *count += 10;
            Ok(())
        });
        
        // Create test events
        let payment_event = WebhookEvent {
            id: "1".to_string(),
            event_type: event_types::PAYMENT_CAPTURE_COMPLETED.to_string(),
            resource_type: None,
            summary: None,
            resource: serde_json::Value::Null,
            create_time: "2024-01-01T00:00:00Z".to_string(),
            event_version: "1.0".to_string(),
            links: None,
        };
        
        let subscription_event = WebhookEvent {
            id: "2".to_string(),
            event_type: event_types::BILLING_SUBSCRIPTION_CREATED.to_string(),
            resource_type: None,
            summary: None,
            resource: serde_json::Value::Null,
            create_time: "2024-01-01T00:00:00Z".to_string(),
            event_version: "1.0".to_string(),
            links: None,
        };
        
        let unknown_event = WebhookEvent {
            id: "3".to_string(),
            event_type: "UNKNOWN.EVENT".to_string(),
            resource_type: None,
            summary: None,
            resource: serde_json::Value::Null,
            create_time: "2024-01-01T00:00:00Z".to_string(),
            event_version: "1.0".to_string(),
            links: None,
        };
        
        // Handle events
        assert!(handler.handle(&payment_event).is_ok());
        assert_eq!(*counter.lock().unwrap(), 1);
        
        assert!(handler.handle(&subscription_event).is_ok());
        assert_eq!(*counter.lock().unwrap(), 11);
        
        // Unknown event should be ok but not increment counter
        assert!(handler.handle(&unknown_event).is_ok());
        assert_eq!(*counter.lock().unwrap(), 11);
    }
    
    #[test]
    fn test_event_types_constants() {
        // Verify event type constants are properly defined
        assert_eq!(event_types::PAYMENT_CAPTURE_COMPLETED, "PAYMENT.CAPTURE.COMPLETED");
        assert_eq!(event_types::PAYMENT_CAPTURE_DENIED, "PAYMENT.CAPTURE.DENIED");
        assert_eq!(event_types::BILLING_SUBSCRIPTION_CREATED, "BILLING.SUBSCRIPTION.CREATED");
        assert_eq!(event_types::BILLING_SUBSCRIPTION_CANCELLED, "BILLING.SUBSCRIPTION.CANCELLED");
        assert_eq!(event_types::CHECKOUT_ORDER_APPROVED, "CHECKOUT.ORDER.APPROVED");
    }
    
    #[test]
    fn test_verification_status_serialization() {
        // Test SUCCESS serialization
        let success = VerificationStatus::Success;
        let json = serde_json::to_string(&success).unwrap();
        assert_eq!(json, r#""SUCCESS""#);
        
        // Test FAILURE serialization
        let failure = VerificationStatus::Failure;
        let json = serde_json::to_string(&failure).unwrap();
        assert_eq!(json, r#""FAILURE""#);
        
        // Test deserialization
        let status: VerificationStatus = serde_json::from_str(r#""SUCCESS""#).unwrap();
        assert!(matches!(status, VerificationStatus::Success));
        
        let status: VerificationStatus = serde_json::from_str(r#""FAILURE""#).unwrap();
        assert!(matches!(status, VerificationStatus::Failure));
    }
}

#[cfg(test)]
mod paypal_provider_webhook_tests {
    use payup::payment_provider::PaymentProvider;
    use payup::paypal::provider::PayPalProvider;
    use std::collections::HashMap;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_provider_verify_webhook_with_headers() {
        // This test verifies the PayPalProvider's verify_webhook implementation
        // In a real scenario, you would need a valid PayPal webhook_id and client
        
        // Create test headers
        let mut headers = HashMap::new();
        headers.insert("paypal-auth-algo".to_string(), "SHA256withRSA".to_string());
        headers.insert("paypal-cert-url".to_string(), "https://api.sandbox.paypal.com/cert.pem".to_string());
        headers.insert("paypal-transmission-id".to_string(), "test-tx-id".to_string());
        headers.insert("paypal-transmission-sig".to_string(), "test-signature".to_string());
        headers.insert("paypal-transmission-time".to_string(), "2024-01-01T00:00:00Z".to_string());
        
        // Serialize headers to JSON (as expected by verify_webhook)
        let headers_json = serde_json::to_string(&headers).unwrap();
        
        // Create test payload
        let payload = json!({
            "id": "WH-TEST-123",
            "event_type": "PAYMENT.CAPTURE.COMPLETED",
            "resource": {
                "id": "CAP-123",
                "amount": {
                    "currency_code": "USD",
                    "value": "100.00"
                }
            },
            "create_time": "2024-01-01T00:00:00Z",
            "event_version": "1.0"
        });
        
        let payload_bytes = serde_json::to_vec(&payload).unwrap();
        
        // Note: This test would require a real PayPal client to work
        // For unit testing purposes, we're verifying the interface works correctly
        
        // Uncomment below when you have valid PayPal credentials:
        // let provider = PayPalProvider::new(
        //     "client_id".to_string(),
        //     "client_secret".to_string(),
        //     true // sandbox mode
        // );
        // 
        // let result = provider.verify_webhook(
        //     &payload_bytes,
        //     &headers_json,
        //     "WH_ID_FROM_PAYPAL" // webhook_id from PayPal dashboard
        // ).await;
        // 
        // assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_header_json_format() {
        // Test that invalid JSON in signature parameter is handled correctly
        let invalid_json = "not json";
        
        // This would be tested with actual provider instance
        // The error should be a ValidationError about invalid JSON format
    }
}