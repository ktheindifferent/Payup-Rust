use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::{
    CryptoPaymentProvider, CryptoPaymentRequest, CryptoPayment,
    Cryptocurrency, Network, PaymentStatus, PriceData, FeeEstimate,
    FeePriority, wallet::WalletAddress, blockchain::Transaction,
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Coinbase Commerce payment provider
#[allow(dead_code)]
pub struct CoinbaseCommerceProvider {
    api_key: String,
    webhook_secret: Option<String>,
    sandbox: bool,
}

impl CoinbaseCommerceProvider {
    pub fn new(api_key: String) -> Self {
        CoinbaseCommerceProvider {
            api_key,
            webhook_secret: None,
            sandbox: false,
        }
    }

    pub fn with_webhook_secret(mut self, secret: String) -> Self {
        self.webhook_secret = Some(secret);
        self
    }

    pub fn sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }

    fn base_url(&self) -> &str {
        if self.sandbox {
            "https://api-sandbox.commerce.coinbase.com"
        } else {
            "https://api.commerce.coinbase.com"
        }
    }

    async fn create_charge(&self, request: &ChargeRequest) -> Result<ChargeResponse> {
        // Mock implementation
        Ok(ChargeResponse {
            id: format!("charge_{}", uuid::Uuid::new_v4()),
            code: format!("CODE{}", rand::random::<u32>() % 10000),
            name: request.name.clone(),
            description: request.description.clone(),
            logo_url: None,
            hosted_url: format!("{}/charges/example", self.base_url()),
            created_at: chrono::Utc::now().to_rfc3339(),
            expires_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339(),
            confirmed_at: None,
            pricing: request.pricing_type.clone(),
            addresses: HashMap::new(),
            timeline: Vec::new(),
        })
    }

    async fn get_charge(&self, charge_id: &str) -> Result<ChargeResponse> {
        // Mock implementation
        Ok(ChargeResponse {
            id: charge_id.to_string(),
            code: "CODE1234".to_string(),
            name: "Test Charge".to_string(),
            description: Some("Test charge description".to_string()),
            logo_url: None,
            hosted_url: format!("{}/charges/{}", self.base_url(), charge_id),
            created_at: chrono::Utc::now().to_rfc3339(),
            expires_at: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::minutes(15))
                .unwrap()
                .to_rfc3339(),
            confirmed_at: None,
            pricing: PricingType::FixedPrice,
            addresses: HashMap::new(),
            timeline: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChargeRequest {
    name: String,
    description: Option<String>,
    pricing_type: PricingType,
    local_price: Option<LocalPrice>,
    metadata: Option<HashMap<String, String>>,
    redirect_url: Option<String>,
    cancel_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChargeResponse {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    logo_url: Option<String>,
    hosted_url: String,
    created_at: String,
    expires_at: String,
    confirmed_at: Option<String>,
    pricing: PricingType,
    addresses: HashMap<String, String>,
    timeline: Vec<TimelineEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PricingType {
    #[serde(rename = "fixed_price")]
    FixedPrice,
    #[serde(rename = "no_price")]
    NoPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocalPrice {
    amount: String,
    currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TimelineEntry {
    time: String,
    status: String,
    context: Option<String>,
}

#[async_trait]
impl CryptoPaymentProvider for CoinbaseCommerceProvider {
    fn name(&self) -> &str {
        "Coinbase Commerce"
    }

    fn supported_cryptocurrencies(&self) -> Vec<Cryptocurrency> {
        vec![
            Cryptocurrency::Bitcoin,
            Cryptocurrency::Ethereum,
            Cryptocurrency::Litecoin,
            Cryptocurrency::BitcoinCash,
            Cryptocurrency::USDC,
            Cryptocurrency::DAI,
        ]
    }

    fn supported_networks(&self) -> Vec<Network> {
        vec![
            Network::BitcoinMainnet,
            Network::EthereumMainnet,
        ]
    }

    async fn create_payment(&self, request: &CryptoPaymentRequest) -> Result<CryptoPayment> {
        let charge_request = ChargeRequest {
            name: request.description.clone().unwrap_or_else(|| "Payment".to_string()),
            description: request.description.clone(),
            pricing_type: PricingType::FixedPrice,
            local_price: Some(LocalPrice {
                amount: request.amount.clone(),
                currency: "USD".to_string(),
            }),
            metadata: request.metadata.clone(),
            redirect_url: request.redirect_url.clone(),
            cancel_url: request.cancel_url.clone(),
        };

        let charge = self.create_charge(&charge_request).await?;

        Ok(CryptoPayment {
            id: charge.id,
            status: PaymentStatus::Pending,
            cryptocurrency: request.cryptocurrency.clone(),
            network: request.network.clone(),
            amount_crypto: request.amount.clone(),
            amount_fiat: Some(request.amount.clone()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: None,
            wallet_address: "".to_string(),
            payment_uri: Some(charge.hosted_url),
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 1,
            created_at: chrono::Utc::now().timestamp(),
            expires_at: chrono::Utc::now().timestamp() + 900,
            completed_at: None,
            metadata: request.metadata.clone(),
        })
    }

    async fn get_payment(&self, payment_id: &str) -> Result<CryptoPayment> {
        let charge = self.get_charge(payment_id).await?;

        Ok(CryptoPayment {
            id: charge.id,
            status: PaymentStatus::Pending,
            cryptocurrency: Cryptocurrency::Bitcoin,
            network: Network::BitcoinMainnet,
            amount_crypto: "0.001".to_string(),
            amount_fiat: Some("50.00".to_string()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: Some(50000.0),
            wallet_address: "".to_string(),
            payment_uri: Some(charge.hosted_url),
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 1,
            created_at: chrono::Utc::now().timestamp() - 300,
            expires_at: chrono::Utc::now().timestamp() + 600,
            completed_at: None,
            metadata: None,
        })
    }

    async fn cancel_payment(&self, _payment_id: &str) -> Result<bool> {
        Ok(true)
    }

    async fn list_payments(&self, _limit: Option<u32>) -> Result<Vec<CryptoPayment>> {
        Ok(Vec::new())
    }

    async fn get_exchange_rate(&self, crypto: &Cryptocurrency, fiat: &str) -> Result<PriceData> {
        Ok(PriceData {
            cryptocurrency: crypto.clone(),
            fiat_currency: fiat.to_string(),
            price: 50000.0,
            timestamp: chrono::Utc::now().timestamp(),
            source: "coinbase".to_string(),
        })
    }

    async fn estimate_fee(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
        priority: &FeePriority,
    ) -> Result<FeeEstimate> {
        Ok(FeeEstimate {
            cryptocurrency: crypto.clone(),
            network: network.clone(),
            priority: priority.clone(),
            fee_amount: "0.0001".to_string(),
            fee_rate: "10 sat/vB".to_string(),
            estimated_time_minutes: 30,
        })
    }

    async fn validate_address(
        &self,
        address: &str,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<bool> {
        let wallet_address = WalletAddress::new(
            address.to_string(),
            crypto.clone(),
            network.clone(),
        );
        wallet_address.validate()
    }

    async fn generate_address(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<WalletAddress> {
        super::wallet::generate_wallet_address(crypto, network)
    }

    async fn get_transaction(&self, tx_hash: &str, network: &Network) -> Result<Transaction> {
        let client = super::blockchain::BlockchainClient::new(
            Cryptocurrency::Bitcoin,
            network.clone(),
        );
        client.get_transaction(tx_hash).await
    }

    async fn get_confirmations(&self, _tx_hash: &str, _network: &Network) -> Result<u32> {
        Ok(6)
    }

    fn verify_webhook(&self, _payload: &[u8], _signature: &str) -> Result<bool> {
        if let Some(_secret) = &self.webhook_secret {
            // Mock HMAC verification
            Ok(true)
        } else {
            Err(PayupError::ValidationError("Webhook secret not configured".to_string()))
        }
    }
}

/// BitPay payment provider
#[allow(dead_code)]
pub struct BitPayProvider {
    api_token: String,
    test_mode: bool,
}

impl BitPayProvider {
    pub fn new(api_token: String) -> Self {
        BitPayProvider {
            api_token,
            test_mode: false,
        }
    }

    pub fn test_mode(mut self) -> Self {
        self.test_mode = true;
        self
    }

    fn base_url(&self) -> &str {
        if self.test_mode {
            "https://test.bitpay.com"
        } else {
            "https://bitpay.com"
        }
    }

    async fn create_invoice(&self, request: &InvoiceRequest) -> Result<InvoiceResponse> {
        // Mock implementation
        Ok(InvoiceResponse {
            id: format!("invoice_{}", uuid::Uuid::new_v4()),
            url: format!("{}/invoice?id=example", self.base_url()),
            status: "new".to_string(),
            price: request.price,
            currency: request.currency.clone(),
            invoice_time: chrono::Utc::now().timestamp_millis(),
            expiration_time: chrono::Utc::now().timestamp_millis() + 900000,
            current_time: chrono::Utc::now().timestamp_millis(),
            exception_status: None,
            payment_codes: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvoiceRequest {
    price: f64,
    currency: String,
    order_id: Option<String>,
    notification_url: Option<String>,
    redirect_url: Option<String>,
    buyer: Option<BuyerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuyerInfo {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvoiceResponse {
    id: String,
    url: String,
    status: String,
    price: f64,
    currency: String,
    invoice_time: i64,
    expiration_time: i64,
    current_time: i64,
    exception_status: Option<String>,
    payment_codes: HashMap<String, PaymentCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PaymentCode {
    address: String,
    amount: String,
}

#[async_trait]
impl CryptoPaymentProvider for BitPayProvider {
    fn name(&self) -> &str {
        "BitPay"
    }

    fn supported_cryptocurrencies(&self) -> Vec<Cryptocurrency> {
        vec![
            Cryptocurrency::Bitcoin,
            Cryptocurrency::BitcoinCash,
            Cryptocurrency::Ethereum,
            Cryptocurrency::Litecoin,
            Cryptocurrency::Dogecoin,
        ]
    }

    fn supported_networks(&self) -> Vec<Network> {
        vec![
            Network::BitcoinMainnet,
            Network::EthereumMainnet,
        ]
    }

    async fn create_payment(&self, request: &CryptoPaymentRequest) -> Result<CryptoPayment> {
        let invoice_request = InvoiceRequest {
            price: request.amount.parse().unwrap_or(0.0),
            currency: "USD".to_string(),
            order_id: request.metadata.as_ref()
                .and_then(|m| m.get("order_id"))
                .cloned(),
            notification_url: request.webhook_url.clone(),
            redirect_url: request.redirect_url.clone(),
            buyer: request.customer_email.as_ref().map(|email| BuyerInfo {
                name: None,
                email: Some(email.clone()),
            }),
        };

        let invoice = self.create_invoice(&invoice_request).await?;
        
        let crypto_symbol = request.cryptocurrency.symbol();
        let wallet_address = invoice.payment_codes
            .get(crypto_symbol)
            .map(|pc| pc.address.clone())
            .unwrap_or_default();

        Ok(CryptoPayment {
            id: invoice.id,
            status: PaymentStatus::Pending,
            cryptocurrency: request.cryptocurrency.clone(),
            network: request.network.clone(),
            amount_crypto: request.amount.clone(),
            amount_fiat: Some(invoice.price.to_string()),
            fiat_currency: Some(invoice.currency),
            exchange_rate: None,
            wallet_address,
            payment_uri: Some(invoice.url),
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 1,
            created_at: invoice.invoice_time / 1000,
            expires_at: invoice.expiration_time / 1000,
            completed_at: None,
            metadata: request.metadata.clone(),
        })
    }

    async fn get_payment(&self, payment_id: &str) -> Result<CryptoPayment> {
        // Mock implementation
        Ok(CryptoPayment {
            id: payment_id.to_string(),
            status: PaymentStatus::Pending,
            cryptocurrency: Cryptocurrency::Bitcoin,
            network: Network::BitcoinMainnet,
            amount_crypto: "0.001".to_string(),
            amount_fiat: Some("50.00".to_string()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: Some(50000.0),
            wallet_address: "".to_string(),
            payment_uri: None,
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 1,
            created_at: chrono::Utc::now().timestamp() - 300,
            expires_at: chrono::Utc::now().timestamp() + 600,
            completed_at: None,
            metadata: None,
        })
    }

    async fn cancel_payment(&self, _payment_id: &str) -> Result<bool> {
        Ok(true)
    }

    async fn list_payments(&self, _limit: Option<u32>) -> Result<Vec<CryptoPayment>> {
        Ok(Vec::new())
    }

    async fn get_exchange_rate(&self, crypto: &Cryptocurrency, fiat: &str) -> Result<PriceData> {
        Ok(PriceData {
            cryptocurrency: crypto.clone(),
            fiat_currency: fiat.to_string(),
            price: 50000.0,
            timestamp: chrono::Utc::now().timestamp(),
            source: "bitpay".to_string(),
        })
    }

    async fn estimate_fee(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
        priority: &FeePriority,
    ) -> Result<FeeEstimate> {
        Ok(FeeEstimate {
            cryptocurrency: crypto.clone(),
            network: network.clone(),
            priority: priority.clone(),
            fee_amount: "0.0001".to_string(),
            fee_rate: "10 sat/vB".to_string(),
            estimated_time_minutes: 30,
        })
    }

    async fn validate_address(
        &self,
        address: &str,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<bool> {
        let wallet_address = WalletAddress::new(
            address.to_string(),
            crypto.clone(),
            network.clone(),
        );
        wallet_address.validate()
    }

    async fn generate_address(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<WalletAddress> {
        super::wallet::generate_wallet_address(crypto, network)
    }

    async fn get_transaction(&self, tx_hash: &str, network: &Network) -> Result<Transaction> {
        let client = super::blockchain::BlockchainClient::new(
            Cryptocurrency::Bitcoin,
            network.clone(),
        );
        client.get_transaction(tx_hash).await
    }

    async fn get_confirmations(&self, _tx_hash: &str, _network: &Network) -> Result<u32> {
        Ok(6)
    }

    fn verify_webhook(&self, _payload: &[u8], _signature: &str) -> Result<bool> {
        // Mock implementation
        Ok(true)
    }
}

/// CoinGate payment provider
#[allow(dead_code)]
pub struct CoinGateProvider {
    api_key: String,
    sandbox: bool,
}

impl CoinGateProvider {
    pub fn new(api_key: String) -> Self {
        CoinGateProvider {
            api_key,
            sandbox: false,
        }
    }

    pub fn sandbox(mut self) -> Self {
        self.sandbox = true;
        self
    }

    fn base_url(&self) -> &str {
        if self.sandbox {
            "https://api-sandbox.coingate.com/v2"
        } else {
            "https://api.coingate.com/v2"
        }
    }
}

#[async_trait]
impl CryptoPaymentProvider for CoinGateProvider {
    fn name(&self) -> &str {
        "CoinGate"
    }

    fn supported_cryptocurrencies(&self) -> Vec<Cryptocurrency> {
        vec![
            Cryptocurrency::Bitcoin,
            Cryptocurrency::Ethereum,
            Cryptocurrency::Litecoin,
            Cryptocurrency::BitcoinCash,
            Cryptocurrency::Dogecoin,
            Cryptocurrency::USDT,
            Cryptocurrency::USDC,
            Cryptocurrency::DAI,
        ]
    }

    fn supported_networks(&self) -> Vec<Network> {
        vec![
            Network::BitcoinMainnet,
            Network::EthereumMainnet,
            Network::Polygon,
            Network::BinanceSmartChain,
        ]
    }

    async fn create_payment(&self, request: &CryptoPaymentRequest) -> Result<CryptoPayment> {
        // Mock implementation
        let payment_id = format!("coingate_{}", uuid::Uuid::new_v4());
        
        Ok(CryptoPayment {
            id: payment_id.clone(),
            status: PaymentStatus::Pending,
            cryptocurrency: request.cryptocurrency.clone(),
            network: request.network.clone(),
            amount_crypto: request.amount.clone(),
            amount_fiat: Some(request.amount.clone()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: Some(50000.0),
            wallet_address: "generated_address".to_string(),
            payment_uri: Some(format!("{}/orders/{}", self.base_url(), payment_id)),
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 2,
            created_at: chrono::Utc::now().timestamp(),
            expires_at: chrono::Utc::now().timestamp() + 1800, // 30 minutes
            completed_at: None,
            metadata: request.metadata.clone(),
        })
    }

    async fn get_payment(&self, payment_id: &str) -> Result<CryptoPayment> {
        // Mock implementation
        Ok(CryptoPayment {
            id: payment_id.to_string(),
            status: PaymentStatus::Pending,
            cryptocurrency: Cryptocurrency::Bitcoin,
            network: Network::BitcoinMainnet,
            amount_crypto: "0.001".to_string(),
            amount_fiat: Some("50.00".to_string()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: Some(50000.0),
            wallet_address: "".to_string(),
            payment_uri: None,
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 2,
            created_at: chrono::Utc::now().timestamp() - 300,
            expires_at: chrono::Utc::now().timestamp() + 1500,
            completed_at: None,
            metadata: None,
        })
    }

    async fn cancel_payment(&self, _payment_id: &str) -> Result<bool> {
        Ok(true)
    }

    async fn list_payments(&self, _limit: Option<u32>) -> Result<Vec<CryptoPayment>> {
        Ok(Vec::new())
    }

    async fn get_exchange_rate(&self, crypto: &Cryptocurrency, fiat: &str) -> Result<PriceData> {
        Ok(PriceData {
            cryptocurrency: crypto.clone(),
            fiat_currency: fiat.to_string(),
            price: 50000.0,
            timestamp: chrono::Utc::now().timestamp(),
            source: "coingate".to_string(),
        })
    }

    async fn estimate_fee(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
        priority: &FeePriority,
    ) -> Result<FeeEstimate> {
        Ok(FeeEstimate {
            cryptocurrency: crypto.clone(),
            network: network.clone(),
            priority: priority.clone(),
            fee_amount: "0.0001".to_string(),
            fee_rate: "10 sat/vB".to_string(),
            estimated_time_minutes: 30,
        })
    }

    async fn validate_address(
        &self,
        address: &str,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<bool> {
        let wallet_address = WalletAddress::new(
            address.to_string(),
            crypto.clone(),
            network.clone(),
        );
        wallet_address.validate()
    }

    async fn generate_address(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
    ) -> Result<WalletAddress> {
        super::wallet::generate_wallet_address(crypto, network)
    }

    async fn get_transaction(&self, tx_hash: &str, network: &Network) -> Result<Transaction> {
        let client = super::blockchain::BlockchainClient::new(
            Cryptocurrency::Bitcoin,
            network.clone(),
        );
        client.get_transaction(tx_hash).await
    }

    async fn get_confirmations(&self, _tx_hash: &str, _network: &Network) -> Result<u32> {
        Ok(6)
    }

    fn verify_webhook(&self, _payload: &[u8], _signature: &str) -> Result<bool> {
        // Mock implementation
        Ok(true)
    }
}

/// Factory function to create payment provider based on configuration
pub fn create_provider(provider_type: &super::CryptoProvider, config: &super::CryptoConfig) -> Result<Box<dyn CryptoPaymentProvider>> {
    match provider_type {
        super::CryptoProvider::Native => {
            // Return native Bitcoin or Ethereum provider based on network
            match config.network {
                Network::BitcoinMainnet | Network::BitcoinTestnet | Network::BitcoinRegtest => {
                    Ok(Box::new(super::bitcoin::BitcoinProvider::new(config.network.clone())))
                }
                Network::EthereumMainnet | Network::EthereumGoerli | Network::EthereumSepolia => {
                    Ok(Box::new(super::ethereum::EthereumProvider::new(config.network.clone())))
                }
                _ => Err(PayupError::UnsupportedOperation(
                    format!("Native provider not available for network {:?}", config.network)
                ))
            }
        }
        super::CryptoProvider::CoinbaseCommerce => {
            let api_key = config.api_key.as_ref()
                .ok_or_else(|| PayupError::ValidationError("API key required for Coinbase Commerce".to_string()))?;
            
            let mut provider = CoinbaseCommerceProvider::new(api_key.clone());
            if let Some(secret) = &config.webhook_secret {
                provider = provider.with_webhook_secret(secret.clone());
            }
            Ok(Box::new(provider))
        }
        super::CryptoProvider::BitPay => {
            let api_token = config.api_key.as_ref()
                .ok_or_else(|| PayupError::ValidationError("API token required for BitPay".to_string()))?;
            
            Ok(Box::new(BitPayProvider::new(api_token.clone())))
        }
        super::CryptoProvider::CoinGate => {
            let api_key = config.api_key.as_ref()
                .ok_or_else(|| PayupError::ValidationError("API key required for CoinGate".to_string()))?;
            
            Ok(Box::new(CoinGateProvider::new(api_key.clone())))
        }
        _ => Err(PayupError::UnsupportedOperation(
            format!("Provider {:?} not yet implemented", provider_type)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_coinbase_commerce_provider() {
        let provider = CoinbaseCommerceProvider::new("test_api_key".to_string());
        
        let cryptos = provider.supported_cryptocurrencies();
        assert!(cryptos.contains(&Cryptocurrency::Bitcoin));
        assert!(cryptos.contains(&Cryptocurrency::Ethereum));
        assert!(cryptos.contains(&Cryptocurrency::USDC));
    }

    #[tokio::test]
    async fn test_bitpay_provider() {
        let provider = BitPayProvider::new("test_token".to_string());
        
        let cryptos = provider.supported_cryptocurrencies();
        assert!(cryptos.contains(&Cryptocurrency::Bitcoin));
        assert!(cryptos.contains(&Cryptocurrency::Dogecoin));
    }

    #[tokio::test]
    async fn test_coingate_provider() {
        let provider = CoinGateProvider::new("test_api_key".to_string());
        
        let networks = provider.supported_networks();
        assert!(networks.contains(&Network::BitcoinMainnet));
        assert!(networks.contains(&Network::Polygon));
        assert!(networks.contains(&Network::BinanceSmartChain));
    }

    #[test]
    fn test_provider_factory() {
        let mut config = super::super::CryptoConfig::default();
        config.api_key = Some("test_key".to_string());
        
        // Test Coinbase Commerce creation
        config.provider = super::super::CryptoProvider::CoinbaseCommerce;
        let provider = create_provider(&config.provider, &config).unwrap();
        assert_eq!(provider.name(), "Coinbase Commerce");
        
        // Test BitPay creation
        config.provider = super::super::CryptoProvider::BitPay;
        let provider = create_provider(&config.provider, &config).unwrap();
        assert_eq!(provider.name(), "BitPay");
        
        // Test Native Bitcoin creation
        config.provider = super::super::CryptoProvider::Native;
        config.network = Network::BitcoinMainnet;
        let provider = create_provider(&config.provider, &config).unwrap();
        assert_eq!(provider.name(), "Bitcoin");
    }
}