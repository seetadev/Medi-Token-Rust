# MediToken NEAR Deployment

## Deployment Info

- **Account**: `harshitnayan.testnet`
- **Network**: NEAR Testnet
- **Token**: MediToken (MEDT)
- **Standard**: NEP-141 Fungible Token
- **Total Supply**: 1,000,000 MEDT
- **Decimals**: 18
- **Latest TX**: [G3KGp8QTiS8sjubfPr4nJ8djQZ22i7GfWV72bQ2mXP5Q](https://explorer.testnet.near.org/transactions/G3KGp8QTiS8sjubfPr4nJ8djQZ22i7GfWV72bQ2mXP5Q)

## NEP-141 Functions

**Core**:
- `ft_transfer` - Transfer tokens
- `ft_transfer_call` - Transfer and call receiver
- `ft_total_supply` - Get total supply
- `ft_balance_of` - Get account balance

**Metadata**:
- `ft_metadata` - Get token info

**Storage**:
- `storage_deposit` - Register account
- `storage_withdraw` - Withdraw deposit
- `storage_unregister` - Unregister account
- `storage_balance_bounds` - Get min balance
- `storage_balance_of` - Get storage balance

## Usage Examples

### View Functions (free)

```bash
# Total supply
near contract call-function as-read-only harshitnayan.testnet \
  ft_total_supply json-args {} network-config testnet now

# Balance
near contract call-function as-read-only harshitnayan.testnet \
  ft_balance_of json-args '{"account_id":"harshitnayan.testnet"}' \
  network-config testnet now

# Metadata
near contract call-function as-read-only harshitnayan.testnet \
  ft_metadata json-args {} network-config testnet now
```

### Transactions (require gas)

```bash
# Register account (0.00125 NEAR for storage)
near contract call-function as-transaction harshitnayan.testnet \
  storage_deposit json-args '{"account_id":"receiver.testnet"}' \
  prepaid-gas '30.0 Tgas' attached-deposit '0.00125 NEAR' \
  sign-as harshitnayan.testnet network-config testnet sign-with-keychain send

# Transfer tokens (1 yoctoNEAR for security)
near contract call-function as-transaction harshitnayan.testnet \
  ft_transfer json-args '{"receiver_id":"receiver.testnet","amount":"1000000000000000000000"}' \
  prepaid-gas '30.0 Tgas' attached-deposit '0.000000000000000000000001 NEAR' \
  sign-as harshitnayan.testnet network-config testnet sign-with-keychain send
```

## Cross-Chain Bridge Next Steps

**NEAR Side** ✅:
- NEP-141 token deployed
- Standard interface ready

**Filecoin Side** (To Do):
- Deploy token contract
- Implement lock mechanism

**Bridge Components** (To Do):
- Relayer service
- Oracle validation
- Lock/mint contracts
- Monitoring system

**Tools to Consider**:
- Rainbow Bridge (NEAR reference)
- Chainlink CCIP
- LayerZero
- Axelar

## Links

- [Explorer](https://explorer.testnet.near.org/accounts/harshitnayan.testnet)
- [NEP-141 Spec](https://nomicon.io/Standards/Tokens/FungibleToken/Core)
- [NEAR Docs](https://docs.near.org/)
- [Contract Source](near-contract/src/lib.rs)
