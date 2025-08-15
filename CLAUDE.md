# Payup - Multi-Provider Payment Processing Library for Rust

## Overview

Payup is a comprehensive payment processing library for Rust that provides both synchronous and asynchronous APIs for multiple payment providers. The library is designed with a unified interface pattern, allowing developers to switch between payment providers with minimal code changes.

## Project Information

- **Name**: payup
- **Version**: 0.4.0
- **License**: MIT OR Apache-2.0
- **Repository**: https://github.com/PixelCoda/Payup-Rust
- **Documentation**: https://docs.rs/payup

## Core Features

### Supported Payment Providers
1. **Stripe** - Full-featured integration with extensive API coverage
2. **PayPal** - Complete payment processing, orders, and subscriptions
3. **Square** - Payments, catalog, customers, and webhooks
4. **Cryptocurrency** - Bitcoin and Ethereum support via multiple providers

### Key Capabilities
- **Unified Payment Interface**: Single API for all providers via `PaymentProvider` trait
- **Rate Limiting**: Built-in rate limiter with exponential backoff
- **Circuit Breaker**: Fault tolerance for external API failures
- **Webhook Support**: Secure webhook handling for all providers
- **Async/Sync Support**: Both async and blocking APIs available
- **Provider Factory**: Dynamic provider instantiation from config or environment

## Codebase Structure

```
/root/repo/
├── src/
│   ├── lib.rs                       # Main library entry point
│   ├── payment_provider.rs          # Unified PaymentProvider trait
│   ├── provider_factory.rs          # Factory pattern for provider creation
│   ├── rate_limiter.rs             # Rate limiting implementation
│   ├── circuit_breaker.rs          # Circuit breaker for fault tolerance
│   ├── http_client.rs              # Shared HTTP client with rate limiting
│   ├── error.rs                    # Error types and handling
│   ├── config.rs                   # Configuration management
│   ├── stripe/                     # Stripe provider implementation
│   │   ├── mod.rs                  # Module exports
│   │   ├── provider.rs             # PaymentProvider implementation
│   │   ├── auth.rs                 # Authentication
│   │   ├── payment_intent.rs       # Payment Intent API
│   │   ├── customer.rs             # Customer management
│   │   ├── charge.rs               # Charge processing
│   │   ├── subscription.rs         # Subscription management
│   │   ├── webhooks.rs             # Webhook handling
│   │   └── ...                     # Other Stripe APIs
│   ├── paypal/                     # PayPal provider implementation
│   │   ├── mod.rs                  # Module exports
│   │   ├── provider.rs             # PaymentProvider implementation
│   │   ├── client.rs               # PayPal API client
│   │   ├── orders.rs               # Order management
│   │   ├── payments.rs             # Payment processing
│   │   ├── subscriptions.rs        # Subscription handling
│   │   └── webhooks.rs             # Webhook verification
│   ├── square/                     # Square provider implementation
│   │   ├── mod.rs                  # Module exports
│   │   ├── provider.rs             # PaymentProvider implementation
│   │   ├── client.rs               # Square API client
│   │   ├── payments.rs             # Payment processing
│   │   ├── catalog.rs              # Catalog management
│   │   ├── customers.rs            # Customer management
│   │   └── webhooks.rs             # Webhook handling
│   └── crypto/                     # Cryptocurrency support
│       ├── mod.rs                  # Module exports
│       ├── bitcoin.rs              # Bitcoin implementation
│       ├── ethereum.rs             # Ethereum implementation
│       ├── providers.rs            # Blockchain providers
│       └── wallet.rs               # Wallet management
├── examples/                       # Usage examples
│   ├── unified_payment_processing.rs  # Unified API example
│   ├── payment_intent_example.rs      # Stripe Payment Intent
│   ├── square_webhook_example.rs      # Square webhooks
│   └── multi_platform.rs             # Multi-provider example
├── tests/                          # Test suite
│   ├── integration_test.rs        # Integration tests
│   ├── unit_tests.rs              # Unit tests
│   └── ...                        # Provider-specific tests
└── Cargo.toml                     # Project dependencies

```

## Dependencies

### Core Dependencies
- **serde** (1.0): Serialization/deserialization
- **serde_json** (1.0): JSON handling
- **reqwest** (0.11): HTTP client with rustls-tls
- **tokio** (1.40): Async runtime
- **async-trait** (0.1): Async trait support
- **thiserror** (1.0): Error handling
- **log** (0.4): Logging facade

