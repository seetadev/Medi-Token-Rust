// Author Name - Ritankar Saha
// Email - ritankar.saha786@gmail.com

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::{MediTokenError, Result};

/// Events emitted by the MediToken contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediTokenEvent {
    Transfer {
        from: String,
        to: String,
        value: u64,
    },
    Approval {
        owner: String,
        spender: String,
        value: u64,
    },
}

/// Core MediToken implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediToken {
    /// Token name
    pub name: String,
    /// Token symbol  
    pub symbol: String,
    /// Number of decimals
    pub decimals: u8,
    /// Total supply of tokens
    pub total_supply: u64,
    /// Balance mapping: address -> balance
    pub balances: HashMap<String, u64>,
    /// Allowance mapping: owner -> spender -> amount
    pub allowances: HashMap<String, HashMap<String, u64>>,
    /// Contract owner
    pub owner: String,
    /// Event log
    pub events: Vec<MediTokenEvent>,
}

impl MediToken {
    /// Creates a new MediToken instance
    pub fn new(initial_supply: u64, owner: String) -> Self {
        let mut balances = HashMap::new();
        balances.insert(owner.clone(), initial_supply);
        
        let mut token = Self {
            name: "MediToken".to_string(),
            symbol: "MEDT".to_string(),
            decimals: 18,
            total_supply: initial_supply,
            balances,
            allowances: HashMap::new(),
            owner: owner.clone(),
            events: Vec::new(),
        };
        
        // Emit initial transfer event (mint)
        token.emit_event(MediTokenEvent::Transfer {
            from: "0x0000000000000000000000000000000000000000".to_string(),
            to: owner,
            value: initial_supply,
        });
        
        token
    }
    
    /// Gets the balance of an address
    pub fn balance_of(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }
    
    /// Gets the allowance for a spender from an owner
    pub fn allowance(&self, owner: &str, spender: &str) -> u64 {
        self.allowances
            .get(owner)
            .and_then(|allowances| allowances.get(spender))
            .copied()
            .unwrap_or(0)
    }
    
    /// Transfers tokens from one address to another
    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<bool> {
        self.validate_transfer(from, to, amount)?;
        
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return Err(MediTokenError::InsufficientBalance {
                required: amount,
                available: from_balance,
            });
        }
        
        // Update balances
        self.balances.insert(from.to_string(), from_balance - amount);
        let to_balance = self.balance_of(to);
        self.balances.insert(to.to_string(), to_balance + amount);
        
        // Emit transfer event
        self.emit_event(MediTokenEvent::Transfer {
            from: from.to_string(),
            to: to.to_string(),
            value: amount,
        });
        
        Ok(true)
    }
    
    /// Approves a spender to spend tokens on behalf of owner
    pub fn approve(&mut self, owner: &str, spender: &str, amount: u64) -> Result<bool> {
        self.validate_approval(owner, spender)?;
        
        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);
        
        // Emit approval event
        self.emit_event(MediTokenEvent::Approval {
            owner: owner.to_string(),
            spender: spender.to_string(),
            value: amount,
        });
        
        Ok(true)
    }
    
    /// Transfers tokens from owner to recipient using allowance
    pub fn transfer_from(&mut self, spender: &str, from: &str, to: &str, amount: u64) -> Result<bool> {
        self.validate_transfer(from, to, amount)?;
        
        let current_allowance = self.allowance(from, spender);
        if current_allowance < amount {
            return Err(MediTokenError::InsufficientAllowance {
                spender: spender.to_string(),
                allowance: current_allowance,
                needed: amount,
            });
        }
        
        // Update allowance
        self.allowances
            .get_mut(from)
            .unwrap()
            .insert(spender.to_string(), current_allowance - amount);
        
        // Perform transfer
        self.transfer(from, to, amount)
    }
    
    /// Validates transfer parameters
    fn validate_transfer(&self, from: &str, to: &str, _amount: u64) -> Result<()> {
        if from == "0x0000000000000000000000000000000000000000" {
            return Err(MediTokenError::TransferFromZeroAddress);
        }
        if to == "0x0000000000000000000000000000000000000000" {
            return Err(MediTokenError::TransferToZeroAddress);
        }
        Ok(())
    }
    
    /// Validates approval parameters
    fn validate_approval(&self, owner: &str, spender: &str) -> Result<()> {
        if owner == "0x0000000000000000000000000000000000000000" {
            return Err(MediTokenError::ApproveFromZeroAddress);
        }
        if spender == "0x0000000000000000000000000000000000000000" {
            return Err(MediTokenError::ApproveToZeroAddress);
        }
        Ok(())
    }
    
    /// Emits an event
    fn emit_event(&mut self, event: MediTokenEvent) {
        self.events.push(event);
    }
    
    /// Gets all events
    pub fn get_events(&self) -> &[MediTokenEvent] {
        &self.events
    }
    
    /// Converts amount with decimals to raw amount
    pub fn to_raw_amount(&self, amount: f64) -> u64 {
        (amount * 10_f64.powi(self.decimals as i32)) as u64
    }
    
    /// Converts raw amount to amount with decimals
    pub fn from_raw_amount(&self, raw_amount: u64) -> f64 {
        raw_amount as f64 / 10_f64.powi(self.decimals as i32)
    }
}

impl Default for MediToken {
    fn default() -> Self {
        Self::new(
            1000 * 10_u64.pow(18), 
            "0x0000000000000000000000000000000000000001".to_string(),
        )
    }
}