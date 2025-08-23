use async_trait::async_trait;
use std::collections::HashMap;
use crate::error::{PayupError, Result};
use crate::payment_provider::{
    PaymentProvider, PaymentFeature, Customer as UnifiedCustomer, PaymentMethod as UnifiedPaymentMethod,
    PaymentMethodType, CardDetails, BankAccountDetails, Charge as UnifiedCharge, ChargeStatus,
    Refund as UnifiedRefund, RefundStatus, RefundReason, Subscription as UnifiedSubscription,
    SubscriptionStatus, Money
};
use super::{
    Auth, Customer, Customers, Charge, PaymentIntent, PaymentIntentStatus,
    PaymentMethod, StripePaymentMethodType, CreatePaymentMethodParams, CreateCardParams,
    PaymentMethodBillingDetails, StripeCardDetails, PaymentMethodAddress,
    Subscription, CreatePaymentIntentParams, UpdatePaymentIntentParams,
    ConfirmPaymentIntentParams, CapturePaymentIntentParams
};
use crate::stripe_ext::refund::Refund as StripeRefund;
use reqwest::Client;
use serde_json::json;

pub struct StripeProvider {
    auth: Auth,
    client: Client,
}

impl StripeProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            auth: Auth::new(api_key.clone(), api_key),
            client: Client::new(),
        }
    }

    pub fn with_client(api_key: String, client: Client) -> Self {
        Self {
            auth: Auth::new(api_key.clone(), api_key),
            client,
        }
    }

    fn map_customer_to_unified(&self, customer: &Customer) -> UnifiedCustomer {
        UnifiedCustomer {
            id: customer.id.clone(),
            email: customer.email.clone(),
            name: customer.name.clone(),
            phone: customer.phone.clone(),
            metadata: None, // TODO: Map metadata when Stripe customer metadata is available
        }
    }

    fn map_unified_to_customer(&self, customer: &UnifiedCustomer) -> Customer {
        Customer {
            id: customer.id.clone(),
            object: Some("customer".to_string()),
            balance: None,
            created: None,
            currency: None,
            default_source: None,
            payment_method: None,
            delinquent: None,
            description: None,
            email: customer.email.clone(),
            invoice_prefix: None,
            livemode: None,
            name: customer.name.clone(),
            next_invoice_sequence: None,
            phone: customer.phone.clone(),
            tax_exempt: None,
        }
    }

    fn map_payment_method_to_unified(&self, pm: &PaymentMethod) -> UnifiedPaymentMethod {
        let method_type = match &pm.payment_method_type {
            Some(StripePaymentMethodType::Card) => PaymentMethodType::Card,
            Some(StripePaymentMethodType::SepaDebit) => PaymentMethodType::BankAccount,
            Some(StripePaymentMethodType::UsBankAccount) => PaymentMethodType::BankAccount,
            Some(StripePaymentMethodType::Paypal) => PaymentMethodType::PayPal,
            _ => PaymentMethodType::Other("stripe".to_string()),
        };

        let card = pm.card.as_ref().map(|c| CardDetails {
            number: None, // Not returned by Stripe for security
            exp_month: c.exp_month.map(|m| m.to_string()).unwrap_or_default(),
            exp_year: c.exp_year.map(|y| y.to_string()).unwrap_or_default(),
            cvv: None, // Not returned by Stripe for security
            brand: c.brand.clone(),
            last4: c.last4.clone(),
        });

        UnifiedPaymentMethod {
            id: pm.id.clone(),
            method_type,
            card,
            bank_account: None, // Could be extended for SEPA/US Bank Account details
        }
    }

    fn map_charge_status(&self, status: &str) -> ChargeStatus {
        match status {
            "pending" => ChargeStatus::Pending,
            "processing" => ChargeStatus::Processing,
            "succeeded" => ChargeStatus::Succeeded,
            "failed" => ChargeStatus::Failed,
            "canceled" => ChargeStatus::Canceled,
            "requires_action" | "requires_payment_method" => ChargeStatus::RequiresAction,
            _ => ChargeStatus::Pending,
        }
    }

    fn map_payment_intent_to_charge(&self, pi: &PaymentIntent) -> UnifiedCharge {
        let status = match pi.status {
            PaymentIntentStatus::RequiresPaymentMethod => ChargeStatus::RequiresAction,
            PaymentIntentStatus::RequiresConfirmation => ChargeStatus::RequiresAction,
            PaymentIntentStatus::RequiresAction => ChargeStatus::RequiresAction,
            PaymentIntentStatus::Processing => ChargeStatus::Processing,
            PaymentIntentStatus::RequiresCapture => ChargeStatus::Processing,
            PaymentIntentStatus::Canceled => ChargeStatus::Canceled,
            PaymentIntentStatus::Succeeded => ChargeStatus::Succeeded,
        };

        UnifiedCharge {
            id: pi.id.clone(),
            amount: Money {
                amount: pi.amount,
                currency: pi.currency.clone(),
            },
            customer_id: pi.customer.clone(),
            payment_method_id: pi.payment_method.clone(),
            status,
            description: pi.description.clone(),
            metadata: pi.metadata.clone(),
            created_at: Some(pi.created),
        }
    }

    fn map_charge_to_unified(&self, charge: &Charge) -> UnifiedCharge {
        let status = if charge.paid.unwrap_or(false) {
            if charge.captured.unwrap_or(false) {
                ChargeStatus::Succeeded
            } else {
                ChargeStatus::Processing
            }
        } else if charge.failure_code.is_some() {
            ChargeStatus::Failed
        } else {
            ChargeStatus::Pending
        };

        UnifiedCharge {
            id: charge.id.clone(),
            amount: Money {
                amount: charge.amount.unwrap_or(0),
                currency: charge.currency.clone().unwrap_or_else(|| "usd".to_string()),
            },
            customer_id: charge.customer.clone(),
            payment_method_id: charge.payment_method.clone(),
            status,
            description: charge.description.clone(),
            metadata: charge.metadata.clone(),
            created_at: charge.created.map(|c| c as i64),
        }
    }

    fn map_refund_to_unified(&self, refund: &StripeRefund) -> UnifiedRefund {
        let status = match refund.status.as_ref() {
            Some(crate::stripe_ext::refund::RefundStatus::Pending) => RefundStatus::Pending,
            Some(crate::stripe_ext::refund::RefundStatus::Succeeded) => RefundStatus::Succeeded,
            Some(crate::stripe_ext::refund::RefundStatus::Failed) => RefundStatus::Failed,
            Some(crate::stripe_ext::refund::RefundStatus::Canceled) => RefundStatus::Canceled,
            _ => RefundStatus::Pending,
        };

        let reason = refund.reason.as_ref().map(|r| {
            match r {
                crate::stripe_ext::refund::RefundReason::Duplicate => RefundReason::Duplicate,
                crate::stripe_ext::refund::RefundReason::Fraudulent => RefundReason::Fraudulent,
                crate::stripe_ext::refund::RefundReason::RequestedByCustomer => RefundReason::RequestedByCustomer,
                _ => RefundReason::Other("other".to_string()),
            }
        });

        UnifiedRefund {
            id: refund.id.clone(),
            charge_id: refund.charge.clone().unwrap_or_default(),
            amount: Some(Money {
                amount: refund.amount.unwrap_or(0),
                currency: refund.currency.clone().unwrap_or_else(|| "usd".to_string()),
            }),
            reason,
            status,
            metadata: refund.metadata.clone(),
        }
    }

    fn map_subscription_status(&self, status: &str) -> SubscriptionStatus {
        match status {
            "active" => SubscriptionStatus::Active,
            "past_due" => SubscriptionStatus::PastDue,
            "canceled" => SubscriptionStatus::Canceled,
            "incomplete" => SubscriptionStatus::Incomplete,
            "incomplete_expired" => SubscriptionStatus::IncompleteExpired,
            "trialing" => SubscriptionStatus::Trialing,
            "unpaid" => SubscriptionStatus::Unpaid,
            _ => SubscriptionStatus::Incomplete,
        }
    }

    fn map_subscription_to_unified(&self, sub: &Subscription) -> UnifiedSubscription {
        let status = sub.status.as_ref()
            .map(|s| self.map_subscription_status(s))
            .unwrap_or(SubscriptionStatus::Incomplete);

        UnifiedSubscription {
            id: sub.id.clone(),
            customer_id: sub.customer.clone().unwrap_or_default(),
            plan_id: None, // Stripe uses price_id instead
            price_id: sub.plan.as_ref().and_then(|p| p.id.clone()),
            status,
            current_period_start: sub.current_period_start,
            current_period_end: sub.current_period_end,
            cancel_at_period_end: sub.cancel_at_period_end.unwrap_or(false),
        }
    }

    async fn stripe_error_to_payup(&self, error: reqwest::Error) -> PayupError {
        if error.is_timeout() {
            return PayupError::TimeoutError("Stripe API request timed out".to_string());
        }
        
        if let Some(status) = error.status() {
            if status.as_u16() == 429 {
                return PayupError::RateLimitError { retry_after: None };
            }
            if status.is_server_error() {
                return PayupError::ServerError(status.as_u16());
            }
        }

        PayupError::NetworkError(error)
    }
}

