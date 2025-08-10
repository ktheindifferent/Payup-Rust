use async_trait::async_trait;
use std::collections::HashMap;
use crate::error::{PayupError, Result};
use crate::payment_provider::{
    PaymentProvider, PaymentFeature, Customer as UnifiedCustomer, PaymentMethod as UnifiedPaymentMethod,
    PaymentMethodType, CardDetails, BankAccountDetails, Charge as UnifiedCharge, ChargeStatus,
    Refund as UnifiedRefund, RefundStatus, RefundReason, Subscription as UnifiedSubscription,
    SubscriptionStatus, Money as UnifiedMoney
};
use super::{
    SquareClient, SquareAuth, SquareConfig, Environment, Money, Address,
    payments::{Payment, CreatePaymentRequest, Refund as SquareRefund},
    customers::{Customer, CreateCustomerRequest, UpdateCustomerRequest},
};

pub struct SquareProvider {
    client: std::sync::Arc<tokio::sync::Mutex<SquareClient>>,
    environment: Environment,
}

impl SquareProvider {
    pub fn new(access_token: String, environment: Environment) -> Result<Self> {
        let config = SquareConfig {
            access_token,
            environment: environment.clone(),
            location_id: None,
        };
        let client = SquareClient::new(config)?;
        
        Ok(Self {
            client: std::sync::Arc::new(tokio::sync::Mutex::new(client)),
            environment,
        })
    }

    fn map_customer(&self, customer: &Customer) -> UnifiedCustomer {
        UnifiedCustomer {
            id: customer.id.clone(),
            email: customer.email_address.clone(),
            name: match (&customer.given_name, &customer.family_name) {
                (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
                (Some(first), None) => Some(first.clone()),
                (None, Some(last)) => Some(last.clone()),
                _ => None,
            },
            phone: customer.phone_number.clone(),
            metadata: None,
        }
    }

    fn map_money(&self, money: &UnifiedMoney) -> Money {
        Money {
            amount: money.amount,
            currency: money.currency.to_uppercase(),
        }
    }

    fn map_square_money(&self, money: &Money) -> UnifiedMoney {
        UnifiedMoney {
            amount: money.amount,
            currency: money.currency.to_lowercase(),
        }
    }

    fn map_payment_status(&self, status: &str) -> ChargeStatus {
        match status {
            "PENDING" => ChargeStatus::Pending,
            "COMPLETED" => ChargeStatus::Succeeded,
            "CANCELED" => ChargeStatus::Canceled,
            "FAILED" => ChargeStatus::Failed,
            _ => ChargeStatus::Pending,
        }
    }

    fn map_payment(&self, payment: &Payment) -> UnifiedCharge {
        UnifiedCharge {
            id: payment.id.clone(),
            amount: payment.amount_money.as_ref()
                .map(|m| self.map_square_money(m))
                .unwrap_or_else(|| UnifiedMoney { amount: 0, currency: "usd".to_string() }),
            customer_id: payment.customer_id.clone(),
            payment_method_id: None, // Square doesn't expose payment method IDs the same way
            status: payment.status.as_ref()
                .map(|s| self.map_payment_status(s.as_str()))
                .unwrap_or(ChargeStatus::Pending),
            description: payment.note.clone(),
            metadata: None,
            created_at: payment.created_at.as_ref()
                .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                .map(|dt| dt.timestamp()),
        }
    }
}

#[async_trait]
impl PaymentProvider for SquareProvider {
    fn name(&self) -> &str {
        "square"
    }

    fn supported_currencies(&self) -> Vec<String> {
        vec![
            "usd", "cad", "gbp", "jpy", "eur", "aud"
        ].into_iter().map(String::from).collect()
    }

    fn supported_features(&self) -> Vec<PaymentFeature> {
        vec![
            PaymentFeature::OneTimePayments,
            PaymentFeature::Refunds,
            PaymentFeature::PartialRefunds,
            PaymentFeature::PaymentMethods,
            PaymentFeature::Webhooks,
            PaymentFeature::CustomerManagement,
        ]
    }

    async fn create_customer(&self, _customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        // TODO: Implement when SquareClient has customer methods
        Err(PayupError::UnsupportedOperation(
            "Square customer creation not yet implemented.".to_string()
        ))
    }

    async fn get_customer(&self, _customer_id: &str) -> Result<UnifiedCustomer> {
        // TODO: Implement when SquareClient has customer methods
        Err(PayupError::UnsupportedOperation(
            "Square customer retrieval not yet implemented.".to_string()
        ))
    }

    async fn update_customer(&self, _customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        // TODO: Implement when SquareClient has customer methods
        Err(PayupError::UnsupportedOperation(
            "Square customer update not yet implemented.".to_string()
        ))
    }

