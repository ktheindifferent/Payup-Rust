use payup::stripe::{StripeWebhookHandler, WebhookEventType};

fn main() {
    // Example of using the Stripe webhook handler
    
    // Your webhook endpoint secret from the Stripe Dashboard
    let webhook_secret = "whsec_test_secret_key_from_stripe_dashboard";
    
    // Create the webhook handler
    let webhook_handler = StripeWebhookHandler::new(webhook_secret.to_string());
    
    // Example webhook payload (this would come from Stripe in a real scenario)
    let payload = r#"{
        "id": "evt_1Ep24XJN5wTTaTpAB7XsZpn3",
        "object": "event",
        "api_version": "2020-08-27",
        "created": 1614556800,
        "data": {
            "object": {
                "id": "pi_1Ep24VJN5wTTaTpAkcCp5cDT",
                "object": "payment_intent",
                "amount": 2000,
                "currency": "usd",
                "status": "succeeded"
            }
        },
        "livemode": false,
        "pending_webhooks": 1,
        "request": {
            "id": "req_test_123",
            "idempotency_key": "idempotency_key_123"
        },
        "type": "payment_intent.succeeded"
    }"#;
    
    // Example signature header (this would come from the Stripe-Signature header)
    // In a real scenario, you'd get this from the HTTP request headers
    let signature_header = "t=1614556800 v1=5257a869e7ecb3f2e3c5a6d5e3b5a6d5e3b5a6d5e3b5a6d5e3b5a6d5e3b5a6d5";
    
    // Verify and construct the event
    match webhook_handler.construct_event(payload, signature_header) {
        Ok(event) => {
            println!("✅ Webhook verified successfully!");
            println!("Event ID: {}", event.id);
            println!("Event Type: {}", event.event_type);
            
            // Handle different event types
            match event.event_type_enum() {
                WebhookEventType::PaymentIntentSucceeded => {
                    println!("💰 Payment succeeded!");
                    if let Some(payment_id) = event.object_id() {
                        println!("Payment Intent ID: {}", payment_id);
                    }
                }
                WebhookEventType::PaymentIntentFailed => {
                    println!("❌ Payment failed!");
                }
                WebhookEventType::CustomerCreated => {
                    println!("👤 New customer created!");
                }
                WebhookEventType::ChargeRefunded => {
                    println!("💸 Charge refunded!");
                }
                WebhookEventType::SubscriptionScheduleCreated => {
                    println!("📅 Subscription created!");
                }
                _ => {
                    println!("Received event: {}", event.event_type);
                }
            }
            
            // Access event data
            println!("Event created at: {}", event.created_at());
            println!("Is live mode: {}", event.is_live());
        }
        Err(e) => {
            eprintln!("❌ Webhook verification failed: {}", e);
            // In production, you should return a 400 Bad Request response
        }
    }
}

// Example of how to use this in a web server (pseudo-code)
/*
async fn handle_stripe_webhook(
    body: String,
    headers: HeaderMap,
) -> Result<impl Response, Error> {
    let signature = headers
        .get("stripe-signature")
        .ok_or_else(|| Error::BadRequest("Missing signature header"))?
        .to_str()?;
    
    let webhook_handler = StripeWebhookHandler::new(env::var("STRIPE_WEBHOOK_SECRET")?);
    
    let event = webhook_handler.construct_event(&body, signature)?;
    
    // Process the event based on its type
    match event.event_type_enum() {
        WebhookEventType::PaymentIntentSucceeded => {
            // Update order status in database
            update_order_status(&event.object_id().unwrap(), "paid").await?;
        }
        WebhookEventType::CustomerSubscriptionDeleted => {
            // Cancel user's subscription
            cancel_subscription(&event.object_id().unwrap()).await?;
        }
        // ... handle other event types
        _ => {}
    }
    
    // Return 200 OK to acknowledge receipt
    Ok(Response::builder()
        .status(200)
        .body("Webhook received"))
}
*/