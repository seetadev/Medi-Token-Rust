# MediToken Rust Implementation

A complete Rust implementation of the MediToken ERC20-like token for healthcare applications.

## 🌟 Features

- **Complete ERC20 Functionality**: Transfer, approve, allowances, and events
- **Multi-Chain Support**: Deploy to Sepolia, Polygon Amoy, Arbitrum, Optimism, and more
- **NEAR NEP-141 Support**: Deployable to the NEAR blockchain
- **Type Safety**: Full Rust type safety with comprehensive error handling
- **Testing Suite**: Extensive unit and integration tests
- **Deployment Tools**: CLI tools for easy deployment and testing
- **Healthcare Focus**: Designed specifically for healthcare applications

## 📋 Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/medi-token-rust
cd medi-token-rust
```

### 2. Set Up Environment

```bash
# Copy environment template
cp .env.example .env

# Edit .env file with your configuration
# Add your private key and RPC URLs
```

### 3. Build the Project

```bash
make build
# or
cargo build --release
```

### 4. Run Tests

```bash
make test
# or 
cargo test
```

### 5. Run the Test Suite

```bash
make run
# or
cargo run --bin test_runner
```

## 🔧 Development Commands

```bash
# Format code
make fmt

# Run linter
make clippy

# Generate documentation
make doc

# Development workflow (format + lint + test)
make dev

# Clean build artifacts
make clean
```

## 🚀 Deployment

Deploy to different networks using the CLI tool:

```bash
# Deploy to Sepolia testnet
make deploy NETWORK=sepolia

# Deploy to Arbitrum Sepolia
make deploy NETWORK=arbitrum

# Deploy to Optimism Sepolia  
make deploy NETWORK=optimism

# Deploy to Polygon Amoy
make deploy NETWORK=amoy

# Deploy to other supported networks
make deploy NETWORK=cardona
make deploy NETWORK=scroll
```

## 🏗️ Project Structure

```
medi-token-rust/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── token.rs            # Core MediToken implementation
│   ├── error.rs            # Error types and handling
│   ├── utils.rs            # Utility functions
│   ├── abi.rs              # Contract ABI definitions
│   ├── deployment.rs       # Deployment utilities
│   ├── near_token.rs       # NEP-141 compliant token for NEAR
│   └── bin/
│       ├── deploy.rs       # Deployment CLI tool
│       └── test_runner.rs  # Test runner binary
├── tests/
│   └── integration_tests.rs # Integration tests
├── Cargo.toml              # Rust dependencies
├── Makefile               # Build automation
├── .env.example           # Environment template
└── README.md              # This file
```

## 💊 Healthcare Use Cases

MediToken can be utilized in various healthcare applications:

- **🔐 Token-Gated Access Control** for resources & functionalities
- **📋 Tokenized Health Records**
- **💳 Patient Payments**
- **🏥 Insurance Claims**
- **🎁 Health Rewards**
- **🔬 Research Funding and Donations**
- **📦 Inventory Management**
- **🗳️ Governance and Voting**
- **👤 Patient Data Ownership**

## 🔌 Integration Examples

### Basic Token Operations

```rust
use medi_token::{MediToken, utils::Utils};

// Create a new token instance
let owner = Utils::generate_random_address();
let initial_supply = Utils::ether_to_wei(1000.0); // 1000 tokens
let mut token = MediToken::new(initial_supply, owner.clone());

// Check balance
let balance = token.balance_of(&owner);
println!("Owner balance: {} MEDT", Utils::wei_to_ether(balance));

// Transfer tokens
let recipient = Utils::generate_random_address();
let amount = Utils::ether_to_wei(100.0);
token.transfer(&owner, &recipient, amount).unwrap();

// Approve spending
let spender = Utils::generate_random_address();
token.approve(&owner, &spender, amount).unwrap();
```

### Error Handling

```rust
use medi_token::{MediToken, MediTokenError, utils::Utils};

let mut token = MediToken::default();
let result = token.transfer("invalid", "addresses", 100);

match result {
    Ok(_) => println!("Transfer successful"),
    Err(MediTokenError::InsufficientBalance { required, available }) => {
        println!("Insufficient balance: need {}, have {}", required, available);
    }
    Err(e) => println!("Error: {}", e),
}
```

## 🧪 Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_transfer

# Run with verbose output
cargo test -- --nocapture

# Run integration tests
cargo test integration_tests
```

## 📚 API Documentation

Generate and view the API documentation:

```bash
make doc
# or
cargo doc --open
```

## 🌐 Deployed Contract Addresses

| Network | Address |
|---------|---------|
| OP Sepolia | `0xc898870DF59123F346a0e3787966023e0ED78B93` |
| Arbitrum Sepolia | `0x89E4F30AFB281689632535e1657D15243a83b802` |
| Sepolia | `0x3B550adA770897B0b215e414e45354861357788c` |
| Polygon Amoy | `0x7aD0A9dB054101be9428fa89bB1194506586D1aD` |
| Polygon Cardona | `0x4216a9c6EB59FcA323169Ef3194783d3dC9b7F23` |
| Scroll Sepolia | `0x6e650a339AbE4D9cf0aa8091fB2099284968beFf` |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Original Solidity Implementation](https://github.com/seetadev/ZKMedical-Billing/tree/main/Medi_Token)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Ethereum Standards](https://eips.ethereum.org/)

---

**Built with ❤️ by Ritankar Saha <ritankar.saha786@gmail.com> for the healthcare industry using Rust 🦀**