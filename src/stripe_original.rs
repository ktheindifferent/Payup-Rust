pub mod response;

// Full V1 API Support Complete
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
        let request = reqwest::Client::new()
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
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }
}

// Full V1 API Support Complete
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
        let request = reqwest::Client::new()
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
        let request = reqwest::blocking::Client::new()
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

        let request = reqwest::blocking::Client::new()
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

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<BalanceTransactions>().await?;
        Ok(json)
    }
}

// TODO - Finish Implementation
/// You can store multiple cards on a customer in order to charge the customer later.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: Option<String>,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub number: Option<String>,
    pub cvc: Option<String>,
    pub network: Option<String>,
    pub country: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub fingerprint: Option<String>,
}
impl Card {
    pub fn new() -> Self {
        Card {
            id: None,
            brand: None,
            last4: None,
            number: None,
            cvc: None,
            network: None,
            country: None,
            exp_month: None,
            exp_year: None,
            fingerprint: None,
        }
    }
}

// Full V1 API Support Complete
/// Represents a charge to a credit or a debit card.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Charge {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: Option<String>,
    #[serde(rename = "amount")]
    pub stripe_amount: Option<i64>,
    #[serde(rename = "amount_captured")]
    pub amount_captured: Option<i64>,
    #[serde(rename = "amount_refunded")]
    pub amount_refunded: Option<i64>,
    #[serde(rename = "balance_transaction")]
    pub balance_transaction: Option<String>,
    #[serde(rename = "billing_details")]
    pub billing_details: Option<BillingDetails>,
    pub captured: Option<bool>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub disputed: Option<bool>,
    #[serde(rename = "fraud_details")]
    pub fraud_details: Option<FraudDetails>,
    pub livemode: Option<bool>,
    // pub metadata: Metadata,
    pub paid: Option<bool>,
    #[serde(rename = "payment_method")]
    pub payment_method: Option<String>,
    #[serde(rename = "payment_method_details")]
    pub payment_method_details: Option<PaymentMethodDetails>,
    #[serde(rename = "receipt_url")]
    pub receipt_url: Option<String>,
    pub refunded: Option<bool>,
    pub refunds: Option<Refunds>,
    pub status: Option<String>,
    // #[serde(rename = "calculated_statement_descriptor")]
    // pub calculated_statement_descriptor: Value,
    pub customer: Option<String>,
    // pub invoice: Value,
    // #[serde(rename = "failure_code")]
    // pub failure_code: Value,
    // #[serde(rename = "failure_message")]
    // pub failure_message: Value,
    // #[serde(rename = "on_behalf_of")]
    // pub on_behalf_of: Value,
    // pub order: Value,
    // pub outcome: Value,
    // #[serde(rename = "payment_intent")]
    // pub payment_intent: Value,
    #[serde(rename = "receipt_email")]
    pub receipt_email: Option<String>,
    pub source: Option<String>,
    // #[serde(rename = "receipt_number")]
    // pub receipt_number: Value,
    // pub review: Value,
    // pub shipping: Value,
    // #[serde(rename = "source_transfer")]
    // pub source_transfer: Value,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Option<String>,
    #[serde(rename = "statement_descriptor_suffix")]
    pub statement_descriptor_suffix: Option<String>,
    // #[serde(rename = "transfer_data")]
    // pub transfer_data: Value,
    // #[serde(rename = "transfer_group")]
    // pub transfer_group: Value,
    // pub application: Value,
    // #[serde(rename = "application_fee")]
    // pub application_fee: Value,
    // #[serde(rename = "application_fee_amount")]
    // pub application_fee_amount: Value,
}
impl Charge {
    /// Returns an empty Charge object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    /// ```ignore
    pub fn new() -> Self {
        return Charge {
            id: None,
            object: None,
            amount: None,
            stripe_amount: None,
            amount_captured: None,
            amount_refunded: None,
            balance_transaction: None,
            billing_details: None,
            captured: None,
            created: None,
            currency: None,
            customer: None,
            description: None,
            disputed: None,
            fraud_details: None,
            livemode: None,
            paid: None,
            payment_method: None,
            payment_method_details: None,
            receipt_url: None,
            refunded: None,
            refunds: None,
            status: None,
            source: None,
            receipt_email: None,
            statement_descriptor: None,
            statement_descriptor_suffix: None,
        };
    }

    /// Asynchronously capture the payment of an existing, uncaptured, charge.
    /// This is the second half of the two-step payment flow, where first you created a charge with the capture option set to false.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.async_post(auth.clone()).await?;
    ///
    /// // Fetch customer using id
    /// let captured_charge = charge.async_capture(auth.clone()).await?;
    /// ```ignore
    pub async fn async_capture(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let url = format!(
            "https://api.stripe.com/v1/charges/{}/capture",
            self.id.clone().ok_or_else(|| crate::error::PayupError::ValidationError("Charge ID is required for capture".to_string()))?
        );

        let request = reqwest::Client::new()
            .post(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_capture_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously retrieves the details of a charge that has previously been created.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the charge you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let charge = payup::stripe::Charge::async_get(auth, "ch_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/charges/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe charges.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let charges = payup::stripe::Charge::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Asynchronously POSTs a new Charge to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.async_post(auth.clone()).await?;
    /// ```ignore
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/charges")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously POSTs an update to an existing Charge
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.async_post(auth.clone()).await?;
    ///
    /// charge.receipt_email = Some("testchanged@test.com".to_string());
    /// charge = charge.async_update(auth.clone()).await?;
    /// ```ignore
    pub async fn async_update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/charges/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Charge ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Capture the payment of an existing, uncaptured, charge.
    /// This is the second half of the two-step payment flow, where first you created a charge with the capture option set to false.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.post(auth.clone());
    ///
    /// // Fetch customer using id
    /// let captured_charge = charge.capture(auth.clone())?;
    /// ```ignore
    pub fn capture(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let url = format!(
            "https://api.stripe.com/v1/charges/{}/capture",
            self.id.clone().ok_or_else(|| crate::error::PayupError::ValidationError("Charge ID is required for capture".to_string()))?
        );

        let request = reqwest::blocking::Client::new()
            .post(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_capture_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Retrieves the details of a charge that has previously been created.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the charge you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let charge = payup::stripe::Charge::get(auth, "ch_test123".to_string());
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/charges/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all stripe charges.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let charges = payup::stripe::Charge::list(auth)?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// POSTs a new Charge to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.post(auth.clone())?;
    /// ```ignore
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/charges")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// POSTs an update to an existing Charge
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut charge = payup::stripe::Charge::new();
    /// charge.amount = Some(100);
    /// charge.currency = Some("usd".to_string());
    /// charge.customer = Some("cust_test123".to_string());
    /// charge.description = Some("test charge".to_string());
    /// charge.receipt_email = Some("test@test.com".to_string());
    /// charge.source = Some("card_test123".to_string());
    ///
    /// charge = charge.async_post(auth.clone()).await?;
    ///
    /// charge.receipt_email = Some("testchanged@test.com".to_string());
    /// charge = charge.update(auth.clone()).await?;
    /// ```ignore
    pub fn update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/charges/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Charge ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    fn list_chunk(creds: Auth, starting_after: Option<String>) -> Result<Charges, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/charges".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/charges?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Charges>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Charges, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/charges".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/charges?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Charges>().await?;
        Ok(json)
    }
    fn to_capture_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];

        match &self.receipt_email {
            Some(receipt_email) => params.push(("receipt_email", receipt_email.as_str())),
            None => {}
        }
        match &self.amount {
            Some(amount) => params.push(("amount", amount.as_str())),
            None => {}
        }
        match &self.statement_descriptor {
            Some(statement_descriptor) => {
                params.push(("statement_descriptor", statement_descriptor.as_str()))
            }
            None => {}
        }
        match &self.statement_descriptor_suffix {
            Some(statement_descriptor_suffix) => params.push((
                "statement_descriptor_suffix",
                statement_descriptor_suffix.as_str(),
            )),
            None => {}
        }
        return params;
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];
        match &self.customer {
            Some(customer) => params.push(("customer", customer.as_str())),
            None => {}
        }
        match &self.description {
            Some(description) => params.push(("description", description.as_str())),
            None => {}
        }
        match &self.receipt_email {
            Some(receipt_email) => params.push(("receipt_email", receipt_email.as_str())),
            None => {}
        }
        match &self.amount {
            Some(amount) => params.push(("amount", amount.as_str())),
            None => {}
        }
        match &self.currency {
            Some(currency) => params.push(("currency", currency.as_str())),
            None => {}
        }
        // TODO - Impliment Shipping
        // match &self.shipping{
        //     Some(shipping) => params.push(("shipping", shipping.as_str())),
        //     None => {}
        // }
        match &self.source {
            Some(source) => params.push(("source", source.as_str())),
            None => {}
        }
        match &self.statement_descriptor {
            Some(statement_descriptor) => {
                params.push(("statement_descriptor", statement_descriptor.as_str()))
            }
            None => {}
        }
        match &self.statement_descriptor_suffix {
            Some(statement_descriptor_suffix) => params.push((
                "statement_descriptor_suffix",
                statement_descriptor_suffix.as_str(),
            )),
            None => {}
        }
        return params;
    }
}

/// Represents a customer of your business.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: Option<String>,
    pub object: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub address: Value,
    pub balance: Option<i64>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    #[serde(rename = "default_source")]
    pub default_source: Option<String>,
    pub payment_method: Option<String>,
    pub delinquent: Option<bool>,
    pub description: Option<String>,
    // pub discount: Value,
    pub email: Option<String>,
    #[serde(rename = "invoice_prefix")]
    pub invoice_prefix: Option<String>,
    // #[serde(rename = "invoice_settings")]
    // pub invoice_settings: InvoiceSettings,
    pub livemode: Option<bool>,
    // pub metadata: Metadata,
    pub name: Option<String>,
    #[serde(rename = "next_invoice_sequence")]
    pub next_invoice_sequence: Option<i64>,
    pub phone: Option<String>,
    // #[serde(rename = "preferred_locales")]
    // pub preferred_locales: Vec<Value>,
    // pub shipping: Value,
    #[serde(rename = "tax_exempt")]
    pub tax_exempt: Option<String>,
}
impl Customer {
    /// Returns an empty Customer object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    /// ```ignore
    pub fn new() -> Self {
        return Customer {
            id: None,
            object: None,
            balance: None,
            created: None,
            currency: None,
            default_source: None,
            payment_method: None,
            delinquent: None,
            description: None,
            email: None,
            invoice_prefix: None,
            livemode: None,
            name: None,
            next_invoice_sequence: None,
            phone: None,
            tax_exempt: None,
        };
    }

