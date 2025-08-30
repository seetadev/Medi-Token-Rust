use medi_token::deployment::{Network, Deployer, get_deployed_addresses};
use std::env;

#[cfg(test)]
mod deployment_tests {
    use super::*;

    #[test]
    fn test_network_properties() {
        assert_eq!(Network::Sepolia.chain_id(), 11155111);
        assert_eq!(Network::Amoy.chain_id(), 80002);
        assert_eq!(Network::ArbitrumSepolia.chain_id(), 421614);
        assert_eq!(Network::OpSepolia.chain_id(), 11155420);
        assert_eq!(Network::Cardona.chain_id(), 2442);
        assert_eq!(Network::ScrollSepolia.chain_id(), 534351);
        assert_eq!(Network::Local.chain_id(), 31337);
    }

    #[test]
    fn test_rpc_url_env_vars() {
        assert_eq!(Network::Sepolia.rpc_url_env_var(), "SEPOLIA_RPC_URL");
        assert_eq!(Network::Amoy.rpc_url_env_var(), "AMOY_RPC_URL");
        assert_eq!(Network::ArbitrumSepolia.rpc_url_env_var(), "ARBITRUM_SEPOLIA_RPC_URL");
        assert_eq!(Network::OpSepolia.rpc_url_env_var(), "OP_SEPOLIA_RPC_URL");
        assert_eq!(Network::Cardona.rpc_url_env_var(), "CARDONA_RPC_URL");
        assert_eq!(Network::ScrollSepolia.rpc_url_env_var(), "SCROLL_SEPOLIA_RPC_URL");
        assert_eq!(Network::Local.rpc_url_env_var(), "LOCAL_RPC_URL");
    }

    #[test]
    fn test_deployed_addresses() {
        let addresses = get_deployed_addresses();
        
        // Check that we have addresses for expected networks
        assert!(addresses.contains_key(&Network::Sepolia));
        assert!(addresses.contains_key(&Network::OpSepolia));
        assert!(addresses.contains_key(&Network::ArbitrumSepolia));
        assert!(addresses.contains_key(&Network::Amoy));
        assert!(addresses.contains_key(&Network::Cardona));
        assert!(addresses.contains_key(&Network::ScrollSepolia));

        // Check address format
        for (_, address) in addresses.iter() {
            assert!(address.starts_with("0x"));
            assert_eq!(address.len(), 42);
        }

        // Check specific known addresses match the original deployment
        assert_eq!(
            addresses.get(&Network::OpSepolia).unwrap(),
            "0xc898870DF59123F346a0e3787966023e0ED78B93"
        );
        assert_eq!(
            addresses.get(&Network::Sepolia).unwrap(),
            "0x3B550adA770897B0b215e414e45354861357788c"
        );
    }

    #[test]
    fn test_deployer_creation_fails_without_env() {
        // Temporarily remove environment variables
        let original_private_key = env::var("PRIVATE_KEY").ok();
        let original_rpc_url = env::var("SEPOLIA_RPC_URL").ok();

        env::remove_var("PRIVATE_KEY");
        env::remove_var("SEPOLIA_RPC_URL");

        // Should fail without required environment variables
        let result = Deployer::new(Network::Sepolia);
        assert!(result.is_err());

        // Restore environment variables if they existed
        if let Some(key) = original_private_key {
            env::set_var("PRIVATE_KEY", key);
        }
        if let Some(url) = original_rpc_url {
            env::set_var("SEPOLIA_RPC_URL", url);
        }
    }

    #[tokio::test]
    async fn test_deployment_simulation() {
        // Set up mock environment variables for testing
        env::set_var("PRIVATE_KEY", "0x1234567890123456789012345678901234567890123456789012345678901234");
        env::set_var("SEPOLIA_RPC_URL", "https://eth-sepolia.g.alchemy.com/v2/test");

        let deployer = Deployer::new(Network::Sepolia).unwrap();
        let deployed = deployer.deploy().await.unwrap();

        // Check deployment result
        assert!(deployed.address.starts_with("0x"));
        assert_eq!(deployed.address.len(), 42);
        assert!(deployed.transaction_hash.starts_with("0x"));
        assert_eq!(deployed.transaction_hash.len(), 66);
        assert!(deployed.gas_used > 0);
        assert!(deployed.block_number > 0);

        // Clean up
        env::remove_var("PRIVATE_KEY");
        env::remove_var("SEPOLIA_RPC_URL");
    }
}
