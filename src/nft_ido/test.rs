#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::nft_ido::NftIdoFeature;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{AccountId, BorshStorageKey, env, Balance, testing_env};
    use near_sdk::json_types::U128;
    use near_sdk::borsh::{self, BorshSerialize};
    use crate::nft_ido::base::IdoCore;
    use crate::nft_ido::utils::contract_token_id;

    const AMOUNT: u64 = 1;
    const PRICE: U128 = U128(10_000_000_000_000_000_000_000);
    const PER_TRANSACTION_MIN: u64 = 1;
    const PER_TRANSACTION_MAX: u64 = 1;
    const BUY_MAX: u64 = 1;
    const VALID_DATE: u64 = 1663513608939000001;

    const ATTACHED_SUPPLY: Balance = 100_000_000_000_000_000_000_000;

    /// Helper structure to for keys of the persistent collections.
    #[derive(BorshStorageKey, BorshSerialize)]
    pub enum StorageKey {
        IdoByToken,
        IdoTokens,
        IdoRandomTokens,
        IdoMintCounter,
        IdosAvailable,
        IdoByFt,
        IdoTokensByContract,
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_instance() -> NftIdoFeature {
        NftIdoFeature::new(
            StorageKey::IdoByToken,
            StorageKey::IdoTokens,
            StorageKey::IdoRandomTokens,
            StorageKey::IdoMintCounter,
            StorageKey::IdosAvailable,
            StorageKey::IdoByFt,
            StorageKey::IdoTokensByContract,
        )
    }

    #[test]
    fn test_nft_ido_add() {
        let mut context = get_context(accounts(1));
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let mut instance = get_instance();

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let ft_token = Some(accounts(3));
        let media = Some("Media".to_string());

        let json_ido = instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            ft_token.clone(),
            media.clone()
        );
        let id = contract_token_id(&contract_id, &ido_id);

        if let Some(ido) = instance.ido_by_id.get(&id){
            assert_eq!(ido.ido_id, ido_id);
            assert_eq!(ido.contract_id, contract_id);
            assert_eq!(ido.name, name);
            assert_eq!(ido.amount, AMOUNT);
            assert_eq!(ido.price, PRICE);
            assert_eq!(ido.per_transaction_min, PER_TRANSACTION_MIN);
            assert_eq!(ido.per_transaction_max, PER_TRANSACTION_MAX);
            assert_eq!(ido.buy_max, BUY_MAX);
            assert_eq!(ido.media, media);
        }

        assert_eq!(instance.ido_by_ft_token.get(&id), ft_token);

        if let Some(set) = instance.ido_tokens_by_contract.get(&contract_id){
            assert_eq!(set.contains(&ido_id), true);
        }

        assert_eq!(json_ido.locked, true);
        assert_eq!(json_ido.start_date, None);
        assert_eq!(json_ido.not_minted, AMOUNT);
    }

    #[test]
    fn test_nft_ido_start() {
        let mut context = get_context(accounts(1));
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let ft_token = Some(accounts(3));
        let media = Some("Media".to_string());

        let mut instance = get_instance();

        instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            ft_token.clone(),
            media
        );

        let nft = "NFT1".to_string();

        instance.internal_ido_add_token(
            &contract_id,
            &ido_id,
            &nft
        );

        let start_date = VALID_DATE + 100;


        let json_idostarted = instance.nft_ido_start(
            contract_id.clone(),
            ido_id.clone(),
            start_date,
        );



        assert_eq!(json_idostarted.locked, false);
        assert_eq!(json_idostarted.start_date, Some(start_date));
        assert_eq!(json_idostarted.not_minted, AMOUNT);
    }

    #[test]
    #[should_panic(expected = "Ido is already started")]
    fn test_nft_ido_update_late() {
        let owner_id = accounts(1);
        let mut context = get_context(owner_id);
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let ft_token = Some(accounts(3));
        let media = Some("Media".to_string());

        let mut instance = get_instance();

        instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            ft_token.clone(),
            media
        );

        let nft = "NFT1".to_string();

        instance.internal_ido_add_token(
            &contract_id,
            &ido_id,
            &nft
        );

        let start_date = VALID_DATE;

        instance.nft_ido_start(
            contract_id.clone(),
            ido_id.clone(),
            start_date,
        );

        let later = start_date + 1_000_000_000;

        let owner_id = accounts(1);
        let mut context = get_context(owner_id);
        context.context.block_timestamp = VALID_DATE;
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        instance.nft_ido_update(
             contract_id.clone(),
             ido_id.clone(),
             later,
             PER_TRANSACTION_MIN,
             PER_TRANSACTION_MAX,
             BUY_MAX
        );
    }

    #[test]
    fn test_nft_ido_update() {
        let mut context = get_context(accounts(1));
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let ft_token = Some(accounts(3));
        let media = Some("Media".to_string());

        let mut instance = get_instance();

        instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            ft_token.clone(),
            media.clone()
        );

        let nft = "NFT1".to_string();

        instance.internal_ido_add_token(
            &contract_id,
            &ido_id,
            &nft
        );

        let start_date = VALID_DATE + 1_000_000;

        instance.nft_ido_start(
             contract_id.clone(),
             ido_id.clone(),
             start_date,
        );

        let after_start = start_date + 50;

        let buy_max_upd = BUY_MAX + 1;

        let json_ido_updated = instance.nft_ido_update(
             contract_id.clone(),
             ido_id.clone(),
             after_start,
             PER_TRANSACTION_MIN,
             PER_TRANSACTION_MAX,
             buy_max_upd
        );

        assert_eq!(json_ido_updated.start_date, Some(after_start));

        let id = contract_token_id(&contract_id, &ido_id);

        if let Some(ido) = instance.ido_by_id.get(&id){
            assert_eq!(ido.ido_id, ido_id);
            assert_eq!(ido.contract_id, contract_id);
            assert_eq!(ido.name, name);
            assert_eq!(ido.amount, AMOUNT);
            assert_eq!(ido.price, PRICE);
            assert_eq!(ido.per_transaction_min, PER_TRANSACTION_MIN);
            assert_eq!(ido.per_transaction_max, PER_TRANSACTION_MAX);
            assert_eq!(ido.buy_max, buy_max_upd);
            assert_eq!(ido.media, media);
        }
    }

    #[test]
    fn test_nft_ido_pause() {
        let mut context = get_context(accounts(1));
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let ft_token = Some(accounts(3));
        let media = Some("Media".to_string());

        let mut instance = get_instance();

        instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            ft_token.clone(),
            media
        );

        let nft = "NFT1".to_string();

        instance.internal_ido_add_token(
            &contract_id,
            &ido_id,
            &nft
        );

        let start_date = VALID_DATE;

        instance.nft_ido_start(
             contract_id.clone(),
             ido_id.clone(),
             start_date,
        );

        let mut pause = true;

        instance.nft_ido_pause(
            contract_id.clone(),
            ido_id.clone(),
            pause
        );

        let id = contract_token_id(&contract_id, &ido_id);

        assert_eq!(instance.idos_available.contains(&id), false);

        pause = false;

        instance.nft_ido_pause(
            contract_id.clone(),
            ido_id.clone(),
            pause
        );

        assert_eq!(instance.idos_available.contains(&id), true);
    }

    #[test]
    fn test_nft_ido_buy_internal() {
        let mut context = get_context(accounts(1));
        testing_env!(context
            .attached_deposit(ATTACHED_SUPPLY)
            .build()
        );

        let contract_id = accounts(1);
        let ido_id = "IDO1".to_string();
        let name = "IDO1_name".to_string();
        let media = Some("Media".to_string());

        let mut instance = get_instance();

        instance.nft_ido_add(
            contract_id.clone(),
            ido_id.clone(),
            name.clone(),
            AMOUNT,
            PRICE,
            PER_TRANSACTION_MIN,
            PER_TRANSACTION_MAX,
            BUY_MAX,
            None,
            media
        );

        let start_date = VALID_DATE;

        let nft = "NFT1".to_string();

        instance.internal_ido_add_token(
            &contract_id,
            &ido_id,
            &nft
        );

        instance.nft_ido_start(
             contract_id.clone(),
             ido_id.clone(),
             start_date,
        );
        let receiver_id = accounts(2);
        let id = contract_token_id(&contract_id, &ido_id);

        if let Some(set) = instance.ido_tokens.get(&id){
            assert_eq!(set.contains(&nft), true);
        }
        assert_eq!(instance.ido_by_token.get(&nft), Some(id.clone()));

        assert_eq!(instance.internal_mint_counter_by_ido(&receiver_id, &id), 0);
        assert_eq!(instance.internal_random_tokens(&contract_id, &ido_id, &AMOUNT)[0], nft);
    }
}
