use payup::error::PayupError;
use payup::stripe::{Auth, Charge, Customer};
use payup::stripe_original::{
    Charge as OriginalCharge, Customer as OriginalCustomer, Dispute, FileLink, Invoice,
    Subscription,
};

/// Test that operations on Charge without ID return proper errors instead of panicking
#[test]
fn test_charge_capture_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = Charge::new();
    charge.id = None; // Explicitly set to None

    // This should return an error, not panic
    let result = charge.capture(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for capture"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_charge_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = Charge::new();
    charge.id = None;

    let result = charge.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_customer_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut customer = Customer::new();
    customer.id = None;

    // Run the async version synchronously in test
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(customer.async_update(auth));
    
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Customer ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

// Tests for original stripe module
#[test]
fn test_original_charge_capture_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = OriginalCharge::new();
    charge.id = None;

    let result = charge.capture(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for capture"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_original_charge_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = OriginalCharge::new();
    charge.id = None;

    let result = charge.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_original_customer_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut customer = OriginalCustomer::new();
    customer.id = None;

    let result = customer.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Customer ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_dispute_close_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut dispute = Dispute::new();
    dispute.id = None;

    let result = dispute.close(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Dispute ID is required for close"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_dispute_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut dispute = Dispute::new();
    dispute.id = None;

    let result = dispute.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Dispute ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_file_link_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut file_link = FileLink::new();
    file_link.id = None;

    let result = file_link.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("FileLink ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_invoice_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut invoice = Invoice::new();
    invoice.id = None;

    let result = invoice.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Invoice ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_subscription_update_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut subscription = Subscription::new();
    subscription.id = None;

    let result = subscription.update(auth);
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Subscription ID is required for update"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

// Async tests
#[tokio::test]
async fn test_async_charge_capture_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = Charge::new();
    charge.id = None;

    let result = charge.async_capture(auth).await;
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for capture"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[tokio::test]
async fn test_async_original_charge_capture_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut charge = OriginalCharge::new();
    charge.id = None;

    let result = charge.async_capture(auth).await;
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Charge ID is required for capture"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}

#[tokio::test]
async fn test_async_dispute_close_without_id_returns_error() {
    let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    let mut dispute = Dispute::new();
    dispute.id = None;

    let result = dispute.async_close(auth).await;
    assert!(result.is_err());
    match result.expect_err("Should return error for invalid operation") {
        PayupError::ValidationError(msg) => {
            assert!(msg.contains("Dispute ID is required for close"));
        }
        other => assert!(false, "Expected ValidationError, got {:?}", other),
    }
}