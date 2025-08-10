use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::{PayupError, Result};

/// Circuit breaker states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Success threshold to close the circuit from half-open state
    pub success_threshold: u32,
    /// Time window for counting failures
    pub failure_window: Duration,
    /// Timeout before attempting to close the circuit
    pub timeout: Duration,
    /// Maximum number of requests in half-open state
    pub half_open_max_requests: u32,
}

impl Default for CircuitBreakerConfig {
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

/// Circuit breaker statistics
#[derive(Debug, Clone)]
struct CircuitStats {
    failures: Vec<Instant>,
    successes: u32,
    half_open_requests: u32,
    last_failure_time: Option<Instant>,
    state: CircuitState,
}

impl Default for CircuitStats {
    fn default() -> Self {
        Self {
            failures: Vec::new(),
            successes: 0,
            half_open_requests: 0,
            last_failure_time: None,
            state: CircuitState::Closed,
        }
    }
}

/// Circuit breaker implementation
#[derive(Clone)]
pub struct CircuitBreaker {
    configs: Arc<Mutex<HashMap<String, CircuitBreakerConfig>>>,
    stats: Arc<Mutex<HashMap<String, CircuitStats>>>,
    default_config: CircuitBreakerConfig,
}

impl CircuitBreaker {
    /// Create a new circuit breaker with default configuration
    pub fn new() -> Self {
        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(HashMap::new())),
            default_config: CircuitBreakerConfig::default(),
        }
    }

    /// Create a circuit breaker with custom default configuration
    pub fn with_default_config(config: CircuitBreakerConfig) -> Self {
        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(HashMap::new())),
            default_config: config,
        }
    }

    /// Set configuration for a specific endpoint
    pub fn set_endpoint_config(&self, endpoint: &str, config: CircuitBreakerConfig) {
        let mut configs = self.configs.lock().unwrap();
        configs.insert(endpoint.to_string(), config);
    }

    /// Check if the circuit allows a request
    pub fn check_circuit(&self, endpoint: &str) -> Result<()> {
        let mut stats_map = self.stats.lock().unwrap();
        let configs = self.configs.lock().unwrap();
        
        let config = configs.get(endpoint).unwrap_or(&self.default_config);
        let stats = stats_map.entry(endpoint.to_string()).or_default();
        
        // Update circuit state based on current conditions
        self.update_state(stats, config);
        
        match stats.state {
            CircuitState::Open => {
                Err(PayupError::GenericError(format!(
                    "Circuit breaker is open for endpoint: {}. Service temporarily unavailable.",
                    endpoint
                )))
            }
            CircuitState::HalfOpen => {
                if stats.half_open_requests >= config.half_open_max_requests {
                    Err(PayupError::GenericError(format!(
                        "Circuit breaker is half-open for endpoint: {}. Maximum requests reached.",
                        endpoint
                    )))
                } else {
                    stats.half_open_requests += 1;
                    Ok(())
                }
            }
            CircuitState::Closed => Ok(()),
        }
    }

    /// Update circuit state based on current conditions
    fn update_state(&self, stats: &mut CircuitStats, config: &CircuitBreakerConfig) {
        let now = Instant::now();
        
        // Remove old failures outside the window
        let window_start = now - config.failure_window;
        stats.failures.retain(|&failure_time| failure_time > window_start);
        
        match stats.state {
            CircuitState::Closed => {
                // Check if we should open the circuit
                if stats.failures.len() >= config.failure_threshold as usize {
                    stats.state = CircuitState::Open;
                    stats.last_failure_time = Some(now);
                }
            }
            CircuitState::Open => {
                // Check if timeout has passed
                if let Some(last_failure) = stats.last_failure_time {
                    if now.duration_since(last_failure) >= config.timeout {
                        stats.state = CircuitState::HalfOpen;
                        stats.half_open_requests = 0;
                        stats.successes = 0;
                    }
                }
            }
            CircuitState::HalfOpen => {
                // Check if we should close the circuit
                if stats.successes >= config.success_threshold {
                    stats.state = CircuitState::Closed;
                    stats.failures.clear();
                    stats.successes = 0;
                    stats.half_open_requests = 0;
                }
            }
        }
    }

    /// Record a successful request
    pub fn record_success(&self, endpoint: &str) {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(endpoint.to_string()).or_default();
        
        if stats.state == CircuitState::HalfOpen {
            stats.successes += 1;
            
            // Check if we should close the circuit
            let configs = self.configs.lock().unwrap();
            let config = configs.get(endpoint).unwrap_or(&self.default_config);
            
            if stats.successes >= config.success_threshold {
                stats.state = CircuitState::Closed;
                stats.failures.clear();
                stats.successes = 0;
                stats.half_open_requests = 0;
            }
        }
    }

    /// Record a failed request
    pub fn record_failure(&self, endpoint: &str) {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(endpoint.to_string()).or_default();
        
        let now = Instant::now();
        stats.failures.push(now);
        
        if stats.state == CircuitState::HalfOpen {
            // Failed in half-open state, reopen the circuit
            stats.state = CircuitState::Open;
            stats.last_failure_time = Some(now);
            stats.half_open_requests = 0;
            stats.successes = 0;
        }
    }

    /// Execute a function with circuit breaker protection
    pub async fn execute<F, T>(&self, endpoint: &str, f: F) -> Result<T>
    where
        F: Fn() -> Result<T>,
    {
        // Check if circuit allows the request
        self.check_circuit(endpoint)?;
        
        // Execute the function
        match f() {
            Ok(result) => {
                self.record_success(endpoint);
                Ok(result)
            }
            Err(e) => {
                // Only record failure for certain error types
                if Self::is_circuit_breaking_error(&e) {
                    self.record_failure(endpoint);
                }
                Err(e)
            }
        }
    }

    /// Execute an async function with circuit breaker protection
    pub async fn execute_async<F, T, Fut>(&self, endpoint: &str, f: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Check if circuit allows the request
        self.check_circuit(endpoint)?;
        
        // Execute the function
        match f().await {
            Ok(result) => {
                self.record_success(endpoint);
                Ok(result)
            }
            Err(e) => {
                // Only record failure for certain error types
                if Self::is_circuit_breaking_error(&e) {
                    self.record_failure(endpoint);
                }
                Err(e)
            }
        }
    }

    /// Check if an error should trigger circuit breaker
    fn is_circuit_breaking_error(error: &PayupError) -> bool {
        match error {
            PayupError::NetworkError(_) => true,
            PayupError::ServerError(status) if *status >= 500 => true,
            PayupError::TimeoutError(_) => true,
            _ => false,
        }
    }

    /// Get the current state of a circuit
    pub fn get_state(&self, endpoint: &str) -> CircuitState {
        let mut stats_map = self.stats.lock().unwrap();
        let configs = self.configs.lock().unwrap();
        
        let config = configs.get(endpoint).unwrap_or(&self.default_config);
        let stats = stats_map.entry(endpoint.to_string()).or_default();
        
        self.update_state(stats, config);
        stats.state
    }

    /// Reset a circuit
    pub fn reset(&self, endpoint: &str) {
        let mut stats_map = self.stats.lock().unwrap();
        if let Some(stats) = stats_map.get_mut(endpoint) {
            *stats = CircuitStats::default();
        }
    }

    /// Reset all circuits
    pub fn reset_all(&self) {
        let mut stats_map = self.stats.lock().unwrap();
        stats_map.clear();
    }
}

