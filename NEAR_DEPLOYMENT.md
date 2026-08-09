# MediToken — NEAR Testnet Deployment Guide

## Deployment Info

| Field | Value |
|---|---|
| **Contract Account** | `0xanurag.testnet` |
| **Network** | NEAR Testnet |
| **Token Name** | MediToken |
| **Symbol** | MEDT |
| **Standard** | NEP-141 Fungible Token |
| **Total Supply** | 1,000,000 MEDT |
| **Decimals** | 18 |
| **Transaction ID** | `7rBECmhopf7Czq6meYPx6gAemV1pExrCiphnHc8gBJ8B` |
| **Explorer** | https://testnet.nearblocks.io/token/0xanurag.testnet |

---

## References

- [NEAR Docs — Fungible Tokens](https://docs.near.org/tutorials/fts/introduction)
- [NEP-141 Standard Spec](https://nomicon.io/Standards/Tokens/FungibleToken/Core)
- [NEP-148 Metadata Spec](https://nomicon.io/Standards/Tokens/FungibleToken/Metadata)
- [cargo-near Docs](https://github.com/near/cargo-near)
- [near-cli-rs Docs](https://github.com/near/near-cli-rs)
- [NearBlocks Testnet Explorer](https://testnet.nearblocks.io)

---

## Step-by-Step Deployment Process

### Step 1 — Install Rust and set toolchain

Following the [NEAR Rust SDK setup guide](https://docs.near.org/sdk/rust/get-started):

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WASM target (NEAR compiles contracts to WebAssembly)
rustup target add wasm32-unknown-unknown
```

The repo pins Rust to `1.95.0` via `rust-toolchain.toml`:
```toml
[toolchain]
channel = "1.95.0"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

---

### Step 2 — Install cargo-near (NEAR build tool)

[cargo-near](https://github.com/near/cargo-near) is NEAR's official build tool. It compiles the contract to WASM, generates the ABI, and optimizes the binary size using `wasm-opt`.

```bash
cargo install cargo-near
```

---

### Step 3 — Install near-cli-rs (NEAR CLI)

The repo uses [near-cli-rs](https://github.com/near/near-cli-rs) — the Rust-based NEAR CLI. It uses the newer `near contract deploy ...` command syntax. The older JavaScript-based `near-cli` uses different syntax and is not compatible with the commands in this repo.

```bash
cargo install near-cli-rs
```

> Note: Do NOT use `npm install -g near-cli` — that is the old JS-based CLI with
> different command syntax and requires native build dependencies.

---

### Step 4 — Create a NEAR Testnet account

A funded testnet account is required to pay for storage and gas fees.

1. Go to **https://testnet.mynearwallet.com**
2. Click **Create Account**
3. Choose an account name — it becomes `yourname.testnet`
4. Complete the setup — NEAR automatically provides free testnet tokens

> Account created: `0xanurag.testnet` with ~10 NEAR testnet balance

---

### Step 5 — Import account into CLI

The CLI needs a local access key linked to the on-chain account to sign transactions.

```bash
near account import-account using-web-wallet network-config testnet
```

This command:
1. Generates a new key pair locally
2. Opens the NEAR wallet in the browser
3. The wallet adds the CLI's public key as a **full access key** on-chain
4. The private key is saved to `~/.near-credentials/testnet/yourname.testnet.json`

> Key saved to: `/home/anurag/.near-credentials/testnet/0xanurag.testnet.json`

---

### Step 6 — Fix near-sdk compatibility (near-sdk 5.x migration)

The original contract used the old near-sdk 4.x API. `cargo-near` resolved
`near-sdk = "5.5.0"` to `5.29.0` which introduced breaking changes.

**Problem:** `ContractState` trait not satisfied

**Root cause:** near-sdk 5.x replaced `#[near_bindgen] + #[derive(BorshDeserialize, BorshSerialize)]`
with a new `#[near(contract_state)]` macro.

Per the [near-sdk 5.x migration guide](https://docs.near.org/sdk/rust/near-bindgen/near-sdk-5):

```rust
// Old API (near-sdk 4.x) — no longer works
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MediTokenNEP141 { ... }

// New API (near-sdk 5.x) — handles borsh + ContractState automatically
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct MediTokenNEP141 { ... }
```

---

### Step 7 — Build the contract

```bash
cd near-contract
cargo near build non-reproducible-wasm
```

`cargo-near` performs the following steps:
1. Collects Cargo project metadata
2. Checks Rust version compatibility with NEAR protocol
3. Generates the ABI (Application Binary Interface) schema
4. Compiles to `wasm32-unknown-unknown` target with release optimizations
5. Runs `wasm-opt` to minimize binary size

**Build output:**
```
✓ Contract successfully built!
   Binary:   near-contract/target/near/medi_token_near.wasm
   Size:     111.9 KB
   SHA-256:  bd90b8eeb2c9c1c0e135a49bfe53ca4b108e9fbdc8eda5c533068fa635cbd768
   ABI:      near-contract/target/near/medi_token_near_abi.json
```

---

### Step 8 — Deploy the contract

Per the [NEAR deployment docs](https://docs.near.org/tools/near-cli#near-deploy),
deployment uploads the WASM binary to the account and calls the `new` initializer
in a single atomic transaction.

```bash
near contract deploy 0xanurag.testnet \
  use-file near-contract/target/near/medi_token_near.wasm \
  with-init-call new \
  json-args '{"owner_id":"0xanurag.testnet","total_supply":"1000000000000000000000000"}' \
  prepaid-gas '100.0 Tgas' \
  attached-deposit '0 NEAR' \
  network-config testnet \
  sign-with-keychain \
  send
```

**Arguments explained:**
| Argument | Value | Description |
|---|---|---|
| `owner_id` | `0xanurag.testnet` | Account that receives the initial token supply |
| `total_supply` | `1000000000000000000000000` | 1,000,000 MEDT × 10^18 decimals |
| `prepaid-gas` | `100.0 Tgas` | Gas allocated for the init function call |
| `attached-deposit` | `0 NEAR` | No NEAR attached to the init call |

**Transaction result:**
```
Transaction ID:  7rBECmhopf7Czq6meYPx6gAemV1pExrCiphnHc8gBJ8B
Gas burned:      1.3 Tgas
Transaction fee: 0.0001255 NEAR
Status:          Contract code has been successfully deployed.
```

---

### Step 9 — Verify the deployment

```bash
# Check token metadata
near contract call-function as-read-only 0xanurag.testnet \
  ft_metadata json-args {} network-config testnet now

# Check owner balance
near contract call-function as-read-only 0xanurag.testnet \
  ft_balance_of \
  json-args '{"account_id":"0xanurag.testnet"}' \
  network-config testnet now

# Check total supply
near contract call-function as-read-only 0xanurag.testnet \
  ft_total_supply json-args {} network-config testnet now
```

---

## How to Transfer Tokens

On NEAR, the recipient must register a storage slot before receiving tokens.
This is required by the [NEP-145 Storage Management standard](https://nomicon.io/Standards/StorageManagement).

```bash
# Step 1 — Register recipient storage (~0.00125 NEAR deposit)
near contract call-function as-transaction 0xanurag.testnet \
  storage_deposit \
  json-args '{"account_id":"recipient.testnet"}' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0.00125 NEAR' \
  sign-as 0xanurag.testnet \
  network-config testnet sign-with-keychain send

# Step 2 — Transfer tokens (1 yoctoNEAR deposit required for security)
near contract call-function as-transaction 0xanurag.testnet \
  ft_transfer \
  json-args '{"receiver_id":"recipient.testnet","amount":"1000000000000000000"}' \
  prepaid-gas '30.0 Tgas' \
  attached-deposit '0.000000000000000000000001 NEAR' \
  sign-as 0xanurag.testnet \
  network-config testnet sign-with-keychain send
```

> The `1 yoctoNEAR` deposit on `ft_transfer` is a NEAR security mechanism —
> it prevents accidental calls and confirms the transaction is intentional.

---

## NEP-141 Functions Reference

| Function | Type | Description |
|---|---|---|
| `ft_transfer` | Transaction | Transfer tokens to a receiver |
| `ft_transfer_call` | Transaction | Transfer tokens and call a receiver contract |
| `ft_total_supply` | View | Get total token supply |
| `ft_balance_of` | View | Get balance of an account |
| `ft_metadata` | View | Get token name, symbol, decimals |
| `storage_deposit` | Transaction | Register an account for token storage |
| `storage_withdraw` | Transaction | Withdraw unused storage deposit |
| `storage_unregister` | Transaction | Unregister and get deposit back |

---

## Transaction Screenshot

<!-- Screenshot of the deployment transaction attached below -->

---

## Links

| Resource | URL |
|---|---|
| Contract on Explorer | https://testnet.nearblocks.io/address/0xanurag.testnet |
| Token Page | https://testnet.nearblocks.io/token/0xanurag.testnet |
| Deployment Transaction | https://explorer.testnet.near.org/transactions/7rBECmhopf7Czq6meYPx6gAemV1pExrCiphnHc8gBJ8B |
| NEP-141 Spec | https://nomicon.io/Standards/Tokens/FungibleToken/Core |
| NEAR Rust SDK Docs | https://docs.near.org/sdk/rust/get-started |
| near-cli-rs Docs | https://github.com/near/near-cli-rs |
| NearBlocks Testnet | https://testnet.nearblocks.io |
