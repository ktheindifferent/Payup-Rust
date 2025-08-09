use serde::{Deserialize, Serialize};
use crate::stripe::Auth;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payout {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: i64,
    pub arrival_date: Option<i64>,
    pub automatic: Option<bool>,
    pub balance_transaction: Option<String>,
    pub created: Option<i64>,
    pub currency: String,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub livemode: Option<bool>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub method: Option<PayoutMethod>,
    pub original_payout: Option<String>,
    pub reconciliation_status: Option<ReconciliationStatus>,
    pub reversed_by: Option<String>,
    pub source_type: Option<String>,
    pub statement_descriptor: Option<String>,
    pub status: Option<PayoutStatus>,
    pub payout_type: Option<PayoutType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethod {
    Standard,
    Instant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    Bank,
    Card,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationStatus {
    NotApplicable,
    InProgress,
    Completed,
}

impl Payout {
    pub fn new() -> Self {
        Self {
            id: None,
            object: None,
            amount: 0,
            arrival_date: None,
            automatic: None,
            balance_transaction: None,
            created: None,
            currency: "usd".to_string(),
            description: None,
            destination: None,
            failure_balance_transaction: None,
            failure_code: None,
            failure_message: None,
            livemode: None,
            metadata: None,
            method: None,
            original_payout: None,
            reconciliation_status: None,
            reversed_by: None,
            source_type: None,
            statement_descriptor: None,
            status: None,
            payout_type: None,
        }
    }

    /// Create a payout
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe_ext::payout::Payout;
    /// use payup::stripe::Auth;
    /// 
    /// let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    /// let mut payout = Payout::new();
    /// payout.amount = 10000; // Payout $100.00
    /// payout.currency = "usd".to_string();
    /// payout.description = Some("Monthly payout".to_string());
    /// let created_payout = payout.post(auth)?;
    /// ```
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let mut params = std::collections::HashMap::new();
        
        params.insert("amount", self.amount.to_string());
        params.insert("currency", self.currency.clone());
        
        if let Some(desc) = &self.description {
            params.insert("description", desc.clone());
        }
        
        if let Some(dest) = &self.destination {
            params.insert("destination", dest.clone());
        }
        
        if let Some(method) = &self.method {
            params.insert("method", format!("{:?}", method).to_lowercase());
        }
        
        if let Some(statement) = &self.statement_descriptor {
            params.insert("statement_descriptor", statement.clone());
        }

        let response = client
            .post("https://api.stripe.com/v1/payouts")
            .header("Authorization", format!("Bearer {}", creds.client))
            .form(&params)
            .send()?;

        response.json()
    }

    /// Retrieve a payout
    pub fn get(creds: Auth, payout_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.stripe.com/v1/payouts/{}", payout_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// Update a payout
    pub fn update(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        if let Some(id) = &self.id {
            let client = reqwest::blocking::Client::new();
            let url = format!("https://api.stripe.com/v1/payouts/{}", id);
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

    /// Cancel a payout
    pub fn cancel(creds: Auth, payout_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.stripe.com/v1/payouts/{}/cancel", payout_id);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// Reverse a payout
    pub fn reverse(creds: Auth, payout_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.stripe.com/v1/payouts/{}/reverse", payout_id);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()?;

        response.json()
    }

    /// List all payouts
    pub fn list(creds: Auth, status: Option<PayoutStatus>, limit: Option<i32>) -> Result<PayoutList, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let mut url = String::from("https://api.stripe.com/v1/payouts?");
        
        if let Some(s) = status {
            url.push_str(&format!("status={}&", format!("{:?}", s).to_lowercase()));
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

    /// Async create a payout
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut params = std::collections::HashMap::new();
        
        params.insert("amount", self.amount.to_string());
        params.insert("currency", self.currency.clone());
        
        if let Some(desc) = &self.description {
            params.insert("description", desc.clone());
        }
        
        if let Some(method) = &self.method {
            params.insert("method", format!("{:?}", method).to_lowercase());
        }

        let response = client
            .post("https://api.stripe.com/v1/payouts")
            .header("Authorization", format!("Bearer {}", creds.client))
            .form(&params)
            .send()
            .await?;

        response.json().await
    }

    /// Async retrieve a payout
    pub async fn async_get(creds: Auth, payout_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("https://api.stripe.com/v1/payouts/{}", payout_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()
            .await?;

        response.json().await
    }

    /// Async cancel a payout
    pub async fn async_cancel(creds: Auth, payout_id: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let url = format!("https://api.stripe.com/v1/payouts/{}/cancel", payout_id);

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", creds.client))
            .send()
            .await?;

        response.json().await
    }

    /// Async list all payouts
    pub async fn async_list(creds: Auth, status: Option<PayoutStatus>, limit: Option<i32>) -> Result<PayoutList, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut url = String::from("https://api.stripe.com/v1/payouts?");
        
        if let Some(s) = status {
            url.push_str(&format!("status={}&", format!("{:?}", s).to_lowercase()));
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
pub struct PayoutList {
    pub object: String,
    pub data: Vec<Payout>,
    pub has_more: bool,
    pub url: String,
}