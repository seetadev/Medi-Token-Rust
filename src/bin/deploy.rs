use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use medi_token::deployment::{Deployer, Network};
use anyhow::Result;

#[derive(ValueEnum, Clone, Debug)]
enum NetworkChoice {
    Sepolia,
    Amoy,
    Arbitrum,
    Optimism,
    Cardona,
    Scroll,
    Local,
}

impl From<NetworkChoice> for Network {
    fn from(choice: NetworkChoice) -> Self {
        match choice {
            NetworkChoice::Sepolia => Network::Sepolia,
            NetworkChoice::Amoy => Network::Amoy,
            NetworkChoice::Arbitrum => Network::ArbitrumSepolia,
            NetworkChoice::Optimism => Network::OpSepolia,
            NetworkChoice::Cardona => Network::Cardona,
            NetworkChoice::Scroll => Network::ScrollSepolia,
            NetworkChoice::Local => Network::Local,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network to deploy to
    #[arg(short, long, value_enum)]
    network: NetworkChoice,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let cli = Cli::parse();
    
    if cli.verbose {
        println!("🔧 Verbose mode enabled");
        println!(" Target network: {:?}", cli.network);
    }
    
    let network: Network = cli.network.into();
    let deployer = Deployer::new(network)?;
    
    let deployed = deployer.deploy().await?;
    
    if cli.verbose {
        println!("\n Deployment Summary:");
        println!("Network: {:?}", deployed.network);
        println!("Address: {}", deployed.address);
        println!("Transaction: {}", deployed.transaction_hash);
        println!("Block: {}", deployed.block_number);
        println!("Gas Used: {}", deployed.gas_used);
    }
    
    Ok(())
}