use payup::rate_limiter::{RateLimiter, RateLimit, get_rate_limiter};
use payup::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, get_circuit_breaker};
use payup::rate_limit_config::{RateLimitConfig, RateLimitBuilder, CircuitBreakerBuilder};
use payup::error::PayupError;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio;

#[tokio::test]
async fn test_rate_limiter_enforces_limits() {
    let rate_limiter = RateLimiter::with_default_limit(RateLimit {
        max_requests: 3,
        window: Duration::from_secs(1),
        auto_retry: false,
        max_retries: 0,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(1),
    });
    
    // First 3 requests should succeed (same endpoint)
    for _ in 0..3 {
        let result = rate_limiter.check_rate_limit("test_endpoint").unwrap();
        assert_eq!(result, ());
    }
    
    // 4th request should fail
    let result = rate_limiter.check_rate_limit("test_endpoint");
    assert!(result.is_err());
    match result {
        Err(PayupError::RateLimitExceeded(_)) => {},
        _ => panic!("Expected RateLimitExceeded error"),
    }
}

#[tokio::test]
async fn test_rate_limiter_wait_if_needed() {
    let rate_limiter = RateLimiter::with_default_limit(RateLimit {
        max_requests: 2,
        window: Duration::from_millis(500),
        auto_retry: true,
        max_retries: 3,
        initial_backoff: Duration::from_millis(50),
        max_backoff: Duration::from_millis(200),
    });
    
    let start = Instant::now();
    
    // First two requests should succeed immediately
    rate_limiter.wait_if_needed("test_wait").await.unwrap();
    rate_limiter.wait_if_needed("test_wait").await.unwrap();
    
    // Third request should wait
    rate_limiter.wait_if_needed("test_wait").await.unwrap();
    
    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::from_millis(400), "Should have waited at least 400ms, but only waited {:?}", elapsed);
}