/// Global circuit breaker instance
static GLOBAL_CIRCUIT_BREAKER: once_cell::sync::Lazy<CircuitBreaker> = 
    once_cell::sync::Lazy::new(|| {
        let breaker = CircuitBreaker::new();
        
        // Configure circuit breakers for different providers
        let stripe_config = CircuitBreakerConfig {
            failure_threshold: 5,
            success_threshold: 2,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
            half_open_max_requests: 3,
        };
        
        let paypal_config = CircuitBreakerConfig {
            failure_threshold: 3,
            success_threshold: 2,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_secs(45),
            half_open_max_requests: 2,
        };
        
        let square_config = CircuitBreakerConfig {
            failure_threshold: 4,
            success_threshold: 2,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
            half_open_max_requests: 3,
        };
        
        breaker.set_endpoint_config("stripe", stripe_config);
        breaker.set_endpoint_config("paypal", paypal_config);
        breaker.set_endpoint_config("square", square_config);
        
        breaker
    });

/// Get the global circuit breaker instance
pub fn get_circuit_breaker() -> &'static CircuitBreaker {
    &GLOBAL_CIRCUIT_BREAKER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_opens_on_failures() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 1,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_secs(10),
            half_open_max_requests: 1,
        };
        
        let breaker = CircuitBreaker::with_default_config(config);
        
        // Circuit should be closed initially
        assert_eq!(breaker.get_state("test"), CircuitState::Closed);
        
        // Record failures
        breaker.record_failure("test");
        assert_eq!(breaker.get_state("test"), CircuitState::Closed);
        
        breaker.record_failure("test");
        assert_eq!(breaker.get_state("test"), CircuitState::Open);
        
        // Circuit should reject requests when open
        assert!(breaker.check_circuit("test").is_err());
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_state() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            success_threshold: 1,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_millis(100),
            half_open_max_requests: 2,
        };
        
        let breaker = CircuitBreaker::with_default_config(config);
        
        // Open the circuit
        breaker.record_failure("test");
        assert_eq!(breaker.get_state("test"), CircuitState::Open);
        
        // Wait for timeout
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should transition to half-open
        assert_eq!(breaker.get_state("test"), CircuitState::HalfOpen);
        
        // Should allow limited requests
        assert!(breaker.check_circuit("test").is_ok());
        assert!(breaker.check_circuit("test").is_ok());
        assert!(breaker.check_circuit("test").is_err()); // Exceeds max requests
        
        // Record success to close the circuit
        breaker.record_success("test");
        assert_eq!(breaker.get_state("test"), CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_reopens_on_half_open_failure() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            success_threshold: 2,
            failure_window: Duration::from_secs(60),
            timeout: Duration::from_millis(1),
            half_open_max_requests: 3,
        };
        
        let breaker = CircuitBreaker::with_default_config(config);
        
        // Open and transition to half-open
        breaker.record_failure("test");
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(breaker.get_state("test"), CircuitState::HalfOpen);
        
        // Failure in half-open should reopen
        breaker.record_failure("test");
        assert_eq!(breaker.get_state("test"), CircuitState::Open);
    }
}