use near_sdk::{ json_types::U128, AccountId };

pub trait MultiFungibleTokenResolver {
    fn mt_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_ids: Vec<AccountId>,
        amounts: Vec<U128>
    ) -> Vec<U128>;
}