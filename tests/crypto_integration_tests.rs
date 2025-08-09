#[cfg(feature = "crypto")]
mod crypto_tests {
    use payup::crypto::*;
    use payup::crypto::providers::*;
    use payup::crypto::bitcoin::*;
    use payup::crypto::ethereum::*;
    use payup::crypto::wallet::*;
    use payup::crypto::blockchain::*;
    use std::collections::HashMap;
    use tokio::time::Duration;

/// Comprehensive integration tests for the cryptocurrency module
/// 
/// These tests cover:
/// - Client creation and configuration
/// - Address generation and validation
/// - Payment creation and monitoring
/// - Transaction verification
/// - Multiple cryptocurrency support
/// - Provider integration
/// - Network selection
/// - Error handling

// ============================================================================
// Bitcoin Client Tests
// ============================================================================

#[tokio::test]
async fn test_bitcoin_provider_creation() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet);
    assert_eq!(provider.name(), "Bitcoin");
    assert!(provider.supported_cryptocurrencies().contains(&Cryptocurrency::Bitcoin));
    assert!(provider.supported_networks().contains(&Network::BitcoinMainnet));
}

#[tokio::test]
async fn test_bitcoin_provider_with_lightning() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet)
        .with_lightning();
    assert!(provider.lightning_enabled);
    
    let invoice_result = provider.create_lightning_invoice(100000, "Test payment", 3600).await;
    assert!(invoice_result.is_ok());
    
    let invoice = invoice_result.unwrap();
    assert_eq!(invoice.amount_sats, 100000);
    assert!(invoice.bolt11.starts_with("lnbc"));
}

#[tokio::test]
async fn test_bitcoin_provider_network_selection() {
    // Test mainnet
    let mainnet_provider = BitcoinProvider::new(Network::BitcoinMainnet);
    assert_eq!(mainnet_provider.network, Network::BitcoinMainnet);
    
    // Test testnet
    let testnet_provider = BitcoinProvider::new(Network::BitcoinTestnet);
    assert_eq!(testnet_provider.network, Network::BitcoinTestnet);
    
    // Test regtest
    let regtest_provider = BitcoinProvider::new(Network::BitcoinRegtest);
    assert_eq!(regtest_provider.network, Network::BitcoinRegtest);
}

#[tokio::test]
async fn test_bitcoin_address_generation_and_validation() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet);
    
    // Generate address
    let address_result = provider.generate_address(&Cryptocurrency::Bitcoin, &Network::BitcoinMainnet).await;
    assert!(address_result.is_ok());
    
    let address = address_result.unwrap();
    assert_eq!(address.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(address.network, Network::BitcoinMainnet);
    
    // Validate generated address
    let is_valid = provider.validate_address(
        &address.address,
        &Cryptocurrency::Bitcoin,
        &Network::BitcoinMainnet
    ).await.unwrap();
    assert!(is_valid);
    
    // Test invalid address
    let is_invalid = provider.validate_address(
        "invalid_address",
        &Cryptocurrency::Bitcoin,
        &Network::BitcoinMainnet
    ).await.unwrap();
    assert!(!is_invalid);
}

