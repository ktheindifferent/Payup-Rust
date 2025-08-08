use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::{Cryptocurrency, Network, AddressType};
use sha2::{Sha256, Digest};

/// Wallet address with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
    pub address_type: AddressType,
    pub label: Option<String>,
    pub memo: Option<String>,
}

impl WalletAddress {
    pub fn new(address: String, cryptocurrency: Cryptocurrency, network: Network) -> Self {
        let address_type = detect_address_type(&address, &cryptocurrency);
        WalletAddress {
            address,
            cryptocurrency,
            network,
            address_type,
            label: None,
            memo: None,
        }
    }

    /// Validate the wallet address format
    pub fn validate(&self) -> Result<bool> {
        match self.cryptocurrency {
            Cryptocurrency::Bitcoin | Cryptocurrency::BitcoinCash | Cryptocurrency::Litecoin => {
                validate_bitcoin_address(&self.address, &self.network)
            }
            Cryptocurrency::Ethereum | Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => {
                validate_ethereum_address(&self.address)
            }
            Cryptocurrency::Dogecoin => validate_dogecoin_address(&self.address),
            Cryptocurrency::Solana => validate_solana_address(&self.address),
            _ => Ok(true), // Default to valid for unsupported cryptocurrencies
        }
    }

    /// Generate a payment URI for QR codes
    pub fn to_payment_uri(&self, amount: Option<&str>) -> String {
        super::utils::generate_payment_uri(
            &self.cryptocurrency,
            &self.address,
            amount,
            self.label.as_deref(),
        )
    }

    /// Check if address is for testnet
    pub fn is_testnet(&self) -> bool {
        self.network.is_testnet()
    }
}

/// Detect address type based on format
fn detect_address_type(address: &str, cryptocurrency: &Cryptocurrency) -> AddressType {
    match cryptocurrency {
        Cryptocurrency::Bitcoin | Cryptocurrency::BitcoinCash | Cryptocurrency::Litecoin => {
            detect_bitcoin_address_type(address)
        }
        Cryptocurrency::Ethereum | Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => {
            if address.len() == 42 && address.starts_with("0x") {
                // Could be either EOA or Contract, default to EOA
                AddressType::EOA
            } else {
                AddressType::Unknown
            }
        }
        _ => AddressType::Unknown,
    }
}

/// Detect Bitcoin address type
fn detect_bitcoin_address_type(address: &str) -> AddressType {
    if address.is_empty() {
        return AddressType::Unknown;
    }

    match &address[0..1] {
        "1" => AddressType::P2PKH,        // Legacy
        "3" => AddressType::P2SH,         // SegWit compatible
        "b" if address.starts_with("bc1q") => AddressType::P2WPKH,  // Native SegWit
        "b" if address.starts_with("bc1p") => AddressType::P2TR,     // Taproot
        _ => AddressType::Unknown,
    }
}

/// Validate Bitcoin address format
fn validate_bitcoin_address(address: &str, network: &Network) -> Result<bool> {
    if address.is_empty() {
        return Ok(false);
    }

    // Basic format validation
    let valid = match network {
        Network::BitcoinMainnet => {
            // P2PKH (starts with 1)
            (address.starts_with('1') && address.len() >= 26 && address.len() <= 35) ||
            // P2SH (starts with 3)
            (address.starts_with('3') && address.len() >= 26 && address.len() <= 35) ||
            // Bech32 (starts with bc1)
            (address.starts_with("bc1") && address.len() >= 42)
        }
        Network::BitcoinTestnet => {
            // Testnet P2PKH (starts with m or n)
            (address.starts_with('m') || address.starts_with('n')) ||
            // Testnet P2SH (starts with 2)
            address.starts_with('2') ||
            // Testnet Bech32 (starts with tb1)
            address.starts_with("tb1")
        }
        _ => false,
    };

    Ok(valid)
}

/// Validate Ethereum address format
fn validate_ethereum_address(address: &str) -> Result<bool> {
    // Check basic format
    if !address.starts_with("0x") || address.len() != 42 {
        return Ok(false);
    }

    // Check if all characters after 0x are hexadecimal
    let hex_part = &address[2..];
    let valid = hex_part.chars().all(|c| c.is_ascii_hexdigit());

    // TODO: Implement EIP-55 checksum validation
    Ok(valid)
}

/// Validate Dogecoin address format
fn validate_dogecoin_address(address: &str) -> Result<bool> {
    // Dogecoin addresses start with D
    Ok(address.starts_with('D') && address.len() >= 26 && address.len() <= 35)
}

/// Validate Solana address format
fn validate_solana_address(address: &str) -> Result<bool> {
    // Solana addresses are base58 encoded and typically 32-44 characters
    Ok(address.len() >= 32 && address.len() <= 44 && is_base58(address))
}

/// Check if string is valid base58
fn is_base58(s: &str) -> bool {
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    s.chars().all(|c| BASE58_ALPHABET.contains(c))
}

/// HD Wallet for deterministic key generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HDWallet {
    pub mnemonic_encrypted: Vec<u8>,
    pub derivation_path: String,
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
}

