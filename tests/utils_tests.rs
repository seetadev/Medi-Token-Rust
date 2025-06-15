use medi_token::{utils::Utils, MediTokenError};

#[cfg(test)]
mod utils_tests {
    use super::*;

    #[test]
    fn test_address_validation() {
        // Valid addresses
        assert!(Utils::is_valid_address("0x1234567890123456789012345678901234567890"));
        assert!(Utils::is_valid_address("0xabcdefABCDEF1234567890123456789012345678"));
        assert!(Utils::is_valid_address("0x0000000000000000000000000000000000000000"));

        // Invalid addresses
        assert!(!Utils::is_valid_address("1234567890123456789012345678901234567890")); 
        assert!(!Utils::is_valid_address("0x123")); 
        assert!(!Utils::is_valid_address("0x12345678901234567890123456789012345678901")); 
        assert!(!Utils::is_valid_address("0x123456789012345678901234567890123456789g")); 
        assert!(!Utils::is_valid_address("")); 
        assert!(!Utils::is_valid_address("0x")); 
    }

    #[test]
    fn test_random_address_generation() {
        let addr1 = Utils::generate_random_address();
        let addr2 = Utils::generate_random_address();

        // Should be valid addresses
        assert!(Utils::is_valid_address(&addr1));
        assert!(Utils::is_valid_address(&addr2));

        // Should be different
        assert_ne!(addr1, addr2);

        // Should start with 0x and be 42 characters long
        assert!(addr1.starts_with("0x"));
        assert_eq!(addr1.len(), 42);
    }

    #[test]
    fn test_wei_ether_conversions() {
        // Test exact conversions
        assert_eq!(Utils::wei_to_ether(1_000_000_000_000_000_000), 1.0);
        assert_eq!(Utils::wei_to_ether(500_000_000_000_000_000), 0.5);
        assert_eq!(Utils::wei_to_ether(2_500_000_000_000_000_000), 2.5);

        assert_eq!(Utils::ether_to_wei(1.0), 1_000_000_000_000_000_000);
        assert_eq!(Utils::ether_to_wei(0.5), 500_000_000_000_000_000);
        assert_eq!(Utils::ether_to_wei(2.5), 2_500_000_000_000_000_000);

        // Test round-trip conversions
        let test_wei_values = vec![
            1_000_000_000_000_000_000,
            500_000_000_000_000_000,
            1_234_567_890_123_456_789,
            1,
            999_999_999_999_999_999,
        ];

        for wei in test_wei_values {
            let ether = Utils::wei_to_ether(wei);
            let back_to_wei = Utils::ether_to_wei(ether);
            // Allow small rounding errors due to floating point precision
            assert!((back_to_wei as i128 - wei as i128).abs() <= 1);
        }
    }

    #[test]
    fn test_keccak256() {
        let input = b"hello world";
        let hash = Utils::keccak256(input);

        // Hash should be 32 bytes
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = Utils::keccak256(input);
        assert_eq!(hash, hash2);

        // Different input should produce different hash
        let hash3 = Utils::keccak256(b"hello world!");
        assert_ne!(hash, hash3);

        // Test known hash (you can verify this with online Keccak256 tools)
        let empty_hash = Utils::keccak256(b"");
        assert_eq!(
            hex::encode(empty_hash),
            "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
        );
    }

    #[test]
    fn test_overflow_check() {
        // Valid additions
        assert_eq!(Utils::check_overflow(100, 200).unwrap(), 300);
        assert_eq!(Utils::check_overflow(0, u64::MAX).unwrap(), u64::MAX);
        assert_eq!(Utils::check_overflow(u64::MAX, 0).unwrap(), u64::MAX);

        // Overflow cases
        assert!(Utils::check_overflow(u64::MAX, 1).is_err());
        assert!(Utils::check_overflow(u64::MAX / 2 + 1, u64::MAX / 2 + 1).is_err());

        // Check error type
        match Utils::check_overflow(u64::MAX, 1).unwrap_err() {
            MediTokenError::ArithmeticOverflow => {}, // Expected
            _ => panic!("Expected ArithmeticOverflow error"),
        }
    }

    #[test]
    fn test_underflow_check() {
        // Valid subtractions
        assert_eq!(Utils::check_underflow(300, 200).unwrap(), 100);
        assert_eq!(Utils::check_underflow(u64::MAX, 0).unwrap(), u64::MAX);
        assert_eq!(Utils::check_underflow(100, 100).unwrap(), 0);

        // Underflow cases
        assert!(Utils::check_underflow(0, 1).is_err());
        assert!(Utils::check_underflow(100, 200).is_err());

        // Check error type
        match Utils::check_underflow(0, 1).unwrap_err() {
            MediTokenError::ArithmeticUnderflow => {}, // Expected
            _ => panic!("Expected ArithmeticUnderflow error"),
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test with maximum values
        let max_wei = u64::MAX;
        let ether_val = Utils::wei_to_ether(max_wei);
        assert!(ether_val > 0.0);

        // Test with minimum values
        assert_eq!(Utils::wei_to_ether(0), 0.0);
        assert_eq!(Utils::ether_to_wei(0.0), 0);

        // Test address validation edge cases
        assert!(!Utils::is_valid_address("0X1234567890123456789012345678901234567890")); // Uppercase X
        assert!(Utils::is_valid_address("0x1234567890123456789012345678901234567890")); // Mixed case hex
    }
}
