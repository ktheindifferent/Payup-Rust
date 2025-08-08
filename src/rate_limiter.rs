use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use crate::error::{PayupError, Result};

/// Rate limiter for API calls with configurable limits per endpoint
#[derive(Clone)]
pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, EndpointLimit>>>,
    default_limit: RateLimit,
}

/// Rate limit configuration for a specific endpoint
#[derive(Debug, Clone)]
struct EndpointLimit {
    limit: RateLimit,
    requests: Vec<Instant>,
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Maximum number of requests allowed
    pub max_requests: usize,
    /// Time window for the rate limit
    pub window: Duration,
    /// Whether to enable automatic retry with backoff
    pub auto_retry: bool,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff duration for retries
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            auto_retry: true,
            max_retries: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
        }
    }
}

impl RateLimiter {
    /// Create a new rate limiter with default settings
    pub fn new() -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            default_limit: RateLimit::default(),
        }
    }

    /// Create a rate limiter with custom default limit
    pub fn with_default_limit(limit: RateLimit) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            default_limit: limit,
        }
    }

    /// Set a custom rate limit for a specific endpoint
    pub fn set_endpoint_limit(&self, endpoint: &str, limit: RateLimit) {
        let mut limits = self.limits.lock().unwrap();
        limits.insert(
            endpoint.to_string(),
            EndpointLimit {
                limit,
                requests: Vec::new(),
            },
        );
    }

    /// Check if a request can be made to the endpoint
    pub fn check_rate_limit(&self, endpoint: &str) -> Result<()> {
        let mut limits = self.limits.lock().unwrap();
        
        let endpoint_limit = limits
            .entry(endpoint.to_string())
            .or_insert_with(|| EndpointLimit {
                limit: self.default_limit.clone(),
                requests: Vec::new(),
            });

        let now = Instant::now();
        let window_start = now - endpoint_limit.limit.window;

        // Remove requests outside the current window
        endpoint_limit.requests.retain(|&req_time| req_time > window_start);

        // Check if we've exceeded the rate limit
        if endpoint_limit.requests.len() >= endpoint_limit.limit.max_requests {
            return Err(PayupError::RateLimitExceeded(
                format!("Rate limit exceeded for endpoint: {}. Max {} requests per {:?}", 
                    endpoint, 
                    endpoint_limit.limit.max_requests,
                    endpoint_limit.limit.window)
            ));
        }

        // Record this request
        endpoint_limit.requests.push(now);
        Ok(())
    }

    /// Wait until a request can be made (async)
    pub async fn wait_if_needed(&self, endpoint: &str) -> Result<()> {
        loop {
            match self.check_rate_limit(endpoint) {
                Ok(()) => return Ok(()),
                Err(PayupError::RateLimitExceeded(_)) => {
                    // Calculate wait time until the oldest request expires
                    let wait_time = self.calculate_wait_time(endpoint);
                    if wait_time > Duration::ZERO {
                        sleep(wait_time).await;
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Calculate how long to wait before the next request can be made
    fn calculate_wait_time(&self, endpoint: &str) -> Duration {
        let limits = self.limits.lock().unwrap();
        
        if let Some(endpoint_limit) = limits.get(endpoint) {
            if endpoint_limit.requests.is_empty() {
                return Duration::ZERO;
            }

            let now = Instant::now();
            let window_start = now - endpoint_limit.limit.window;
            
            // Find the oldest request in the current window
            if let Some(oldest_request) = endpoint_limit.requests.iter()
                .filter(|&&req_time| req_time > window_start)
                .min() {
                
                // Wait until this request expires from the window
                let expiry = *oldest_request + endpoint_limit.limit.window;
                if expiry > now {
                    return expiry - now;
                }
            }
        }
        
        Duration::ZERO
    }

    /// Execute a function with rate limiting and retry logic
    pub async fn execute_with_retry<F, T>(
        &self,
        endpoint: &str,
        f: F,
    ) -> Result<T>
    where
        F: Fn() -> Result<T> + Clone,
    {
        let limit = {
            let limits = self.limits.lock().unwrap();
            limits.get(endpoint)
                .map(|el| el.limit.clone())
                .unwrap_or_else(|| self.default_limit.clone())
        };

        let mut attempt = 0;
        let mut backoff = limit.initial_backoff;

        loop {
            // Wait for rate limit if needed
            self.wait_if_needed(endpoint).await?;

            // Try to execute the function
            match f() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Check if we should retry
                    if !limit.auto_retry || attempt >= limit.max_retries {
                        return Err(e);
                    }

                    // Check if the error is retryable
                    if !is_retryable_error(&e) {
                        return Err(e);
                    }

                    attempt += 1;
                    
                    // Wait with exponential backoff
                    sleep(backoff).await;
                    
                    // Increase backoff for next attempt
                    backoff = std::cmp::min(backoff * 2, limit.max_backoff);
                }
            }
        }
    }

    /// Execute an async function with rate limiting and retry logic
    pub async fn execute_with_retry_async<F, T, Fut>(
        &self,
        endpoint: &str,
        f: F,
    ) -> Result<T>
    where
        F: Fn() -> Fut + Clone,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let limit = {
            let limits = self.limits.lock().unwrap();
            limits.get(endpoint)
                .map(|el| el.limit.clone())
                .unwrap_or_else(|| self.default_limit.clone())
        };

        let mut attempt = 0;
        let mut backoff = limit.initial_backoff;

        loop {
            // Wait for rate limit if needed
            self.wait_if_needed(endpoint).await?;

            // Try to execute the function
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    // Check if we should retry
                    if !limit.auto_retry || attempt >= limit.max_retries {
                        return Err(e);
                    }

                    // Check if the error is retryable
                    if !is_retryable_error(&e) {
                        return Err(e);
                    }

                    attempt += 1;
                    
                    // Wait with exponential backoff
                    sleep(backoff).await;
                    
                    // Increase backoff for next attempt
                    backoff = std::cmp::min(backoff * 2, limit.max_backoff);
                }
            }
        }
    }

    /// Reset rate limit tracking for an endpoint
    pub fn reset_endpoint(&self, endpoint: &str) {
        let mut limits = self.limits.lock().unwrap();
        if let Some(endpoint_limit) = limits.get_mut(endpoint) {
            endpoint_limit.requests.clear();
        }
    }

    /// Reset all rate limit tracking
    pub fn reset_all(&self) {
        let mut limits = self.limits.lock().unwrap();
        for endpoint_limit in limits.values_mut() {
            endpoint_limit.requests.clear();
        }
    }
}

