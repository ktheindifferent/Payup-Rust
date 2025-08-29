use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::stripe::auth::Auth;
use crate::http_client::get_shared_client;

/// Event data wrapper - contains the actual object that triggered the event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// The API resource object
    pub object: Value,
    /// Previous attributes of the object (for *.updated events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<Value>,
}

/// Request information for an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRequest {
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Idempotency key used for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

/// Events occur when the state of another API resource changes.
/// 
/// Stripe uses webhooks to notify your application when an event happens in your account.
/// Events occur when the state of another API resource changes. For example, a 
/// charge.succeeded event is created when a charge is successful, and an 
/// invoice.payment_failed event is created when an invoice payment attempt fails.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the object
    pub id: String,
    /// String representing the object's type (always "event")
    pub object: String,
    /// The API version used to render this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Time at which the object was created (Unix timestamp)
    pub created: i64,
    /// The event data containing the object that triggered the event
    pub data: EventData,
    /// Has the value true if the object exists in live mode
    pub livemode: bool,
    /// Number of webhooks that haven't been successfully delivered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_webhooks: Option<i64>,
    /// Information on the API request that instigated the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<EventRequest>,
    /// Description of the event type (e.g., invoice.created, charge.failed)
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Event {
    /// Retrieves the event with the given ID
    ///
    /// # Arguments
    ///
    /// * `auth` - Authentication credentials
    /// * `id` - The ID of the event to retrieve
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use payup::stripe::{Auth, Event};
    /// 
    /// let auth = Auth::new("sk_test_...".to_string(), "".to_string());
    /// let event = Event::get(&auth, "evt_1234567890").await?;
    /// println!("Event type: {}", event.type_field);
    /// ```
    pub async fn get(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/events/{}", id);
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let event = response.json::<Self>().await?;
        Ok(event)
    }

    /// Lists events, returning the most recent events first
    ///
    /// # Arguments
    ///
    /// * `auth` - Authentication credentials
    /// * `params` - Optional list parameters
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use payup::stripe::{Auth, Event};
    /// 
    /// let auth = Auth::new("sk_test_...".to_string(), "".to_string());
    /// let events = Event::list(&auth, None).await?;
    /// for event in events.data {
    ///     println!("Event: {} - {}", event.id, event.type_field);
    /// }
    /// ```
    pub async fn list(auth: &Auth, params: Option<ListEventsParams>) -> Result<EventList, crate::error::PayupError> {
        let mut url = "https://api.stripe.com/v1/events".to_string();
        
        if let Some(params) = params {
            let query = params.to_query_string();
            if !query.is_empty() {
                url.push_str("?");
                url.push_str(&query);
            }
        }
        
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let events = response.json::<EventList>().await?;
        Ok(events)
    }

    /// Extracts the typed object from the event data
    /// 
    /// This is a convenience method to deserialize the event's data.object
    /// into a specific Stripe object type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use payup::stripe::{Event, Charge};
    /// 
    /// let event = Event::get(&auth, "evt_charge_succeeded").await?;
    /// if event.type_field == "charge.succeeded" {
    ///     let charge: Charge = event.extract_object()?;
    ///     println!("Charge amount: {}", charge.amount.unwrap_or(0));
    /// }
    /// ```
    pub fn extract_object<T>(&self) -> Result<T, crate::error::PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_json::from_value(self.data.object.clone())
            .map_err(|e| crate::error::PayupError::SerializationError(e.to_string()))
    }

    /// Gets the previous attributes for update events
    ///
    /// For *.updated events, this returns the previous values of the
    /// attributes that changed.
    pub fn get_previous_attributes<T>(&self) -> Result<Option<T>, crate::error::PayupError>
    where
        T: for<'de> Deserialize<'de>,
    {
        match &self.data.previous_attributes {
            Some(attrs) => {
                let result = serde_json::from_value(attrs.clone())
                    .map_err(|e| crate::error::PayupError::SerializationError(e.to_string()))?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }
}

/// Parameters for listing events
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListEventsParams {
    /// Only return events created after this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<EventTimeFilter>,
    /// Filter events by whether all webhooks were successfully delivered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_success: Option<bool>,
    /// A cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Maximum number of events to return (1-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// A cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Filter by event type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    /// Filter by specific event types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl ListEventsParams {
    /// Creates a new ListEventsParams with default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Converts parameters to query string
    pub fn to_query_string(&self) -> String {
        let mut params = Vec::new();
        
        if let Some(ref created) = self.created {
            params.push(format!("created={}", created.to_query_string()));
        }
        
        if let Some(delivery_success) = self.delivery_success {
            params.push(format!("delivery_success={}", delivery_success));
        }
        
        if let Some(ref ending_before) = self.ending_before {
            params.push(format!("ending_before={}", ending_before));
        }
        
        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }
        
        if let Some(ref starting_after) = self.starting_after {
            params.push(format!("starting_after={}", starting_after));
        }
        
        if let Some(ref type_field) = self.type_field {
            params.push(format!("type={}", type_field));
        }
        
        if let Some(ref types) = self.types {
            for t in types {
                params.push(format!("types[]={}", t));
            }
        }
        
        params.join("&")
    }
}

