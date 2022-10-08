pub(crate) mod core;
pub(crate) mod bounties;
pub(crate) mod delegation;
pub(crate) mod policy;
pub(crate) mod proposals;
pub(crate) mod types;
pub(crate) mod upgrade;
pub(crate) mod views;
mod macros;

pub use self::core::{ Dao };
pub use self::policy::{ VersionedPolicy, VotePolicy, Policy };
pub use self::proposals::{ ProposalInput, Proposal };
pub use self::views::{ ProposalOutput, BountyOutput };
pub use self::types::{ Action };
pub use self::upgrade::{ FactoryInfo };
pub use self::bounties::{ BountyClaim };
use near_sdk::json_types::{ U128, Base58CryptoHash, U64 };
use near_sdk::{ AccountId, PromiseOrValue, Promise, Balance };
use crate::dao::types::Config;

pub trait DaoCore {
    fn remove_blob(&mut self, hash: Base58CryptoHash) -> Promise;

    fn get_factory_info(&self) -> FactoryInfo;

    fn bounty_claim(&mut self, id: u64, deadline: U64);

    fn bounty_done(&mut self, id: u64, account_id: Option<AccountId>, description: String);

    fn bounty_giveup(&mut self, id: u64) -> PromiseOrValue<()>;
    fn get_user_weight(&self, account_id: &AccountId) -> Balance;
    fn register_delegation(&mut self, account_id: &AccountId);
    fn delegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128);

    fn undelegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128);
    //

    fn add_proposal(&mut self, proposal: ProposalInput) -> u64;
    fn act_proposal(&mut self, id: u64, action: Action, memo: Option<String>);
    fn on_proposal_callback(&mut self, proposal_id: u64) -> PromiseOrValue<()>;
    //
    fn version(&self) -> String;

    fn get_config(&self) -> Config;
    fn get_policy(&self) -> Policy;
    fn get_staking_contract(self) -> String;
    fn has_blob(&self, hash: Base58CryptoHash) -> bool;
    fn get_locked_storage_amount(&self) -> U128;

    fn get_available_amount(&self) -> U128;
    fn delegation_total_supply(&self) -> U128;

    fn delegation_balance_of(&self, account_id: AccountId) -> U128;
    fn delegation_balance_ratio(&self, account_id: AccountId) -> (U128, U128);
    fn get_last_proposal_id(&self) -> u64;
    fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput>;
    fn get_proposal(&self, id: u64) -> ProposalOutput;
    fn get_bounty(&self, id: u64) -> BountyOutput;

    fn get_last_bounty_id(&self) -> u64;

    fn get_bounties(&self, from_index: u64, limit: u64) -> Vec<BountyOutput>;

    fn get_bounty_claims(&self, account_id: AccountId) -> Vec<BountyClaim>;

    fn get_bounty_number_of_claims(&self, id: u64) -> u32;
}

// use $crate::dao::{BountyClaim, BountyOutput, ProposalOutput, Policy, Action, ProposalInput, FactoryInfo};