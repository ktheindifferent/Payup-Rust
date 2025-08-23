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
    PayPalClient, PayPalConfig, PayPalEnvironment, PayPalAuth, PayPalMoney,
    PayPalPayer, PayPalName, PayPalAddress, PayPalPhone, PayPalPhoneNumber, PayPalTaxInfo,
    orders::{Order, OrderIntent, OrderStatus, PurchaseUnit, CaptureRequest, CaptureResponse},
    payments::{Payment, PaymentStatus, Refund as PayPalRefund, RefundStatus as PayPalRefundStatus, RefundRequest},
    subscriptions::{self, Subscription as PayPalSubscription, SubscriptionStatus as PayPalSubscriptionStatus, Plan}
};

pub struct PayPalProvider {
    client: std::sync::Arc<tokio::sync::Mutex<PayPalClient>>,
    environment: PayPalEnvironment,
}

impl PayPalProvider {
    pub fn new(client_id: String, client_secret: String, environment: PayPalEnvironment) -> Result<Self> {
        let config = PayPalConfig {
            client_id,
            client_secret,
            environment: environment.clone(),
            webhook_id: None,
        };
        
        let client = PayPalClient::new(config)?;
        
        Ok(Self {
            client: std::sync::Arc::new(tokio::sync::Mutex::new(client)),
            environment,
        })
    }

    fn map_payer_to_customer(&self, payer: &PayPalPayer) -> UnifiedCustomer {
        UnifiedCustomer {
            id: payer.payer_id.clone(),
            email: payer.email_address.clone(),
            name: payer.name.as_ref().and_then(|n| {
                match (&n.given_name, &n.surname) {
                    (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
                    (Some(first), None) => Some(first.clone()),
                    (None, Some(last)) => Some(last.clone()),
                    _ => None,
                }
            }),
            phone: payer.phone.as_ref().and_then(|p| {
                p.phone_number.national_number.clone().into()
            }),
            metadata: None,
        }
    }

    fn map_customer_to_payer(&self, customer: &UnifiedCustomer) -> PayPalPayer {
        let name = customer.name.as_ref().map(|n| {
            let parts: Vec<&str> = n.split_whitespace().collect();
            PayPalName {
                given_name: parts.first().map(|s| s.to_string()),
                surname: if parts.len() > 1 {
                    Some(parts[1..].join(" "))
                } else {
                    None
                },
            }
        });

        PayPalPayer {
            name,
            email_address: customer.email.clone(),
            payer_id: customer.id.clone(),
            phone: customer.phone.as_ref().map(|p| PayPalPhone {
                phone_type: Some("MOBILE".to_string()),
                phone_number: PayPalPhoneNumber {
                    national_number: p.clone(),
                },
            }),
            birth_date: None,
            tax_info: None,
            address: None,
        }
    }

    fn map_money(&self, money: &Money) -> PayPalMoney {
        PayPalMoney {
            currency_code: money.currency.to_uppercase(),
            value: format!("{:.2}", money.amount as f64 / 100.0), // Convert cents to dollars
        }
    }

    fn map_paypal_money(&self, money: &PayPalMoney) -> Money {
        Money {
            amount: (money.value.parse::<f64>().unwrap_or(0.0) * 100.0) as i64, // Convert dollars to cents
            currency: money.currency_code.to_lowercase(),
        }
    }

    fn map_order_status(&self, status: &OrderStatus) -> ChargeStatus {
        match status {
            OrderStatus::Created | OrderStatus::Saved => ChargeStatus::Pending,
            OrderStatus::Approved => ChargeStatus::Processing,
            OrderStatus::Completed => ChargeStatus::Succeeded,
            OrderStatus::Voided => ChargeStatus::Canceled,
            OrderStatus::PayerActionRequired => ChargeStatus::RequiresAction,
        }
    }

    fn map_payment_status(&self, status: &PaymentStatus) -> ChargeStatus {
        match status {
            PaymentStatus::Created => ChargeStatus::Pending,
            PaymentStatus::Captured | PaymentStatus::Completed => ChargeStatus::Succeeded,
            PaymentStatus::Denied | PaymentStatus::Failed | PaymentStatus::Declined => ChargeStatus::Failed,
            PaymentStatus::Canceled | PaymentStatus::Voided => ChargeStatus::Canceled,
            PaymentStatus::Pending | PaymentStatus::PartiallyRefunded => ChargeStatus::Processing,
            PaymentStatus::Refunded => ChargeStatus::Succeeded, // Refunded payments were successful initially
        }
    }

    fn map_order_to_charge(&self, order: &Order) -> UnifiedCharge {
        let amount = order.purchase_units.first()
            .map(|pu| self.map_paypal_money(&pu.amount))
            .unwrap_or_else(|| Money { amount: 0, currency: "usd".to_string() });

        UnifiedCharge {
            id: order.id.clone(),
            amount,
            customer_id: order.payer.as_ref().and_then(|p| p.payer_id.clone()),
            payment_method_id: None, // PayPal doesn't use payment method IDs the same way
            status: order.status.as_ref()
                .map(|s| self.map_order_status(s))
                .unwrap_or(ChargeStatus::Pending),
            description: order.purchase_units.first()
                .and_then(|pu| pu.description.clone()),
            metadata: None,
            created_at: order.create_time.as_ref()
                .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                .map(|dt| dt.timestamp()),
        }
    }

    fn map_refund_status(&self, status: &PayPalRefundStatus) -> RefundStatus {
        match status {
            PayPalRefundStatus::Pending => RefundStatus::Pending,
            PayPalRefundStatus::Completed => RefundStatus::Succeeded,
            PayPalRefundStatus::Failed => RefundStatus::Failed,
            PayPalRefundStatus::Canceled | PayPalRefundStatus::Cancelled => RefundStatus::Canceled,
        }
    }

    fn map_subscription_status(&self, status: &PayPalSubscriptionStatus) -> SubscriptionStatus {
        match status {
            PayPalSubscriptionStatus::ApprovalPending => SubscriptionStatus::Incomplete,
            PayPalSubscriptionStatus::Approved => SubscriptionStatus::Incomplete,
            PayPalSubscriptionStatus::Active => SubscriptionStatus::Active,
            PayPalSubscriptionStatus::Suspended => SubscriptionStatus::PastDue,
            PayPalSubscriptionStatus::Cancelled => SubscriptionStatus::Canceled,
            PayPalSubscriptionStatus::Expired => SubscriptionStatus::Canceled,
        }
    }
}

#[async_trait]
impl PaymentProvider for PayPalProvider {
    fn name(&self) -> &str {
        "paypal"
    }

