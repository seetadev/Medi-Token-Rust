
use thiserror::Error;

/// Custom error types for MediToken operations
#[derive(Error, Debug)]
pub enum MediTokenError {
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("Insufficient allowance: spender {spender}, allowance {allowance}, needed {needed}")]
    InsufficientAllowance { spender: String, allowance: u64, needed: u64 },
    
    #[error("Invalid address: {address}")]
    InvalidAddress { address: String },
    
    #[error("Transfer to zero address")]
    TransferToZeroAddress,
    
    #[error("Transfer from zero address")]
    TransferFromZeroAddress,
    
    #[error("Approve to zero address")]
    ApproveToZeroAddress,
    
    #[error("Approve from zero address")]
    ApproveFromZeroAddress,
    
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[error("Arithmetic underflow")]
    ArithmeticUnderflow,
    
    #[error("Contract error: {message}")]
    ContractError { message: String },
    
    #[error("Network error: {message}")]
    NetworkError { message: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, MediTokenError>;