### Security & Cryptography
- **sha2** (0.10): SHA hashing
- **hmac** (0.12): HMAC for webhook verification
- **base64** (0.22): Base64 encoding
- **hex** (0.4): Hex encoding

### Utilities
- **chrono** (0.4): Date/time handling
- **uuid** (1.11): UUID generation
- **url** (2.5): URL parsing
- **urlencoding** (2.1): URL encoding
- **once_cell** (1.20): Lazy static initialization
- **rand** (0.8): Random number generation

## Architecture Patterns

### 1. Unified Payment Provider Interface
All payment providers implement the `PaymentProvider` trait, providing a consistent API:
```rust
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn create_charge(&self, charge: Charge) -> Result<Charge>;
    async fn get_charge(&self, charge_id: &str) -> Result<Charge>;
    async fn refund_charge(&self, charge_id: &str, amount: Option<Money>) -> Result<Refund>;
    // ... other methods
}
```

### 2. Provider Factory Pattern
Dynamic provider instantiation based on configuration:
```rust
let provider = ProviderFactory::create(ProviderConfig {
    provider: "stripe".to_string(),
    api_key: "sk_test_...".to_string(),
    sandbox: true,
})?;
```

### 3. Rate Limiting with Circuit Breaker
Automatic rate limiting and fault tolerance:
- Configurable per-endpoint rate limits
- Exponential backoff on rate limit errors
- Circuit breaker for API failures
- Thread-safe global rate limiter

### 4. Builder Pattern
Flexible configuration using builders:
```rust
let provider = ProviderBuilder::new()
    .provider("paypal")
    .api_key("client_id")
    .client_secret("secret")
    .sandbox(true)
    .build()?;
```

## Recent Improvements

### Code Quality
- Fixed 71 failing doc tests
- Resolved 57+ compiler warnings
- Rust 2024 compatibility
- Zero warnings in codebase

### New Features
- Unified PaymentProvider trait for all providers
- Provider factory with environment variable support
- Comprehensive rate limiting system
- Circuit breaker for fault tolerance
- Payment Intent API for Stripe
- Transfer API for Stripe
- Connect Account API for Stripe

### Testing
- 29 passing unit tests
- 8+ integration tests per provider
- Comprehensive webhook testing
- Rate limiter stress testing

## Usage Examples

### Basic Payment Processing
```rust
use payup::provider_factory::ProviderFactory;
use payup::payment_provider::{Charge, Money};

let provider = ProviderFactory::from_env()?;
let charge = Charge {
    amount: Money {
        amount: 2000,
        currency: "USD".to_string(),
    },
    customer_id: Some("cust_123".to_string()),
    description: Some("Payment for order #123".to_string()),
    ..Default::default()
};

let result = provider.create_charge(charge).await?;
```

### Webhook Handling
```rust
use payup::stripe::webhooks::verify_webhook_signature;

let signature = request.headers().get("Stripe-Signature")?;
let payload = request.body();
let webhook_secret = "whsec_...";

if verify_webhook_signature(payload, signature, webhook_secret)? {
    // Process webhook event
}
```

## Testing

Run tests with:
```bash
# Unit tests
cargo test --lib

# Integration tests (requires API keys)
cargo test --test integration_test

# Provider-specific tests
cargo test --test stripe_webhook_tests
cargo test --test square_integration_tests
```

## Environment Configuration

Set these environment variables for automatic provider configuration:
```bash
PAYMENT_PROVIDER=stripe|paypal|square
PAYMENT_API_KEY=your_api_key
PAYMENT_CLIENT_SECRET=your_secret (for PayPal/Square)
PAYMENT_SANDBOX=true|false
```

## Build Commands

```bash
# Build the library
cargo build --release

# Run linting
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Generate documentation
cargo doc --no-deps --open
```

## Security Considerations

1. **API Key Management**: Never commit API keys. Use environment variables.
2. **Webhook Verification**: Always verify webhook signatures before processing.
3. **Rate Limiting**: Built-in rate limiting prevents API abuse.
4. **HTTPS Only**: All API calls use HTTPS with rustls.
5. **Input Validation**: All inputs are validated before API calls.

## Contributing

When contributing to this codebase:
1. Follow existing code patterns and conventions
2. Add tests for new functionality
3. Update documentation for API changes
4. Run `cargo clippy` and `cargo fmt` before committing
5. Ensure all tests pass with `cargo test`

## License

This project is dual-licensed under MIT OR Apache-2.0. See LICENSE and LICENSE-MIT files for details.