#[tokio::test]
async fn test_rate_limiter_with_retry() {
    let rate_limiter = RateLimiter::with_default_limit(RateLimit {
        max_requests: 10,
        window: Duration::from_secs(1),
        auto_retry: true,
        max_retries: 3,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(1),
    });
    
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();
    
    // Function that fails twice then succeeds
    let result = rate_limiter.execute_with_retry_async("test_retry", move || {
        let attempt_count = attempt_count_clone.clone();
        async move {
            let attempts = attempt_count.fetch_add(1, Ordering::SeqCst);
            if attempts < 2 {
                // Use ServerError which is retryable
                Err(PayupError::ServerError(500))
            } else {
                Ok("Success".to_string())
            }
        }
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_circuit_breaker_opens_on_failures() {
    let circuit_breaker = CircuitBreaker::with_default_config(CircuitBreakerConfig {
        failure_threshold: 2,
        success_threshold: 1,
        failure_window: Duration::from_secs(60),
        timeout: Duration::from_millis(500),
        half_open_max_requests: 1,
    });
    
    // Record failures to open the circuit
    circuit_breaker.record_failure("test_circuit");
    circuit_breaker.record_failure("test_circuit");
    
    // Circuit should be open now
    let result = circuit_breaker.check_circuit("test_circuit");
    assert!(result.is_err());
    
    // Wait for timeout
    tokio::time::sleep(Duration::from_millis(600)).await;
    
    // Circuit should be half-open now
    let result = circuit_breaker.check_circuit("test_circuit");
    assert!(result.is_ok());
    
    // Record success to close the circuit
    circuit_breaker.record_success("test_circuit");
    
    // Circuit should be closed now
    let result = circuit_breaker.check_circuit("test_circuit");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_rate_limiter_with_circuit_breaker() {
    // This test verifies that rate limiter and circuit breaker work together
    let rate_limiter = get_rate_limiter();
    let circuit_breaker = get_circuit_breaker();
    
    // Configure a test endpoint
    rate_limiter.set_endpoint_limit("test_integration", RateLimit {
        max_requests: 5,
        window: Duration::from_secs(1),
        auto_retry: true,
        max_retries: 2,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_millis(500),
    });
    
    circuit_breaker.set_endpoint_config("test_integration", CircuitBreakerConfig {
        failure_threshold: 3,
        success_threshold: 2,
        failure_window: Duration::from_secs(60),
        timeout: Duration::from_millis(500),
        half_open_max_requests: 2,
    });
    
    // Reset to ensure clean state
    circuit_breaker.reset("test_integration");
    rate_limiter.reset_endpoint("test_integration");
    
    // Function that succeeds after a few retries
    let failure_count = Arc::new(AtomicU32::new(0));
    let failure_count_clone = failure_count.clone();
    
    let result = rate_limiter.execute_with_retry_async("test_integration", move || {
        let count = failure_count_clone.fetch_add(1, Ordering::SeqCst);
        async move {
            // Succeed on the second attempt (after 1 retry)
            if count < 1 {
                Err(PayupError::ServerError(500))
            } else {
                Ok("Success".to_string())
            }
        }
    }).await;
    
    // Should eventually succeed after retries
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
}

#[tokio::test]
async fn test_custom_configuration() {
    let config = RateLimitConfig::new()
        .with_stripe_limit(
            RateLimitBuilder::new()
                .max_requests(200)
                .window(Duration::from_secs(2))
                .auto_retry(true)
                .max_retries(5)
                .build()
        )
        .with_stripe_circuit(
            CircuitBreakerBuilder::new()
                .failure_threshold(10)
                .success_threshold(3)
                .timeout(Duration::from_secs(45))
                .build()
        )
        .with_custom_endpoint(
            "custom_endpoint",
            RateLimitBuilder::new()
                .max_requests(10)
                .window(Duration::from_secs(10))
                .build(),
            Some(CircuitBreakerBuilder::new()
                .failure_threshold(2)
                .build())
        );
    
    // Apply the configuration
    config.apply();
    
    let rate_limiter = get_rate_limiter();
    let circuit_breaker = get_circuit_breaker();
    
    // Test that custom endpoint configuration is applied
    rate_limiter.reset_endpoint("custom_endpoint");
    circuit_breaker.reset("custom_endpoint");
    
    // Should allow 10 requests
    for _ in 0..10 {
        rate_limiter.check_rate_limit("custom_endpoint").unwrap();
    }
    
    // 11th request should fail
    let result = rate_limiter.check_rate_limit("custom_endpoint");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_provider_specific_limits() {
    // Create a fresh rate limiter with specific limits for testing
    let rate_limiter = RateLimiter::new();
    
    // Set provider specific limits
    rate_limiter.set_endpoint_limit("stripe", RateLimit {
        max_requests: 100,
        window: Duration::from_secs(1),
        auto_retry: false,
        max_retries: 0,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(1),
    });
    
    // Test Stripe limits (100 req/sec)
    let mut stripe_count = 0;
    for _ in 0..150 {
        if rate_limiter.check_rate_limit("stripe").is_ok() {
            stripe_count += 1;
        } else {
            break;
        }
    }
    assert!(stripe_count == 100, "Stripe should be limited to 100 requests per second, got {}", stripe_count);
    
    // Test PayPal limits (30 req/sec)
    rate_limiter.set_endpoint_limit("paypal", RateLimit {
        max_requests: 30,
        window: Duration::from_secs(1),
        auto_retry: false,
        max_retries: 0,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(1),
    });
    
    let mut paypal_count = 0;
    for _ in 0..50 {
        if rate_limiter.check_rate_limit("paypal").is_ok() {
            paypal_count += 1;
        } else {
            break;
        }
    }
    assert!(paypal_count == 30, "PayPal should be limited to 30 requests per second, got {}", paypal_count);
    
    // Test Square limits (50 req/sec)
    rate_limiter.set_endpoint_limit("square", RateLimit {
        max_requests: 50,
        window: Duration::from_secs(1),
        auto_retry: false,
        max_retries: 0,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_secs(1),
    });
    
    let mut square_count = 0;
    for _ in 0..70 {
        if rate_limiter.check_rate_limit("square").is_ok() {
            square_count += 1;
        } else {
            break;
        }
    }
    assert!(square_count == 50, "Square should be limited to 50 requests per second, got {}", square_count);
}

#[tokio::test]
async fn test_exponential_backoff() {
    let rate_limiter = RateLimiter::with_default_limit(RateLimit {
        max_requests: 10,
        window: Duration::from_secs(1),
        auto_retry: true,
        max_retries: 3,
        initial_backoff: Duration::from_millis(100),
        max_backoff: Duration::from_millis(400),
    });
    
    let attempt_times = Arc::new(std::sync::Mutex::new(Vec::new()));
    let attempt_times_clone = attempt_times.clone();
    
    // Function that always fails
    let result = rate_limiter.execute_with_retry_async("test_backoff", move || {
        let attempt_times = attempt_times_clone.clone();
        async move {
            attempt_times.lock().unwrap().push(Instant::now());
            Err::<String, _>(PayupError::ServerError(500))
        }
    }).await;
    
    assert!(result.is_err());
    
    let times = attempt_times.lock().unwrap();
    assert_eq!(times.len(), 4); // Initial attempt + 3 retries
    
    // Check that backoff is increasing
    if times.len() >= 2 {
        let first_backoff = times[1].duration_since(times[0]);
        assert!(first_backoff >= Duration::from_millis(90)); // Allow some tolerance
        assert!(first_backoff <= Duration::from_millis(110));
    }
    
    if times.len() >= 3 {
        let second_backoff = times[2].duration_since(times[1]);
        assert!(second_backoff >= Duration::from_millis(190)); // ~200ms
        assert!(second_backoff <= Duration::from_millis(210));
    }
    
    if times.len() >= 4 {
        let third_backoff = times[3].duration_since(times[2]);
        assert!(third_backoff >= Duration::from_millis(390)); // Capped at max_backoff (400ms)
        assert!(third_backoff <= Duration::from_millis(410));
    }
}