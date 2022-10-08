use crate::nft::burn::{ NonFungibleTokenBurnable };
use crate::nft::{ TokenId, NonFungibleToken };
use near_sdk::{ env };

impl NonFungibleTokenBurnable for NonFungibleToken {
    fn nft_burn(&mut self, token_id: &TokenId) {
        let sender_id = env::predecessor_account_id();

        self.internal_burn_token(&sender_id, &token_id);
    }
}
