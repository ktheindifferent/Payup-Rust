use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::stripe::auth::Auth;
use crate::http_client::{get_shared_client, get_shared_blocking_client};

/// Represents an invoice issued to a customer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Unique identifier for the object
    pub id: String,
    /// String representing the object's type (always "invoice")
    pub object: String,
    /// The country of the business associated with this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_country: Option<String>,
    /// The public name of the business associated with this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// The account tax IDs associated with the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    /// Final amount due at this time for this invoice
    pub amount_due: i64,
    /// The amount that was paid
    pub amount_paid: i64,
    /// The amount remaining that is due
    pub amount_remaining: i64,
    /// The fee in cents that will be applied to the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Number of payment attempts made for this invoice
    pub attempt_count: i64,
    /// Whether an attempt has been made to pay the invoice
    pub attempted: bool,
    /// Controls whether Stripe will perform automatic collection of the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    /// Settings for automatic tax calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,
    /// Indicates the reason why the invoice was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<BillingReason>,
    /// ID of the latest charge generated for this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    /// Either charge_automatically or send_invoice
    pub collection_method: CollectionMethod,
    /// Time at which the object was created (Unix timestamp)
    pub created: i64,
    /// Three-letter ISO currency code
    pub currency: String,
    /// Custom fields displayed on the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    /// The ID of the customer who will be billed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// The customer's address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<Address>,
    /// The customer's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    /// The customer's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// The customer's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,
    /// The customer's shipping information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_shipping: Option<Shipping>,
    /// The customer's tax exempt status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_exempt: Option<TaxExempt>,
    /// The customer's tax IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<TaxId>>,
    /// ID of the default payment method for the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    /// ID of the default payment source for the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,
    /// The tax rates applied to this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,
    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Describes the current discount applied to this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    /// The discounts applied to the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountItem>>,
    /// The date on which payment for this invoice is due
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    /// Ending customer balance after the invoice is finalized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<i64>,
    /// Footer displayed on the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// The URL for the hosted invoice page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,
    /// The link to download the PDF for the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,
    /// The individual line items that make up the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<InvoiceLineItemList>,
    /// Has the value true if the object exists in live mode
    pub livemode: bool,
    /// Set of key-value pairs that you can attach to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// The time at which payment will next be attempted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt: Option<i64>,
    /// A unique, identifying string that appears on emails sent to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// The account (if any) for which the funds of the invoice payment are intended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    /// Whether payment was successfully collected for this invoice
    pub paid: bool,
    /// The PaymentIntent associated with this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    /// End of the usage period during which invoice items were added to this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<i64>,
    /// Start of the usage period during which invoice items were added to this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<i64>,
    /// Total amount of all post-payment credit notes issued for this invoice
    pub post_payment_credit_notes_amount: i64,
    /// Total amount of all pre-payment credit notes issued for this invoice
    pub pre_payment_credit_notes_amount: i64,
    /// The quote this invoice was generated from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// This is the transaction number that appears on email receipts sent for this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    /// Starting customer balance before the invoice is finalized
    pub starting_balance: i64,
    /// Extra information about an invoice for the customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// The status of the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,
    /// The subscription that this invoice was prepared for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    /// The integer amount in cents representing the subtotal of the invoice before any invoice level discounts or taxes are applied
    pub subtotal: i64,
    /// The integer amount in cents representing the total amount of the invoice including all discounts but excluding all tax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    /// The amount of tax on this invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amounts: Option<Vec<TaxAmount>>,
    /// The integer amount in cents representing the total amount of the invoice including all discounts and taxes
    pub total: i64,
    /// The integer amount in cents representing the total amount of the invoice including all discounts but excluding all tax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks_delivered_at: Option<i64>,
}

/// Invoice status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

/// Billing reason for the invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    Quote,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
}

/// Collection method for the invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

/// Tax exempt status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxExempt {
    Exempt,
    None,
    Reverse,
}

/// Automatic tax settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomaticTax {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Custom field for invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
}

/// Address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Shipping information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// Tax ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxId {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

/// Tax rate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub id: String,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub display_name: String,
    pub inclusive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    pub percentage: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
}

/// Discount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Coupon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}

/// Discount item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

/// Coupon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub duration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,
    pub valid: bool,
}

/// Tax amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxAmount {
    pub amount: i64,
    pub inclusive: bool,
    pub tax_rate: String,
}

/// Invoice line item list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItemList {
    pub object: String,
    pub data: Vec<InvoiceLineItem>,
    pub has_more: bool,
    pub url: String,
}

/// Invoice line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub object: String,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_excluding_tax: Option<i64>,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amounts: Option<Vec<DiscountAmount>>,
    pub discountable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    pub period: Period,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    pub proration: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_details: Option<ProrationDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_excluding_tax: Option<String>,
}

/// Discount amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountAmount {
    pub amount: i64,
    pub discount: String,
}

/// Period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub end: i64,
    pub start: i64,
}

/// Proration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProrationDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credited_items: Option<CreditedItems>,
}

/// Credited items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditedItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    pub invoice_line_items: Vec<String>,
}

impl Invoice {
    /// Retrieves an invoice by ID
    pub async fn get(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}", id);
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Lists all invoices
    pub async fn list(auth: &Auth) -> Result<InvoiceList, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/invoices";
        let response = get_shared_client()
            .get(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoices = response.json::<InvoiceList>().await?;
        Ok(invoices)
    }

    /// Creates a new invoice
    pub async fn create(auth: &Auth, params: CreateInvoiceParams) -> Result<Self, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/invoices";
        let response = get_shared_client()
            .post(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Updates an invoice
    pub async fn update(auth: &Auth, id: &str, params: UpdateInvoiceParams) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Finalizes an invoice
    pub async fn finalize(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}/finalize", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Pays an invoice
    pub async fn pay(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}/pay", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Sends an invoice for manual payment
    pub async fn send(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}/send", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Voids an invoice
    pub async fn void(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}/void", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Marks an invoice as uncollectible
    pub async fn mark_uncollectible(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}/mark_uncollectible", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let invoice = response.json::<Self>().await?;
        Ok(invoice)
    }

    /// Deletes a draft invoice
    pub async fn delete(auth: &Auth, id: &str) -> Result<DeletedInvoice, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/invoices/{}", id);
        let response = get_shared_client()
            .delete(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let result = response.json::<DeletedInvoice>().await?;
        Ok(result)
    }
}

/// Response from listing invoices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceList {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<Invoice>,
}

/// Response from deleting an invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedInvoice {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

/// Parameters for creating an invoice
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInvoiceParams {
    pub customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
}

/// Parameters for updating an invoice
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInvoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
}

/// Transfer data for invoices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferData {
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}