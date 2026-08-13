use crate::error::{PayupError, Result};
use crate::http_client::{get_shared_blocking_client, get_shared_client};
use crate::stripe::request::{
    idempotent_blocking_request, idempotent_request, parse_async_response,
    parse_blocking_response, IdempotencyKey,
};
use crate::stripe::Auth;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: Option<String>,
    pub object: Option<String>,
    pub active: Option<bool>,
    pub created: Option<i64>,
    pub default_price: Option<String>,
    pub description: Option<String>,
    pub images: Option<Vec<String>>,
    pub livemode: Option<bool>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub name: String,
    pub package_dimensions: Option<PackageDimensions>,
    pub shippable: Option<bool>,
    pub statement_descriptor: Option<String>,
    pub tax_code: Option<String>,
    pub unit_label: Option<String>,
    pub updated: Option<i64>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

impl Product {
    pub fn new() -> Self {
        Self {
            id: None,
            object: None,
            active: Some(true),
            created: None,
            default_price: None,
            description: None,
            images: None,
            livemode: None,
            metadata: None,
            name: String::new(),
            package_dimensions: None,
            shippable: None,
            statement_descriptor: None,
            tax_code: None,
            unit_label: None,
            updated: None,
            url: None,
        }
    }

    /// Create a new product
    ///
    /// # Example
    /// ```ignore
    /// use payup::stripe_ext::product::Product;
    /// use payup::stripe::Auth;
    ///
    /// let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    /// let mut product = Product::new();
    /// product.name = "Premium Widget".to_string();
    /// product.description = Some("A high-quality widget".to_string());
    /// let created_product = product.post(auth)?;
    /// ```
    pub fn post(&self, creds: Auth) -> Result<Self> {
        let client = get_shared_blocking_client();
        let mut params = std::collections::HashMap::new();

        params.insert("name", self.name.clone());

        if let Some(desc) = &self.description {
            params.insert("description", desc.clone());
        }

        if let Some(active) = &self.active {
            params.insert("active", active.to_string());
        }

        if let Some(url) = &self.url {
            params.insert("url", url.clone());
        }

        let request = client
            .post("https://api.stripe.com/v1/products")
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_blocking_request(request, &IdempotencyKey::generate())
            .form(&params)
            .send()?;

        parse_blocking_response(response)
    }

    /// Retrieve a product
    pub fn get(creds: Auth, product_id: String) -> Result<Self> {
        let client = get_shared_blocking_client();
        let url = format!("https://api.stripe.com/v1/products/{}", product_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()))
            .send()?;

        parse_blocking_response(response)
    }

    /// Update a product
    pub fn update(&self, creds: Auth) -> Result<Self> {
        if let Some(id) = &self.id {
            let client = get_shared_blocking_client();
            let url = format!("https://api.stripe.com/v1/products/{}", id);
            let mut params = std::collections::HashMap::new();

            if let Some(desc) = &self.description {
                params.insert("description", desc.clone());
            }

            if let Some(active) = &self.active {
                params.insert("active", active.to_string());
            }

            params.insert("name", self.name.clone());

            let request = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", creds.api_key()));
            let response = idempotent_blocking_request(request, &IdempotencyKey::generate())
                .form(&params)
                .send()?;

            parse_blocking_response(response)
        } else {
            Err(PayupError::ValidationError(
                "Product ID is required for update".to_string(),
            ))
        }
    }

    /// List all products
    pub fn list(creds: Auth, limit: Option<i32>) -> Result<ProductList> {
        let client = get_shared_blocking_client();
        let mut url = String::from("https://api.stripe.com/v1/products");

        if let Some(lim) = limit {
            url.push_str(&format!("?limit={}", lim));
        }

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()))
            .send()?;

        parse_blocking_response(response)
    }

    /// Delete a product
    pub fn delete(creds: Auth, product_id: String) -> Result<DeletedProduct> {
        let client = get_shared_blocking_client();
        let url = format!("https://api.stripe.com/v1/products/{}", product_id);

        let request = client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_blocking_request(request, &IdempotencyKey::generate()).send()?;

        parse_blocking_response(response)
    }

    /// Async create a new product
    pub async fn async_post(&self, creds: Auth) -> Result<Self> {
        let client = get_shared_client();
        let mut params = std::collections::HashMap::new();

        params.insert("name", self.name.clone());

        if let Some(desc) = &self.description {
            params.insert("description", desc.clone());
        }

        if let Some(active) = &self.active {
            params.insert("active", active.to_string());
        }

        let request = client
            .post("https://api.stripe.com/v1/products")
            .header("Authorization", format!("Bearer {}", creds.api_key()));
        let response = idempotent_request(request, &IdempotencyKey::generate())
            .form(&params)
            .send()
            .await?;

        parse_async_response(response).await
    }

    /// Async retrieve a product
    pub async fn async_get(creds: Auth, product_id: String) -> Result<Self> {
        let client = get_shared_client();
        let url = format!("https://api.stripe.com/v1/products/{}", product_id);

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()))
            .send()
            .await?;

        parse_async_response(response).await
    }

    /// Async list all products
    pub async fn async_list(
        creds: Auth,
        limit: Option<i32>,
    ) -> Result<ProductList> {
        let client = get_shared_client();
        let mut url = String::from("https://api.stripe.com/v1/products");

        if let Some(lim) = limit {
            url.push_str(&format!("?limit={}", lim));
        }

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", creds.api_key()))
            .send()
            .await?;

        parse_async_response(response).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductList {
    pub object: String,
    pub data: Vec<Product>,
    pub has_more: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedProduct {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}
