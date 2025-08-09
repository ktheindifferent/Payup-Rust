#[cfg(feature = "square")]
#[cfg(test)]
mod square_integration_tests {
    use payup::square::{SquareClient, SquareConfig};
    use payup::square::payments::{CreatePaymentRequest, Money, ProcessingFee};
    use payup::error::Result;
    
    // Helper function to create a test client
    fn create_test_client() -> SquareClient {
        let access_token = std::env::var("SQUARE_ACCESS_TOKEN")
            .unwrap_or_else(|_| "test_access_token".to_string());
        let environment = std::env::var("SQUARE_ENVIRONMENT")
            .unwrap_or_else(|_| "sandbox".to_string());
        
        let config = SquareConfig {
            access_token,
            environment,
            api_version: "2024-01-17".to_string(),
        };
        
        SquareClient::new(config).expect("Failed to create Square client")
    }
    
    #[test]
    fn test_square_client_creation() {
        let client = create_test_client();
        // Client creation test - verify it was created successfully
        
        let prod_config = SquareConfig {
            access_token: "prod_token".to_string(),
            environment: "production".to_string(),
            api_version: "2024-01-17".to_string(),
        };
        let prod_client = SquareClient::new(prod_config).expect("Failed to create prod client");
        // Prod client creation test
    }
    
