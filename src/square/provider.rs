use async_trait::async_trait;
use crate::error::{PayupError, Result};
use crate::payment_provider::{
    PaymentProvider, PaymentFeature, Customer as UnifiedCustomer, PaymentMethod as UnifiedPaymentMethod,
    Charge as UnifiedCharge, ChargeStatus,
    Refund as UnifiedRefund, RefundStatus, RefundReason, Subscription as UnifiedSubscription,
    Money as UnifiedMoney
};
use super::{
    SquareClient, SquareConfig, Environment, Money,
    payments::{Payment, CreatePaymentRequest, Refund as SquareRefund, RefundPaymentRequest},
    customers::{Customer, CreateCustomerRequest, UpdateCustomerRequest},
};

pub struct SquareProvider {
    client: std::sync::Arc<tokio::sync::Mutex<SquareClient>>,
    #[allow(dead_code)]
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
    
    fn map_refund(&self, refund: &SquareRefund) -> UnifiedRefund {
        let mut metadata = std::collections::HashMap::new();
        if let Some(order_id) = &refund.order_id {
            metadata.insert("order_id".to_string(), order_id.clone());
        }
        if let Some(location_id) = &refund.location_id {
            metadata.insert("location_id".to_string(), location_id.clone());
        }
        
        UnifiedRefund {
            id: refund.id.clone(),
            charge_id: refund.payment_id.clone().unwrap_or_default(),
            amount: Some(self.map_square_money(&refund.amount_money)),
            reason: refund.reason.as_ref().map(|r| match r.as_str() {
                "DUPLICATE" => RefundReason::Duplicate,
                "FRAUDULENT" => RefundReason::Fraudulent,
                "REQUESTED_BY_CUSTOMER" => RefundReason::RequestedByCustomer,
                other => RefundReason::Other(other.to_string()),
            }),
            status: refund.status.as_ref().map(|s| match s.as_str() {
                "PENDING" => RefundStatus::Pending,
                "COMPLETED" => RefundStatus::Succeeded,
                "REJECTED" | "FAILED" => RefundStatus::Failed,
                "CANCELED" => RefundStatus::Canceled,
                _ => RefundStatus::Pending,
            }).unwrap_or(RefundStatus::Pending),
            metadata: if metadata.is_empty() { None } else { Some(metadata) },
        }
    }

    fn map_customer(&self, customer: &Customer) -> UnifiedCustomer {
        let mut metadata = std::collections::HashMap::new();
        if let Some(note) = &customer.note {
            metadata.insert("note".to_string(), note.clone());
        }
        if let Some(ref_id) = &customer.reference_id {
            metadata.insert("reference_id".to_string(), ref_id.clone());
        }
        
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
            metadata: if metadata.is_empty() { None } else { Some(metadata) },
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
        let mut metadata = std::collections::HashMap::new();
        if let Some(ref_id) = &payment.reference_id {
            metadata.insert("reference_id".to_string(), ref_id.clone());
        }
        if let Some(order_id) = &payment.order_id {
            metadata.insert("order_id".to_string(), order_id.clone());
        }
        if let Some(receipt_url) = &payment.receipt_url {
            metadata.insert("receipt_url".to_string(), receipt_url.clone());
        }
        
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
            metadata: if metadata.is_empty() { None } else { Some(metadata) },
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

    async fn create_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        let client = self.client.lock().await;
        
        // Split the name if provided
        let (given_name, family_name) = if let Some(name) = &customer.name {
            let parts: Vec<&str> = name.split_whitespace().collect();
            match parts.len() {
                0 => (None, None),
                1 => (Some(parts[0].to_string()), None),
                _ => {
                    let given = parts[0].to_string();
                    let family = parts[1..].join(" ");
                    (Some(given), Some(family))
                }
            }
        } else {
            (None, None)
        };
        
        let request = CreateCustomerRequest {
            idempotency_key: Some(uuid::Uuid::new_v4().to_string()),
            given_name,
            family_name,
            company_name: None,
            nickname: None,
            email_address: customer.email.clone(),
            address: None,
            phone_number: customer.phone.clone(),
            reference_id: customer.id.clone(),
            note: customer.metadata.as_ref()
                .and_then(|m| m.get("note"))
                .cloned(),
            birthday: None,
        };
        
        let square_customer = Customer::async_create(&client, &request).await?;
        Ok(self.map_customer(&square_customer))
    }

    async fn get_customer(&self, customer_id: &str) -> Result<UnifiedCustomer> {
        let client = self.client.lock().await;
        let square_customer = Customer::async_get(&client, customer_id).await?;
        Ok(self.map_customer(&square_customer))
    }

    async fn update_customer(&self, customer: &UnifiedCustomer) -> Result<UnifiedCustomer> {
        let client = self.client.lock().await;
        
        let customer_id = customer.id.as_ref()
            .ok_or_else(|| PayupError::GenericError("Customer ID is required for update".to_string()))?;
        
        // Split the name if provided
        let (given_name, family_name) = if let Some(name) = &customer.name {
            let parts: Vec<&str> = name.split_whitespace().collect();
            match parts.len() {
                0 => (None, None),
                1 => (Some(parts[0].to_string()), None),
                _ => {
                    let given = parts[0].to_string();
                    let family = parts[1..].join(" ");
                    (Some(given), Some(family))
                }
            }
        } else {
            (None, None)
        };
        
        let request = UpdateCustomerRequest {
            given_name,
            family_name,
            company_name: None,
            nickname: None,
            email_address: customer.email.clone(),
            address: None,
            phone_number: customer.phone.clone(),
            reference_id: None,
            note: customer.metadata.as_ref()
                .and_then(|m| m.get("note"))
                .cloned(),
            birthday: None,
            version: None,
        };
        
        let square_customer = Customer::async_update(&client, customer_id, &request).await?;
        Ok(self.map_customer(&square_customer))
    }

