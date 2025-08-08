use reqwest::blocking::Client as HttpClient;
use reqwest::Client as AsyncHttpClient;
use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use crate::http_utils::{HttpRequestBuilder, build_url};
use super::{SquareConfig, SquareAuth, ApiResponse};

pub struct SquareClient {
    pub config: SquareConfig,
    pub auth: SquareAuth,
    http_client: HttpClient,
    async_http_client: AsyncHttpClient,
    request_builder: HttpRequestBuilder,
}

impl SquareClient {
    pub fn new(config: SquareConfig) -> Result<Self> {
        let auth = SquareAuth::new(
            config.access_token.clone(),
            config.environment.clone(),
        );
        auth.validate()?;

        Ok(Self {
            config,
            auth,
            http_client: HttpClient::new(),
            async_http_client: AsyncHttpClient::new(),
            request_builder: HttpRequestBuilder::new("Square"),
        })
    }

    pub fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .header("Square-Version", "2024-01-01")
            .send()
            .map_err(PayupError::from)?;

        self.process_square_response(response)
    }

    fn process_square_response<T>(&self, response: reqwest::blocking::Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if !response.status().is_success() {
            return self.handle_error_response(response);
        }

        let api_response: ApiResponse<T> = response.json().map_err(PayupError::from)?;
        self.extract_data_from_response(api_response)
    }

    fn handle_error_response<T>(&self, response: reqwest::blocking::Response) -> Result<T> {
        let status = response.status().to_string();
        let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(PayupError::ApiError {
            code: status,
            message: error_text,
            provider: "Square".to_string(),
        })
    }

    fn extract_data_from_response<T>(&self, api_response: ApiResponse<T>) -> Result<T> {
        if let Some(errors) = api_response.errors {
            if let Some(first_error) = errors.first() {
                return Err(PayupError::ApiError {
                    code: first_error.code.clone(),
                    message: first_error.detail.clone()
                        .unwrap_or_else(|| first_error.category.clone()),
                    provider: "Square".to_string(),
                });
            }
        }

        api_response.data
            .ok_or_else(|| PayupError::GenericError("No data in response".to_string()))
    }

    pub async fn async_get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.async_http_client
            .get(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .header("Square-Version", "2024-01-01")
            .send()
            .await
            .map_err(PayupError::from)?;

        self.process_async_square_response(response).await
    }

    async fn process_async_square_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        if !response.status().is_success() {
            return self.handle_async_error_response(response).await;
        }

        let api_response: ApiResponse<T> = response.json().await.map_err(PayupError::from)?;
        self.extract_data_from_response(api_response)
    }

    async fn handle_async_error_response<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status().to_string();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(PayupError::ApiError {
            code: status,
            message: error_text,
            provider: "Square".to_string(),
        })
    }

    pub fn post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .header("Square-Version", "2024-01-01")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        self.process_square_response(response)
    }

    pub async fn async_post<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.async_http_client
            .post(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .header("Square-Version", "2024-01-01")
            .json(body)
            .send()
            .await
            .map_err(PayupError::from)?;

        self.process_async_square_response(response).await
    }

    pub fn put<T, B>(&self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.http_client
            .put(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .header("Square-Version", "2024-01-01")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        self.process_square_response(response)
    }

    pub fn delete(&self, endpoint: &str) -> Result<bool> {
        let url = build_url(self.auth.base_url(), endpoint);
        
        let response = self.http_client
            .delete(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Square-Version", "2024-01-01")
            .send()
            .map_err(PayupError::from)?;

        Ok(response.status().is_success())
    }
}