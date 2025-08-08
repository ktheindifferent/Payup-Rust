use payup::stripe::{Auth, Customer, PaymentMethod, Card, Subscription, Plan, Price};

#[test]
fn test_auth_creation() {
    let auth = Auth::new("sk_test_example".to_string(), "secret_example".to_string());
    assert_eq!(auth.client, "sk_test_example");
    assert_eq!(auth.secret, "secret_example");
}

#[test]
fn test_customer_creation() {
    let mut customer = Customer::new();
    customer.name = Some("Test Customer".to_string());
    customer.email = Some("test@example.com".to_string());
    customer.phone = Some("555-555-5555".to_string());
    customer.description = Some("Test description".to_string());
    
    assert_eq!(customer.name, Some("Test Customer".to_string()));
    assert_eq!(customer.email, Some("test@example.com".to_string()));
}

#[test]
fn test_card_creation() {
    let mut card = Card::new();
    card.number = Some("4242424242424242".to_string());
    card.exp_month = Some("12".to_string());
    card.exp_year = Some("2025".to_string());
    card.cvc = Some("123".to_string());
    
    assert_eq!(card.number, Some("4242424242424242".to_string()));
    assert_eq!(card.exp_month, Some("12".to_string()));
}

#[test]
fn test_payment_method_creation() {
    let mut payment_method = PaymentMethod::new();
    payment_method.method_type = Some("card".to_string());
    
    let mut card = Card::new();
    card.number = Some("4242424242424242".to_string());
    payment_method.card = Some(card);
    
    assert_eq!(payment_method.method_type, Some("card".to_string()));
    assert!(payment_method.card.is_some());
}

#[test]
fn test_subscription_creation() {
    let mut subscription = Subscription::new();
    subscription.customer = Some("cus_test123".to_string());
    subscription.default_payment_method = Some("pm_test123".to_string());
    
    let mut price_items = Vec::new();
    price_items.push("price_test123".to_string());
    subscription.price_items = Some(price_items);
    
    assert_eq!(subscription.customer, Some("cus_test123".to_string()));
    assert!(subscription.price_items.is_some());
}

#[test]
fn test_plan_creation() {
    let mut plan = Plan::new();
    plan.amount = Some("1000".to_string());
    plan.currency = Some("usd".to_string());
    plan.interval = Some("month".to_string());
    plan.product = Some("prod_test123".to_string());
    
    assert_eq!(plan.amount, Some("1000".to_string()));
    assert_eq!(plan.currency, Some("usd".to_string()));
    assert_eq!(plan.interval, Some("month".to_string()));
}

#[test]
fn test_price_creation() {
    let mut price = Price::new();
    price.unit_amount = Some("2000".to_string());
    price.currency = Some("usd".to_string());
    price.product = Some("prod_test123".to_string());
    
    assert_eq!(price.unit_amount, Some("2000".to_string()));
    assert_eq!(price.currency, Some("usd".to_string()));
}

#[cfg(test)]
mod mock_api_tests {
    
    #[test]
    #[ignore] // Ignored by default as it requires valid API keys
    fn test_balance_get() {
        // This test would require valid API credentials
        // let auth = Auth::new("sk_test_xxx".to_string(), "".to_string());
        // let balance = Balance::get(auth);
        // assert!(balance.is_ok());
    }
    
    #[test]
    #[ignore] // Ignored by default as it requires valid API keys
    fn test_customer_list() {
        // This test would require valid API credentials
        // let auth = Auth::new("sk_test_xxx".to_string(), "".to_string());
        // let customers = Customer::list(auth);
        // assert!(customers.is_ok());
    }
}