    async fn delete_customer(&self, _customer_id: &str) -> Result<bool> {
        // TODO: Implement when SquareClient has customer methods
        Err(PayupError::UnsupportedOperation(
            "Square customer deletion not yet implemented.".to_string()
        ))
    }

    async fn list_customers(&self, _limit: Option<u32>, _offset: Option<u32>) -> Result<Vec<UnifiedCustomer>> {
        // TODO: Implement when SquareClient has customer methods
        Err(PayupError::UnsupportedOperation(
            "Square customer listing not yet implemented.".to_string()
        ))
    }

    async fn create_payment_method(&self, _payment_method: &UnifiedPaymentMethod) -> Result<UnifiedPaymentMethod> {
        // Square handles payment methods differently through their checkout flow
        Err(PayupError::UnsupportedOperation(
            "Square handles payment methods through their checkout flow, not via API.".to_string()
        ))
    }

    async fn get_payment_method(&self, _payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "Square doesn't expose payment method details via API.".to_string()
        ))
    }

    async fn attach_payment_method(&self, _payment_method_id: &str, _customer_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "Square handles payment method attachment through cards on file.".to_string()
        ))
    }

    async fn detach_payment_method(&self, _payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "Square handles payment method detachment through cards on file.".to_string()
        ))
    }

    async fn create_charge(&self, _charge: &UnifiedCharge) -> Result<UnifiedCharge> {
        // TODO: Implement when SquareClient has payment methods
        Err(PayupError::UnsupportedOperation(
            "Square payment creation not yet implemented.".to_string()
        ))
    }

    async fn get_charge(&self, _charge_id: &str) -> Result<UnifiedCharge> {
        // TODO: Implement when SquareClient has payment methods
        Err(PayupError::UnsupportedOperation(
            "Square payment retrieval not yet implemented.".to_string()
        ))
    }

    async fn capture_charge(&self, _charge_id: &str, _amount: Option<UnifiedMoney>) -> Result<UnifiedCharge> {
        // Square captures payments immediately by default
        Err(PayupError::UnsupportedOperation(
            "Square captures payments immediately. Use delayed capture for manual capture.".to_string()
        ))
    }

    async fn list_charges(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedCharge>> {
        // TODO: Implement when SquareClient has payment methods
        Err(PayupError::UnsupportedOperation(
            "Square payment listing not yet implemented.".to_string()
        ))
    }

    async fn create_refund(&self, _refund: &UnifiedRefund) -> Result<UnifiedRefund> {
        // TODO: Implement when SquareClient has refund methods
        Err(PayupError::UnsupportedOperation(
            "Square refund creation not yet implemented.".to_string()
        ))
    }

    async fn get_refund(&self, _refund_id: &str) -> Result<UnifiedRefund> {
        // TODO: Implement when SquareClient has refund methods
        Err(PayupError::UnsupportedOperation(
            "Square refund retrieval not yet implemented.".to_string()
        ))
    }

    async fn list_refunds(&self, _charge_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedRefund>> {
        // Square doesn't provide a direct endpoint to list refunds by payment
        Err(PayupError::UnsupportedOperation(
            "Square doesn't provide a direct API to list refunds by payment.".to_string()
        ))
    }

    async fn create_subscription(&self, _subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        // Square subscriptions are handled through their separate subscriptions API
        Err(PayupError::UnsupportedOperation(
            "Square subscriptions require using their Subscriptions API.".to_string()
        ))
    }

    async fn get_subscription(&self, _subscription_id: &str) -> Result<UnifiedSubscription> {
        Err(PayupError::UnsupportedOperation(
            "Square subscriptions require using their Subscriptions API.".to_string()
        ))
    }

    async fn update_subscription(&self, _subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        Err(PayupError::UnsupportedOperation(
            "Square subscriptions require using their Subscriptions API.".to_string()
        ))
    }

    async fn cancel_subscription(&self, _subscription_id: &str, _at_period_end: bool) -> Result<UnifiedSubscription> {
        Err(PayupError::UnsupportedOperation(
            "Square subscriptions require using their Subscriptions API.".to_string()
        ))
    }

    async fn list_subscriptions(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedSubscription>> {
        Err(PayupError::UnsupportedOperation(
            "Square subscriptions require using their Subscriptions API.".to_string()
        ))
    }

    async fn verify_webhook(&self, _payload: &[u8], _signature: &str, _secret: &str) -> Result<bool> {
        // Square webhook verification would go here
        // For now, return unsupported
        Err(PayupError::UnsupportedOperation(
            "Square webhook verification not yet implemented.".to_string()
        ))
    }
}