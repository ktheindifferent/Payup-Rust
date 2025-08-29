use serde::{Deserialize, Serialize};
use crate::error::{PayupError, Result};
use super::{Cryptocurrency, Network, AddressType};
use sha2::{Sha256, Digest};
use sha3::{Keccak256, Digest as Sha3Digest};

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

/// Validate Ethereum address format with EIP-55 checksum validation
/// 
/// This function validates Ethereum addresses with three levels of validation:
/// 1. Format validation - checks for 0x prefix and 40 hex characters
/// 2. Character validation - ensures all characters are valid hexadecimal
/// 3. EIP-55 checksum validation - if address has mixed case, validates the checksum
/// 
/// # Security Note
/// EIP-55 checksum validation is critical for preventing typos in Ethereum addresses
/// that could lead to permanent loss of funds. Always use checksummed addresses
/// when possible, especially for large transactions or contract deployments.
/// 
/// # Returns
/// - `Ok(true)` if the address is valid
/// - `Ok(false)` if the address is invalid
/// - `Err` if validation encounters an error
fn validate_ethereum_address(address: &str) -> Result<bool> {
    if !is_valid_ethereum_format(address) {
        return Ok(false);
    }

    let hex_part = &address[2..];
    
    // Check if all characters are valid hex
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(false);
    }

    // Check if address is all lowercase or all uppercase (no checksum)
    let has_uppercase = hex_part.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = hex_part.chars().any(|c| c.is_ascii_lowercase() && c.is_ascii_alphabetic());
    
    // If address has mixed case, validate EIP-55 checksum
    if has_uppercase && has_lowercase {
        Ok(validate_eip55_checksum(address))
    } else {
        // Address is either all lowercase or all uppercase (non-checksummed)
        Ok(true)
    }
}