    /// Asynchronously destroy a stripe Customer
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::async_delete(auth, "cust_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_delete(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/customers/{}", id);

        let request = reqwest::Client::new()
            .delete(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously lookup a stripe Customer using customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::async_get(auth, "cust_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/customers/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all Invoices belonging to the customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `customer_id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let customers_invoices = payup::stripe::Customer::invoices(auth, "cust_test123".to_string()).await?;     
    /// ```ignore    
    pub async fn async_invoices(
        creds: Auth,
        customer_id: String,
    ) -> Result<Vec<crate::stripe::response::Invoice>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::Invoice> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::get_invoices_chunk(
                creds.clone(),
                customer_id.clone(),
                starting_after.clone(),
            )?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        Ok(objects)
    }

    /// Asynchronously returns all stripe customers owned by the account.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let customers = payup::stripe::Customer::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Asynchronously returns all PaymentMethods belonging to the customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `customer_id` - A string representing an existing stripe customer_id
    /// * `method_type` - A string representing the type of payment method (acss_debit, afterpay_clearpay, alipay, au_becs_debit, bacs_debit, bancontact, boleto, card, eps, fpx, giropay, grabpay, ideal, klarna, oxxo, p24, sepa_debit, sofort, wechat_pay)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let customers_payment_methods = payup::stripe::Customer::async_payment_methods(auth, "cust_test123".to_string(), "card".to_string()).await?;     
    /// ```ignore
    pub async fn async_payment_methods(
        creds: Auth,
        customer_id: String,
        method_type: String,
    ) -> Result<Vec<crate::stripe::response::PaymentMethod>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::PaymentMethod> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::get_payment_methods_chunk_async(
                creds.clone(),
                customer_id.clone(),
                method_type.clone(),
                starting_after.clone(),
            )
            .await?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        Ok(objects)
    }

    /// Asynchronously POSTs a new customer to the stripe api
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Build a customer object
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    ///
    /// // Post customer to stripe and update the local cust variable
    /// let customer = cust.async_post(auth).await?;
    /// ```ignore
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/customers")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await;
        match request {
            Ok(req) => {
                let json = req.json::<Self>().await?;

                Ok(json)
            }
            Err(err) => Err(err),
        }
    }

    /// Asynchronously POSTs updates to an existing stripe Customer
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Build a customer object
    /// let mut customer = payup::stripe::Customer::new();
    /// customer.name = Some("Rust Test".to_string());
    /// customer.description = Some("A test customer from rust.".to_string());
    /// customer.phone = Some("333-333-3333".to_string());
    /// customer.email = Some("rust@test.com".to_string());
    /// customer.payment_method = None;
    ///
    /// // Post customer to stripe and update the local cust variable
    /// customer = cust.async_post(auth).await?;
    ///
    /// // Makes changes
    /// customer.email = Some("RustNewEmail@test.com".to_string());
    ///
    /// // Update customer
    /// customer = cust.async_update(auth).await?;
    /// ```ignore
    pub async fn async_update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/customers/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Customer ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Destroy a stripe Customer
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::async_delete(auth, "cust_test123".to_string()).await?;
    /// ```ignore
    pub fn delete(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/customers/{}", id);

        let request = reqwest::blocking::Client::new()
            .delete(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Lookup a stripe Customer using customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let customer = payup::stripe::Customer::get(auth, "cust_test123".to_string())?;
    /// ```ignore
    pub fn get(auth: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/customers/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(auth.client.as_str(), Some(auth.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all Invoices belonging to the customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - A string representing an existing stripe customer_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let customers_invoices = payup::stripe::Customer::invoices(auth, "cust_test123".to_string())?;     
    /// ```ignore    
    pub fn invoices(
        creds: Auth,
        customer_id: String,
    ) -> Result<Vec<crate::stripe::response::Invoice>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::Invoice> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::get_invoices_chunk(
                creds.clone(),
                customer_id.clone(),
                starting_after.clone(),
            )?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        Ok(objects)
    }

    /// Returns all stripe customers
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let customers = payup::stripe::Customer::list(auth.clone())?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Returns all PaymentMethods belonging to the customer_id
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `customer_id` - A string representing an existing stripe customer_id
    /// * `method_type` - A string representing the type of payment method (acss_debit, afterpay_clearpay, alipay, au_becs_debit, bacs_debit, bancontact, boleto, card, eps, fpx, giropay, grabpay, ideal, klarna, oxxo, p24, sepa_debit, sofort, wechat_pay)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let customers_payment_methods = payup::stripe::Customer::payment_methods(auth, "cust_test123".to_string(), "card".to_string())?;     
    /// ```ignore
    pub fn payment_methods(
        creds: Auth,
        customer_id: String,
        method_type: String,
    ) -> Result<Vec<crate::stripe::response::PaymentMethod>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::PaymentMethod> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::get_payment_methods_chunk(
                creds.clone(),
                customer_id.clone(),
                method_type.clone(),
                starting_after.clone(),
            )?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }
        Ok(objects)
    }

    /// POSTs a new customer to the stripe api
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Build a customer object
    /// let mut cust = payup::stripe::Customer::new();
    /// cust.name = Some("Rust Test".to_string());
    /// cust.description = Some("A test customer from rust.".to_string());
    /// cust.phone = Some("333-333-3333".to_string());
    /// cust.email = Some("rust@test.com".to_string());
    /// cust.payment_method = None;
    ///
    /// // Post customer to stripe and update the local cust variable
    /// let customer = cust.post(auth).unwrap();
    /// ```ignore
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/customers")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<Self>()?;
                Ok(json)
            }
            Err(err) => Err(err),
        }
    }

    /// POSTs updates to an existing stripe Customer
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Build a customer object
    /// let mut customer = payup::stripe::Customer::new();
    /// customer.name = Some("Rust Test".to_string());
    /// customer.description = Some("A test customer from rust.".to_string());
    /// customer.phone = Some("333-333-3333".to_string());
    /// customer.email = Some("rust@test.com".to_string());
    /// customer.payment_method = None;
    ///
    /// // Post customer to stripe and update the local cust variable
    /// customer = cust.async_post(auth)?;
    ///
    /// // Makes changes
    /// customer.email = Some("RustNewEmail@test.com".to_string());
    ///
    /// // Update customer
    /// customer = cust.update(auth)?;
    /// ```ignore
    pub fn update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/customers/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Customer ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    fn get_invoices_chunk(
        creds: Auth,
        customer_id: String,
        starting_after: Option<String>,
    ) -> Result<crate::stripe::response::Invoices, reqwest::Error> {
        let url = if let Some(ref after) = starting_after {
            format!(
                "https://api.stripe.com/v1/invoices?customer={}&starting_after={}",
                customer_id, after
            )
        } else {
            format!(
                "https://api.stripe.com/v1/invoices?customer={}",
                customer_id
            )
        };

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<crate::stripe::response::Invoices>()?;
        Ok(json)
    }

    fn list_chunk(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/customers?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Customers>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/customers?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Customers>().await?;
        Ok(json)
    }

    fn get_payment_methods_chunk(
        creds: Auth,
        customer_id: String,
        method_type: String,
        starting_after: Option<String>,
    ) -> Result<crate::stripe::response::PaymentMethods, reqwest::Error> {
        let url = if let Some(ref after) = starting_after {
            format!(
                "https://api.stripe.com/v1/customers/{}/payment_methods?type={}&starting_after={}",
                customer_id, method_type, after
            )
        } else {
            format!(
                "https://api.stripe.com/v1/customers/{}/payment_methods?type={}",
                customer_id, method_type
            )
        };

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<crate::stripe::response::PaymentMethods>()?;
        Ok(json)
    }

    async fn get_payment_methods_chunk_async(
        creds: Auth,
        customer_id: String,
        method_type: String,
        starting_after: Option<String>,
    ) -> Result<crate::stripe::response::PaymentMethods, reqwest::Error> {
        let url = if let Some(ref after) = starting_after {
            format!(
                "https://api.stripe.com/v1/customers/{}/payment_methods?type={}&starting_after={}",
                customer_id, method_type, after
            )
        } else {
            format!(
                "https://api.stripe.com/v1/customers/{}/payment_methods?type={}",
                customer_id, method_type
            )
        };

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request
            .json::<crate::stripe::response::PaymentMethods>()
            .await?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.payment_method {
            Some(payment_method) => params.push(("payment_method", payment_method.as_str())),
            None => {}
        }
        match &self.description {
            Some(description) => params.push(("description", description.as_str())),
            None => {}
        }
        match &self.email {
            Some(email) => params.push(("email", email.as_str())),
            None => {}
        }
        match &self.name {
            Some(name) => params.push(("name", name.as_str())),
            None => {}
        }
        match &self.phone {
            Some(phone) => params.push(("phone", phone.as_str())),
            None => {}
        }
        return params;
    }
}