    async fn delete_customer(&self, customer_id: &str) -> Result<bool> {
        let client = self.client.lock().await;
        Customer::async_delete(&client, customer_id).await
    }

    async fn list_customers(&self, limit: Option<u32>, _offset: Option<u32>) -> Result<Vec<UnifiedCustomer>> {
        let client = self.client.lock().await;
        
        // Square uses cursor-based pagination, we'll convert offset to cursor if needed
        // Note: Square doesn't directly support offset, uses cursor pagination
        // For simplicity, we'll just use limit for now
        let customers = Customer::async_list(&client, None, limit.map(|l| l as i32)).await?;
        
        let unified_customers: Vec<UnifiedCustomer> = customers.iter()
            .map(|c| self.map_customer(c))
            .collect();
        
        Ok(unified_customers)
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

    async fn create_charge(&self, charge: &UnifiedCharge) -> Result<UnifiedCharge> {
        let client = self.client.lock().await;
        
        // For Square, we need a source_id (payment source like card nonce or customer card on file)
        // Since we don't have that in the unified charge, we'll need to use a placeholder
        // In a real implementation, this would come from a separate tokenization process
        let source_id = charge.payment_method_id.as_ref()
            .ok_or_else(|| PayupError::GenericError(
                "Square requires a source_id (payment token) to create a payment".to_string()
            ))?;
        
        let request = CreatePaymentRequest {
            source_id: source_id.clone(),
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            amount_money: self.map_money(&charge.amount),
            tip_money: None,
            app_fee_money: None,
            autocomplete: Some(true), // Auto-capture the payment
            customer_id: charge.customer_id.clone(),
            location_id: None, // Would need to be configured
            reference_id: charge.metadata.as_ref()
                .and_then(|m| m.get("reference_id"))
                .cloned(),
            note: charge.description.clone(),
            order_id: charge.metadata.as_ref()
                .and_then(|m| m.get("order_id"))
                .cloned(),
            verification_token: None,
        };
        
        let payment = Payment::async_create(&client, &request).await?;
        Ok(self.map_payment(&payment))
    }

    async fn get_charge(&self, charge_id: &str) -> Result<UnifiedCharge> {
        let client = self.client.lock().await;
        let payment = Payment::async_get(&client, charge_id).await?;
        Ok(self.map_payment(&payment))
    }

    async fn capture_charge(&self, _charge_id: &str, _amount: Option<UnifiedMoney>) -> Result<UnifiedCharge> {
        // Square captures payments immediately by default
        Err(PayupError::UnsupportedOperation(
            "Square captures payments immediately. Use delayed capture for manual capture.".to_string()
        ))
    }

    async fn list_charges(&self, customer_id: Option<&str>, limit: Option<u32>) -> Result<Vec<UnifiedCharge>> {
        let client = self.client.lock().await;
        
        // Square doesn't support filtering by customer_id directly in list payments
        // We'd need to use the Search API for that, but for now we'll list all
        // Add location_id if configured (Square may require this for listing payments)
        let payments = Payment::async_list(&client, None, limit.map(|l| l as i32)).await?;
        
        let mut unified_charges = Vec::new();
        for payment in payments {
            // Filter by customer_id if provided
            if let Some(cid) = customer_id {
                if payment.customer_id.as_deref() != Some(cid) {
                    continue;
                }
            }
            
            unified_charges.push(self.map_payment(&payment));
        }
        
        Ok(unified_charges)
    }

    async fn create_refund(&self, refund: &UnifiedRefund) -> Result<UnifiedRefund> {
        let client = self.client.lock().await;
        
        let amount_money = if let Some(amount) = &refund.amount {
            self.map_money(amount)
        } else {
            // If no amount specified, we'd need to get the payment to refund the full amount
            // For now, we'll require an amount
            return Err(PayupError::GenericError(
                "Square requires an amount for refunds".to_string()
            ));
        };
        
        let request = RefundPaymentRequest {
            idempotency_key: uuid::Uuid::new_v4().to_string(),
            amount_money,
            app_fee_money: None,
            payment_id: refund.charge_id.clone(),
            reason: refund.reason.as_ref().map(|r| match r {
                RefundReason::Duplicate => "DUPLICATE".to_string(),
                RefundReason::Fraudulent => "FRAUDULENT".to_string(),
                RefundReason::RequestedByCustomer => "REQUESTED_BY_CUSTOMER".to_string(),
                RefundReason::Other(reason) => reason.clone(),
            }),
        };
        
        let square_refund = SquareRefund::async_create(&client, &request).await?;
        Ok(self.map_refund(&square_refund))
    }

    async fn get_refund(&self, refund_id: &str) -> Result<UnifiedRefund> {
        let client = self.client.lock().await;
        let square_refund = SquareRefund::async_get(&client, refund_id).await?;
        Ok(self.map_refund(&square_refund))
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