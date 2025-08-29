use payup::stripe::*;

#[cfg(test)]
mod auth_tests {
    use super::*;

    #[test]
    fn test_auth_new() {
        let auth = Auth::new("client_key".to_string(), "secret_key".to_string());
        assert_eq!(auth.client, "client_key");
        assert_eq!(auth.secret, "secret_key");
    }
    
    #[test]
    fn test_auth_clone() {
        let auth = Auth::new("client_key".to_string(), "secret_key".to_string());
        let auth_clone = auth.clone();
        assert_eq!(auth.client, auth_clone.client);
        assert_eq!(auth.secret, auth_clone.secret);
    }
}

#[cfg(test)]
mod balance_tests {
    use super::*;

    #[test]
    fn test_balance_fields() {
        // Balance doesn't have a new() method, it's retrieved via API
        // This test would verify the structure if we had mock data
        // Skipping for now as Balance::new() doesn't exist
    }
}

#[cfg(test)]
mod customer_tests {
    use super::*;

    #[test]
    fn test_customer_new() {
        let customer = Customer::new();
        assert!(customer.id.is_none());
        assert!(customer.name.is_none());
        assert!(customer.email.is_none());
        assert!(customer.phone.is_none());
    }

    #[test]
    fn test_customer_with_data() {
        let mut customer = Customer::new();
        customer.name = Some("John Doe".to_string());
        customer.email = Some("john@example.com".to_string());
        customer.phone = Some("123-456-7890".to_string());
        
        assert_eq!(customer.name.as_deref(), Some("John Doe"));
        assert_eq!(customer.email.as_deref(), Some("john@example.com"));
        assert_eq!(customer.phone.as_deref(), Some("123-456-7890"));
    }
}

#[cfg(test)]
mod charge_tests {
    use super::*;

    #[test]
    fn test_charge_new() {
        let charge = Charge::new();
        assert!(charge.id.is_none());
        assert!(charge.amount.is_none());
        assert!(charge.currency.is_none());
        assert!(charge.customer.is_none());
    }

    #[test]
    fn test_charge_with_amount() {
        let mut charge = Charge::new();
        charge.amount = Some("1000".to_string());
        charge.currency = Some("usd".to_string());
        
        assert_eq!(charge.amount.as_deref(), Some("1000"));
        assert_eq!(charge.currency.as_deref(), Some("usd"));
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new();
        assert!(card.number.is_none());
        assert!(card.exp_month.is_none());
        assert!(card.exp_year.is_none());
        assert!(card.cvc.is_none());
    }

    #[test]
    fn test_card_with_valid_data() {
        let mut card = Card::new();
        card.number = Some("4242424242424242".to_string());
        card.exp_month = Some("12".to_string());
        card.exp_year = Some("2025".to_string());
        card.cvc = Some("123".to_string());
        
        assert_eq!(card.number.as_deref(), Some("4242424242424242"));
        assert_eq!(card.exp_month.as_deref(), Some("12"));
        assert_eq!(card.exp_year.as_deref(), Some("2025"));
        assert_eq!(card.cvc.as_deref(), Some("123"));
    }
}

#[cfg(test)]
mod payment_method_tests {
    use super::*;

    #[test]
    fn test_payment_method_new() {
        let pm = PaymentMethod::new();
        assert!(pm.id.is_none());
        assert!(pm.method_type.is_none());
        assert!(pm.card.is_none());
    }

    #[test]
    fn test_payment_method_with_card() {
        let mut pm = PaymentMethod::new();
        pm.method_type = Some("card".to_string());
        
        let mut card = Card::new();
        card.number = Some("4242424242424242".to_string());
        pm.card = Some(card);
        
        assert_eq!(pm.method_type.as_deref(), Some("card"));
        assert!(pm.card.is_some());
    }
}

#[cfg(test)]
mod subscription_tests {
    use super::*;

    #[test]
    fn test_subscription_new() {
        let sub = Subscription::new();
        assert!(sub.id.is_none());
        assert!(sub.customer.is_none());
        assert!(sub.status.is_none());
    }

