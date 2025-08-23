use serde::{Deserialize, Serialize};
use crate::stripe::Auth;

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
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);
        
        let params = if cancel_at_period_end {
            vec![("cancel_at_period_end", "true")]
        } else {
            vec![]
        };

        let request = reqwest::Client::new()
            .delete(&url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&params)
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Subscription>().await?;
        Ok(json)
    }

    pub async fn async_get(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);

        let request = reqwest::Client::new()
            .get(&url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Subscription>().await?;
        Ok(json)
    }

    pub async fn async_update(
        &self,
        creds: Auth,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let url = format!(
            "https://api.stripe.com/v1/subscriptions/{}",
            self.id.as_ref().unwrap()
        );

        let request = reqwest::Client::new()
            .post(&url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Subscription>().await?;
        Ok(json)
    }

    pub async fn async_post(
        &self,
        creds: Auth,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/subscriptions")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Subscription>().await?;
        Ok(json)
    }

    pub async fn async_list(
        creds: Auth,
        customer_id: Option<String>,
        limit: Option<i32>,
    ) -> Result<crate::stripe::response::Subscriptions, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/subscriptions".to_string();
        let mut params = vec![];
        
        if let Some(customer) = customer_id {
            params.push(format!("customer={}", customer));
        }
        
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let request = reqwest::Client::new()
            .get(&url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Subscriptions>().await?;
        Ok(json)
    }

    pub fn to_params(&self) -> Vec<(&str, String)> {
        let mut params = vec![];
        
        if let Some(customer) = &self.customer {
            params.push(("customer", customer.clone()));
        }

        if let Some(default_payment_method) = &self.default_payment_method {
            params.push(("default_payment_method", default_payment_method.clone()));
        }

        if let Some(collection_method) = &self.collection_method {
            params.push(("collection_method", collection_method.clone()));
        }

        if let Some(cancel_at_period_end) = &self.cancel_at_period_end {
            params.push(("cancel_at_period_end", cancel_at_period_end.to_string()));
        }

        if let Some(days_until_due) = &self.days_until_due {
            params.push(("days_until_due", days_until_due.to_string()));
        }

        if let Some(price_items) = &self.price_items {
            for (i, item) in price_items.iter().enumerate() {
                if i < 20 {
                    params.push((
                        Box::leak(format!("items[{}][price]", i).into_boxed_str()),
                        item.clone()
                    ));
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