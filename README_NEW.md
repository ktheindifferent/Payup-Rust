# Payup - Multi-Platform Payment Processing for Rust

[![Crates.io](https://img.shields.io/crates/v/payup.svg)](https://crates.io/crates/payup)
[![Documentation](https://docs.rs/payup/badge.svg)](https://docs.rs/payup)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](LICENSE)

A comprehensive synchronous and asynchronous payment processing library for Rust, supporting multiple payment platforms including Stripe, PayPal, and Square.

## Features

- 🚀 **Multi-Platform Support**: Stripe, PayPal, Square
- ⚡ **Sync & Async**: Both synchronous and asynchronous operations
- 🔒 **Type-Safe**: Leverages Rust's type system for safety
- 🛠️ **Comprehensive APIs**: Payments, Subscriptions, Refunds, Customers, and more
- 📦 **Modular Design**: Use only the platforms you need
- 🧪 **Well Tested**: Extensive test coverage
- 📝 **Great Documentation**: Detailed docs and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
payup = "0.3.0"
```

### Feature Flags

```toml
# Use specific platforms
payup = { version = "0.3.0", features = ["stripe"] }
payup = { version = "0.3.0", features = ["paypal"] }
payup = { version = "0.3.0", features = ["square"] }

# Or use all platforms
payup = { version = "0.3.0", features = ["all"] }
```

## Quick Start

### Stripe Example

```rust
use payup::stripe::{Auth, Customer, PaymentMethod, Card, Subscription};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Stripe
    let auth = Auth::new("sk_test_your_key".to_string(), "".to_string());
    
    // Create a customer
    let mut customer = Customer::new();
    customer.name = Some("John Doe".to_string());
    customer.email = Some("john@example.com".to_string());
    let customer = customer.post(auth.clone())?;
    
    // Create a payment method
    let mut card = Card::new();
    card.number = Some("4242424242424242".to_string());
    card.exp_month = Some("12".to_string());
    card.exp_year = Some("2025".to_string());
    card.cvc = Some("123".to_string());
    
    let mut payment_method = PaymentMethod::new();
    payment_method.method_type = Some("card".to_string());
    payment_method.card = Some(card);
    let payment_method = payment_method.post(auth.clone())?;
    
    // Attach payment method to customer
    payment_method.attach(customer.clone(), auth.clone())?;
    
    Ok(())
}
```

### PayPal Example

```rust
use payup::paypal::{PayPalConfig, PayPalClient, PayPalEnvironment};
use payup::paypal::orders::{create_simple_order, Order};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize PayPal
    let config = PayPalConfig {
        client_id: "your_client_id".to_string(),
        client_secret: "your_client_secret".to_string(),
        environment: PayPalEnvironment::Sandbox,
        webhook_id: None,
    };
    
    let mut client = PayPalClient::new(config)?;
    
    // Create an order
    let order = create_simple_order(
        99.99,
        "USD",
        Some("Premium subscription".to_string()),
    );
    
    let created_order = order.create(&mut client)?;
    println!("Order created: {:?}", created_order.id);
    
    // Capture payment
    let capture = Order::capture(&mut client, &created_order.id.unwrap(), None)?;
    println!("Payment captured: {:?}", capture);
    
    Ok(())
}
```

### Square Example

```rust
use payup::square::{SquareConfig, SquareClient, Environment};
use payup::square::payments::{create_simple_payment, Payment};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Square
    let config = SquareConfig {
        access_token: "your_access_token".to_string(),
        environment: Environment::Sandbox,
        location_id: Some("location_id".to_string()),
    };
    
    let client = SquareClient::new(config)?;
    
    // Create a payment
    let payment_request = create_simple_payment(
        "cnon:card-nonce",
        1000, // $10.00 in cents
        "USD",
        "unique_idempotency_key",
    );
    
    let payment = Payment::create(&client, &payment_request)?;
    println!("Payment created: {:?}", payment.id);
    
    Ok(())
}
```

## Async Support

All major operations support async/await:

```rust
use payup::stripe::{Auth, Customer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = Auth::new("sk_test_your_key".to_string(), "".to_string());
    
    // Async customer creation
    let mut customer = Customer::new();
    customer.email = Some("async@example.com".to_string());
    let customer = customer.async_post(auth.clone()).await?;
    
    Ok(())
}
```

## Supported APIs

### Stripe
- ✅ Customers
- ✅ Payment Methods
- ✅ Charges
- ✅ Subscriptions
- ✅ Plans & Prices
- ✅ Invoices
- ✅ Balance & Transactions
- ✅ Disputes
- ✅ Files
- ✅ Events
- ✅ Products (Extended)
- ✅ Refunds (Extended)
- ✅ Payouts (Extended)

### PayPal
- ✅ OAuth Authentication
- ✅ Orders API
- ✅ Payments API
- ✅ Subscriptions API
- ✅ Webhooks
- ✅ Refunds

### Square
- ✅ Payments
- ✅ Customers
- ✅ Catalog (Products)
- ✅ Refunds
- ✅ Locations

## Error Handling

The library provides comprehensive error handling with custom error types:

```rust
use payup::error::{PayupError, Result};

fn process_payment() -> Result<()> {
    match create_payment() {
        Ok(payment) => println!("Payment successful: {:?}", payment),
        Err(PayupError::ApiError { code, message, provider }) => {
            eprintln!("{} API error {}: {}", provider, code, message);
        },
        Err(PayupError::NetworkError(e)) => {
            eprintln!("Network error: {}", e);
        },
        Err(e) => eprintln!("Other error: {}", e),
    }
    Ok(())
}
```

## Webhook Handling

### PayPal Webhooks

```rust
use payup::paypal::webhooks::{WebhookEvent, WebhookHandler};

let mut handler = WebhookHandler::new();

handler.on("PAYMENT.CAPTURE.COMPLETED", |event| {
    println!("Payment completed: {:?}", event.resource);
    Ok(())
});

handler.on("BILLING.SUBSCRIPTION.CREATED", |event| {
    println!("Subscription created: {:?}", event.resource);
    Ok(())
});

// Process incoming webhook
let event = WebhookEvent::parse(&webhook_body)?;
handler.handle(&event)?;
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests for specific platform
cargo test --features stripe
cargo test --features paypal
cargo test --features square

# Run with all features
cargo test --all-features
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Roadmap

- [x] v0.1.x - Basic Stripe support
- [x] v0.3.0 - PayPal and Square integration
- [ ] v0.4.0 - Braintree support
- [ ] v0.5.0 - Cryptocurrency payments
- [ ] v1.0.0 - Stable release with full documentation

## License

This project is dual-licensed under either:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Support

For issues, questions, or contributions, please visit our [GitHub repository](https://github.com/PixelCoda/Payup-Rust).

## Author

Created by Caleb Mitchell Smith-Woolrich (PixelCoda)

---

*Built with ❤️ in Rust*