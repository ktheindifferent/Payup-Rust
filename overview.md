# Payup - Multi-Platform Payment Processing Library

## Project Overview

Payup is a comprehensive Rust library for payment processing that provides unified interfaces for multiple payment platforms including Stripe, PayPal, Square, and various cryptocurrency providers.

## Architecture

### Core Design Principles
- **Unified Interface**: Trait-based architecture with `PaymentProvider` trait for consistent API across platforms
- **Dual Mode Support**: Both synchronous and asynchronous operations supported
- **Modular Structure**: Each payment platform is isolated in its own module
- **Comprehensive Error Handling**: Custom `PayupError` enum with provider-specific error variants

### Module Structure

```
src/
├── lib.rs                    # Main library entry point
├── payment_provider.rs       # Unified payment interface traits
├── error.rs                  # Centralized error handling
├── http_utils.rs            # Shared HTTP utilities
├── stripe.rs & stripe_ext/  # Stripe integration (16+ APIs)
├── paypal/                  # PayPal integration (7 modules)
├── square/                  # Square integration (5 modules)
└── crypto/                  # Cryptocurrency support (7 modules)
```

## Features

### Payment Platforms

#### Stripe Integration
- **Core APIs**: Balance, Charge, Customer, Invoice, Subscription, PaymentMethod
- **Extended APIs**: Product, Refund, Payout, Dispute, Event, Files
- **Coverage**: 16 fully implemented APIs with more in development

#### PayPal Integration
- **OAuth Authentication**: Full OAuth2 implementation
- **APIs**: Orders, Payments, Subscriptions, Webhooks
- **Features**: Invoice management, dispute handling, refund processing

#### Square Integration
- **Authentication**: OAuth2 and API key support
- **APIs**: Payments, Customers, Catalog, Locations
- **Features**: Transaction processing, customer management

#### Cryptocurrency Support
- **Supported Coins**: Bitcoin, Ethereum, Litecoin, Dogecoin, and 7+ others
- **Providers**: Native, Coinbase Commerce, BitPay, CoinGate
- **Features**: Wallet management, address validation, transaction monitoring
- **Layer 2**: Polygon, Arbitrum, Optimism support

### Technical Features

- **Async/Sync Operations**: Full support for both paradigms
- **Error Handling**: Comprehensive error types with detailed context
- **HTTP Utilities**: Shared request/response handling reduces code duplication
- **Feature Flags**: Selective compilation of payment providers
- **Testing**: Unit tests, integration tests (requires API keys)

## Current State

### Version
- **Current**: 0.4.0
- **Previous**: 0.1.45 (major architectural changes)
- **License**: MIT OR Apache-2.0 (dual licensed)

### Dependencies
- **Runtime**: tokio 1.45, reqwest 0.11
- **Serialization**: serde ecosystem
- **Crypto**: Multiple specialized libraries per provider

### Test Coverage
- **Unit Tests**: 17/17 passing
- **Integration Tests**: 7/9 passing (2 require API keys)
- **Doc Tests**: 5/76 passing (71 need fixing)

## Known Issues

### High Priority
1. **Documentation**: 71 failing doc tests need immediate attention
2. **Compiler Warnings**: 57 warnings from unused imports/variables
3. **Code Organization**: stripe.rs is 4,638 lines (needs modularization)
4. **Rust 2024**: Never type fallback warning in PayPal module

### Medium Priority
1. **Missing APIs**: Several Stripe APIs not yet implemented
2. **Test Coverage**: PayPal, Square, Crypto lack integration tests
3. **Production Features**: No rate limiting or retry logic
4. **Logging**: No structured logging system

## Development Status

### Recently Completed
- Multi-platform payment architecture
- Unified payment provider interface
- Comprehensive cryptocurrency support
- HTTP utilities extraction and refactoring

### In Progress
- Documentation improvements
- Code cleanup and warning resolution
- Stripe API expansion
- Test coverage improvements

### Planned
- Rate limiting and retry logic
- Request/response caching
- Performance benchmarking
- Comprehensive API documentation

## Usage

### Basic Example

```rust
use payup::{PaymentProvider, StripeClient};

// Synchronous usage
let client = StripeClient::new("sk_test_...");
let charge = client.create_charge(100, "usd", "tok_visa")?;

// Asynchronous usage
let client = StripeClient::new_async("sk_test_...");
let charge = client.create_charge_async(100, "usd", "tok_visa").await?;
```

### Feature Flags

```toml
# Default (Stripe + PayPal)
payup = "0.4.0"

# All platforms
payup = { version = "0.4.0", features = ["all"] }

# Specific platforms
payup = { version = "0.4.0", features = ["stripe", "square", "crypto"] }
```

## Contributing

The project welcomes contributions. Key areas needing help:
- Fixing failing doc tests
- Implementing missing Stripe APIs
- Adding integration test coverage
- Improving documentation
- Performance optimization

See todo.md for current task list and project_description.md for detailed development tracking.