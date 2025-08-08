use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{SquareClient, Address};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub nickname: Option<String>,
    pub company_name: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<Address>,
    pub phone_number: Option<String>,
    pub birthday: Option<String>,
    pub reference_id: Option<String>,
    pub note: Option<String>,
    pub preferences: Option<CustomerPreferences>,
    pub groups: Option<Vec<CustomerGroupInfo>>,
    pub creation_source: Option<String>,
    pub segment_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPreferences {
    pub email_unsubscribed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroupInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub idempotency_key: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub company_name: Option<String>,
    pub nickname: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<Address>,
    pub phone_number: Option<String>,
    pub reference_id: Option<String>,
    pub note: Option<String>,
    pub birthday: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub company_name: Option<String>,
    pub nickname: Option<String>,
    pub email_address: Option<String>,
    pub address: Option<Address>,
    pub phone_number: Option<String>,
    pub reference_id: Option<String>,
    pub note: Option<String>,
    pub birthday: Option<String>,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCustomersRequest {
    pub filter: Option<CustomerFilter>,
    pub sort: Option<CustomerSort>,
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerFilter {
    pub creation_source: Option<CustomerCreationSourceFilter>,
    pub created_at: Option<TimeRange>,
    pub updated_at: Option<TimeRange>,
    pub email_address: Option<CustomerTextFilter>,
    pub phone_number: Option<CustomerTextFilter>,
    pub reference_id: Option<CustomerTextFilter>,
    pub group_ids: Option<FilterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreationSourceFilter {
    pub values: Vec<String>,
    pub rule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start_at: Option<String>,
    pub end_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTextFilter {
    pub exact: Option<String>,
    pub fuzzy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterValue {
    pub all: Option<Vec<String>>,
    pub any: Option<Vec<String>>,
    pub none: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSort {
    pub field: String,
    pub order: Option<String>,
}

impl Customer {
    pub fn new() -> Self {
        Self {
            id: None,
            created_at: None,
            updated_at: None,
            given_name: None,
            family_name: None,
            nickname: None,
            company_name: None,
            email_address: None,
            address: None,
            phone_number: None,
            birthday: None,
            reference_id: None,
            note: None,
            preferences: None,
            groups: None,
            creation_source: None,
            segment_ids: None,
        }
    }

    pub fn create(client: &SquareClient, request: &CreateCustomerRequest) -> Result<Self> {
        client.post("/v2/customers", request)
    }

    pub async fn async_create(client: &SquareClient, request: &CreateCustomerRequest) -> Result<Self> {
        client.async_post("/v2/customers", request).await
    }

    pub fn get(client: &SquareClient, customer_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/customers/{}", customer_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &SquareClient, customer_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/customers/{}", customer_id);
        client.async_get(&endpoint).await
    }

    pub fn update(client: &SquareClient, customer_id: &str, request: &UpdateCustomerRequest) -> Result<Self> {
        let endpoint = format!("/v2/customers/{}", customer_id);
        client.put(&endpoint, request)
    }

    pub fn delete(client: &SquareClient, customer_id: &str) -> Result<bool> {
        let endpoint = format!("/v2/customers/{}", customer_id);
        client.delete(&endpoint)
    }

    pub fn list(client: &SquareClient, cursor: Option<&str>, limit: Option<i32>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/customers?");
        if let Some(c) = cursor {
            endpoint.push_str(&format!("cursor={}&", c));
        }
        if let Some(l) = limit {
            endpoint.push_str(&format!("limit={}", l));
        }
        client.get(&endpoint)
    }

    pub fn search(client: &SquareClient, request: &SearchCustomersRequest) -> Result<Vec<Self>> {
        client.post("/v2/customers/search", request)
    }

    pub async fn async_search(client: &SquareClient, request: &SearchCustomersRequest) -> Result<Vec<Self>> {
        client.async_post("/v2/customers/search", request).await
    }
}

// Helper function to create a simple customer
pub fn create_simple_customer(
    email: &str,
    given_name: Option<&str>,
    family_name: Option<&str>,
) -> CreateCustomerRequest {
    CreateCustomerRequest {
        idempotency_key: None,
        given_name: given_name.map(|s| s.to_string()),
        family_name: family_name.map(|s| s.to_string()),
        company_name: None,
        nickname: None,
        email_address: Some(email.to_string()),
        address: None,
        phone_number: None,
        reference_id: None,
        note: None,
        birthday: None,
    }
}