use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::{
    CryptoPaymentProvider, CryptoPaymentRequest, CryptoPayment,
    Cryptocurrency, Network, PaymentStatus, PriceData, FeeEstimate,
    FeePriority, wallet::WalletAddress, blockchain::Transaction,
    types::*,
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Ethereum payment provider with support for ETH and ERC-20 tokens
pub struct EthereumProvider {
    pub network: Network,
    pub rpc_url: Option<String>,
    pub api_key: Option<String>,
    pub infura_project_id: Option<String>,
    pub alchemy_api_key: Option<String>,
    pub layer2_enabled: bool,
}

impl EthereumProvider {
    pub fn new(network: Network) -> Self {
        EthereumProvider {
            network,
            rpc_url: None,
            api_key: None,
            infura_project_id: None,
            alchemy_api_key: None,
            layer2_enabled: false,
        }
    }

    pub fn with_infura(mut self, project_id: String) -> Self {
        self.infura_project_id = Some(project_id.clone());
        self.rpc_url = Some(self.get_infura_endpoint(&project_id));
        self
    }

    pub fn with_alchemy(mut self, api_key: String) -> Self {
        self.alchemy_api_key = Some(api_key.clone());
        self.rpc_url = Some(self.get_alchemy_endpoint(&api_key));
        self
    }

    pub fn with_layer2(mut self) -> Self {
        self.layer2_enabled = true;
        self
    }

    fn get_infura_endpoint(&self, project_id: &str) -> String {
        match self.network {
            Network::EthereumMainnet => format!("https://mainnet.infura.io/v3/{}", project_id),
            Network::EthereumGoerli => format!("https://goerli.infura.io/v3/{}", project_id),
            Network::EthereumSepolia => format!("https://sepolia.infura.io/v3/{}", project_id),
            Network::Polygon => format!("https://polygon-mainnet.infura.io/v3/{}", project_id),
            Network::Arbitrum => format!("https://arbitrum-mainnet.infura.io/v3/{}", project_id),
            Network::Optimism => format!("https://optimism-mainnet.infura.io/v3/{}", project_id),
            _ => format!("https://mainnet.infura.io/v3/{}", project_id),
        }
    }

    fn get_alchemy_endpoint(&self, api_key: &str) -> String {
        match self.network {
            Network::EthereumMainnet => format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key),
            Network::EthereumGoerli => format!("https://eth-goerli.g.alchemy.com/v2/{}", api_key),
            Network::EthereumSepolia => format!("https://eth-sepolia.g.alchemy.com/v2/{}", api_key),
            Network::Polygon => format!("https://polygon-mainnet.g.alchemy.com/v2/{}", api_key),
            Network::Arbitrum => format!("https://arb-mainnet.g.alchemy.com/v2/{}", api_key),
            Network::Optimism => format!("https://opt-mainnet.g.alchemy.com/v2/{}", api_key),
            _ => format!("https://eth-mainnet.g.alchemy.com/v2/{}", api_key),
        }
    }

    /// Create EIP-681 payment URI for Ethereum
    pub fn create_payment_uri(
        &self,
        address: &str,
        amount_eth: Option<f64>,
        token_address: Option<&str>,
        chain_id: Option<u64>,
    ) -> String {
        let mut uri = format!("ethereum:{}", address);
        
        if let Some(token) = token_address {
            uri = format!("ethereum:{}@{}/transfer", token, chain_id.unwrap_or(1));
            uri.push_str(&format!("?address={}", address));
            
            if let Some(amt) = amount_eth {
                let wei = (amt * 1e18) as u128;
                uri.push_str(&format!("&uint256={}", wei));
            }
        } else if let Some(amt) = amount_eth {
            let wei = (amt * 1e18) as u128;
            uri.push_str(&format!("?value={}", wei));
        }
        
        uri
    }

    /// Estimate gas for a transaction
    pub async fn estimate_gas(
        &self,
        from: &str,
        to: &str,
        value: Option<&str>,
        data: Option<&str>,
    ) -> Result<u64> {
        // Mock implementation - in production, use JSON-RPC
        let base_gas = if data.is_some() { 100000 } else { 21000 };
        Ok(base_gas)
    }

    /// Get current gas prices
    pub async fn get_gas_prices(&self) -> Result<GasPrices> {
        // Mock implementation - in production, query gas oracle
        Ok(GasPrices {
            safe_low: 20,
            standard: 30,
            fast: 50,
            fastest: 70,
            base_fee: Some(25),
            priority_fees: Some(PriorityFees {
                low: 1,
                medium: 2,
                high: 3,
            }),
        })
    }

    /// Check if address is a smart contract
    pub async fn is_contract(&self, address: &str) -> Result<bool> {
        // Mock implementation - in production, check bytecode
        Ok(address.len() == 42 && address.starts_with("0x"))
    }

    /// Get ERC-20 token balance
    pub async fn get_token_balance(
        &self,
        token_address: &str,
        wallet_address: &str,
    ) -> Result<String> {
        // Mock implementation
        Ok("1000000".to_string()) // 1 USDC (6 decimals)
    }

    /// Send raw transaction
    pub async fn send_raw_transaction(&self, signed_tx: &str) -> Result<String> {
        // Mock implementation
        Ok(format!("0x{}", hex::encode(&signed_tx.as_bytes()[0..32])))
    }

    /// Create transaction for ERC-20 transfer
    pub fn create_erc20_transfer(
        &self,
        token_address: &str,
        to: &str,
        amount: &str,
    ) -> Result<EthereumTransaction> {
        let data = self.encode_erc20_transfer(to, amount)?;
        
        Ok(EthereumTransaction {
            from: None,
            to: Some(token_address.to_string()),
            value: "0x0".to_string(),
            data: Some(data),
            gas: None,
            gas_price: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            nonce: None,
            chain_id: self.network.chain_id(),
        })
    }

    /// Encode ERC-20 transfer function call
    fn encode_erc20_transfer(&self, to: &str, amount: &str) -> Result<String> {
        // Function selector for transfer(address,uint256)
        let selector = "0xa9059cbb";
        
        // Remove 0x prefix from address if present
        let address = if to.starts_with("0x") { &to[2..] } else { to };
        let padded_address = format!("{:0>64}", address);
        
        // Convert amount to hex and pad
        let amount_hex = format!("{:x}", amount.parse::<u128>().unwrap_or(0));
        let padded_amount = format!("{:0>64}", amount_hex);
        
        Ok(format!("{}{}{}", selector, padded_address, padded_amount))
    }

    /// Monitor pending transaction
    pub async fn wait_for_transaction(
        &self,
        tx_hash: &str,
        confirmations_required: u32,
    ) -> Result<TransactionReceipt> {
        // Mock implementation
        Ok(TransactionReceipt {
            transaction_hash: tx_hash.to_string(),
            block_number: 17000000,
            block_hash: "0xabc123".to_string(),
            from: "0xfrom".to_string(),
            to: Some("0xto".to_string()),
            contract_address: None,
            gas_used: 21000,
            effective_gas_price: 30000000000,
            status: true,
            logs: Vec::new(),
        })
    }

    /// Get account nonce
    pub async fn get_nonce(&self, address: &str) -> Result<u64> {
        // Mock implementation
        Ok(1)
    }

    /// Sign transaction (requires private key)
    pub fn sign_transaction(
        &self,
        tx: &EthereumTransaction,
        private_key: &str,
    ) -> Result<String> {
        // Mock implementation - in production, use proper signing
        Ok(format!("0xsigned_{}", private_key.chars().take(8).collect::<String>()))
    }
}

