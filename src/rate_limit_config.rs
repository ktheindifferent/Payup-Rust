use std::time::Duration;
use crate::rate_limiter::{RateLimit, get_rate_limiter};
use crate::circuit_breaker::{CircuitBreakerConfig, get_circuit_breaker};

/// Configuration for rate limiting and circuit breaking
pub struct RateLimitConfig {
    /// Provider-specific rate limit configurations
    pub stripe: Option<RateLimit>,
    pub paypal: Option<RateLimit>,
    pub square: Option<RateLimit>,
    
    /// Provider-specific circuit breaker configurations
    pub stripe_circuit: Option<CircuitBreakerConfig>,
    pub paypal_circuit: Option<CircuitBreakerConfig>,
    pub square_circuit: Option<CircuitBreakerConfig>,
    
    /// Custom endpoint configurations
    pub custom_endpoints: Vec<EndpointConfig>,
}

/// Configuration for a specific endpoint
pub struct EndpointConfig {
    /// Endpoint identifier (e.g., "stripe/payment_intents", "paypal/orders")
    pub endpoint: String,
    /// Rate limit configuration for this endpoint
    pub rate_limit: RateLimit,
    /// Circuit breaker configuration for this endpoint
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            stripe: Some(RateLimit {
                max_requests: 100,
                window: Duration::from_secs(1),
                auto_retry: true,
                max_retries: 3,
                initial_backoff: Duration::from_millis(500),
                max_backoff: Duration::from_secs(30),
            }),
            paypal: Some(RateLimit {
                max_requests: 30,
                window: Duration::from_secs(1),
                auto_retry: true,
                max_retries: 3,
                initial_backoff: Duration::from_secs(1),
                max_backoff: Duration::from_secs(60),
            }),
            square: Some(RateLimit {
                max_requests: 50,
                window: Duration::from_secs(1),
                auto_retry: true,
                max_retries: 3,
                initial_backoff: Duration::from_millis(750),
                max_backoff: Duration::from_secs(45),
            }),
            stripe_circuit: Some(CircuitBreakerConfig {
                failure_threshold: 5,
                success_threshold: 2,
                failure_window: Duration::from_secs(60),
                timeout: Duration::from_secs(30),
                half_open_max_requests: 3,
            }),
            paypal_circuit: Some(CircuitBreakerConfig {
                failure_threshold: 3,
                success_threshold: 2,
                failure_window: Duration::from_secs(60),
                timeout: Duration::from_secs(45),
                half_open_max_requests: 2,
            }),
            square_circuit: Some(CircuitBreakerConfig {
                failure_threshold: 4,
                success_threshold: 2,
                failure_window: Duration::from_secs(60),
                timeout: Duration::from_secs(30),
                half_open_max_requests: 3,
            }),
            custom_endpoints: Vec::new(),
        }
    }
}

impl RateLimitConfig {
    /// Create a new configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set Stripe rate limit configuration
    pub fn with_stripe_limit(mut self, limit: RateLimit) -> Self {
        self.stripe = Some(limit);
        self
    }
    
    /// Set PayPal rate limit configuration
    pub fn with_paypal_limit(mut self, limit: RateLimit) -> Self {
        self.paypal = Some(limit);
        self
    }
    
    /// Set Square rate limit configuration
    pub fn with_square_limit(mut self, limit: RateLimit) -> Self {
        self.square = Some(limit);
        self
    }
    
    /// Set Stripe circuit breaker configuration
    pub fn with_stripe_circuit(mut self, config: CircuitBreakerConfig) -> Self {
        self.stripe_circuit = Some(config);
        self
    }
    
    /// Set PayPal circuit breaker configuration
    pub fn with_paypal_circuit(mut self, config: CircuitBreakerConfig) -> Self {
        self.paypal_circuit = Some(config);
        self
    }
    
    /// Set Square circuit breaker configuration
    pub fn with_square_circuit(mut self, config: CircuitBreakerConfig) -> Self {
        self.square_circuit = Some(config);
        self
    }
    
    /// Add a custom endpoint configuration
    pub fn with_custom_endpoint(mut self, endpoint: impl Into<String>, rate_limit: RateLimit, circuit_breaker: Option<CircuitBreakerConfig>) -> Self {
        self.custom_endpoints.push(EndpointConfig {
            endpoint: endpoint.into(),
            rate_limit,
            circuit_breaker,
        });
        self
    }
    
    /// Apply this configuration to the global rate limiter and circuit breaker
    pub fn apply(&self) {
        let rate_limiter = get_rate_limiter();
        let circuit_breaker = get_circuit_breaker();
        
        // Apply provider-specific rate limits
        if let Some(ref limit) = self.stripe {
            rate_limiter.set_endpoint_limit("stripe", limit.clone());
        }
        if let Some(ref limit) = self.paypal {
            rate_limiter.set_endpoint_limit("paypal", limit.clone());
        }
        if let Some(ref limit) = self.square {
            rate_limiter.set_endpoint_limit("square", limit.clone());
        }
        
        // Apply provider-specific circuit breaker configs
        if let Some(ref config) = self.stripe_circuit {
            circuit_breaker.set_endpoint_config("stripe", config.clone());
        }
        if let Some(ref config) = self.paypal_circuit {
            circuit_breaker.set_endpoint_config("paypal", config.clone());
        }
        if let Some(ref config) = self.square_circuit {
            circuit_breaker.set_endpoint_config("square", config.clone());
        }
        
        // Apply custom endpoint configurations
        for endpoint_config in &self.custom_endpoints {
            rate_limiter.set_endpoint_limit(&endpoint_config.endpoint, endpoint_config.rate_limit.clone());
            
            if let Some(ref circuit_config) = endpoint_config.circuit_breaker {
                circuit_breaker.set_endpoint_config(&endpoint_config.endpoint, circuit_config.clone());
            }
        }
    }
}

