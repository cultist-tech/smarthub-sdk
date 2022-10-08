#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::rent::{RentFeature};
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{AccountId, BorshStorageKey, Balance, testing_env, env};
    use near_sdk::json_types::U128;
    use near_sdk::borsh::{self, BorshSerialize};
    use crate::rent::base::RentFeatureCore;
    use crate::rent::RentFeatureEnumeration;
    use crate::rent::utils::{contract_token_id};
    use std::collections::HashMap;

    const PRICE_PER_HOUR: U128 = U128(10_000_000_000_000_000_000_000);

    const ATTACHED_SUPPLY: Balance = 100_000_000_000_000_000_000_000;
    const MIN_TIME: u64 = 3700000000000;
    const MAX_TIME: u64 = 8540000000000000;

    /// Helper structure for keys of the persistent collections.
    #[derive(BorshSerialize, BorshStorageKey)]
    pub enum StorageKey {
        RentsCurrent,
        RentsPending,
        RentTokensPerAccount,
        RentsById,
        RentsPerAccount,
        RentsAt,
        RentContractTokens,
        ApprovedOwners
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_instance() -> RentFeature {
        RentFeature::new(
          Some(StorageKey::ApprovedOwners),
          StorageKey::RentsCurrent,
          StorageKey::RentsPending,
          StorageKey::RentsById,
          StorageKey::RentTokensPerAccount,
          StorageKey::RentsPerAccount,
          StorageKey::RentsAt,
          StorageKey::RentContractTokens,
        )
    }

    #[test]
    fn test_rent_add_internal() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id, PRICE_PER_HOUR);


        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id = contract_token_id(&nft_contract_id , &token_id);

        if let Some(owner_by_id) = instance.approved_owner_by_id{
            if let Some(owner) =owner_by_id.get(&id) {
                assert_eq!(owner, owner_id);
            }
        }

        assert_eq!(instance.rents_pending.contains(&id), true);

        if let Some(rent) = instance.rents_by_id.get(&id) {
            assert_eq!(rent.owner_id, owner_id);
            assert_eq!(rent.contract_id, nft_contract_id);
            assert_eq!(rent.token_id, token_id);
            assert_eq!(rent.min_time, MIN_TIME);
            assert_eq!(rent.max_time, MAX_TIME);
            assert_eq!(rent.created_at, env::block_timestamp());
            assert_eq!(rent.sale_conditions, sale_conditions);
        }

        if let Some(set) = instance.rents_per_account.get(&owner_id){
            assert_eq!(set.contains(&id), true);
        }

         if let Some(set) = instance.rent_tokens_by_contract.get(&nft_contract_id){
            assert_eq!(set.contains(&token_id), true);
        }
    }

    #[test]
    fn test_rent_update() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

        let add_date = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let ft_token_id_new = AccountId::new_unchecked("ft_token2".to_string());
        let min_time_new: u64 = MIN_TIME + 1_000_000;
        let max_time_new: u64 = MAX_TIME - 10_000_000;
        let price_per_hour_new = U128(1_000_000_000_000_000_000_000);

        instance.rent_update(
            nft_contract_id.clone(),
            token_id.clone(),
            &ft_token_id_new,
            price_per_hour_new,
            min_time_new,
            max_time_new
        );

        let id = contract_token_id(&nft_contract_id , &token_id);

        assert_eq!(instance.rents_pending.contains(&id), true);

        if let Some(rent) = instance.rents_by_id.get(&id) {
            assert_eq!(rent.owner_id, owner_id);
            assert_eq!(rent.contract_id, nft_contract_id);
            assert_eq!(rent.token_id, token_id);
            assert_eq!(rent.min_time, min_time_new);
            assert_eq!(rent.max_time, max_time_new);
            assert_eq!(rent.created_at, add_date);
            assert_eq!(rent.sale_conditions.get(&ft_token_id), Some(&PRICE_PER_HOUR));
            assert_eq!(rent.sale_conditions.get(&ft_token_id_new), Some(&price_per_hour_new));
        }
    }

    #[test]
    fn test_rent_remove() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

       instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.rent_remove(
            nft_contract_id.clone(),
            token_id.clone(),
        );

        let id = contract_token_id(&nft_contract_id , &token_id);

        assert_eq!(instance.rents_by_id.contains_key(&id), false);
        assert_eq!(instance.rents_pending.contains(&id), false);

        if let Some(set) = instance.rents_per_account.get(&owner_id){
            assert_eq!(set.contains(&id), false);
        } else {
            assert_eq!(instance.rents_per_account.contains_key(&owner_id), false);
        }

         if let Some(set) = instance.rent_tokens_by_contract.get(&nft_contract_id){
            assert_eq!(set.contains(&token_id), false);
        } else {
            assert_eq!(instance.rent_tokens_by_contract.contains_key(&nft_contract_id), false);
        }
    }

    #[test]
    fn test_rent_token_is_locked() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

       instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let is_locked = instance.rent_token_is_locked(
            nft_contract_id,
            token_id,
        );

        assert_eq!(is_locked, false);
    }

    #[test]
    fn test_rent_is_ended_internal() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

       instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );


        let end_time = env::block_timestamp() + 1_000;

        let id = contract_token_id(&nft_contract_id , &token_id);

        instance.rents_end_by_id.insert(&id, &end_time);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let is_ended = instance.rent_is_ended(
            nft_contract_id,
            token_id,
        );

        assert_eq!(is_ended, false);
    }

    #[test]
    fn test_rent_total_supply() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

       instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let supply = instance.rent_total_supply();

        assert_eq!(supply, 1);
    }

    #[test]
    fn test_rent_is_approved() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let approved = instance.rent_is_approved(
            nft_contract_id,
            token_id,
            owner_id,
        );

        assert_eq!(approved, true);
    }

    #[test]
    fn test_enum_rents() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date1 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);
        let date2 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let json_rent = instance.rents(Some(U128(0)),Some(5));

        assert_eq!(json_rent[0].token_id, token_id);
        assert_eq!(json_rent[0].contract_id, nft_contract_id);
        assert_eq!(json_rent[0].owner_id, owner_id);
        assert_eq!(json_rent[0].sale_conditions, sale_conditions);
        assert_eq!(json_rent[0].min_time, MIN_TIME);
        assert_eq!(json_rent[0].max_time, MAX_TIME);
        assert_eq!(json_rent[0].ended_at, None);
        assert_eq!(json_rent[0].renter_id, None);
        assert_eq!(json_rent[0].created_at, date1);

        assert_eq!(json_rent[1].token_id, token_id2);
        assert_eq!(json_rent[1].contract_id, nft_contract_id);
        assert_eq!(json_rent[1].owner_id, owner_id);
        assert_eq!(json_rent[1].sale_conditions, sale_conditions2);
        assert_eq!(json_rent[1].min_time, MIN_TIME);
        assert_eq!(json_rent[1].max_time, MAX_TIME);
        assert_eq!(json_rent[1].ended_at, None);
        assert_eq!(json_rent[1].renter_id, None);
        assert_eq!(json_rent[1].created_at, date2);
    }

    #[test]
    fn test_enum_rents_for_account() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date1 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);
        let date2 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let json_rent= instance.rents_for_account(owner_id.clone(), Some(U128(0)), Some(2));

        assert_eq!(json_rent[0].token_id, token_id);
        assert_eq!(json_rent[0].contract_id, nft_contract_id);
        assert_eq!(json_rent[0].owner_id, owner_id);
        assert_eq!(json_rent[0].sale_conditions, sale_conditions);
        assert_eq!(json_rent[0].min_time, MIN_TIME);
        assert_eq!(json_rent[0].max_time, MAX_TIME);
        assert_eq!(json_rent[0].ended_at, None);
        assert_eq!(json_rent[0].renter_id, None);
        assert_eq!(json_rent[0].created_at, date1);

        assert_eq!(json_rent[1].token_id, token_id2);
        assert_eq!(json_rent[1].contract_id, nft_contract_id);
        assert_eq!(json_rent[1].owner_id, owner_id);
        assert_eq!(json_rent[1].sale_conditions, sale_conditions2);
        assert_eq!(json_rent[1].min_time, MIN_TIME);
        assert_eq!(json_rent[1].max_time, MAX_TIME);
        assert_eq!(json_rent[1].ended_at, None);
        assert_eq!(json_rent[1].renter_id, None);
        assert_eq!(json_rent[1].created_at, date2);
    }

    #[test]
    fn test_enum_rents_by_ids() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date1 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id1 = contract_token_id(&nft_contract_id , &token_id);

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);
        let date2 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id2 = contract_token_id(&nft_contract_id , &token_id2);

        let token_vec = vec![id1, id2];

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let json_rent= instance.rents_by_ids(token_vec);

        assert_eq!(json_rent[0].token_id, token_id);
        assert_eq!(json_rent[0].contract_id, nft_contract_id);
        assert_eq!(json_rent[0].owner_id, owner_id);
        assert_eq!(json_rent[0].sale_conditions, sale_conditions);
        assert_eq!(json_rent[0].min_time, MIN_TIME);
        assert_eq!(json_rent[0].max_time, MAX_TIME);
        assert_eq!(json_rent[0].ended_at, None);
        assert_eq!(json_rent[0].renter_id, None);
        assert_eq!(json_rent[0].created_at, date1);

        assert_eq!(json_rent[1].token_id, token_id2);
        assert_eq!(json_rent[1].contract_id, nft_contract_id);
        assert_eq!(json_rent[1].owner_id, owner_id);
        assert_eq!(json_rent[1].sale_conditions, sale_conditions2);
        assert_eq!(json_rent[1].min_time, MIN_TIME);
        assert_eq!(json_rent[1].max_time, MAX_TIME);
        assert_eq!(json_rent[1].ended_at, None);
        assert_eq!(json_rent[1].renter_id, None);
        assert_eq!(json_rent[1].created_at, date2);
    }

    #[test]
    fn test_enum_rents_supply_for_account() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let supply= instance.rents_supply_for_account(owner_id);

        assert_eq!(supply, U128(2));
    }

     #[test]
    fn test_enum_rent() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        if let Some(json_rent)= instance.rent(
            nft_contract_id.clone(),
            token_id.clone()
        ) {
            assert_eq!(json_rent.token_id, token_id);
            assert_eq!(json_rent.contract_id, nft_contract_id);
            assert_eq!(json_rent.owner_id, owner_id);
            assert_eq!(json_rent.sale_conditions, sale_conditions);
            assert_eq!(json_rent.min_time, MIN_TIME);
            assert_eq!(json_rent.max_time, MAX_TIME);
            assert_eq!(json_rent.ended_at, None);
            assert_eq!(json_rent.renter_id, None);
            assert_eq!(json_rent.created_at, date);
          }
    }

    #[test]
    fn test_enum_rented_tokens_for_account() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date1 = env::block_timestamp();
        let end_time1 = date1 + 1_000_000;

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let receiver_id = accounts(2);

        let id = contract_token_id(&nft_contract_id , &token_id);
        instance.internal_add_token_to_account(&receiver_id, &nft_contract_id, &token_id);
        instance.rents_end_by_id.insert(&id, &end_time1);

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);
        let date2 = env::block_timestamp();
        let end_time2 = date1 + 1_000_000;

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id2 = contract_token_id(&nft_contract_id , &token_id2);
        instance.internal_add_token_to_account(&receiver_id, &nft_contract_id, &token_id2);
        instance.rents_end_by_id.insert(&id2, &end_time2);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let json_rent = instance.rented_tokens_for_account(receiver_id.clone(), Some(U128(0)), Some(5));

        assert_eq!(json_rent[0].token_id, token_id);
        assert_eq!(json_rent[0].contract_id, nft_contract_id);
        assert_eq!(json_rent[0].owner_id, owner_id);
        assert_eq!(json_rent[0].sale_conditions, sale_conditions);
        assert_eq!(json_rent[0].min_time, MIN_TIME);
        assert_eq!(json_rent[0].max_time, MAX_TIME);
        assert_eq!(json_rent[0].ended_at, Some(end_time1));
        assert_eq!(json_rent[0].renter_id, None);
        assert_eq!(json_rent[0].created_at, date1);

        assert_eq!(json_rent[1].token_id, token_id2);
        assert_eq!(json_rent[1].contract_id, nft_contract_id);
        assert_eq!(json_rent[1].owner_id, owner_id);
        assert_eq!(json_rent[1].sale_conditions, sale_conditions2);
        assert_eq!(json_rent[1].min_time, MIN_TIME);
        assert_eq!(json_rent[1].max_time, MAX_TIME);
        assert_eq!(json_rent[1].ended_at, Some(end_time2));
        assert_eq!(json_rent[1].renter_id, None);
        assert_eq!(json_rent[1].created_at, date2);
    }

    #[test]
    fn test_enum_rented_tokens_supply_for_account() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

        let end_time = env::block_timestamp() + 1_000_000;

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id = contract_token_id(&nft_contract_id , &token_id);
        let receiver_id = accounts(2);
        instance.internal_add_token_to_account(&receiver_id, &nft_contract_id, &token_id);
        instance.rents_end_by_id.insert(&id, &end_time);

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let id2 = contract_token_id(&nft_contract_id , &token_id2);
        instance.internal_add_token_to_account(&receiver_id, &nft_contract_id, &token_id2);
        instance.rents_end_by_id.insert(&id2, &end_time);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let supply= instance.rented_tokens_supply_for_account(receiver_id.clone());

        assert_eq!(supply, U128(2));
    }

    #[test]
    fn test_enum_rents_by_contract() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);
        let date1 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);
        let date2 = env::block_timestamp();

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let json_rent= instance.rents_by_contract(nft_contract_id.clone(), Some(U128(0)), Some(5));

        assert_eq!(json_rent[0].token_id, token_id);
        assert_eq!(json_rent[0].contract_id, nft_contract_id);
        assert_eq!(json_rent[0].owner_id, owner_id);
        assert_eq!(json_rent[0].sale_conditions, sale_conditions);
        assert_eq!(json_rent[0].min_time, MIN_TIME);
        assert_eq!(json_rent[0].max_time, MAX_TIME);
        assert_eq!(json_rent[0].ended_at, None);
        assert_eq!(json_rent[0].renter_id, None);
        assert_eq!(json_rent[0].created_at, date1);

        assert_eq!(json_rent[1].token_id, token_id2);
        assert_eq!(json_rent[1].contract_id, nft_contract_id);
        assert_eq!(json_rent[1].owner_id, owner_id);
        assert_eq!(json_rent[1].sale_conditions, sale_conditions2);
        assert_eq!(json_rent[1].min_time, MIN_TIME);
        assert_eq!(json_rent[1].max_time, MAX_TIME);
        assert_eq!(json_rent[1].ended_at, None);
        assert_eq!(json_rent[1].renter_id, None);
        assert_eq!(json_rent[1].created_at, date2);
    }

    #[test]
    fn test_enum_rents_supply_by_contract() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let nft_contract_id = AccountId::new_unchecked("nft_token_contract".to_string());

        let token_id = "Token1".to_string();

        let ft_token_id = AccountId::new_unchecked("ft_token".to_string());
        let mut sale_conditions = HashMap::new();
        sale_conditions.insert(ft_token_id.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id,
            &owner_id,
            &sale_conditions,
            &MIN_TIME,
            &MAX_TIME,
        );

        let token_id2 = "Token2".to_string();
        let ft_token_id2 = AccountId::new_unchecked("ft_token2".to_string());
        let mut sale_conditions2 = HashMap::new();
        sale_conditions2.insert(ft_token_id2.clone(), PRICE_PER_HOUR);

        instance.internal_rent_add(
            &nft_contract_id,
            &token_id2,
            &owner_id,
            &sale_conditions2,
            &MIN_TIME,
            &MAX_TIME,
        );

        let mut context = get_context(owner_id.clone());
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let supply= instance.rents_supply_by_contract(nft_contract_id.clone());

        assert_eq!(supply, U128(2));
    }
}