#[async_trait]
impl CryptoPaymentProvider for EthereumProvider {
    fn name(&self) -> &str {
        "Ethereum"
    }

    fn supported_cryptocurrencies(&self) -> Vec<Cryptocurrency> {
        vec![
            Cryptocurrency::Ethereum,
            Cryptocurrency::USDC,
            Cryptocurrency::USDT,
            Cryptocurrency::DAI,
        ]
    }

    fn supported_networks(&self) -> Vec<Network> {
        let mut networks = vec![
            Network::EthereumMainnet,
            Network::EthereumGoerli,
            Network::EthereumSepolia,
        ];
        
        if self.layer2_enabled {
            networks.extend(vec![
                Network::Polygon,
                Network::Arbitrum,
                Network::Optimism,
            ]);
        }
        
        networks
    }

    async fn create_payment(&self, request: &CryptoPaymentRequest) -> Result<CryptoPayment> {
        let wallet_address = self.generate_address(
            &request.cryptocurrency,
            &request.network,
        ).await?;

        let payment_id = format!("eth_{}", uuid::Uuid::new_v4());
        let expires_at = chrono::Utc::now().timestamp() + 900; // 15 minutes

        let payment_uri = if request.cryptocurrency.is_erc20() {
            let token_address = self.get_token_contract_address(&request.cryptocurrency)?;
            Some(self.create_payment_uri(
                &wallet_address.address,
                Some(request.amount.parse().unwrap_or(0.0)),
                Some(&token_address),
                self.network.chain_id(),
            ))
        } else {
            Some(self.create_payment_uri(
                &wallet_address.address,
                Some(request.amount.parse().unwrap_or(0.0)),
                None,
                self.network.chain_id(),
            ))
        };

        Ok(CryptoPayment {
            id: payment_id,
            status: PaymentStatus::Pending,
            cryptocurrency: request.cryptocurrency.clone(),
            network: request.network.clone(),
            amount_crypto: request.amount.clone(),
            amount_fiat: None,
            fiat_currency: None,
            exchange_rate: None,
            wallet_address: wallet_address.address,
            payment_uri,
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 12,
            created_at: chrono::Utc::now().timestamp(),
            expires_at,
            completed_at: None,
            metadata: request.metadata.clone(),
        })
    }

