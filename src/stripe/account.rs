use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::http_client::{get_shared_client, get_shared_blocking_client};
use super::Auth;

/// Represents a Stripe Account (Connect account)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub business_profile: Option<BusinessProfile>,
    pub business_type: Option<String>,
    pub capabilities: Option<Capabilities>,
    pub charges_enabled: bool,
    pub country: String,
    pub created: i64,
    pub default_currency: Option<String>,
    pub details_submitted: bool,
    pub email: Option<String>,
    pub external_accounts: Option<ExternalAccountList>,
    pub future_requirements: Option<Requirements>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub payouts_enabled: bool,
    pub requirements: Option<Requirements>,
    pub settings: Option<AccountSettings>,
    pub tos_acceptance: Option<TosAcceptance>,
    pub r#type: String,
}

/// Business profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfile {
    pub mcc: Option<String>,
    pub name: Option<String>,
    pub product_description: Option<String>,
    pub support_address: Option<Address>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub support_url: Option<String>,
    pub url: Option<String>,
}

/// Account capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub card_payments: Option<String>,
    pub transfers: Option<String>,
    pub legacy_payments: Option<String>,
    pub platform_payments: Option<String>,
}

/// External accounts list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAccountList {
    pub object: String,
    pub data: Vec<ExternalAccount>,
    pub has_more: bool,
    pub url: String,
}

/// External account (bank account or card)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExternalAccount {
    BankAccount(BankAccount),
    Card(ConnectCard),
}

/// Bank account for Connect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub object: String,
    pub account_holder_name: Option<String>,
    pub account_holder_type: Option<String>,
    pub account_type: Option<String>,
    pub bank_name: Option<String>,
    pub country: String,
    pub currency: String,
    pub fingerprint: Option<String>,
    pub last4: String,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub routing_number: Option<String>,
    pub status: String,
}

/// Card for Connect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectCard {
    pub id: String,
    pub object: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub brand: String,
    pub country: Option<String>,
    pub cvc_check: Option<String>,
    pub dynamic_last4: Option<String>,
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub last4: String,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub name: Option<String>,
}

/// Account requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub current_deadline: Option<i64>,
    pub currently_due: Vec<String>,
    pub disabled_reason: Option<String>,
    pub errors: Vec<RequirementError>,
    pub eventually_due: Vec<String>,
    pub past_due: Vec<String>,
    pub pending_verification: Vec<String>,
}

/// Requirement error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementError {
    pub code: String,
    pub reason: String,
    pub requirement: String,
}

/// Account settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub branding: Option<BrandingSettings>,
    pub card_issuing: Option<CardIssuingSettings>,
    pub card_payments: Option<CardPaymentSettings>,
    pub dashboard: Option<DashboardSettings>,
    pub payments: Option<PaymentSettings>,
    pub payouts: Option<PayoutSettings>,
}

/// Branding settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
}

/// Card issuing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardIssuingSettings {
    pub tos_acceptance: Option<TosAcceptance>,
}

/// Card payment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPaymentSettings {
    pub decline_on: Option<DeclineChargeOn>,
    pub statement_descriptor_prefix: Option<String>,
    pub statement_descriptor_prefix_kana: Option<String>,
    pub statement_descriptor_prefix_kanji: Option<String>,
}

/// Dashboard settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSettings {
    pub display_name: Option<String>,
    pub timezone: Option<String>,
}

/// Payment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentSettings {
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_kana: Option<String>,
    pub statement_descriptor_kanji: Option<String>,
}

/// Payout settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutSettings {
    pub debit_negative_balances: bool,
    pub schedule: PayoutSchedule,
    pub statement_descriptor: Option<String>,
}

/// Payout schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutSchedule {
    pub delay_days: u32,
    pub interval: String,
    pub monthly_anchor: Option<u32>,
    pub weekly_anchor: Option<String>,
}

/// Decline charge settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclineChargeOn {
    pub avs_failure: bool,
    pub cvc_failure: bool,
}

/// Terms of Service acceptance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TosAcceptance {
    pub date: Option<i64>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

/// Address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
}

/// Parameters for creating an account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilitiesParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfileParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<IndividualParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptanceParams>,
}

/// Account capabilities parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCapabilitiesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilityParams>,
}

/// Capability parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParams {
    pub requested: bool,
}

/// Business profile parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessProfileParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Company parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
}

/// Individual parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
}

/// Date of birth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateOfBirth {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

