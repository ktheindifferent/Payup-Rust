use std::collections::HashMap;
use crate::stripe::{
    Charge, Shipping, ShippingAddress, BillingDetails, Address,
    Event, EventData, EventRequest,
    Invoice, InvoiceStatus, BillingReason, CollectionMethod, CreateInvoiceParams, UpdateInvoiceParams,
    Plan, Price, CreatePlanParams, CreatePriceParams, UpdatePlanParams, UpdatePriceParams,
    BillingScheme, Interval, AggregateUsage, TiersMode, UsageType, PriceType, TaxBehavior, Recurring,
    PaymentIntent, CreatePaymentIntentParams, UpdatePaymentIntentParams, ConfirmPaymentIntentParams,
    CaptureMethod, ConfirmationMethod, SetupFutureUsage, ShippingDetails, Address as PaymentIntentAddress,
};

/// Builder for creating a Charge
pub struct ChargeBuilder {
    charge: Charge,
}

impl ChargeBuilder {
    /// Creates a new ChargeBuilder
    pub fn new() -> Self {
        ChargeBuilder {
            charge: Charge::new(),
        }
    }

    /// Sets the amount to charge (in cents)
    pub fn amount(mut self, amount: i64) -> Self {
        self.charge.stripe_amount = Some(amount);
        self.charge.amount = Some(amount.to_string());
        self
    }

    /// Sets the currency
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.charge.currency = Some(currency.into());
        self
    }

    /// Sets the customer ID
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.charge.customer = Some(customer.into());
        self
    }

    /// Sets the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.charge.description = Some(description.into());
        self
    }

    /// Sets the receipt email
    pub fn receipt_email(mut self, email: impl Into<String>) -> Self {
        self.charge.receipt_email = Some(email.into());
        self
    }

    /// Sets the payment source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.charge.source = Some(source.into());
        self
    }

    /// Sets the statement descriptor
    pub fn statement_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.charge.statement_descriptor = Some(descriptor.into());
        self
    }

    /// Sets the shipping information
    pub fn shipping(mut self, shipping: Shipping) -> Self {
        self.charge.shipping = Some(shipping);
        self
    }

    /// Sets billing details
    pub fn billing_details(mut self, details: BillingDetails) -> Self {
        self.charge.billing_details = Some(details);
        self
    }

    /// Sets whether to capture the charge immediately
    pub fn captured(mut self, captured: bool) -> Self {
        self.charge.captured = Some(captured);
        self
    }

    /// Builds the Charge
    pub fn build(self) -> Charge {
        self.charge
    }
}

/// Builder for creating Shipping information
pub struct ShippingBuilder {
    shipping: Shipping,
}

impl ShippingBuilder {
    /// Creates a new ShippingBuilder
    pub fn new() -> Self {
        ShippingBuilder {
            shipping: Shipping::new(),
        }
    }

    /// Sets the recipient name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.shipping.name = Some(name.into());
        self
    }

    /// Sets the phone number
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.shipping.phone = Some(phone.into());
        self
    }

    /// Sets the carrier
    pub fn carrier(mut self, carrier: impl Into<String>) -> Self {
        self.shipping.carrier = Some(carrier.into());
        self
    }

    /// Sets the tracking number
    pub fn tracking_number(mut self, tracking_number: impl Into<String>) -> Self {
        self.shipping.tracking_number = Some(tracking_number.into());
        self
    }

    /// Sets the address
    pub fn address(mut self, address: ShippingAddress) -> Self {
        self.shipping.address = Some(address);
        self
    }

    /// Builds the Shipping
    pub fn build(self) -> Shipping {
        self.shipping
    }
}

/// Builder for creating a ShippingAddress
pub struct ShippingAddressBuilder {
    address: ShippingAddress,
}

impl ShippingAddressBuilder {
    /// Creates a new ShippingAddressBuilder
    pub fn new() -> Self {
        ShippingAddressBuilder {
            address: ShippingAddress {
                city: None,
                country: None,
                line1: None,
                line2: None,
                postal_code: None,
                state: None,
            },
        }
    }

