use medi_token::{MediToken, utils::Utils, MediTokenError};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_token_workflow() {
        let owner = Utils::generate_random_address();
        let user1 = Utils::generate_random_address();
        let user2 = Utils::generate_random_address();
        
        let initial_supply = Utils::ether_to_wei(1000.0);
        let mut token = MediToken::new(initial_supply, owner.clone());
        
        // Test initial state
        assert_eq!(token.balance_of(&owner), initial_supply);
        assert_eq!(token.total_supply, initial_supply);
        
        // Transfer tokens to user1
        let transfer_amount = Utils::ether_to_wei(100.0);
        assert!(token.transfer(&owner, &user1, transfer_amount).is_ok());
        assert_eq!(token.balance_of(&user1), transfer_amount);
        
        // User1 approves user2 to spend tokens
        let allowance_amount = Utils::ether_to_wei(50.0);
        assert!(token.approve(&user1, &user2, allowance_amount).is_ok());
        assert_eq!(token.allowance(&user1, &user2), allowance_amount);
        
        // User2 transfers from user1 to owner
        let spend_amount = Utils::ether_to_wei(25.0);
        assert!(token.transfer_from(&user2, &user1, &owner, spend_amount).is_ok());
        
        // Verify final balances
        assert_eq!(token.balance_of(&user1), transfer_amount - spend_amount);
        assert_eq!(token.balance_of(&owner), initial_supply - transfer_amount + spend_amount);
        assert_eq!(token.allowance(&user1, &user2), allowance_amount - spend_amount);
    }
    
    #[test]
    fn test_token_events() {
        let owner = Utils::generate_random_address();
        let user = Utils::generate_random_address();
        let mut token = MediToken::new(Utils::ether_to_wei(1000.0), owner.clone());
        
        let initial_events = token.get_events().len();
        
        // Perform a transfer
        token.transfer(&owner, &user, Utils::ether_to_wei(100.0)).unwrap();
        
        // Perform an approval
        token.approve(&user, &owner, Utils::ether_to_wei(50.0)).unwrap();
        
        let final_events = token.get_events().len();
        assert_eq!(final_events, initial_events + 2);
    }
    
    #[test]
    fn test_decimal_conversions() {
        let token = MediToken::default();
        
        let amount = 1.5; // 1.5 tokens
        let raw_amount = token.to_raw_amount(amount);
        let converted_back = token.from_raw_amount(raw_amount);
        
        assert_eq!(converted_back, amount);
    }
    
    #[test]
    fn test_error_scenarios() {
        let owner = Utils::generate_random_address();
        let user = Utils::generate_random_address();
        let zero_addr = "0x0000000000000000000000000000000000000000";
        
        let mut token = MediToken::new(Utils::ether_to_wei(1000.0), owner.clone());
        
        // Test insufficient balance error
        let result = token.transfer(&user, &owner, Utils::ether_to_wei(1.0));
        assert!(matches!(result.unwrap_err(), MediTokenError::InsufficientBalance { .. }));
        
        // Test zero address errors
        let result = token.transfer(&owner, zero_addr, Utils::ether_to_wei(1.0));
        assert!(matches!(result.unwrap_err(), MediTokenError::TransferToZeroAddress));
        
        let result = token.approve(&owner, zero_addr, Utils::ether_to_wei(1.0));
        assert!(matches!(result.unwrap_err(), MediTokenError::ApproveToZeroAddress));
    }
}