//! MediToken - A Rust implementation of an ERC20-like token for healthcare applications
//! 
//! This crate provides a complete implementation of the MediToken, including:
//! - Token contract logic
//! - Deployment utilities  
//! - Testing framework
//! - Multi-chain support

// NEAR token module - always included
pub mod near_token;
pub use near_token::MediTokenNEP141;

// For non-WASM builds, include EVM/blockchain modules
#[cfg(not(target_arch = "wasm32"))]
pub mod token;
#[cfg(not(target_arch = "wasm32"))]
pub mod error;
#[cfg(not(target_arch = "wasm32"))]
pub mod utils;
#[cfg(not(target_arch = "wasm32"))]
pub mod deployment;
#[cfg(not(target_arch = "wasm32"))]
pub mod abi;

#[cfg(not(target_arch = "wasm32"))]
pub use token::MediToken;
#[cfg(not(target_arch = "wasm32"))]
pub use error::MediTokenError;
#[cfg(not(target_arch = "wasm32"))]
pub use utils::Utils;