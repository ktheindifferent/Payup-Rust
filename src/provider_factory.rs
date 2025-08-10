use std::sync::Arc;
use crate::error::{PayupError, Result};
use crate::payment_provider::PaymentProvider;
use crate::stripe::StripeProvider;
use crate::paypal::{PayPalProvider, PayPalEnvironment};
use crate::square::{SquareProvider, Environment as SquareEnvironment};

/// Configuration for creating payment providers
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// Provider name (stripe, paypal, square)
    pub provider: String,
    /// API key or access token
    pub api_key: String,
    /// Optional client secret (for PayPal)
    pub client_secret: Option<String>,
    /// Environment (sandbox/production)
    pub sandbox: bool,
}

/// Factory for creating payment provider instances
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create a new payment provider instance based on configuration
    pub fn create(config: ProviderConfig) -> Result<Arc<dyn PaymentProvider>> {
        match config.provider.to_lowercase().as_str() {
            "stripe" => {
                let provider = StripeProvider::new(config.api_key);
                Ok(Arc::new(provider))
            }
            "paypal" => {
                let client_secret = config.client_secret
                    .ok_or_else(|| PayupError::ValidationError(
                        "PayPal requires both client_id (api_key) and client_secret".to_string()
                    ))?;
                
                let environment = if config.sandbox {
                    PayPalEnvironment::Sandbox
                } else {
                    PayPalEnvironment::Live
                };
                
                let provider = PayPalProvider::new(
                    config.api_key,
                    client_secret,
                    environment
                )?;
                Ok(Arc::new(provider))
            }
            "square" => {
                let environment = if config.sandbox {
                    SquareEnvironment::Sandbox
                } else {
                    SquareEnvironment::Production
                };
                
                let provider = SquareProvider::new(config.api_key, environment)?;
                Ok(Arc::new(provider))
            }
            provider => {
                Err(PayupError::ValidationError(
                    format!("Unknown payment provider: {}", provider)
                ))
            }
        }
    }
    
    /// Create a provider from environment variables
    /// 
    /// Expected environment variables:
    /// - PAYMENT_PROVIDER: stripe, paypal, or square
    /// - PAYMENT_API_KEY: API key or client ID
    /// - PAYMENT_CLIENT_SECRET: Client secret (for PayPal)
    /// - PAYMENT_SANDBOX: true/false (default: false)
    pub fn from_env() -> Result<Arc<dyn PaymentProvider>> {
        let provider = std::env::var("PAYMENT_PROVIDER")
            .map_err(|_| PayupError::ValidationError(
                "PAYMENT_PROVIDER environment variable not set".to_string()
            ))?;
        
        let api_key = std::env::var("PAYMENT_API_KEY")
            .map_err(|_| PayupError::ValidationError(
                "PAYMENT_API_KEY environment variable not set".to_string()
            ))?;
        
        let client_secret = std::env::var("PAYMENT_CLIENT_SECRET").ok();
        
        let sandbox = std::env::var("PAYMENT_SANDBOX")
            .unwrap_or_else(|_| "false".to_string())
            .to_lowercase() == "true";
        
        Self::create(ProviderConfig {
            provider,
            api_key,
            client_secret,
            sandbox,
        })
    }
    
    /// Get a list of all available provider names
    pub fn available_providers() -> Vec<&'static str> {
        vec!["stripe", "paypal", "square"]
    }
    
    /// Check if a provider is available
    pub fn is_provider_available(name: &str) -> bool {
        Self::available_providers()
            .iter()
            .any(|&p| p == name.to_lowercase())
    }
}

/// Builder pattern for creating providers with more control
pub struct ProviderBuilder {
    provider: Option<String>,
    api_key: Option<String>,
    client_secret: Option<String>,
    sandbox: bool,
}

impl ProviderBuilder {
    pub fn new() -> Self {
        Self {
            provider: None,
            api_key: None,
            client_secret: None,
            sandbox: false,
        }
    }
    
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }
    
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }
    
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }
    
    pub fn build(self) -> Result<Arc<dyn PaymentProvider>> {
        let provider = self.provider
            .ok_or_else(|| PayupError::ValidationError("Provider name not specified".to_string()))?;
        
        let api_key = self.api_key
            .ok_or_else(|| PayupError::ValidationError("API key not specified".to_string()))?;
        
        ProviderFactory::create(ProviderConfig {
            provider,
            api_key,
            client_secret: self.client_secret,
            sandbox: self.sandbox,
        })
    }
}

impl Default for ProviderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provider_availability() {
        assert!(ProviderFactory::is_provider_available("stripe"));
        assert!(ProviderFactory::is_provider_available("paypal"));
        assert!(ProviderFactory::is_provider_available("square"));
        assert!(!ProviderFactory::is_provider_available("unknown"));
    }
    
    #[test]
    fn test_provider_builder() {
        let result = ProviderBuilder::new()
            .provider("stripe")
            .api_key("test_key")
            .sandbox(true)
            .build();
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_provider() {
        let result = ProviderFactory::create(ProviderConfig {
            provider: "unknown".to_string(),
            api_key: "test".to_string(),
            client_secret: None,
            sandbox: false,
        });
        
        assert!(result.is_err());
    }
}