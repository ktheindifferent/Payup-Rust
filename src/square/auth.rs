use crate::error::{PayupError, Result};
use super::Environment;

#[derive(Debug, Clone)]
pub struct SquareAuth {
    pub access_token: String,
    pub environment: Environment,
}

impl SquareAuth {
    pub fn new(access_token: String, environment: Environment) -> Self {
        Self {
            access_token,
            environment,
        }
    }

    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    pub fn base_url(&self) -> &str {
        self.environment.base_url()
    }

    pub fn validate(&self) -> Result<()> {
        if self.access_token.is_empty() {
            return Err(PayupError::AuthenticationError(
                "Square access token is empty".to_string()
            ));
        }
        Ok(())
    }
}