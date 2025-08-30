//! MediToken - A Rust implementation of an ERC20-like token for healthcare applications
//! 
//! This crate provides a complete implementation of the MediToken, including:
//! - Token contract logic
//! - Deployment utilities  
//! - Testing framework
//! - Multi-chain support

pub mod token;
pub mod error;
pub mod utils;
pub mod deployment;
pub mod abi;
pub mod near_token;

pub use token::MediToken;
pub use error::MediTokenError;