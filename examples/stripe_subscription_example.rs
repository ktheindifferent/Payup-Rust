use payup::stripe::StripeProvider;
use payup::payment_provider::{PaymentProvider, Subscription, SubscriptionStatus, Customer};
use payup::provider_factory::ProviderFactory;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key = env::var("STRIPE_TEST_API_KEY")
        .expect("STRIPE_TEST_API_KEY environment variable must be set");
    
    // Create provider
    let provider = StripeProvider::new(api_key);
    
    println!("Stripe Subscription Management Example");
    println!("======================================");
    
    // First, create a test customer
    println!("\n1. Creating a test customer...");
    let customer = Customer {
        id: None,
        email: Some("subscription_test@example.com".to_string()),
        name: Some("Subscription Test Customer".to_string()),
        phone: None,
        metadata: None,
    };
    
    let created_customer = provider.create_customer(&customer).await?;
    let customer_id = created_customer.id.expect("Customer should have ID");
    println!("   Customer created: {}", customer_id);
    
    // Note: You need to create a product and price in your Stripe dashboard first
    // For testing, you can use the Stripe test mode and create a recurring price
    let price_id = env::var("STRIPE_TEST_PRICE_ID")
        .unwrap_or_else(|_| {
            println!("\n   NOTE: Set STRIPE_TEST_PRICE_ID to test with a real price.");
            println!("   Using placeholder price ID for demonstration.");
            "price_placeholder".to_string()
        });
    
    // Create a subscription
    println!("\n2. Creating a subscription...");
    let subscription = Subscription {
        id: None,
        customer_id: customer_id.clone(),
        plan_id: None,
        price_id: Some(price_id.clone()),
        status: SubscriptionStatus::Active,
        current_period_start: None,
        current_period_end: None,
        cancel_at_period_end: false,
    };
    
    match provider.create_subscription(&subscription).await {
        Ok(created_sub) => {
            let sub_id = created_sub.id.clone().expect("Subscription should have ID");
            println!("   Subscription created: {}", sub_id);
            println!("   Status: {:?}", created_sub.status);
            println!("   Customer: {}", created_sub.customer_id);
            
            // Get subscription details
            println!("\n3. Fetching subscription details...");
            let fetched_sub = provider.get_subscription(&sub_id).await?;
            println!("   Subscription ID: {:?}", fetched_sub.id);
            println!("   Status: {:?}", fetched_sub.status);
            println!("   Cancel at period end: {}", fetched_sub.cancel_at_period_end);
            
            if let (Some(start), Some(end)) = (fetched_sub.current_period_start, fetched_sub.current_period_end) {
                println!("   Current period: {} to {}", 
                    chrono::NaiveDateTime::from_timestamp_opt(start, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| start.to_string()),
                    chrono::NaiveDateTime::from_timestamp_opt(end, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| end.to_string())
                );
            }
            
            // Update subscription to cancel at period end
            println!("\n4. Updating subscription to cancel at period end...");
            let mut updated_sub = fetched_sub.clone();
            updated_sub.cancel_at_period_end = true;
            
            let result = provider.update_subscription(&updated_sub).await?;
            println!("   Subscription updated");
            println!("   Cancel at period end: {}", result.cancel_at_period_end);
            
            // List customer's subscriptions
            println!("\n5. Listing customer's subscriptions...");
            let subscriptions = provider.list_subscriptions(Some(&customer_id), Some(10)).await?;
            println!("   Found {} subscription(s) for customer", subscriptions.len());
            for (i, sub) in subscriptions.iter().enumerate() {
                println!("   {}. ID: {:?}, Status: {:?}", 
                    i + 1, sub.id, sub.status);
            }
            
            // Cancel subscription immediately
            println!("\n6. Canceling subscription immediately...");
            let canceled_sub = provider.cancel_subscription(&sub_id, false).await?;
            println!("   Subscription canceled");
            println!("   Final status: {:?}", canceled_sub.status);
            
        }
        Err(e) => {
            println!("   Error creating subscription: {}", e);
            println!("   Make sure you have:");
            println!("   - A valid STRIPE_TEST_API_KEY");
            println!("   - A valid STRIPE_TEST_PRICE_ID (create a recurring price in Stripe dashboard)");
        }
    }
    
    // Clean up - delete test customer
    println!("\n7. Cleaning up - deleting test customer...");
    match provider.delete_customer(&customer_id).await {
        Ok(_) => println!("   Customer deleted"),
        Err(e) => println!("   Error deleting customer: {}", e),
    }
    
    println!("\n✅ Example completed!");
    Ok(())
}

/// Helper function to demonstrate using the factory pattern
#[allow(dead_code)]
async fn create_subscription_with_factory() -> Result<(), Box<dyn std::error::Error>> {
    // You can also use the provider factory for a more generic approach
    env::set_var("PAYMENT_PROVIDER", "stripe");
    env::set_var("PAYMENT_API_KEY", "sk_test_...");
    
    let provider = ProviderFactory::from_env()?;
    
    let subscription = Subscription {
        id: None,
        customer_id: "cus_123".to_string(),
        plan_id: None,
        price_id: Some("price_123".to_string()),
        status: SubscriptionStatus::Active,
        current_period_start: None,
        current_period_end: None,
        cancel_at_period_end: false,
    };
    
    let created = provider.create_subscription(&subscription).await?;
    println!("Created subscription: {:?}", created.id);
    
    Ok(())
}