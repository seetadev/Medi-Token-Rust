
use crate::error::{MediTokenError, Result};
use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha3::{Digest, Keccak256};

/// Utility functions for MediToken operations
pub struct Utils;

impl Utils {
    /// Validates an Ethereum address format
    pub fn is_valid_address(address: &str) -> bool {
        if !address.starts_with("0x") {
            return false;
        }
        
        let hex_part = &address[2..];
        hex_part.len() == 40 && hex_part.chars().all(|c| c.is_ascii_hexdigit())
    }
    
    /// Generates a random Ethereum address for testing
    pub fn generate_random_address() -> String {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let public_key_bytes = verifying_key.to_encoded_point(false);
        
        let mut hasher = Keccak256::new();
        hasher.update(&public_key_bytes.as_bytes()[1..]); 
        let hash = hasher.finalize();
        
        format!("0x{}", hex::encode(&hash[12..]))
    }
    
    /// Converts wei to ether (considering 18 decimals)
    pub fn wei_to_ether(wei: u64) -> f64 {
        wei as f64 / 1e18
    }
    
    /// Converts ether to wei (considering 18 decimals)
    pub fn ether_to_wei(ether: f64) -> u64 {
        (ether * 1e18) as u64
    }
    
    /// Calculates Keccak256 hash of input
    pub fn keccak256(input: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(input);
        hasher.finalize().into()
    }
    
    /// Validates that an amount doesn't cause overflow
    pub fn check_overflow(a: u64, b: u64) -> Result<u64> {
        a.checked_add(b)
            .ok_or(MediTokenError::ArithmeticOverflow)
    }
    
    /// Validates that an amount doesn't cause underflow
    pub fn check_underflow(a: u64, b: u64) -> Result<u64> {
        a.checked_sub(b)
            .ok_or(MediTokenError::ArithmeticUnderflow)
    }
}