    /// Sets the city
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.address.city = Some(city.into());
        self
    }

    /// Sets the country
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.address.country = Some(country.into());
        self
    }

    /// Sets the first address line
    pub fn line1(mut self, line1: impl Into<String>) -> Self {
        self.address.line1 = Some(line1.into());
        self
    }

    /// Sets the second address line
    pub fn line2(mut self, line2: impl Into<String>) -> Self {
        self.address.line2 = Some(line2.into());
        self
    }

    /// Sets the postal code
    pub fn postal_code(mut self, postal_code: impl Into<String>) -> Self {
        self.address.postal_code = Some(postal_code.into());
        self
    }

    /// Sets the state
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.address.state = Some(state.into());
        self
    }

    /// Builds the ShippingAddress
    pub fn build(self) -> ShippingAddress {
        self.address
    }
}

/// Builder for creating PaymentIntent parameters
pub struct PaymentIntentBuilder {
    params: CreatePaymentIntentParams,
}

impl PaymentIntentBuilder {
    /// Creates a new PaymentIntentBuilder
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        PaymentIntentBuilder {
            params: CreatePaymentIntentParams {
                amount,
                currency: currency.into(),
                automatic_payment_methods: None,
                capture_method: None,
                confirm: None,
                confirmation_method: None,
                customer: None,
                description: None,
                metadata: None,
                off_session: None,
                on_behalf_of: None,
                payment_method: None,
                payment_method_options: None,
                payment_method_types: None,
                receipt_email: None,
                setup_future_usage: None,
                shipping: None,
                statement_descriptor: None,
                statement_descriptor_suffix: None,
                transfer_data: None,
                transfer_group: None,
            },
        }
    }

    /// Sets the customer ID
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.params.customer = Some(customer.into());
        self
    }

    /// Sets the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = Some(description.into());
        self
    }

    /// Sets the capture method
    pub fn capture_method(mut self, method: CaptureMethod) -> Self {
        self.params.capture_method = Some(method);
        self
    }

    /// Sets whether to confirm immediately
    pub fn confirm(mut self, confirm: bool) -> Self {
        self.params.confirm = Some(confirm);
        self
    }

    /// Sets the payment method
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.params.payment_method = Some(payment_method.into());
        self
    }

    /// Sets the receipt email
    pub fn receipt_email(mut self, email: impl Into<String>) -> Self {
        self.params.receipt_email = Some(email.into());
        self
    }

    /// Sets shipping details
    pub fn shipping(mut self, shipping: ShippingDetails) -> Self {
        self.params.shipping = Some(shipping);
        self
    }

    /// Sets metadata
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.params.metadata = Some(metadata);
        self
    }

    /// Sets the statement descriptor
    pub fn statement_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.params.statement_descriptor = Some(descriptor.into());
        self
    }

    /// Sets the statement descriptor suffix
    pub fn statement_descriptor_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.params.statement_descriptor_suffix = Some(suffix.into());
        self
    }

    /// Sets setup future usage
    pub fn setup_future_usage(mut self, usage: SetupFutureUsage) -> Self {
        self.params.setup_future_usage = Some(usage);
        self
    }

    /// Builds the CreatePaymentIntentParams
    pub fn build(self) -> CreatePaymentIntentParams {
        self.params
    }
}

/// Builder for creating Invoice parameters
pub struct InvoiceBuilder {
    params: CreateInvoiceParams,
}

impl InvoiceBuilder {
    /// Creates a new InvoiceBuilder
    pub fn new(customer: impl Into<String>) -> Self {
        InvoiceBuilder {
            params: CreateInvoiceParams {
                customer: customer.into(),
                auto_advance: None,
                collection_method: None,
                description: None,
                metadata: None,
                subscription: None,
                account_tax_ids: None,
                application_fee_amount: None,
                custom_fields: None,
                days_until_due: None,
                default_payment_method: None,
                default_source: None,
                default_tax_rates: None,
                discounts: None,
                due_date: None,
                footer: None,
                on_behalf_of: None,
                statement_descriptor: None,
                transfer_data: None,
            },
        }
    }

    /// Sets the collection method
    pub fn collection_method(mut self, method: CollectionMethod) -> Self {
        self.params.collection_method = Some(method);
        self
    }

