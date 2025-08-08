use payup::error::Result;
use payup::stripe;
use payup::paypal::{PayPalConfig, PayPalClient, PayPalEnvironment};
use payup::paypal::orders::{Order, OrderIntent, create_simple_order};

fn main() -> Result<()> {
    println!("Payup Multi-Platform Payment Example\n");
    
    // Example 1: Stripe Payment
    stripe_example()?;
    
    // Example 2: PayPal Payment
    paypal_example()?;
    
    println!("\nAll examples completed successfully!");
    Ok(())
}

fn stripe_example() -> Result<()> {
    println!("=== STRIPE EXAMPLE ===");
    
    // Initialize Stripe
    let stripe_auth = stripe::Auth::new(
        "sk_test_your_stripe_key".to_string(),
        "".to_string(),
    );
    
    // Create a customer
    let mut customer = stripe::Customer::new();
    customer.name = Some("John Doe".to_string());
    customer.email = Some("john@example.com".to_string());
    
    println!("Created Stripe customer: {:?}", customer.name);
    
    // Note: Actual API calls would require valid credentials
    // customer = customer.post(stripe_auth.clone())?;
    
    // Create a payment method
    let mut card = stripe::Card::new();
    card.number = Some("4242424242424242".to_string());
    card.exp_month = Some("12".to_string());
    card.exp_year = Some("2025".to_string());
    card.cvc = Some("123".to_string());
    
    let mut payment_method = stripe::PaymentMethod::new();
    payment_method.method_type = Some("card".to_string());
    payment_method.card = Some(card);
    
    println!("Created Stripe payment method");
    
    // Create a subscription
    let mut subscription = stripe::Subscription::new();
    subscription.customer = Some("cus_test123".to_string());
    subscription.default_payment_method = Some("pm_test123".to_string());
    
    println!("Created Stripe subscription template\n");
    
    Ok(())
}

fn paypal_example() -> Result<()> {
    println!("=== PAYPAL EXAMPLE ===");
    
    // Initialize PayPal
    let config = PayPalConfig {
        client_id: "your_paypal_client_id".to_string(),
        client_secret: "your_paypal_client_secret".to_string(),
        environment: PayPalEnvironment::Sandbox,
        webhook_id: None,
    };
    
    // Note: This would fail without valid credentials
    // let mut client = PayPalClient::new(config)?;
    
    // Create an order
    let order = create_simple_order(
        99.99,
        "USD",
        Some("Premium subscription".to_string()),
    );
    
    println!("Created PayPal order for $99.99 USD");
    println!("Order intent: {:?}", order.intent);
    println!("Purchase units: {} items", order.purchase_units.len());
    
    // Note: Actual API call would require valid client
    // let created_order = order.create(&mut client)?;
    // println!("Order ID: {}", created_order.id.unwrap());
    
    // Capture payment
    // let capture = Order::capture(&mut client, &order_id, None)?;
    // println!("Payment captured: {:?}", capture.status);
    
    println!("PayPal order ready for processing\n");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stripe_customer_creation() {
        let mut customer = stripe::Customer::new();
        customer.name = Some("Test User".to_string());
        assert_eq!(customer.name, Some("Test User".to_string()));
    }
    
    #[test]
    fn test_paypal_order_creation() {
        let order = create_simple_order(50.0, "EUR", None);
        assert_eq!(order.purchase_units.len(), 1);
        assert_eq!(order.purchase_units[0].amount.currency_code, "EUR");
        assert_eq!(order.purchase_units[0].amount.value, "50.00");
    }
}