/// Time filter for events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventTimeFilter {
    /// Exact timestamp
    Timestamp(i64),
    /// Range filter
    Range {
        #[serde(skip_serializing_if = "Option::is_none")]
        gt: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gte: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lt: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        lte: Option<i64>,
    },
}

impl EventTimeFilter {
    /// Converts the time filter to a query string parameter
    pub fn to_query_string(&self) -> String {
        match self {
            EventTimeFilter::Timestamp(ts) => ts.to_string(),
            EventTimeFilter::Range { gt, gte, lt, lte } => {
                let mut parts = Vec::new();
                if let Some(v) = gt {
                    parts.push(format!("gt:{}", v));
                }
                if let Some(v) = gte {
                    parts.push(format!("gte:{}", v));
                }
                if let Some(v) = lt {
                    parts.push(format!("lt:{}", v));
                }
                if let Some(v) = lte {
                    parts.push(format!("lte:{}", v));
                }
                parts.join(",")
            }
        }
    }
}

/// Response from listing events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventList {
    /// String representing the object's type (always "list")
    pub object: String,
    /// URL for this list
    pub url: String,
    /// Whether there are more events available
    pub has_more: bool,
    /// The list of events
    pub data: Vec<Event>,
}

/// Common event types in Stripe
pub mod event_types {
    // Account events
    pub const ACCOUNT_UPDATED: &str = "account.updated";
    pub const ACCOUNT_APPLICATION_AUTHORIZED: &str = "account.application.authorized";
    pub const ACCOUNT_APPLICATION_DEAUTHORIZED: &str = "account.application.deauthorized";
    pub const ACCOUNT_EXTERNAL_ACCOUNT_CREATED: &str = "account.external_account.created";
    pub const ACCOUNT_EXTERNAL_ACCOUNT_DELETED: &str = "account.external_account.deleted";
    pub const ACCOUNT_EXTERNAL_ACCOUNT_UPDATED: &str = "account.external_account.updated";

    // Charge events
    pub const CHARGE_CAPTURED: &str = "charge.captured";
    pub const CHARGE_EXPIRED: &str = "charge.expired";
    pub const CHARGE_FAILED: &str = "charge.failed";
    pub const CHARGE_PENDING: &str = "charge.pending";
    pub const CHARGE_REFUNDED: &str = "charge.refunded";
    pub const CHARGE_SUCCEEDED: &str = "charge.succeeded";
    pub const CHARGE_UPDATED: &str = "charge.updated";
    pub const CHARGE_DISPUTE_CLOSED: &str = "charge.dispute.closed";
    pub const CHARGE_DISPUTE_CREATED: &str = "charge.dispute.created";
    pub const CHARGE_DISPUTE_FUNDS_REINSTATED: &str = "charge.dispute.funds_reinstated";
    pub const CHARGE_DISPUTE_FUNDS_WITHDRAWN: &str = "charge.dispute.funds_withdrawn";
    pub const CHARGE_DISPUTE_UPDATED: &str = "charge.dispute.updated";
    pub const CHARGE_REFUND_UPDATED: &str = "charge.refund.updated";

    // Customer events
    pub const CUSTOMER_CREATED: &str = "customer.created";
    pub const CUSTOMER_DELETED: &str = "customer.deleted";
    pub const CUSTOMER_UPDATED: &str = "customer.updated";
    pub const CUSTOMER_DISCOUNT_CREATED: &str = "customer.discount.created";
    pub const CUSTOMER_DISCOUNT_DELETED: &str = "customer.discount.deleted";
    pub const CUSTOMER_DISCOUNT_UPDATED: &str = "customer.discount.updated";
    pub const CUSTOMER_SOURCE_CREATED: &str = "customer.source.created";
    pub const CUSTOMER_SOURCE_DELETED: &str = "customer.source.deleted";
    pub const CUSTOMER_SOURCE_EXPIRING: &str = "customer.source.expiring";
    pub const CUSTOMER_SOURCE_UPDATED: &str = "customer.source.updated";
    pub const CUSTOMER_SUBSCRIPTION_CREATED: &str = "customer.subscription.created";
    pub const CUSTOMER_SUBSCRIPTION_DELETED: &str = "customer.subscription.deleted";
    pub const CUSTOMER_SUBSCRIPTION_PAUSED: &str = "customer.subscription.paused";
    pub const CUSTOMER_SUBSCRIPTION_PENDING_UPDATE_APPLIED: &str = "customer.subscription.pending_update_applied";
    pub const CUSTOMER_SUBSCRIPTION_PENDING_UPDATE_EXPIRED: &str = "customer.subscription.pending_update_expired";
    pub const CUSTOMER_SUBSCRIPTION_RESUMED: &str = "customer.subscription.resumed";
    pub const CUSTOMER_SUBSCRIPTION_TRIAL_WILL_END: &str = "customer.subscription.trial_will_end";
    pub const CUSTOMER_SUBSCRIPTION_UPDATED: &str = "customer.subscription.updated";
    pub const CUSTOMER_TAX_ID_CREATED: &str = "customer.tax_id.created";
    pub const CUSTOMER_TAX_ID_DELETED: &str = "customer.tax_id.deleted";
    pub const CUSTOMER_TAX_ID_UPDATED: &str = "customer.tax_id.updated";

