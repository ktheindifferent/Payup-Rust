use serde::{Deserialize, Serialize};

use crate::stripe::auth::Auth;
use crate::http_client::{get_shared_client, get_shared_blocking_client};

/// Represents your Stripe balance.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub object: String,
    pub available: Vec<BalanceAvailable>,
    pub livemode: bool,
    pub pending: Vec<BalancePending>,
}

impl Balance {
    /// Asynchronously retrieves the current account balance based on the authentication that was used to make the request.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch balance
    /// let balance = payup::stripe::Balance::async_get(auth).await;
    /// ```ignore
    pub async fn async_get(creds: Auth) -> Result<Self, reqwest::Error> {
        let url = "https://api.stripe.com/v1/balance";
        let request = get_shared_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Retrieves the current account balance based on the authentication that was used to make the request.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch balance
    /// let balance = payup::stripe::Balance::get(auth);
    /// ```ignore
    pub fn get(creds: Auth) -> Result<Self, reqwest::Error> {
        let url = "https://api.stripe.com/v1/balance";
        let request = get_shared_blocking_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }
}

/// Represents funds moving through your Stripe account.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceTransaction {
    pub id: String,
    pub object: String,
    pub amount: i64,
    #[serde(rename = "available_on")]
    pub available_on: i64,
    pub created: i64,
    pub currency: String,
    pub description: String,
    // #[serde(rename = "exchange_rate")]
    // pub exchange_rate: Value,
    pub fee: i64,
    #[serde(rename = "fee_details")]
    pub fee_details: Vec<FeeDetail>,
    pub net: i64,
    #[serde(rename = "reporting_category")]
    pub reporting_category: String,
    pub source: String,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl BalanceTransaction {
    /// Asynchronously retrieves the balance transaction with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe transaction balance id
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Retrieve the balance transaction with the given ID.
    /// let balance_transaction = payup::stripe::BalanceTransaction::async_get(auth, "txn_test123".to_string()).await;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/balance_transactions/{}", id);
        let request = get_shared_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously lists all balance transactions
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // List all balance transactions.
    /// let balance_transactions = payup::stripe::BalanceTransaction::async_list(auth).await;
    /// ```ignore
    pub async fn async_list(creds: Auth) -> Result<Vec<Self>, reqwest::Error> {
        let mut objects = Vec::new();
        let mut has_more = true;
        let mut starting_after = None;

        while has_more {
            let json = Self::list_chunk_async(creds.clone(), starting_after.clone()).await?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = objects.last().map(|obj| obj.id.clone());
        }
        Ok(objects)
    }

    /// Retrieves the balance transaction with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe transaction balance id
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Retrieve the balance transaction with the given ID.
    /// let balance_transaction = payup::stripe::BalanceTransaction::get(auth, "txn_test123".to_string());
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/balance_transactions/{}", id);
        let request = get_shared_blocking_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Lists all balance transactions
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignoreno_run
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // List all balance transactions.
    /// let balance_transactions = payup::stripe::BalanceTransaction::list(auth);
    /// ```ignore
    pub fn list(creds: Auth) -> Result<Vec<Self>, reqwest::Error> {
        let mut objects = Vec::new();
        let mut has_more = true;
        let mut starting_after = None;

        while has_more {
            let json = Self::list_chunk(creds.clone(), starting_after.clone())?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = objects.last().map(|obj| obj.id.clone());
        }
        Ok(objects)
    }

    fn list_chunk(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<BalanceTransactions, reqwest::Error> {
        let url = match starting_after {
            Some(ref id) => format!(
                "https://api.stripe.com/v1/balance_transactions?starting_after={}",
                id
            ),
            None => "https://api.stripe.com/v1/balance_transactions".to_string(),
        };

        let request = get_shared_blocking_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<BalanceTransactions>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<BalanceTransactions, reqwest::Error> {
        let url = match starting_after {
            Some(ref id) => format!(
                "https://api.stripe.com/v1/balance_transactions?starting_after={}",
                id
            ),
            None => "https://api.stripe.com/v1/balance_transactions".to_string(),
        };

        let request = get_shared_client()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<BalanceTransactions>().await?;
        Ok(json)
    }
}

// Supporting types for Balance and BalanceTransaction

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct BalanceAvailable {
    pub amount: i64,
    pub currency: String,
    #[serde(rename = "source_types")]
    pub source_types: BalanceSourceTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct BalanceSourceTypes {
    pub card: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct BalancePending {
    pub amount: i64,
    pub currency: String,
    #[serde(rename = "source_types")]
    pub source_types: BalanceSourceTypes,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct BalanceTransactions {
    pub object: String,
    pub data: Vec<BalanceTransaction>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct FeeDetail {
    pub amount: i64,
    // pub application: Value,
    pub currency: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
}