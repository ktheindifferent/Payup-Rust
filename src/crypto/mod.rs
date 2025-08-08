// Cryptocurrency payment support module

pub mod bitcoin;
pub mod ethereum;
pub mod wallet;
pub mod blockchain;
pub mod providers;
pub mod types;

use serde::{Deserialize, Serialize};
use crate::error::Result;
use std::collections::HashMap;

pub use types::*;
pub use wallet::WalletAddress;
pub use blockchain::Transaction;

// Supported cryptocurrencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Cryptocurrency {
    Bitcoin,
    Ethereum,
    BitcoinCash,
    Litecoin,
    Dogecoin,
    USDC,  // USD Coin (ERC-20)
    USDT,  // Tether (ERC-20)
    DAI,   // DAI Stablecoin (ERC-20)
    BNB,   // Binance Coin
    Polygon,
    Solana,
    Custom(String),
}

impl Cryptocurrency {
    pub fn symbol(&self) -> &str {
        match self {
            Cryptocurrency::Bitcoin => "BTC",
            Cryptocurrency::Ethereum => "ETH",
            Cryptocurrency::BitcoinCash => "BCH",
            Cryptocurrency::Litecoin => "LTC",
            Cryptocurrency::Dogecoin => "DOGE",
            Cryptocurrency::USDC => "USDC",
            Cryptocurrency::USDT => "USDT",
            Cryptocurrency::DAI => "DAI",
            Cryptocurrency::BNB => "BNB",
            Cryptocurrency::Polygon => "MATIC",
            Cryptocurrency::Solana => "SOL",
            Cryptocurrency::Custom(s) => s,
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Cryptocurrency::Bitcoin => 8,
            Cryptocurrency::Ethereum => 18,
            Cryptocurrency::BitcoinCash => 8,
            Cryptocurrency::Litecoin => 8,
            Cryptocurrency::Dogecoin => 8,
            Cryptocurrency::USDC => 6,
            Cryptocurrency::USDT => 6,
            Cryptocurrency::DAI => 18,
            Cryptocurrency::BNB => 18,
            Cryptocurrency::Polygon => 18,
            Cryptocurrency::Solana => 9,
            Cryptocurrency::Custom(_) => 18,
        }
    }

    pub fn is_stablecoin(&self) -> bool {
        matches!(self, 
            Cryptocurrency::USDC | 
            Cryptocurrency::USDT | 
            Cryptocurrency::DAI
        )
    }

    pub fn is_erc20(&self) -> bool {
        matches!(self,
            Cryptocurrency::USDC |
            Cryptocurrency::USDT |
            Cryptocurrency::DAI
        )
    }
}

// Network types for different blockchains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Network {
    // Bitcoin networks
    BitcoinMainnet,
    BitcoinTestnet,
    BitcoinRegtest,
    
    // Ethereum networks
    EthereumMainnet,
    EthereumGoerli,
    EthereumSepolia,
    
    // Layer 2 networks
    Polygon,
    Arbitrum,
    Optimism,
    
    // Other networks
    BinanceSmartChain,
    Solana,
    
    Custom(String),
}

impl Network {
    pub fn chain_id(&self) -> Option<u64> {
        match self {
            Network::EthereumMainnet => Some(1),
            Network::EthereumGoerli => Some(5),
            Network::EthereumSepolia => Some(11155111),
            Network::Polygon => Some(137),
            Network::Arbitrum => Some(42161),
            Network::Optimism => Some(10),
            Network::BinanceSmartChain => Some(56),
            _ => None,
        }
    }

    pub fn is_testnet(&self) -> bool {
        matches!(self,
            Network::BitcoinTestnet |
            Network::BitcoinRegtest |
            Network::EthereumGoerli |
            Network::EthereumSepolia
        )
    }
}

// Configuration for crypto payments
#[derive(Debug, Clone)]
pub struct CryptoConfig {
    pub provider: CryptoProvider,
    pub network: Network,
    pub api_key: Option<String>,
    pub webhook_secret: Option<String>,
    pub confirmations_required: u32,
    pub payment_timeout_minutes: u32,
    pub auto_convert_to_fiat: bool,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            provider: CryptoProvider::Native,
            network: Network::BitcoinMainnet,
            api_key: None,
            webhook_secret: None,
            confirmations_required: 1,
            payment_timeout_minutes: 15,
            auto_convert_to_fiat: false,
        }
    }
}

// Supported crypto payment providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoProvider {
    Native,              // Direct blockchain integration
    CoinbaseCommerce,    // Coinbase Commerce API
    BitPay,             // BitPay API
    CoinGate,           // CoinGate API
    NowPayments,        // NOWPayments API
    CoinPayments,       // CoinPayments.net API
    Custom(String),
}

// Crypto payment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPaymentRequest {
    pub amount: String,
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
    pub description: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub customer_email: Option<String>,
    pub redirect_url: Option<String>,
    pub cancel_url: Option<String>,
    pub webhook_url: Option<String>,
}