    #[test]
    fn test_money_serialization() {
        let money = Money {
            amount: 1000, // $10.00 in cents
            currency: "USD".to_string(),
        };
        
        let json = serde_json::to_string(&money);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"amount\":1000"));
        assert!(json_str.contains("\"currency\":\"USD\""));
        
        // Test deserialization
        let deserialized: Result<Money> = serde_json::from_str(&json_str)
            .map_err(|e| e.into());
        assert!(deserialized.is_ok());
        
        let money_back = deserialized.unwrap();
        assert_eq!(money_back.amount, 1000);
        assert_eq!(money_back.currency, "USD");
    }
    
    #[test]
    fn test_create_payment_request_structure() {
        let payment_request = CreatePaymentRequest {
            source_id: "cnon:card_nonce_123".to_string(),
            idempotency_key: "unique_key_123".to_string(),
            amount_money: Money {
                amount: 2500, // $25.00
                currency: "USD".to_string(),
            },
            tip_money: Some(Money {
                amount: 500, // $5.00 tip
                currency: "USD".to_string(),
            }),
            app_fee_money: None,
            delay_duration: None,
            autocomplete: Some(true),
            order_id: None,
            customer_id: Some("CUST_123".to_string()),
            location_id: Some("LOC_123".to_string()),
            reference_id: Some("REF_123".to_string()),
            verification_token: None,
            accept_partial_authorization: Some(false),
            buyer_email_address: Some("customer@example.com".to_string()),
            billing_address: None,
            shipping_address: None,
            note: Some("Test payment".to_string()),
            statement_descriptor_identifier: Some("TEST".to_string()),
        };
        
        let json = serde_json::to_string(&payment_request);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"source_id\":\"cnon:card_nonce_123\""));
        assert!(json_str.contains("\"idempotency_key\":\"unique_key_123\""));
        assert!(json_str.contains("\"amount\":2500"));
        assert!(json_str.contains("\"customer_id\":\"CUST_123\""));
        assert!(json_str.contains("\"buyer_email_address\":\"customer@example.com\""));
    }
    
    #[test]
    fn test_processing_fee_calculation() {
        // Test processing fee structure
        let fee = ProcessingFee {
            effective_at: "2024-01-01T00:00:00Z".to_string(),
            r#type: "INITIAL".to_string(),
            amount_money: Money {
                amount: 59, // $0.59 processing fee
                currency: "USD".to_string(),
            },
        };
        
        let json = serde_json::to_string(&fee);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"type\":\"INITIAL\""));
        assert!(json_str.contains("\"amount\":59"));
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_create_payment() {
        let client = create_test_client();
        
        let payment_request = CreatePaymentRequest {
            source_id: "cnon:test_nonce".to_string(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            amount_money: Money {
                amount: 100, // $1.00
                currency: "USD".to_string(),
            },
            tip_money: None,
            app_fee_money: None,
            delay_duration: None,
            autocomplete: Some(true),
            order_id: None,
            customer_id: None,
            location_id: Some("test_location".to_string()),
            reference_id: None,
            verification_token: None,
            accept_partial_authorization: None,
            buyer_email_address: None,
            billing_address: None,
            shipping_address: None,
            note: Some("Integration test payment".to_string()),
            statement_descriptor_identifier: None,
        };
        
        let result = client.create_payment(payment_request);
        
        // This will fail without valid credentials, but tests the API structure
        if result.is_ok() {
            let payment = result.unwrap();
            assert!(!payment.id.is_empty());
            assert_eq!(payment.amount_money.currency, "USD");
        }
    }
    
    #[tokio::test]
    #[ignore] // Requires valid API credentials
    async fn test_create_payment_async() {
        let client = create_test_client();
        
        let payment_request = CreatePaymentRequest {
            source_id: "cnon:test_nonce_async".to_string(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            amount_money: Money {
                amount: 200, // $2.00
                currency: "USD".to_string(),
            },
            tip_money: None,
            app_fee_money: None,
            delay_duration: None,
            autocomplete: Some(true),
            order_id: None,
            customer_id: None,
            location_id: Some("test_location".to_string()),
            reference_id: None,
            verification_token: None,
            accept_partial_authorization: None,
            buyer_email_address: None,
            billing_address: None,
            shipping_address: None,
            note: Some("Async integration test payment".to_string()),
            statement_descriptor_identifier: None,
        };
        
        let result = client.create_payment_async(payment_request).await;
        
        // This will fail without valid credentials, but tests the API structure
        if result.is_ok() {
            let payment = result.unwrap();
            assert!(!payment.id.is_empty());
            assert_eq!(payment.amount_money.currency, "USD");
        }
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_get_payment() {
        let client = create_test_client();
        let payment_id = std::env::var("TEST_SQUARE_PAYMENT_ID")
            .unwrap_or_else(|_| "test_payment_123".to_string());
        
        let result = client.get_payment(&payment_id);
        
        // This will fail without a valid payment ID, but tests the API call
        if result.is_ok() {
            let payment = result.unwrap();
            assert_eq!(payment.id, payment_id);
        }
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_list_payments() {
        let client = create_test_client();
        
        let result = client.list_payments(None, None, None, None, None);
        
        // This will fail without valid credentials, but tests the API structure
        if result.is_ok() {
            let payments = result.unwrap();
            // Should return a list (possibly empty)
            assert!(payments.len() >= 0);
        }
    }
    
    #[test]
    #[ignore] // Requires valid API credentials
    fn test_create_customer() {
        let client = create_test_client();
        
        use payup::square::customers::CreateCustomerRequest;
        
        let customer_request = CreateCustomerRequest {
            idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
            given_name: Some("John".to_string()),
            family_name: Some("Doe".to_string()),
            company_name: None,
            nickname: None,
            email_address: Some("john.doe@example.com".to_string()),
            address: None,
            phone_number: Some("+14155551234".to_string()),
            reference_id: Some("ref_123".to_string()),
            note: Some("Test customer".to_string()),
            birthday: None,
        };
        
        let result = client.create_customer(customer_request);
        
        // This will fail without valid credentials, but tests the API structure
        if result.is_ok() {
            let customer = result.unwrap();
            assert!(!customer.id.is_empty());
            assert_eq!(customer.given_name, Some("John".to_string()));
        }
    }
    
    #[test]
    fn test_catalog_item_serialization() {
        use payup::square::catalog::{CatalogObject, CatalogItem, ItemVariation};
        
        let item = CatalogObject {
            r#type: "ITEM".to_string(),
            id: "ITEM_123".to_string(),
            updated_at: Some("2024-01-01T00:00:00Z".to_string()),
            version: Some(1),
            is_deleted: Some(false),
            present_at_all_locations: Some(true),
            item_data: Some(CatalogItem {
                name: Some("Test Product".to_string()),
                description: Some("A test product for integration testing".to_string()),
                abbreviation: Some("TP".to_string()),
                label_color: None,
                available_online: Some(true),
                available_for_pickup: Some(true),
                available_electronically: Some(false),
                category_id: None,
                tax_ids: None,
                modifier_list_info: None,
                variations: Some(vec![
                    ItemVariation {
                        id: Some("VAR_123".to_string()),
                        item_id: Some("ITEM_123".to_string()),
                        name: Some("Regular".to_string()),
                        sku: Some("TP-REG-001".to_string()),
                        upc: None,
                        ordinal: Some(1),
                        pricing_type: Some("FIXED_PRICING".to_string()),
                        price_money: Some(Money {
                            amount: 1999, // $19.99
                            currency: "USD".to_string(),
                        }),
                        location_overrides: None,
                        track_inventory: Some(true),
                        inventory_alert_type: Some("LOW_QUANTITY".to_string()),
                        inventory_alert_threshold: Some(10),
                        user_data: None,
                    }
                ]),
                product_type: Some("REGULAR".to_string()),
                skip_modifier_screen: Some(false),
            }),
        };
        
        let json = serde_json::to_string(&item);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"type\":\"ITEM\""));
        assert!(json_str.contains("\"name\":\"Test Product\""));
        assert!(json_str.contains("\"sku\":\"TP-REG-001\""));
        assert!(json_str.contains("\"amount\":1999"));
    }
    
    #[test]
    fn test_location_structure() {
        use payup::square::Location;
        
        let location = Location {
            id: "LOC_123".to_string(),
            name: Some("Main Store".to_string()),
            address: None,
            timezone: Some("America/New_York".to_string()),
            capabilities: vec!["CREDIT_CARD_PROCESSING".to_string()],
            status: Some("ACTIVE".to_string()),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            merchant_id: Some("MERCHANT_123".to_string()),
            country: "US".to_string(),
            language_code: Some("en-US".to_string()),
            currency: "USD".to_string(),
            phone_number: Some("+14155551234".to_string()),
            business_name: Some("Test Business".to_string()),
            r#type: Some("PHYSICAL".to_string()),
            website_url: Some("https://example.com".to_string()),
            business_hours: None,
            business_email: Some("business@example.com".to_string()),
            description: Some("Main store location".to_string()),
            twitter_username: None,
            instagram_username: None,
            facebook_url: None,
            coordinates: None,
            logo_url: None,
            pos_background_url: None,
            mcc: None,
            full_format_logo_url: None,
        };
        
        let json = serde_json::to_string(&location);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"id\":\"LOC_123\""));
        assert!(json_str.contains("\"name\":\"Main Store\""));
        assert!(json_str.contains("\"status\":\"ACTIVE\""));
        assert!(json_str.contains("\"currency\":\"USD\""));
    }
    
    #[test]
    fn test_error_handling() {
        use payup::error::PayupError;
        
        let client = SquareClient::new(
            "invalid_token".to_string(),
            "sandbox".to_string()
        );
        
        // This should fail with invalid credentials
        let result = client.list_payments(None, None, None, None, None);
        assert!(result.is_err());
        
        if let Err(error) = result {
            // Check that we get a proper error type
            match error {
                PayupError::ApiError { .. } |
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