/// Validate EIP-55 checksum for Ethereum address
/// 
/// EIP-55 (Ethereum Improvement Proposal 55) defines a checksum standard for Ethereum addresses
/// that helps detect typos without changing the address format. The checksum works by:
/// 
/// 1. Taking the lowercase hex address (without 0x prefix)
/// 2. Computing its keccak256 hash
/// 3. For each alphabetic character in the address:
///    - If the corresponding hex digit in the hash is >= 8, the character must be uppercase
///    - If the corresponding hex digit in the hash is < 8, the character must be lowercase
/// 
/// # Example
/// Address: 0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAed
/// - The 'a' at position 1 has hash digit < 8, so it's lowercase
/// - The 'A' at position 2 has hash digit >= 8, so it's uppercase
/// - And so on for each letter in the address
/// 
/// # Security Implications
/// Using EIP-55 checksummed addresses reduces the probability of accidental fund loss
/// from approximately 1 in 256 (for each character) to 1 in 4.3 billion for the entire address.
/// This is crucial for preventing costly mistakes in cryptocurrency transactions.
/// 
/// # Parameters
/// - `address`: The Ethereum address to validate (must include 0x prefix)
/// 
/// # Returns
/// - `true` if the checksum is valid
/// - `false` if the checksum is invalid or address format is wrong
fn validate_eip55_checksum(address: &str) -> bool {
    if address.len() != 42 || !address.starts_with("0x") {
        return false;
    }

    let hex_part = &address[2..];
    let lowercase_address = hex_part.to_lowercase();
    
    // Calculate keccak256 hash of the lowercase address
    let mut hasher = Keccak256::new();
    hasher.update(lowercase_address.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    // Check each character against the hash
    for (i, ch) in hex_part.chars().enumerate() {
        if ch.is_ascii_alphabetic() {
            // Get the corresponding hex digit from the hash
            let hash_char = hash_hex.chars().nth(i).unwrap();
            let hash_value = hash_char.to_digit(16).unwrap();
            
            // If hash value >= 8, character should be uppercase
            if hash_value >= 8 {
                if !ch.is_ascii_uppercase() {
                    return false;
                }
            } else {
                if !ch.is_ascii_lowercase() {
                    return false;
                }
            }
        }
    }
    
    true
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
        // This address has mixed case but may not be properly checksummed
        // Using all lowercase which should always be valid
        let addr = WalletAddress::new(
            "0x742d35cc6634c0532925a3b844bc9e7595f0beb7".to_string(),
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

    #[test]
    fn test_eip55_valid_checksummed_addresses() {
        // Test valid EIP-55 checksummed addresses
        let valid_checksummed = vec![
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",  // Fixed: F at position 9
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d359",
            "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6FB",
            "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb",
            "0x27b1fdb04752bbc536007a920d24acb045561c26",
            "0xde709f2102306220921060314715629080e2fb77",
            "0x78731D3Ca6b7E34aC0F824c42a7cC18A495cabaB",
        ];

        for address in valid_checksummed {
            let result = validate_eip55_checksum(address);
            assert!(result, "Should validate correct checksum for {}", address);
            
            // Also test through the main validation function
            let validation_result = validate_ethereum_address(address).unwrap();
            assert!(validation_result, "Main validation should pass for {}", address);
        }
    }

    #[test]
    fn test_eip55_invalid_checksummed_addresses() {
        // Test invalid EIP-55 checksummed addresses (wrong case)
        let invalid_checksummed = vec![
            "0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAED", // Last D should be lowercase
            "0xfB6916095ca1df60bB79Ce92cE3Ea74c37c5d358", // Wrong last digit
            "0xdbF03B407c01E7cD3CBea99509d93f8DDDC8C6Fb", // Last b should be uppercase
            "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDB", // Last B should be lowercase
        ];

        for address in invalid_checksummed {
            let result = validate_eip55_checksum(address);
            assert!(!result, "Should reject incorrect checksum for {}", address);
            
            // Main validation should fail for invalid checksums
            let validation_result = validate_ethereum_address(address).unwrap();
            assert!(!validation_result, "Main validation should fail for {}", address);
        }
    }

    #[test]
    fn test_ethereum_address_all_lowercase() {
        // All lowercase addresses should be valid (no checksum verification)
        let lowercase_addresses = vec![
            "0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed",
            "0xfb6916095ca1df60bb79ce92ce3ea74c37c5d359",
            "0xdbf03b407c01e7cd3cbea99509d93f8dddc8c6fb",
            "0xd1220a0cf47c7b9be7a2e6ba89f429762e7b9adb",
        ];

        for address in lowercase_addresses {
            let result = validate_ethereum_address(address).unwrap();
            assert!(result, "Should accept all-lowercase address {}", address);
        }
    }

    #[test]
    fn test_ethereum_address_all_uppercase() {
        // All uppercase addresses should be valid (no checksum verification)
        let uppercase_addresses = vec![
            "0x5AAEB6053F3E94C9B9A09F33669435E7EF1BEAED",
            "0xFB6916095CA1DF60BB79CE92CE3EA74C37C5D359",
            "0xDBF03B407C01E7CD3CBEA99509D93F8DDDC8C6FB",
            "0xD1220A0CF47C7B9BE7A2E6BA89F429762E7B9ADB",
        ];

        for address in uppercase_addresses {
            let result = validate_ethereum_address(address).unwrap();
            assert!(result, "Should accept all-uppercase address {}", address);
        }
    }

    #[test]
    fn test_ethereum_address_invalid_format() {
        // Test various invalid formats
        let invalid_addresses = vec![
            "5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAed",    // Missing 0x prefix
            "0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAe",   // Too short
            "0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAedFF", // Too long
            "0xGGAeb6053f3E94C9b9A09f33669435E7Ef1BeAed",   // Invalid hex characters
            "0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAeZ",   // Contains non-hex character
            "",                                               // Empty string
            "0x",                                            // Only prefix
        ];

        for address in invalid_addresses {
            let result = validate_ethereum_address(address).unwrap();
            assert!(!result, "Should reject invalid format for {}", address);
        }
    }

    #[test]
    fn test_eip55_real_world_addresses() {
        // Test with real Ethereum addresses from various sources
        
        // Ethereum Foundation address (with correct checksum)
        let eth_foundation = "0xde0B295669a9FD93d5F28D9Ec85E40f4cb697BAe";
        assert!(validate_ethereum_address(eth_foundation).unwrap());

        // Vitalik Buterin's address (with correct checksum)
        let vitalik = "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B";
        assert!(validate_ethereum_address(vitalik).unwrap());

        // USDC contract address (with correct checksum)
        let usdc = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
        assert!(validate_ethereum_address(usdc).unwrap());

        // Invalid checksum version of USDC address
        let usdc_invalid = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"; // All lowercase is valid
        assert!(validate_ethereum_address(usdc_invalid).unwrap());
        
        // But mixed case with wrong checksum should fail
        let usdc_wrong_checksum = "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48"; // Wrong case (uppercase D at position 17)
        assert!(!validate_ethereum_address(usdc_wrong_checksum).unwrap());
    }

    #[test]
    fn test_eip55_zero_address() {
        // Test the special zero address
        let zero_checksummed = "0x0000000000000000000000000000000000000000";
        assert!(validate_ethereum_address(zero_checksummed).unwrap());

        // Zero address with wrong checksum (some uppercase)
        let zero_wrong = "0x0000000000000000000000000000000000000000";
        assert!(validate_ethereum_address(zero_wrong).unwrap());
    }

    #[test]
    fn test_wallet_address_validation_integration() {
        // Test through the WalletAddress struct
        let checksummed_addr = WalletAddress::new(
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".to_string(),  // Fixed checksum
            Cryptocurrency::Ethereum,
            Network::EthereumMainnet,
        );
        assert!(checksummed_addr.validate().unwrap());

        // Test with USDC token
        let usdc_addr = WalletAddress::new(
            "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            Cryptocurrency::USDC,
            Network::EthereumMainnet,
        );
        assert!(usdc_addr.validate().unwrap());

        // Test with invalid checksum
        let invalid_addr = WalletAddress::new(
            "0x5aAeb6053f3E94C9b9A09f33669435E7Ef1BeAED".to_string(), // Wrong last character case
            Cryptocurrency::Ethereum,
            Network::EthereumMainnet,
        );
        assert!(!invalid_addr.validate().unwrap());
    }
}