# NEAR Contract Setup

## Quick Start

```bash
# Build the contract
make build-near

# Deploy (first time with initialization)
near contract deploy YOUR_ACCOUNT.testnet \
  use-file near-contract/target/near/medi_token_near.wasm \
  with-init-call new json-args '{"owner_id":"YOUR_ACCOUNT.testnet","total_supply":"1000000000000000000000000"}' \
  prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' \
  network-config testnet sign-with-keychain send

# Update existing contract
near contract deploy YOUR_ACCOUNT.testnet \
  use-file near-contract/target/near/medi_token_near.wasm \
  without-init-call network-config testnet sign-with-keychain send
```

## Project Structure

```
near-contract/
├── Cargo.toml    # Minimal dependencies (near-sdk, near-contract-standards, borsh)
└── src/
    └── lib.rs    # NEP-141 fungible token implementation
```

## Build Process

`make build-near` runs `cargo near build` which:
- Generates ABI schema
- Compiles optimized WASM
- Applies wasm-opt size optimization
- Embeds NEP-330 contract metadata

Output: `near-contract/target/near/medi_token_near.wasm`

## Current Deployment

- **Account**: `harshitnayan.testnet`
- **Network**: NEAR Testnet
- **Total Supply**: 1,000,000 MEDT
- **Transaction**: [G3KGp8QTiS8sjubfPr4nJ8djQZ22i7GfWV72bQ2mXP5Q](https://explorer.testnet.near.org/transactions/G3KGp8QTiS8sjubfPr4nJ8djQZ22i7GfWV72bQ2mXP5Q)

## Testing

```bash
# Check total supply
near contract call-function as-read-only harshitnayan.testnet \
  ft_total_supply json-args {} network-config testnet now

# Check metadata
near contract call-function as-read-only harshitnayan.testnet \
  ft_metadata json-args {} network-config testnet now

# Check balance
near contract call-function as-read-only harshitnayan.testnet \
  ft_balance_of json-args '{"account_id":"harshitnayan.testnet"}' \
  network-config testnet now
```
