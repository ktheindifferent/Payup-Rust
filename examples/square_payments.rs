use payup::square::{SquareProvider, Environment};
use payup::payment_provider::{
    PaymentProvider, Customer, Charge, Refund, RefundReason, Money, ChargeStatus
};
use payup::error::Result;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Square provider
    let access_token = std::env::var("SQUARE_ACCESS_TOKEN")
        .expect("Please set SQUARE_ACCESS_TOKEN environment variable");
    
    let provider = SquareProvider::new(access_token, Environment::Sandbox)?;
    
    println!("Square Provider Example");
    println!("=======================");
    println!("Provider: {}", provider.name());
    println!("Supported currencies: {:?}", provider.supported_currencies());
    println!("Supported features: {:?}", provider.supported_features());
    println!();
    
    // Customer Management Example
    println!("Customer Management");
    println!("-------------------");
    
    // Create a customer
    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), "example_app".to_string());
    
    let customer = Customer {
        id: None,
        email: Some("customer@example.com".to_string()),
        name: Some("John Smith".to_string()),
        phone: Some("+14155551234".to_string()),
        metadata: Some(metadata),
    };
    
    match provider.create_customer(&customer).await {
        Ok(created_customer) => {
            println!("✓ Customer created: {:?}", created_customer.id);
            
            if let Some(customer_id) = created_customer.id {
                // Get customer details
                match provider.get_customer(&customer_id).await {
                    Ok(retrieved) => {
                        println!("✓ Customer retrieved: {} ({})", 
                            retrieved.name.unwrap_or_default(),
                            retrieved.email.unwrap_or_default()
                        );
                    }
                    Err(e) => println!("✗ Failed to retrieve customer: {}", e),
                }
                
                // Update customer
                let mut updated_customer = customer.clone();
                updated_customer.id = Some(customer_id.clone());
                updated_customer.name = Some("John M. Smith".to_string());
                
                match provider.update_customer(&updated_customer).await {
                    Ok(updated) => {
                        println!("✓ Customer updated: {}", updated.name.unwrap_or_default());
                    }
                    Err(e) => println!("✗ Failed to update customer: {}", e),
                }
                
                // Clean up - delete customer
                match provider.delete_customer(&customer_id).await {
                    Ok(deleted) => {
                        if deleted {
                            println!("✓ Customer deleted");
                        }
                    }
                    Err(e) => println!("✗ Failed to delete customer: {}", e),
                }
            }
        }
        Err(e) => println!("✗ Failed to create customer: {}", e),
    }
    
    println!();
    
    // Payment Processing Example
    println!("Payment Processing");
    println!("------------------");
    
    // Note: For Square, you need a valid payment token (source_id)
    // In production, this would come from Square's payment form or SDK
    // For testing, you can use Square's sandbox test tokens:
    // - "cnon:card_nonce_ok" for successful payments
    // - "cnon:card_nonce_declined" for declined payments
    
    let charge = Charge {
        id: None,
        amount: Money {
            amount: 2500, // $25.00
            currency: "usd".to_string(),
        },
        customer_id: None, // Could link to a customer
        payment_method_id: Some("cnon:card_nonce_ok".to_string()), // Square test token
        status: ChargeStatus::Pending,
        description: Some("Example payment from Rust SDK".to_string()),
        metadata: None,
        created_at: None,
    };
    
    match provider.create_charge(&charge).await {
        Ok(created_charge) => {
            println!("✓ Payment created: {:?}", created_charge.id);
            println!("  Amount: ${:.2}", created_charge.amount.amount as f64 / 100.0);
            println!("  Status: {:?}", created_charge.status);
            
            if let Some(charge_id) = created_charge.id {
                // Get payment details
                match provider.get_charge(&charge_id).await {
                    Ok(retrieved) => {
                        println!("✓ Payment retrieved: ${:.2} ({})", 
                            retrieved.amount.amount as f64 / 100.0,
                            retrieved.description.unwrap_or_default()
                        );
                    }
                    Err(e) => println!("✗ Failed to retrieve payment: {}", e),
                }
                
                // Create a partial refund
                println!();
                println!("Refund Processing");
                println!("-----------------");
                
                let refund = Refund {
                    id: None,
                    charge_id: charge_id.clone(),
                    amount: Some(Money {
                        amount: 1000, // $10.00 partial refund
                        currency: "usd".to_string(),
                    }),
                    reason: Some(RefundReason::RequestedByCustomer),
                    status: payup::payment_provider::RefundStatus::Pending,
                    metadata: None,
                };
                
                match provider.create_refund(&refund).await {
                    Ok(created_refund) => {
                        println!("✓ Refund created: {:?}", created_refund.id);
                        println!("  Amount: ${:.2}", 
                            created_refund.amount
                                .map(|a| a.amount as f64 / 100.0)
                                .unwrap_or(0.0)
                        );
                        
                        if let Some(refund_id) = created_refund.id {
                            // Get refund details
                            match provider.get_refund(&refund_id).await {
                                Ok(retrieved) => {
                                    println!("✓ Refund retrieved: ${:.2}",
                                        retrieved.amount
                                            .map(|a| a.amount as f64 / 100.0)
                                            .unwrap_or(0.0)
                                    );
                                }
                                Err(e) => println!("✗ Failed to retrieve refund: {}", e),
                            }
                        }
                    }
                    Err(e) => println!("✗ Failed to create refund: {}", e),
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to create payment: {}", e);
            println!("  Note: Square requires a valid payment token (source_id)");
            println!("  In production, obtain this from Square's payment form or SDK");
        }
    }
    
    println!();
    
    // List operations example
    println!("List Operations");
    println!("---------------");
    
    // List customers
    match provider.list_customers(Some(5), None).await {
        Ok(customers) => {
            println!("✓ Listed {} customers", customers.len());
            for (i, customer) in customers.iter().take(3).enumerate() {
                println!("  {}. {} ({})", 
                    i + 1,
                    customer.name.as_deref().unwrap_or("Unknown"),
                    customer.email.as_deref().unwrap_or("No email")
                );
            }
        }
        Err(e) => println!("✗ Failed to list customers: {}", e),
    }
    
    // List charges
    match provider.list_charges(None, Some(5)).await {
        Ok(charges) => {
            println!("✓ Listed {} charges", charges.len());
            for (i, charge) in charges.iter().take(3).enumerate() {
                println!("  {}. ${:.2} - {:?}", 
                    i + 1,
                    charge.amount.amount as f64 / 100.0,
                    charge.status
                );
            }
        }
        Err(e) => println!("✗ Failed to list charges: {}", e),
    }
    
    println!();
    println!("Example completed!");
    
    Ok(())
}