    // Invoice events
    pub const INVOICE_CREATED: &str = "invoice.created";
    pub const INVOICE_DELETED: &str = "invoice.deleted";
    pub const INVOICE_FINALIZATION_FAILED: &str = "invoice.finalization_failed";
    pub const INVOICE_FINALIZED: &str = "invoice.finalized";
    pub const INVOICE_MARKED_UNCOLLECTIBLE: &str = "invoice.marked_uncollectible";
    pub const INVOICE_PAID: &str = "invoice.paid";
    pub const INVOICE_PAYMENT_ACTION_REQUIRED: &str = "invoice.payment_action_required";
    pub const INVOICE_PAYMENT_FAILED: &str = "invoice.payment_failed";
    pub const INVOICE_PAYMENT_SUCCEEDED: &str = "invoice.payment_succeeded";
    pub const INVOICE_SENT: &str = "invoice.sent";
    pub const INVOICE_UPCOMING: &str = "invoice.upcoming";
    pub const INVOICE_UPDATED: &str = "invoice.updated";
    pub const INVOICE_VOIDED: &str = "invoice.voided";

    // Payment Intent events
    pub const PAYMENT_INTENT_AMOUNT_CAPTURABLE_UPDATED: &str = "payment_intent.amount_capturable_updated";
    pub const PAYMENT_INTENT_CANCELED: &str = "payment_intent.canceled";
    pub const PAYMENT_INTENT_CREATED: &str = "payment_intent.created";
    pub const PAYMENT_INTENT_PARTIALLY_FUNDED: &str = "payment_intent.partially_funded";
    pub const PAYMENT_INTENT_PAYMENT_FAILED: &str = "payment_intent.payment_failed";
    pub const PAYMENT_INTENT_PROCESSING: &str = "payment_intent.processing";
    pub const PAYMENT_INTENT_REQUIRES_ACTION: &str = "payment_intent.requires_action";
    pub const PAYMENT_INTENT_SUCCEEDED: &str = "payment_intent.succeeded";

    // Payment Method events
    pub const PAYMENT_METHOD_ATTACHED: &str = "payment_method.attached";
    pub const PAYMENT_METHOD_AUTOMATICALLY_UPDATED: &str = "payment_method.automatically_updated";
    pub const PAYMENT_METHOD_DETACHED: &str = "payment_method.detached";
    pub const PAYMENT_METHOD_UPDATED: &str = "payment_method.updated";

    // Payout events
    pub const PAYOUT_CANCELED: &str = "payout.canceled";
    pub const PAYOUT_CREATED: &str = "payout.created";
    pub const PAYOUT_FAILED: &str = "payout.failed";
    pub const PAYOUT_PAID: &str = "payout.paid";
    pub const PAYOUT_RECONCILIATION_COMPLETED: &str = "payout.reconciliation_completed";
    pub const PAYOUT_UPDATED: &str = "payout.updated";

    // Plan events
    pub const PLAN_CREATED: &str = "plan.created";
    pub const PLAN_DELETED: &str = "plan.deleted";
    pub const PLAN_UPDATED: &str = "plan.updated";

    // Price events
    pub const PRICE_CREATED: &str = "price.created";
    pub const PRICE_DELETED: &str = "price.deleted";
    pub const PRICE_UPDATED: &str = "price.updated";

    // Product events
    pub const PRODUCT_CREATED: &str = "product.created";
    pub const PRODUCT_DELETED: &str = "product.deleted";
    pub const PRODUCT_UPDATED: &str = "product.updated";

    // Subscription Schedule events
    pub const SUBSCRIPTION_SCHEDULE_ABORTED: &str = "subscription_schedule.aborted";
    pub const SUBSCRIPTION_SCHEDULE_CANCELED: &str = "subscription_schedule.canceled";
    pub const SUBSCRIPTION_SCHEDULE_COMPLETED: &str = "subscription_schedule.completed";
    pub const SUBSCRIPTION_SCHEDULE_CREATED: &str = "subscription_schedule.created";
    pub const SUBSCRIPTION_SCHEDULE_EXPIRING: &str = "subscription_schedule.expiring";
    pub const SUBSCRIPTION_SCHEDULE_RELEASED: &str = "subscription_schedule.released";
    pub const SUBSCRIPTION_SCHEDULE_UPDATED: &str = "subscription_schedule.updated";
}