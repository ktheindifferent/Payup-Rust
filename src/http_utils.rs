use reqwest::blocking::{RequestBuilder, Response};
use reqwest::{RequestBuilder as AsyncRequestBuilder, Response as AsyncResponse};
use serde::Deserialize;
use crate::error::{PayupError, Result};

pub struct HttpRequestBuilder {
    provider_name: String,
}

impl HttpRequestBuilder {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider_name: provider.into(),
        }
    }

    pub fn process_response<T>(&self, response: Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.handle_response_status(response)?;
        response.json().map_err(PayupError::from)
    }

    pub async fn process_async_response<T>(&self, response: AsyncResponse) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.handle_async_response_status(response).await?.json().await.map_err(PayupError::from)
    }

    fn handle_response_status(&self, response: Response) -> Result<Response> {
        if response.status().is_success() {
            return Ok(response);
        }

        let status = response.status().to_string();
        let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
        
        Err(PayupError::ApiError {
            code: status,
            message: error_text,
            provider: self.provider_name.clone(),
        })
    }

    async fn handle_async_response_status(&self, response: AsyncResponse) -> Result<AsyncResponse> {
        if response.status().is_success() {
            return Ok(response);
        }

        let status = response.status().to_string();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        
        Err(PayupError::ApiError {
            code: status,
            message: error_text,
            provider: self.provider_name.clone(),
        })
    }

    pub fn add_common_headers(mut builder: RequestBuilder, auth_header: &str) -> RequestBuilder {
        builder = builder
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json");
        builder
    }

    pub fn add_async_common_headers(mut builder: AsyncRequestBuilder, auth_header: &str) -> AsyncRequestBuilder {
        builder = builder
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json");
        builder
    }
}

pub trait HttpMethod {
    fn method_name(&self) -> &str;
}

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod for Method {
    fn method_name(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
        }
    }
}

pub fn build_url(base_url: &str, endpoint: &str) -> String {
    format!("{}{}", base_url, endpoint)
}

pub trait ResponseHandler {
    fn extract_error_details(&self) -> (String, String);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let result = build_url("https://api.example.com", "/v1/users");
        assert_eq!(result, "https://api.example.com/v1/users");
    }

    #[test]
    fn test_method_names() {
        assert_eq!(Method::Get.method_name(), "GET");
        assert_eq!(Method::Post.method_name(), "POST");
        assert_eq!(Method::Put.method_name(), "PUT");
        assert_eq!(Method::Patch.method_name(), "PATCH");
        assert_eq!(Method::Delete.method_name(), "DELETE");
    }
}