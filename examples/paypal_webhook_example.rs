/// PayPal Webhook Verification and Handling Example
/// 
/// This example demonstrates how to properly verify and handle PayPal webhooks
/// with security best practices.
/// 
/// Security Best Practices:
/// 1. Always verify webhook signatures before processing
/// 2. Validate that cert URLs are from paypal.com domain
/// 3. Handle replay attacks by tracking processed webhook IDs
/// 4. Use HTTPS for your webhook endpoint
/// 5. Implement proper error handling and logging
/// 6. Store webhook IDs to prevent duplicate processing

use payup::paypal::webhooks::{PayPalWebhookHandler, WebhookEvent, WebhookHandler, event_types};
use payup::paypal::client::PayPalClient;
use payup::paypal::provider::PayPalProvider;
use payup::payment_provider::PaymentProvider;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Simple in-memory store to track processed webhooks (use database in production)
struct WebhookStore {
    processed_ids: Arc<Mutex<HashSet<String>>>,
}

impl WebhookStore {
    fn new() -> Self {
        Self {
            processed_ids: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    
    fn is_duplicate(&self, webhook_id: &str) -> bool {
        let ids = self.processed_ids.lock().unwrap();
        ids.contains(webhook_id)
    }
    
    fn mark_processed(&self, webhook_id: String) {
        let mut ids = self.processed_ids.lock().unwrap();
        ids.insert(webhook_id);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize PayPal provider
    let client_id = std::env::var("PAYPAL_CLIENT_ID")
        .unwrap_or_else(|_| "your_client_id".to_string());
    let client_secret = std::env::var("PAYPAL_CLIENT_SECRET")
        .unwrap_or_else(|_| "your_client_secret".to_string());
    let webhook_id = std::env::var("PAYPAL_WEBHOOK_ID")
        .unwrap_or_else(|_| "WH_YOUR_WEBHOOK_ID".to_string());
    
    println!("🔐 PayPal Webhook Verification Example");
    println!("=====================================\n");
    
    // Create PayPal provider and client
    let provider = PayPalProvider::new(client_id.clone(), client_secret.clone(), true);
    let mut client = PayPalClient::new(client_id, client_secret, true);
    
    // Initialize webhook handler and store
    let webhook_handler = PayPalWebhookHandler::new(webhook_id.clone());
    let webhook_store = WebhookStore::new();
    
    // Example 1: Basic Webhook Verification
    println!("Example 1: Basic Webhook Verification");
    println!("--------------------------------------");
    
    // Simulate webhook headers (in production, these come from HTTP request)
    let webhook_headers = vec![
        ("PayPal-Auth-Algo", "SHA256withRSA"),
        ("PayPal-Cert-Url", "https://api.sandbox.paypal.com/v1/notifications/certs/CERT-360caa42-fca2a594-7a8abba8"),
        ("PayPal-Transmission-Id", "4f8b4e90-1b8a-11ef-8c45-03f1d3c00cf0"),
        ("PayPal-Transmission-Sig", "example-signature-data"),
        ("PayPal-Transmission-Time", "2024-01-15T10:30:00Z"),
    ];
    
    let headers = PayPalWebhookHandler::headers_from_slice(&webhook_headers);
    
    // Validate headers first (security check)
    match PayPalWebhookHandler::validate_headers(&headers) {
        Ok(_) => println!("✅ Headers validated successfully"),
        Err(e) => {
            println!("❌ Header validation failed: {}", e);
            println!("   Rejecting webhook for security reasons");
            return Ok(());
        }
    }
    
    // Example webhook payload
    let webhook_payload = r#"{
        "id": "WH-7F159630MH811482K-4J716366NF957051V",
        "event_version": "1.0",
        "create_time": "2024-01-15T10:30:00.000Z",
        "resource_type": "capture",
        "event_type": "PAYMENT.CAPTURE.COMPLETED",
        "summary": "Payment completed for $127.50 USD",
        "resource": {
            "id": "3VW75242481980346",
            "amount": {
                "currency_code": "USD",
                "value": "127.50"
            },
            "status": "COMPLETED",
            "custom_id": "ORDER-12345",
            "invoice_id": "INV-12345"
        }
    }"#;
    
    // Check for duplicate processing (replay attack prevention)
    let event = WebhookEvent::parse(webhook_payload)?;
    if webhook_store.is_duplicate(&event.id) {
        println!("⚠️  Duplicate webhook detected (ID: {})", event.id);
        println!("   Skipping to prevent replay attack");
        return Ok(());
    }
    
    println!("\n📦 Webhook Event Details:");
    println!("   ID: {}", event.id);
    println!("   Type: {}", event.event_type);
    println!("   Summary: {}", event.summary.as_ref().unwrap_or(&"N/A".to_string()));
    
    // Example 2: Using Provider's verify_webhook Method
    println!("\nExample 2: Provider Webhook Verification");
    println!("-----------------------------------------");
    
    // For the provider's verify_webhook, we need to pass headers as JSON
    let headers_json = serde_json::to_string(&headers)?;
    
    // Verify using the provider (this would call PayPal's API in production)
    match provider.verify_webhook(
        webhook_payload.as_bytes(),
        &headers_json,
        &webhook_id
    ).await {
        Ok(is_valid) => {
            if is_valid {
                println!("✅ Webhook signature verified successfully");
                webhook_store.mark_processed(event.id.clone());
            } else {
                println!("❌ Webhook signature verification failed");
                return Ok(());
            }
        }
        Err(e) => {
            println!("❌ Verification error: {}", e);
            // In production, log this and return appropriate HTTP response
            return Ok(());
        }
    }
    
    // Example 3: Event Type Handling with WebhookHandler
    println!("\nExample 3: Event Type Handling");
    println!("-------------------------------");
    
    let mut event_handler = WebhookHandler::new();
    
    // Register handler for payment captures
    event_handler.on(event_types::PAYMENT_CAPTURE_COMPLETED, |event| {
        println!("💰 Payment Captured!");
        
        // Extract payment details
        #[derive(serde::Deserialize)]
        struct CaptureResource {
            id: String,
            amount: Amount,
            custom_id: Option<String>,
            invoice_id: Option<String>,
        }
        
        #[derive(serde::Deserialize)]
        struct Amount {
            currency_code: String,
            value: String,
        }
        
        if let Ok(resource) = event.get_resource::<CaptureResource>() {
            println!("   Capture ID: {}", resource.id);
            println!("   Amount: {} {}", resource.value, resource.currency_code);
            if let Some(custom_id) = resource.custom_id {
                println!("   Custom ID: {}", custom_id);
            }
            if let Some(invoice_id) = resource.invoice_id {
                println!("   Invoice ID: {}", invoice_id);
            }
            
            // TODO: Update your database, send confirmation email, etc.
        }
        
        Ok(())
    });
    
    // Register handler for subscriptions
    event_handler.on(event_types::BILLING_SUBSCRIPTION_ACTIVATED, |event| {
        println!("📅 Subscription Activated!");
        println!("   Event ID: {}", event.id);
        
        // TODO: Provision access, update user status, etc.
        
        Ok(())
    });
    
    // Register handler for refunds
    event_handler.on(event_types::PAYMENT_CAPTURE_REFUNDED, |event| {
        println!("💸 Payment Refunded!");
        
        if let Some(summary) = &event.summary {
            println!("   Summary: {}", summary);
        }
        
        // TODO: Update order status, reverse provisions, etc.
        
        Ok(())
    });
    
    // Process the webhook event
    match event_handler.handle(&event) {
        Ok(_) => println!("✅ Event processed successfully"),
        Err(e) => println!("❌ Event processing failed: {}", e),
    }
    
    // Example 4: Advanced Verification with API Call
    println!("\nExample 4: Advanced API Verification");
    println!("-------------------------------------");
    
    // This demonstrates using the webhook handler directly with the client
    // Note: This would make an actual API call to PayPal in production
    
    // Uncomment below when you have valid PayPal credentials:
    /*
    match webhook_handler.verify_with_api(&mut client, headers.clone(), webhook_payload).await {
        Ok(is_valid) => {
            if is_valid {
                println!("✅ API verification successful");
                
                // Parse and verify in one step
                match webhook_handler.parse_and_verify(&mut client, headers, webhook_payload).await {
                    Ok(verified_event) => {
                        println!("✅ Event parsed and verified: {}", verified_event.id);
                    }
                    Err(e) => {
                        println!("❌ Parse and verify failed: {}", e);
                    }
                }
            } else {
                println!("❌ API verification failed");
            }
        }
        Err(e) => {
            println!("❌ API call failed: {}", e);
        }
    }
    */
    
    // Security Best Practices Summary
    println!("\n🔒 Security Best Practices Applied:");
    println!("------------------------------------");
    println!("✓ Validated webhook headers before processing");
    println!("✓ Verified cert URL is from paypal.com domain");
    println!("✓ Checked for duplicate webhook IDs (replay attack prevention)");
    println!("✓ Verified webhook signature with PayPal API");
    println!("✓ Implemented proper error handling");
    println!("✓ Used type-safe resource extraction");
    
    println!("\n📝 Additional Security Recommendations:");
    println!("----------------------------------------");
    println!("• Use HTTPS for your webhook endpoint");
    println!("• Implement request timeout limits");
    println!("• Log all webhook activities for audit");
    println!("• Store processed webhook IDs in persistent storage");
    println!("• Implement retry logic for failed processing");
    println!("• Monitor for unusual webhook patterns");
    println!("• Set up alerts for verification failures");
    println!("• Regularly rotate webhook secrets");
    println!("• Implement rate limiting on webhook endpoint");
    
    Ok(())
}

// Example HTTP server endpoint (using a web framework like Actix or Rocket)
// This shows how you might handle webhooks in a real web application
#[allow(dead_code)]
async fn webhook_endpoint(
    headers: HashMap<String, String>,
    body: String,
    provider: &PayPalProvider,
    webhook_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Extract PayPal headers
    let paypal_headers = PayPalWebhookHandler::extract_headers(
        &headers.into_iter().collect::<Vec<_>>()
    );
    
    // 2. Validate headers
    PayPalWebhookHandler::validate_headers(&paypal_headers)?;
    
    // 3. Verify webhook signature
    let headers_json = serde_json::to_string(&paypal_headers)?;
    let is_valid = provider.verify_webhook(
        body.as_bytes(),
        &headers_json,
        webhook_id
    ).await?;
    
    if !is_valid {
        return Err("Invalid webhook signature".into());
    }
    
    // 4. Parse and process event
    let event = WebhookEvent::parse(&body)?;
    
    // 5. Handle based on event type
    match event.event_type.as_str() {
        event_types::PAYMENT_CAPTURE_COMPLETED => {
            // Handle payment capture
            println!("Processing payment capture: {}", event.id);
        }
        event_types::BILLING_SUBSCRIPTION_ACTIVATED => {
            // Handle subscription activation
            println!("Processing subscription activation: {}", event.id);
        }
        _ => {
            println!("Unhandled event type: {}", event.event_type);
        }
    }
    
    // 6. Return success response
    Ok("Webhook processed successfully".to_string())
}