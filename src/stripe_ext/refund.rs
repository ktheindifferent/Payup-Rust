use serde::{Deserialize, Serialize};
use crate::stripe::Auth;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Refund {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: Option<i64>,
    pub balance_transaction: Option<String>,
    pub charge: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_reason: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub payment_intent: Option<String>,
    pub reason: Option<RefundReason>,
    pub receipt_number: Option<String>,
    pub source_transfer_reversal: Option<String>,
    pub status: Option<RefundStatus>,
    pub transfer_reversal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    ExpiredUncapturedCharge,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Pending,
    Succeeded,
    Failed,
    Canceled,
    RequiresAction,
}

impl Refund {
    pub fn new() -> Self {
        Self {
            id: None,
            object: None,
            amount: None,
            balance_transaction: None,
            charge: None,
            created: None,
            currency: None,
            description: None,
            failure_balance_transaction: None,
            failure_reason: None,
            metadata: None,
            payment_intent: None,
            reason: None,
            receipt_number: None,
            source_transfer_reversal: None,
            status: None,
            transfer_reversal: None,
        }
    }

    /// Create a refund
    /// 
    /// # Example
    /// ```no_run
    /// let mut refund = Refund::new();
    /// refund.charge = Some("ch_test123".to_string());
    /// refund.amount = Some(1000); // Refund $10.00
    /// refund.reason = Some(RefundReason::RequestedByCustomer);
    /// let created_refund = refund.post(auth)?;
    /// ```
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let mut params = std::collections::HashMap::new();
        
        if let Some(charge) = &self.charge {
            params.insert("charge", charge.clone());
        }
        
        if let Some(payment_intent) = &self.payment_intent {
            params.insert("payment_intent", payment_intent.clone());
        }
        
        if let Some(amount) = &self.amount {
            params.insert("amount", amount.to_string());
        }
        
        if let Some(reason) = &self.reason {
            params.insert("reason", format!("{:?}", reason).to_lowercase());
        }
        
        if let Some(desc) = &self.description {
            params.insert("description", desc.clone());
        }

        let response = client
            .post("https://api.stripe.com/v1/refunds")
            .header("Authorization", format!("Bearer {}", creds.client))
            .form(&params)
            .send()?;

        response.json()
    }

    /// Retrieve a refund
    pub fn get(creds: Auth, refund_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.stripe.com/v1/refunds/{}", refund_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// Update a refund
    pub fn update(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        if let Some(id) = &self.id {
            let client = reqwest::blocking::Client::new();
            let url = format!("https://api.stripe.com/v1/refunds/{}", id);
            let mut params = std::collections::HashMap::new();

            if let Some(metadata) = &self.metadata {
                for (key, value) in metadata {
                    params.insert(format!("metadata[{}]", key), value.clone());
                }
            }

            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", creds.client))
                .form(&params)
                .send()?;

            response.json()
        } else {
            // Return a mock error - in production, use proper error handling
            let client = reqwest::blocking::Client::new();
            let response = client.get("https://invalid.url").send()?;
            response.json()
        }
    }

    /// Cancel a refund
    pub fn cancel(creds: Auth, refund_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.stripe.com/v1/refunds/{}/cancel", refund_id);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// List all refunds
    pub fn list(creds: Auth, charge_id: Option<String>, limit: Option<i32>) -> Result<RefundList, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let mut url = String::from("https://api.stripe.com/v1/refunds?");
        
        if let Some(charge) = charge_id {
            url.push_str(&format!("charge={}&", charge));
        }
        
        if let Some(lim) = limit {
            url.push_str(&format!("limit={}", lim));
        }

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// Async create a refund
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut params = std::collections::HashMap::new();
        
        if let Some(charge) = &self.charge {
            params.insert("charge", charge.clone());
        }
        
        if let Some(payment_intent) = &self.payment_intent {
            params.insert("payment_intent", payment_intent.clone());
        }
        
        if let Some(amount) = &self.amount {
            params.insert("amount", amount.to_string());
        }
        
        if let Some(reason) = &self.reason {
            params.insert("reason", format!("{:?}", reason).to_lowercase());
        }

        let response = client
            .post("https://api.stripe.com/v1/refunds")
            .header("Authorization", format!("Bearer {}", creds.client))
            .form(&params)
            .send()
            .await?;

        response.json().await
    }

    /// Async retrieve a refund
    pub async fn async_get(creds: Auth, refund_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("https://api.stripe.com/v1/refunds/{}", refund_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()
            .await?;

        response.json().await
    }

    /// Async list all refunds
    pub async fn async_list(creds: Auth, charge_id: Option<String>, limit: Option<i32>) -> Result<RefundList, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut url = String::from("https://api.stripe.com/v1/refunds?");
        
        if let Some(charge) = charge_id {
            url.push_str(&format!("charge={}&", charge));
        }
        
        if let Some(lim) = limit {
            url.push_str(&format!("limit={}", lim));
        }

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()
            .await?;

        response.json().await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefundList {
    pub object: String,
    pub data: Vec<Refund>,
    pub has_more: bool,
    pub url: String,
}