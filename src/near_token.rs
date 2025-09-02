use near_contract_standards::fungible_token::core::FungibleTokenCore;
use near_contract_standards::fungible_token::FungibleTokenResolver;
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::storage_management::{StorageManagement, StorageBalance, StorageBalanceBounds};
use near_sdk::{near, AccountId, NearToken, PromiseOrValue};
use near_sdk::json_types::U128;

#[near(contract_state)]
pub struct MediTokenNEP141 {
    token: FungibleToken,
    metadata: FungibleTokenMetadata,
}

#[near]
impl MediTokenNEP141 {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        let mut contract = Self {
            token: FungibleToken::new(b"t".to_vec()),
            metadata: FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "MediToken".to_string(),
                symbol: "MEDT".to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals: 18,
            },
        };
        contract.token.internal_register_account(&owner_id);
        contract.token.internal_deposit(&owner_id, total_supply.0);
        contract
    }
}

#[near]
impl FungibleTokenCore for MediTokenNEP141 {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        self.token.ft_transfer(receiver_id, amount, memo)
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.token.ft_transfer_call(receiver_id, amount, memo, msg)
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenResolver for MediTokenNEP141 {
    #[private]
    fn ft_resolve_transfer(&mut self, owner_id: AccountId, receiver_id: AccountId, amount: U128) -> U128 {
        self.token.ft_resolve_transfer(owner_id, receiver_id, amount)
    }
}

#[near]
impl FungibleTokenMetadataProvider for MediTokenNEP141 {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.clone()
    }
}

#[near]
impl StorageManagement for MediTokenNEP141 {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        self.token.storage_withdraw(amount)
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.token.internal_storage_unregister(force).is_some()
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.token.storage_balance_of(account_id)
    }
}