// Crypto payment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPayment {
    pub id: String,
    pub status: PaymentStatus,
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
    pub amount_crypto: String,
    pub amount_fiat: Option<String>,
    pub fiat_currency: Option<String>,
    pub exchange_rate: Option<f64>,
    pub wallet_address: String,
    pub payment_uri: Option<String>,
    pub qr_code: Option<String>,
    pub transaction_hash: Option<String>,
    pub confirmations: u32,
    pub confirmations_required: u32,
    pub created_at: i64,
    pub expires_at: i64,
    pub completed_at: Option<i64>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaymentStatus {
    Pending,
    AwaitingConfirmations,
    Processing,
    Completed,
    Failed,
    Expired,
    Cancelled,
    Refunded,
}

// Price data for crypto/fiat conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub cryptocurrency: Cryptocurrency,
    pub fiat_currency: String,
    pub price: f64,
    pub timestamp: i64,
    pub source: String,
}

// Blockchain fee estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
    pub priority: FeePriority,
    pub fee_amount: String,
    pub fee_rate: String, // satoshis per byte for BTC, gwei for ETH
    pub estimated_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeePriority {
    Low,
    Medium,
    High,
    Custom(String),
}

// Webhook event for crypto payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoWebhookEvent {
    pub event_type: WebhookEventType,
    pub payment_id: String,
    pub transaction_hash: Option<String>,
    pub confirmations: u32,
    pub amount: String,
    pub cryptocurrency: Cryptocurrency,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    PaymentCreated,
    PaymentPending,
    PaymentConfirmed,
    PaymentCompleted,
    PaymentFailed,
    PaymentExpired,
}

// Trait for crypto payment providers
#[async_trait::async_trait]
pub trait CryptoPaymentProvider: Send + Sync {
    // Provider information
    fn name(&self) -> &str;
    fn supported_cryptocurrencies(&self) -> Vec<Cryptocurrency>;
    fn supported_networks(&self) -> Vec<Network>;
    
    // Payment operations
    async fn create_payment(&self, request: &CryptoPaymentRequest) -> Result<CryptoPayment>;
    async fn get_payment(&self, payment_id: &str) -> Result<CryptoPayment>;
    async fn cancel_payment(&self, payment_id: &str) -> Result<bool>;
    async fn list_payments(&self, limit: Option<u32>) -> Result<Vec<CryptoPayment>>;
    
    // Price and fee operations
    async fn get_exchange_rate(&self, crypto: &Cryptocurrency, fiat: &str) -> Result<PriceData>;
    async fn estimate_fee(&self, crypto: &Cryptocurrency, network: &Network, priority: &FeePriority) -> Result<FeeEstimate>;
    
    // Wallet operations
    async fn validate_address(&self, address: &str, crypto: &Cryptocurrency, network: &Network) -> Result<bool>;
    async fn generate_address(&self, crypto: &Cryptocurrency, network: &Network) -> Result<WalletAddress>;
    
    // Transaction operations
    async fn get_transaction(&self, tx_hash: &str, network: &Network) -> Result<Transaction>;
    async fn get_confirmations(&self, tx_hash: &str, network: &Network) -> Result<u32>;
    
    // Webhook verification
    fn verify_webhook(&self, payload: &[u8], signature: &str) -> Result<bool>;
}

// Helper functions for crypto payments
pub mod utils {
    use super::*;
    
    /// Convert amount from smallest unit to decimal representation
    /// e.g., satoshis to BTC, wei to ETH
    pub fn from_smallest_unit(amount: u128, decimals: u8) -> f64 {
        amount as f64 / 10_f64.powi(decimals as i32)
    }
    
    /// Convert amount from decimal to smallest unit representation
    /// e.g., BTC to satoshis, ETH to wei
    pub fn to_smallest_unit(amount: f64, decimals: u8) -> u128 {
        (amount * 10_f64.powi(decimals as i32)) as u128
    }
    
    /// Generate a payment URI for QR codes
    pub fn generate_payment_uri(
        cryptocurrency: &Cryptocurrency,
        address: &str,
        amount: Option<&str>,
        label: Option<&str>,
    ) -> String {
        let scheme = match cryptocurrency {
            Cryptocurrency::Bitcoin => "bitcoin",
            Cryptocurrency::Ethereum => "ethereum",
            Cryptocurrency::Litecoin => "litecoin",
            Cryptocurrency::Dogecoin => "dogecoin",
            _ => return address.to_string(),
        };
        
        let mut uri = format!("{}:{}", scheme, address);
        let mut params = Vec::new();
        
        if let Some(amt) = amount {
            params.push(format!("amount={}", amt));
        }
        
        if let Some(lbl) = label {
            params.push(format!("label={}", urlencoding::encode(lbl)));
        }
        
        if !params.is_empty() {
            uri.push('?');
            uri.push_str(&params.join("&"));
        }
        
        uri
    }
    
    /// Validate if a transaction has enough confirmations
    pub fn is_confirmed(confirmations: u32, required: u32) -> bool {
        confirmations >= required
    }
    
    /// Calculate payment expiration status
    pub fn is_expired(expires_at: i64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        now > expires_at
    }
}