use serde::{Deserialize, Serialize};

use crate::stripe::auth::Auth;
use crate::http_client::{get_shared_client, get_shared_blocking_client};

/// Shipping information for charges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ShippingAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// Shipping address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Shipping {
    pub fn new() -> Self {
        Shipping {
            address: None,
            carrier: None,
            name: None,
            phone: None,
            tracking_number: None,
        }
    }

    /// Convert to URL-encoded parameters for API requests
    pub fn to_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        
        if let Some(address) = &self.address {
            if let Some(city) = &address.city {
                params.push(("shipping[address][city]".to_string(), city.clone()));
            }
            if let Some(country) = &address.country {
                params.push(("shipping[address][country]".to_string(), country.clone()));
            }
            if let Some(line1) = &address.line1 {
                params.push(("shipping[address][line1]".to_string(), line1.clone()));
            }
            if let Some(line2) = &address.line2 {
                params.push(("shipping[address][line2]".to_string(), line2.clone()));
            }
            if let Some(postal_code) = &address.postal_code {
                params.push(("shipping[address][postal_code]".to_string(), postal_code.clone()));
            }
            if let Some(state) = &address.state {
                params.push(("shipping[address][state]".to_string(), state.clone()));
            }
        }
        
        if let Some(carrier) = &self.carrier {
            params.push(("shipping[carrier]".to_string(), carrier.clone()));
        }
        if let Some(name) = &self.name {
            params.push(("shipping[name]".to_string(), name.clone()));
        }
        if let Some(phone) = &self.phone {
            params.push(("shipping[phone]".to_string(), phone.clone()));
        }
        if let Some(tracking_number) = &self.tracking_number {
            params.push(("shipping[tracking_number]".to_string(), tracking_number.clone()));
        }
        
        params
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
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
            shipping: None,
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

        let request = get_shared_client()
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
        let request = get_shared_client()
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
        let request = get_shared_client()
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
        let request = get_shared_client()
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
            self.id.clone().unwrap()
        );

        let request = get_shared_blocking_client()
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
        let request = get_shared_blocking_client()
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
        let request = get_shared_blocking_client()
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
        let request = get_shared_blocking_client()
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

        let request = get_shared_blocking_client()
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

        let request = get_shared_client()
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

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];
        if let Some(customer) = &self.customer {
            params.push(("customer".to_string(), customer.clone()));
        }
        if let Some(description) = &self.description {
            params.push(("description".to_string(), description.clone()));
        }
        if let Some(receipt_email) = &self.receipt_email {
            params.push(("receipt_email".to_string(), receipt_email.clone()));
        }
        if let Some(amount) = &self.amount {
            params.push(("amount".to_string(), amount.clone()));
        }
        if let Some(currency) = &self.currency {
            params.push(("currency".to_string(), currency.clone()));
        }
        // Handle shipping parameters
        if let Some(shipping) = &self.shipping {
            let shipping_params = shipping.to_params();
            params.extend(shipping_params);
        }
        if let Some(source) = &self.source {
            params.push(("source".to_string(), source.clone()));
        }
        if let Some(statement_descriptor) = &self.statement_descriptor {
            params.push(("statement_descriptor".to_string(), statement_descriptor.clone()));
        }
        if let Some(statement_descriptor_suffix) = &self.statement_descriptor_suffix {
            params.push(("statement_descriptor_suffix".to_string(), statement_descriptor_suffix.clone()));
        }
        params
    }
}

// Supporting types for Charge

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
pub struct SepaDebit {
    pub reference: String,
    pub url: String,
}