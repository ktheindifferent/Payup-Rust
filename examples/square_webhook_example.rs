use payup::square::{
    SquareWebhookHandler, WebhookEvent, WebhookEventType, WebhookEventHandler,
};
use payup::error::Result;

/// Example of setting up and using Square webhook handlers
fn main() -> Result<()> {
    // Example 1: Basic webhook signature verification
    basic_webhook_verification()?;
    
    // Example 2: Event-specific handlers
    event_specific_handlers()?;
    
    // Example 3: Full webhook server integration example
    webhook_server_example();
    
    // Example 4: Processing specific event types
    process_payment_events()?;
    
    Ok(())
}

/// Example 1: Basic webhook signature verification
fn basic_webhook_verification() -> Result<()> {
    println!("=== Basic Webhook Verification ===\n");
    
    // Initialize webhook handler with your signature key
    let signature_key = std::env::var("SQUARE_WEBHOOK_SIGNATURE_KEY")
        .unwrap_or_else(|_| "your_webhook_signature_key".to_string());
    
    let handler = SquareWebhookHandler::new(signature_key);
    
    // Sample webhook payload (would come from HTTP request body)
    let payload = r#"{
        "merchant_id": "MERCHANT_123",
        "location_id": "LOC_456",
        "entity_id": "PAYMENT_789",
        "type": "payment.created",
        "event_id": "evt_abc123",
        "created_at": "2024-01-15T10:00:00Z",
        "data": {
            "type": "payment",
            "id": "PAYMENT_789",
            "object": {
                "id": "PAYMENT_789",
                "amount_money": {
                    "amount": 2500,
                    "currency": "USD"
                },
                "status": "COMPLETED",
                "source_type": "CARD"
            }
        }
    }"#;
    
    // Signature from Square (would come from X-Square-Signature header)
    let signature = "sample_signature_base64_encoded";
    
    // Request URL (your webhook endpoint)
    let request_url = "https://yourdomain.com/webhooks/square";
    
    // Verify and construct the event
    match handler.construct_event(payload, signature, request_url) {
        Ok(event) => {
            println!("✓ Webhook verified successfully!");
            println!("  Event Type: {}", event.event_type);
            println!("  Event ID: {}", event.event_id);
            println!("  Merchant ID: {}", event.get_merchant_id());
            if let Some(location_id) = event.get_location_id() {
                println!("  Location ID: {}", location_id);
            }
            if let Some(entity_id) = event.get_entity_id() {
                println!("  Entity ID: {}", entity_id);
            }
        }
        Err(e) => {
            println!("✗ Webhook verification failed: {}", e);
        }
    }
    
    println!();
    Ok(())
}

