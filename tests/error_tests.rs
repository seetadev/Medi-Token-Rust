use medi_token::{MediTokenError, utils::Utils};

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let insufficient_balance = MediTokenError::InsufficientBalance {
            required: 1000,
            available: 500,
        };
        assert_eq!(
            insufficient_balance.to_string(),
            "Insufficient balance: required 1000, available 500"
        );

        let insufficient_allowance = MediTokenError::InsufficientAllowance {
            spender: "0x123".to_string(),
            allowance: 100,
            needed: 200,
        };
        assert_eq!(
            insufficient_allowance.to_string(),
            "Insufficient allowance: spender 0x123, allowance 100, needed 200"
        );

        let invalid_address = MediTokenError::InvalidAddress {
            address: "invalid".to_string(),
        };
        assert_eq!(
            invalid_address.to_string(),
            "Invalid address: invalid"
        );
    }

    #[test]
    fn test_error_types() {
        // Test that we can match on error types
        let error = MediTokenError::TransferToZeroAddress;
        match error {
            MediTokenError::TransferToZeroAddress => {}, // Expected
            _ => panic!("Wrong error type"),
        }

        let error = MediTokenError::ArithmeticOverflow;
        match error {
            MediTokenError::ArithmeticOverflow => {}, // Expected
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_error_from_conversions() {
        // Test error conversions
        let json_error = serde_json::Error::syntax(serde_json::error::ErrorCode::ExpectedColon, 1, 1);
        let converted: MediTokenError = json_error.into();
        match converted {
            MediTokenError::SerializationError(_) => {}, // Expected
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_arithmetic_errors() {
        // Test overflow
        let overflow_result = Utils::check_overflow(u64::MAX, 1);
        assert!(overflow_result.is_err());
        assert!(matches!(overflow_result.unwrap_err(), MediTokenError::ArithmeticOverflow));

        // Test underflow
        let underflow_result = Utils::check_underflow(0, 1);
        assert!(underflow_result.is_err());
        assert!(matches!(underflow_result.unwrap_err(), MediTokenError::ArithmeticUnderflow));
    }
}
