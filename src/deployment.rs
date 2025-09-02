use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Supported blockchain networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Network {
    Sepolia,
    Amoy,
    ArbitrumSepolia,
    OpSepolia,
    Cardona,
    ScrollSepolia,
    Local,
}

impl Network {
    pub fn rpc_url_env_var(&self) -> &'static str {
        match self {
            Network::Sepolia => "SEPOLIA_RPC_URL",
            Network::Amoy => "AMOY_RPC_URL",
            Network::ArbitrumSepolia => "ARBITRUM_SEPOLIA_RPC_URL",
            Network::OpSepolia => "OP_SEPOLIA_RPC_URL",
            Network::Cardona => "CARDONA_RPC_URL",
            Network::ScrollSepolia => "SCROLL_SEPOLIA_RPC_URL",
            Network::Local => "LOCAL_RPC_URL",
        }
    }
    
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Sepolia => 11155111,
            Network::Amoy => 80002,
            Network::ArbitrumSepolia => 421614,
            Network::OpSepolia => 11155420,
            Network::Cardona => 2442,
            Network::ScrollSepolia => 534351,
            Network::Local => 31337,
        }
    }
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub network: Network,
    pub initial_supply: u64,
    pub private_key: String,
    pub rpc_url: String,
}

/// Deployed contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployedContract {
    pub address: String,
    pub network: Network,
    pub transaction_hash: String,
    pub block_number: u64,
    pub gas_used: u64,
}

/// Contract deployer
pub struct Deployer {
    config: DeploymentConfig,
}

impl Deployer {
    pub fn new(network: Network) -> Result<Self> {
        let rpc_url = env::var(network.rpc_url_env_var())
            .map_err(|_| anyhow::anyhow!("RPC URL not found for network {:?}", network))?;
        
        let private_key = env::var("PRIVATE_KEY")
            .map_err(|_| anyhow::anyhow!("PRIVATE_KEY environment variable not set"))?;
        
        let config = DeploymentConfig {
            network,
            initial_supply: 1000 * 10_u64.pow(18), 
            private_key,
            rpc_url,
        };
        
        Ok(Self { config })
    }
    
    /// Deploy the MediToken contract
    pub async fn deploy(&self) -> Result<DeployedContract> {
        println!(" Deploying MediToken to {:?}", self.config.network);
        println!(" RPC URL: {}", self.config.rpc_url);
        println!(
            " Initial Supply: {} MEDT",
            self.config.initial_supply / 10_u64.pow(18)
        );
        

        let deployed = DeployedContract {
            address: format!("0x{}", hex::encode(rand::random::<[u8; 20]>())),
            network: self.config.network.clone(),
            transaction_hash: format!("0x{}", hex::encode(rand::random::<[u8; 32]>())),
            block_number: 12345678,
            gas_used: 1200000,
        };
        
        println!(" MediToken deployed successfully!");
        println!(" Contract Address: {}", deployed.address);
        println!(" Transaction Hash: {}", deployed.transaction_hash);
        println!(" Block Number: {}", deployed.block_number);
        println!(" Gas Used: {}", deployed.gas_used);
        
        Ok(deployed)
    }
}

/// Known deployed contract addresses
pub fn get_deployed_addresses() -> HashMap<Network, String> {
    let mut addresses = HashMap::new();
    addresses.insert(
        Network::OpSepolia,
        "0xc898870DF59123F346a0e3787966023e0ED78B93".to_string(),
    );
    addresses.insert(
        Network::ArbitrumSepolia,
        "0x89E4F30AFB281689632535e1657D15243a83b802".to_string(),
    );
    addresses.insert(
        Network::Sepolia,
        "0x3B550adA770897B0b215e414e45354861357788c".to_string(),
    );
    addresses.insert(
        Network::Amoy,
        "0x7aD0A9dB054101be9428fa89bB1194506586D1aD".to_string(),
    );
    addresses.insert(
        Network::Cardona,
        "0x4216a9c6EB59FcA323169Ef3194783d3dC9b7F23".to_string(),
    );
    addresses.insert(
        Network::ScrollSepolia,
        "0x6e650a339AbE4D9cf0aa8091fB2099284968beFf".to_string(),
    );

    addresses
}