#[async_trait]
impl PaymentProvider for StripeProvider {
    fn name(&self) -> &str {
        "stripe"
    }

    fn supported_currencies(&self) -> Vec<String> {
        vec![
            "usd", "eur", "gbp", "jpy", "cad", "aud", "chf", "cny", "hkd", "nzd",
            "sek", "krw", "sgd", "nok", "mxn", "inr", "rub", "zar", "try", "brl",
            "twd", "dkk", "pln", "thb", "idr", "huf", "czk", "ils", "clp", "php",
            "aed", "cop", "sar", "myr", "ron"
        ].into_iter().map(String::from).collect()
    }

    fn supported_features(&self) -> Vec<PaymentFeature> {
        vec![
            PaymentFeature::OneTimePayments,
            PaymentFeature::RecurringPayments,
            PaymentFeature::Refunds,
            PaymentFeature::PartialRefunds,
            PaymentFeature::PaymentMethods,
            PaymentFeature::Webhooks,
            PaymentFeature::Subscriptions,
            PaymentFeature::CustomerManagement,
            PaymentFeature::DisputeManagement,
            PaymentFeature::Invoicing,
        ]
    }

    async fn create_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        let stripe_customer = self.map_unified_to_customer(customer);
        let created = stripe_customer.async_post(self.auth.clone()).await?;
        Ok(self.map_customer_to_unified(&created))
    }

    async fn get_customer(&self, customer_id: &str) -> Result<UnifiedCustomer> {
        let customer = Customer::async_get(self.auth.clone(), customer_id.to_string()).await?;
        Ok(self.map_customer_to_unified(&customer))
    }

    async fn update_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        let stripe_customer = self.map_unified_to_customer(customer);
        let updated = stripe_customer.async_post(self.auth.clone()).await?;
        Ok(self.map_customer_to_unified(&updated))
    }

    async fn delete_customer(&self, customer_id: &str) -> Result<bool> {
        Customer::async_delete(self.auth.clone(), customer_id.to_string()).await?;
        Ok(true)
    }

    async fn list_customers(&self, limit: Option<u32>, _offset: Option<u32>) -> Result<Vec<UnifiedCustomer>> {
        let customers = Customer::async_list(self.auth.clone()).await?;
        let limited = if let Some(l) = limit {
            customers.into_iter().take(l as usize).collect()
        } else {
            customers
        };
        Ok(limited
            .iter()
            .map(|c| self.map_customer_to_unified(c))
            .collect())
    }

    async fn create_payment_method(&self, payment_method: &UnifiedPaymentMethod) -> Result<UnifiedPaymentMethod> {
        // Map unified payment method to Stripe payment method type
        let payment_method_type = match &payment_method.method_type {
            PaymentMethodType::Card => StripePaymentMethodType::Card,
            PaymentMethodType::BankAccount => StripePaymentMethodType::SepaDebit,
            PaymentMethodType::PayPal => StripePaymentMethodType::Paypal,
            PaymentMethodType::ApplePay => StripePaymentMethodType::Card, // Apple Pay uses card
            PaymentMethodType::GooglePay => StripePaymentMethodType::Card, // Google Pay uses card
            _ => return Err(PayupError::UnsupportedOperation(
                format!("Payment method type {:?} not supported by Stripe", payment_method.method_type)
            )),
        };

        // Build parameters based on payment method type
        let mut params = CreatePaymentMethodParams {
            payment_method_type,
            billing_details: None,
            card: None,
            metadata: None,
        };

        // Add card details if it's a card payment method
        if let Some(card) = &payment_method.card {
            if let (Some(number), exp_month, exp_year) = (
                card.number.as_ref(),
                card.exp_month.parse::<i32>().ok(),
                card.exp_year.parse::<i32>().ok(),
            ) {
                params.card = Some(CreateCardParams {
                    number: number.clone(),
                    exp_month: exp_month.unwrap_or(1),
                    exp_year: exp_year.unwrap_or(2025),
                    cvc: card.cvv.clone(),
                });
            }
        }

        // Create the payment method using Stripe API
        let created = PaymentMethod::create_async(&self.auth, params).await?;
        Ok(self.map_payment_method_to_unified(&created))
    }

    async fn get_payment_method(&self, payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        let payment_method = PaymentMethod::retrieve_async(&self.auth, payment_method_id).await?;
        Ok(self.map_payment_method_to_unified(&payment_method))
    }

    async fn attach_payment_method(&self, payment_method_id: &str, customer_id: &str) -> Result<UnifiedPaymentMethod> {
        let attached = PaymentMethod::attach_async(&self.auth, payment_method_id, customer_id).await?;
        Ok(self.map_payment_method_to_unified(&attached))
    }

    async fn detach_payment_method(&self, payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        let detached = PaymentMethod::detach_async(&self.auth, payment_method_id).await?;
        Ok(self.map_payment_method_to_unified(&detached))
    }

    async fn create_charge(&self, charge: &UnifiedCharge) -> Result<UnifiedCharge> {
        // Use PaymentIntent for creating charges (modern Stripe approach)
        let params = CreatePaymentIntentParams {
            amount: charge.amount.amount,
            currency: charge.amount.currency.clone(),
            customer: charge.customer_id.clone(),
            payment_method: charge.payment_method_id.clone(),
            description: charge.description.clone(),
            metadata: charge.metadata.clone(),
            confirm: Some(true),
            ..Default::default()
        };

        let pi = PaymentIntent::create_async(&self.auth, params).await?;
        Ok(self.map_payment_intent_to_charge(&pi))
    }

    async fn get_charge(&self, charge_id: &str) -> Result<UnifiedCharge> {
        // Try to get as PaymentIntent first
        if charge_id.starts_with("pi_") {
            let pi = PaymentIntent::retrieve_async(&self.auth, charge_id).await?;
            Ok(self.map_payment_intent_to_charge(&pi))
        } else {
            // Fall back to legacy Charge API - need to check its async methods
            let charge = Charge::async_get(self.auth.clone(), charge_id.to_string()).await?;
            Ok(self.map_charge_to_unified(&charge))
        }
    }

    async fn capture_charge(&self, charge_id: &str, amount: Option<Money>) -> Result<UnifiedCharge> {
        if charge_id.starts_with("pi_") {
            let params = CapturePaymentIntentParams {
                amount_to_capture: amount.map(|m| m.amount),
                ..Default::default()
            };
            let pi = PaymentIntent::capture_async(&self.auth, charge_id, params).await?;
            Ok(self.map_payment_intent_to_charge(&pi))
        } else {
            // Fall back to legacy Charge API - need to get charge first then capture
            let mut charge = Charge::async_get(self.auth.clone(), charge_id.to_string()).await?;
            if let Some(amt) = amount {
                charge.amount = Some(amt.amount);
            }
            let captured = charge.async_capture(self.auth.clone()).await?;
            Ok(self.map_charge_to_unified(&captured))
        }
    }

    async fn list_charges(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedCharge>> {
        // The async_list method doesn't support filtering by customer_id or limit
        // This would need to be implemented with proper query parameters
        let charges = Charge::async_list(self.auth.clone()).await?;
        Ok(charges
            .iter()
            .map(|c| self.map_charge_to_unified(c))
            .collect())
    }

    async fn create_refund(&self, refund: &UnifiedRefund) -> Result<UnifiedRefund> {
        let mut stripe_refund = StripeRefund::new();
        stripe_refund.charge = Some(refund.charge_id.clone());
        stripe_refund.amount = refund.amount.as_ref().map(|m| m.amount);
        stripe_refund.reason = refund.reason.as_ref().map(|r| match r {
            RefundReason::Duplicate => crate::stripe_ext::refund::RefundReason::Duplicate,
            RefundReason::Fraudulent => crate::stripe_ext::refund::RefundReason::Fraudulent,
            RefundReason::RequestedByCustomer => crate::stripe_ext::refund::RefundReason::RequestedByCustomer,
            RefundReason::Other(_) => crate::stripe_ext::refund::RefundReason::RequestedByCustomer,
        });
        stripe_refund.metadata = refund.metadata.clone();
        
        let created = stripe_refund.async_post(self.auth.clone()).await?;
        Ok(self.map_refund_to_unified(&created))
    }

    async fn get_refund(&self, refund_id: &str) -> Result<UnifiedRefund> {
        let refund = StripeRefund::async_get(self.auth.clone(), refund_id.to_string()).await?;
        Ok(self.map_refund_to_unified(&refund))
    }

    async fn list_refunds(&self, charge_id: Option<&str>, limit: Option<u32>) -> Result<Vec<UnifiedRefund>> {
        let refunds = StripeRefund::async_list(
            self.auth.clone(),
            charge_id.map(String::from),
            limit.map(|l| l as i32)
        ).await?;
        Ok(refunds.data
            .iter()
            .map(|r| self.map_refund_to_unified(r))
            .collect())
    }

    async fn create_subscription(&self, _subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        // TODO: Implement proper Subscription creation using Stripe API
        Err(PayupError::UnsupportedOperation(
            "Subscription creation not yet implemented for Stripe provider".to_string()
        ))
    }

    async fn get_subscription(&self, _subscription_id: &str) -> Result<UnifiedSubscription> {
        // TODO: Implement proper Subscription retrieval using Stripe API
        Err(PayupError::UnsupportedOperation(
            "Subscription retrieval not yet implemented for Stripe provider".to_string()
        ))
    }

    async fn update_subscription(&self, _subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        // TODO: Implement proper Subscription update using Stripe API
        Err(PayupError::UnsupportedOperation(
            "Subscription update not yet implemented for Stripe provider".to_string()
        ))
    }

    async fn cancel_subscription(&self, _subscription_id: &str, _at_period_end: bool) -> Result<UnifiedSubscription> {
        // TODO: Implement proper Subscription cancellation using Stripe API
        Err(PayupError::UnsupportedOperation(
            "Subscription cancellation not yet implemented for Stripe provider".to_string()
        ))
    }

    async fn list_subscriptions(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedSubscription>> {
        // TODO: Implement proper Subscription listing using Stripe API
        Err(PayupError::UnsupportedOperation(
            "Subscription listing not yet implemented for Stripe provider".to_string()
        ))
    }

    async fn verify_webhook(&self, payload: &[u8], signature: &str, secret: &str) -> Result<bool> {
        use super::webhooks::StripeWebhookHandler;
        
        let handler = StripeWebhookHandler::new(secret.to_string());
        match handler.verify_signature(std::str::from_utf8(payload).map_err(|e| 
            PayupError::ValidationError(format!("Invalid UTF-8 in webhook payload: {}", e))
        )?, signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}