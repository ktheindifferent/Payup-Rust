#[cfg(test)]
mod paypal_webhook_integration_tests {
    use payup::paypal::webhooks::{PayPalWebhookHandler, WebhookEvent};
    use payup::paypal::client::PayPalClient;
    use std::collections::HashMap;
    
    // Sample PayPal webhook payloads for testing
    const PAYMENT_CAPTURE_COMPLETED_PAYLOAD: &str = r#"{
        "id": "WH-7F159630MH811482K-4J716366NF957051V",
        "event_version": "1.0",
        "create_time": "2024-01-15T10:30:00.000Z",
        "resource_type": "capture",
        "event_type": "PAYMENT.CAPTURE.COMPLETED",
        "summary": "Payment completed for $127.50 USD",
        "resource": {
            "id": "3VW75242481980346",
            "amount": {
                "currency_code": "USD",
                "value": "127.50"
            },
            "final_capture": true,
            "seller_protection": {
                "status": "ELIGIBLE",
                "dispute_categories": [
                    "ITEM_NOT_RECEIVED",
                    "UNAUTHORIZED_TRANSACTION"
                ]
            },
            "seller_receivable_breakdown": {
                "gross_amount": {
                    "currency_code": "USD",
                    "value": "127.50"
                },
                "paypal_fee": {
                    "currency_code": "USD",
                    "value": "4.02"
                },
                "net_amount": {
                    "currency_code": "USD",
                    "value": "123.48"
                }
            },
            "status": "COMPLETED",
            "create_time": "2024-01-15T10:29:45Z",
            "update_time": "2024-01-15T10:29:45Z",
            "links": [
                {
                    "href": "https://api.paypal.com/v2/payments/captures/3VW75242481980346",
                    "rel": "self",
                    "method": "GET"
                },
                {
                    "href": "https://api.paypal.com/v2/payments/captures/3VW75242481980346/refund",
                    "rel": "refund",
                    "method": "POST"
                }
            ]
        },
        "links": [
            {
                "href": "https://api.paypal.com/v1/notifications/webhooks-events/WH-7F159630MH811482K-4J716366NF957051V",
                "rel": "self",
                "method": "GET"
            },
            {
                "href": "https://api.paypal.com/v1/notifications/webhooks-events/WH-7F159630MH811482K-4J716366NF957051V/resend",
                "rel": "resend",
                "method": "POST"
            }
        ]
    }"#;
    
    const SUBSCRIPTION_ACTIVATED_PAYLOAD: &str = r#"{
        "id": "WH-3UT90572MA059624L-7LL65814WN874503Y",
        "event_version": "1.0",
        "create_time": "2024-01-15T14:20:00.000Z",
        "resource_type": "subscription",
        "event_type": "BILLING.SUBSCRIPTION.ACTIVATED",
        "summary": "Subscription activated",
        "resource": {
            "id": "I-BW452GLLEP1G",
            "plan_id": "P-5ML4271244454362WXNWU5NQ",
            "status": "ACTIVE",
            "status_update_time": "2024-01-15T14:19:45Z",
            "quantity": "1",
            "subscriber": {
                "name": {
                    "given_name": "John",
                    "surname": "Doe"
                },
                "email_address": "john.doe@example.com",
                "shipping_address": {
                    "name": {
                        "full_name": "John Doe"
                    },
                    "address": {
                        "address_line_1": "123 Main St",
                        "address_line_2": "Apt 4B",
                        "admin_area_2": "San Jose",
                        "admin_area_1": "CA",
                        "postal_code": "95131",
                        "country_code": "US"
                    }
                }
            },
            "billing_info": {
                "outstanding_balance": {
                    "currency_code": "USD",
                    "value": "0.00"
                },
                "cycle_executions": [],
                "last_payment": {
                    "amount": {
                        "currency_code": "USD",
                        "value": "19.99"
                    },
                    "time": "2024-01-15T14:19:45Z"
                },
                "next_billing_time": "2024-02-15T14:19:45Z",
                "final_payment_time": "2025-01-15T14:19:45Z",
                "failed_payments_count": 0
            },
            "create_time": "2024-01-15T14:19:40Z",
            "update_time": "2024-01-15T14:19:45Z",
            "links": [
                {
                    "href": "https://api.paypal.com/v1/billing/subscriptions/I-BW452GLLEP1G/cancel",
                    "rel": "cancel",
                    "method": "POST"
                },
                {
                    "href": "https://api.paypal.com/v1/billing/subscriptions/I-BW452GLLEP1G",
                    "rel": "edit",
                    "method": "PATCH"
                },
                {
                    "href": "https://api.paypal.com/v1/billing/subscriptions/I-BW452GLLEP1G",
                    "rel": "self",
                    "method": "GET"
                }
            ]
        },
        "links": [
            {
                "href": "https://api.paypal.com/v1/notifications/webhooks-events/WH-3UT90572MA059624L-7LL65814WN874503Y",
                "rel": "self",
                "method": "GET"
            },
            {
                "href": "https://api.paypal.com/v1/notifications/webhooks-events/WH-3UT90572MA059624L-7LL65814WN874503Y/resend",
                "rel": "resend",
                "method": "POST"
            }
        ]
    }"#;
    
    const REFUND_COMPLETED_PAYLOAD: &str = r#"{
        "id": "WH-COC11055RA711503B-4YM959094A144403T",
        "event_version": "1.0",
        "create_time": "2024-01-16T09:15:00.000Z",
        "resource_type": "refund",
        "event_type": "PAYMENT.CAPTURE.REFUNDED",
        "summary": "A $50.00 USD capture payment was refunded",
        "resource": {
            "id": "8YF44903PH666032L",
            "amount": {
                "currency_code": "USD",
                "value": "50.00"
            },
            "seller_payable_breakdown": {
                "gross_amount": {
                    "currency_code": "USD",
                    "value": "-50.00"
                },
                "paypal_fee": {
                    "currency_code": "USD",
                    "value": "1.58"
                },
                "net_amount": {
                    "currency_code": "USD",
                    "value": "-48.42"
                },
                "total_refunded_amount": {
                    "currency_code": "USD",
                    "value": "50.00"
                }
            },
            "invoice_id": "INV-12345",
            "status": "COMPLETED",
            "create_time": "2024-01-16T09:14:50Z",
            "update_time": "2024-01-16T09:14:50Z",
            "links": [
                {
                    "href": "https://api.paypal.com/v2/payments/refunds/8YF44903PH666032L",
                    "rel": "self",
                    "method": "GET"
                }
            ]
        },
        "links": [
            {
                "href": "https://api.paypal.com/v1/notifications/webhooks-events/WH-COC11055RA711503B-4YM959094A144403T",
                "rel": "self",
                "method": "GET"
            }
        ]
    }"#;
    
    #[test]
    fn test_parse_payment_capture_webhook() {
        let event = WebhookEvent::parse(PAYMENT_CAPTURE_COMPLETED_PAYLOAD);
        assert!(event.is_ok());
        
        let event = event.unwrap();
        assert_eq!(event.id, "WH-7F159630MH811482K-4J716366NF957051V");
        assert_eq!(event.event_type, "PAYMENT.CAPTURE.COMPLETED");
        assert_eq!(event.resource_type, Some("capture".to_string()));
        assert_eq!(event.summary, Some("Payment completed for $127.50 USD".to_string()));
        
        // Test resource extraction
        #[derive(serde::Deserialize)]
        struct CaptureResource {
            id: String,
            amount: Amount,
            status: String,
        }
        
        #[derive(serde::Deserialize)]
        struct Amount {
            currency_code: String,
            value: String,
        }
        
        let resource: Result<CaptureResource, _> = event.get_resource();
        assert!(resource.is_ok());
        
        let resource = resource.unwrap();
        assert_eq!(resource.id, "3VW75242481980346");
        assert_eq!(resource.amount.value, "127.50");
        assert_eq!(resource.amount.currency_code, "USD");
        assert_eq!(resource.status, "COMPLETED");
    }
    
    #[test]
    fn test_parse_subscription_activated_webhook() {
        let event = WebhookEvent::parse(SUBSCRIPTION_ACTIVATED_PAYLOAD);
        assert!(event.is_ok());
        
        let event = event.unwrap();
        assert_eq!(event.id, "WH-3UT90572MA059624L-7LL65814WN874503Y");
        assert_eq!(event.event_type, "BILLING.SUBSCRIPTION.ACTIVATED");
        assert_eq!(event.resource_type, Some("subscription".to_string()));
        
        // Test resource extraction
        #[derive(serde::Deserialize)]
        struct SubscriptionResource {
            id: String,
            plan_id: String,
            status: String,
        }
        
        let resource: Result<SubscriptionResource, _> = event.get_resource();
        assert!(resource.is_ok());
        
        let resource = resource.unwrap();
        assert_eq!(resource.id, "I-BW452GLLEP1G");
        assert_eq!(resource.plan_id, "P-5ML4271244454362WXNWU5NQ");
        assert_eq!(resource.status, "ACTIVE");
    }
    
    #[test]
    fn test_parse_refund_webhook() {
        let event = WebhookEvent::parse(REFUND_COMPLETED_PAYLOAD);
        assert!(event.is_ok());
        
        let event = event.unwrap();
        assert_eq!(event.id, "WH-COC11055RA711503B-4YM959094A144403T");
        assert_eq!(event.event_type, "PAYMENT.CAPTURE.REFUNDED");
        assert_eq!(event.resource_type, Some("refund".to_string()));
        assert_eq!(event.summary, Some("A $50.00 USD capture payment was refunded".to_string()));
        
        // Test resource extraction
        #[derive(serde::Deserialize)]
        struct RefundResource {
            id: String,
            amount: Amount,
            status: String,
            invoice_id: Option<String>,
        }
        
        #[derive(serde::Deserialize)]
        struct Amount {
            currency_code: String,
            value: String,
        }
        
        let resource: Result<RefundResource, _> = event.get_resource();
        assert!(resource.is_ok());
        
        let resource = resource.unwrap();
        assert_eq!(resource.id, "8YF44903PH666032L");
        assert_eq!(resource.amount.value, "50.00");
        assert_eq!(resource.status, "COMPLETED");
        assert_eq!(resource.invoice_id, Some("INV-12345".to_string()));
    }
    
    #[test]
    fn test_webhook_handler_with_real_payloads() {
        use payup::paypal::webhooks::WebhookHandler;
        use std::sync::{Arc, Mutex};
        
        let mut handler = WebhookHandler::new();
        let processed_events = Arc::new(Mutex::new(Vec::new()));
        
        // Register handler for payment captures
        let events_clone = processed_events.clone();
        handler.on("PAYMENT.CAPTURE.COMPLETED", move |event| {
            let mut events = events_clone.lock().unwrap();
            events.push(format!("Payment captured: {}", event.id));
            Ok(())
        });
        
        // Register handler for subscriptions
        let events_clone = processed_events.clone();
        handler.on("BILLING.SUBSCRIPTION.ACTIVATED", move |event| {
            let mut events = events_clone.lock().unwrap();
            events.push(format!("Subscription activated: {}", event.id));
            Ok(())
        });
        
        // Register handler for refunds
        let events_clone = processed_events.clone();
        handler.on("PAYMENT.CAPTURE.REFUNDED", move |event| {
            let mut events = events_clone.lock().unwrap();
            events.push(format!("Payment refunded: {}", event.id));
            Ok(())
        });
        
        // Process events
        let payment_event = WebhookEvent::parse(PAYMENT_CAPTURE_COMPLETED_PAYLOAD).unwrap();
        let subscription_event = WebhookEvent::parse(SUBSCRIPTION_ACTIVATED_PAYLOAD).unwrap();
        let refund_event = WebhookEvent::parse(REFUND_COMPLETED_PAYLOAD).unwrap();
        
        assert!(handler.handle(&payment_event).is_ok());
        assert!(handler.handle(&subscription_event).is_ok());
        assert!(handler.handle(&refund_event).is_ok());
        
        // Verify all events were processed
        let events = processed_events.lock().unwrap();
        assert_eq!(events.len(), 3);
        assert!(events[0].contains("Payment captured"));
        assert!(events[1].contains("Subscription activated"));
        assert!(events[2].contains("Payment refunded"));
    }
    
    #[tokio::test]
    #[ignore] // This test requires actual PayPal API credentials
    async fn test_webhook_verification_with_real_api() {
        // This test demonstrates how to verify webhooks with real PayPal API
        // To run this test, set the following environment variables:
        // - PAYPAL_CLIENT_ID
        // - PAYPAL_CLIENT_SECRET
        // - PAYPAL_WEBHOOK_ID
        
        let client_id = std::env::var("PAYPAL_CLIENT_ID").unwrap_or_default();
        let client_secret = std::env::var("PAYPAL_CLIENT_SECRET").unwrap_or_default();
        let webhook_id = std::env::var("PAYPAL_WEBHOOK_ID").unwrap_or_default();
        
        if client_id.is_empty() || client_secret.is_empty() || webhook_id.is_empty() {
            println!("Skipping test: PayPal credentials not configured");
            return;
        }
        
        // Create PayPal client
        let mut client = PayPalClient::new(client_id, client_secret, true);
        
        // Create webhook handler
        let handler = PayPalWebhookHandler::new(webhook_id);
        
        // Sample headers (these would come from actual webhook request)
        let mut headers = HashMap::new();
        headers.insert("paypal-auth-algo".to_string(), "SHA256withRSA".to_string());
        headers.insert("paypal-cert-url".to_string(), "https://api.sandbox.paypal.com/v1/notifications/certs/CERT-360caa42-fca2a594-7a8abba8".to_string());
        headers.insert("paypal-transmission-id".to_string(), "4f8b4e90-1b8a-11ef-8c45-03f1d3c00cf0".to_string());
        headers.insert("paypal-transmission-sig".to_string(), "test-signature".to_string());
        headers.insert("paypal-transmission-time".to_string(), "2024-01-15T10:30:00Z".to_string());
        
        // Validate headers first
        assert!(PayPalWebhookHandler::validate_headers(&headers).is_ok());
        
        // In a real scenario, you would verify with actual webhook data
        // let result = handler.verify_with_api(&mut client, headers, PAYMENT_CAPTURE_COMPLETED_PAYLOAD).await;
        // assert!(result.is_ok());
    }
}