use reqwest::blocking::Client as HttpClient;
use reqwest::Client as AsyncHttpClient;
use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use crate::http_utils::{HttpRequestBuilder, build_url};
use super::{PayPalConfig, PayPalAuth};

pub struct PayPalClient {
    pub config: PayPalConfig,
    pub auth: Option<PayPalAuth>,
    http_client: HttpClient,
    async_http_client: AsyncHttpClient,
    request_builder: HttpRequestBuilder,
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
            request_builder: HttpRequestBuilder::new("PayPal"),
        })
    }

    pub fn ensure_auth(&mut self) -> Result<()> {
        if self.needs_auth_refresh() {
            self.refresh_auth()?;
        }
        Ok(())
    }

    fn needs_auth_refresh(&self) -> bool {
        self.auth.as_ref().map_or(true, |auth| auth.is_expired())
    }

    fn refresh_auth(&mut self) -> Result<()> {
        self.auth = Some(PayPalAuth::new(
            self.config.client_id.clone(),
            self.config.client_secret.clone(),
            self.config.environment.clone(),
        )?);
        Ok(())
    }

    pub async fn async_ensure_auth(&mut self) -> Result<()> {
        if self.needs_auth_refresh() {
            self.auth = Some(PayPalAuth::async_get_access_token(
                self.config.client_id.clone(),
                self.config.client_secret.clone(),
                self.config.environment.clone(),
            ).await?);
        }
        Ok(())
    }

    fn get_auth_header(&self) -> Result<String> {
        self.auth
            .as_ref()
            .map(|auth| auth.authorization_header())
            .ok_or_else(|| PayupError::AuthenticationError("No authentication available".to_string()))
    }

    pub fn get<T>(&mut self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.ensure_auth()?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.http_client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .send()
            .map_err(PayupError::from)?;

        self.request_builder.process_response(response)
    }

    pub async fn async_get<T>(&mut self, endpoint: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.async_ensure_auth().await?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.async_http_client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(PayupError::from)?;

        self.request_builder.process_async_response(response).await
    }

    pub fn post<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.ensure_auth()?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        self.request_builder.process_response(response)
    }

    pub async fn async_post<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.async_ensure_auth().await?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.async_http_client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(PayupError::from)?;

        self.request_builder.process_async_response(response).await
    }

    pub fn patch<T, B>(&mut self, endpoint: &str, body: &B) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize,
    {
        self.ensure_auth()?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.http_client
            .patch(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .map_err(PayupError::from)?;

        self.request_builder.process_response(response)
    }

    pub fn delete(&mut self, endpoint: &str) -> Result<bool> {
        self.ensure_auth()?;
        let url = build_url(self.config.environment.base_url(), endpoint);
        let auth_header = self.get_auth_header()?;
        
        let response = self.http_client
            .delete(&url)
            .header("Authorization", auth_header)
            .send()
            .map_err(PayupError::from)?;

        Ok(response.status().is_success())
    }
}