/// Check if an error is retryable
fn is_retryable_error(error: &PayupError) -> bool {
    match error {
        PayupError::NetworkError(_) => true,
        PayupError::RateLimitExceeded(_) => true,
        PayupError::ServerError(status) if *status >= 500 => true,
        PayupError::TimeoutError(_) => true,
        _ => false,
    }
}

/// Global rate limiter instance
static GLOBAL_RATE_LIMITER: once_cell::sync::Lazy<RateLimiter> = 
    once_cell::sync::Lazy::new(|| {
        let limiter = RateLimiter::new();
        
        // Configure default limits for different providers
        let stripe_limit = RateLimit {
            max_requests: 100,
            window: Duration::from_secs(1),
            auto_retry: true,
            max_retries: 3,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
        };
        
        let paypal_limit = RateLimit {
            max_requests: 30,
            window: Duration::from_secs(1),
            auto_retry: true,
            max_retries: 3,
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(60),
        };
        
        let square_limit = RateLimit {
            max_requests: 50,
            window: Duration::from_secs(1),
            auto_retry: true,
            max_retries: 3,
            initial_backoff: Duration::from_millis(750),
            max_backoff: Duration::from_secs(45),
        };
        
        // Set provider-specific limits
        limiter.set_endpoint_limit("stripe", stripe_limit);
        limiter.set_endpoint_limit("paypal", paypal_limit);
        limiter.set_endpoint_limit("square", square_limit);
        
        limiter
    });

/// Get the global rate limiter instance
pub fn get_rate_limiter() -> &'static RateLimiter {
    &GLOBAL_RATE_LIMITER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_creation() {
        let limiter = RateLimiter::new();
        assert!(limiter.check_rate_limit("test").is_ok());
    }

    #[test]
    fn test_rate_limit_exceeded() {
        let limit = RateLimit {
            max_requests: 2,
            window: Duration::from_secs(1),
            auto_retry: false,
            max_retries: 0,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
        };
        
        let limiter = RateLimiter::with_default_limit(limit);
        
        // First two requests should succeed
        assert!(limiter.check_rate_limit("test").is_ok());
        assert!(limiter.check_rate_limit("test").is_ok());
        
        // Third request should fail
        assert!(limiter.check_rate_limit("test").is_err());
    }

    #[tokio::test]
    async fn test_wait_if_needed() {
        let limit = RateLimit {
            max_requests: 1,
            window: Duration::from_millis(100),
            auto_retry: true,
            max_retries: 3,
            initial_backoff: Duration::from_millis(10),
            max_backoff: Duration::from_millis(100),
        };
        
        let limiter = RateLimiter::with_default_limit(limit);
        
        // First request should succeed immediately
        assert!(limiter.wait_if_needed("test").await.is_ok());
        
        // Second request should wait and then succeed
        let start = Instant::now();
        assert!(limiter.wait_if_needed("test").await.is_ok());
        let elapsed = start.elapsed();
        
        // Should have waited approximately the window duration
        assert!(elapsed >= Duration::from_millis(90));
    }

    #[test]
    fn test_endpoint_specific_limits() {
        let limiter = RateLimiter::new();
        
        // Set a strict limit for a specific endpoint
        let strict_limit = RateLimit {
            max_requests: 1,
            window: Duration::from_secs(1),
            auto_retry: false,
            max_retries: 0,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
        };
        
        limiter.set_endpoint_limit("strict", strict_limit);
        
        // Strict endpoint should allow only 1 request
        assert!(limiter.check_rate_limit("strict").is_ok());
        assert!(limiter.check_rate_limit("strict").is_err());
        
        // Other endpoints should use default limit (100 requests)
        for _ in 0..10 {
            assert!(limiter.check_rate_limit("normal").is_ok());
        }
    }

    #[test]
    fn test_reset_endpoint() {
        let limit = RateLimit {
            max_requests: 1,
            window: Duration::from_secs(10),
            auto_retry: false,
            max_retries: 0,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1),
        };
        
        let limiter = RateLimiter::with_default_limit(limit);
        
        // Use up the limit
        assert!(limiter.check_rate_limit("test").is_ok());
        assert!(limiter.check_rate_limit("test").is_err());
        
        // Reset the endpoint
        limiter.reset_endpoint("test");
        
        // Should be able to make requests again
        assert!(limiter.check_rate_limit("test").is_ok());
    }
}