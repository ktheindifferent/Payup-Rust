#[cfg(test)]
mod stripe_unit_tests {
    use payup::stripe::{
        Charge, Shipping, ShippingAddress,
        Event, EventData, EventRequest, EventList, ListEventsParams, EventTimeFilter,
        Invoice, InvoiceStatus, BillingReason, CollectionMethod,
        Plan, Price, BillingScheme, Interval, UsageType, PriceType, TaxBehavior,
        ChargeBuilder, ShippingBuilder, ShippingAddressBuilder,
        PaymentIntentBuilder, InvoiceBuilder, PlanBuilder, PriceBuilder,
    };
    use std::collections::HashMap;
    use serde_json::json;

    #[test]
    fn test_charge_with_shipping() {
        let address = ShippingAddress {
            city: Some("San Francisco".to_string()),
            country: Some("US".to_string()),
            line1: Some("123 Main St".to_string()),
            line2: None,
            postal_code: Some("94102".to_string()),
            state: Some("CA".to_string()),
        };

        let shipping = Shipping {
            address: Some(address),
            carrier: Some("USPS".to_string()),
            name: Some("John Doe".to_string()),
            phone: Some("+14155551234".to_string()),
            tracking_number: Some("1234567890".to_string()),
        };

        let mut charge = Charge::new();
        charge.shipping = Some(shipping);

        assert!(charge.shipping.is_some());
        let shipping = charge.shipping.unwrap();
        assert_eq!(shipping.name, Some("John Doe".to_string()));
        assert_eq!(shipping.carrier, Some("USPS".to_string()));
    }

    #[test]
    fn test_shipping_to_params() {
        let address = ShippingAddress {
            city: Some("San Francisco".to_string()),
            country: Some("US".to_string()),
            line1: Some("123 Main St".to_string()),
            line2: Some("Apt 4B".to_string()),
            postal_code: Some("94102".to_string()),
            state: Some("CA".to_string()),
        };

        let shipping = Shipping {
            address: Some(address),
            carrier: Some("FedEx".to_string()),
            name: Some("Jane Smith".to_string()),
            phone: Some("+14155555678".to_string()),
            tracking_number: Some("9876543210".to_string()),
        };

        let params = shipping.to_params();
        
        // Check that all parameters are present
        assert!(params.iter().any(|(k, v)| k == "shipping[address][city]" && v == "San Francisco"));
        assert!(params.iter().any(|(k, v)| k == "shipping[address][country]" && v == "US"));
        assert!(params.iter().any(|(k, v)| k == "shipping[address][line1]" && v == "123 Main St"));
        assert!(params.iter().any(|(k, v)| k == "shipping[address][line2]" && v == "Apt 4B"));
        assert!(params.iter().any(|(k, v)| k == "shipping[address][postal_code]" && v == "94102"));
        assert!(params.iter().any(|(k, v)| k == "shipping[address][state]" && v == "CA"));
        assert!(params.iter().any(|(k, v)| k == "shipping[carrier]" && v == "FedEx"));
        assert!(params.iter().any(|(k, v)| k == "shipping[name]" && v == "Jane Smith"));
        assert!(params.iter().any(|(k, v)| k == "shipping[phone]" && v == "+14155555678"));
        assert!(params.iter().any(|(k, v)| k == "shipping[tracking_number]" && v == "9876543210"));
    }

    #[test]
    fn test_event_data_extraction() {
        let event_data = EventData {
            object: json!({
                "id": "ch_123",
                "amount": 2000,
                "currency": "usd"
            }),
            previous_attributes: Some(json!({
                "amount": 1500
            })),
        };

        let event = Event {
            id: "evt_123".to_string(),
            object: "event".to_string(),
            api_version: Some("2020-08-27".to_string()),
            created: 1234567890,
            data: event_data,
            livemode: false,
            pending_webhooks: Some(0),
            request: None,
            type_field: "charge.updated".to_string(),
        };

        assert_eq!(event.id, "evt_123");
        assert_eq!(event.type_field, "charge.updated");
        assert_eq!(event.created, 1234567890);
        
        // Test extracting object data
        let obj = &event.data.object;
        assert_eq!(obj["id"], "ch_123");
        assert_eq!(obj["amount"], 2000);
        assert_eq!(obj["currency"], "usd");
        
        // Test previous attributes
        let prev = event.data.previous_attributes.unwrap();
        assert_eq!(prev["amount"], 1500);
    }

