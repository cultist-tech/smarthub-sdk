use crate::nft::reveal::NonFungibleTokenReveal;
use crate::nft::{ NonFungibleToken, TokenId };
use near_sdk::{ env, assert_one_yocto };

impl NonFungibleTokenReveal for NonFungibleToken {
    fn nft_reveal(&mut self, token_id: TokenId) {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();

        self.internal_reveal_token(&sender_id, &token_id);
    }
}
