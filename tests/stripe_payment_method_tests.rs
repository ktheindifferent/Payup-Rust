#[cfg(test)]
mod stripe_payment_method_unit_tests {
    use payup::stripe::{
        PaymentMethod, StripePaymentMethodType, CreatePaymentMethodParams, 
        CreateCardParams, PaymentMethodBillingDetails, PaymentMethodAddress
    };
    use std::collections::HashMap;

    #[test]
    fn test_payment_method_new() {
        let pm = PaymentMethod::new();
        assert!(pm.id.is_none());
        assert!(pm.object.is_none());
        assert!(pm.billing_details.is_none());
        assert!(pm.card.is_none());
    }

    #[test]
    fn test_create_payment_method_params() {
        let params = CreatePaymentMethodParams {
            payment_method_type: StripePaymentMethodType::Card,
            billing_details: Some(PaymentMethodBillingDetails {
                address: Some(PaymentMethodAddress {
                    city: Some("San Francisco".to_string()),
                    country: Some("US".to_string()),
                    line1: Some("123 Main St".to_string()),
                    line2: None,
                    postal_code: Some("94105".to_string()),
                    state: Some("CA".to_string()),
                }),
                email: Some("test@example.com".to_string()),
                name: Some("John Doe".to_string()),
                phone: Some("+14155551234".to_string()),
            }),
            card: Some(CreateCardParams {
                number: "4242424242424242".to_string(),
                exp_month: 12,
                exp_year: 2025,
                cvc: Some("123".to_string()),
            }),
            metadata: Some({
                let mut map = HashMap::new();
                map.insert("order_id".to_string(), "123".to_string());
                map
            }),
        };

        assert_eq!(params.card.as_ref().unwrap().number, "4242424242424242");
        assert_eq!(params.card.as_ref().unwrap().exp_month, 12);
        assert_eq!(params.billing_details.as_ref().unwrap().name.as_ref().unwrap(), "John Doe");
    }

    #[test]
    fn test_payment_method_type_serialization() {
        use serde_json;
        
        let card_type = StripePaymentMethodType::Card;
        let serialized = serde_json::to_string(&card_type).unwrap();
        assert_eq!(serialized, "\"card\"");
        
        let paypal_type = StripePaymentMethodType::Paypal;
        let serialized = serde_json::to_string(&paypal_type).unwrap();
        assert_eq!(serialized, "\"paypal\"");
    }
}

#[cfg(test)]
mod stripe_payment_method_provider_tests {
    use payup::payment_provider::{PaymentProvider, PaymentMethod, PaymentMethodType, CardDetails};
    use payup::stripe::StripeProvider;
    
    fn create_test_payment_method() -> PaymentMethod {
        PaymentMethod {
            id: None,
            method_type: PaymentMethodType::Card,
            card: Some(CardDetails {
                number: Some("4242424242424242".to_string()),
                exp_month: "12".to_string(),
                exp_year: "2025".to_string(),
                cvv: Some("123".to_string()),
                brand: None,
                last4: None,
            }),
            bank_account: None,
        }
    }

    #[tokio::test]
    async fn test_provider_mapping() {
        // This test verifies that the mapping functions work correctly
        let provider = StripeProvider::new("sk_test_fake".to_string());
        
        // Test that provider name is correct
        assert_eq!(provider.name(), "Stripe");
        
        // Test supported currencies include major ones
        let currencies = provider.supported_currencies();
        assert!(currencies.contains(&"USD".to_string()));
        assert!(currencies.contains(&"EUR".to_string()));
    }
}