use std::time::Duration;

pub mod api {
    pub const STRIPE_BASE_URL: &str = "https://api.stripe.com/v1/";
    pub const STRIPE_API_VERSION: &str = "2020-08-27";
    
    pub const PAYPAL_BASE_URL: &str = "https://api.paypal.com/";
    pub const PAYPAL_SANDBOX_URL: &str = "https://api.sandbox.paypal.com/";
    
    pub const SQUARE_BASE_URL: &str = "https://connect.squareup.com/v2/";
    pub const SQUARE_SANDBOX_URL: &str = "https://connect.squareupsandbox.com/v2/";
}

pub mod timeouts {
    use super::Duration;
    
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    pub const LONG_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
    pub const WEBHOOK_TIMEOUT: Duration = Duration::from_secs(45);
    pub const RETRY_INITIAL_DELAY: Duration = Duration::from_millis(500);
    pub const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(300);
}

pub mod limits {
    pub const MAX_RETRY_ATTEMPTS: u32 = 3;
    pub const MAX_REQUESTS_PER_SECOND: u32 = 100;
    pub const MAX_REQUESTS_PER_MINUTE: u32 = 30;
    pub const MAX_BATCH_SIZE: usize = 50;
    pub const MAX_PAGE_SIZE: usize = 100;
    pub const DEFAULT_PAGE_SIZE: usize = 10;
}

pub mod http {
    pub const STATUS_OK: u16 = 200;
    pub const STATUS_CREATED: u16 = 201;
    pub const STATUS_NO_CONTENT: u16 = 204;
    pub const STATUS_BAD_REQUEST: u16 = 400;
    pub const STATUS_UNAUTHORIZED: u16 = 401;
    pub const STATUS_FORBIDDEN: u16 = 403;
    pub const STATUS_NOT_FOUND: u16 = 404;
    pub const STATUS_CONFLICT: u16 = 409;
    pub const STATUS_RATE_LIMITED: u16 = 429;
    pub const STATUS_SERVER_ERROR: u16 = 500;
    pub const STATUS_SERVICE_UNAVAILABLE: u16 = 503;
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub base_url: String,
    pub api_version: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
    pub use_sandbox: bool,
}

impl ProviderConfig {
    pub fn stripe(use_sandbox: bool) -> Self {
        Self {
            base_url: api::STRIPE_BASE_URL.to_string(),
            api_version: Some(api::STRIPE_API_VERSION.to_string()),
            timeout: timeouts::DEFAULT_REQUEST_TIMEOUT,
            max_retries: limits::MAX_RETRY_ATTEMPTS,
            use_sandbox,
        }
    }
    
    pub fn paypal(use_sandbox: bool) -> Self {
        let base_url = if use_sandbox {
            api::PAYPAL_SANDBOX_URL
        } else {
            api::PAYPAL_BASE_URL
        };
        
        Self {
            base_url: base_url.to_string(),
            api_version: None,
            timeout: timeouts::DEFAULT_REQUEST_TIMEOUT,
            max_retries: limits::MAX_RETRY_ATTEMPTS,
            use_sandbox,
        }
    }
    
    pub fn square(use_sandbox: bool) -> Self {
        let base_url = if use_sandbox {
            api::SQUARE_SANDBOX_URL
        } else {
            api::SQUARE_BASE_URL
        };
        
        Self {
            base_url: base_url.to_string(),
            api_version: None,
            timeout: timeouts::DEFAULT_REQUEST_TIMEOUT,
            max_retries: limits::MAX_RETRY_ATTEMPTS,
            use_sandbox,
        }
    }
}