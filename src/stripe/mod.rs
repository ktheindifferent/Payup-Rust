pub mod response;
pub mod webhooks;
pub mod provider;

// Core modules
pub mod account;
pub mod auth;
pub mod balance;
pub mod builders;
pub mod charge;
pub mod customer;
pub mod event;
pub mod invoice;
pub mod payment_intent;
pub mod payment_method;
pub mod plan;
pub mod subscription;
pub mod transfer;

// Re-exports for backward compatibility
pub use account::{Account, CreateAccountParams, BusinessProfile, Capabilities, Requirements, AccountSettings};
pub use auth::Auth;
pub use balance::{Balance, BalanceTransaction, BalanceAvailable, BalancePending, BalanceTransactions, FeeDetail, BalanceSourceTypes};
pub use charge::{Charge, Card, Charges, PaymentMethodDetails, FraudDetails, BillingDetails, Address, Refunds, SepaDebit, Shipping, ShippingAddress};
pub use event::{Event, EventData, EventRequest, EventList, ListEventsParams, EventTimeFilter, event_types};
pub use invoice::{Invoice, InvoiceList, CreateInvoiceParams, UpdateInvoiceParams, InvoiceStatus, BillingReason, CollectionMethod, InvoiceLineItem, InvoiceLineItemList};
pub use customer::{Customer, Customers};
pub use payment_intent::{
    PaymentIntent, PaymentIntentStatus, ConfirmationMethod, CaptureMethod, SetupFutureUsage,
    CreatePaymentIntentParams, UpdatePaymentIntentParams, ConfirmPaymentIntentParams,
    CapturePaymentIntentParams, CancelPaymentIntentParams, AutomaticPaymentMethods,
    ShippingDetails, Address as PaymentIntentAddress, PaymentMethodOptions, TransferData
};
pub use payment_method::{
    PaymentMethod, PaymentMethodType as StripePaymentMethodType, 
    CreatePaymentMethodParams, CreateCardParams, BillingDetails as PaymentMethodBillingDetails,
    CardDetails as StripeCardDetails, Address as PaymentMethodAddress
};
pub use plan::{
    Plan, Price, PlanList, PriceList, PriceSearchResult, 
    CreatePlanParams, UpdatePlanParams, CreatePriceParams, UpdatePriceParams,
    BillingScheme, Interval, AggregateUsage, TiersMode, UsageType, PriceType, TaxBehavior,
    TransformUsage, TransformQuantity, RoundingMode, Recurring, PlanTier, PriceTier
};
pub use subscription::Subscription;
pub use transfer::{Transfer, TransferReversal, TransferReversalList, CreateTransferParams, UpdateTransferParams, CreateReversalParams};
pub use webhooks::{StripeWebhookHandler, WebhookEvent, WebhookEventType, WebhookEventData, WebhookRequest};
pub use provider::StripeProvider;
pub use builders::{
    ChargeBuilder, ShippingBuilder, ShippingAddressBuilder,
    PaymentIntentBuilder, InvoiceBuilder, PlanBuilder, PriceBuilder
};