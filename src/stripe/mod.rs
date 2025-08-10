pub mod response;
pub mod webhooks;
pub mod provider;

// Core modules
pub mod account;
pub mod auth;
pub mod balance;
pub mod charge;
pub mod customer;
pub mod payment_intent;
pub mod payment_method;
pub mod plan;
pub mod subscription;
pub mod transfer;

// Re-exports for backward compatibility
pub use account::{Account, CreateAccountParams, BusinessProfile, Capabilities, Requirements, AccountSettings};
pub use auth::Auth;
pub use balance::{Balance, BalanceTransaction, BalanceAvailable, BalancePending, BalanceTransactions, FeeDetail, BalanceSourceTypes};
pub use charge::{Charge, Card, Charges, PaymentMethodDetails, FraudDetails, BillingDetails, Address, Refunds, SepaDebit};
pub use customer::{Customer, Customers};
pub use payment_intent::{
    PaymentIntent, PaymentIntentStatus, ConfirmationMethod, CaptureMethod, SetupFutureUsage,
    CreatePaymentIntentParams, UpdatePaymentIntentParams, ConfirmPaymentIntentParams,
    CapturePaymentIntentParams, CancelPaymentIntentParams, AutomaticPaymentMethods,
    ShippingDetails, Address as PaymentIntentAddress, PaymentMethodOptions, TransferData
};
pub use payment_method::PaymentMethod;
pub use plan::{Plan, Price};
pub use subscription::Subscription;
pub use transfer::{Transfer, TransferReversal, TransferReversalList, CreateTransferParams, UpdateTransferParams, CreateReversalParams};
pub use webhooks::{StripeWebhookHandler, WebhookEvent, WebhookEventType, WebhookEventData, WebhookRequest};
pub use provider::StripeProvider;