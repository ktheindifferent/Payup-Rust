#[cfg(feature = "square")]
#[cfg(test)]
mod square_simple_tests {
    use payup::square::{SquareClient, SquareConfig, Environment, Money};
    use payup::square::payments::CreatePaymentRequest;
    use payup::square::customers::CreateCustomerRequest;
    
    // Helper function to create a test client
    fn create_test_client() -> SquareClient {
        let config = SquareConfig {
            access_token: "test_access_token".to_string(),
            environment: Environment::Sandbox,
            location_id: Some("test_location".to_string()),
        };
        
        SquareClient::new(config).expect("Failed to create Square client")
    }
    
    #[test]
    fn test_square_client_creation() {
        let client = create_test_client();
        assert!(matches!(client.config.environment, Environment::Sandbox));
        assert_eq!(client.config.location_id, Some("test_location".to_string()));
    }
    
    #[test]
    fn test_environment_base_url() {
        let sandbox = Environment::Sandbox;
        let production = Environment::Production;
        
        assert_eq!(sandbox.base_url(), "https://connect.squareupsandbox.com");
        assert_eq!(production.base_url(), "https://connect.squareup.com");
    }
    
    #[test]
    fn test_money_serialization() {
        let money = Money {
            amount: 1000, // $10.00 in cents
            currency: "USD".to_string(),
        };
        
        let json = serde_json::to_string(&money).unwrap();
        assert!(json.contains("\"amount\":1000"));
        assert!(json.contains("\"currency\":\"USD\""));
        
        // Test deserialization
        let deserialized: Money = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.amount, 1000);
        assert_eq!(deserialized.currency, "USD");
    }
    
    #[test]
    fn test_create_payment_request_serialization() {
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
        
        let json = serde_json::to_string(&payment_request).unwrap();
        assert!(json.contains("\"source_id\":\"cnon:card_nonce_123\""));
        assert!(json.contains("\"idempotency_key\":\"unique_key_123\""));
        assert!(json.contains("\"amount\":2500"));
    }
    
    #[test]
    fn test_create_customer_request_serialization() {
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
        
        let json = serde_json::to_string(&customer_request).unwrap();
        assert!(json.contains("\"given_name\":\"Jane\""));
        assert!(json.contains("\"family_name\":\"Smith\""));
        assert!(json.contains("\"email_address\":\"jane.smith@example.com\""));
    }
    
    #[test]
    fn test_square_auth() {
        use payup::square::SquareAuth;
        
        let auth = SquareAuth::new(
            "test_token".to_string(),
            Environment::Sandbox
        );
        
        // Test that auth validates correctly
        assert!(auth.validate().is_ok());
        
        // Test authorization header
        assert_eq!(auth.authorization_header(), "Bearer test_token");
        
        // Test base URL for sandbox
        assert_eq!(auth.base_url(), "https://connect.squareupsandbox.com");
    }
    
    #[test]
    fn test_address_structure() {
        use payup::square::Address;
        
        let address = Address {
            address_line_1: Some("123 Main St".to_string()),
            address_line_2: Some("Apt 4B".to_string()),
            address_line_3: None,
            locality: Some("San Francisco".to_string()),
            sublocality: None,
            administrative_district_level_1: Some("CA".to_string()),
            postal_code: Some("94107".to_string()),
            country: "US".to_string(),
        };
        
        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("\"address_line_1\":\"123 Main St\""));
        assert!(json.contains("\"locality\":\"San Francisco\""));
        assert!(json.contains("\"country\":\"US\""));
    }
    
    #[test]
    fn test_api_response_structure() {
        use payup::square::ApiResponse;
        
        let response: ApiResponse<Money> = ApiResponse {
            data: Some(Money {
                amount: 100,
                currency: "USD".to_string(),
            }),
            errors: None,
            cursor: None,
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"amount\":100"));
    }
}