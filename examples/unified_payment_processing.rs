use payup::payment_provider::{PaymentProvider, Customer, Charge, ChargeStatus, Money};
use payup::provider_factory::{ProviderFactory, ProviderConfig, ProviderBuilder};
use std::sync::Arc;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Create provider from environment variables
    // Set PAYMENT_PROVIDER=stripe, PAYMENT_API_KEY=sk_test_..., PAYMENT_SANDBOX=true
    let provider = match ProviderFactory::from_env() {
        Ok(p) => p,
        Err(_) => {
            // Fallback to manual configuration with environment variables
            // SECURITY WARNING: Never hardcode API keys in production code!
            // Use environment variables or secure credential management systems.
            let api_key = env::var("STRIPE_API_KEY")
                .unwrap_or_else(|_| {
                    eprintln!("WARNING: STRIPE_API_KEY not set. Using placeholder.");
                    eprintln!("Please set STRIPE_API_KEY environment variable with your actual API key.");
                    "sk_test_PLACEHOLDER_REPLACE_WITH_REAL_KEY".to_string()
                });
            
            ProviderFactory::create(ProviderConfig {
                provider: "stripe".to_string(),
                api_key,
                client_secret: None,
                sandbox: true,
            })?
        }
    };
    
    println!("Using payment provider: {}", provider.name());
    
    // Example 2: Create provider using builder pattern
    // SECURITY WARNING: Always load credentials from environment variables!
    let paypal_client_id = env::var("PAYPAL_CLIENT_ID")
        .unwrap_or_else(|_| {
            eprintln!("WARNING: PAYPAL_CLIENT_ID not set. Using placeholder.");
            "PLACEHOLDER_CLIENT_ID".to_string()
        });
    let paypal_client_secret = env::var("PAYPAL_CLIENT_SECRET")
        .unwrap_or_else(|_| {
            eprintln!("WARNING: PAYPAL_CLIENT_SECRET not set. Using placeholder.");
            "PLACEHOLDER_CLIENT_SECRET".to_string()
        });
    
    let provider_builder = ProviderBuilder::new()
        .provider("paypal")
        .api_key(paypal_client_id)
        .client_secret(paypal_client_secret)
        .sandbox(true)
        .build();
    
    // Example 3: Provider-agnostic payment processing
    match process_payment(provider.clone()).await {
        Ok(charge) => {
            println!("Payment processed successfully!");
            println!("Charge ID: {:?}", charge.id);
            println!("Amount: {} {}", charge.amount.amount, charge.amount.currency);
            println!("Status: {:?}", charge.status);
        }
        Err(e) => {
            println!("Payment processing failed: {}", e);
        }
    }
    
    // Example 4: Check provider capabilities
    let features = provider.supported_features();
    println!("\nProvider features:");
    for feature in features {
        println!("  - {:?}", feature);
    }
    
    let currencies = provider.supported_currencies();
    println!("\nSupported currencies: {:?}", currencies);
    
    Ok(())
}

/// Provider-agnostic payment processing function
/// This function works with any payment provider that implements the PaymentProvider trait
async fn process_payment(provider: Arc<dyn PaymentProvider>) -> Result<Charge, Box<dyn std::error::Error>> {
    // Create a customer
    let customer = Customer {
        id: None,
        email: Some("customer@example.com".to_string()),
        name: Some("John Doe".to_string()),
        phone: Some("+1234567890".to_string()),
        metadata: None,
    };
    
    let created_customer = provider.create_customer(&customer).await?;
    println!("Created customer: {:?}", created_customer.id);
    
    // Create a charge
    let charge = Charge {
        id: None,
        amount: Money {
            amount: 1000, // $10.00 in cents
            currency: "usd".to_string(),
        },
        customer_id: created_customer.id.clone(),
        payment_method_id: None,
        status: ChargeStatus::Pending,
        description: Some("Test charge".to_string()),
        metadata: None,
        created_at: None,
    };
    
    let created_charge = provider.create_charge(&charge).await?;
    
    // Check charge status
    if let Some(charge_id) = &created_charge.id {
        let retrieved_charge = provider.get_charge(charge_id).await?;
        println!("Charge status: {:?}", retrieved_charge.status);
        
        // Process refund if needed
        if matches!(retrieved_charge.status, ChargeStatus::Succeeded) {
            // Example: Create a partial refund
            use payup::payment_provider::{Refund, RefundReason};
            
            let refund = Refund {
                id: None,
                charge_id: charge_id.clone(),
                amount: Some(Money {
                    amount: 500, // Refund $5.00
                    currency: "usd".to_string(),
                }),
                reason: Some(RefundReason::RequestedByCustomer),
                status: payup::payment_provider::RefundStatus::Pending,
                metadata: None,
            };
            
            match provider.create_refund(&refund).await {
                Ok(created_refund) => {
                    println!("Refund created: {:?}", created_refund.id);
                }
                Err(e) => {
                    println!("Refund not supported or failed: {}", e);
                }
            }
        }
    }
    
    Ok(created_charge)
}

