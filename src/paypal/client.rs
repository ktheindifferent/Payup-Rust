use reqwest::blocking::Client as HttpClient;
use reqwest::Client as AsyncHttpClient;
use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::{PayPalConfig, PayPalAuth, PayPalEnvironment};

pub struct PayPalClient {
    pub config: PayPalConfig,
    pub auth: Option<PayPalAuth>,
    http_client: HttpClient,
    async_http_client: AsyncHttpClient,
}

impl PayPalClient {
    pub fn new(config: PayPalConfig) -> Result<Self> {
        let auth = PayPalAuth::new(
            config.client_id.clone(),
            config.client_secret.clone(),
            config.environment.clone(),
        )?;

        Ok(Self {
            config,
            auth: Some(auth),
            http_client: HttpClient::new(),
            async_http_client: AsyncHttpClient::new(),
        })
    }

    pub fn ensure_auth(&mut self) -> Result<()> {
        if self.auth.is_none() || self.auth.as_ref().unwrap().is_expired() {
            self.auth = Some(PayPalAuth::new(
                self.config.client_id.clone(),
                self.config.client_secret.clone(),
                self.config.environment.clone(),
            )?);
        }
        Ok(())
    }

    pub async fn async_ensure_auth(&mut self) -> Result<()> {
        if self.auth.is_none() || self.auth.as_ref().unwrap().is_expired() {
            self.auth = Some(PayPalAuth::async_get_access_token(
                self.config.client_id.clone(),
                self.config.client_secret.clone(),
                self.config.environment.clone(),
            ).await?);
        }
        Ok(())
    }

    pub fn get<T>(&mut self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.ensure_auth()?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .header("Content-Type", "application/json")
            .send()
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let status = response.status().to_string();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::ApiError {
                code: status,
                message: error_text,
                provider: "PayPal".to_string(),
            });
        }

        response.json().map_err(PayupError::from)
    }

    pub async fn async_get<T>(&mut self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.async_ensure_auth().await?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.async_http_client
            .get(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let status = response.status().to_string();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::ApiError {
                code: status,
                message: error_text,
                provider: "PayPal".to_string(),
            });
        }

        response.json().await.map_err(PayupError::from)
    }

    pub fn post<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.ensure_auth()?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let status = response.status().to_string();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::ApiError {
                code: status,
                message: error_text,
                provider: "PayPal".to_string(),
            });
        }

        response.json().map_err(PayupError::from)
    }

    pub async fn async_post<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.async_ensure_auth().await?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.async_http_client
            .post(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let status = response.status().to_string();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::ApiError {
                code: status,
                message: error_text,
                provider: "PayPal".to_string(),
            });
        }

        response.json().await.map_err(PayupError::from)
    }

    pub fn patch<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.ensure_auth()?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.http_client
            .patch(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let status = response.status().to_string();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::ApiError {
                code: status,
                message: error_text,
                provider: "PayPal".to_string(),
            });
        }

        response.json().map_err(PayupError::from)
    }

    pub fn delete(&mut self, endpoint: &str) -> Result<bool> {
        self.ensure_auth()?;
        let url = format!("{}{}", self.config.environment.base_url(), endpoint);
        
        let response = self.http_client
            .delete(&url)
            .header("Authorization", self.auth.as_ref().unwrap().authorization_header())
            .send()
            .map_err(PayupError::from)?;

        Ok(response.status().is_success())
    }
}