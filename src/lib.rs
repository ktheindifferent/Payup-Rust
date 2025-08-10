// Copyright 2021 Caleb Mitchell Smith-Woolrich (PixelCoda)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ## Description
//!
//! A synchronous + asynchronous payment library for processing payments with rust + stripe.
//!
//! I built this library due to a lack of synchronous payment libraries. Currently im only focused on features I need for another project.
//!
//! ## Current Stripe Features:
//!
//! * Balance:
//!     * Ability to fetch the account balance for your stripe account
//!
//! * BalanceTransaction:
//!     * Ability to retrieve a BalanceTransaction
//!     * Ability to list all BalanceTransactions
//!
//! * Card:
//!     * Ability to attach a Card to a PaymentMethod
//!
//! * Charge:
//!     * Ability to retrieve a Charge
//!     * Ability to list all Charges
//!     * Ability to update an existing Charge
//!     * Ability to create a new Charge
//!     * Ability to capture a charge
//!
//! * Customer:
//!     * Ability to retrieve a Customer
//!     * Ability to list all Customers
//!     * Ability to update an existing Customer
//!     * Ability to create a new Customer
//!     * Ability to destroy a Customer
//!     * Ability to attach payment methods to Customers
//!     * Ability to list a customers invoices
//!     * Ability to list a customers payment methods
//!
//! * Dispute:
//!     * Ability to retrieve a Dispute
//!     * Ability to list all Disputes
//!     * Ability to close a Dispute
//!     * Ability to update an existing Dispute
//!
//! * Event:
//!     * Ability to retrieve an Event
//!     * Ability to list all Events
//!
//! * Files:
//!     * Ability to retrieve a File
//!     * Ability to list all Files
//!     * Ability to create a File
//!
//! * FileLink:
//!     * Ability to retrieve a FileLink
//!     * Ability to list all FileLinks
//!     * Ability to create a FileLink
//!     * Ability to update an existing FileLink
//!
//! * Invoice:
//!     * Ability to retrieve an Invoice
//!     * Ability to list all Invoices
//!     * Ability to create an Invoice
//!     * Ability to update an existing Invoice
//!
//! * Mandate:
//!     * Ability to retrieve a Mandate
//!
//! * PaymentMethod:
//!     * Ability to retrieve a PaymentMethod
//!     * Ability to create a new PaymentMethod
//!
//! * Plan:
//!     * Ability to retrieve a Plan
//!     * Ability to list all Plan
//!     * Ability to update an existing Plan
//!     * Ability to create a new Plan
//!
//! * Price
//!     * Ability to create a new Price
//!
//! * Subscription
//!     * Ability to create a new Subscription
//!     * Ability to retrieve a Subscription
//!     * Ability to update an existing Subscription
//!     * Ability to cancel a subscription

/// Stripe API Implementation
pub mod stripe;

/// Error handling module
pub mod error;

/// HTTP utilities for reducing code duplication
pub mod http_utils;

/// Unified payment provider interface
pub mod payment_provider;
pub mod provider_factory;

/// PayPal API Implementation
pub mod paypal;

/// Square API Implementation
pub mod square;

/// Extended Stripe API implementations
pub mod stripe_ext;

/// Cryptocurrency payment support
#[cfg(feature = "crypto")]
pub mod crypto;

/// Rate limiting and retry logic for API calls
pub mod rate_limiter;

/// Centralized HTTP client with connection pooling
pub mod http_client;

/// Configuration and constants
pub mod config;

/// Builder patterns and utilities
pub mod builders;

/// Async/sync code generation macros
pub mod async_sync_macro;

/// Safe utility functions for error handling
pub mod safe_utils;
