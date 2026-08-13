use crate::error::Result;
use crate::http_client::get_shared_client;
use crate::stripe::request::{idempotent_request, parse_async_response, IdempotencyKey};
use crate::stripe::Auth;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: Option<String>,
    pub object: Option<String>,
    pub billing_cycle_anchor: Option<i64>,
    pub cancel_at: Option<i64>,
    pub cancel_at_period_end: Option<bool>,
    pub canceled_at: Option<i64>,
    pub collection_method: Option<String>,
    pub created: Option<i64>,
    pub current_period_end: Option<i64>,
    pub current_period_start: Option<i64>,
    pub customer: Option<String>,
    pub days_until_due: Option<i64>,
    pub default_payment_method: Option<String>,
    pub ended_at: Option<i64>,
    pub latest_invoice: Option<String>,
    pub livemode: Option<bool>,
    pub quantity: Option<i64>,
    pub start_date: Option<i64>,
    pub status: Option<String>,
    pub price_items: Option<Vec<String>>,
    /// Existing Stripe subscription-item IDs paired by index with `price_items` on update.
    pub subscription_item_ids: Option<Vec<String>>,
}

impl Subscription {
    pub fn new() -> Self {
        Subscription {
            id: None,
            object: None,
            billing_cycle_anchor: None,
            cancel_at: None,
            cancel_at_period_end: None,
            canceled_at: None,
            collection_method: None,
            created: None,
            current_period_end: None,
            current_period_start: None,
            customer: None,
            price_items: None,
            subscription_item_ids: None,
            days_until_due: None,
            default_payment_method: None,
            ended_at: None,
            latest_invoice: None,
            livemode: None,
            quantity: None,
            start_date: None,
            status: None,
        }
    }

    pub async fn async_cancel(
        creds: Auth,
        id: String,
        cancel_at_period_end: bool,
    ) -> Result<crate::stripe::response::Subscription> {
        Self::async_cancel_with_idempotency_key(
            creds,
            id,
            cancel_at_period_end,
            IdempotencyKey::generate(),
        )
        .await
    }

    pub async fn async_cancel_with_idempotency_key(
        creds: Auth,
        id: String,
        cancel_at_period_end: bool,
        idempotency_key: IdempotencyKey,
    ) -> Result<crate::stripe::response::Subscription> {
        if cancel_at_period_end {
            let mut subscription = Self::new();
            subscription.id = Some(id);
            subscription.cancel_at_period_end = Some(true);
            return subscription
                .async_update_with_idempotency_key(creds, idempotency_key)
                .await;
        }

        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);
        let request = get_shared_client()
            .delete(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_request(request, &idempotency_key)
            .send()
            .await?;
        parse_async_response(response).await
    }

    pub async fn async_get(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Subscription> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);

        let response = get_shared_client()
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()))
            .send()
            .await?;

        parse_async_response(response).await
    }

    pub async fn async_update(&self, creds: Auth) -> Result<crate::stripe::response::Subscription> {
        self.async_update_with_idempotency_key(creds, IdempotencyKey::generate())
            .await
    }

    pub async fn async_update_with_idempotency_key(
        &self,
        creds: Auth,
        idempotency_key: IdempotencyKey,
    ) -> Result<crate::stripe::response::Subscription> {
        let id = self.id.as_ref().ok_or_else(|| {
            crate::error::PayupError::ValidationError(
                "Subscription ID is required for update".to_string(),
            )
        })?;
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);
        let request = get_shared_client()
            .post(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_request(request, &idempotency_key)
            .form(&self.to_params())
            .send()
            .await?;
        parse_async_response(response).await
    }

    pub async fn async_post(&self, creds: Auth) -> Result<crate::stripe::response::Subscription> {
        self.async_post_with_idempotency_key(creds, IdempotencyKey::generate())
            .await
    }

    pub async fn async_post_with_idempotency_key(
        &self,
        creds: Auth,
        idempotency_key: IdempotencyKey,
    ) -> Result<crate::stripe::response::Subscription> {
        let request = get_shared_client()
            .post("https://api.stripe.com/v1/subscriptions")
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_request(request, &idempotency_key)
            .form(&self.to_params())
            .send()
            .await?;
        parse_async_response(response).await
    }

    pub async fn async_list(
        creds: Auth,
        customer_id: Option<String>,
        limit: Option<i32>,
    ) -> Result<crate::stripe::response::Subscriptions> {
        let mut request = get_shared_client()
            .get("https://api.stripe.com/v1/subscriptions")
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        if let Some(customer_id) = customer_id {
            request = request.query(&[("customer", customer_id)]);
        }
        if let Some(limit) = limit {
            request = request.query(&[("limit", limit)]);
        }
        let response = request.send().await?;
        parse_async_response(response).await
    }

    pub fn to_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];

        if let Some(customer) = &self.customer {
            params.push(("customer".to_string(), customer.clone()));
        }

        if let Some(default_payment_method) = &self.default_payment_method {
            params.push((
                "default_payment_method".to_string(),
                default_payment_method.clone(),
            ));
        }

        if let Some(collection_method) = &self.collection_method {
            params.push(("collection_method".to_string(), collection_method.clone()));
        }

        if let Some(cancel_at_period_end) = &self.cancel_at_period_end {
            params.push((
                "cancel_at_period_end".to_string(),
                cancel_at_period_end.to_string(),
            ));
        }

        if let Some(days_until_due) = &self.days_until_due {
            params.push(("days_until_due".to_string(), days_until_due.to_string()));
        }

        if let Some(price_items) = &self.price_items {
            for (i, item) in price_items.iter().enumerate() {
                if i < 20 {
                    if let Some(item_id) = self
                        .subscription_item_ids
                        .as_ref()
                        .and_then(|ids| ids.get(i))
                    {
                        params.push((format!("items[{}][id]", i), item_id.clone()));
                    }
                    params.push((format!("items[{}][price]", i), item.clone()));
                }
            }
        }

        params
    }
}

impl Default for Subscription {
    fn default() -> Self {
        Self::new()
    }
}
