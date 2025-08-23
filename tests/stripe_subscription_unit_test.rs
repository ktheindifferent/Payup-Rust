#[cfg(test)]
mod tests {
    use payup::stripe::subscription::Subscription;
    use payup::stripe::Auth;
    
    #[test]
    fn test_subscription_creation() {
        let mut sub = Subscription::new();
        assert!(sub.id.is_none());
        assert!(sub.customer.is_none());
        assert!(sub.price_items.is_none());
        assert_eq!(sub.cancel_at_period_end, None);
        
        // Set some values
        sub.customer = Some("cus_test123".to_string());
        sub.price_items = Some(vec!["price_test123".to_string()]);
        sub.cancel_at_period_end = Some(false);
        
        assert_eq!(sub.customer, Some("cus_test123".to_string()));
        assert_eq!(sub.price_items, Some(vec!["price_test123".to_string()]));
        assert_eq!(sub.cancel_at_period_end, Some(false));
    }
    
    #[test]
    fn test_subscription_params() {
        let mut sub = Subscription::new();
        sub.customer = Some("cus_test123".to_string());
        sub.price_items = Some(vec!["price_test123".to_string()]);
        sub.default_payment_method = Some("pm_test123".to_string());
        sub.collection_method = Some("charge_automatically".to_string());
        sub.cancel_at_period_end = Some(true);
        sub.days_until_due = Some(7);
        
        let params = sub.to_params();
        
        // Check that params include expected values
        assert!(params.iter().any(|(k, v)| *k == "customer" && v == "cus_test123"));
        assert!(params.iter().any(|(k, v)| *k == "default_payment_method" && v == "pm_test123"));
        assert!(params.iter().any(|(k, v)| *k == "collection_method" && v == "charge_automatically"));
        assert!(params.iter().any(|(k, v)| *k == "cancel_at_period_end" && v == "true"));
        assert!(params.iter().any(|(k, v)| *k == "days_until_due" && v == "7"));
        assert!(params.iter().any(|(k, v)| k.starts_with("items[0][price]") && v == "price_test123"));
    }
    
    #[test]
    fn test_subscription_default() {
        let sub = Subscription::default();
        assert!(sub.id.is_none());
        assert!(sub.customer.is_none());
        assert!(sub.status.is_none());
    }
}

#[cfg(test)]
mod provider_tests {
    use payup::stripe::{StripeProvider};
    use payup::payment_provider::{PaymentProvider, Subscription as UnifiedSubscription, SubscriptionStatus};
    
    #[test]
    fn test_provider_instantiation() {
        let provider = StripeProvider::new("sk_test_placeholder".to_string());
        assert_eq!(provider.name(), "stripe");
    }
    
    #[test]
    fn test_subscription_status_mapping() {
        // This test verifies that SubscriptionStatus enum values are properly handled
        let statuses = vec![
            SubscriptionStatus::Active,
            SubscriptionStatus::PastDue,
            SubscriptionStatus::Canceled,
            SubscriptionStatus::Incomplete,
            SubscriptionStatus::IncompleteExpired,
            SubscriptionStatus::Trialing,
            SubscriptionStatus::Unpaid,
        ];
        
        for status in statuses {
            let sub = UnifiedSubscription {
                id: Some("sub_test".to_string()),
                customer_id: "cus_test".to_string(),
                plan_id: None,
                price_id: Some("price_test".to_string()),
                status: status.clone(),
                current_period_start: None,
                current_period_end: None,
                cancel_at_period_end: false,
            };
            
            // Verify the subscription can be created with each status
            assert_eq!(sub.status, status);
        }
    }
}