    fn supported_currencies(&self) -> Vec<String> {
        vec![
            "usd", "eur", "gbp", "jpy", "cad", "aud", "chf", "cny", "sek", "nzd",
            "mxn", "sgd", "hkd", "nok", "krw", "try", "rub", "inr", "brl", "zar",
            "myr", "huf", "twd", "dkk", "pln", "thb", "idr", "czk", "ils", "php",
            "aed"
        ].into_iter().map(String::from).collect()
    }

    fn supported_features(&self) -> Vec<PaymentFeature> {
        vec![
            PaymentFeature::OneTimePayments,
            PaymentFeature::RecurringPayments,
            PaymentFeature::Refunds,
            PaymentFeature::PartialRefunds,
            PaymentFeature::Webhooks,
            PaymentFeature::Subscriptions,
            PaymentFeature::DigitalWallets,
        ]
    }

    async fn create_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        // PayPal doesn't have a traditional customer creation endpoint
        // Customers are created implicitly when they make purchases
        // Return the customer as-is with a generated ID
        let mut result = customer.clone();
        if result.id.is_none() {
            result.id = Some(format!("CUST_{}", uuid::Uuid::new_v4()));
        }
        Ok(result)
    }

    async fn get_customer(&self, customer_id: &str) -> Result<UnifiedCustomer> {
        // PayPal doesn't have a traditional customer retrieval endpoint
        // This would typically be implemented by storing customer data separately
        Err(PayupError::UnsupportedOperation(
            "Direct customer retrieval not supported by PayPal. Customer data is embedded in transactions.".to_string()
        ))
    }

