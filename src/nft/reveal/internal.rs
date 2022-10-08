use crate::nft::events::{ NftReveal, NftMint, NftBurn };
use crate::nft::metadata::HIDDEN_TOKEN;
use crate::nft::{ NonFungibleToken, TokenId };
use crate::utils::random_use;
use near_sdk::{ env, AccountId };
use rand::Rng;

impl NonFungibleToken {
    pub fn internal_reveal_token(&mut self, sender_id: &AccountId, token_id: &TokenId) {
        self.assert_token_holder(&token_id);

        assert!(self.tokens_to_reveal.contains(token_id), "Token is not hidden");

        let meta_by_id = self.token_metadata_by_id.as_ref().unwrap();
        assert!(
            meta_by_id.get(token_id).unwrap().title ==
                Some(format!("{} #{}", HIDDEN_TOKEN.to_string(), token_id.to_string())),
            "Token meta is not hidden"
        );

        let reveal_time = self.token_reveal_time_by_id
            .get(&token_id)
            .expect("There is no reveal time for hidden token");
        assert!(reveal_time <= env::block_timestamp(), "Token is too early to reveal");

        self.internal_reveal_token_unguarded(&sender_id, &token_id);
    }

    pub fn internal_reveal_token_unguarded(&mut self, sender_id: &AccountId, token_id: &TokenId) {
        let new_id = &token_id[2..].to_string();

        self.token_metadata_by_id.as_mut().unwrap().remove(token_id);

        self.tokens_to_reveal.remove(token_id);

        let mut rnd = random_use();

        let rand_index = rnd.gen_range(0, self.token_hidden_metadata.len().clone());

        let revealed_meta = self.token_hidden_metadata.as_vector().get(rand_index).unwrap();

        self.token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &revealed_meta));

        self.token_hidden_metadata.remove(&revealed_meta);
        self.token_reveal_time_by_id.remove(token_id);

        self.internal_burn_token_unguarded(&sender_id, &token_id);
        self.internal_mint_nft(
            &new_id,
            Some(sender_id.clone()),
            Some(revealed_meta),
            None,
            None,
            None,

            None,
            None,
            None,
            None
        );
        // self.internal_mint_nft(
        //     &token_id,
        //     Some(sender_id.clone()),
        //     &revealed_meta
        // );

        (NftReveal {
            owner_id: &sender_id,
            token_ids: &[new_id],
            authorized_id: None,
            memo: None,
        }).emit();
    }
}
