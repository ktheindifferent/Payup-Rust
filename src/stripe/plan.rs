use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::stripe::auth::Auth;
use crate::http_client::{get_shared_client, get_shared_blocking_client};

/// Represents a subscription plan
/// 
/// Plans define the base price, currency, and billing cycle for recurring purchases 
/// of products. Products help you track inventory or provisioning, and plans help you track pricing.
/// Note: The Plan API is deprecated - use the Prices API instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Unique identifier for the object
    pub id: String,
    /// String representing the object's type (always "plan")
    pub object: String,
    /// Whether the plan is currently available for new subscriptions
    pub active: bool,
    /// Specifies a usage aggregation strategy for plans of usage_type=metered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<AggregateUsage>,
    /// The unit amount in cents to be charged, represented as a whole integer if possible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// The unit amount in cents to be charged, represented as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_decimal: Option<String>,
    /// Describes how to compute the price per period
    pub billing_scheme: BillingScheme,
    /// Time at which the object was created (Unix timestamp)
    pub created: i64,
    /// Three-letter ISO currency code
    pub currency: String,
    /// The frequency at which a subscription is billed
    pub interval: Interval,
    /// The number of intervals between subscription billings
    pub interval_count: u32,
    /// Has the value true if the object exists in live mode
    pub livemode: bool,
    /// Set of key-value pairs that you can attach to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// A brief description of the plan, hidden from customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The product whose pricing this plan determines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Each element represents a pricing tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PlanTier>>,
    /// Defines if the tiering price should be graduated or volume based
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,
    /// Apply a transformation to the reported usage or set quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<TransformUsage>,
    /// Default number of trial days when subscribing a customer to this plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined
    pub usage_type: UsageType,
}

/// Represents a price object
/// 
/// Prices define the unit cost, currency, and (optional) billing cycle for both 
/// recurring and one-time purchases of products.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    /// Unique identifier for the object
    pub id: String,
    /// String representing the object's type (always "price")
    pub object: String,
    /// Whether the price can be used for new purchases
    pub active: bool,
    /// Describes how to compute the price per period
    pub billing_scheme: BillingScheme,
    /// Time at which the object was created (Unix timestamp)
    pub created: i64,
    /// Three-letter ISO currency code
    pub currency: String,
    /// A lookup key used to retrieve prices dynamically from a static string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    /// Has the value true if the object exists in live mode
    pub livemode: bool,
    /// Set of key-value pairs that you can attach to an object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// A brief description of the price, hidden from customers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// The ID of the product this price is associated with
    pub product: String,
    /// The recurring components of a price such as interval and interval_count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    /// Each element represents a pricing tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,
    /// Defines if the tiering price should be graduated or volume based
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,
    /// Apply a transformation to the reported usage or set quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,
    /// One of one_time or recurring depending on whether the price is for a one-time purchase or a recurring subscription
    #[serde(rename = "type")]
    pub type_field: PriceType,
    /// The unit amount in cents to be charged, represented as a whole integer if possible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// The unit amount in cents to be charged, represented as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// Billing scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BillingScheme {
    PerUnit,
    Tiered,
}

/// Billing interval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Interval {
    Day,
    Week,
    Month,
    Year,
}

/// Usage aggregation method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregateUsage {
    Sum,
    LastDuringPeriod,
    LastEver,
    Max,
}

/// Tiers mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TiersMode {
    Graduated,
    Volume,
}

/// Usage type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageType {
    Licensed,
    Metered,
}

/// Price type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    OneTime,
    Recurring,
}

/// Tax behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

/// Transform usage settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformUsage {
    /// Divide usage by this number
    pub divide_by: i64,
    /// After division, either round up or down
    pub round: RoundingMode,
}

/// Transform quantity settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformQuantity {
    /// Divide usage by this number
    pub divide_by: i64,
    /// After division, either round up or down
    pub round: RoundingMode,
}

/// Rounding mode
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundingMode {
    Up,
    Down,
}