/// TOS acceptance parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TosAcceptanceParams {
    pub date: i64,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl Account {
    /// Create a new Connect account
    /// 
    /// # Example
    /// ```ignore
    /// use payup::stripe::{Auth, Account, CreateAccountParams};
    /// 
    /// let auth = Auth::new("test_key".to_string(), "test_secret".to_string());
    /// let params = CreateAccountParams {
    ///     type: Some("express".to_string()),
    ///     country: Some("US".to_string()),
    ///     email: Some("user@example.com".to_string()),
    ///     ..Default::default()
    /// };
    /// let account = Account::create(&auth, params)?;
    /// ```
    pub fn create(auth: &Auth, params: CreateAccountParams) -> Result<Self> {
        let client = get_shared_blocking_client();
        let response = client
            .post("https://api.stripe.com/v1/accounts")
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()?;
        
        let account: Account = response.json()?;
        Ok(account)
    }

    /// Create a new Connect account (async)
    pub async fn create_async(auth: &Auth, params: CreateAccountParams) -> Result<Self> {
        let client = get_shared_client();
        let response = client
            .post("https://api.stripe.com/v1/accounts")
            .header("Authorization", format!("Bearer {}", auth.secret))
            .form(&params)
            .send()
            .await?;
        
        let account: Account = response.json().await?;
        Ok(account)
    }

    /// Retrieve an account by ID
    pub fn retrieve(auth: &Auth, account_id: &str) -> Result<Self> {
        let client = get_shared_blocking_client();
        let response = client
            .get(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let account: Account = response.json()?;
        Ok(account)
    }

    /// Retrieve an account by ID (async)
    pub async fn retrieve_async(auth: &Auth, account_id: &str) -> Result<Self> {
        let client = get_shared_client();
        let response = client
            .get(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()
            .await?;
        
        let account: Account = response.json().await?;
        Ok(account)
    }

    /// Update an account
    pub fn update(auth: &Auth, account_id: &str, params: serde_json::Value) -> Result<Self> {
        let client = get_shared_blocking_client();
        let response = client
            .post(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .json(&params)
            .send()?;
        
        let account: Account = response.json()?;
        Ok(account)
    }

    /// Update an account (async)
    pub async fn update_async(auth: &Auth, account_id: &str, params: serde_json::Value) -> Result<Self> {
        let client = get_shared_client();
        let response = client
            .post(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .json(&params)
            .send()
            .await?;
        
        let account: Account = response.json().await?;
        Ok(account)
    }

    /// Delete an account
    pub fn delete(auth: &Auth, account_id: &str) -> Result<DeletedAccount> {
        let client = get_shared_blocking_client();
        let response = client
            .delete(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        let deleted: DeletedAccount = response.json()?;
        Ok(deleted)
    }

    /// Delete an account (async)
    pub async fn delete_async(auth: &Auth, account_id: &str) -> Result<DeletedAccount> {
        let client = get_shared_client();
        let response = client
            .delete(&format!("https://api.stripe.com/v1/accounts/{}", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()
            .await?;
        
        let deleted: DeletedAccount = response.json().await?;
        Ok(deleted)
    }

    /// List all accounts
    pub fn list(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let client = get_shared_blocking_client();
        let mut url = "https://api.stripe.com/v1/accounts".to_string();
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()?;
        
        #[derive(Deserialize)]
        struct AccountList {
            data: Vec<Account>,
        }
        
        let list: AccountList = response.json()?;
        Ok(list.data)
    }

    /// List all accounts (async)
    pub async fn list_async(auth: &Auth, limit: Option<u32>) -> Result<Vec<Self>> {
        let client = get_shared_client();
        let mut url = "https://api.stripe.com/v1/accounts".to_string();
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth.secret))
            .send()
            .await?;
        
        #[derive(Deserialize)]
        struct AccountList {
            data: Vec<Account>,
        }
        
        let list: AccountList = response.json().await?;
        Ok(list.data)
    }

    /// Reject an account
    pub fn reject(auth: &Auth, account_id: &str, reason: &str) -> Result<Self> {
        let client = get_shared_blocking_client();
        let params = serde_json::json!({
            "reason": reason
        });
        
        let response = client
            .post(&format!("https://api.stripe.com/v1/accounts/{}/reject", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .json(&params)
            .send()?;
        
        let account: Account = response.json()?;
        Ok(account)
    }

    /// Reject an account (async)
    pub async fn reject_async(auth: &Auth, account_id: &str, reason: &str) -> Result<Self> {
        let client = get_shared_client();
        let params = serde_json::json!({
            "reason": reason
        });
        
        let response = client
            .post(&format!("https://api.stripe.com/v1/accounts/{}/reject", account_id))
            .header("Authorization", format!("Bearer {}", auth.secret))
            .json(&params)
            .send()
            .await?;
        
        let account: Account = response.json().await?;
        Ok(account)
    }
}

/// Represents a deleted account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedAccount {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

impl Default for CreateAccountParams {
    fn default() -> Self {
        Self {
            r#type: None,
            country: None,
            email: None,
            capabilities: None,
            business_type: None,
            business_profile: None,
            company: None,
            individual: None,
            metadata: None,
            tos_acceptance: None,
        }
    }
}