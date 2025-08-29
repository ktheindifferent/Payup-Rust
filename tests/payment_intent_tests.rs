use payup::stripe::{
    PaymentIntent, PaymentIntentStatus, ConfirmationMethod, CaptureMethod, SetupFutureUsage,
    CreatePaymentIntentParams, UpdatePaymentIntentParams, ConfirmPaymentIntentParams,
    CapturePaymentIntentParams, CancelPaymentIntentParams, AutomaticPaymentMethods,
    ShippingDetails, PaymentIntentAddress, TransferData, Auth,
};

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

    #[test] 
    fn test_confirmation_method_serialization() {
        use serde_json;
        
        let method = ConfirmationMethod::Automatic;
        let serialized = serde_json::to_string(&method).expect("Failed to serialize ConfirmationMethod");
        assert_eq!(serialized, "\"automatic\"");
        
        let method = ConfirmationMethod::Manual;
        let serialized = serde_json::to_string(&method).expect("Failed to serialize ConfirmationMethod");
        assert_eq!(serialized, "\"manual\"");
    }

    #[test]
    fn test_capture_method_serialization() {
        use serde_json;
        
        let method = CaptureMethod::Automatic;
        let serialized = serde_json::to_string(&method).expect("Failed to serialize ConfirmationMethod");
        assert_eq!(serialized, "\"automatic\"");
        
        let method = CaptureMethod::Manual;
        let serialized = serde_json::to_string(&method).expect("Failed to serialize ConfirmationMethod");
        assert_eq!(serialized, "\"manual\"");
    }

    #[test]
    fn test_setup_future_usage_serialization() {
        use serde_json;
        
        let usage = SetupFutureUsage::OnSession;
        let serialized = serde_json::to_string(&usage).expect("Failed to serialize SetupFutureUsage");
        assert_eq!(serialized, "\"on_session\"");
        
        let usage = SetupFutureUsage::OffSession;
        let serialized = serde_json::to_string(&usage).expect("Failed to serialize SetupFutureUsage");
        assert_eq!(serialized, "\"off_session\"");
    }

    #[test]
    fn test_automatic_payment_methods_struct() {
        let apm = AutomaticPaymentMethods {
            enabled: true,
            allow_redirects: Some("never".to_string()),
        };
        
        assert!(apm.enabled);
        assert_eq!(apm.allow_redirects.as_deref(), Some("never"));
    }

    #[test]
    fn test_auth_struct() {
        let auth = Auth::new("pk_test_123".to_string(), "sk_test_123".to_string());
        assert_eq!(auth.client, "pk_test_123");
        assert_eq!(auth.secret, "sk_test_123");
    }
}