/// Example 2: Setting up event-specific handlers
fn event_specific_handlers() -> Result<()> {
    println!("=== Event-Specific Handlers ===\n");
    
    let mut event_handler = WebhookEventHandler::new();
    
    // Handle payment created events
    event_handler.on("payment.created", |event| {
        println!("🎉 Payment Created!");
        println!("  Event ID: {}", event.event_id);
        println!("  Merchant ID: {}", event.get_merchant_id());
        
        // Extract payment details
        if let Ok(payment) = event.get_resource::<serde_json::Value>() {
            if let Some(amount) = payment.get("amount_money") {
                println!("  Amount: {} {}", 
                    amount.get("amount").and_then(|v| v.as_i64()).unwrap_or(0),
                    amount.get("currency").and_then(|v| v.as_str()).unwrap_or(""));
            }
            if let Some(status) = payment.get("status").and_then(|v| v.as_str()) {
                println!("  Status: {}", status);
            }
        }
        Ok(())
    });
    
    // Handle refund created events
    event_handler.on("refund.created", |event| {
        println!("💸 Refund Created!");
        println!("  Event ID: {}", event.event_id);
        
        if let Ok(refund) = event.get_resource::<serde_json::Value>() {
            if let Some(amount) = refund.get("amount_money") {
                println!("  Refund Amount: {} {}",
                    amount.get("amount").and_then(|v| v.as_i64()).unwrap_or(0),
                    amount.get("currency").and_then(|v| v.as_str()).unwrap_or(""));
            }
            if let Some(reason) = refund.get("reason").and_then(|v| v.as_str()) {
                println!("  Reason: {}", reason);
            }
        }
        Ok(())
    });
    
    // Handle customer events
    event_handler.on("customer.created", |event| {
        println!("👤 New Customer Created!");
        println!("  Event ID: {}", event.event_id);
        
        if let Ok(customer) = event.get_resource::<serde_json::Value>() {
            if let Some(email) = customer.get("email_address").and_then(|v| v.as_str()) {
                println!("  Email: {}", email);
            }
            if let Some(name) = customer.get("given_name").and_then(|v| v.as_str()) {
                println!("  Name: {}", name);
            }
        }
        Ok(())
    });
    
    // Handle subscription events
    event_handler.on("subscription.created", |event| {
        println!("📅 Subscription Created!");
        println!("  Event ID: {}", event.event_id);
        
        if let Ok(subscription) = event.get_resource::<serde_json::Value>() {
            if let Some(plan_id) = subscription.get("plan_id").and_then(|v| v.as_str()) {
                println!("  Plan ID: {}", plan_id);
            }
            if let Some(status) = subscription.get("status").and_then(|v| v.as_str()) {
                println!("  Status: {}", status);
            }
        }
        Ok(())
    });
    
    // Set a default handler for unregistered events
    event_handler.default(|event| {
        println!("📬 Unhandled Event: {}", event.event_type);
        println!("  Event ID: {}", event.event_id);
        Ok(())
    });
    
    // Simulate handling various events
    let test_events = vec![
        create_test_event("payment.created", "PAYMENT_001"),
        create_test_event("refund.created", "REFUND_001"),
        create_test_event("customer.created", "CUSTOMER_001"),
        create_test_event("subscription.created", "SUB_001"),
        create_test_event("unknown.event", "UNKNOWN_001"),
    ];
    
    for event in test_events {
        event_handler.handle(&event)?;
        println!();
    }
    
    Ok(())
}

