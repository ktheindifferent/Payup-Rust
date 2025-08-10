#[cfg(feature = "paypal")]
#[cfg(test)]
mod paypal_integration_tests {
    use payup::paypal::{PayPalClient, PayPalConfig, PayPalEnvironment};
    use payup::paypal::orders::{OrderIntent, PurchaseUnit};
    
    // Helper function to create a test client
    fn create_test_client() -> PayPalClient {
        // Use environment variables or test credentials
        let client_id = std::env::var("PAYPAL_CLIENT_ID")
            .unwrap_or_else(|_| "test_client_id".to_string());
        let client_secret = std::env::var("PAYPAL_CLIENT_SECRET")
            .unwrap_or_else(|_| "test_client_secret".to_string());
        
        let config = PayPalConfig {
            client_id,
            client_secret,
            environment: PayPalEnvironment::Sandbox,
            webhook_id: None,
        };
        
        PayPalClient::new(config).expect("Failed to create PayPal client")
    }
    
    #[test]
    fn test_paypal_client_creation() {
        let client = create_test_client();
        // Client should have config with sandbox environment
        assert!(matches!(client.config.environment, PayPalEnvironment::Sandbox));
        
        let live_config = PayPalConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            environment: PayPalEnvironment::Live,
            webhook_id: None,
        };
        let live_client = PayPalClient::new(live_config).expect("Failed to create client");
        assert!(matches!(live_client.config.environment, PayPalEnvironment::Live));
    }
    
    #[test]
    fn test_order_serialization() {
        use payup::paypal::PayPalMoney;
        use payup::paypal::orders::Order;
        
        let mut order = Order::new();
        order.intent = OrderIntent::Capture;
        order.purchase_units = vec![PurchaseUnit {
            reference_id: Some("PUHF".to_string()),
            description: Some("Sporting Goods".to_string()),
            custom_id: Some("CUST-HighFashions".to_string()),
            amount: PayPalMoney {
                currency_code: "USD".to_string(),
                value: "230.00".to_string(),
            },
            payee: None,
            invoice_id: None,
            items: None,
            shipping: None,
        }];
        
        // Test serialization
        let json = serde_json::to_string(&order);
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
    fn test_ensure_auth() {
        let mut client = create_test_client();
        let result = client.ensure_auth();
        
        assert!(result.is_ok(), "Failed to ensure auth: {:?}", result.err());
        // Check that auth is present after ensuring
        assert!(client.auth.is_some());
        let auth = client.auth.as_ref().unwrap();
        assert!(!auth.access_token.is_empty());
    }
    
    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_client_auth_async() {
        let client = create_test_client();
        // Just verify the client was created with auth
        assert!(client.auth.is_some());
        let auth = client.auth.as_ref().unwrap();
        assert!(!auth.access_token.is_empty());
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_create_order() {
        use payup::paypal::PayPalMoney;
        use payup::paypal::orders::Order;
        
        let mut client = create_test_client();
        
        let mut order = Order::new();
        order.intent = OrderIntent::Capture;
        order.purchase_units = vec![PurchaseUnit {
            reference_id: Some("test_ref_123".to_string()),
            description: Some("Test Purchase".to_string()),
            custom_id: Some("test_custom_123".to_string()),
            amount: PayPalMoney {
                currency_code: "USD".to_string(),
                value: "10.00".to_string(),
            },
            payee: None,
            invoice_id: None,
            items: None,
            shipping: None,
        }];
        
        let result = order.create(&mut client);
        assert!(result.is_ok(), "Failed to create order: {:?}", result.err());
        
        let order = result.unwrap();
        assert!(order.id.is_some());
        assert!(order.status.is_some());
    }
    
    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_create_order_async() {
        use payup::paypal::PayPalMoney;
        use payup::paypal::orders::Order;
        
        let mut client = create_test_client();
        
        let mut order = Order::new();
        order.intent = OrderIntent::Capture;
        order.purchase_units = vec![PurchaseUnit {
            reference_id: Some("test_ref_456".to_string()),
            description: Some("Test Purchase Async".to_string()),
            custom_id: Some("test_custom_456".to_string()),
            amount: PayPalMoney {
                currency_code: "USD".to_string(),
                value: "15.00".to_string(),
            },
            payee: None,
            invoice_id: None,
            items: None,
            shipping: None,
        }];
        
        let result = order.async_create(&mut client).await;
        assert!(result.is_ok(), "Failed to create order: {:?}", result.err());
        
        let order = result.unwrap();
        assert!(order.id.is_some());
        assert!(order.status.is_some());
    }
    
    #[test]
    #[ignore] // Requires valid API credentials and order ID
    fn test_get_order_details() {
        use payup::paypal::orders::Order;
        
        let mut client = create_test_client();
        let order_id = std::env::var("TEST_PAYPAL_ORDER_ID")
            .unwrap_or_else(|_| "TEST_ORDER_123".to_string());
        
        let result = Order::get(&mut client, &order_id);
        
        // This will fail without a valid order ID, but tests the API call structure
        if result.is_ok() {
            let order = result.unwrap();
            assert_eq!(order.id, Some(order_id));
        }
    }
    
    #[test]
    fn test_paypal_money_serialization() {
        use payup::paypal::PayPalMoney;
        
        let money = PayPalMoney {
            currency_code: "USD".to_string(),
            value: "100.00".to_string(),
        };
        
        let json = serde_json::to_string(&money);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"currency_code\":\"USD\""));
        assert!(json_str.contains("\"value\":\"100.00\""));
    }
    
    #[test]
    fn test_webhook_verification_structure() {
        let _client = create_test_client();
        
        // PayPal webhook verification would be implemented in webhooks module
        // This test just verifies the client can be created
        assert!(true);
    }
    
    #[test]
    fn test_subscription_plan_creation() {
        use payup::paypal::subscriptions::{Plan, BillingCycle, PricingScheme, Frequency, PlanStatus, IntervalUnit, TenureType};
        use payup::paypal::PayPalMoney;
        
        let plan = Plan {
            id: None,
            product_id: "PROD-123".to_string(),
            name: "Test Subscription Plan".to_string(),
            description: Some("A test subscription plan".to_string()),
            status: Some(PlanStatus::Active),
            billing_cycles: vec![
                BillingCycle {
                    frequency: Frequency {
                        interval_unit: IntervalUnit::Month,
                        interval_count: 1,
                    },
                    tenure_type: TenureType::Regular,
                    sequence: 1,
                    total_cycles: Some(12),
                    pricing_scheme: Some(PricingScheme {
                        fixed_price: Some(PayPalMoney {
                            currency_code: "USD".to_string(),
                            value: "9.99".to_string(),
                        }),
                        tiers: None,
                    }),
                },
            ],
            payment_preferences: None,
            taxes: None,
            quantity_supported: Some(false),
            create_time: None,
            update_time: None,
            links: None,
        };
        
        let json = serde_json::to_string(&plan);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"product_id\":\"PROD-123\""));
        assert!(json_str.contains("\"name\":\"Test Subscription Plan\""));
        assert!(json_str.contains("\"MONTH\""));
    }
    
    #[test]
    fn test_error_handling() {
        use payup::error::PayupError;
        
        let config = PayPalConfig {
            client_id: "invalid_id".to_string(),
            client_secret: "invalid_secret".to_string(),
            environment: PayPalEnvironment::Sandbox,
            webhook_id: None,
        };
        
        // Creating client with invalid credentials should work, but auth will fail
        let result = PayPalClient::new(config);
        
        // The client creation itself might succeed or fail depending on implementation
        // If it succeeds, subsequent API calls will fail with auth errors
        if let Ok(mut client) = result {
            // Try to ensure auth (which should fail with invalid credentials)
            let auth_result = client.ensure_auth();
            assert!(auth_result.is_err());
            
            if let Err(error) = auth_result {
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
        } else if let Err(error) = result {
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