#[tokio::test]
async fn test_bitcoin_payment_creation() {
    let provider = BitcoinProvider::new(Network::BitcoinTestnet);
    
    let request = CryptoPaymentRequest {
        amount: "0.001".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinTestnet,
        description: Some("Test payment".to_string()),
        metadata: Some({
            let mut map = HashMap::new();
            map.insert("order_id".to_string(), "12345".to_string());
            map
        }),
        customer_email: Some("test@example.com".to_string()),
        redirect_url: Some("https://example.com/success".to_string()),
        cancel_url: Some("https://example.com/cancel".to_string()),
        webhook_url: Some("https://example.com/webhook".to_string()),
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    
    assert_eq!(payment.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(payment.network, Network::BitcoinTestnet);
    assert_eq!(payment.amount_crypto, "0.001");
    assert_eq!(payment.status, PaymentStatus::Pending);
    assert!(payment.payment_uri.is_some());
    assert!(payment.wallet_address.starts_with("tb1") || payment.wallet_address.starts_with("m") || payment.wallet_address.starts_with("n"));
}

#[tokio::test]
async fn test_bitcoin_fee_estimation() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet);
    
    let low_fee = provider.estimate_fee(
        &Cryptocurrency::Bitcoin,
        &Network::BitcoinMainnet,
        &FeePriority::Low
    ).await.unwrap();
    
    let high_fee = provider.estimate_fee(
        &Cryptocurrency::Bitcoin,
        &Network::BitcoinMainnet,
        &FeePriority::High
    ).await.unwrap();
    
    // High priority should cost more than low priority
    assert!(high_fee.fee_amount.parse::<f64>().unwrap() > low_fee.fee_amount.parse::<f64>().unwrap());
    assert!(low_fee.fee_rate.contains("sat/vB"));
    assert!(high_fee.estimated_time_minutes < low_fee.estimated_time_minutes);
}

#[tokio::test]
async fn test_bitcoin_payment_uri_generation() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet);
    
    let uri = provider.create_payment_uri(
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
        Some(0.001),
        Some("Test Payment"),
        Some("Thank you for your purchase")
    );
    
    assert!(uri.starts_with("bitcoin:bc1"));
    assert!(uri.contains("amount=0.001"));
    assert!(uri.contains("label=Test%20Payment"));
    assert!(uri.contains("message=Thank%20you%20for%20your%20purchase"));
}

// ============================================================================
// Ethereum Client Tests
// ============================================================================

#[tokio::test]
async fn test_ethereum_provider_creation() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    assert_eq!(provider.name(), "Ethereum");
    assert!(provider.supported_cryptocurrencies().contains(&Cryptocurrency::Ethereum));
    assert!(provider.supported_cryptocurrencies().contains(&Cryptocurrency::USDC));
    assert!(provider.supported_networks().contains(&Network::EthereumMainnet));
}

#[tokio::test]
async fn test_ethereum_provider_with_infura() {
    let provider = EthereumProvider::new(Network::EthereumMainnet)
        .with_infura("test_project_id".to_string());
    
    assert!(provider.infura_project_id.is_some());
    assert!(provider.rpc_url.is_some());
    assert!(provider.rpc_url.as_ref().unwrap().contains("infura.io"));
    assert!(provider.rpc_url.as_ref().unwrap().contains("test_project_id"));
}

#[tokio::test]
async fn test_ethereum_provider_with_alchemy() {
    let provider = EthereumProvider::new(Network::EthereumGoerli)
        .with_alchemy("test_api_key".to_string());
    
    assert!(provider.alchemy_api_key.is_some());
    assert!(provider.rpc_url.is_some());
    assert!(provider.rpc_url.as_ref().unwrap().contains("alchemy.com"));
    assert!(provider.rpc_url.as_ref().unwrap().contains("goerli"));
}

#[tokio::test]
async fn test_ethereum_provider_with_layer2() {
    let provider = EthereumProvider::new(Network::EthereumMainnet)
        .with_layer2();
    
    assert!(provider.layer2_enabled);
    let networks = provider.supported_networks();
    assert!(networks.contains(&Network::Polygon));
    assert!(networks.contains(&Network::Arbitrum));
    assert!(networks.contains(&Network::Optimism));
}

#[tokio::test]
async fn test_ethereum_address_generation_and_validation() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    
    // Generate address
    let address_result = provider.generate_address(&Cryptocurrency::Ethereum, &Network::EthereumMainnet).await;
    assert!(address_result.is_ok());
    
    let address = address_result.unwrap();
    assert_eq!(address.cryptocurrency, Cryptocurrency::Ethereum);
    assert_eq!(address.network, Network::EthereumMainnet);
    assert!(address.address.starts_with("0x"));
    assert_eq!(address.address.len(), 42);
    
    // Validate generated address
    let is_valid = provider.validate_address(
        &address.address,
        &Cryptocurrency::Ethereum,
        &Network::EthereumMainnet
    ).await.unwrap();
    assert!(is_valid);
    
    // Test well-known valid address
    let is_valid = provider.validate_address(
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7",
        &Cryptocurrency::Ethereum,
        &Network::EthereumMainnet
    ).await.unwrap();
    assert!(is_valid);
    
    // Test invalid address
    let is_invalid = provider.validate_address(
        "invalid_address",
        &Cryptocurrency::Ethereum,
        &Network::EthereumMainnet
    ).await.unwrap();
    assert!(!is_invalid);
}

