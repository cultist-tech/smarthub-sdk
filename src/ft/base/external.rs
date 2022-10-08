use near_sdk::{ ext_contract, AccountId, PromiseOrValue };
use near_sdk::json_types::U128;

#[ext_contract(ext_ft)]
pub trait ExtFtReceiver {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String
    ) -> PromiseOrValue<U128>;

    /// Returns the total supply of the token in a decimal string representation.
    fn ft_total_supply(&self) -> U128;

    /// Returns the balance of the account. If the account doesn't exist, `"0"` must be returned.
    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}