    #[test]
    fn test_list_events_params() {
        let mut params = ListEventsParams::new();
        params.type_field = Some("charge.succeeded".to_string());
        params.limit = Some(10);
        params.starting_after = Some("evt_123".to_string());

        let query = params.to_query_string();
        assert!(query.contains("type=charge.succeeded"));
        assert!(query.contains("limit=10"));
        assert!(query.contains("starting_after=evt_123"));
    }

    #[test]
    fn test_event_time_filter() {
        // Test exact timestamp
        let filter = EventTimeFilter::Timestamp(1234567890);
        assert_eq!(filter.to_query_string(), "1234567890");

        // Test range filter
        let range_filter = EventTimeFilter::Range {
            gt: Some(1234567890),
            gte: None,
            lt: Some(1234567900),
            lte: None,
        };
        let query = range_filter.to_query_string();
        assert!(query.contains("gt:1234567890"));
        assert!(query.contains("lt:1234567900"));
    }

    #[test]
    fn test_invoice_status_enum() {
        // Test serialization of invoice status
        let status = InvoiceStatus::Paid;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"paid\"");

        let status = InvoiceStatus::Open;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"open\"");
    }

    #[test]
    fn test_collection_method_enum() {
        let method = CollectionMethod::ChargeAutomatically;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"charge_automatically\"");

        let method = CollectionMethod::SendInvoice;
        let serialized = serde_json::to_string(&method).unwrap();
        assert_eq!(serialized, "\"send_invoice\"");
    }

    #[test]
    fn test_plan_intervals() {
        let interval = Interval::Month;
        let serialized = serde_json::to_string(&interval).unwrap();
        assert_eq!(serialized, "\"month\"");

        let interval = Interval::Year;
        let serialized = serde_json::to_string(&interval).unwrap();
        assert_eq!(serialized, "\"year\"");
    }

    #[test]
    fn test_price_type() {
        let price_type = PriceType::Recurring;
        let serialized = serde_json::to_string(&price_type).unwrap();
        assert_eq!(serialized, "\"recurring\"");

        let price_type = PriceType::OneTime;
        let serialized = serde_json::to_string(&price_type).unwrap();
        assert_eq!(serialized, "\"one_time\"");
    }

    #[test]
    fn test_charge_builder_with_shipping() {
        let address = ShippingAddressBuilder::new()
            .line1("456 Oak Ave")
            .city("Los Angeles")
            .state("CA")
            .postal_code("90001")
            .country("US")
            .build();

        let shipping = ShippingBuilder::new()
            .name("Alice Johnson")
            .phone("+13105551234")
            .address(address)
            .carrier("UPS")
            .tracking_number("ABC123")
            .build();

        let charge = ChargeBuilder::new()
            .amount(5000)
            .currency("usd")
            .customer("cust_456")
            .description("Test charge with shipping")
            .shipping(shipping)
            .build();

        assert_eq!(charge.stripe_amount, Some(5000));
        assert_eq!(charge.currency, Some("usd".to_string()));
        assert_eq!(charge.customer, Some("cust_456".to_string()));
        assert!(charge.shipping.is_some());
        
        let shipping = charge.shipping.unwrap();
        assert_eq!(shipping.name, Some("Alice Johnson".to_string()));
        assert_eq!(shipping.carrier, Some("UPS".to_string()));
        assert_eq!(shipping.tracking_number, Some("ABC123".to_string()));
    }

    #[test]
    fn test_payment_intent_builder_with_shipping() {
        use payup::stripe::{ShippingDetails, Address, CaptureMethod};

        let shipping = ShippingDetails {
            address: Some(Address {
                city: Some("Seattle".to_string()),
                country: Some("US".to_string()),
                line1: Some("789 Pine St".to_string()),
                line2: None,
                postal_code: Some("98101".to_string()),
                state: Some("WA".to_string()),
            }),
            carrier: Some("DHL".to_string()),
            name: Some("Bob Wilson".to_string()),
            phone: Some("+12065551234".to_string()),
            tracking_number: Some("XYZ789".to_string()),
        };

        let params = PaymentIntentBuilder::new(10000, "usd")
            .customer("cust_789")
            .description("Payment with shipping")
            .shipping(shipping.clone())
            .capture_method(CaptureMethod::Manual)
            .build();

        assert_eq!(params.amount, 10000);
        assert_eq!(params.currency, "usd");
        assert_eq!(params.customer, Some("cust_789".to_string()));
        assert!(params.shipping.is_some());
        
        let param_shipping = params.shipping.unwrap();
        assert_eq!(param_shipping.name, Some("Bob Wilson".to_string()));
        assert_eq!(param_shipping.carrier, Some("DHL".to_string()));
    }

    #[test]
    fn test_invoice_builder() {
        let params = InvoiceBuilder::new("cust_999")
            .description("Monthly subscription invoice")
            .collection_method(CollectionMethod::SendInvoice)
            .days_until_due(30)
            .footer("Thank you for your business!")
            .build();

        assert_eq!(params.customer, "cust_999");
        assert_eq!(params.description, Some("Monthly subscription invoice".to_string()));
        assert_eq!(params.collection_method, Some(CollectionMethod::SendInvoice));
        assert_eq!(params.days_until_due, Some(30));
        assert_eq!(params.footer, Some("Thank you for your business!".to_string()));
    }

    #[test]
    fn test_plan_builder() {
        let params = PlanBuilder::new("usd", Interval::Month, "prod_abc")
            .amount(1999)
            .nickname("Pro Plan")
            .interval_count(1)
            .trial_period_days(7)
            .usage_type(UsageType::Licensed)
            .build();

        assert_eq!(params.currency, "usd");
        assert_eq!(params.interval, Interval::Month);
        assert_eq!(params.product, "prod_abc");
        assert_eq!(params.amount, Some(1999));
        assert_eq!(params.nickname, Some("Pro Plan".to_string()));
        assert_eq!(params.interval_count, Some(1));
        assert_eq!(params.trial_period_days, Some(7));
        assert_eq!(params.usage_type, Some(UsageType::Licensed));
    }

    #[test]
    fn test_price_builder() {
        use payup::stripe::Recurring;

        let recurring = Recurring {
            interval: Interval::Year,
            interval_count: 1,
            aggregate_usage: None,
            trial_period_days: Some(30),
            usage_type: None,
        };

        let params = PriceBuilder::new("eur", "prod_xyz")
            .unit_amount(9999)
            .nickname("Annual Premium")
            .recurring(recurring)
            .tax_behavior(TaxBehavior::Inclusive)
            .active(true)
            .build();

        assert_eq!(params.currency, "eur");
        assert_eq!(params.product, "prod_xyz");
        assert_eq!(params.unit_amount, Some(9999));
        assert_eq!(params.nickname, Some("Annual Premium".to_string()));
        assert_eq!(params.tax_behavior, Some(TaxBehavior::Inclusive));
        assert_eq!(params.active, Some(true));
        assert!(params.recurring.is_some());
        
        let recurring = params.recurring.unwrap();
        assert_eq!(recurring.interval, Interval::Year);
        assert_eq!(recurring.interval_count, 1);
        assert_eq!(recurring.trial_period_days, Some(30));
    }

    #[test]
    fn test_price_builder_with_simple_recurring() {
        let params = PriceBuilder::new("gbp", "prod_qrs")
            .unit_amount(4999)
            .recurring_simple(Interval::Month, 3)
            .build();

        assert_eq!(params.currency, "gbp");
        assert_eq!(params.product, "prod_qrs");
        assert_eq!(params.unit_amount, Some(4999));
        assert!(params.recurring.is_some());
        
        let recurring = params.recurring.unwrap();
        assert_eq!(recurring.interval, Interval::Month);
        assert_eq!(recurring.interval_count, 3);
    }
}