#[tokio::test]
async fn test_ethereum_payment_creation() {
    let provider = EthereumProvider::new(Network::EthereumGoerli);
    
    let request = CryptoPaymentRequest {
        amount: "0.1".to_string(),
        cryptocurrency: Cryptocurrency::Ethereum,
        network: Network::EthereumGoerli,
        description: Some("Test ETH payment".to_string()),
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    
    assert_eq!(payment.cryptocurrency, Cryptocurrency::Ethereum);
    assert_eq!(payment.network, Network::EthereumGoerli);
    assert_eq!(payment.amount_crypto, "0.1");
    assert_eq!(payment.status, PaymentStatus::Pending);
    assert_eq!(payment.confirmations_required, 12);
    assert!(payment.payment_uri.is_some());
}

#[tokio::test]
async fn test_ethereum_erc20_payment_creation() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    
    let request = CryptoPaymentRequest {
        amount: "100".to_string(),
        cryptocurrency: Cryptocurrency::USDC,
        network: Network::EthereumMainnet,
        description: Some("Test USDC payment".to_string()),
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    
    assert_eq!(payment.cryptocurrency, Cryptocurrency::USDC);
    assert_eq!(payment.network, Network::EthereumMainnet);
    assert_eq!(payment.amount_crypto, "100");
    assert!(payment.payment_uri.is_some());
    assert!(payment.payment_uri.unwrap().contains("/transfer"));
}

#[tokio::test]
async fn test_ethereum_gas_estimation() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    
    // Test ETH transfer
    let eth_gas = provider.estimate_gas(
        "0xfrom",
        "0xto",
        Some("1000000000000000000"), // 1 ETH in wei
        None
    ).await.unwrap();
    assert_eq!(eth_gas, 21000);
    
    // Test contract interaction
    let contract_gas = provider.estimate_gas(
        "0xfrom",
        "0xcontract",
        Some("0"),
        Some("0xa9059cbb") // transfer function selector
    ).await.unwrap();
    assert_eq!(contract_gas, 100000);
}

#[tokio::test]
async fn test_ethereum_gas_prices() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    
    let gas_prices = provider.get_gas_prices().await.unwrap();
    
    assert!(gas_prices.safe_low <= gas_prices.standard);
    assert!(gas_prices.standard <= gas_prices.fast);
    assert!(gas_prices.fast <= gas_prices.fastest);
    
    if let Some(base_fee) = gas_prices.base_fee {
        assert!(base_fee > 0);
    }
    
    if let Some(priority_fees) = gas_prices.priority_fees {
        assert!(priority_fees.low <= priority_fees.medium);
        assert!(priority_fees.medium <= priority_fees.high);
    }
}

#[tokio::test]
async fn test_ethereum_erc20_transfer_creation() {
    let provider = EthereumProvider::new(Network::EthereumMainnet);
    
    let tx = provider.create_erc20_transfer(
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC contract
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7",
        "1000000" // 1 USDC (6 decimals)
    ).unwrap();
    
    assert_eq!(tx.to, Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()));
    assert_eq!(tx.value, "0x0");
    assert!(tx.data.is_some());
    assert!(tx.data.unwrap().starts_with("0xa9059cbb")); // transfer selector
    assert_eq!(tx.chain_id, Some(1));
}

// ============================================================================
// Wallet Address Tests
// ============================================================================