/// Represents a charge to a credit or a debit card.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dispute {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: Option<i64>,
    // #[serde(rename = "balance_transactions")]
    // pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub evidence: Option<Evidence>,
    #[serde(rename = "evidence_details")]
    pub evidence_details: Option<EvidenceDetails>,
    #[serde(rename = "is_charge_refundable")]
    pub is_charge_refundable: Option<bool>,
    pub livemode: Option<bool>,
    pub submit: Option<bool>,
    // pub metadata: Metadata,
    #[serde(rename = "payment_intent")]
    pub payment_intent: Option<String>,
    pub reason: Option<String>,
    pub status: Option<String>,
}
impl Dispute {
    /// Returns an empty Dispute object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut dispute = payup::stripe::Dispute::new();
    /// dispute.amount = Some(100);
    /// ```ignore
    pub fn new() -> Self {
        return Dispute {
            id: None,
            object: None,
            amount: None,
            charge: None,
            created: None,
            currency: None,
            // balance_transactions: None,
            // captured: None,
            evidence: None,
            evidence_details: None,
            is_charge_refundable: None,
            livemode: None,
            submit: None,
            payment_intent: None,
            reason: None,
            status: None,
        };
    }

    /// Asynchronously close a dispute.
    /// Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut dispute = payup::stripe::Dispute::new();
    /// dispute.id = Some(format!("dp_"));
    ///
    /// dispute = dispute.async_close(auth.clone()).await?;
    /// ```ignore
    pub async fn async_close(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/disputes/{}/close",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Dispute ID is required for close".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously retrieves the dispute with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the dispute you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let dispute = payup::stripe::Dispute::async_get(auth, "ch_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/disputes/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe Disputes.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let disputes = payup::stripe::Dispute::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Asynchronously POSTs an update to an existing Dispute
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Crate some evidence to update the dispute with
    /// let mut evidence = payup::stripe::Evidence::new();
    /// evidence.billing_address = Some(format!(""));
    /// evidence.cancellation_policy = Some(format!(""));
    ///
    /// let mut dispute = payup::stripe::Dispute::new();
    /// dispute.id = Some(format!("dp_"));
    /// dispute.evidence = Some(evidence);
    ///
    /// // Submit the evidence to the bank
    /// dispute.submit = Some(true);
    ///
    /// // Update the dispute
    /// dispute = dispute.async_update(auth.clone()).await?;
    /// ```ignore
    pub async fn async_update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/disputes/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Dispute ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Close a dispute.
    /// Closing the dispute for a charge indicates that you do not have any evidence to submit and are essentially dismissing the dispute, acknowledging it as lost.
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut dispute = payup::stripe::Dispute::new();
    /// dispute.id = Some(format!("dp_"));
    ///
    /// dispute = dispute.async_close(auth.clone())?;
    /// ```ignore
    pub fn close(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/disputes/{}/close",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Dispute ID is required for close".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Retrieves the dispute with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the dispute you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let dispute = payup::stripe::Dispute::get(auth, "ch_test123".to_string())?;
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/disputes/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all stripe disputes.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let charges = payup::stripe::Dispute::list(auth)?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// POSTs an update to an existing Dispute
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Crate some evidence to update the dispute with
    /// let mut evidence = payup::stripe::Evidence::new();
    /// evidence.billing_address = Some(format!(""));
    /// evidence.cancellation_policy = Some(format!(""));
    ///
    /// let mut dispute = payup::stripe::Dispute::new();
    /// dispute.id = Some(format!("dp_"));
    /// dispute.evidence = Some(evidence);
    ///
    /// // Submit the evidence to the bank
    /// dispute.submit = Some(true);
    ///
    /// // Update the dispute
    /// dispute = dispute.async_update(auth.clone())?;
    /// ```ignore
    pub fn update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/disputes/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Dispute ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    fn list_chunk(creds: Auth, starting_after: Option<String>) -> Result<Disputes, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/disputes".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/disputes?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Disputes>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Disputes, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/disputes".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/disputes?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Disputes>().await?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];
        match &self.evidence {
            Some(evidence) => {
                match &evidence.access_activity_log {
                    Some(access_activity_log) => {
                        params.push((
                            "evidence[access_activity_log]",
                            access_activity_log.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.billing_address {
                    Some(billing_address) => {
                        params.push(("evidence[billing_address]", billing_address.as_str()));
                    }
                    None => {}
                }

                match &evidence.cancellation_policy {
                    Some(cancellation_policy) => {
                        params.push((
                            "evidence[cancellation_policy]",
                            cancellation_policy.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.cancellation_policy_disclosure {
                    Some(cancellation_policy_disclosure) => {
                        params.push((
                            "evidence[cancellation_policy_disclosure]",
                            cancellation_policy_disclosure.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.cancellation_rebuttal {
                    Some(cancellation_rebuttal) => {
                        params.push((
                            "evidence[cancellation_rebuttal]",
                            cancellation_rebuttal.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.customer_communication {
                    Some(customer_communication) => {
                        params.push((
                            "evidence[customer_communication]",
                            customer_communication.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.customer_email_address {
                    Some(customer_email_address) => {
                        params.push((
                            "evidence[customer_email_address]",
                            customer_email_address.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.customer_name {
                    Some(customer_name) => {
                        params.push(("evidence[customer_name]", customer_name.as_str()));
                    }
                    None => {}
                }

                match &evidence.customer_purchase_ip {
                    Some(customer_purchase_ip) => {
                        params.push((
                            "evidence[customer_purchase_ip]",
                            customer_purchase_ip.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.customer_signature {
                    Some(customer_signature) => {
                        params.push(("evidence[customer_signature]", customer_signature.as_str()));
                    }
                    None => {}
                }

                match &evidence.duplicate_charge_documentation {
                    Some(duplicate_charge_documentation) => {
                        params.push((
                            "evidence[duplicate_charge_documentation]",
                            duplicate_charge_documentation.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.duplicate_charge_explanation {
                    Some(duplicate_charge_explanation) => {
                        params.push((
                            "evidence[duplicate_charge_explanation]",
                            duplicate_charge_explanation.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.duplicate_charge_id {
                    Some(duplicate_charge_id) => {
                        params.push((
                            "evidence[duplicate_charge_id]",
                            duplicate_charge_id.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.product_description {
                    Some(product_description) => {
                        params.push((
                            "evidence[product_description]",
                            product_description.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.receipt {
                    Some(receipt) => {
                        params.push(("evidence[receipt]", receipt.as_str()));
                    }
                    None => {}
                }

                match &evidence.refund_policy {
                    Some(refund_policy) => {
                        params.push(("evidence[refund_policy]", refund_policy.as_str()));
                    }
                    None => {}
                }

                match &evidence.refund_policy_disclosure {
                    Some(refund_policy_disclosure) => {
                        params.push((
                            "evidence[refund_policy_disclosure]",
                            refund_policy_disclosure.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.refund_refusal_explanation {
                    Some(refund_refusal_explanation) => {
                        params.push((
                            "evidence[refund_refusal_explanation]",
                            refund_refusal_explanation.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.service_date {
                    Some(service_date) => {
                        params.push(("evidence[service_date]", service_date.as_str()));
                    }
                    None => {}
                }

                match &evidence.service_documentation {
                    Some(service_documentation) => {
                        params.push((
                            "evidence[service_documentation]",
                            service_documentation.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.shipping_address {
                    Some(shipping_address) => {
                        params.push(("evidence[shipping_address]", shipping_address.as_str()));
                    }
                    None => {}
                }

                match &evidence.shipping_carrier {
                    Some(shipping_carrier) => {
                        params.push(("evidence[shipping_carrier]", shipping_carrier.as_str()));
                    }
                    None => {}
                }

                match &evidence.shipping_date {
                    Some(shipping_date) => {
                        params.push(("evidence[shipping_date]", shipping_date.as_str()));
                    }
                    None => {}
                }

                match &evidence.shipping_documentation {
                    Some(shipping_documentation) => {
                        params.push((
                            "evidence[shipping_documentation]",
                            shipping_documentation.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.shipping_tracking_number {
                    Some(shipping_tracking_number) => {
                        params.push((
                            "evidence[shipping_tracking_number]",
                            shipping_tracking_number.as_str(),
                        ));
                    }
                    None => {}
                }

                match &evidence.uncategorized_file {
                    Some(uncategorized_file) => {
                        params.push(("evidence[uncategorized_file]", uncategorized_file.as_str()));
                    }
                    None => {}
                }

                match &evidence.uncategorized_text {
                    Some(uncategorized_text) => {
                        params.push(("evidence[uncategorized_text]", uncategorized_text.as_str()));
                    }
                    None => {}
                }
            }
            None => {}
        }
        // "metadata[order_id]"=6735
        // match &self.metadata{
        //     Some(metadata) => params.push(("metadata", metadata.as_str())),
        //     None => {}
        // }
        match &self.submit {
            Some(submit) => {
                if *submit {
                    params.push(("submit", "true"));
                } else {
                    params.push(("submit", "false"));
                }
            }
            None => {}
        }
        return params;
    }
}

// TODO - Impliment data/object. This can be any stripe object so it's best to build out the other structs before implementing.
/// Events occur when the state of another API resource changes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: Option<String>,
    pub object: Option<String>,
    #[serde(rename = "api_version")]
    pub api_version: Option<String>,
    pub created: Option<i64>,
    // pub data: Option<Data>,
    pub livemode: Option<bool>,
    #[serde(rename = "pending_webhooks")]
    pub pending_webhooks: Option<i64>,
    pub request: Option<Request>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}
impl Event {
    /// Asynchronously retrieves the event with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the event you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let event = payup::stripe::Event::async_get(auth, "ch_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/events/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe Events.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let events = payup::stripe::Event::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Retrieves the event with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the event you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let event = payup::stripe::Event::get(auth, "ch_test123".to_string())?;
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/events/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all stripe events.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let events = payup::stripe::Event::list(auth)?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    fn list_chunk(creds: Auth, starting_after: Option<String>) -> Result<Events, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/events".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/events?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Events>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Events, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/events".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/events?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Events>().await?;
        Ok(json)
    }
}

/// Represents a file hosted on Stripe's servers.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i64>,
    pub filename: Option<String>,
    pub links: Option<Links>,
    pub purpose: Option<String>,
    pub size: Option<i64>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub url: Option<String>,
    pub file: Option<Vec<u8>>,
}
impl File {
    /// Returns an empty File object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut file = payup::stripe::File::new();
    /// file.title = Some(format!("Title"));
    /// ```ignore
    pub fn new() -> Self {
        return File {
            id: None,
            object: None,
            created: None,
            expires_at: None,
            filename: None,
            links: None,
            purpose: None,
            size: None,
            title: None,
            type_field: None,
            url: None,
            file: None,
        };
    }

    /// Asynchronously retrieves a file with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the file you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let file = payup::stripe::File::async_get(auth, "ch_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/files/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe Files.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let disputes = payup::stripe::Dispute::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Asynchronously POSTs a new File to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file = payup::stripe::File::new();
    /// ley bytes: Vec<u8> = Vec::new();
    /// charge.amount = Some(100);
    /// file.file = Some(bytes);
    ///
    /// // The purpose of the uploaded file.
    /// // Possible enum values: account_requirement, additional_verification, business_icon, business_logo, customer_signature, dispute_evidence, identity_document, pci_document, tax_document_user_upload
    /// file.purpose = Some("cust_test123".to_string());
    ///
    /// file = file.post(auth.clone()).await?;
    /// ```ignore
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let form = self.to_multipart_form_async().await;

        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/files")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .multipart(form)
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// POSTs a new File to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file = payup::stripe::File::new();
    /// ley bytes: Vec<u8> = Vec::new();
    /// charge.amount = Some(100);
    /// file.file = Some(bytes);
    ///
    /// // The purpose of the uploaded file.
    /// // Possible enum values: account_requirement, additional_verification, business_icon, business_logo, customer_signature, dispute_evidence, identity_document, pci_document, tax_document_user_upload
    /// file.purpose = Some("cust_test123".to_string());
    ///
    /// file = file.post(auth.clone())?;
    /// ```ignore
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let form = self.to_multipart_form();

        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/files")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .multipart(form)
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Retrieves the dispute with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the dispute you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let dispute = payup::stripe::Dispute::get(auth, "ch_test123".to_string())?;
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/files/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all stripe disputes.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let charges = payup::stripe::Dispute::list(auth)?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    fn list_chunk(creds: Auth, starting_after: Option<String>) -> Result<Files, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/files".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/files?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Files>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Files, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/files".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/files?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Files>().await?;
        Ok(json)
    }

    fn to_multipart_form(&self) -> reqwest::blocking::multipart::Form {
        let mut form = reqwest::blocking::multipart::Form::new();

        match &self.purpose {
            Some(purpose) => {
                form = form.text("purpose", purpose.clone());
            }
            None => {}
        }

        match &self.file {
            Some(file) => {
                let part = reqwest::blocking::multipart::Part::bytes(file.clone());
                form = form.part("file", part);
            }
            None => {}
        }

        return form;
    }

    async fn to_multipart_form_async(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        match &self.purpose {
            Some(purpose) => {
                form = form.text("purpose", purpose.clone());
            }
            None => {}
        }

        match &self.file {
            Some(file) => {
                let part = reqwest::multipart::Part::bytes(file.clone());
                form = form.part("file", part);
            }
            None => {}
        }

        return form;
    }
}

/// To share the contents of a File object with non-Stripe users, you can create a FileLink
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileLink {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub expired: Option<bool>,
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i64>,
    pub link_expires_at: Option<String>,
    pub file: Option<String>,
    pub livemode: Option<bool>,
    // pub metadata: Metadata,
    pub url: Option<String>,
}
impl FileLink {
    /// Returns an empty FileLink object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut file_link = payup::stripe::FileLink::new();
    /// file_link.file = Some("file_test123".to_string());
    /// ```ignore
    pub fn new() -> Self {
        return FileLink {
            id: None,
            object: None,
            created: None,
            expired: None,
            expires_at: None,
            link_expires_at: None,
            file: None,
            livemode: None,
            url: None,
        };
    }

    /// Asynchronously retrieves a file link with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the file you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let file = payup::stripe::FileLink::async_get(auth, "link_").await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/file_links/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe FileLinks.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let file_links = payup::stripe::FileLink::async_list(auth).await?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// Asynchronously POSTs a new FileLink to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file_link = payup::stripe::FileLink::new();
    /// file_link.file = Some("file_test123".to_string());
    /// file_link.link_expires_at = Some(format!("1643341848"));
    ///
    /// file_link = file_link.async_post(auth.clone()).await?;
    /// ```ignore
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/file_links")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously POSTs an update to an existing FileLink
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file_link = payup::stripe::FileLink::new();
    /// file_link.file = Some("file_test123".to_string());
    ///
    /// file_link = file_link.async_post(auth.clone()).await?;
    ///
    /// file_link.link_expires_at = Some(format!("1643341848"));
    /// file_link = file_link.async_update(auth.clone()).await?;
    /// ```ignore
    pub async fn async_update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/file_links/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("FileLink ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Retrieves a file link with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the FileLink you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let file_link = payup::stripe::FileLink::get(auth, "ch_test123".to_string());
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/file_links/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Eeturns all stripe FileLinks.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let file_links = payup::stripe::FileLink::list(auth)?;
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
            if let Some(last) = objects.last() {
                starting_after = last.id.clone();
            }
        }
        Ok(objects)
    }

    /// POSTs a new FileLink to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file_link = payup::stripe::FileLink::new();
    /// file_link.file = Some("file_test123".to_string());
    /// file_link.link_expires_at = Some(format!("1643341848"));
    ///
    /// file_link = file_link.async_post(auth.clone())?;
    /// ```ignore
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/file_links")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// POSTs an update to an existing FileLink
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut file_link = payup::stripe::FileLink::new();
    /// file_link.file = Some("file_test123".to_string());
    ///
    /// file_link = file_link.post(auth.clone())?;
    ///
    /// file_link.link_expires_at = Some(format!("1643341848"));
    /// file_link = file_link.update(auth.clone())?;
    /// ```ignore
    pub fn update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/file_links/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("FileLink ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    fn list_chunk(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<FileLinks, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/file_links".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/file_links?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<FileLinks>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<FileLinks, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/file_links".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/file_links?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<FileLinks>().await?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];
        match &self.file {
            Some(file) => params.push(("file", file.as_str())),
            None => {}
        }
        match &self.link_expires_at {
            Some(link_expires_at) => params.push(("expires_at", link_expires_at.as_str())),
            None => {}
        }

        return params;
    }
}

// TODO - Finish Implementation
/// Invoices are statements of amounts owed by a customer.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: Option<String>,
    pub object: Option<String>,
    #[serde(rename = "account_country")]
    pub account_country: Option<String>,
    #[serde(rename = "account_name")]
    pub account_name: Option<String>,
    #[serde(rename = "account_tax_ids")]
    pub account_tax_ids: Option<String>,
    #[serde(rename = "amount_due")]
    pub amount_due: Option<i64>,
    #[serde(rename = "amount_paid")]
    pub amount_paid: Option<i64>,
    #[serde(rename = "amount_remaining")]
    pub amount_remaining: Option<i64>,
    #[serde(rename = "application_fee_amount")]
    pub application_fee_amount: Option<i64>,
    #[serde(rename = "attempt_count")]
    pub attempt_count: Option<i64>,
    pub attempted: Option<bool>,
    #[serde(rename = "auto_advance")]
    pub auto_advance: Option<bool>,
    // #[serde(rename = "automatic_tax")]
    // pub automatic_tax: Option<AutomaticTax>,
    #[serde(rename = "billing_reason")]
    pub billing_reason: Option<String>,
    // pub charge: Value,
    #[serde(rename = "collection_method")]
    pub collection_method: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    // #[serde(rename = "custom_fields")]
    // pub custom_fields: Value,
    pub customer: Option<String>,
    #[serde(rename = "customer_address")]
    pub customer_address: Option<String>,
    #[serde(rename = "customer_email")]
    pub customer_email: Option<String>,
    #[serde(rename = "customer_name")]
    pub customer_name: Option<String>,
    #[serde(rename = "customer_phone")]
    pub customer_phone: Option<String>,
    #[serde(rename = "customer_shipping")]
    pub customer_shipping: Option<String>,
    #[serde(rename = "customer_tax_exempt")]
    pub customer_tax_exempt: Option<String>,
    #[serde(rename = "customer_tax_ids")]
    pub customer_tax_ids: Option<Vec<String>>,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Option<String>,
    #[serde(rename = "default_source")]
    pub default_source: Option<String>,
    // #[serde(rename = "default_tax_rates")]
    // pub default_tax_rates: Option<Vec<String>>,
    pub description: Option<String>,
    // pub discount: Value,
    // pub discounts: Vec<Value>,
    // #[serde(rename = "due_date")]
    // pub due_date: Value,
    // #[serde(rename = "ending_balance")]
    // pub ending_balance: Value,
    // pub footer: Value,
    #[serde(rename = "hosted_invoice_url")]
    pub hosted_invoice_url: Option<String>,
    #[serde(rename = "invoice_pdf")]
    pub invoice_pdf: Option<String>,
    // #[serde(rename = "last_finalization_error")]
    // pub last_finalization_error: Value,
    pub lines: Option<InvoiceLines>,
    pub livemode: Option<bool>,
    // pub metadata: Metadata3,
    #[serde(rename = "next_payment_attempt")]
    pub next_payment_attempt: Option<i64>,
    // pub number: Value,
    // #[serde(rename = "on_behalf_of")]
    // pub on_behalf_of: Value,
    pub paid: Option<bool>,
    #[serde(rename = "paid_out_of_band")]
    pub paid_out_of_band: Option<bool>,
    // #[serde(rename = "payment_intent")]
    // pub payment_intent: Value,
    #[serde(rename = "payment_settings")]
    pub payment_settings: Option<PaymentSettings>,
    #[serde(rename = "period_end")]
    pub period_end: Option<i64>,
    #[serde(rename = "period_start")]
    pub period_start: Option<i64>,
    #[serde(rename = "post_payment_credit_notes_amount")]
    pub post_payment_credit_notes_amount: Option<i64>,
    #[serde(rename = "pre_payment_credit_notes_amount")]
    pub pre_payment_credit_notes_amount: Option<i64>,
    // pub quote: Value,
    // #[serde(rename = "receipt_number")]
    // pub receipt_number: Value,
    #[serde(rename = "starting_balance")]
    pub starting_balance: Option<i64>,
    // #[serde(rename = "statement_descriptor")]
    // pub statement_descriptor: Value,
    pub status: Option<String>,
    #[serde(rename = "status_transitions")]
    pub status_transitions: Option<StatusTransitions>,
    // pub subscription: Value,
    pub subtotal: Option<i64>,
    pub subscription: Option<String>,

    // pub tax: Value,
    pub total: Option<i64>,
    // #[serde(rename = "total_discount_amounts")]
    // pub total_discount_amounts: Vec<Value>,
    // #[serde(rename = "total_tax_amounts")]
    // pub total_tax_amounts: Vec<Value>,
    // #[serde(rename = "transfer_data")]
    // pub transfer_data: Value,
    // #[serde(rename = "webhooks_delivered_at")]
    // pub webhooks_delivered_at: Value,
}
impl Invoice {
    /// Returns an empty Invoice object
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut invoice = payup::stripe::Invoice::new();
    /// invoice.customer = Some("cust_test123".to_string());
    /// invoice.collection_method = Some(format!("charge_automatically"));
    /// invoice.auto_advance = Some(true);
    /// ```ignore
    pub fn new() -> Self {
        return Invoice {
            id: None,
            object: None,
            account_country: None,
            account_name: None,
            account_tax_ids: None,
            amount_due: None,
            amount_paid: None,
            amount_remaining: None,
            application_fee_amount: None,
            attempt_count: None,
            attempted: None,
            auto_advance: None,
            billing_reason: None,
            collection_method: None,
            created: None,
            currency: None,
            customer: None,
            customer_address: None,
            customer_email: None,
            customer_name: None,
            customer_phone: None,
            customer_shipping: None,
            customer_tax_exempt: None,
            customer_tax_ids: None,
            default_payment_method: None,
            default_source: None,
            description: None,
            hosted_invoice_url: None,
            invoice_pdf: None,
            lines: None,
            livemode: None,
            next_payment_attempt: None,
            paid: None,
            paid_out_of_band: None,
            payment_settings: None,
            period_end: None,
            period_start: None,
            post_payment_credit_notes_amount: None,
            pre_payment_credit_notes_amount: None,
            subscription: None,
            status: None,
            starting_balance: None,
            status_transitions: None,
            subtotal: None,
            total: None,
        };
    }

    /// Asynchronously retrieves the Invoice by the id.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the invoice you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let invoice = payup::stripe::Invoice::async_get(auth, "in_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/invoices/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously returns all stripe Invoices.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `status` - Ex: draft, open, paid, uncollectible, void (optional)
    /// * `customer` - Limit to invoices belonging to a Customer's id (optional)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let invoices = payup::stripe::Invoice::async_list(auth).await?;
    /// ```ignore
    pub async fn async_list(
        creds: Auth,
        status: Option<String>,
        customer: Option<String>,
    ) -> Result<Vec<Self>, reqwest::Error> {
        let mut objects: Vec<Self> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::list_chunk_async(
                creds.clone(),
                starting_after,
                status.clone(),
                customer.clone(),
            )
            .await?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = objects.last().and_then(|obj| obj.id.clone());
        }
        Ok(objects)
    }

    /// Asynchronously POSTs a new Invoice to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut invoice = payup::stripe::Invoice::new();
    /// invoice.customer = Some("cust_test123".to_string());
    /// invoice.collection_method = Some(format!("charge_automatically"));
    /// invoice.auto_advance = Some(true);
    /// invoice = invoice.async_post(auth).await?;
    /// ```ignore
    pub async fn async_post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/invoices")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Asynchronously POSTs an update to an existing Invoice
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut invoice = payup::stripe::Invoice::new();
    /// invoice.customer = Some("cust_test123".to_string());
    /// invoice.collection_method = Some(format!("charge_automatically"));
    /// invoice.auto_advance = Some(false);
    /// invoice = invoice.async_post(auth).await?;
    ///
    /// invoice.auto_advance = Some(true);
    /// invoice = invoice.async_update(auth).await?;
    /// ```ignore
    pub async fn async_update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/invoices/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Invoice ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Retrieves the details of an Invoice by the id.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the invoice you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let invoice = payup::stripe::Invoice::get(auth, "in_test123".to_string());
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/invoices/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// Returns all stripe invoices.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `status` - Ex: draft, open, paid, uncollectible, void (optional)
    /// * `customer` - Limit to invoices belonging to a Customer's id (optional)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch all customers from stripe
    /// let invoices = payup::stripe::Invoice::list(auth)?;
    /// ```ignore
    pub fn list(
        creds: Auth,
        status: Option<String>,
        customer: Option<String>,
    ) -> Result<Vec<Self>, reqwest::Error> {
        let mut objects: Vec<Self> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::list_chunk(
                creds.clone(),
                starting_after,
                status.clone(),
                customer.clone(),
            )?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = objects.last().and_then(|obj| obj.id.clone());
        }
        Ok(objects)
    }

    /// POSTs a new Invoice to the stripe api
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut invoice = payup::stripe::Invoice::new();
    /// invoice.customer = Some("cust_test123".to_string());
    /// invoice.collection_method = Some(format!("charge_automatically"));
    /// invoice.auto_advance = Some(true);
    /// invoice = invoice.post(auth)?;
    /// ```ignore
    pub fn post(&self, creds: Auth) -> Result<Self, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/invoices")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    /// POSTs an update to an existing Charge
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let mut invoice = payup::stripe::Invoice::new();
    /// invoice.customer = Some("cust_test123".to_string());
    /// invoice.collection_method = Some(format!("charge_automatically"));
    /// invoice.auto_advance = Some(false);
    /// invoice = invoice.post(auth)?;
    ///
    /// invoice.auto_advance = Some(true);
    /// invoice = invoice.update(auth)?;
    /// ```ignore
    pub fn update(&self, creds: Auth) -> Result<Self, crate::error::PayupError> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/invoices/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Invoice ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Self>()?;
        Ok(json)
    }

    fn list_chunk(
        creds: Auth,
        starting_after: Option<String>,
        status: Option<String>,
        customer: Option<String>,
    ) -> Result<Invoices, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/invoices".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/invoices?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        if status.is_some() {
            if url.contains("?") {
                url = format!("{}{}={}", url, "&status", status.expect("status should be Some at this point"));
            } else {
                url = format!("{}{}={}", url, "?status", status.expect("status should be Some at this point"));
            }
        }

        if customer.is_some() {
            if url.contains("?") {
                url = format!("{}{}={}", url, "&customer", customer.expect("customer should be Some at this point"));
            } else {
                url = format!("{}{}={}", url, "?customer", customer.expect("customer should be Some at this point"));
            }
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<Invoices>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
        status: Option<String>,
        customer: Option<String>,
    ) -> Result<Invoices, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/invoices".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/invoices?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        if status.is_some() {
            if url.contains("?") {
                url = format!("{}{}={}", url, "&status", status.expect("status should be Some at this point"));
            } else {
                url = format!("{}{}={}", url, "?status", status.expect("status should be Some at this point"));
            }
        }

        if customer.is_some() {
            if url.contains("?") {
                url = format!("{}{}={}", url, "&customer", customer.expect("customer should be Some at this point"));
            } else {
                url = format!("{}{}={}", url, "?customer", customer.expect("customer should be Some at this point"));
            }
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<Invoices>().await?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];
        match &self.customer {
            Some(customer) => params.push(("customer", customer.as_str())),
            None => {}
        }
        // match &self.auto_advance{
        //     Some(auto_advance) => params.push(("auto_advance", auto_advance.as_str())),
        //     None => {}
        // }
        match &self.collection_method {
            Some(collection_method) => {
                params.push(("collection_method", collection_method.as_str()))
            }
            None => {}
        }
        match &self.description {
            Some(description) => params.push(("description", description.as_str())),
            None => {}
        }
        // match &self.metadata{
        //     Some(metadata) => params.push(("metadata", metadata.as_str())),
        //     None => {}
        // }

        match &self.subscription {
            Some(subscription) => params.push(("subscription", subscription.as_str())),
            None => {}
        }

        return params;
    }
}

/// A Mandate is a record of the permission a customer has given you to debit their payment method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mandate {
    pub id: String,
    pub object: String,
    #[serde(rename = "customer_acceptance")]
    pub customer_acceptance: CustomerAcceptance,
    pub livemode: bool,
    // #[serde(rename = "multi_use")]
    // pub multi_use: MultiUse,
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "payment_method_details")]
    pub payment_method_details: PaymentMethodDetails,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
impl Mandate {
    /// Asynchronously retrieves a Mandate with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the Mandate you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let file = payup::stripe::Mandate::async_get(auth, "mandate_test123".to_string()).await?;
    /// ```ignore
    pub async fn async_get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/file_links/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<Self>().await?;
        Ok(json)
    }

    /// Retrieves a Mandate with the given ID.
    ///
    /// # Arguments
    ///
    /// * `auth` - payup::stripe::Auth::new(client, secret)
    /// * `id` - The id of the Mandate you want to retrieve.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// // Fetch customer using id
    /// let mandate = payup::stripe::Mandate::get(auth, "mandate_test123".to_string())?;
    /// ```ignore
    pub fn get(creds: Auth, id: String) -> Result<Self, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/file_links/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;
        let json = request.json::<Self>()?;
        Ok(json)
    }
}

// TODO - Finish Implementation
/// PaymentMethod objects represent your customer's payment instruments.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    pub id: Option<String>,
    pub method_type: Option<String>,
    pub created: Option<String>,
    pub customer: Option<String>,
    pub livemode: Option<bool>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub billing_details: Option<crate::stripe::response::BillingDetails>,
    pub card: Option<Card>,
    pub type_field: Option<String>,
}
impl PaymentMethod {
    pub fn new() -> Self {
        return PaymentMethod {
            id: None,
            method_type: None,
            created: None,
            customer: None,
            livemode: None,
            name: None,
            phone: None,
            billing_details: None,
            card: None,
            type_field: None,
        };
    }
    pub fn attach(&self, customer: Customer, creds: Auth) -> Result<bool, reqwest::Error> {
        match &self.id {
            Some(id) => match &customer.id {
                Some(cust_id) => {
                    let url = format!("https://api.stripe.com/v1/payment_methods/{}/attach", id);

                    let params = [("customer", cust_id.as_str())];
                    reqwest::blocking::Client::new()
                        .post(url)
                        .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
                        .form(&params)
                        .send()?;
                    return Ok(true);
                }
                None => return Ok(false),
            },
            None => return Ok(false),
        }
    }
    pub fn get(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::PaymentMethod, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/payment_methods/{}", id);

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send();
        match request {
            Ok(req) => {
                let json = req
                    .json::<crate::stripe::response::PaymentMethod>()?;
                Ok(json)
            }
            Err(err) => Err(err),
        }
    }
    pub fn post(&self, creds: Auth) -> Result<PaymentMethod, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/payment_methods")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send();

        match request {
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::PaymentMethod>()?;
                plan.id = Some(json.id);
                Ok(plan)
            }
            Err(err) => Err(err),
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.method_type {
            Some(method_type) => params.push(("type", method_type.as_str())),
            None => {}
        }
        match &self.card {
            Some(card) => {
                match &card.number {
                    Some(number) => params.push(("card[number]", number.as_str())),
                    None => {}
                }
                match &card.exp_month {
                    Some(exp_month) => params.push(("card[exp_month]", exp_month.as_str())),
                    None => {}
                }
                match &card.exp_year {
                    Some(exp_year) => params.push(("card[exp_year]", exp_year.as_str())),
                    None => {}
                }
                match &card.cvc {
                    Some(cvc) => params.push(("card[cvc]", cvc.as_str())),
                    None => {}
                }
            }
            None => {}
        }
        return params;
    }
}

// TODO - Finish Implementation
/// Plans define the base price, currency, and billing cycle for recurring purchases of products.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plan {
    pub id: Option<String>,
    pub active: Option<String>,
    pub amount: Option<String>,
    pub amount_decimal: Option<String>,
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub interval: Option<String>,
    pub interval_count: Option<String>,
    pub product: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        return Plan {
            id: None,
            active: None,
            amount: None,
            amount_decimal: None,
            billing_scheme: None,
            created: None,
            currency: None,
            interval: None,
            interval_count: None,
            product: None,
        };
    }

    pub async fn async_delete(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);

        let request = reqwest::Client::new()
            .delete(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Plan>().await?;
        Ok(json)
    }

    pub async fn async_get(
        auth: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);
        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(auth.client.as_str(), Some(auth.secret.as_str()))
            .send()
            .await?;
        let json = request.json::<crate::stripe::response::Plan>().await?;
        Ok(json)
    }

    pub async fn async_list(
        creds: Auth,
    ) -> Result<Vec<crate::stripe::response::Plan>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::Plan> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::list_chunk_async(creds.clone(), starting_after.clone()).await?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }

        Ok(objects)
    }

    pub async fn async_post(
        &self,
        creds: Auth,
    ) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let request = reqwest::Client::new()
            .post("https://api.stripe.com/v1/plans")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Plan>().await?;
        Ok(json)
    }

    pub fn delete(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);

        let request = reqwest::blocking::Client::new()
            .delete(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<crate::stripe::response::Plan>()?;
        Ok(json)
    }

    pub fn get(auth: Auth, id: String) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);
        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(auth.client.as_str(), Some(auth.secret.as_str()))
            .send()?;
        let json = request.json::<crate::stripe::response::Plan>()?;
        Ok(json)
    }

    pub fn list(creds: Auth) -> Result<Vec<crate::stripe::response::Plan>, reqwest::Error> {
        let mut objects: Vec<crate::stripe::response::Plan> = Vec::new();

        let mut has_more = true;
        let mut starting_after: Option<String> = None;
        while has_more {
            let json = Self::list_chunk(creds.clone(), starting_after.clone())?;
            for json_object in json.data {
                objects.push(json_object);
            }
            has_more = json.has_more;
            starting_after = Some(objects[objects.len() - 1].id.clone());
        }

        Ok(objects)
    }

    pub fn post(&self, creds: Auth) -> Result<crate::stripe::response::Plan, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/plans")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<crate::stripe::response::Plan>()?;
        Ok(json)
    }

    fn list_chunk(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<crate::stripe::response::Plans, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/plans".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/plans?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()?;

        let json = request.json::<crate::stripe::response::Plans>()?;
        Ok(json)
    }

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<crate::stripe::response::Plans, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/plans".to_string();

        if starting_after.is_some() {
            url = format!(
                "https://api.stripe.com/v1/plans?starting_after={}",
                starting_after.expect("starting_after should be Some at this point")
            );
        }

        let request = reqwest::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send()
            .await?;

        let json = request.json::<crate::stripe::response::Plans>().await?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.amount {
            Some(amount) => params.push(("amount", amount.as_str())),
            None => {}
        }
        match &self.currency {
            Some(currency) => params.push(("currency", currency.as_str())),
            None => {}
        }
        match &self.interval {
            Some(interval) => params.push(("interval", interval.as_str())),
            None => {}
        }
        match &self.product {
            Some(product) => params.push(("product", product.as_str())),
            None => {}
        }
        match &self.active {
            Some(active) => params.push(("active", active.as_str())),
            None => {}
        }
        return params;
    }
}

// TODO - Finish Implementation
/// Prices define the unit cost, currency, and (optional) billing cycle.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub id: Option<String>,
    pub active: Option<bool>,
    pub billing_scheme: Option<String>,
    pub created: Option<i64>,
    pub currency: Option<String>,
    pub livemode: Option<bool>,
    pub product: Option<String>,
    pub tax_behavior: Option<String>,
    pub type_field: Option<String>,
    pub unit_amount: Option<String>,
    pub unit_amount_decimal: Option<String>,
}
impl Price {
    pub fn new() -> Self {
        return Price {
            id: None,
            active: None,
            billing_scheme: None,
            created: None,
            currency: None,
            livemode: None,
            product: None,
            tax_behavior: None,
            type_field: None,
            unit_amount: None,
            unit_amount_decimal: None,
        };
    }
    pub fn post(&self, creds: Auth) -> Result<Price, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/prices")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send();

        match request {
            Ok(req) => {
                let mut plan = self.clone();
                let json = req.json::<crate::stripe::response::Price>()?;
                plan.id = Some(json.id);
                Ok(plan)
            }
            Err(err) => Err(err),
        }
    }
    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.currency {
            Some(currency) => params.push(("currency", currency.as_str())),
            None => {}
        }
        match &self.unit_amount {
            Some(unit_amount) => params.push(("unit_amount", unit_amount.as_str())),
            None => {}
        }

        // TODO Impliment product

        return params;
    }
}

// TODO - Finish Implementation
/// Subscriptions allow you to charge a customer on a recurring basis.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subscription {
    pub id: Option<String>,
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
        return Subscription {
            id: None,
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
        };
    }
    pub fn cancel(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);

        let request = reqwest::blocking::Client::new()
            .delete(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>()?;
                Ok(json)
            }
            Err(err) => Err(err),
        }
    }

    /// Returns a subscription
    ///
    /// # Arguments
    ///
    /// * `creds` - A struct containing the client + secret for authentication over the stripe API.
    /// * `id` - A string representing an existing stripe subscription_id
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Client and Secret for Stripe account
    /// // In a production environment...load values from environment variables.
    /// let client = format!("sk_test_51Jo2sKGrEH09RU9uu8d8ARKasYUKHXAHk4vUNup1JLgP5wFnQQf6t7UpKfh7woVMhI9oeuziolW2dK1uwmgAheVI00bN8ews6g");
    /// let secret = format!("");
    /// // Create the Authentication refererence
    /// let auth = payup::stripe::Auth::new("test_key".to_string(), "test_secret".to_string());
    ///
    /// let get_subscription = payup::stripe::Subscription::get(auth, "subscription_id");
    /// ```ignore
    pub fn get(
        creds: Auth,
        id: String,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let url = format!("https://api.stripe.com/v1/subscriptions/{}", id);

        let request = reqwest::blocking::Client::new()
            .get(url)
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>()?;
                Ok(json)
            }
            Err(err) => Err(err),
        }
    }
    pub fn update(
        &self,
        creds: Auth,
    ) -> Result<crate::stripe::response::Subscription, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post(format!(
                "https://api.stripe.com/v1/subscriptions/{}",
                self.clone().id.ok_or_else(|| crate::error::PayupError::ValidationError("Subscription ID is required for update".to_string()))?
            ))
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<crate::stripe::response::Subscription>()?;
                Ok(json)
            }
            Err(err) => Err(err),
        }
    }
    pub fn post(&self, creds: Auth) -> Result<Subscription, reqwest::Error> {
        let request = reqwest::blocking::Client::new()
            .post("https://api.stripe.com/v1/subscriptions")
            .basic_auth(creds.client.as_str(), Some(creds.secret.as_str()))
            .form(&self.to_params())
            .send()?;

        let json = request.json::<Subscription>()?;
        Ok(json)
    }

    fn to_params(&self) -> Vec<(&str, &str)> {
        // return Customer{client, secret};
        let mut params = vec![];
        match &self.customer {
            Some(customer) => params.push(("customer", customer.as_str())),
            None => {}
        }

        match &self.default_payment_method {
            Some(default_payment_method) => {
                params.push(("default_payment_method", default_payment_method.as_str()))
            }
            None => {}
        }

        match &self.price_items {
            Some(price_items) => {
                let mut ii = 0;
                for item in price_items {
                    if ii < 20 {
                        if ii == 0 {
                            params.push(("items[0][price]", item.as_str()));
                        }
                        ii += 1;
                    }
                }
            }
            None => {}
        }

        return params;
    }
}

// =====================================================================================
// All structs below this point are just used to support the implimented structs above
// =====================================================================================

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Charges {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Charge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct PaymentMethodDetails {
    #[serde(rename = "sepa_debit")]
    pub sepa_debit: Option<SepaDebit>,
    pub card: Option<Card>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct FraudDetails {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct BillingDetails {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Refunds {
    pub object: String,
    // pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Customers {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Customer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Disputes {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Dispute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Evidence {
    #[serde(rename = "access_activity_log")]
    pub access_activity_log: Option<String>,
    #[serde(rename = "billing_address")]
    pub billing_address: Option<String>,
    #[serde(rename = "cancellation_policy")]
    pub cancellation_policy: Option<String>,
    #[serde(rename = "cancellation_policy_disclosure")]
    pub cancellation_policy_disclosure: Option<String>,
    #[serde(rename = "cancellation_rebuttal")]
    pub cancellation_rebuttal: Option<String>,
    #[serde(rename = "customer_communication")]
    pub customer_communication: Option<String>,
    #[serde(rename = "customer_email_address")]
    pub customer_email_address: Option<String>,
    #[serde(rename = "customer_name")]
    pub customer_name: Option<String>,
    #[serde(rename = "customer_purchase_ip")]
    pub customer_purchase_ip: Option<String>,
    #[serde(rename = "customer_signature")]
    pub customer_signature: Option<String>,
    #[serde(rename = "duplicate_charge_documentation")]
    pub duplicate_charge_documentation: Option<String>,
    #[serde(rename = "duplicate_charge_explanation")]
    pub duplicate_charge_explanation: Option<String>,
    #[serde(rename = "duplicate_charge_id")]
    pub duplicate_charge_id: Option<String>,
    #[serde(rename = "product_description")]
    pub product_description: Option<String>,
    pub receipt: Option<String>,
    #[serde(rename = "refund_policy")]
    pub refund_policy: Option<String>,
    #[serde(rename = "refund_policy_disclosure")]
    pub refund_policy_disclosure: Option<String>,
    #[serde(rename = "refund_refusal_explanation")]
    pub refund_refusal_explanation: Option<String>,
    #[serde(rename = "service_date")]
    pub service_date: Option<String>,
    #[serde(rename = "service_documentation")]
    pub service_documentation: Option<String>,
    #[serde(rename = "shipping_address")]
    pub shipping_address: Option<String>,
    #[serde(rename = "shipping_carrier")]
    pub shipping_carrier: Option<String>,
    #[serde(rename = "shipping_date")]
    pub shipping_date: Option<String>,
    #[serde(rename = "shipping_documentation")]
    pub shipping_documentation: Option<String>,
    #[serde(rename = "shipping_tracking_number")]
    pub shipping_tracking_number: Option<String>,
    #[serde(rename = "uncategorized_file")]
    pub uncategorized_file: Option<String>,
    #[serde(rename = "uncategorized_text")]
    pub uncategorized_text: Option<String>,
}
impl Evidence {
    pub fn new() -> Self {
        return Evidence {
            access_activity_log: None,
            billing_address: None,
            cancellation_policy: None,
            cancellation_policy_disclosure: None,
            cancellation_rebuttal: None,
            customer_communication: None,
            customer_email_address: None,
            customer_name: None,
            customer_purchase_ip: None,
            customer_signature: None,
            duplicate_charge_documentation: None,
            duplicate_charge_explanation: None,
            duplicate_charge_id: None,
            product_description: None,
            receipt: None,
            refund_policy: None,
            refund_policy_disclosure: None,
            refund_refusal_explanation: None,
            service_date: None,
            service_documentation: None,
            shipping_address: None,
            shipping_carrier: None,
            shipping_date: None,
            shipping_documentation: None,
            shipping_tracking_number: None,
            uncategorized_file: None,
            uncategorized_text: None,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct EvidenceDetails {
    #[serde(rename = "due_by")]
    pub due_by: i64,
    #[serde(rename = "has_evidence")]
    pub has_evidence: bool,
    #[serde(rename = "past_due")]
    pub past_due: bool,
    #[serde(rename = "submission_count")]
    pub submission_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Events {
    pub object: String,
    pub data: Vec<Event>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Request {
    pub id: String,
    #[serde(rename = "idempotency_key")]
    pub idempotency_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Files {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Links {
    pub object: String,
    // pub data: Vec<Value>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct FileLinks {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<FileLink>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Invoices {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Invoice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct CustomerAcceptance {
    #[serde(rename = "accepted_at")]
    pub accepted_at: i64,
    pub online: Online,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Online {
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    #[serde(rename = "user_agent")]
    pub user_agent: String,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct MultiUse {
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct SepaDebit {
    pub reference: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct InvoiceLines {
    pub object: String,
    pub data: Vec<InvoiceLine>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct InvoiceLine {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    // #[serde(rename = "discount_amounts")]
    // pub discount_amounts: Vec<Value>,
    pub discountable: Option<bool>,
    // pub discounts: Vec<Value>,
    #[serde(rename = "invoice_item")]
    pub invoice_item: Option<String>,
    pub livemode: Option<bool>,
    // pub metadata: Metadata,
    // pub period: Option<Period>,
    // pub price: Price,
    pub proration: Option<bool>,
    pub quantity: Option<i64>,
    // pub subscription: Value,
    // #[serde(rename = "tax_amounts")]
    // pub tax_amounts: Vec<Value>,
    // #[serde(rename = "tax_rates")]
    // pub tax_rates: Vec<Value>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Period {
    pub end: Option<i64>,
    pub start: Option<i64>,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[doc(hidden)]
// pub struct Price {
//     pub id: Option<String>,
//     pub object: Option<String>,
//     pub active: Option<bool>,
//     #[serde(rename = "billing_scheme")]
//     pub billing_scheme: Option<String>,
//     pub created: Option<i64>,
//     pub currency: Option<String>,
//     pub livemode: Option<bool>,
//     // #[serde(rename = "lookup_key")]
//     // pub lookup_key: Value,
//     // pub metadata: Metadata2,
//     pub nickname: Option<String>,
//     pub product: Option<String>,
//     // pub recurring: Option<,
//     #[serde(rename = "tax_behavior")]
//     pub tax_behavior: Option<String>,
//     // #[serde(rename = "tiers_mode")]
//     // pub tiers_mode: Value,
//     // #[serde(rename = "transform_quantity")]
//     // pub transform_quantity: Value,
//     #[serde(rename = "type")]
//     pub type_field: Option<String>,
//     #[serde(rename = "unit_amount")]
//     pub unit_amount: Option<i64>,
//     #[serde(rename = "unit_amount_decimal")]
//     pub unit_amount_decimal: Option<String>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct PaymentSettings {
    // #[serde(rename = "payment_method_options")]
    // pub payment_method_options: Value,
    // #[serde(rename = "payment_method_types")]
    // pub payment_method_types: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct StatusTransitions {
    #[serde(rename = "finalized_at")]
    pub finalized_at: Option<i64>,
    #[serde(rename = "marked_uncollectible_at")]
    pub marked_uncollectible_at: Option<i64>,
    #[serde(rename = "paid_at")]
    pub paid_at: Option<i64>,
    #[serde(rename = "voided_at")]
    pub voided_at: Option<i64>,
}