impl HDWallet {
    /// Standard derivation paths for different cryptocurrencies
    pub fn default_derivation_path(cryptocurrency: &Cryptocurrency) -> String {
        match cryptocurrency {
            Cryptocurrency::Bitcoin => "m/84'/0'/0'/0/0".to_string(),      // Native SegWit
            Cryptocurrency::Ethereum => "m/44'/60'/0'/0/0".to_string(),
            Cryptocurrency::Litecoin => "m/84'/2'/0'/0/0".to_string(),
            Cryptocurrency::Dogecoin => "m/44'/3'/0'/0/0".to_string(),
            Cryptocurrency::BitcoinCash => "m/44'/145'/0'/0/0".to_string(),
            _ => "m/44'/0'/0'/0/0".to_string(),
        }
    }
}

/// Wallet balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub address: String,
    pub cryptocurrency: Cryptocurrency,
    pub confirmed_balance: String,
    pub unconfirmed_balance: String,
    pub total_received: String,
    pub total_sent: String,
    pub transaction_count: u32,
    pub last_updated: i64,
}

/// Multi-signature wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    pub address: String,
    pub cryptocurrency: Cryptocurrency,
    pub network: Network,
    pub required_signatures: u32,
    pub total_signers: u32,
    pub signers: Vec<WalletAddress>,
    pub pending_transactions: Vec<PendingMultiSigTx>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMultiSigTx {
    pub tx_id: String,
    pub amount: String,
    pub destination: String,
    pub signatures_collected: u32,
    pub signers_signed: Vec<String>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

/// Watch-only wallet for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchOnlyWallet {
    pub addresses: Vec<WalletAddress>,
    pub name: String,
    pub description: Option<String>,
    pub total_balance: Option<String>,
    pub last_sync: Option<i64>,
}

impl WatchOnlyWallet {
    pub fn new(name: String) -> Self {
        WatchOnlyWallet {
            addresses: Vec::new(),
            name,
            description: None,
            total_balance: None,
            last_sync: None,
        }
    }

    pub fn add_address(&mut self, address: WalletAddress) -> Result<()> {
        if address.validate()? {
            self.addresses.push(address);
            Ok(())
        } else {
            Err(PayupError::ValidationError("Invalid wallet address".to_string()))
        }
    }

    pub fn remove_address(&mut self, address: &str) -> bool {
        let len_before = self.addresses.len();
        self.addresses.retain(|a| a.address != address);
        self.addresses.len() < len_before
    }
}

/// Generate a new wallet address (mock implementation)
pub fn generate_wallet_address(
    cryptocurrency: &Cryptocurrency,
    network: &Network,
) -> Result<WalletAddress> {
    // This is a mock implementation
    // In production, use proper cryptographic libraries
    let random_bytes: [u8; 32] = rand::random();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    let hash = hasher.finalize();
    
    let address = match cryptocurrency {
        Cryptocurrency::Bitcoin => {
            if network.is_testnet() {
                format!("tb1q{}", hex::encode(&hash[0..20]))
            } else {
                format!("bc1q{}", hex::encode(&hash[0..20]))
            }
        }
        Cryptocurrency::Ethereum | Cryptocurrency::USDC | Cryptocurrency::USDT | Cryptocurrency::DAI => {
            format!("0x{}", hex::encode(&hash[0..20]))
        }
        _ => {
            format!("{}", hex::encode(&hash[0..20]))
        }
    };

    Ok(WalletAddress::new(address, cryptocurrency.clone(), network.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoin_address_validation() {
        // Mainnet P2PKH
        let addr = WalletAddress::new(
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            Cryptocurrency::Bitcoin,
            Network::BitcoinMainnet,
        );
        assert!(addr.validate().unwrap());

        // Mainnet P2SH
        let addr = WalletAddress::new(
            "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy".to_string(),
            Cryptocurrency::Bitcoin,
            Network::BitcoinMainnet,
        );
        assert!(addr.validate().unwrap());

        // Mainnet Bech32
        let addr = WalletAddress::new(
            "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(),
            Cryptocurrency::Bitcoin,
            Network::BitcoinMainnet,
        );
        assert!(addr.validate().unwrap());
    }

    #[test]
    fn test_ethereum_address_validation() {
        let addr = WalletAddress::new(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string(),
            Cryptocurrency::Ethereum,
            Network::EthereumMainnet,
        );
        assert!(addr.validate().unwrap());

        // Invalid address
        let addr = WalletAddress::new(
            "invalid_address".to_string(),
            Cryptocurrency::Ethereum,
            Network::EthereumMainnet,
        );
        assert!(!addr.validate().unwrap());
    }

    #[test]
    fn test_address_type_detection() {
        assert_eq!(
            detect_bitcoin_address_type("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
            AddressType::P2PKH
        );
        
        assert_eq!(
            detect_bitcoin_address_type("3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy"),
            AddressType::P2SH
        );
        
        assert_eq!(
            detect_bitcoin_address_type("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"),
            AddressType::P2WPKH
        );
    }

    #[test]
    fn test_watch_only_wallet() {
        let mut wallet = WatchOnlyWallet::new("My Portfolio".to_string());
        
        let addr = WalletAddress::new(
            "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(),
            Cryptocurrency::Bitcoin,
            Network::BitcoinMainnet,
        );
        
        wallet.add_address(addr).unwrap();
        assert_eq!(wallet.addresses.len(), 1);
        
        wallet.remove_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
        assert_eq!(wallet.addresses.len(), 0);
    }
}