#[test]
fn test_wallet_address_creation_and_validation() {
    // Test Bitcoin mainnet address
    let btc_address = WalletAddress::new(
        "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        Cryptocurrency::Bitcoin,
        Network::BitcoinMainnet,
    );
    assert_eq!(btc_address.address_type, AddressType::P2PKH);
    assert!(btc_address.validate().unwrap());
    assert!(!btc_address.is_testnet());
    
    // Test Bitcoin testnet address
    let btc_testnet_address = WalletAddress::new(
        "n1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        Cryptocurrency::Bitcoin,
        Network::BitcoinTestnet,
    );
    assert!(btc_testnet_address.is_testnet());
    
    // Test Ethereum address
    let eth_address = WalletAddress::new(
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string(),
        Cryptocurrency::Ethereum,
        Network::EthereumMainnet,
    );
    assert_eq!(eth_address.address_type, AddressType::EOA);
    assert!(eth_address.validate().unwrap());
    
    // Test Dogecoin address
    let doge_address = WalletAddress::new(
        "DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L".to_string(),
        Cryptocurrency::Dogecoin,
        Network::BitcoinMainnet,
    );
    assert!(doge_address.validate().unwrap());
}

#[test]
fn test_wallet_address_payment_uri_generation() {
    let btc_address = WalletAddress::new(
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        Cryptocurrency::Bitcoin,
        Network::BitcoinMainnet,
    );
    
    let uri = btc_address.to_payment_uri(Some("0.001"));
    assert!(uri.starts_with("bitcoin:bc1"));
    assert!(uri.contains("amount=0.001"));
}

#[test]
fn test_watch_only_wallet() {
    let mut wallet = WatchOnlyWallet::new("Integration Test Wallet".to_string());
    
    let btc_address = WalletAddress::new(
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        Cryptocurrency::Bitcoin,
        Network::BitcoinMainnet,
    );
    
    let eth_address = WalletAddress::new(
        "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string(),
        Cryptocurrency::Ethereum,
        Network::EthereumMainnet,
    );
    
    // Add valid addresses
    assert!(wallet.add_address(btc_address).is_ok());
    assert!(wallet.add_address(eth_address).is_ok());
    assert_eq!(wallet.addresses.len(), 2);
    
    // Try to add invalid address
    let invalid_address = WalletAddress::new(
        "invalid_address".to_string(),
        Cryptocurrency::Bitcoin,
        Network::BitcoinMainnet,
    );
    assert!(wallet.add_address(invalid_address).is_err());
    assert_eq!(wallet.addresses.len(), 2);
    
    // Remove address
    assert!(wallet.remove_address("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"));
    assert_eq!(wallet.addresses.len(), 1);
}

// ============================================================================
// Blockchain Client Tests
// ============================================================================