    async fn get_payment(&self, payment_id: &str) -> Result<CryptoPayment> {
        // Mock implementation
        Ok(CryptoPayment {
            id: payment_id.to_string(),
            status: PaymentStatus::Pending,
            cryptocurrency: Cryptocurrency::Ethereum,
            network: self.network.clone(),
            amount_crypto: "0.1".to_string(),
            amount_fiat: Some("200.00".to_string()),
            fiat_currency: Some("USD".to_string()),
            exchange_rate: Some(2000.0),
            wallet_address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string(),
            payment_uri: None,
            qr_code: None,
            transaction_hash: None,
            confirmations: 0,
            confirmations_required: 12,
            created_at: chrono::Utc::now().timestamp() - 300,
            expires_at: chrono::Utc::now().timestamp() + 600,
            completed_at: None,
            metadata: None,
        })
    }

    async fn cancel_payment(&self, payment_id: &str) -> Result<bool> {
        Ok(true)
    }

    async fn list_payments(&self, limit: Option<u32>) -> Result<Vec<CryptoPayment>> {
        Ok(Vec::new())
    }

    async fn get_exchange_rate(&self, crypto: &Cryptocurrency, fiat: &str) -> Result<PriceData> {
        let price = match crypto {
            Cryptocurrency::Ethereum => 2000.0,
            Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => 1.0,
            _ => 0.0,
        };

        Ok(PriceData {
            cryptocurrency: crypto.clone(),
            fiat_currency: fiat.to_string(),
            price,
            timestamp: chrono::Utc::now().timestamp(),
            source: "mock".to_string(),
        })
    }

    async fn estimate_fee(
        &self,
        crypto: &Cryptocurrency,
        network: &Network,
        priority: &FeePriority,
    ) -> Result<FeeEstimate> {
        let gas_prices = self.get_gas_prices().await?;
        let gas_price = match priority {
            FeePriority::Low => gas_prices.safe_low,
            FeePriority::Medium => gas_prices.standard,
            FeePriority::High => gas_prices.fast,
            FeePriority::Custom(gwei) => gwei.parse().unwrap_or(gas_prices.standard),
        };

        let gas_limit = if crypto.is_erc20() { 100000 } else { 21000 };
        let fee_wei = gas_price as u128 * gas_limit * 1_000_000_000; // Convert gwei to wei
        let fee_eth = fee_wei as f64 / 1e18;

        Ok(FeeEstimate {
            cryptocurrency: crypto.clone(),
            network: network.clone(),
            priority: priority.clone(),
            fee_amount: format!("{:.6}", fee_eth),
            fee_rate: format!("{} gwei", gas_price),
            estimated_time_minutes: match priority {
                FeePriority::High => 1,
                FeePriority::Medium => 3,
                FeePriority::Low => 10,
                _ => 3,
            },
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
            Cryptocurrency::Ethereum,
            network.clone(),
        );
        client.get_transaction(tx_hash).await
    }

    async fn get_confirmations(&self, tx_hash: &str, network: &Network) -> Result<u32> {
        let tx = self.get_transaction(tx_hash, network).await?;
        Ok(tx.confirmations)
    }

    fn verify_webhook(&self, payload: &[u8], signature: &str) -> Result<bool> {
        // Mock implementation - in production, verify HMAC signature
        Ok(true)
    }
}

impl EthereumProvider {
    fn get_token_contract_address(&self, crypto: &Cryptocurrency) -> Result<String> {
        match (crypto, &self.network) {
            (Cryptocurrency::USDC, Network::EthereumMainnet) => {
                Ok("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string())
            }
            (Cryptocurrency::USDT, Network::EthereumMainnet) => {
                Ok("0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string())
            }
            (Cryptocurrency::DAI, Network::EthereumMainnet) => {
                Ok("0x6B175474E89094C44Da98b954EedeAC495271d0F".to_string())
            }
            _ => Err(PayupError::UnsupportedOperation(
                format!("Token {} not supported on {:?}", crypto.symbol(), self.network)
            ))
        }
    }
}

/// Ethereum transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthereumTransaction {
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: String, // In wei, as hex string
    pub data: Option<String>,
    pub gas: Option<u64>,
    pub gas_price: Option<u64>, // Legacy
    pub max_fee_per_gas: Option<u64>, // EIP-1559
    pub max_priority_fee_per_gas: Option<u64>, // EIP-1559
    pub nonce: Option<u64>,
    pub chain_id: Option<u64>,
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub block_hash: String,
    pub from: String,
    pub to: Option<String>,
    pub contract_address: Option<String>,
    pub gas_used: u64,
    pub effective_gas_price: u64,
    pub status: bool,
    pub logs: Vec<EventLog>,
}

/// Event log from transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub log_index: u32,
    pub removed: bool,
}

/// Gas price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPrices {
    pub safe_low: u64,  // In gwei
    pub standard: u64,
    pub fast: u64,
    pub fastest: u64,
    pub base_fee: Option<u64>, // EIP-1559
    pub priority_fees: Option<PriorityFees>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityFees {
    pub low: u64,
    pub medium: u64,
    pub high: u64,
}

/// ENS (Ethereum Name Service) resolver
pub struct EnsResolver {
    network: Network,
}

impl EnsResolver {
    pub fn new(network: Network) -> Self {
        EnsResolver { network }
    }