/// Example 3: Full webhook server integration
fn webhook_server_example() {
    println!("=== Webhook Server Integration Example ===\n");
    
    println!("Example webhook endpoint handler (for use with web framework):\n");
    
    println!(r#"
use axum::{{
    extract::{{Json, HeaderMap}},
    http::StatusCode,
    response::IntoResponse,
}};

async fn handle_square_webhook(
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {{
    // Get the signature from headers
    let signature = match headers.get("x-square-signature") {{
        Some(sig) => sig.to_str().unwrap_or(""),
        None => return (StatusCode::BAD_REQUEST, "Missing signature"),
    }};
    
    // Get the request URL
    let request_url = "https://yourdomain.com/webhooks/square";
    
    // Initialize handler with your signature key
    let handler = SquareWebhookHandler::new(
        std::env::var("SQUARE_WEBHOOK_SIGNATURE_KEY").unwrap()
    );
    
    // Verify and process the webhook
    match handler.construct_event(&body, signature, request_url) {{
        Ok(event) => {{
            // Process the event based on type
            match event.event_type_enum() {{
                WebhookEventType::PaymentCreated => {{
                    handle_payment_created(event).await;
                }}
                WebhookEventType::RefundCreated => {{
                    handle_refund_created(event).await;
                }}
                WebhookEventType::CustomerCreated => {{
                    handle_customer_created(event).await;
                }}
                _ => {{
                    // Log unhandled events
                    println!("Unhandled event type: {{}}", event.event_type);
                }}
            }}
            
            (StatusCode::OK, "Event processed")
        }}
        Err(e) => {{
            eprintln!("Webhook verification failed: {{}}", e);
            (StatusCode::BAD_REQUEST, "Invalid webhook")
        }}
    }}
}}
"#);
    
    println!();
}

/// Example 4: Processing specific payment events
fn process_payment_events() -> Result<()> {
    println!("=== Processing Payment Events ===\n");
    
    // Create a payment event
    let payment_event = WebhookEvent {
        merchant_id: "MERCHANT_ABC".to_string(),
        location_id: Some("LOC_123".to_string()),
        entity_id: Some("PAYMENT_XYZ".to_string()),
        event_type: "payment.created".to_string(),
        event_id: "evt_payment_001".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        data: payup::square::WebhookEventData {
            data_type: "payment".to_string(),
            id: "PAYMENT_XYZ".to_string(),
            object: serde_json::json!({
                "id": "PAYMENT_XYZ",
                "amount_money": {
                    "amount": 5000,
                    "currency": "USD"
                },
                "status": "COMPLETED",
                "source_type": "CARD",
                "card_details": {
                    "status": "CAPTURED",
                    "card": {
                        "card_brand": "VISA",
                        "last_4": "1234",
                        "exp_month": 12,
                        "exp_year": 2025
                    }
                },
                "customer_id": "CUSTOMER_ABC",
                "reference_id": "ORDER_123",
                "note": "Payment for order #123",
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:30Z"
            }),
        },
    };
    
    // Process the payment event
    match payment_event.event_type_enum() {
        WebhookEventType::PaymentCreated => {
            println!("Processing Payment Created Event:");
            println!("  Event ID: {}", payment_event.event_id);
            println!("  Merchant ID: {}", payment_event.get_merchant_id());
            
            // Extract detailed payment information
            let payment: serde_json::Value = payment_event.get_resource()?;
            
            // Amount information
            if let Some(amount_money) = payment.get("amount_money") {
                let amount = amount_money.get("amount")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                let currency = amount_money.get("currency")
                    .and_then(|v| v.as_str())
                    .unwrap_or("USD");
                
                println!("  Amount: {:.2} {}", amount as f64 / 100.0, currency);
            }
            
            // Payment status
            if let Some(status) = payment.get("status").and_then(|v| v.as_str()) {
                println!("  Status: {}", status);
            }
            
            // Card details
            if let Some(card_details) = payment.get("card_details") {
                if let Some(card) = card_details.get("card") {
                    let brand = card.get("card_brand")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let last4 = card.get("last_4")
                        .and_then(|v| v.as_str())
                        .unwrap_or("****");
                    
                    println!("  Card: {} ending in {}", brand, last4);
                }
            }
            
            // Customer and order information
            if let Some(customer_id) = payment.get("customer_id").and_then(|v| v.as_str()) {
                println!("  Customer ID: {}", customer_id);
            }
            
            if let Some(reference_id) = payment.get("reference_id").and_then(|v| v.as_str()) {
                println!("  Reference ID: {}", reference_id);
            }
            
            if let Some(note) = payment.get("note").and_then(|v| v.as_str()) {
                println!("  Note: {}", note);
            }
            
            println!("\n✓ Payment processed successfully!");
        }
        _ => {
            println!("Not a payment event");
        }
    }
    
    Ok(())
}

/// Helper function to create test events
fn create_test_event(event_type: &str, entity_id: &str) -> WebhookEvent {
    WebhookEvent {
        merchant_id: "MERCHANT_TEST".to_string(),
        location_id: Some("LOC_TEST".to_string()),
        entity_id: Some(entity_id.to_string()),
        event_type: event_type.to_string(),
        event_id: format!("evt_{}", entity_id.to_lowercase()),
        created_at: chrono::Utc::now().to_rfc3339(),
        data: payup::square::WebhookEventData {
            data_type: event_type.split('.').next().unwrap_or("unknown").to_string(),
            id: entity_id.to_string(),
            object: match event_type {
                "payment.created" => serde_json::json!({
                    "amount_money": {"amount": 2500, "currency": "USD"},
                    "status": "COMPLETED"
                }),
                "refund.created" => serde_json::json!({
                    "amount_money": {"amount": 500, "currency": "USD"},
                    "reason": "Customer request"
                }),
                "customer.created" => serde_json::json!({
                    "email_address": "customer@example.com",
                    "given_name": "John"
                }),
                "subscription.created" => serde_json::json!({
                    "plan_id": "PLAN_123",
                    "status": "ACTIVE"
                }),
                _ => serde_json::json!({}),
            },
        },
    }
}