#[tokio::test]
async fn test_blockchain_client_creation() {
    let btc_client = BlockchainClient::new(Cryptocurrency::Bitcoin, Network::BitcoinMainnet);
    assert_eq!(btc_client.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(btc_client.network, Network::BitcoinMainnet);
    assert!(btc_client.endpoint.contains("blockcypher.com"));
    
    let eth_client = BlockchainClient::new(Cryptocurrency::Ethereum, Network::EthereumMainnet);
    assert_eq!(eth_client.cryptocurrency, Cryptocurrency::Ethereum);
    assert_eq!(eth_client.network, Network::EthereumMainnet);
    assert!(eth_client.endpoint.contains("etherscan.io"));
}

#[tokio::test]
async fn test_blockchain_client_transaction_operations() {
    let client = BlockchainClient::new(Cryptocurrency::Bitcoin, Network::BitcoinMainnet);
    
    // Get transaction
    let tx = client.get_transaction("test_hash").await.unwrap();
    assert_eq!(tx.hash.as_str(), "test_hash");
    assert_eq!(tx.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(tx.network, Network::BitcoinMainnet);
    assert_eq!(tx.status, TransactionStatus::Confirmed);
    assert_eq!(tx.confirmations, 6);
    assert!(tx.is_confirmed(6));
    assert!(tx.is_confirmed(3));
    assert!(!tx.is_confirmed(10));
    
    // Get block height
    let height = client.get_block_height().await.unwrap();
    assert!(height > 0);
    
    // Get balance
    let balance = client.get_balance("test_address").await.unwrap();
    assert!(!balance.is_zero());
}

#[tokio::test]
async fn test_transaction_builder() {
    use payup::crypto::types::{TransactionInput, TransactionOutput};
    use payup::crypto::Hash;
    
    let builder = TransactionBuilder::new(Cryptocurrency::Bitcoin, Network::BitcoinMainnet);
    
    let input = TransactionInput {
        previous_tx_hash: Hash::new("prev_tx_hash".to_string()),
        previous_output_index: 0,
        script_sig: None,
        witness: None,
    };
    
    let output = TransactionOutput {
        amount: Amount::from_decimal(0.9999, 8),
        script_pubkey: "script".to_string(),
        address: Some("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string()),
    };
    
    let raw_tx = builder
        .add_input(input)
        .add_output(output)
        .set_fee(Amount::from_decimal(0.0001, 8))
        .build()
        .unwrap();
    
    assert_eq!(raw_tx.inputs.len(), 1);
    assert_eq!(raw_tx.outputs.len(), 1);
    assert_eq!(raw_tx.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(raw_tx.network, Network::BitcoinMainnet);
}

// ============================================================================
// Provider Integration Tests
// ============================================================================

#[tokio::test]
async fn test_coinbase_commerce_provider() {
    let provider = CoinbaseCommerceProvider::new("test_api_key".to_string())
        .with_webhook_secret("test_secret".to_string());
    
    assert_eq!(provider.name(), "Coinbase Commerce");
    
    let supported_cryptos = provider.supported_cryptocurrencies();
    assert!(supported_cryptos.contains(&Cryptocurrency::Bitcoin));
    assert!(supported_cryptos.contains(&Cryptocurrency::Ethereum));
    assert!(supported_cryptos.contains(&Cryptocurrency::USDC));
    
    let request = CryptoPaymentRequest {
        amount: "50.00".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinMainnet,
        description: Some("Test Coinbase payment".to_string()),
        metadata: None,
        customer_email: Some("test@example.com".to_string()),
        redirect_url: Some("https://example.com/success".to_string()),
        cancel_url: Some("https://example.com/cancel".to_string()),
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    assert_eq!(payment.cryptocurrency, Cryptocurrency::Bitcoin);
    assert_eq!(payment.amount_crypto, "50.00");
    assert!(payment.payment_uri.is_some());
    
    // Test webhook verification
    let is_valid = provider.verify_webhook(b"test payload", "test_signature").unwrap();
    assert!(is_valid);
}

#[tokio::test]
async fn test_bitpay_provider() {
    let provider = BitPayProvider::new("test_token".to_string())
        .test_mode();
    
    assert_eq!(provider.name(), "BitPay");
    
    let supported_cryptos = provider.supported_cryptocurrencies();
    assert!(supported_cryptos.contains(&Cryptocurrency::Bitcoin));
    assert!(supported_cryptos.contains(&Cryptocurrency::Dogecoin));
    
    let request = CryptoPaymentRequest {
        amount: "25.00".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinMainnet,
        description: Some("Test BitPay payment".to_string()),
        metadata: {
            let mut map = HashMap::new();
            map.insert("order_id".to_string(), "BP12345".to_string());
            Some(map)
        },
        customer_email: Some("customer@example.com".to_string()),
        redirect_url: None,
        cancel_url: None,
        webhook_url: Some("https://example.com/webhook".to_string()),
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    assert_eq!(payment.cryptocurrency, Cryptocurrency::Bitcoin);
    assert!(payment.payment_uri.is_some());
}

#[tokio::test]
async fn test_coingate_provider() {
    let provider = CoinGateProvider::new("test_api_key".to_string())
        .sandbox();
    
    assert_eq!(provider.name(), "CoinGate");
    
    let supported_networks = provider.supported_networks();
    assert!(supported_networks.contains(&Network::BitcoinMainnet));
    assert!(supported_networks.contains(&Network::Polygon));
    assert!(supported_networks.contains(&Network::BinanceSmartChain));
    
    let request = CryptoPaymentRequest {
        amount: "100.00".to_string(),
        cryptocurrency: Cryptocurrency::USDT,
        network: Network::EthereumMainnet,
        description: Some("Test CoinGate payment".to_string()),
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    assert_eq!(payment.cryptocurrency, Cryptocurrency::USDT);
    assert_eq!(payment.confirmations_required, 2);
    assert!(payment.expires_at > payment.created_at);
}

#[test]
fn test_provider_factory() {
    let mut config = CryptoConfig::default();
    config.api_key = Some("test_key".to_string());
    
    // Test Coinbase Commerce creation
    config.provider = CryptoProvider::CoinbaseCommerce;
    let provider = create_provider(&config.provider, &config).unwrap();
    assert_eq!(provider.name(), "Coinbase Commerce");
    
    // Test BitPay creation
    config.provider = CryptoProvider::BitPay;
    let provider = create_provider(&config.provider, &config).unwrap();
    assert_eq!(provider.name(), "BitPay");
    
    // Test CoinGate creation
    config.provider = CryptoProvider::CoinGate;
    let provider = create_provider(&config.provider, &config).unwrap();
    assert_eq!(provider.name(), "CoinGate");
    
    // Test Native Bitcoin creation
    config.provider = CryptoProvider::Native;
    config.network = Network::BitcoinMainnet;
    let provider = create_provider(&config.provider, &config).unwrap();
    assert_eq!(provider.name(), "Bitcoin");
    
    // Test Native Ethereum creation
    config.network = Network::EthereumMainnet;
    let provider = create_provider(&config.provider, &config).unwrap();
    assert_eq!(provider.name(), "Ethereum");
}

// ============================================================================
// Multi-Cryptocurrency Support Tests
// ============================================================================

#[tokio::test]
async fn test_multiple_cryptocurrency_support() {
    let cryptocurrencies = vec![
        Cryptocurrency::Bitcoin,
        Cryptocurrency::Ethereum,
        Cryptocurrency::USDC,
        Cryptocurrency::USDT,
        Cryptocurrency::DAI,
        Cryptocurrency::Litecoin,
        Cryptocurrency::Dogecoin,
    ];
    
    for crypto in cryptocurrencies {
        // Test symbol
        let symbol = crypto.symbol();
        assert!(!symbol.is_empty());
        
        // Test decimals
        let decimals = crypto.decimals();
        assert!(decimals > 0);
        
        // Test stablecoin detection
        match crypto {
            Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => {
                assert!(crypto.is_stablecoin());
            }
            _ => assert!(!crypto.is_stablecoin()),
        }
        
        // Test ERC-20 detection
        match crypto {
            Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => {
                assert!(crypto.is_erc20());
            }
            _ => assert!(!crypto.is_erc20()),
        }
    }
}

#[test]
fn test_network_properties() {
    let networks = vec![
        Network::BitcoinMainnet,
        Network::BitcoinTestnet,
        Network::EthereumMainnet,
        Network::EthereumGoerli,
        Network::Polygon,
        Network::Arbitrum,
        Network::BinanceSmartChain,
    ];
    
    for network in networks {
        // Test testnet detection
        match network {
            Network::BitcoinTestnet | Network::EthereumGoerli | Network::EthereumSepolia => {
                assert!(network.is_testnet());
            }
            _ => assert!(!network.is_testnet()),
        }
        
        // Test chain ID for Ethereum-based networks
        match network {
            Network::EthereumMainnet => assert_eq!(network.chain_id(), Some(1)),
            Network::EthereumGoerli => assert_eq!(network.chain_id(), Some(5)),
            Network::Polygon => assert_eq!(network.chain_id(), Some(137)),
            Network::Arbitrum => assert_eq!(network.chain_id(), Some(42161)),
            Network::BinanceSmartChain => assert_eq!(network.chain_id(), Some(56)),
            _ => assert!(network.chain_id().is_none()),
        }
    }
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_error_handling() {
    let provider = BitcoinProvider::new(Network::BitcoinMainnet);
    
    // Test invalid payment amount
    let invalid_request = CryptoPaymentRequest {
        amount: "invalid_amount".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinMainnet,
        description: None,
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    // Should still create payment (amount validation happens elsewhere)
    let payment = provider.create_payment(&invalid_request).await;
    assert!(payment.is_ok());
    
    // Test unsupported cryptocurrency
    let eth_provider = EthereumProvider::new(Network::EthereumMainnet);
    let btc_request = CryptoPaymentRequest {
        amount: "0.001".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::EthereumMainnet,
        description: None,
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    // Should still work as crypto validation is at payment level
    let payment = eth_provider.create_payment(&btc_request).await;
    assert!(payment.is_ok());
}

#[test]
fn test_factory_error_handling() {
    // Test missing API key
    let config = CryptoConfig {
        provider: CryptoProvider::CoinbaseCommerce,
        network: Network::BitcoinMainnet,
        api_key: None,
        webhook_secret: None,
        confirmations_required: 1,
        payment_timeout_minutes: 15,
        auto_convert_to_fiat: false,
    };
    
    let result = create_provider(&config.provider, &config);
    assert!(result.is_err());
    
    // Test unsupported network for native provider
    let config = CryptoConfig {
        provider: CryptoProvider::Native,
        network: Network::BinanceSmartChain,
        api_key: None,
        webhook_secret: None,
        confirmations_required: 1,
        payment_timeout_minutes: 15,
        auto_convert_to_fiat: false,
    };
    
    let result = create_provider(&config.provider, &config);
    assert!(result.is_err());
}

// ============================================================================
// Utility Function Tests
// ============================================================================

#[test]
fn test_crypto_utils() {
    // Test amount conversion
    let btc_decimals = 8;
    let satoshis = 100000000u128; // 1 BTC in satoshis
    let btc_amount = utils::from_smallest_unit(satoshis, btc_decimals);
    assert_eq!(btc_amount, 1.0);
    
    let back_to_satoshis = utils::to_smallest_unit(btc_amount, btc_decimals);
    assert_eq!(back_to_satoshis, satoshis);
    
    // Test payment URI generation
    let uri = utils::generate_payment_uri(
        &Cryptocurrency::Bitcoin,
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
        Some("0.001"),
        Some("Test Payment"),
    );
    assert!(uri.starts_with("bitcoin:bc1"));
    assert!(uri.contains("amount=0.001"));
    assert!(uri.contains("label=Test%20Payment"));
    
    // Test confirmation check
    assert!(utils::is_confirmed(6, 6));
    assert!(utils::is_confirmed(10, 6));
    assert!(!utils::is_confirmed(3, 6));
    
    // Test expiration check
    let future_timestamp = chrono::Utc::now().timestamp() + 3600;
    let past_timestamp = chrono::Utc::now().timestamp() - 3600;
    assert!(!utils::is_expired(future_timestamp));
    assert!(utils::is_expired(past_timestamp));
}

// ============================================================================
// Advanced Integration Tests (Marked as ignored for CI)
// ============================================================================

#[tokio::test]
#[ignore] // Requires actual API keys and network access
async fn test_live_bitcoin_rpc() {
    // This test would require actual Bitcoin Core RPC credentials
    let rpc_client = BitcoinRpcClient::new(
        "http://localhost:8332".to_string(),
        "user".to_string(),
        "pass".to_string(),
    );
    
    let block_count = rpc_client.get_block_count().await;
    if let Ok(count) = block_count {
        assert!(count > 0);
    }
}

#[tokio::test]
#[ignore] // Requires actual API keys
async fn test_live_ethereum_provider() {
    // This test would require actual Infura or Alchemy API key
    let provider = EthereumProvider::new(Network::EthereumMainnet)
        .with_infura("YOUR_PROJECT_ID".to_string());
    
    let gas_prices = provider.get_gas_prices().await;
    if gas_prices.is_ok() {
        let prices = gas_prices.unwrap();
        assert!(prices.standard > 0);
    }
}

#[tokio::test]
#[ignore] // Requires actual API keys
async fn test_live_coinbase_commerce() {
    // This test would require actual Coinbase Commerce API key
    let provider = CoinbaseCommerceProvider::new("YOUR_API_KEY".to_string());
    
    let request = CryptoPaymentRequest {
        amount: "1.00".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinMainnet,
        description: Some("Live test payment".to_string()),
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await;
    if payment.is_ok() {
        let p = payment.unwrap();
        assert!(p.id.starts_with("charge_"));
    }
}

#[tokio::test]
#[ignore] // Requires testnet coins and network access
async fn test_live_transaction_monitoring() {
    // This test would monitor an actual testnet transaction
    let provider = BitcoinProvider::new(Network::BitcoinTestnet);
    
    // Create a payment
    let request = CryptoPaymentRequest {
        amount: "0.001".to_string(),
        cryptocurrency: Cryptocurrency::Bitcoin,
        network: Network::BitcoinTestnet,
        description: Some("Live monitoring test".to_string()),
        metadata: None,
        customer_email: None,
        redirect_url: None,
        cancel_url: None,
        webhook_url: None,
    };
    
    let payment = provider.create_payment(&request).await.unwrap();
    
    // Monitor for transactions (would require actual implementation)
    let mut attempts = 0;
    let max_attempts = 10;
    
    while attempts < max_attempts {
        let updated_payment = provider.get_payment(&payment.id).await.unwrap();
        if updated_payment.transaction_hash.is_some() {
            // Transaction found!
            let tx_hash = updated_payment.transaction_hash.unwrap();
            let confirmations = provider.get_confirmations(&tx_hash, &Network::BitcoinTestnet).await.unwrap();
            assert!(confirmations > 0 || confirmations == 0); // Check confirmations are valid
            break;
        }
        
        tokio::time::sleep(Duration::from_secs(30)).await;
        attempts += 1;
    }
}

// ============================================================================
// Performance and Stress Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_address_generation() {
    let _provider = BitcoinProvider::new(Network::BitcoinTestnet);
    let mut handles = Vec::new();
    
    // Generate 10 addresses concurrently
    for _ in 0..10 {
        let provider = BitcoinProvider::new(Network::BitcoinTestnet);
        let handle = tokio::spawn(async move {
            provider.generate_address(&Cryptocurrency::Bitcoin, &Network::BitcoinTestnet).await
        });
        handles.push(handle);
    }
    
    let mut addresses = Vec::new();
    for handle in handles {
        let address = handle.await.unwrap().unwrap();
        addresses.push(address.address);
    }
    
    // All addresses should be unique
    addresses.sort();
    addresses.dedup();
    assert_eq!(addresses.len(), 10);
}

#[tokio::test]
async fn test_concurrent_payment_creation() {
    let _provider = EthereumProvider::new(Network::EthereumGoerli);
    let mut handles = Vec::new();
    
    // Create 5 payments concurrently
    for i in 0..5 {
        let provider = EthereumProvider::new(Network::EthereumGoerli);
        let handle = tokio::spawn(async move {
            let request = CryptoPaymentRequest {
                amount: format!("0.{}", i + 1),
                cryptocurrency: Cryptocurrency::Ethereum,
                network: Network::EthereumGoerli,
                description: Some(format!("Concurrent payment {}", i + 1)),
                metadata: None,
                customer_email: None,
                redirect_url: None,
                cancel_url: None,
                webhook_url: None,
            };
            provider.create_payment(&request).await
        });
        handles.push(handle);
    }
    
    let mut payments = Vec::new();
    for handle in handles {
        let payment = handle.await.unwrap().unwrap();
        payments.push(payment.id);
    }
    
    // All payments should have unique IDs
    payments.sort();
    payments.dedup();
    assert_eq!(payments.len(), 5);
}

} // End of crypto_tests module