    /// Resolve ENS name to address
    pub async fn resolve_name(&self, ens_name: &str) -> Result<String> {
        // Mock implementation
        if ens_name.ends_with(".eth") {
            Ok("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string())
        } else {
            Err(PayupError::ValidationError("Invalid ENS name".to_string()))
        }
    }

    /// Reverse resolve address to ENS name
    pub async fn reverse_resolve(&self, address: &str) -> Result<Option<String>> {
        // Mock implementation
        Ok(Some("vitalik.eth".to_string()))
    }
}

/// Web3 provider for direct Ethereum interaction
pub struct Web3Provider {
    rpc_url: String,
}

impl Web3Provider {
    pub fn new(rpc_url: String) -> Self {
        Web3Provider { rpc_url }
    }

    /// Call JSON-RPC method
    pub async fn call(&self, method: &str, params: Vec<serde_json::Value>) -> Result<serde_json::Value> {
        // Mock implementation
        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "0x1"
        }))
    }

    /// Get block by number
    pub async fn get_block(&self, block_number: &str) -> Result<serde_json::Value> {
        // Mock implementation
        Ok(serde_json::json!({
            "number": block_number,
            "hash": "0xabc123",
            "timestamp": "0x60000000",
            "transactions": []
        }))
    }

    /// Get transaction by hash
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<serde_json::Value> {
        // Mock implementation
        Ok(serde_json::json!({
            "hash": tx_hash,
            "from": "0xfrom",
            "to": "0xto",
            "value": "0x0",
            "gas": "0x5208",
            "gasPrice": "0x3b9aca00"
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_uri_generation() {
        let provider = EthereumProvider::new(Network::EthereumMainnet);
        
        // ETH payment URI
        let uri = provider.create_payment_uri(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7",
            Some(0.1),
            None,
            Some(1),
        );
        assert!(uri.starts_with("ethereum:0x"));
        assert!(uri.contains("value="));

        // ERC-20 token payment URI
        let uri = provider.create_payment_uri(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7",
            Some(100.0),
            Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            Some(1),
        );
        assert!(uri.contains("/transfer"));
        assert!(uri.contains("uint256="));
    }

    #[tokio::test]
    async fn test_gas_estimation() {
        let provider = EthereumProvider::new(Network::EthereumMainnet);
        
        let gas = provider.estimate_gas(
            "0xfrom",
            "0xto",
            Some("1000000000000000000"),
            None,
        ).await.unwrap();
        
        assert_eq!(gas, 21000);

        let gas_with_data = provider.estimate_gas(
            "0xfrom",
            "0xto",
            Some("0"),
            Some("0x1234"),
        ).await.unwrap();
        
        assert_eq!(gas_with_data, 100000);
    }

    #[test]
    fn test_erc20_transfer_encoding() {
        let provider = EthereumProvider::new(Network::EthereumMainnet);
        
        let data = provider.encode_erc20_transfer(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7",
            "1000000",
        ).unwrap();
        
        assert!(data.starts_with("0xa9059cbb")); // transfer function selector
        assert_eq!(data.len(), 138); // 10 (selector) + 64 (address) + 64 (amount)
    }

    #[tokio::test]
    async fn test_ens_resolution() {
        let resolver = EnsResolver::new(Network::EthereumMainnet);
        
        let address = resolver.resolve_name("vitalik.eth").await.unwrap();
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42);

        let name = resolver.reverse_resolve(&address).await.unwrap();
        assert_eq!(name, Some("vitalik.eth".to_string()));
    }
}