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
        use Cryptocurrency::*;
        
        match self.cryptocurrency {
            Bitcoin | BitcoinCash | Litecoin => 
                validate_bitcoin_like_address(&self.address, &self.network),
            
            Ethereum | USDC | USDT | DAI => 
                validate_ethereum_address(&self.address),
            
            Dogecoin => 
                validate_dogecoin_address(&self.address),
            
            Solana => 
                validate_solana_address(&self.address),
            
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
    use Cryptocurrency::*;
    
    match cryptocurrency {
        Bitcoin | BitcoinCash | Litecoin => 
            detect_bitcoin_address_type(address),
        
        Ethereum | USDC | USDT | DAI => 
            detect_ethereum_address_type(address),
        
        _ => AddressType::Unknown,
    }
}

/// Detect Ethereum address type
fn detect_ethereum_address_type(address: &str) -> AddressType {
    if is_valid_ethereum_format(address) {
        AddressType::EOA  // Could be either EOA or Contract, default to EOA
    } else {
        AddressType::Unknown
    }
}

/// Check if address has valid Ethereum format
fn is_valid_ethereum_format(address: &str) -> bool {
    address.len() == 42 && address.starts_with("0x")
}

/// Detect Bitcoin address type
fn detect_bitcoin_address_type(address: &str) -> AddressType {
    if address.is_empty() {
        return AddressType::Unknown;
    }

    let first_char = &address[0..1];
    
    match first_char {
        "1" => AddressType::P2PKH,        // Legacy
        "3" => AddressType::P2SH,         // SegWit compatible
        "b" => detect_bech32_address_type(address),
        _ => AddressType::Unknown,
    }
}

/// Detect specific Bech32 address type
fn detect_bech32_address_type(address: &str) -> AddressType {
    if address.starts_with("bc1q") {
        AddressType::P2WPKH  // Native SegWit
    } else if address.starts_with("bc1p") {
        AddressType::P2TR    // Taproot
    } else {
        AddressType::Unknown
    }
}

/// Validate Bitcoin-like address format
fn validate_bitcoin_like_address(address: &str, network: &Network) -> Result<bool> {
    if address.is_empty() {
        return Ok(false);
    }

    let valid = match network {
        Network::BitcoinMainnet => validate_bitcoin_mainnet_address(address),
        Network::BitcoinTestnet => validate_bitcoin_testnet_address(address),
        _ => false,
    };

    Ok(valid)
}

/// Validate Bitcoin mainnet address
fn validate_bitcoin_mainnet_address(address: &str) -> bool {
    is_valid_p2pkh_mainnet(address) ||
    is_valid_p2sh_mainnet(address) ||
    is_valid_bech32_mainnet(address)
}

/// Validate Bitcoin testnet address
fn validate_bitcoin_testnet_address(address: &str) -> bool {
    is_valid_p2pkh_testnet(address) ||
    is_valid_p2sh_testnet(address) ||
    is_valid_bech32_testnet(address)
}

/// Check if address is valid P2PKH mainnet
fn is_valid_p2pkh_mainnet(address: &str) -> bool {
    address.starts_with('1') && is_valid_base58_length(address)
}

/// Check if address is valid P2SH mainnet
fn is_valid_p2sh_mainnet(address: &str) -> bool {
    address.starts_with('3') && is_valid_base58_length(address)
}

/// Check if address is valid Bech32 mainnet
fn is_valid_bech32_mainnet(address: &str) -> bool {
    address.starts_with("bc1") && address.len() >= 42
}

/// Check if address is valid P2PKH testnet
fn is_valid_p2pkh_testnet(address: &str) -> bool {
    (address.starts_with('m') || address.starts_with('n')) && is_valid_base58_length(address)
}

/// Check if address is valid P2SH testnet
fn is_valid_p2sh_testnet(address: &str) -> bool {
    address.starts_with('2') && is_valid_base58_length(address)
}

/// Check if address is valid Bech32 testnet
fn is_valid_bech32_testnet(address: &str) -> bool {
    address.starts_with("tb1") && address.len() >= 42
}

/// Check if address has valid base58 length
fn is_valid_base58_length(address: &str) -> bool {
    address.len() >= 26 && address.len() <= 35
}

/// Validate Ethereum address format
fn validate_ethereum_address(address: &str) -> Result<bool> {
    if !is_valid_ethereum_format(address) {
        return Ok(false);
    }

    let hex_part = &address[2..];
    let has_valid_hex_chars = hex_part.chars().all(|c| c.is_ascii_hexdigit());

    // TODO: Implement EIP-55 checksum validation
    Ok(has_valid_hex_chars)
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
    let hash = generate_address_hash();
    let address = format_address_for_cryptocurrency(cryptocurrency, network, &hash);
    
    Ok(WalletAddress::new(address, cryptocurrency.clone(), network.clone()))
}

/// Generate hash for address creation
fn generate_address_hash() -> Vec<u8> {
    let random_bytes: [u8; 32] = rand::random();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    hasher.finalize().to_vec()
}

/// Format address based on cryptocurrency type
fn format_address_for_cryptocurrency(
    cryptocurrency: &Cryptocurrency,
    network: &Network,
    hash: &[u8],
) -> String {
    use Cryptocurrency::*;
    
    let address_bytes = &hash[0..20];
    
    match cryptocurrency {
        Bitcoin => format_bitcoin_address(network, address_bytes),
        Ethereum | USDC | USDT | DAI => format_ethereum_address(address_bytes),
        _ => hex::encode(address_bytes),
    }
}

/// Format Bitcoin address
fn format_bitcoin_address(network: &Network, address_bytes: &[u8]) -> String {
    let prefix = if network.is_testnet() { "tb1q" } else { "bc1q" };
    format!("{}{}", prefix, hex::encode(address_bytes))
}

/// Format Ethereum address
fn format_ethereum_address(address_bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(address_bytes))
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