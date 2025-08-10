use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::rate_limiter::get_rate_limiter;
use super::Auth;

/// Represents a Stripe Transfer object for moving funds between Stripe accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub amount_reversed: i64,
    pub balance_transaction: Option<String>,
    pub created: i64,
    pub currency: String,
    pub description: Option<String>,
    pub destination: String,
    pub destination_payment: Option<String>,
    pub livemode: bool,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub reversals: TransferReversalList,
    pub reversed: bool,
    pub source_transaction: Option<String>,
    pub source_type: String,
    pub transfer_group: Option<String>,
}

/// List of transfer reversals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReversalList {
    pub object: String,
    pub data: Vec<TransferReversal>,
    pub has_more: bool,
    pub url: String,
}

/// Represents a reversal of a transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReversal {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub balance_transaction: Option<String>,
    pub created: i64,
    pub currency: String,
    pub destination_payment_refund: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub source_refund: Option<String>,
    pub transfer: String,
}

/// Parameters for creating a transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferParams {
    pub amount: i64,
    pub currency: String,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

/// Parameters for updating a transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTransferParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

/// Parameters for creating a transfer reversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReversalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

impl Transfer {
    /// Create a new transfer
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, Transfer, CreateTransferParams};
    /// 
    /// let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    /// let params = CreateTransferParams {
    ///     amount: 1000,
    ///     currency: "usd".to_string(),
    ///     destination: "acct_1234567890".to_string(),
    ///     description: Some("Transfer for services".to_string()),
    ///     metadata: None,
    ///     source_transaction: None,
    ///     transfer_group: None,
    /// };
    /// let transfer = Transfer::create(&auth, params)?;
    /// ```
    pub fn create(auth: &Auth, params: CreateTransferParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.stripe.com/v1/transfers")
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let transfer: Transfer = response.json()?;
        Ok(transfer)
    }

    /// Create a new transfer (async)
    pub async fn create_async(auth: &Auth, params: CreateTransferParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        
        rate_limiter.execute_with_retry_async("stripe", || async {
            let client = reqwest::Client::new();
            let response = client
                .post("https://api.stripe.com/v1/transfers")
                .header("Authorization", format!("Bearer {}", auth.secret))
                .form(&params)
                .send()
                .await?;
            
            let transfer: Transfer = response.json().await?;
            Ok(transfer)
        }).await
    }

    /// Retrieve a transfer by ID
    pub fn retrieve(auth: &Auth, transfer_id: &str) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&format!("https://api.stripe.com/v1/transfers/{}", transfer_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let transfer: Transfer = response.json()?;
        Ok(transfer)
    }

    /// Retrieve a transfer by ID (async)
    pub async fn retrieve_async(auth: &Auth, transfer_id: &str) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let transfer_id = transfer_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let transfer_id = transfer_id.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .get(&format!("https://api.stripe.com/v1/transfers/{}", transfer_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                let transfer: Transfer = response.json().await?;
                Ok(transfer)
            }
        }).await
    }

    /// Update a transfer
    pub fn update(auth: &Auth, transfer_id: &str, params: UpdateTransferParams) -> Result<Self> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/transfers/{}", transfer_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let transfer: Transfer = response.json()?;
        Ok(transfer)
    }

    /// Update a transfer (async)
    pub async fn update_async(auth: &Auth, transfer_id: &str, params: UpdateTransferParams) -> Result<Self> {
        let rate_limiter = get_rate_limiter();
        let transfer_id = transfer_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let transfer_id = transfer_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/transfers/{}", transfer_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let transfer: Transfer = response.json().await?;
                Ok(transfer)
            }
        }).await
    }

    /// List all transfers
    pub fn list(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let client = reqwest::blocking::Client::new();
        let mut url = "https://api.stripe.com/v1/transfers".to_string();
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        #[derive(Deserialize)]
        struct TransferList {
            data: Vec<Transfer>,
        }
        
        let list: TransferList = response.json()?;
        Ok(list.data)
    }

    /// List all transfers (async)
    pub async fn list_async(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let rate_limiter = get_rate_limiter();
        
        rate_limiter.execute_with_retry_async("stripe", move || async move {
            let client = reqwest::Client::new();
            let mut url = "https://api.stripe.com/v1/transfers".to_string();
            if let Some(limit) = limit {
                url = format!("{}?limit={}", url, limit);
            }
            
            let response = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", auth.secret))
                .send()
                .await?;
            
            #[derive(Deserialize)]
            struct TransferList {
                data: Vec<Transfer>,
            }
            
            let list: TransferList = response.json().await?;
            Ok(list.data)
        }).await
    }

    /// Create a transfer reversal
    pub fn create_reversal(auth: &Auth, transfer_id: &str, params: CreateReversalParams) -> Result<TransferReversal> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&format!("https://api.stripe.com/v1/transfers/{}/reversals", transfer_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let reversal: TransferReversal = response.json()?;
        Ok(reversal)
    }

    /// Create a transfer reversal (async)
    pub async fn create_reversal_async(auth: &Auth, transfer_id: &str, params: CreateReversalParams) -> Result<TransferReversal> {
        let rate_limiter = get_rate_limiter();
        let transfer_id = transfer_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let transfer_id = transfer_id.clone();
            let params = params.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("https://api.stripe.com/v1/transfers/{}/reversals", transfer_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .form(&params)
                    .send()
                    .await?;
                
                let reversal: TransferReversal = response.json().await?;
                Ok(reversal)
            }
        }).await
    }

    /// Retrieve a transfer reversal
    pub fn retrieve_reversal(auth: &Auth, transfer_id: &str, reversal_id: &str) -> Result<TransferReversal> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&format!("https://api.stripe.com/v1/transfers/{}/reversals/{}", transfer_id, reversal_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let reversal: TransferReversal = response.json()?;
        Ok(reversal)
    }

    /// Retrieve a transfer reversal (async)
    pub async fn retrieve_reversal_async(auth: &Auth, transfer_id: &str, reversal_id: &str) -> Result<TransferReversal> {
        let rate_limiter = get_rate_limiter();
        let transfer_id = transfer_id.to_string();
        let reversal_id = reversal_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let transfer_id = transfer_id.clone();
            let reversal_id = reversal_id.clone();
            async move {
                let client = reqwest::Client::new();
                let response = client
                    .get(&format!("https://api.stripe.com/v1/transfers/{}/reversals/{}", transfer_id, reversal_id))
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                let reversal: TransferReversal = response.json().await?;
                Ok(reversal)
            }
        }).await
    }

    /// List all reversals for a transfer
    pub fn list_reversals(auth: &Auth, transfer_id: &str, limit: Option<u32>) -> Result<Vec<TransferReversal>> {
        let client = reqwest::blocking::Client::new();
        let mut url = format!("https://api.stripe.com/v1/transfers/{}/reversals", transfer_id);
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        #[derive(Deserialize)]
        struct ReversalList {
            data: Vec<TransferReversal>,
        }
        
        let list: ReversalList = response.json()?;
        Ok(list.data)
    }

    /// List all reversals for a transfer (async)
    pub async fn list_reversals_async(auth: &Auth, transfer_id: &str, limit: Option<u32>) -> Result<Vec<TransferReversal>> {
        let rate_limiter = get_rate_limiter();
        let transfer_id = transfer_id.to_string();
        
        rate_limiter.execute_with_retry_async("stripe", move || {
            let transfer_id = transfer_id.clone();
            async move {
                let client = reqwest::Client::new();
                let mut url = format!("https://api.stripe.com/v1/transfers/{}/reversals", transfer_id);
                if let Some(limit) = limit {
                    url = format!("{}?limit={}", url, limit);
                }
                
                let response = client
                    .get(&url)
                    .header("Authorization", format!("Bearer {}", auth.secret))
                    .send()
                    .await?;
                
                #[derive(Deserialize)]
                struct ReversalList {
                    data: Vec<TransferReversal>,
                }
                
                let list: ReversalList = response.json().await?;
                Ok(list.data)
            }
        }).await
    }
}