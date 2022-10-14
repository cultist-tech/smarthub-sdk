#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use crate::nft::reveal::NonFungibleTokenReveal;
    use crate::nft::*;
    use near_sdk::borsh::{ self, BorshSerialize };
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::{ accounts, VMContextBuilder };
    use near_sdk::{ env, testing_env, AccountId, Balance, BorshStorageKey };

    const ATTACHED_SUPPLY: Balance = 100_000_000_000_000_000_000_000;

    /// Helper structure for keys of the persistent collections.
    #[derive(BorshSerialize, BorshStorageKey)]
    pub enum StorageKey {
        OwnerById,
        TokenMetadata,
        Enumeration,
        Approval,
        NftRoyaltu,
        BindToOwner,
        RevealHiddenMeta,
        RevealTokens,
        RevealTime,
        // Extra info
        TokenRarity,
        TokenCollection,
        TokenType,
        TokenSubType,
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(predecessor_account_id.clone())
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn get_instance() -> NonFungibleToken {
        NonFungibleToken::new(
            StorageKey::OwnerById,
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
            StorageKey::NftRoyaltu,
            StorageKey::BindToOwner,
            StorageKey::RevealHiddenMeta,
            StorageKey::RevealTokens,
            StorageKey::RevealTime,
            Some(StorageKey::TokenRarity),
            Some(StorageKey::TokenCollection),
            Some(StorageKey::TokenType),
            Some(StorageKey::TokenSubType)
        )
    }

    #[test]
    fn test_reveal_mint() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context.attached_deposit(ATTACHED_SUPPLY).build());

        let mut instance = get_instance();

        let token_id = "Token1".to_string();
        let title = "Token1_title".to_string();
        let description = "Description".to_string();

        let token_metadata = TokenMetadata {
            title: Some(title),
            description: Some(description),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        let reveal_time = env::block_timestamp();

        let token = instance.nft_mint(
            token_id.clone(),
            None,
            token_metadata.clone(),
            None,
            None,
            Some(reveal_time),
            None,
            None,
            None,
            None
        );

        let token_hidden_id = format!("_r{}", token_id.clone());
        assert_eq!(token.token_id, token_hidden_id);

        assert_eq!(token.owner_id, owner_id);

        let token_hidden_title =
            format!("{} #{}", HIDDEN_TOKEN.to_string(), token_hidden_id.clone());
        assert_eq!(
            token.metadata.unwrap().title,
            Some(token_hidden_title)
        );

        assert_eq!(
            instance.token_hidden_metadata.contains(&token_metadata),
            true
        );
        assert_eq!(instance.tokens_to_reveal.contains(&token_hidden_id), true);

        if let Some(time) = instance.token_reveal_time_by_id.get(&token_id) {
            assert_eq!(time, reveal_time);
        }
    }

    #[test]
    #[should_panic(expected = "Token is too early to reveal")]
    fn test_reveal_nft_early() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context.attached_deposit(ATTACHED_SUPPLY).build());

        let mut instance = get_instance();

        let token_id = "Token1".to_string();
        let title = "Token1_title".to_string();
        let description = "Description".to_string();

        let token_metadata = TokenMetadata {
            title: Some(title),
            description: Some(description),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        let reveal_time = env::block_timestamp() + 100;

        instance.nft_mint(
            token_id.clone(),
            None,
            token_metadata.clone(),
            None,
            None,
            Some(reveal_time),
            None,
            None,
            None,
            None
        );

        let mut context = get_context(owner_id.clone());
        context.context.block_timestamp = reveal_time - 50;
        testing_env!(context.attached_deposit(1).build());

        let token_hidden_id = format!("_r{}", token_id.clone());

        instance.nft_reveal(token_hidden_id.clone());
    }

    #[test]
    fn test_reveal_nft() {
        let owner_id = accounts(1);

        let mut context = get_context(owner_id.clone());
        testing_env!(context.attached_deposit(ATTACHED_SUPPLY).build());

        let mut instance = get_instance();

        let token_id = "Token1".to_string();
        let title = "Token1_title".to_string();
        let description = "Description".to_string();

        let token_metadata = TokenMetadata {
            title: Some(title),
            description: Some(description),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };

        let reveal_time = env::block_timestamp() + 100;

        instance.nft_mint(
            token_id.clone(),
            None,
            token_metadata.clone(),
            None,
            None,
            Some(reveal_time),
            None,
            None,
            None,
            None
        );

        let mut context = get_context(owner_id.clone());
        context.context.block_timestamp = reveal_time;
        testing_env!(context.attached_deposit(1).build());

        let token_hidden_id = format!("_r{}", token_id.clone());

        instance.nft_reveal(token_hidden_id.clone());

        if let Some(metadata) = instance.token_metadata_by_id.unwrap().get(&token_id) {
            assert_eq!(metadata, token_metadata);
        }

        assert_eq!(
            instance.token_hidden_metadata.contains(&token_metadata),
            false
        );
        assert_eq!(instance.tokens_to_reveal.contains(&token_id), false);
        assert_eq!(
            instance.token_reveal_time_by_id.get(&token_id).is_some(),
            false
        );
    }
}