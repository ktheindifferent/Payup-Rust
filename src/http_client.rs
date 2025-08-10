use std::sync::Arc;
use std::time::Duration;
use reqwest::{Client, Response, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use crate::error::PayupError;
use once_cell::sync::Lazy;

pub static SHARED_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
    Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create HTTP client")
    )
});

pub static SHARED_BLOCKING_CLIENT: Lazy<Arc<reqwest::blocking::Client>> = Lazy::new(|| {
    Arc::new(
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create blocking HTTP client")
    )
});

#[derive(Clone)]
pub struct HttpClient {
    client: Arc<Client>,
    blocking_client: Arc<reqwest::blocking::Client>,
    base_url: String,
    auth_header: Option<String>,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: SHARED_CLIENT.clone(),
            blocking_client: SHARED_BLOCKING_CLIENT.clone(),
            base_url: base_url.into(),
            auth_header: None,
        }
    }

    pub fn with_auth(mut self, auth_type: &str, token: &str) -> Self {
        self.auth_header = Some(format!("{} {}", auth_type, token));
        self
    }

    pub fn with_bearer_auth(self, token: &str) -> Self {
        self.with_auth("Bearer", token)
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        if let Some(auth) = &self.auth_header {
            if let Ok(header_value) = HeaderValue::from_str(auth) {
                headers.insert(AUTHORIZATION, header_value);
            }
        }
        
        headers
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_response(response).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client
            .post(&url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .await
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_response(response).await
    }

    pub async fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client
            .put(&url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .await
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_response(response).await
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client
            .delete(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_response(response).await
    }

    pub async fn post_form<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut headers = self.build_headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        
        let response = self.client
            .post(&url)
            .headers(headers)
            .form(params)
            .send()
            .await
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if response.status().is_success() {
            response.json::<T>().await
                .map_err(|e| PayupError::Deserialization(e.to_string()))
        } else {
            let status = response.status().as_u16();
            let error_text = response.text().await
                .unwrap_or_else(|_| format!("HTTP error {}", status));
            Err(PayupError::Http(format!("HTTP {}: {}", status, error_text)))
        }
    }

    pub fn get_blocking<T>(&self, endpoint: &str) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.blocking_client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_blocking_response(response)
    }

    pub fn post_blocking<T, B>(&self, endpoint: &str, body: &B) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.blocking_client
            .post(&url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_blocking_response(response)
    }

    pub fn post_form_blocking<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut headers = self.build_headers();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        
        let response = self.blocking_client
            .post(&url)
            .headers(headers)
            .form(params)
            .send()
            .map_err(|e| PayupError::Http(e.to_string()))?;
        
        self.handle_blocking_response(response)
    }

    fn handle_blocking_response<T>(&self, response: reqwest::blocking::Response) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        if response.status().is_success() {
            response.json::<T>()
                .map_err(|e| PayupError::Deserialization(e.to_string()))
        } else {
            let status = response.status().as_u16();
            let error_text = response.text()
                .unwrap_or_else(|_| format!("HTTP error {}", status));
            Err(PayupError::Http(format!("HTTP {}: {}", status, error_text)))
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    method: Method,
    endpoint: String,
    query_params: Vec<(String, String)>,
    body: Option<serde_json::Value>,
    form_params: Option<Vec<(String, String)>>,
}

#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl RequestBuilder {
    pub fn new(method: Method, endpoint: impl Into<String>) -> Self {
        Self {
            method,
            endpoint: endpoint.into(),
            query_params: Vec::new(),
            body: None,
            form_params: None,
        }
    }

    pub fn get(endpoint: impl Into<String>) -> Self {
        Self::new(Method::Get, endpoint)
    }

    pub fn post(endpoint: impl Into<String>) -> Self {
        Self::new(Method::Post, endpoint)
    }

    pub fn put(endpoint: impl Into<String>) -> Self {
        Self::new(Method::Put, endpoint)
    }

    pub fn delete(endpoint: impl Into<String>) -> Self {
        Self::new(Method::Delete, endpoint)
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    pub fn query_opt(self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        if let Some(v) = value {
            self.query(key, v)
        } else {
            self
        }
    }

    pub fn json<T: Serialize>(mut self, body: T) -> Result<Self, PayupError> {
        self.body = Some(serde_json::to_value(body)
            .map_err(|e| PayupError::Serialization(e.to_string()))?);
        Ok(self)
    }

    pub fn form(mut self, params: Vec<(String, String)>) -> Self {
        self.form_params = Some(params);
        self
    }

    pub fn form_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let params = self.form_params.get_or_insert_with(Vec::new);
        params.push((key.into(), value.into()));
        self
    }

    pub fn form_param_opt(self, key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        if let Some(v) = value {
            self.form_param(key, v)
        } else {
            self
        }
    }

    pub async fn send<T>(&self, client: &HttpClient) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let endpoint = self.build_endpoint();
        
        match &self.method {
            Method::Get => client.get(&endpoint).await,
            Method::Post => {
                if let Some(form_params) = &self.form_params {
                    let params: Vec<(&str, &str)> = form_params
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    client.post_form(&endpoint, &params).await
                } else if let Some(body) = &self.body {
                    client.post(&endpoint, body).await
                } else {
                    client.post(&endpoint, &serde_json::Value::Null).await
                }
            },
            Method::Put => {
                let body = self.body.as_ref().unwrap_or(&serde_json::Value::Null);
                client.put(&endpoint, body).await
            },
            Method::Delete => client.delete(&endpoint).await,
            Method::Patch => {
                let body = self.body.as_ref().unwrap_or(&serde_json::Value::Null);
                client.put(&endpoint, body).await
            },
        }
    }

    pub fn send_blocking<T>(&self, client: &HttpClient) -> Result<T, PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let endpoint = self.build_endpoint();
        
        match &self.method {
            Method::Get => client.get_blocking(&endpoint),
            Method::Post => {
                if let Some(form_params) = &self.form_params {
                    let params: Vec<(&str, &str)> = form_params
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.as_str()))
                        .collect();
                    client.post_form_blocking(&endpoint, &params)
                } else if let Some(body) = &self.body {
                    client.post_blocking(&endpoint, body)
                } else {
                    client.post_blocking(&endpoint, &serde_json::Value::Null)
                }
            },
            _ => Err(PayupError::Http("Blocking mode only supports GET and POST".to_string())),
        }
    }

    fn build_endpoint(&self) -> String {
        if self.query_params.is_empty() {
            self.endpoint.clone()
        } else {
            let query = self.query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", self.endpoint, query)
        }
    }
}