    #[test]
    fn test_subscription_with_customer() {
        let mut sub = Subscription::new();
        sub.customer = Some("cus_123".to_string());
        sub.status = Some("active".to_string());
        
        assert_eq!(sub.customer.as_deref(), Some("cus_123"));
        assert_eq!(sub.status.as_deref(), Some("active"));
    }
}

#[cfg(test)]
mod plan_tests {
    use super::*;

    #[test]
    fn test_plan_new() {
        let plan = Plan::new();
        assert!(plan.id.is_none());
        assert!(plan.amount.is_none());
        assert!(plan.currency.is_none());
        assert!(plan.interval.is_none());
    }

    #[test]
    fn test_plan_with_pricing() {
        let mut plan = Plan::new();
        plan.amount = Some("999".to_string());
        plan.currency = Some("usd".to_string());
        plan.interval = Some("month".to_string());
        
        assert_eq!(plan.amount.as_deref(), Some("999"));
        assert_eq!(plan.currency.as_deref(), Some("usd"));
        assert_eq!(plan.interval.as_deref(), Some("month"));
    }
}

#[cfg(test)]
mod invoice_tests {
    use super::*;

    #[test]
    fn test_invoice_new() {
        let invoice = Invoice::new();
        assert!(invoice.id.is_none());
        assert!(invoice.customer.is_none());
        assert!(invoice.status.is_none());
    }

    #[test]
    fn test_invoice_with_customer() {
        let mut invoice = Invoice::new();
        invoice.customer = Some("cus_456".to_string());
        invoice.status = Some("paid".to_string());
        
        assert_eq!(invoice.customer.as_deref(), Some("cus_456"));
        assert_eq!(invoice.status.as_deref(), Some("paid"));
    }
}

#[cfg(test)]
mod payment_intent_tests {
    use super::*;

    #[test]
    fn test_payment_intent_status_variants() {
        use serde_json;
        
        // Test serialization/deserialization of status variants
        let status = PaymentIntentStatus::RequiresPaymentMethod;
        let serialized = serde_json::to_string(&status).expect("Failed to serialize PaymentIntentStatus");
        assert_eq!(serialized, "\"requires_payment_method\"");
        
        let deserialized: PaymentIntentStatus = serde_json::from_str(&serialized).expect("Failed to deserialize PaymentIntentStatus");
        match deserialized {
            PaymentIntentStatus::RequiresPaymentMethod => {},
            _ => assert!(false, "Expected RequiresPaymentMethod variant, got something else"),
        }
    }

    #[test]
    fn test_create_payment_intent_params() {
        let params = CreatePaymentIntentParams {
            amount: 2000,
            currency: "usd".to_string(),
            automatic_payment_methods: Some(AutomaticPaymentMethods {
                enabled: true,
                allow_redirects: None,
            }),
            capture_method: Some(CaptureMethod::Automatic),
            confirmation_method: Some(ConfirmationMethod::Automatic),
            customer: Some("cus_123".to_string()),
            description: Some("Test payment".to_string()),
            metadata: None,
            payment_method: None,
            payment_method_options: None,
            payment_method_types: Some(vec!["card".to_string()]),
            receipt_email: Some("test@example.com".to_string()),
            setup_future_usage: None,
            shipping: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
            transfer_data: None,
            transfer_group: None,
        };
        
        assert_eq!(params.amount, 2000);
        assert_eq!(params.currency, "usd");
        assert_eq!(params.customer.as_deref(), Some("cus_123"));
        assert_eq!(params.description.as_deref(), Some("Test payment"));
        assert_eq!(params.receipt_email.as_deref(), Some("test@example.com"));
        assert!(params.automatic_payment_methods.is_some());
        assert!(params.capture_method.is_some());
        assert!(params.confirmation_method.is_some());
    }