/// Example of switching providers dynamically
async fn switch_provider_example() -> Result<(), Box<dyn std::error::Error>> {
    let providers = vec!["stripe", "paypal", "square"];
    
    for provider_name in providers {
        println!("\nSwitching to provider: {}", provider_name);
        
        // SECURITY WARNING: Production code must use secure credential storage!
        // Never commit real API keys to version control.
        let config = match provider_name {
            "stripe" => {
                let api_key = env::var("STRIPE_API_KEY")
                    .unwrap_or_else(|_| {
                        eprintln!("WARNING: STRIPE_API_KEY not set for provider switching example.");
                        "sk_test_PLACEHOLDER".to_string()
                    });
                ProviderConfig {
                    provider: provider_name.to_string(),
                    api_key,
                    client_secret: None,
                    sandbox: true,
                }
            },
            "paypal" => {
                let client_id = env::var("PAYPAL_CLIENT_ID")
                    .unwrap_or_else(|_| {
                        eprintln!("WARNING: PAYPAL_CLIENT_ID not set for provider switching example.");
                        "PLACEHOLDER_CLIENT_ID".to_string()
                    });
                let client_secret = env::var("PAYPAL_CLIENT_SECRET")
                    .unwrap_or_else(|_| {
                        eprintln!("WARNING: PAYPAL_CLIENT_SECRET not set for provider switching example.");
                        "PLACEHOLDER_SECRET".to_string()
                    });
                ProviderConfig {
                    provider: provider_name.to_string(),
                    api_key: client_id,
                    client_secret: Some(client_secret),
                    sandbox: true,
                }
            },
            "square" => {
                let access_token = env::var("SQUARE_ACCESS_TOKEN")
                    .unwrap_or_else(|_| {
                        eprintln!("WARNING: SQUARE_ACCESS_TOKEN not set for provider switching example.");
                        "PLACEHOLDER_ACCESS_TOKEN".to_string()
                    });
                ProviderConfig {
                    provider: provider_name.to_string(),
                    api_key: access_token,
                    client_secret: None,
                    sandbox: true,
                }
            },
            _ => continue,
        };
        
        let provider = ProviderFactory::create(config)?;
        
        // Use the same payment processing logic regardless of provider
        match process_simple_payment(provider).await {
            Ok(_) => println!("Payment successful with {}", provider_name),
            Err(e) => println!("Payment failed with {}: {}", provider_name, e),
        }
    }
    
    Ok(())
}

/// Simple payment processing that works with any provider
async fn process_simple_payment(provider: Arc<dyn PaymentProvider>) -> Result<(), Box<dyn std::error::Error>> {
    let charge = Charge {
        id: None,
        amount: Money {
            amount: 2500, // $25.00
            currency: "usd".to_string(),
        },
        customer_id: None,
        payment_method_id: None,
        status: ChargeStatus::Pending,
        description: Some("Provider-agnostic charge".to_string()),
        metadata: None,
        created_at: None,
    };
    
    let result = provider.create_charge(&charge).await?;
    println!("Charge created with {}: {:?}", provider.name(), result.id);
    
    Ok(())
}