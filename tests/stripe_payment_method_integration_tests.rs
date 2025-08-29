use payup::stripe::{Auth, PaymentMethod, StripePaymentMethodType, CreatePaymentMethodParams, CreateCardParams, PaymentMethodBillingDetails, PaymentMethodAddress};
use payup::payment_provider::{PaymentProvider, PaymentMethod as UnifiedPaymentMethod, PaymentMethodType, CardDetails};
use payup::stripe::StripeProvider;

/// Helper function to get test API key from environment
fn get_test_api_key() -> Option<String> {
    std::env::var("STRIPE_TEST_API_KEY").ok()
}

/// Helper function to create test Auth
fn create_test_auth() -> Auth {
    let api_key = get_test_api_key().unwrap_or_else(|| "sk_test_fake".to_string());
    Auth::new(api_key.clone(), api_key)
}

/// Helper function to create test card params
fn create_test_card_params() -> CreateCardParams {
    CreateCardParams {
        number: "4242424242424242".to_string(),
        exp_month: 12,
        exp_year: 2030,
        cvc: Some("123".to_string()),
    }
}

/// Helper function to create test billing details
fn create_test_billing_details() -> PaymentMethodBillingDetails {
    PaymentMethodBillingDetails {
        address: Some(PaymentMethodAddress {
            city: Some("San Francisco".to_string()),
            country: Some("US".to_string()),
            line1: Some("123 Test Street".to_string()),
            line2: None,
            postal_code: Some("94105".to_string()),
            state: Some("CA".to_string()),
        }),
        email: Some("test@example.com".to_string()),
        name: Some("Test User".to_string()),
        phone: Some("+14155551234".to_string()),
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_create_payment_method_async() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let auth = create_test_auth();
    let params = CreatePaymentMethodParams {
        payment_method_type: StripePaymentMethodType::Card,
        billing_details: Some(create_test_billing_details()),
        card: Some(create_test_card_params()),
        metadata: None,
    };

    let result = PaymentMethod::create_async(&auth, params).await;
    match result {
        Ok(payment_method) => {
            assert!(payment_method.id.is_some());
            assert_eq!(payment_method.object, Some("payment_method".to_string()));
            assert!(payment_method.card.is_some());
            
            let card = payment_method.card.expect("Payment method should have card details");
            assert_eq!(card.last4, Some("4242".to_string()));
            assert_eq!(card.brand, Some("visa".to_string()));
        }
        Err(e) => {
            eprintln!("Error creating payment method: {:?}", e);
            assert!(false, "Failed to create payment method: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_retrieve_payment_method_async() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let auth = create_test_auth();
    
    // First create a payment method
    let params = CreatePaymentMethodParams {
        payment_method_type: StripePaymentMethodType::Card,
        billing_details: Some(create_test_billing_details()),
        card: Some(create_test_card_params()),
        metadata: None,
    };
    
    let created = PaymentMethod::create_async(&auth, params).await
        .expect("Failed to create payment method for test");
    let payment_method_id = created.id.expect("Created payment method should have an ID");
    
    // Now retrieve it
    let retrieved = PaymentMethod::retrieve_async(&auth, &payment_method_id).await;
    match retrieved {
        Ok(payment_method) => {
            assert_eq!(payment_method.id, Some(payment_method_id));
            assert_eq!(payment_method.object, Some("payment_method".to_string()));
            assert!(payment_method.card.is_some());
        }
        Err(e) => {
            eprintln!("Error retrieving payment method: {:?}", e);
            assert!(false, "Failed to retrieve payment method: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_attach_and_detach_payment_method_async() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let auth = create_test_auth();
    
    // First create a payment method
    let params = CreatePaymentMethodParams {
        payment_method_type: StripePaymentMethodType::Card,
        billing_details: Some(create_test_billing_details()),
        card: Some(create_test_card_params()),
        metadata: None,
    };
    
    let created = PaymentMethod::create_async(&auth, params).await
        .expect("Failed to create payment method for test");
    let payment_method_id = created.id.expect("Created payment method should have an ID");
    
    // Create a customer to attach the payment method to
    use payup::stripe::Customer;
    let customer = Customer::create(&auth).expect("Failed to create customer for test");
    let customer_id = customer.id.expect("Created customer should have an ID");
    
    // Attach the payment method to the customer
    let attached = PaymentMethod::attach_async(&auth, &payment_method_id, &customer_id).await;
    match attached {
        Ok(payment_method) => {
            assert_eq!(payment_method.customer, Some(customer_id.clone()));
        }
        Err(e) => {
            eprintln!("Error attaching payment method: {:?}", e);
            assert!(false, "Failed to attach payment method: {:?}", e);
        }
    }
    
    // Detach the payment method
    let detached = PaymentMethod::detach_async(&auth, &payment_method_id).await;
    match detached {
        Ok(payment_method) => {
            assert_eq!(payment_method.customer, None);
        }
        Err(e) => {
            eprintln!("Error detaching payment method: {:?}", e);
            assert!(false, "Failed to detach payment method: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_provider_create_payment_method() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let api_key = get_test_api_key().expect("STRIPE_TEST_API_KEY should be available after check");
    let provider = StripeProvider::new(api_key);
    
    let payment_method = UnifiedPaymentMethod {
        id: None,
        method_type: PaymentMethodType::Card,
        card: Some(CardDetails {
            number: Some("4242424242424242".to_string()),
            exp_month: "12".to_string(),
            exp_year: "2030".to_string(),
            cvv: Some("123".to_string()),
            brand: None,
            last4: None,
        }),
        bank_account: None,
    };
    
    let result = provider.create_payment_method(&payment_method).await;
    match result {
        Ok(created) => {
            assert!(created.id.is_some());
            assert_eq!(created.method_type, PaymentMethodType::Card);
            assert!(created.card.is_some());
            
            let card = created.card.expect("Created payment method should have card details");
            assert_eq!(card.last4, Some("4242".to_string()));
            assert_eq!(card.brand, Some("visa".to_string()));
        }
        Err(e) => {
            eprintln!("Error creating payment method via provider: {:?}", e);
            assert!(false, "Failed to create payment method via provider: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_provider_get_payment_method() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let api_key = get_test_api_key().expect("STRIPE_TEST_API_KEY should be available after check");
    let provider = StripeProvider::new(api_key);
    
    // First create a payment method
    let payment_method = UnifiedPaymentMethod {
        id: None,
        method_type: PaymentMethodType::Card,
        card: Some(CardDetails {
            number: Some("4242424242424242".to_string()),
            exp_month: "12".to_string(),
            exp_year: "2030".to_string(),
            cvv: Some("123".to_string()),
            brand: None,
            last4: None,
        }),
        bank_account: None,
    };
    
    let created = provider.create_payment_method(&payment_method).await
        .expect("Failed to create payment method for test");
    let payment_method_id = created.id.expect("Created payment method should have an ID");
    
    // Now retrieve it
    let retrieved = provider.get_payment_method(&payment_method_id).await;
    match retrieved {
        Ok(pm) => {
            assert_eq!(pm.id, Some(payment_method_id));
            assert_eq!(pm.method_type, PaymentMethodType::Card);
            assert!(pm.card.is_some());
        }
        Err(e) => {
            eprintln!("Error retrieving payment method via provider: {:?}", e);
            assert!(false, "Failed to retrieve payment method via provider: {:?}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignore by default since it requires real API key
async fn test_provider_attach_detach_payment_method() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let api_key = get_test_api_key().expect("STRIPE_TEST_API_KEY should be available after check");
    let provider = StripeProvider::new(api_key);
    
    // Create a payment method
    let payment_method = UnifiedPaymentMethod {
        id: None,
        method_type: PaymentMethodType::Card,
        card: Some(CardDetails {
            number: Some("4242424242424242".to_string()),
            exp_month: "12".to_string(),
            exp_year: "2030".to_string(),
            cvv: Some("123".to_string()),
            brand: None,
            last4: None,
        }),
        bank_account: None,
    };
    
    let created = provider.create_payment_method(&payment_method).await
        .expect("Failed to create payment method for test");
    let payment_method_id = created.id.expect("Created payment method should have an ID");
    
    // Create a customer
    use payup::payment_provider::Customer;
    let customer = Customer {
        id: None,
        email: Some("test@example.com".to_string()),
        name: Some("Test Customer".to_string()),
        phone: None,
        metadata: None,
    };
    
    let created_customer = provider.create_customer(&customer).await
        .expect("Failed to create customer for test");
    let customer_id = created_customer.id.expect("Created customer should have an ID");
    
    // Attach payment method to customer
    let attached = provider.attach_payment_method(&payment_method_id, &customer_id).await;
    assert!(attached.is_ok());
    
    // Detach payment method
    let detached = provider.detach_payment_method(&payment_method_id).await;
    assert!(detached.is_ok());
}

#[test]
fn test_sync_payment_method_create() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let auth = create_test_auth();
    let params = CreatePaymentMethodParams {
        payment_method_type: StripePaymentMethodType::Card,
        billing_details: Some(create_test_billing_details()),
        card: Some(create_test_card_params()),
        metadata: None,
    };

    let result = PaymentMethod::create(&auth, params);
    match result {
        Ok(payment_method) => {
            assert!(payment_method.id.is_some());
            assert_eq!(payment_method.object, Some("payment_method".to_string()));
        }
        Err(e) => {
            eprintln!("Error creating payment method (sync): {:?}", e);
            if !get_test_api_key().expect("API key should exist").starts_with("sk_test_") {
                assert!(false, "Failed to create payment method - ensure you're using a test API key");
            }
        }
    }
}

#[test]
fn test_sync_payment_method_retrieve() {
    if get_test_api_key().is_none() {
        eprintln!("Skipping test: STRIPE_TEST_API_KEY not set");
        return;
    }

    let auth = create_test_auth();
    
    // First create a payment method
    let params = CreatePaymentMethodParams {
        payment_method_type: StripePaymentMethodType::Card,
        billing_details: Some(create_test_billing_details()),
        card: Some(create_test_card_params()),
        metadata: None,
    };
    
    let created = PaymentMethod::create(&auth, params)
        .expect("Failed to create payment method for test");
    let payment_method_id = created.id.expect("Created payment method should have an ID");
    
    // Now retrieve it
    let retrieved = PaymentMethod::retrieve(&auth, &payment_method_id);
    match retrieved {
        Ok(payment_method) => {
            assert_eq!(payment_method.id, Some(payment_method_id));
            assert_eq!(payment_method.object, Some("payment_method".to_string()));
        }
        Err(e) => {
            eprintln!("Error retrieving payment method (sync): {:?}", e);
            assert!(false, "Failed to retrieve payment method: {:?}", e);
        }
    }
}