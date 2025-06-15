use medi_token::{MediToken, utils::Utils, MediTokenError, token::MediTokenEvent};

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn test_token_initialization() {
        let owner = "0x1234567890123456789012345678901234567890".to_string();
        let initial_supply = 1000 * 10_u64.pow(18);
        let token = MediToken::new(initial_supply, owner.clone());

        assert_eq!(token.name, "MediToken");
        assert_eq!(token.symbol, "MEDT");
        assert_eq!(token.decimals, 18);
        assert_eq!(token.total_supply, initial_supply);
        assert_eq!(token.balance_of(&owner), initial_supply);
        assert_eq!(token.owner, owner);

        // Check initial mint event
        let events = token.get_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            MediTokenEvent::Transfer { from, to, value } => {
                assert_eq!(from, "0x0000000000000000000000000000000000000000");
                assert_eq!(to, &owner);
                assert_eq!(*value, initial_supply);
            }
            _ => panic!("Expected Transfer event"),
        }
    }

    #[test]
    fn test_transfer_success() {
        let owner = Utils::generate_random_address();
        let recipient = Utils::generate_random_address();
        let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());

        let transfer_amount = 100 * 10_u64.pow(18);
        let result = token.transfer(&owner, &recipient, transfer_amount);

        assert!(result.is_ok());
        assert_eq!(token.balance_of(&owner), 900 * 10_u64.pow(18));
        assert_eq!(token.balance_of(&recipient), 100 * 10_u64.pow(18));

        // Check transfer event
        let events = token.get_events();
        assert_eq!(events.len(), 2); // Initial mint + transfer
        match &events[1] {
            MediTokenEvent::Transfer { from, to, value } => {
                assert_eq!(from, &owner);
                assert_eq!(to, &recipient);
                assert_eq!(*value, transfer_amount);
            }
            _ => panic!("Expected Transfer event"),
        }
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let owner = Utils::generate_random_address();
        let recipient = Utils::generate_random_address();
        let mut token = MediToken::new(100 * 10_u64.pow(18), owner.clone());

        let transfer_amount = 200 * 10_u64.pow(18); // More than balance
        let result = token.transfer(&owner, &recipient, transfer_amount);

        assert!(result.is_err());
        match result.unwrap_err() {
            MediTokenError::InsufficientBalance { required, available } => {
                assert_eq!(required, transfer_amount);
                assert_eq!(available, 100 * 10_u64.pow(18));
            }
            _ => panic!("Expected InsufficientBalance error"),
        }
    }

    #[test]
    fn test_transfer_to_zero_address() {
        let owner = Utils::generate_random_address();
        let mut token = MediToken::new(100 * 10_u64.pow(18), owner.clone());

        let result = token.transfer(&owner, "0x0000000000000000000000000000000000000000", 50);
        assert!(matches!(result.unwrap_err(), MediTokenError::TransferToZeroAddress));
    }

    #[test]
    fn test_transfer_from_zero_address() {
        let recipient = Utils::generate_random_address();
        let mut token = MediToken::new(100 * 10_u64.pow(18), recipient.clone());

        let result = token.transfer("0x0000000000000000000000000000000000000000", &recipient, 50);
        assert!(matches!(result.unwrap_err(), MediTokenError::TransferFromZeroAddress));
    }

    #[test]
    fn test_approve_success() {
        let owner = Utils::generate_random_address();
        let spender = Utils::generate_random_address();
        let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());

        let allowance_amount = 200 * 10_u64.pow(18);
        let result = token.approve(&owner, &spender, allowance_amount);

        assert!(result.is_ok());
        assert_eq!(token.allowance(&owner, &spender), allowance_amount);

        // Check approval event
        let events = token.get_events();
        assert_eq!(events.len(), 2); // Initial mint + approval
        match &events[1] {
            MediTokenEvent::Approval { owner: event_owner, spender: event_spender, value } => {
                assert_eq!(event_owner, &owner);
                assert_eq!(event_spender, &spender);
                assert_eq!(*value, allowance_amount);
            }
            _ => panic!("Expected Approval event"),
        }
    }

    #[test]
    fn test_approve_to_zero_address() {
        let owner = Utils::generate_random_address();
        let mut token = MediToken::new(100 * 10_u64.pow(18), owner.clone());

        let result = token.approve(&owner, "0x0000000000000000000000000000000000000000", 50);
        assert!(matches!(result.unwrap_err(), MediTokenError::ApproveToZeroAddress));
    }

    #[test]
    fn test_approve_from_zero_address() {
        let spender = Utils::generate_random_address();
        let mut token = MediToken::new(100 * 10_u64.pow(18), spender.clone());

        let result = token.approve("0x0000000000000000000000000000000000000000", &spender, 50);
        assert!(matches!(result.unwrap_err(), MediTokenError::ApproveFromZeroAddress));
    }

    #[test]
    fn test_transfer_from_success() {
        let owner = Utils::generate_random_address();
        let spender = Utils::generate_random_address();
        let recipient = Utils::generate_random_address();
        let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());

        let allowance_amount = 200 * 10_u64.pow(18);
        let transfer_amount = 50 * 10_u64.pow(18);

        // First approve
        token.approve(&owner, &spender, allowance_amount).unwrap();

        // Then transfer from
        let result = token.transfer_from(&spender, &owner, &recipient, transfer_amount);

        assert!(result.is_ok());
        assert_eq!(token.balance_of(&owner), 950 * 10_u64.pow(18));
        assert_eq!(token.balance_of(&recipient), 50 * 10_u64.pow(18));
        assert_eq!(token.allowance(&owner, &spender), 150 * 10_u64.pow(18));
    }

    #[test]
    fn test_transfer_from_insufficient_allowance() {
        let owner = Utils::generate_random_address();
        let spender = Utils::generate_random_address();
        let recipient = Utils::generate_random_address();
        let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());

        let allowance_amount = 50 * 10_u64.pow(18);
        let transfer_amount = 100 * 10_u64.pow(18); // More than allowance

        // First approve
        token.approve(&owner, &spender, allowance_amount).unwrap();

        // Then try to transfer more than allowance
        let result = token.transfer_from(&spender, &owner, &recipient, transfer_amount);

        assert!(result.is_err());
        match result.unwrap_err() {
            MediTokenError::InsufficientAllowance { spender: err_spender, allowance, needed } => {
                assert_eq!(err_spender, spender);
                assert_eq!(allowance, allowance_amount);
                assert_eq!(needed, transfer_amount);
            }
            _ => panic!("Expected InsufficientAllowance error"),
        }
    }

    #[test]
    fn test_multiple_approvals() {
        let owner = Utils::generate_random_address();
        let spender1 = Utils::generate_random_address();
        let spender2 = Utils::generate_random_address();
        let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());

        // Approve multiple spenders
        token.approve(&owner, &spender1, 100 * 10_u64.pow(18)).unwrap();
        token.approve(&owner, &spender2, 200 * 10_u64.pow(18)).unwrap();

        assert_eq!(token.allowance(&owner, &spender1), 100 * 10_u64.pow(18));
        assert_eq!(token.allowance(&owner, &spender2), 200 * 10_u64.pow(18));

        // Update approval for spender1
        token.approve(&owner, &spender1, 300 * 10_u64.pow(18)).unwrap();
        assert_eq!(token.allowance(&owner, &spender1), 300 * 10_u64.pow(18));
        assert_eq!(token.allowance(&owner, &spender2), 200 * 10_u64.pow(18)); // Unchanged
    }

    #[test]
    fn test_balance_of_nonexistent_address() {
        let owner = Utils::generate_random_address();
        let token = MediToken::new(1000 * 10_u64.pow(18), owner);

        let nonexistent = Utils::generate_random_address();
        assert_eq!(token.balance_of(&nonexistent), 0);
    }

    #[test]
    fn test_allowance_nonexistent_addresses() {
        let owner = Utils::generate_random_address();
        let token = MediToken::new(1000 * 10_u64.pow(18), owner);

        let addr1 = Utils::generate_random_address();
        let addr2 = Utils::generate_random_address();
        assert_eq!(token.allowance(&addr1, &addr2), 0);
    }

    #[test]
    fn test_decimal_conversions() {
        let token = MediToken::default();

        // Test various amounts
        let test_cases = vec![1.0, 0.5, 1.5, 100.0, 0.001, 999.999];

        for amount in test_cases {
            let raw = token.to_raw_amount(amount);
            let converted_back = token.from_raw_amount(raw);
            assert!((converted_back - amount).abs() < 1e-10, 
                   "Conversion failed for {}: got {}", amount, converted_back);
        }
    }

    #[test]
    fn test_large_transfers() {
        let owner = Utils::generate_random_address();
        let recipient = Utils::generate_random_address();
        let large_amount = u64::MAX / 2; // Large but safe amount
        let mut token = MediToken::new(large_amount, owner.clone());

        let transfer_amount = large_amount / 4;
        let result = token.transfer(&owner, &recipient, transfer_amount);

        assert!(result.is_ok());
        assert_eq!(token.balance_of(&owner), large_amount - transfer_amount);
        assert_eq!(token.balance_of(&recipient), transfer_amount);
    }
}
