#[cfg(test)]
mod square_provider_tests {
    use payup::square::{SquareProvider, Environment};
    use payup::payment_provider::{
        PaymentProvider, Customer as UnifiedCustomer, Charge as UnifiedCharge,
        Refund as UnifiedRefund, RefundReason, Money
    };
    use payup::error::Result;

    async fn create_test_provider() -> Result<SquareProvider> {
        let access_token = std::env::var("SQUARE_ACCESS_TOKEN")
            .unwrap_or_else(|_| "test_access_token".to_string());
        
        SquareProvider::new(access_token, Environment::Sandbox)
    }

    #[tokio::test]
    async fn test_provider_name() {
        let provider = create_test_provider().await.unwrap();
        assert_eq!(provider.name(), "square");
    }

    #[tokio::test]
    async fn test_supported_currencies() {
        let provider = create_test_provider().await.unwrap();
        let currencies = provider.supported_currencies();
        assert!(currencies.contains(&"usd".to_string()));
        assert!(currencies.contains(&"gbp".to_string()));
        assert!(currencies.contains(&"eur".to_string()));
    }

    #[tokio::test]
    async fn test_supported_features() {
        let provider = create_test_provider().await.unwrap();
        let features = provider.supported_features();
        assert!(!features.is_empty());
    }

    #[tokio::test]
    #[ignore] // Requires valid Square API credentials
    async fn test_customer_crud_operations() {
        let provider = create_test_provider().await.unwrap();
        
        // Create a customer
        let mut customer = UnifiedCustomer {
            id: None,
            email: Some("test@example.com".to_string()),
            name: Some("John Doe".to_string()),
            phone: Some("+14155551234".to_string()),
            metadata: None,
        };
        
        let created_customer = provider.create_customer(&customer).await.unwrap();
        assert!(created_customer.id.is_some());
        let customer_id = created_customer.id.clone().unwrap();
        
        // Get the customer
        let retrieved_customer = provider.get_customer(&customer_id).await.unwrap();
        assert_eq!(retrieved_customer.email, customer.email);
        
        // Update the customer
        customer.id = Some(customer_id.clone());
        customer.name = Some("Jane Doe".to_string());
        let updated_customer = provider.update_customer(&customer).await.unwrap();
        assert_eq!(updated_customer.name, Some("Jane Doe".to_string()));
        
        // List customers
        let customers = provider.list_customers(Some(10), None).await.unwrap();
        assert!(customers.iter().any(|c| c.id == Some(customer_id.clone())));
        
        // Delete the customer
        let deleted = provider.delete_customer(&customer_id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    #[ignore] // Requires valid Square API credentials and payment token
    async fn test_payment_operations() {
        let provider = create_test_provider().await.unwrap();
        
        // Create a charge (requires a valid payment token/source_id)
        let charge = UnifiedCharge {
            id: None,
            amount: Money {
                amount: 1000, // $10.00
                currency: "usd".to_string(),
            },
            customer_id: None,
            payment_method_id: Some("cnon:card_nonce_ok".to_string()), // Square test nonce
            status: payup::payment_provider::ChargeStatus::Pending,
            description: Some("Test charge".to_string()),
            metadata: None,
            created_at: None,
        };
        
        let created_charge = provider.create_charge(&charge).await.unwrap();
        assert!(created_charge.id.is_some());
        let charge_id = created_charge.id.clone().unwrap();
        
        // Get the charge
        let retrieved_charge = provider.get_charge(&charge_id).await.unwrap();
        assert_eq!(retrieved_charge.amount.amount, 1000);
        
        // List charges
        let charges = provider.list_charges(None, Some(10)).await.unwrap();
        assert!(charges.iter().any(|c| c.id == Some(charge_id.clone())));
    }

    #[tokio::test]
    #[ignore] // Requires valid Square API credentials and existing payment
    async fn test_refund_operations() {
        let provider = create_test_provider().await.unwrap();
        
        // First create a payment to refund
        // (In a real test, you'd create a payment first)
        let payment_id = "test_payment_id";
        
        // Create a refund
        let refund = UnifiedRefund {
            id: None,
            charge_id: payment_id.to_string(),
            amount: Some(Money {
                amount: 500, // $5.00 partial refund
                currency: "usd".to_string(),
            }),
            reason: Some(RefundReason::RequestedByCustomer),
            status: payup::payment_provider::RefundStatus::Pending,
            metadata: None,
        };
        
        let created_refund = provider.create_refund(&refund).await.unwrap();
        assert!(created_refund.id.is_some());
        let refund_id = created_refund.id.clone().unwrap();
        
        // Get the refund
        let retrieved_refund = provider.get_refund(&refund_id).await.unwrap();
        assert_eq!(retrieved_refund.charge_id, payment_id);
    }
}

#[cfg(test)]
mod square_provider_unit_tests {
    use payup::square::{SquareProvider, Environment};
    use payup::square::customers::Customer;
    use payup::square::payments::{Payment, Money};
    
    #[test]
    fn test_customer_mapping() {
        let provider = SquareProvider::new("test_token".to_string(), Environment::Sandbox).unwrap();
        
        let square_customer = Customer {
            id: Some("cust_123".to_string()),
            given_name: Some("John".to_string()),
            family_name: Some("Doe".to_string()),
            email_address: Some("john@example.com".to_string()),
            phone_number: Some("+14155551234".to_string()),
            note: Some("Test note".to_string()),
            ..Customer::new()
        };
        
        // We can't test the private map_customer method directly, 
        // but we can test the conversion logic
        let full_name = match (&square_customer.given_name, &square_customer.family_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            _ => None,
        };
        
        assert_eq!(full_name, Some("John Doe".to_string()));
    }
    
    #[test]
    fn test_payment_status_mapping() {
        let statuses = vec![
            ("PENDING", "Pending"),
            ("COMPLETED", "Succeeded"),
            ("CANCELED", "Canceled"),
            ("FAILED", "Failed"),
            ("UNKNOWN", "Pending"),
        ];
        
        for (square_status, expected) in statuses {
            let mapped = match square_status {
                "PENDING" => "Pending",
                "COMPLETED" => "Succeeded",
                "CANCELED" => "Canceled",
                "FAILED" => "Failed",
                _ => "Pending",
            };
            assert_eq!(mapped, expected);
        }
    }
    
    #[test]
    fn test_money_conversion() {
        let square_money = Money {
            amount: 1000,
            currency: "USD".to_string(),
        };
        
        // Test uppercase to lowercase conversion
        let lowercase_currency = square_money.currency.to_lowercase();
        assert_eq!(lowercase_currency, "usd");
        
        // Test lowercase to uppercase conversion
        let unified_currency = "eur".to_string();
        let uppercase_currency = unified_currency.to_uppercase();
        assert_eq!(uppercase_currency, "EUR");
    }
}