    /// Sets the description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = Some(description.into());
        self
    }

    /// Sets auto advance
    pub fn auto_advance(mut self, auto_advance: bool) -> Self {
        self.params.auto_advance = Some(auto_advance);
        self
    }

    /// Sets the subscription ID
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.params.subscription = Some(subscription.into());
        self
    }

    /// Sets metadata
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.params.metadata = Some(metadata);
        self
    }

    /// Sets days until due
    pub fn days_until_due(mut self, days: i32) -> Self {
        self.params.days_until_due = Some(days);
        self
    }

    /// Sets the due date
    pub fn due_date(mut self, timestamp: i64) -> Self {
        self.params.due_date = Some(timestamp);
        self
    }

    /// Sets the footer
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.params.footer = Some(footer.into());
        self
    }

    /// Sets the statement descriptor
    pub fn statement_descriptor(mut self, descriptor: impl Into<String>) -> Self {
        self.params.statement_descriptor = Some(descriptor.into());
        self
    }

    /// Sets the default payment method
    pub fn default_payment_method(mut self, payment_method: impl Into<String>) -> Self {
        self.params.default_payment_method = Some(payment_method.into());
        self
    }

    /// Builds the CreateInvoiceParams
    pub fn build(self) -> CreateInvoiceParams {
        self.params
    }
}

/// Builder for creating Plan parameters
pub struct PlanBuilder {
    params: CreatePlanParams,
}

impl PlanBuilder {
    /// Creates a new PlanBuilder
    pub fn new(currency: impl Into<String>, interval: Interval, product: impl Into<String>) -> Self {
        PlanBuilder {
            params: CreatePlanParams {
                currency: currency.into(),
                interval,
                product: product.into(),
                active: None,
                aggregate_usage: None,
                amount: None,
                amount_decimal: None,
                billing_scheme: None,
                id: None,
                interval_count: None,
                metadata: None,
                nickname: None,
                tiers: None,
                tiers_mode: None,
                transform_usage: None,
                trial_period_days: None,
                usage_type: None,
            },
        }
    }

    /// Sets the amount (in cents)
    pub fn amount(mut self, amount: i64) -> Self {
        self.params.amount = Some(amount);
        self
    }

    /// Sets whether the plan is active
    pub fn active(mut self, active: bool) -> Self {
        self.params.active = Some(active);
        self
    }

    /// Sets the plan ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.params.id = Some(id.into());
        self
    }

    /// Sets the interval count
    pub fn interval_count(mut self, count: u32) -> Self {
        self.params.interval_count = Some(count);
        self
    }

    /// Sets the nickname
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.params.nickname = Some(nickname.into());
        self
    }

    /// Sets metadata
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.params.metadata = Some(metadata);
        self
    }

    /// Sets the billing scheme
    pub fn billing_scheme(mut self, scheme: BillingScheme) -> Self {
        self.params.billing_scheme = Some(scheme);
        self
    }

    /// Sets the usage type
    pub fn usage_type(mut self, usage_type: UsageType) -> Self {
        self.params.usage_type = Some(usage_type);
        self
    }

    /// Sets the trial period days
    pub fn trial_period_days(mut self, days: u32) -> Self {
        self.params.trial_period_days = Some(days);
        self
    }

    /// Sets the aggregate usage
    pub fn aggregate_usage(mut self, aggregate: AggregateUsage) -> Self {
        self.params.aggregate_usage = Some(aggregate);
        self
    }

    /// Builds the CreatePlanParams
    pub fn build(self) -> CreatePlanParams {
        self.params
    }
}

/// Builder for creating Price parameters
pub struct PriceBuilder {
    params: CreatePriceParams,
}

impl PriceBuilder {
    /// Creates a new PriceBuilder
    pub fn new(currency: impl Into<String>, product: impl Into<String>) -> Self {
        PriceBuilder {
            params: CreatePriceParams {
                currency: currency.into(),
                product: product.into(),
                active: None,
                billing_scheme: None,
                lookup_key: None,
                metadata: None,
                nickname: None,
                recurring: None,
                tax_behavior: None,
                tiers: None,
                tiers_mode: None,
                transform_quantity: None,
                unit_amount: None,
                unit_amount_decimal: None,
            },
        }
    }

    /// Sets the unit amount (in cents)
    pub fn unit_amount(mut self, amount: i64) -> Self {
        self.params.unit_amount = Some(amount);
        self
    }

    /// Sets whether the price is active
    pub fn active(mut self, active: bool) -> Self {
        self.params.active = Some(active);
        self
    }

    /// Sets the lookup key
    pub fn lookup_key(mut self, key: impl Into<String>) -> Self {
        self.params.lookup_key = Some(key.into());
        self
    }