/// Recurring price settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurring {
    /// Specifies a usage aggregation strategy for prices of usage_type=metered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<AggregateUsage>,
    /// The frequency at which a subscription is billed
    pub interval: Interval,
    /// The number of intervals between subscription billings
    pub interval_count: u32,
    /// Default number of trial days when subscribing a customer to this price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    /// Configures how the quantity per period should be determined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
}

/// Plan pricing tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanTier {
    /// Price for the entire tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as flat_amount, but as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
    /// Per unit price for units relevant to the tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as unit_amount, but as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
    /// Up to and including to this quantity will be contained in the tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to: Option<i64>,
}

/// Price pricing tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTier {
    /// Price for the entire tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,
    /// Same as flat_amount, but as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
    /// Per unit price for units relevant to the tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as unit_amount, but as a decimal string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
    /// Up to and including to this quantity will be contained in the tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to: Option<i64>,
}

impl Plan {
    /// Creates a new plan
    pub async fn create(auth: &Auth, params: CreatePlanParams) -> Result<Self, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/plans";
        let response = get_shared_client()
            .post(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let plan = response.json::<Self>().await?;
        Ok(plan)
    }

    /// Retrieves a plan by ID
    pub async fn get(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let plan = response.json::<Self>().await?;
        Ok(plan)
    }

    /// Updates a plan
    pub async fn update(auth: &Auth, id: &str, params: UpdatePlanParams) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let plan = response.json::<Self>().await?;
        Ok(plan)
    }

    /// Deletes a plan
    pub async fn delete(auth: &Auth, id: &str) -> Result<DeletedPlan, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/plans/{}", id);
        let response = get_shared_client()
            .delete(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let result = response.json::<DeletedPlan>().await?;
        Ok(result)
    }

    /// Lists all plans
    pub async fn list(auth: &Auth) -> Result<PlanList, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/plans";
        let response = get_shared_client()
            .get(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let plans = response.json::<PlanList>().await?;
        Ok(plans)
    }
}

impl Price {
    /// Creates a new price
    pub async fn create(auth: &Auth, params: CreatePriceParams) -> Result<Self, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/prices";
        let response = get_shared_client()
            .post(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let price = response.json::<Self>().await?;
        Ok(price)
    }

    /// Retrieves a price by ID
    pub async fn get(auth: &Auth, id: &str) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/prices/{}", id);
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let price = response.json::<Self>().await?;
        Ok(price)
    }

    /// Updates a price
    pub async fn update(auth: &Auth, id: &str, params: UpdatePriceParams) -> Result<Self, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/prices/{}", id);
        let response = get_shared_client()
            .post(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let price = response.json::<Self>().await?;
        Ok(price)
    }

    /// Lists all prices
    pub async fn list(auth: &Auth) -> Result<PriceList, crate::error::PayupError> {
        let url = "https://api.stripe.com/v1/prices";
        let response = get_shared_client()
            .get(url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let prices = response.json::<PriceList>().await?;
        Ok(prices)
    }

    /// Searches for prices
    pub async fn search(auth: &Auth, query: &str) -> Result<PriceSearchResult, crate::error::PayupError> {
        let url = format!("https://api.stripe.com/v1/prices/search?query={}", urlencoding::encode(query));
        let response = get_shared_client()
            .get(&url)
            .basic_auth(&auth.client, Some(&auth.secret))
            .send()
            .await?;
        
        let result = response.json::<PriceSearchResult>().await?;
        Ok(result)
    }
}

/// Response from listing plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanList {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<Plan>,
}

/// Response from listing prices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<Price>,
}

/// Response from searching prices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSearchResult {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<Price>,
    pub next_page: Option<String>,
}

/// Response from deleting a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedPlan {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

/// Parameters for creating a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanParams {
    pub currency: String,
    pub interval: Interval,
    pub product: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<AggregateUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_decimal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<BillingScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PlanTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<TransformUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
}

/// Parameters for updating a plan
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePlanParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

/// Parameters for creating a price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePriceParams {
    pub currency: String,
    pub product: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<BillingScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// Parameters for updating a price
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdatePriceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
}