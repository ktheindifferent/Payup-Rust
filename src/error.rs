use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum PayupError {
    // Network errors
    NetworkError(reqwest::Error),
    
    // API errors
    ApiError {
        code: String,
        message: String,
        provider: String,
    },
    
    // Authentication errors
    AuthenticationError(String),
    
    // Validation errors
    ValidationError(String),
    
    // Rate limiting
    RateLimitError {
        retry_after: Option<u64>,
    },
    
    // Rate limit exceeded (for internal rate limiter)
    RateLimitExceeded(String),
    
    // Timeout error
    TimeoutError(String),
    
    // Server error
    ServerError(u16),
    
    // Serialization/Deserialization errors
    SerializationError(serde_json::Error),
    
    // Generic errors
    GenericError(String),
    
    // Provider-specific errors
    StripeError {
        error_type: String,
        code: Option<String>,
        message: String,
        param: Option<String>,
    },
    
    PayPalError {
        name: String,
        message: String,
        debug_id: Option<String>,
        details: Option<Vec<PayPalErrorDetail>>,
    },
    
    // Unsupported operation
    UnsupportedOperation(String),
}

#[derive(Debug)]
pub struct PayPalErrorDetail {
    pub field: Option<String>,
    pub issue: String,
    pub description: Option<String>,
}

impl fmt::Display for PayupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PayupError::NetworkError(e) => 
                write!(f, "Network error: {}", e),
            
            PayupError::ApiError { code, message, provider } => 
                write!(f, "{} API error {}: {}", provider, code, message),
            
            PayupError::AuthenticationError(msg) => 
                write!(f, "Authentication error: {}", msg),
            
            PayupError::ValidationError(msg) => 
                write!(f, "Validation error: {}", msg),
            
            PayupError::RateLimitError { retry_after } => 
                self.format_rate_limit_error(f, *retry_after),
            
            PayupError::RateLimitExceeded(msg) => 
                write!(f, "Rate limit exceeded: {}", msg),
            
            PayupError::TimeoutError(msg) => 
                write!(f, "Timeout error: {}", msg),
            
            PayupError::ServerError(status) => 
                write!(f, "Server error: HTTP {}", status),
            
            PayupError::SerializationError(e) => 
                write!(f, "Serialization error: {}", e),
            
            PayupError::GenericError(msg) => 
                write!(f, "Error: {}", msg),
            
            PayupError::StripeError { error_type, code, message, param } => 
                self.format_stripe_error(f, error_type, code.as_deref(), message, param.as_deref()),
            
            PayupError::PayPalError { name, message, debug_id, .. } => 
                self.format_paypal_error(f, name, message, debug_id.as_deref()),
            
            PayupError::UnsupportedOperation(msg) => 
                write!(f, "Unsupported operation: {}", msg),
        }
    }
}

impl PayupError {
    fn format_rate_limit_error(&self, f: &mut fmt::Formatter<'_>, retry_after: Option<u64>) -> fmt::Result {
        match retry_after {
            Some(seconds) => write!(f, "Rate limit exceeded. Retry after {} seconds", seconds),
            None => write!(f, "Rate limit exceeded"),
        }
    }

    fn format_stripe_error(
        &self,
        f: &mut fmt::Formatter<'_>,
        error_type: &str,
        code: Option<&str>,
        message: &str,
        param: Option<&str>,
    ) -> fmt::Result {
        write!(f, "Stripe {} error", error_type)?;
        if let Some(c) = code {
            write!(f, " ({})", c)?;
        }
        write!(f, ": {}", message)?;
        if let Some(p) = param {
            write!(f, " [param: {}]", p)?;
        }
        Ok(())
    }

    fn format_paypal_error(
        &self,
        f: &mut fmt::Formatter<'_>,
        name: &str,
        message: &str,
        debug_id: Option<&str>,
    ) -> fmt::Result {
        write!(f, "PayPal error {}: {}", name, message)?;
        if let Some(id) = debug_id {
            write!(f, " [debug_id: {}]", id)?;
        }
        Ok(())
    }
}

impl StdError for PayupError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            PayupError::NetworkError(e) => Some(e),
            PayupError::SerializationError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for PayupError {
    fn from(err: reqwest::Error) -> Self {
        PayupError::NetworkError(err)
    }
}

impl From<serde_json::Error> for PayupError {
    fn from(err: serde_json::Error) -> Self {
        PayupError::SerializationError(err)
    }
}

pub type Result<T> = std::result::Result<T, PayupError>;