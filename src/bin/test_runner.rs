use medi_token::{MediToken, utils::Utils};
// use std::collections::HashMap;

fn main() {
    println!(" Running MediToken Tests");
    println!("==========================");
    
    // Test 1: Token Creation
    test_token_creation();
    
    // Test 2: Balance Check  
    test_balance_check();
    
    // Test 3: Transfer
    test_transfer();
    
    // Test 4: Allowance and Transfer From
    test_allowance_and_transfer_from();
    
    // Test 5: Error Cases
    test_error_cases();
    
    // Test 6: Utility Functions
    test_utils();
    
    println!("\n✅ All tests passed!");
}

fn test_token_creation() {
    println!("🔸 Test 1: Token Creation");
    
    let owner = "0x1234567890123456789012345678901234567890".to_string();
    let initial_supply = 1000 * 10_u64.pow(18);
    let token = MediToken::new(initial_supply, owner.clone());
    
    assert_eq!(token.name, "MediToken");
    assert_eq!(token.symbol, "MEDT");
    assert_eq!(token.decimals, 18);
    assert_eq!(token.total_supply, initial_supply);
    assert_eq!(token.balance_of(&owner), initial_supply);
    assert_eq!(token.owner, owner);
    
    println!("   ✓ Token created with correct parameters");
}

fn test_balance_check() {
    println!("🔸 Test 2: Balance Check");
    
    let owner = "0x1234567890123456789012345678901234567890".to_string();
    let user = "0x0987654321098765432109876543210987654321".to_string();
    let token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());
    
    assert_eq!(token.balance_of(&owner), 1000 * 10_u64.pow(18));
    assert_eq!(token.balance_of(&user), 0);
    
    println!("   ✓ Balance checks working correctly");
}

fn test_transfer() {
    println!("🔸 Test 3: Transfer");
    
    let owner = "0x1234567890123456789012345678901234567890".to_string();
    let recipient = "0x0987654321098765432109876543210987654321".to_string();
    let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());
    
    let transfer_amount = 100 * 10_u64.pow(18);
    let result = token.transfer(&owner, &recipient, transfer_amount);
    
    assert!(result.is_ok());
    assert_eq!(token.balance_of(&owner), 900 * 10_u64.pow(18));
    assert_eq!(token.balance_of(&recipient), 100 * 10_u64.pow(18));
    
    // Check events
    let events = token.get_events();
    assert_eq!(events.len(), 2); // Initial mint + transfer
    
    println!("   ✓ Transfer working correctly");
}

fn test_allowance_and_transfer_from() {
    println!("🔸 Test 4: Allowance and Transfer From");
    
    let owner = "0x1234567890123456789012345678901234567890".to_string();
    let spender = "0x0987654321098765432109876543210987654321".to_string();
    let recipient = "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string();
    let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());
    
    let allowance_amount = 200 * 10_u64.pow(18);
    let transfer_amount = 50 * 10_u64.pow(18);
    
    // Approve spender
    let approve_result = token.approve(&owner, &spender, allowance_amount);
    assert!(approve_result.is_ok());
    assert_eq!(token.allowance(&owner, &spender), allowance_amount);
    
    // Transfer from owner to recipient via spender
    let transfer_result = token.transfer_from(&spender, &owner, &recipient, transfer_amount);
    assert!(transfer_result.is_ok());
    
    assert_eq!(token.balance_of(&owner), 950 * 10_u64.pow(18));
    assert_eq!(token.balance_of(&recipient), 50 * 10_u64.pow(18));
    assert_eq!(token.allowance(&owner, &spender), 150 * 10_u64.pow(18));
    
    println!("   ✓ Allowance and transfer from working correctly");
}

fn test_error_cases() {
    println!("🔸 Test 5: Error Cases");
    
    let owner = "0x1234567890123456789012345678901234567890".to_string();
    let user = "0x0987654321098765432109876543210987654321".to_string();
    let zero_address = "0x0000000000000000000000000000000000000000".to_string();
    let mut token = MediToken::new(1000 * 10_u64.pow(18), owner.clone());
    
    // Test insufficient balance
    let insufficient_result = token.transfer(&user, &owner, 100);
    assert!(insufficient_result.is_err());
    
    // Test transfer to zero address
    let zero_transfer_result = token.transfer(&owner, &zero_address, 100);
    assert!(zero_transfer_result.is_err());
    
    // Test approve to zero address
    let zero_approve_result = token.approve(&owner, &zero_address, 100);
    assert!(zero_approve_result.is_err());
    
    // Test insufficient allowance
    let no_allowance_result = token.transfer_from(&user, &owner, &user, 100);
    assert!(no_allowance_result.is_err());
    
    println!("   ✓ Error cases handled correctly");
}

fn test_utils() {
    println!("🔸 Test 6: Utility Functions");
    
    // Test address validation
    assert!(Utils::is_valid_address("0x1234567890123456789012345678901234567890"));
    assert!(!Utils::is_valid_address("invalid_address"));
    assert!(!Utils::is_valid_address("0x123")); // Too short
    
    // Test random address generation
    let random_addr = Utils::generate_random_address();
    assert!(Utils::is_valid_address(&random_addr));
    
    // Test wei/ether conversion
    let wei_amount = 1000000000000000000u64; // 1 ether in wei
    let ether_amount = Utils::wei_to_ether(wei_amount);
    assert_eq!(ether_amount, 1.0);
    
    let converted_back = Utils::ether_to_wei(ether_amount);
    assert_eq!(converted_back, wei_amount);
    
    // Test overflow/underflow checks
    let overflow_result = Utils::check_overflow(u64::MAX, 1);
    assert!(overflow_result.is_err());
    
    let underflow_result = Utils::check_underflow(0, 1);
    assert!(underflow_result.is_err());
    
    let valid_add = Utils::check_overflow(100, 200);
    assert_eq!(valid_add.unwrap(), 300);
    
    let valid_sub = Utils::check_underflow(200, 100);
    assert_eq!(valid_sub.unwrap(), 100);
    
    println!("   ✓ Utility functions working correctly");
}