    async fn update_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        // PayPal doesn't have a traditional customer update endpoint
        Err(PayupError::UnsupportedOperation(
            "Direct customer updates not supported by PayPal.".to_string()
        ))
    }

    async fn delete_customer(&self, _customer_id: &str) -> Result<bool> {
        // PayPal doesn't have a traditional customer deletion endpoint
        Err(PayupError::UnsupportedOperation(
            "Customer deletion not supported by PayPal.".to_string()
        ))
    }

    async fn list_customers(&self, _limit: Option<u32>, _offset: Option<u32>) -> Result<Vec<UnifiedCustomer>> {
        // PayPal doesn't have a traditional customer listing endpoint
        Err(PayupError::UnsupportedOperation(
            "Customer listing not supported by PayPal.".to_string()
        ))
    }

    async fn create_payment_method(&self, _payment_method: &UnifiedPaymentMethod) -> Result<UnifiedPaymentMethod> {
        // PayPal handles payment methods differently through their checkout flow
        Err(PayupError::UnsupportedOperation(
            "PayPal handles payment methods through their checkout flow, not via API.".to_string()
        ))
    }

    async fn get_payment_method(&self, _payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "PayPal doesn't expose payment method details via API.".to_string()
        ))
    }

    async fn attach_payment_method(&self, _payment_method_id: &str, _customer_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "PayPal handles payment method attachment through their vault system.".to_string()
        ))
    }

    async fn detach_payment_method(&self, _payment_method_id: &str) -> Result<UnifiedPaymentMethod> {
        Err(PayupError::UnsupportedOperation(
            "PayPal handles payment method detachment through their vault system.".to_string()
        ))
    }

    async fn create_charge(&self, charge: &UnifiedCharge) -> Result<UnifiedCharge> {
        // Create a PayPal order
        let mut order = Order::new();
        order.intent = OrderIntent::Capture;
        
        let purchase_unit = PurchaseUnit {
            reference_id: Some(uuid::Uuid::new_v4().to_string()),
            amount: self.map_money(&charge.amount),
            payee: None,
            description: charge.description.clone(),
            custom_id: None,
            invoice_id: None,
            items: None,
            shipping: None,
        };
        
        order.purchase_units = vec![purchase_unit];
        
        // If we have customer info, add it as payer
        if let Some(customer_id) = &charge.customer_id {
            order.payer = Some(PayPalPayer {
                payer_id: Some(customer_id.clone()),
                name: None,
                email_address: None,
                phone: None,
                birth_date: None,
                tax_info: None,
                address: None,
            });
        }

        let mut client = self.client.lock().await;
        let created_order = order.async_create(&mut *client).await?;
        Ok(self.map_order_to_charge(&created_order))
    }

    async fn get_charge(&self, charge_id: &str) -> Result<UnifiedCharge> {
        let mut client = self.client.lock().await;
        let order = Order::async_get(&mut *client, charge_id).await?;
        Ok(self.map_order_to_charge(&order))
    }

    async fn capture_charge(&self, charge_id: &str, amount: Option<Money>) -> Result<UnifiedCharge> {
        let capture_request = CaptureRequest {
            amount: amount.map(|m| self.map_money(&m)),
            final_capture: Some(true),
            note_to_payer: None,
        };
        
        let mut client = self.client.lock().await;
        let _capture_response = Order::async_capture(&mut *client, charge_id, Some(capture_request)).await?;
        
        // Get the updated order
        let order = Order::async_get(&mut *client, charge_id).await?;
        Ok(self.map_order_to_charge(&order))
    }

    async fn list_charges(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedCharge>> {
        // PayPal doesn't provide a direct way to list all orders/charges
        // This would typically require transaction search API
        Err(PayupError::UnsupportedOperation(
            "PayPal requires using Transaction Search API for listing charges.".to_string()
        ))
    }

    async fn create_refund(&self, refund: &UnifiedRefund) -> Result<UnifiedRefund> {
        let mut client = self.client.lock().await;
        
        let refund_request = RefundRequest {
            amount: refund.amount.as_ref().map(|m| self.map_money(m)),
            invoice_id: None,
            note_to_payer: refund.reason.as_ref().and_then(|r| match r {
                RefundReason::RequestedByCustomer => Some("Customer requested refund".to_string()),
                RefundReason::Duplicate => Some("Duplicate payment".to_string()),
                RefundReason::Fraudulent => Some("Fraudulent transaction".to_string()),
                RefundReason::Other(msg) => Some(msg.clone()),
            }),
        };
        
        let paypal_refund = Payment::async_refund(&mut *client, &refund.charge_id, Some(refund_request)).await?;
        
        Ok(UnifiedRefund {
            id: paypal_refund.id,
            charge_id: refund.charge_id.clone(),
            amount: paypal_refund.amount.map(|m| self.map_paypal_money(&m)),
            reason: refund.reason.clone(),
            status: paypal_refund.status.as_ref()
                .map(|s| self.map_refund_status(s))
                .unwrap_or(RefundStatus::Pending),
            metadata: None,
        })
    }

    async fn get_refund(&self, refund_id: &str) -> Result<UnifiedRefund> {
        let mut client = self.client.lock().await;
        let paypal_refund = PayPalRefund::async_get(&mut *client, refund_id).await?;
        
        Ok(UnifiedRefund {
            id: paypal_refund.id,
            charge_id: paypal_refund.invoice_id.clone().unwrap_or_default(),
            amount: paypal_refund.amount.map(|m| self.map_paypal_money(&m)),
            reason: paypal_refund.note_to_payer.map(|note| RefundReason::Other(note)),
            status: paypal_refund.status.as_ref()
                .map(|s| self.map_refund_status(s))
                .unwrap_or(RefundStatus::Pending),
            metadata: None,
        })
    }

    async fn list_refunds(&self, _charge_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedRefund>> {
        // PayPal doesn't provide a direct endpoint to list refunds
        Err(PayupError::UnsupportedOperation(
            "PayPal doesn't provide a direct API to list refunds.".to_string()
        ))
    }

    async fn create_subscription(&self, subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        let mut paypal_sub = PayPalSubscription::new();
        paypal_sub.plan_id = subscription.plan_id.clone().or(subscription.price_id.clone()).unwrap_or_default();
        paypal_sub.subscriber = Some(subscriptions::Subscriber {
            name: None,
            email_address: None,
            payer_id: Some(subscription.customer_id.clone()),
            phone: None,
            shipping_address: None,
        });
        
        let mut client = self.client.lock().await;
        let created = paypal_sub.async_create(&mut *client).await?;
        
        Ok(UnifiedSubscription {
            id: created.id,
            customer_id: subscription.customer_id.clone(),
            plan_id: Some(created.plan_id.clone()),
            price_id: None,
            status: created.status.as_ref()
                .map(|s| self.map_subscription_status(s))
                .unwrap_or(SubscriptionStatus::Incomplete),
            current_period_start: created.billing_info.as_ref()
                .and_then(|bi| bi.last_payment.as_ref())
                .and_then(|lp| chrono::DateTime::parse_from_rfc3339(&lp.time).ok())
                .map(|dt| dt.timestamp()),
            current_period_end: created.billing_info.as_ref()
                .and_then(|bi| bi.next_billing_time.as_ref())
                .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                .map(|dt| dt.timestamp()),
            cancel_at_period_end: false,
        })
    }

    async fn get_subscription(&self, subscription_id: &str) -> Result<UnifiedSubscription> {
        let mut client = self.client.lock().await;
        let paypal_sub = PayPalSubscription::async_get(&mut *client, subscription_id).await?;
        
        Ok(UnifiedSubscription {
            id: paypal_sub.id,
            customer_id: paypal_sub.subscriber.as_ref()
                .and_then(|s| s.payer_id.clone())
                .unwrap_or_default(),
            plan_id: Some(paypal_sub.plan_id.clone()),
            price_id: None,
            status: paypal_sub.status.as_ref()
                .map(|s| self.map_subscription_status(s))
                .unwrap_or(SubscriptionStatus::Incomplete),
            current_period_start: paypal_sub.billing_info.as_ref()
                .and_then(|bi| bi.last_payment.as_ref())
                .and_then(|lp| chrono::DateTime::parse_from_rfc3339(&lp.time).ok())
                .map(|dt| dt.timestamp()),
            current_period_end: paypal_sub.billing_info.as_ref()
                .and_then(|bi| bi.next_billing_time.as_ref())
                .and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok())
                .map(|dt| dt.timestamp()),
            cancel_at_period_end: false,
        })
    }

    async fn update_subscription(&self, subscription: &UnifiedSubscription) -> Result<UnifiedSubscription> {
        // PayPal subscription updates are limited
        // Can mainly suspend/reactivate
        if subscription.cancel_at_period_end {
            let mut client = self.client.lock().await;
            PayPalSubscription::suspend(
                &mut *client,
                subscription.id.as_ref().unwrap_or(&String::new()),
                Some("User requested cancellation at period end".to_string())
            )?;
        }
        
        self.get_subscription(subscription.id.as_ref().unwrap_or(&String::new())).await
    }

    async fn cancel_subscription(&self, subscription_id: &str, _at_period_end: bool) -> Result<UnifiedSubscription> {
        let mut client = self.client.lock().await;
        PayPalSubscription::cancel(&mut *client, subscription_id, Some("User requested cancellation".to_string()))?;
        self.get_subscription(subscription_id).await
    }

    async fn list_subscriptions(&self, _customer_id: Option<&str>, _limit: Option<u32>) -> Result<Vec<UnifiedSubscription>> {
        // PayPal doesn't provide a direct way to list subscriptions
        Err(PayupError::UnsupportedOperation(
            "PayPal doesn't provide a direct API to list subscriptions.".to_string()
        ))
    }

    async fn verify_webhook(&self, _payload: &[u8], _signature: &str, _secret: &str) -> Result<bool> {
        // TODO: Implement webhook verification once PayPalWebhookHandler is available
        // use super::webhooks::PayPalWebhookHandler;
        // 
        // let handler = PayPalWebhookHandler::new(secret.to_string());
        // Ok(handler.verify_signature(
        //     std::str::from_utf8(payload).map_err(|e| 
        //         PayupError::ValidationError(format!("Invalid UTF-8 in webhook payload: {}", e))
        //     )?,
        //     signature
        // ))
        Ok(false)
    }
}