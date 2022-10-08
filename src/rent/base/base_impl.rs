use near_sdk::{ AccountId, env, IntoStorageKey, ext_contract, Gas, Promise, is_promise_success };
use near_sdk::json_types::{ U128 };
use near_sdk::collections::{ LookupMap, UnorderedSet, TreeMap };
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use crate::rent::events::{ RentClaim };
use crate::rent::{ TokenId, contract_token_id, RentFeatureResolve, RentFeatureCore, Rent };
use crate::nft::base::external::ext_nft;
use crate::utils::near_ft;

pub const GAS_FOR_RENT_PAY: Gas = Gas(60_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);
const GAS_FOR_RENT_CLAIM: Gas = Gas(60_000_000_000_000);

#[ext_contract(ext_self)]
trait ExtSelf {
    fn rent_resolve_pay(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        owner_id: AccountId,
        receiver_id: AccountId,
        time: u64,
        end_time: u64,
        ft_token_id: AccountId,
        price: U128
    );
    fn rent_resolve_claim(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        owner_id: AccountId,
        renter_id: AccountId
    );
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RentFeature {
    // approved nft tokens
    pub approved_owner_by_id: Option<LookupMap<TokenId, AccountId>>,
    // paid rents
    pub rents_current: TreeMap<TokenId, AccountId>,
    // available rents for pay
    pub rents_pending: UnorderedSet<TokenId>,
    // rents info
    pub rents_by_id: TreeMap<TokenId, Rent>,
    // rents per account
    pub rents_per_account: TreeMap<AccountId, UnorderedSet<TokenId>>,
    // rents start time
    pub rents_end_by_id: LookupMap<TokenId, u64>,
    // rented nft tokens per account
    pub rent_tokens_per_account: LookupMap<AccountId, UnorderedSet<TokenId>>,
    //
    pub rent_tokens_by_contract: TreeMap<AccountId, UnorderedSet<TokenId>>,
}

impl RentFeature {
    pub fn new<R0, R1, R2, R3, R4, R5, R6, R7>(
        approved_owner_prefix: Option<R0>,
        rents_current_prefix: R1,
        rents_pending_prefix: R2,
        rents_by_id_prefix: R3,
        rent_tokens_per_account_prefix: R4,
        rents_per_account_prefix: R5,
        rents_at_prefix: R6,
        rent_tokens_by_contract_prefix: R7
    )
        -> Self
        where
            R0: IntoStorageKey,
            R1: IntoStorageKey,
            R2: IntoStorageKey,
            R3: IntoStorageKey,
            R4: IntoStorageKey,
            R5: IntoStorageKey,
            R6: IntoStorageKey,
            R7: IntoStorageKey
    {
        let this = Self {
            approved_owner_by_id: approved_owner_prefix.map(LookupMap::new),
            rents_current: TreeMap::new(rents_current_prefix),
            rents_pending: UnorderedSet::new(rents_pending_prefix),
            rents_by_id: TreeMap::new(rents_by_id_prefix),
            rent_tokens_per_account: LookupMap::new(rent_tokens_per_account_prefix),
            rents_per_account: TreeMap::new(rents_per_account_prefix),
            rents_end_by_id: LookupMap::new(rents_at_prefix),
            rent_tokens_by_contract: TreeMap::new(rent_tokens_by_contract_prefix),
        };

        this
    }
}

impl RentFeatureCore for RentFeature {
    fn rent_token_is_locked(&self, contract_id: AccountId, token_id: TokenId) -> bool {
        self.internal_rent_token_is_locked(&contract_id, &token_id)
    }

    fn rent_update(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        ft_token_id: &AccountId,
        price_per_hour: U128,
        min_time: u64,
        max_time: u64
    ) {
        let account_id = env::predecessor_account_id();

        self.assert_approved(&contract_id, &token_id);
        self.assert_valid_time(&min_time);
        self.assert_valid_time(&max_time);

        self.internal_rent_update(
            &contract_id,
            &token_id,
            &account_id,
            &ft_token_id,
            &price_per_hour,
            &min_time,
            &max_time
        );
    }

    fn rent_remove(&mut self, contract_id: AccountId, token_id: TokenId) {
        let account_id = env::predecessor_account_id();

        self.assert_approved(&contract_id, &token_id);
        self.internal_remove_pending_rent(&contract_id, &token_id, &account_id)
    }

    // #[payable]
    fn rent_pay(
        &mut self,
        contract_id: AccountId,
        token_id: TokenId,
        time: u64,
        receiver_id: AccountId
    ) -> Promise {
        let deposit = env::attached_deposit();

        self.internal_process_purchase(
            &contract_id,
            &token_id,
            &env::predecessor_account_id(),
            &receiver_id,
            &time,
            &near_ft(),
            &U128::from(deposit)
        )
    }

    fn rent_claim(&mut self, contract_id: AccountId, token_id: TokenId) -> Promise {
        let account_id = env::predecessor_account_id();
        let id = contract_token_id(&contract_id, &token_id);

        let rent = self.rents_by_id.get(&id).expect("Not found rent");

        assert_eq!(&rent.owner_id, &account_id, "Not authorized");

        let renter_id = self.rents_current.get(&id).expect("Not found renter");
        let is_ended = self.internal_rent_is_ended(&id);

        assert!(is_ended, "Rent is not expired");

        ext_nft
            ::ext(contract_id.clone())
            .with_static_gas(GAS_FOR_NFT_TRANSFER)
            .with_attached_deposit(1)
            .nft_transfer(rent.owner_id.clone(), token_id.clone(), None, None)
            .then(
                ext_self
                    ::ext(env::current_account_id())
                    .with_static_gas(env::prepaid_gas() - GAS_FOR_RENT_CLAIM)
                    .rent_resolve_claim(contract_id, token_id, account_id, renter_id)
            )
    }

    fn rent_is_ended(&self, contract_id: AccountId, token_id: TokenId) -> bool {
        let id = contract_token_id(&contract_id, &token_id);

        self.internal_rent_is_ended(&id)
    }

    fn rent_total_supply(&self) -> u64 {
        self.rents_pending.len()
    }

    fn rent_is_approved(
        &self,
        contract_id: AccountId,
        token_id: TokenId,
        account_id: AccountId
    ) -> bool {
        let id = contract_token_id(&contract_id, &token_id);

        let approve_id = self.approved_owner_by_id.as_ref().unwrap().get(&id);

        if let Some(approve_id) = approve_id {
            return account_id == approve_id;
        }

        false
    }
}