    /// Sets the nickname
    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.params.nickname = Some(nickname.into());
        self
    }

    /// Sets metadata
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.params.metadata = Some(metadata);
        self
    }

    /// Sets the billing scheme
    pub fn billing_scheme(mut self, scheme: BillingScheme) -> Self {
        self.params.billing_scheme = Some(scheme);
        self
    }

    /// Sets the tax behavior
    pub fn tax_behavior(mut self, behavior: TaxBehavior) -> Self {
        self.params.tax_behavior = Some(behavior);
        self
    }

    /// Sets recurring price parameters
    pub fn recurring(mut self, recurring: Recurring) -> Self {
        self.params.recurring = Some(recurring);
        self
    }

    /// Sets recurring price with simple parameters
    pub fn recurring_simple(mut self, interval: Interval, interval_count: u32) -> Self {
        self.params.recurring = Some(Recurring {
            interval,
            interval_count,
            aggregate_usage: None,
            trial_period_days: None,
            usage_type: None,
        });
        self
    }

    /// Builds the CreatePriceParams
    pub fn build(self) -> CreatePriceParams {
        self.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_builder() {
        let charge = ChargeBuilder::new()
            .amount(2000)
            .currency("usd")
            .customer("cust_123")
            .description("Test charge")
            .receipt_email("test@example.com")
            .build();

        assert_eq!(charge.stripe_amount, Some(2000));
        assert_eq!(charge.currency, Some("usd".to_string()));
        assert_eq!(charge.customer, Some("cust_123".to_string()));
        assert_eq!(charge.description, Some("Test charge".to_string()));
        assert_eq!(charge.receipt_email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_shipping_builder() {
        let address = ShippingAddressBuilder::new()
            .line1("123 Main St")
            .city("San Francisco")
            .state("CA")
            .postal_code("94102")
            .country("US")
            .build();

        let shipping = ShippingBuilder::new()
            .name("John Doe")
            .phone("+14155551234")
            .address(address)
            .carrier("USPS")
            .tracking_number("1234567890")
            .build();

        assert_eq!(shipping.name, Some("John Doe".to_string()));
        assert_eq!(shipping.phone, Some("+14155551234".to_string()));
        assert_eq!(shipping.carrier, Some("USPS".to_string()));
        assert_eq!(shipping.tracking_number, Some("1234567890".to_string()));
        assert!(shipping.address.is_some());
    }

    #[test]
    fn test_payment_intent_builder() {
        let params = PaymentIntentBuilder::new(2000, "usd")
            .customer("cust_123")
            .description("Test payment")
            .receipt_email("test@example.com")
            .capture_method(CaptureMethod::Manual)
            .build();

        assert_eq!(params.amount, 2000);
        assert_eq!(params.currency, "usd");
        assert_eq!(params.customer, Some("cust_123".to_string()));
        assert_eq!(params.description, Some("Test payment".to_string()));
        assert_eq!(params.receipt_email, Some("test@example.com".to_string()));
        assert_eq!(params.capture_method, Some(CaptureMethod::Manual));
    }

    #[test]
    fn test_invoice_builder() {
        let params = InvoiceBuilder::new("cust_123")
            .description("Monthly subscription")
            .collection_method(CollectionMethod::ChargeAutomatically)
            .days_until_due(30)
            .build();

        assert_eq!(params.customer, "cust_123");
        assert_eq!(params.description, Some("Monthly subscription".to_string()));
        assert_eq!(params.collection_method, Some(CollectionMethod::ChargeAutomatically));
        assert_eq!(params.days_until_due, Some(30));
    }

    #[test]
    fn test_plan_builder() {
        let params = PlanBuilder::new("usd", Interval::Month, "prod_123")
            .amount(999)
            .nickname("Basic Plan")
            .interval_count(1)
            .trial_period_days(14)
            .build();

        assert_eq!(params.currency, "usd");
        assert_eq!(params.interval, Interval::Month);
        assert_eq!(params.product, "prod_123");
        assert_eq!(params.amount, Some(999));
        assert_eq!(params.nickname, Some("Basic Plan".to_string()));
        assert_eq!(params.interval_count, Some(1));
        assert_eq!(params.trial_period_days, Some(14));
    }

    #[test]
    fn test_price_builder() {
        let params = PriceBuilder::new("usd", "prod_123")
            .unit_amount(1999)
            .nickname("Pro Price")
            .recurring_simple(Interval::Month, 1)
            .tax_behavior(TaxBehavior::Inclusive)
            .build();

        assert_eq!(params.currency, "usd");
        assert_eq!(params.product, "prod_123");
        assert_eq!(params.unit_amount, Some(1999));
        assert_eq!(params.nickname, Some("Pro Price".to_string()));
        assert_eq!(params.tax_behavior, Some(TaxBehavior::Inclusive));
        assert!(params.recurring.is_some());
    }
}