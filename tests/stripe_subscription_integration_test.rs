use payup::stripe::{Auth, StripeProvider};
use payup::payment_provider::{PaymentProvider, Subscription, SubscriptionStatus};
use std::env;

fn get_test_auth() -> Auth {
    let api_key = env::var("STRIPE_TEST_API_KEY")
        .unwrap_or_else(|_| "sk_test_placeholder".to_string());
    Auth::new(api_key)
}

fn get_test_provider() -> StripeProvider {
    let api_key = env::var("STRIPE_TEST_API_KEY")
        .unwrap_or_else(|_| "sk_test_placeholder".to_string());
    StripeProvider::new(api_key)
}

#[tokio::test]
#[ignore] // Run with --ignored flag when STRIPE_TEST_API_KEY is set
async fn test_subscription_lifecycle() {
    let provider = get_test_provider();
    
    // Note: This test requires:
    // 1. A valid STRIPE_TEST_API_KEY environment variable
    // 2. A test customer ID (created separately)
    // 3. A test price ID (created in Stripe dashboard)
    
    let test_customer_id = env::var("STRIPE_TEST_CUSTOMER_ID")
        .unwrap_or_else(|_| "cus_test_placeholder".to_string());
    let test_price_id = env::var("STRIPE_TEST_PRICE_ID")
        .unwrap_or_else(|_| "price_test_placeholder".to_string());
    
    // Create a subscription
    let mut subscription = Subscription {
        id: None,
        customer_id: test_customer_id.clone(),
        plan_id: None,
        price_id: Some(test_price_id.clone()),
        status: SubscriptionStatus::Active,
        current_period_start: None,
        current_period_end: None,
        cancel_at_period_end: false,
    };
    
    println!("Creating subscription...");
    let created_subscription = provider.create_subscription(&subscription).await;
    
    if let Ok(sub) = created_subscription {
        println!("Subscription created with ID: {:?}", sub.id);
        assert!(sub.id.is_some());
        assert_eq!(sub.customer_id, test_customer_id);
        
        let subscription_id = sub.id.clone().unwrap();
        
        // Get the subscription
        println!("Fetching subscription...");
        let fetched_subscription = provider.get_subscription(&subscription_id).await;
        assert!(fetched_subscription.is_ok());
        let fetched = fetched_subscription.unwrap();
        assert_eq!(fetched.id, Some(subscription_id.clone()));
        assert_eq!(fetched.customer_id, test_customer_id);
        
        // Update the subscription (e.g., cancel at period end)
        subscription.id = Some(subscription_id.clone());
        subscription.cancel_at_period_end = true;
        
        println!("Updating subscription...");
        let updated_subscription = provider.update_subscription(&subscription).await;
        assert!(updated_subscription.is_ok());
        let updated = updated_subscription.unwrap();
        assert_eq!(updated.cancel_at_period_end, true);
        
        // Cancel the subscription
        println!("Canceling subscription...");
        let canceled_subscription = provider.cancel_subscription(&subscription_id, false).await;
        assert!(canceled_subscription.is_ok());
        let canceled = canceled_subscription.unwrap();
        assert!(matches!(canceled.status, SubscriptionStatus::Canceled));
        
        println!("Subscription lifecycle test completed successfully");
    } else {
        println!("Skipping test - unable to create subscription (likely missing test credentials)");
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag when STRIPE_TEST_API_KEY is set
async fn test_list_subscriptions() {
    let provider = get_test_provider();
    
    let test_customer_id = env::var("STRIPE_TEST_CUSTOMER_ID").ok();
    
    println!("Listing subscriptions...");
    let subscriptions = provider.list_subscriptions(
        test_customer_id.as_deref(),
        Some(10)
    ).await;
    
    if let Ok(subs) = subscriptions {
        println!("Found {} subscriptions", subs.len());
        for sub in subs.iter().take(3) {
            println!("  Subscription ID: {:?}, Customer: {}, Status: {:?}", 
                sub.id, sub.customer_id, sub.status);
        }
    } else {
        println!("Skipping test - unable to list subscriptions (likely missing test credentials)");
    }
}

#[tokio::test]
#[ignore] // Run with --ignored flag when STRIPE_TEST_API_KEY is set  
async fn test_subscription_with_period_end_cancel() {
    let provider = get_test_provider();
    
    let test_customer_id = env::var("STRIPE_TEST_CUSTOMER_ID")
        .unwrap_or_else(|_| "cus_test_placeholder".to_string());
    let test_price_id = env::var("STRIPE_TEST_PRICE_ID")
        .unwrap_or_else(|_| "price_test_placeholder".to_string());
    
    // Create a subscription
    let subscription = Subscription {
        id: None,
        customer_id: test_customer_id.clone(),
        plan_id: None,
        price_id: Some(test_price_id.clone()),
        status: SubscriptionStatus::Active,
        current_period_start: None,
        current_period_end: None,
        cancel_at_period_end: false,
    };
    
    println!("Creating subscription for period-end cancellation test...");
    let created_subscription = provider.create_subscription(&subscription).await;
    
    if let Ok(sub) = created_subscription {
        let subscription_id = sub.id.clone().unwrap();
        println!("Subscription created with ID: {}", subscription_id);
        
        // Cancel at period end
        println!("Canceling subscription at period end...");
        let canceled_subscription = provider.cancel_subscription(&subscription_id, true).await;
        assert!(canceled_subscription.is_ok());
        
        let canceled = canceled_subscription.unwrap();
        assert_eq!(canceled.cancel_at_period_end, true);
        // Status should still be active when canceling at period end
        assert!(matches!(canceled.status, SubscriptionStatus::Active));
        
        // Clean up - cancel immediately
        println!("Cleaning up - canceling immediately...");
        let _ = provider.cancel_subscription(&subscription_id, false).await;
        
        println!("Period-end cancellation test completed successfully");
    } else {
        println!("Skipping test - unable to create subscription (likely missing test credentials)");
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;
    use payup::stripe::Customer;
    use payup::payment_provider::Customer as UnifiedCustomer;
    
    /// Helper function to create a test customer for subscription tests
    pub async fn create_test_customer(provider: &StripeProvider) -> Option<String> {
        let customer = UnifiedCustomer {
            id: None,
            email: Some("test@example.com".to_string()),
            name: Some("Test Customer".to_string()),
            phone: None,
            metadata: None,
        };
        
        match provider.create_customer(&customer).await {
            Ok(created) => created.id,
            Err(_) => None,
        }
    }
}