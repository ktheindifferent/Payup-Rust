use serde::{Deserialize, Serialize};

use crate::stripe::auth::Auth;

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

    // ... Other methods (synchronous versions) for brevity
    // The full implementation would include all the synchronous methods as well

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

    async fn list_chunk_async(
        creds: Auth,
        starting_after: Option<String>,
    ) -> Result<Customers, reqwest::Error> {
        let mut url = "https://api.stripe.com/v1/customers".to_string();

        if let Some(ref after) = starting_after {
            url = format!(
                "https://api.stripe.com/v1/customers?starting_after={}",
                after
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

// Supporting type for Customer
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