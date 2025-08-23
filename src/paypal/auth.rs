use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use crate::error::{PayupError, Result};
use crate::http_client::{get_shared_client, get_shared_blocking_client};
use super::PayPalEnvironment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalAuth {
    pub access_token: String,
    pub token_type: String,
    pub app_id: String,
    pub expires_in: i64,
    pub nonce: String,
    #[serde(skip, default = "std::time::Instant::now")]
    pub created_at: std::time::Instant,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct TokenRequest {
    grant_type: String,
}

impl PayPalAuth {
    pub fn new(
        client_id: String,
        client_secret: String,
        environment: PayPalEnvironment,
    ) -> Result<Self> {
        let auth = Self::get_access_token(client_id, client_secret, environment)?;
        Ok(auth)
    }

    pub fn get_access_token(
        client_id: String,
        client_secret: String,
        environment: PayPalEnvironment,
    ) -> Result<Self> {
        let client = get_shared_blocking_client();
        let url = format!("{}/v1/oauth2/token", environment.base_url());
        
        // Create Basic auth header
        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Basic {}", encoded))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::AuthenticationError(format!(
                "PayPal authentication failed: {}",
                error_text
            )));
        }

        let mut auth: PayPalAuth = response.json().map_err(PayupError::from)?;
        auth.created_at = std::time::Instant::now();
        
        Ok(auth)
    }

    pub async fn async_get_access_token(
        client_id: String,
        client_secret: String,
        environment: PayPalEnvironment,
    ) -> Result<Self> {
        let client = get_shared_client();
        let url = format!("{}/v1/oauth2/token", environment.base_url());
        
        // Create Basic auth header
        let credentials = format!("{}:{}", client_id, client_secret);
        let encoded = general_purpose::STANDARD.encode(credentials.as_bytes());
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Basic {}", encoded))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=client_credentials")
            .send()
            .await
            .map_err(PayupError::from)?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(PayupError::AuthenticationError(format!(
                "PayPal authentication failed: {}",
                error_text
            )));
        }

        let mut auth: PayPalAuth = response.json().await.map_err(PayupError::from)?;
        auth.created_at = std::time::Instant::now();
        
        Ok(auth)
    }

    pub fn is_expired(&self) -> bool {
        let elapsed = self.created_at.elapsed().as_secs() as i64;
        elapsed >= self.expires_in - 60 // Refresh 1 minute before expiry
    }

    pub fn authorization_header(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}