use medi_token::abi::get_meditoken_abi;
use serde_json::Value;

#[cfg(test)]
mod abi_tests {
    use super::*;

    #[test]
    fn test_abi_structure() {
        let abi = get_meditoken_abi();
        
        // Should be an array
        assert!(abi.is_array());
        let abi_array = abi.as_array().unwrap();
        
        // Should have multiple entries
        assert!(abi_array.len() > 10);
        
        // Check for required functions
        let function_names: Vec<String> = abi_array
            .iter()
            .filter(|item| item["type"] == "function")
            .map(|item| item["name"].as_str().unwrap().to_string())
            .collect();
        
        assert!(function_names.contains(&"transfer".to_string()));
        assert!(function_names.contains(&"approve".to_string()));
        assert!(function_names.contains(&"transferFrom".to_string()));
        assert!(function_names.contains(&"balanceOf".to_string()));
        assert!(function_names.contains(&"allowance".to_string()));
        assert!(function_names.contains(&"totalSupply".to_string()));
        assert!(function_names.contains(&"name".to_string()));
        assert!(function_names.contains(&"symbol".to_string()));
        assert!(function_names.contains(&"decimals".to_string()));
    }

    #[test]
    fn test_abi_events() {
        let abi = get_meditoken_abi();
        let abi_array = abi.as_array().unwrap();
        
        // Check for required events
        let event_names: Vec<String> = abi_array
            .iter()
            .filter(|item| item["type"] == "event")
            .map(|item| item["name"].as_str().unwrap().to_string())
            .collect();
        
        assert!(event_names.contains(&"Transfer".to_string()));
        assert!(event_names.contains(&"Approval".to_string()));
    }

    #[test]
    fn test_abi_errors() {
        let abi = get_meditoken_abi();
        let abi_array = abi.as_array().unwrap();
        
        // Check for error definitions
        let error_names: Vec<String> = abi_array
            .iter()
            .filter(|item| item["type"] == "error")
            .map(|item| item["name"].as_str().unwrap().to_string())
            .collect();
        
        assert!(error_names.contains(&"ERC20InsufficientBalance".to_string()));
        assert!(error_names.contains(&"ERC20InsufficientAllowance".to_string()));
    }

    #[test]
    fn test_abi_constructor() {
        let abi = get_meditoken_abi();
        let abi_array = abi.as_array().unwrap();
        
        // Find constructor
        let constructor = abi_array
            .iter()
            .find(|item| item["type"] == "constructor");
        
        assert!(constructor.is_some());
        let constructor = constructor.unwrap();
        
        // Check constructor inputs
        let inputs = constructor["inputs"].as_array().unwrap();
        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0]["name"], "initialSupply");
        assert_eq!(inputs[0]["type"], "uint256");
    }

    #[test]
    fn test_abi_serialization() {
        let abi = get_meditoken_abi();
        
        // Should be able to serialize back to JSON string
        let json_string = serde_json::to_string(&abi).unwrap();
        assert!(json_string.len() > 1000); // Should be a substantial JSON string
        
        // Should be able to parse back
        let parsed: Value = serde_json::from_str(&json_string).unwrap();
        assert_eq!(parsed, abi);
    }
}