    #[test]
    fn test_update_payment_intent_params_default() {
        let params = UpdatePaymentIntentParams::default();
        
        assert!(params.amount.is_none());
        assert!(params.currency.is_none());
        assert!(params.customer.is_none());
        assert!(params.description.is_none());
        assert!(params.metadata.is_none());
        assert!(params.payment_method.is_none());
        assert!(params.payment_method_options.is_none());
        assert!(params.payment_method_types.is_none());
        assert!(params.receipt_email.is_none());
        assert!(params.setup_future_usage.is_none());
        assert!(params.shipping.is_none());
        assert!(params.statement_descriptor.is_none());
        assert!(params.statement_descriptor_suffix.is_none());
        assert!(params.transfer_data.is_none());
        assert!(params.transfer_group.is_none());
    }

    #[test]
    fn test_confirm_payment_intent_params() {
        let params = ConfirmPaymentIntentParams {
            payment_method: Some("pm_123".to_string()),
            payment_method_options: None,
            return_url: Some("https://example.com/return".to_string()),
            setup_future_usage: Some(SetupFutureUsage::OffSession),
            shipping: None,
        };
        
        assert_eq!(params.payment_method.as_deref(), Some("pm_123"));
        assert_eq!(params.return_url.as_deref(), Some("https://example.com/return"));
        assert!(params.setup_future_usage.is_some());
    }

    #[test]
    fn test_capture_payment_intent_params() {
        let params = CapturePaymentIntentParams {
            amount_to_capture: Some(1500),
            application_fee_amount: Some(100),
            statement_descriptor: Some("CAPTURE".to_string()),
            statement_descriptor_suffix: Some("ORDER".to_string()),
            transfer_data: None,
        };
        
        assert_eq!(params.amount_to_capture, Some(1500));
        assert_eq!(params.application_fee_amount, Some(100));
        assert_eq!(params.statement_descriptor.as_deref(), Some("CAPTURE"));
        assert_eq!(params.statement_descriptor_suffix.as_deref(), Some("ORDER"));
    }

    #[test]
    fn test_cancel_payment_intent_params() {
        let params = CancelPaymentIntentParams {
            cancellation_reason: Some("requested_by_customer".to_string()),
        };
        
        assert_eq!(params.cancellation_reason.as_deref(), Some("requested_by_customer"));
    }

    #[test]
    fn test_address_struct() {
        let address = PaymentIntentAddress {
            city: Some("San Francisco".to_string()),
            country: Some("US".to_string()),
            line1: Some("123 Main St".to_string()),
            line2: Some("Apt 1".to_string()),
            postal_code: Some("94102".to_string()),
            state: Some("CA".to_string()),
        };
        
        assert_eq!(address.city.as_deref(), Some("San Francisco"));
        assert_eq!(address.country.as_deref(), Some("US"));
        assert_eq!(address.line1.as_deref(), Some("123 Main St"));
        assert_eq!(address.line2.as_deref(), Some("Apt 1"));
        assert_eq!(address.postal_code.as_deref(), Some("94102"));
        assert_eq!(address.state.as_deref(), Some("CA"));
    }

    #[test]
    fn test_shipping_details_struct() {
        let shipping = ShippingDetails {
            address: Some(PaymentIntentAddress {
                city: Some("San Francisco".to_string()),
                country: Some("US".to_string()),
                line1: Some("123 Main St".to_string()),
                line2: None,
                postal_code: Some("94102".to_string()),
                state: Some("CA".to_string()),
            }),
            carrier: Some("FedEx".to_string()),
            name: Some("John Doe".to_string()),
            phone: Some("+1-555-123-4567".to_string()),
            tracking_number: Some("123456789".to_string()),
        };
        
        assert!(shipping.address.is_some());
        assert_eq!(shipping.carrier.as_deref(), Some("FedEx"));
        assert_eq!(shipping.name.as_deref(), Some("John Doe"));
        assert_eq!(shipping.phone.as_deref(), Some("+1-555-123-4567"));
        assert_eq!(shipping.tracking_number.as_deref(), Some("123456789"));
    }

    #[test]
    fn test_transfer_data_struct() {
        let transfer_data = TransferData {
            amount: Some(500),
            destination: "acct_123".to_string(),
        };
        
        assert_eq!(transfer_data.amount, Some(500));
        assert_eq!(transfer_data.destination, "acct_123");
    }
}