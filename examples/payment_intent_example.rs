use payup::stripe::{
    Auth, PaymentIntent, CreatePaymentIntentParams, UpdatePaymentIntentParams,
    ConfirmPaymentIntentParams, CapturePaymentIntentParams, CancelPaymentIntentParams,
    AutomaticPaymentMethods, CaptureMethod, ConfirmationMethod, SetupFutureUsage
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Stripe authentication
    let auth = Auth::new(
        "pk_test_example".to_string(),
        "sk_test_example".to_string()
    );

    // Example 1: Create a basic payment intent
    println!("Creating a new payment intent...");
    let create_params = CreatePaymentIntentParams {
        amount: 2000, // $20.00 in cents
        currency: "usd".to_string(),
        automatic_payment_methods: Some(AutomaticPaymentMethods {
            enabled: true,
            allow_redirects: None,
        }),
        capture_method: Some(CaptureMethod::Automatic),
        confirmation_method: Some(ConfirmationMethod::Automatic),
        customer: Some("cus_example123".to_string()),
        description: Some("Payment for premium subscription".to_string()),
        metadata: Some({
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("order_id".to_string(), "order_123".to_string());
            metadata.insert("product".to_string(), "premium_plan".to_string());
            metadata
        }),
        payment_method_types: Some(vec!["card".to_string(), "us_bank_account".to_string()]),
        receipt_email: Some("customer@example.com".to_string()),
        statement_descriptor: Some("MYCOMPANY SUB".to_string()),
        ..Default::default()
    };

    // Note: This would fail in a real environment without valid API keys
    // let payment_intent = PaymentIntent::create_async(&auth, create_params).await?;
    // println!("Created payment intent: {}", payment_intent.id);

    println!("Payment intent creation parameters configured successfully!");

    // Example 2: Update a payment intent
    println!("Configuring payment intent update...");
    let update_params = UpdatePaymentIntentParams {
        amount: Some(2500), // Update amount to $25.00
        description: Some("Updated payment for premium subscription + addon".to_string()),
        receipt_email: Some("updated-customer@example.com".to_string()),
        ..Default::default()
    };

    // let updated_payment_intent = PaymentIntent::update_async(&auth, "pi_example123", update_params).await?;
    // println!("Updated payment intent: {}", updated_payment_intent.id);

    println!("Payment intent update parameters configured successfully!");

    // Example 3: Confirm a payment intent
    println!("Configuring payment intent confirmation...");
    let confirm_params = ConfirmPaymentIntentParams {
        payment_method: Some("pm_card_visa".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        setup_future_usage: Some(SetupFutureUsage::OffSession),
        payment_method_options: None,
        shipping: None,
    };

    // let confirmed_payment_intent = PaymentIntent::confirm_async(&auth, "pi_example123", confirm_params).await?;
    // println!("Confirmed payment intent: {}", confirmed_payment_intent.id);

    println!("Payment intent confirmation parameters configured successfully!");

    // Example 4: Capture a payment intent (for manual capture)
    println!("Configuring payment intent capture...");
    let capture_params = CapturePaymentIntentParams {
        amount_to_capture: Some(2000), // Capture the full amount
        application_fee_amount: Some(200), // $2.00 application fee
        statement_descriptor: Some("CAPTURE".to_string()),
        statement_descriptor_suffix: Some("ORDER123".to_string()),
        transfer_data: None,
    };

    // let captured_payment_intent = PaymentIntent::capture_async(&auth, "pi_example123", capture_params).await?;
    // println!("Captured payment intent: {}", captured_payment_intent.id);

    println!("Payment intent capture parameters configured successfully!");

    // Example 5: Cancel a payment intent
    println!("Configuring payment intent cancellation...");
    let cancel_params = CancelPaymentIntentParams {
        cancellation_reason: Some("requested_by_customer".to_string()),
    };

    // let canceled_payment_intent = PaymentIntent::cancel_async(&auth, "pi_example123", cancel_params).await?;
    // println!("Canceled payment intent: {}", canceled_payment_intent.id);

    println!("Payment intent cancellation parameters configured successfully!");

    // Example 6: Retrieve a payment intent
    println!("Payment intent retrieval would work like:");
    println!("let payment_intent = PaymentIntent::retrieve_async(&auth, \"pi_example123\").await?;");

    // Example 7: List payment intents
    println!("Payment intent listing would work like:");
    println!("let payment_intents = PaymentIntent::list_async(&auth, Some(10)).await?;");

    println!("\nAll Payment Intent API examples configured successfully!");
    println!("Note: Actual API calls are commented out to avoid requiring valid API keys.");

    Ok(())
}