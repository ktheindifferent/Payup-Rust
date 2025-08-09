#[cfg(feature = "square")]
#[cfg(test)]
mod square_tests {
    use payup::square::{SquareClient, SquareConfig};
    use payup::square::payments::{CreatePaymentRequest, Money};
    use payup::square::customers::CreateCustomerRequest;
    use payup::error::Result;
    
    // Helper function to create a test client
    fn create_test_client() -> SquareClient {
        let config = SquareConfig {
            access_token: "test_access_token".to_string(),
            environment: "sandbox".to_string(),
            api_version: "2024-01-17".to_string(),
        };
        
        SquareClient::new(config).expect("Failed to create Square client")
    }
    
    #[test]
    fn test_square_client_creation() {
        let client = create_test_client();
        assert_eq!(client.config.environment, "sandbox");
        assert_eq!(client.config.api_version, "2024-01-17");
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
    }
    
    #[test]
    fn test_create_payment_request_structure() {
        let payment_request = CreatePaymentRequest {
            source_id: "cnon:card_nonce_123".to_string(),
            idempotency_key: "unique_key_123".to_string(),
            amount_money: Money {
                amount: 2500,
                currency: "USD".to_string(),
            },
            tip_money: Some(Money {
                amount: 500,
                currency: "USD".to_string(),
            }),
            app_fee_money: None,
            autocomplete: Some(true),
            order_id: None,
            customer_id: Some("CUST_123".to_string()),
            location_id: Some("LOC_123".to_string()),
            reference_id: Some("REF_123".to_string()),
            verification_token: None,
            note: Some("Test payment".to_string()),
        };
        
        let json = serde_json::to_string(&payment_request);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"source_id\":\"cnon:card_nonce_123\""));
        assert!(json_str.contains("\"idempotency_key\":\"unique_key_123\""));
        assert!(json_str.contains("\"amount\":2500"));
    }
    
    #[test]
    fn test_create_customer_request_structure() {
        let customer_request = CreateCustomerRequest {
            idempotency_key: Some("unique_key_456".to_string()),
            given_name: Some("Jane".to_string()),
            family_name: Some("Smith".to_string()),
            company_name: Some("Acme Corp".to_string()),
            nickname: None,
            email_address: Some("jane.smith@example.com".to_string()),
            address: None,
            phone_number: Some("+14155555678".to_string()),
            reference_id: Some("ref_456".to_string()),
            note: Some("VIP customer".to_string()),
            birthday: None,
        };
        
        let json = serde_json::to_string(&customer_request);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"given_name\":\"Jane\""));
        assert!(json_str.contains("\"family_name\":\"Smith\""));
        assert!(json_str.contains("\"email_address\":\"jane.smith@example.com\""));
    }
    
    #[test]
    fn test_square_auth_validation() {
        use payup::square::SquareAuth;
        
        let auth = SquareAuth::new(
            "test_token".to_string(),
            "sandbox".to_string()
        );
        
        // Test that auth validates correctly
        assert!(auth.validate().is_ok());
        
        // Test authorization header
        assert_eq!(auth.authorization_header(), "Bearer test_token");
        
        // Test base URL for sandbox
        assert!(auth.base_url().contains("sandbox"));
    }
    
    #[test]
    fn test_catalog_object_structure() {
        use payup::square::catalog::{CatalogObject, CatalogItem};
        
        let item = CatalogObject {
            object_type: "ITEM".to_string(),
            id: Some("ITEM_789".to_string()),
            updated_at: Some("2024-01-01T00:00:00Z".to_string()),
            created_at: None,
            version: Some(1),
            is_deleted: Some(false),
            present_at_all_locations: Some(true),
            present_at_location_ids: None,
            absent_at_location_ids: None,
            catalog_v1_ids: None,
            item_data: Some(CatalogItem {
                name: Some("Test Item".to_string()),
                description: Some("A test item".to_string()),
                abbreviation: Some("TI".to_string()),
                label_color: None,
                available_online: Some(true),
                available_for_pickup: Some(false),
                available_electronically: Some(false),
                category_id: None,
                tax_ids: None,
                modifier_list_info: None,
                variations: None,
                product_type: Some("REGULAR".to_string()),
                skip_modifier_screen: Some(false),
                item_options: None,
            }),
            category_data: None,
            item_variation_data: None,
            tax_data: None,
            discount_data: None,
            modifier_list_data: None,
            modifier_data: None,
            time_period_data: None,
            product_set_data: None,
            pricing_rule_data: None,
            image_data: None,
            measurement_unit_data: None,
            item_option_data: None,
            item_option_value_data: None,
        };
        
        let json = serde_json::to_string(&item);
        assert!(json.is_ok());
        
        let json_str = json.unwrap();
        assert!(json_str.contains("\"object_type\":\"ITEM\""));
        assert!(json_str.contains("\"name\":\"Test Item\""));
    }
}