/// Builder for creating custom rate limit configurations
pub struct RateLimitBuilder {
    max_requests: usize,
    window: Duration,
    auto_retry: bool,
    max_retries: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl Default for RateLimitBuilder {
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

impl RateLimitBuilder {
    /// Create a new rate limit builder with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the maximum number of requests allowed in the window
    pub fn max_requests(mut self, requests: usize) -> Self {
        self.max_requests = requests;
        self
    }
    
    /// Set the time window for the rate limit
    pub fn window(mut self, window: Duration) -> Self {
        self.window = window;
        self
    }
    
    /// Set whether to automatically retry on failure
    pub fn auto_retry(mut self, retry: bool) -> Self {
        self.auto_retry = retry;
        self
    }
    
    /// Set the maximum number of retries
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    
    /// Set the initial backoff duration for retries
    pub fn initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }
    
    /// Set the maximum backoff duration
    pub fn max_backoff(mut self, backoff: Duration) -> Self {
        self.max_backoff = backoff;
        self
    }
    
    /// Build the RateLimit configuration
    pub fn build(self) -> RateLimit {
        RateLimit {
            max_requests: self.max_requests,
            window: self.window,
            auto_retry: self.auto_retry,
            max_retries: self.max_retries,
            initial_backoff: self.initial_backoff,
            max_backoff: self.max_backoff,
        }
    }
}

/// Builder for creating custom circuit breaker configurations
pub struct CircuitBreakerBuilder {
    failure_threshold: u32,
    success_threshold: u32,
    failure_window: Duration,
    timeout: Duration,
    half_open_max_requests: u32,
}

impl Default for CircuitBreakerBuilder {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
            half_open_max_requests: 3,
        }
    }
}

impl CircuitBreakerBuilder {
    /// Create a new circuit breaker builder with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the number of failures before opening the circuit
    pub fn failure_threshold(mut self, threshold: u32) -> Self {
        self.failure_threshold = threshold;
        self
    }
    
    /// Set the success threshold to close the circuit from half-open state
    pub fn success_threshold(mut self, threshold: u32) -> Self {
        self.success_threshold = threshold;
        self
    }
    
    /// Set the time window for counting failures
    pub fn failure_window(mut self, window: Duration) -> Self {
        self.failure_window = window;
        self
    }
    
    /// Set the timeout before attempting to close the circuit
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set the maximum number of requests in half-open state
    pub fn half_open_max_requests(mut self, max_requests: u32) -> Self {
        self.half_open_max_requests = max_requests;
        self
    }
    
    /// Build the CircuitBreakerConfig
    pub fn build(self) -> CircuitBreakerConfig {
        CircuitBreakerConfig {
            failure_threshold: self.failure_threshold,
            success_threshold: self.success_threshold,
            failure_window: self.failure_window,
            timeout: self.timeout,
            half_open_max_requests: self.half_open_max_requests,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_builder() {
        let limit = RateLimitBuilder::new()
            .max_requests(50)
            .window(Duration::from_secs(30))
            .auto_retry(false)
            .max_retries(5)
            .initial_backoff(Duration::from_millis(100))
            .max_backoff(Duration::from_secs(10))
            .build();
        
        assert_eq!(limit.max_requests, 50);
        assert_eq!(limit.window, Duration::from_secs(30));
        assert_eq!(limit.auto_retry, false);
        assert_eq!(limit.max_retries, 5);
        assert_eq!(limit.initial_backoff, Duration::from_millis(100));
        assert_eq!(limit.max_backoff, Duration::from_secs(10));
    }

    #[test]
    fn test_circuit_breaker_builder() {
        let config = CircuitBreakerBuilder::new()
            .failure_threshold(10)
            .success_threshold(3)
            .failure_window(Duration::from_secs(120))
            .timeout(Duration::from_secs(60))
            .half_open_max_requests(5)
            .build();
        
        assert_eq!(config.failure_threshold, 10);
        assert_eq!(config.success_threshold, 3);
        assert_eq!(config.failure_window, Duration::from_secs(120));
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.half_open_max_requests, 5);
    }

    #[test]
    fn test_rate_limit_config() {
        let custom_stripe_limit = RateLimitBuilder::new()
            .max_requests(200)
            .window(Duration::from_secs(2))
            .build();
        
        let custom_circuit = CircuitBreakerBuilder::new()
            .failure_threshold(10)
            .timeout(Duration::from_secs(45))
            .build();
        
        let config = RateLimitConfig::new()
            .with_stripe_limit(custom_stripe_limit)
            .with_stripe_circuit(custom_circuit)
            .with_custom_endpoint(
                "stripe/special_endpoint",
                RateLimitBuilder::new().max_requests(10).build(),
                Some(CircuitBreakerBuilder::new().failure_threshold(2).build()),
            );
        
        // Verify the configuration was created correctly
        assert!(config.stripe.is_some());
        assert_eq!(config.stripe.as_ref().unwrap().max_requests, 200);
        assert_eq!(config.custom_endpoints.len(), 1);
        assert_eq!(config.custom_endpoints[0].endpoint, "stripe/special_endpoint");
    }
}