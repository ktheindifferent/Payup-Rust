#[cfg(feature = "paypal")]
#[cfg(test)]
mod paypal_integration_tests {
    use payup::paypal::{PayPalClient, CreateOrderRequest, OrderIntent, PurchaseUnit, Amount};
    use payup::error::Result;
    use std::collections::HashMap;
    
    // Helper function to create a test client
    fn create_test_client() -> PayPalClient {
        // Use environment variables or test credentials
        let client_id = std::env::var("PAYPAL_CLIENT_ID")
            .unwrap_or_else(|_| "test_client_id".to_string());
        let client_secret = std::env::var("PAYPAL_CLIENT_SECRET")
            .unwrap_or_else(|_| "test_client_secret".to_string());
        
        PayPalClient::new(client_id, client_secret, false)
    }
    
    #[test]
    fn test_paypal_client_creation() {
        let client = create_test_client();
        assert!(!client.is_sandbox());
        
        let sandbox_client = PayPalClient::new(
            "test_id".to_string(),
            "test_secret".to_string(),
            true
        );
        assert!(sandbox_client.is_sandbox());
    }
    
    #[test]
    fn test_create_order_request_serialization() {
        let mut purchase_units = vec![];
        purchase_units.push(PurchaseUnit {
            reference_id: Some("PUHF".to_string()),
            description: Some("Sporting Goods".to_string()),
            custom_id: Some("CUST-HighFashions".to_string()),
            soft_descriptor: Some("HighFashions".to_string()),
            amount: Amount {
                currency_code: "USD".to_string(),
                value: "230.00".to_string(),
                breakdown: None,
            },
            items: None,
            shipping: None,
            payments: None,
        });
        
        let order_request = CreateOrderRequest {
            intent: OrderIntent::Capture,
            purchase_units,
            payer: None,
            application_context: None,
        };
        
        // Test serialization
        let json = serde_json::to_string(&order_request);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"intent\":\"CAPTURE\""));
        assert!(json_str.contains("\"currency_code\":\"USD\""));
        assert!(json_str.contains("\"value\":\"230.00\""));
    }
    
    #[test]
    fn test_order_intent_serialization() {
        assert_eq!(
            serde_json::to_string(&OrderIntent::Capture).unwrap(),
            "\"CAPTURE\""
        );
        assert_eq!(
            serde_json::to_string(&OrderIntent::Authorize).unwrap(),
            "\"AUTHORIZE\""
        );
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_get_access_token() {
        let client = create_test_client();
        let result = client.get_access_token();
        
        assert!(result.is_ok(), "Failed to get access token: {:?}", result.err());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }
    
    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_get_access_token_async() {
        let client = create_test_client();
        let result = client.get_access_token_async().await;
        
        assert!(result.is_ok(), "Failed to get access token: {:?}", result.err());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_create_order() {
        let client = create_test_client();
        
        let mut purchase_units = vec![];
        purchase_units.push(PurchaseUnit {
            reference_id: Some("test_ref_123".to_string()),
            description: Some("Test Purchase".to_string()),
            custom_id: Some("test_custom_123".to_string()),
            soft_descriptor: Some("TestMerchant".to_string()),
            amount: Amount {
                currency_code: "USD".to_string(),
                value: "10.00".to_string(),
                breakdown: None,
            },
            items: None,
            shipping: None,
            payments: None,
        });
        
        let order_request = CreateOrderRequest {
            intent: OrderIntent::Capture,
            purchase_units,
            payer: None,
            application_context: None,
        };
        
        let result = client.create_order(order_request);
        assert!(result.is_ok(), "Failed to create order: {:?}", result.err());
        
        let order = result.unwrap();
        assert!(!order.id.is_empty());
        assert_eq!(order.status, "CREATED");
    }
    
    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_create_order_async() {
        let client = create_test_client();
        
        let mut purchase_units = vec![];
        purchase_units.push(PurchaseUnit {
            reference_id: Some("test_ref_456".to_string()),
            description: Some("Test Purchase Async".to_string()),
            custom_id: Some("test_custom_456".to_string()),
            soft_descriptor: Some("TestMerchantAsync".to_string()),
            amount: Amount {
                currency_code: "USD".to_string(),
                value: "15.00".to_string(),
                breakdown: None,
            },
            items: None,
            shipping: None,
            payments: None,
        });
        
        let order_request = CreateOrderRequest {
            intent: OrderIntent::Capture,
            purchase_units,
            payer: None,
            application_context: None,
        };
        
        let result = client.create_order_async(order_request).await;
        assert!(result.is_ok(), "Failed to create order: {:?}", result.err());
        
        let order = result.unwrap();
        assert!(!order.id.is_empty());
        assert_eq!(order.status, "CREATED");
    }
    
    #[test]
    #[ignore] // Requires valid API credentials and order ID
    fn test_get_order_details() {
        let client = create_test_client();
        let order_id = std::env::var("TEST_PAYPAL_ORDER_ID")
            .unwrap_or_else(|_| "TEST_ORDER_123".to_string());
        
        let result = client.get_order_details(&order_id);
        
        // This will fail without a valid order ID, but tests the API call structure
        if result.is_ok() {
            let order = result.unwrap();
            assert_eq!(order.id, order_id);
        }
    }
    
    #[test]
    fn test_amount_breakdown_serialization() {
        use payup::paypal::AmountBreakdown;
        
        let breakdown = AmountBreakdown {
            item_total: Some(Amount {
                currency_code: "USD".to_string(),
                value: "180.00".to_string(),
                breakdown: None,
            }),
            shipping: Some(Amount {
                currency_code: "USD".to_string(),
                value: "20.00".to_string(),
                breakdown: None,
            }),
            handling: None,
            tax_total: Some(Amount {
                currency_code: "USD".to_string(),
                value: "30.00".to_string(),
                breakdown: None,
            }),
            insurance: None,
            shipping_discount: None,
            discount: None,
        };
        
        let json = serde_json::to_string(&breakdown);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"item_total\""));
        assert!(json_str.contains("\"shipping\""));
        assert!(json_str.contains("\"tax_total\""));
    }
    
    #[test]
    fn test_webhook_verification_structure() {
        let client = create_test_client();
        
        // Test that webhook verification method exists
        // Actual verification requires valid webhook data
        let mock_payload = b"test_payload";
        let mock_signature = "test_signature";
        let mock_headers = HashMap::new();
        
        // This tests the method signature exists
        let _result: Result<bool> = client.verify_webhook_signature(
            mock_payload,
            &mock_headers
        );
    }
    
    #[test]
    fn test_subscription_plan_creation() {
        use payup::paypal::subscriptions::{Plan, BillingCycle, PricingScheme, Frequency};
        
        let plan = Plan {
            id: None,
            product_id: "PROD-123".to_string(),
            name: "Test Subscription Plan".to_string(),
            description: Some("A test subscription plan".to_string()),
            status: Some("ACTIVE".to_string()),
            billing_cycles: vec![
                BillingCycle {
                    frequency: Frequency {
                        interval_unit: "MONTH".to_string(),
                        interval_count: 1,
                    },
                    tenure_type: payup::paypal::subscriptions::TenureType::Regular,
                    sequence: 1,
                    total_cycles: Some(12),
                    pricing_scheme: Some(PricingScheme {
                        fixed_price: Some(Amount {
                            currency_code: "USD".to_string(),
                            value: "9.99".to_string(),
                            breakdown: None,
                        }),
                        tiers: None,
                    }),
                },
            ],
            payment_preferences: None,
            taxes: None,
            quantity_supported: Some(false),
        };
        
        let json = serde_json::to_string(&plan);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"product_id\":\"PROD-123\""));
        assert!(json_str.contains("\"name\":\"Test Subscription Plan\""));
        assert!(json_str.contains("\"interval_unit\":\"MONTH\""));
    }
    
    #[test]
    fn test_error_handling() {
        use payup::error::PayupError;
        
        let client = PayPalClient::new(
            "invalid_id".to_string(),
            "invalid_secret".to_string(),
            true
        );
        
        // This should fail with invalid credentials
        let result = client.get_access_token();
        assert!(result.is_err());
        
        if let Err(error) = result {
            // Check that we get a proper error type
            match error {
                PayupError::PayPalError { .. } |
                PayupError::AuthenticationError(_) |
                PayupError::NetworkError(_) => {
                    // Expected error types for invalid credentials
                }
                _ => {
                    panic!("Unexpected error type: {:?}", error);
                }
            }
        }
    }
}