use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a cryptographic hash (transaction ID, block hash, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Hash(pub String);

impl Hash {
    pub fn new(hash: String) -> Self {
        Hash(hash.to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_valid(&self) -> bool {
        // Basic validation - can be extended per blockchain
        !self.0.is_empty() && self.0.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Amount representation with precision handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Amount {
    /// Value in the smallest unit (satoshis, wei, etc.)
    pub value: u128,
    /// Number of decimal places
    pub decimals: u8,
}

impl Amount {
    pub fn new(value: u128, decimals: u8) -> Self {
        Amount { value, decimals }
    }

    pub fn from_decimal(decimal: f64, decimals: u8) -> Self {
        let value = (decimal * 10_f64.powi(decimals as i32)) as u128;
        Amount { value, decimals }
    }

    pub fn to_decimal(&self) -> f64 {
        self.value as f64 / 10_f64.powi(self.decimals as i32)
    }

    pub fn to_string(&self) -> String {
        format!("{:.prec$}", self.to_decimal(), prec = self.decimals as usize)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn add(&self, other: &Amount) -> Result<Amount, String> {
        if self.decimals != other.decimals {
            return Err("Cannot add amounts with different decimals".to_string());
        }
        Ok(Amount::new(
            self.value.saturating_add(other.value),
            self.decimals,
        ))
    }

    pub fn subtract(&self, other: &Amount) -> Result<Amount, String> {
        if self.decimals != other.decimals {
            return Err("Cannot subtract amounts with different decimals".to_string());
        }
        if self.value < other.value {
            return Err("Insufficient amount".to_string());
        }
        Ok(Amount::new(
            self.value - other.value,
            self.decimals,
        ))
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub hash: Hash,
    pub timestamp: i64,
    pub confirmations: u32,
}

/// UTXO (Unspent Transaction Output) for Bitcoin-like blockchains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    pub tx_hash: Hash,
    pub output_index: u32,
    pub amount: Amount,
    pub script_pubkey: String,
    pub confirmations: u32,
}

/// Smart contract address for Ethereum-like blockchains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractAddress {
    pub address: String,
    pub chain_id: u64,
}

impl ContractAddress {
    pub fn new(address: String, chain_id: u64) -> Self {
        ContractAddress {
            address: address.to_lowercase(),
            chain_id,
        }
    }

    pub fn is_valid(&self) -> bool {
        // Basic Ethereum address validation
        self.address.starts_with("0x") && 
        self.address.len() == 42 &&
        self.address[2..].chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// Gas settings for Ethereum transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasSettings {
    pub gas_limit: u64,
    pub gas_price: Option<u64>,     // Legacy transactions
    pub max_fee_per_gas: Option<u64>, // EIP-1559
    pub max_priority_fee_per_gas: Option<u64>, // EIP-1559
}

impl Default for GasSettings {
    fn default() -> Self {
        GasSettings {
            gas_limit: 21000, // Basic transfer
            gas_price: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        }
    }
}

/// Memo/Tag for certain cryptocurrencies (XRP, XLM, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMemo {
    pub memo_type: MemoType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoType {
    Text,
    Id,
    Hash,
    Return,
}

/// Lightning Network invoice for fast Bitcoin payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningInvoice {
    pub bolt11: String,
    pub payment_hash: Hash,
    pub amount_msat: u64,
    pub description: Option<String>,
    pub expires_at: i64,
}

impl LightningInvoice {
    pub fn amount_sats(&self) -> u64 {
        self.amount_msat / 1000
    }

    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        now > self.expires_at
    }
}

/// Multi-signature wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigConfig {
    pub required_signatures: u32,
    pub total_signers: u32,
    pub signers: Vec<String>,
}

impl MultiSigConfig {
    pub fn is_valid(&self) -> bool {
        self.required_signatures > 0 &&
        self.required_signatures <= self.total_signers &&
        self.signers.len() == self.total_signers as usize
    }
}

/// Token information for ERC-20, BEP-20, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub contract_address: ContractAddress,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: Option<Amount>,
}

/// Transaction input/output for detailed transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_tx_hash: Hash,
    pub previous_output_index: u32,
    pub script_sig: Option<String>,
    pub witness: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub amount: Amount,
    pub script_pubkey: String,
    pub address: Option<String>,
}

/// Detailed transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub fee: Amount,
    pub size: u32,
    pub weight: Option<u32>,
    pub locktime: Option<u32>,
}

/// Address type for different blockchain formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddressType {
    // Bitcoin address types
    P2PKH,          // Pay to Public Key Hash (Legacy)
    P2SH,           // Pay to Script Hash
    P2WPKH,         // Pay to Witness Public Key Hash (Native SegWit)
    P2WSH,          // Pay to Witness Script Hash
    P2TR,           // Pay to Taproot
    
    // Ethereum address types
    EOA,            // Externally Owned Account
    Contract,       // Smart Contract
    
    // Other
    Unknown,
}

/// Blockchain metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainMetrics {
    pub block_height: u64,
    pub block_time_seconds: u32,
    pub difficulty: Option<f64>,
    pub hash_rate: Option<f64>,
    pub pending_transactions: u32,
    pub average_fee: Amount,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_validation() {
        let valid_hash = Hash::new("abc123def456".to_string());
        assert!(valid_hash.is_valid());

        let invalid_hash = Hash::new("xyz!@#".to_string());
        assert!(!invalid_hash.is_valid());
    }

    #[test]
    fn test_amount_operations() {
        let amt1 = Amount::from_decimal(1.5, 8);
        let amt2 = Amount::from_decimal(0.5, 8);

        let sum = amt1.add(&amt2).unwrap();
        assert_eq!(sum.to_decimal(), 2.0);

        let diff = amt1.subtract(&amt2).unwrap();
        assert_eq!(diff.to_decimal(), 1.0);
    }

    #[test]
    fn test_contract_address_validation() {
        let valid_addr = ContractAddress::new(
            "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7".to_string(),
            1,
        );
        assert!(valid_addr.is_valid());

        let invalid_addr = ContractAddress::new("invalid".to_string(), 1);
        assert!(!invalid_addr.is_valid());
    }

    #[test]
    fn test_lightning_invoice() {
        let invoice = LightningInvoice {
            bolt11: "lnbc100n1...".to_string(),
            payment_hash: Hash::new("abc123".to_string()),
            amount_msat: 100000,
            description: Some("Test payment".to_string()),
            expires_at: 9999999999,
        };

        assert_eq!(invoice.amount_sats(), 100);
        assert!(!invoice.is_expired());
    }
}