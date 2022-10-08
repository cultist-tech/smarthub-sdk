#[macro_export]
macro_rules! impl_dao {
    ($contract:ident, $instance:ident) => {
        use near_sdk::json_types::{U64};
        use $crate::dao::{BountyClaim, BountyOutput, ProposalOutput, Policy, Action, ProposalInput, FactoryInfo, DaoCore};

        #[near_bindgen]
        impl DaoCore for $contract  {
          fn remove_blob(&mut self, hash: Base58CryptoHash) -> Promise {
            self.$instance.remove_blob(hash)
          }

          fn get_factory_info(&self) -> FactoryInfo {
             self.$instance.get_factory_info()
          }
          //
          #[payable]
          fn bounty_claim(&mut self, id: u64, deadline: U64) {
            self.$instance.bounty_claim(id, deadline)
          }

          #[payable]
          fn bounty_done(&mut self, id: u64, account_id: Option<AccountId>, description: String) {
              self.$instance.bounty_done(id, account_id, description)
          }

          fn bounty_giveup(&mut self, id: u64) -> PromiseOrValue<()> {
          self.$instance.bounty_giveup(id)
          }

          fn get_user_weight(&self, account_id: &AccountId) -> Balance {
          self.$instance.get_user_weight(account_id)
          }

          #[payable]
          fn register_delegation(&mut self, account_id: &AccountId) {
          self.$instance.register_delegation(account_id)
          }

          fn delegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128) {
          self.$instance.delegate(account_id, amount)
          }

          fn undelegate(&mut self, account_id: &AccountId, amount: U128) -> (U128, U128, U128) {
          self.$instance.undelegate(account_id, amount)
          }

          //
          #[payable]
          fn add_proposal(&mut self, proposal: ProposalInput) -> u64 {
            self.$instance.add_proposal(proposal)
          }

          fn act_proposal(&mut self, id: u64, action: Action, memo: Option<String>) {
            self.$instance.act_proposal(id, action, memo)
          }


          fn on_proposal_callback(&mut self, proposal_id: u64) -> PromiseOrValue<()> {
            self.$instance.on_proposal_callback(proposal_id)
          }

          //
          fn version(&self) -> String {
            self.$instance.version()
          }

          fn get_config(&self) -> Config {
            self.$instance.get_config()
          }

          fn get_policy(&self) -> Policy {
          self.$instance.get_policy()
          }

          fn get_staking_contract(self) -> String {
          self.$instance.get_staking_contract()
          }

          fn has_blob(&self, hash: Base58CryptoHash) -> bool {
          self.$instance.has_blob(hash)
          }

          fn get_locked_storage_amount(&self) -> U128 {
          self.$instance.get_locked_storage_amount()
          }

          fn get_available_amount(&self) -> U128 {
          self.$instance.get_available_amount()
          }

          fn delegation_total_supply(&self) -> U128 {
          self.$instance.delegation_total_supply()
          }

          fn delegation_balance_of(&self, account_id: AccountId) -> U128 {
          self.$instance.delegation_balance_of(account_id)
          }

          fn delegation_balance_ratio(&self, account_id: AccountId) -> (U128, U128) {
          self.$instance.delegation_balance_ratio(account_id)
          }

          fn get_last_proposal_id(&self) -> u64 {
          self.$instance.get_last_proposal_id()
          }

          fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<ProposalOutput> {
          self.$instance.get_proposals(from_index, limit)
          }

          fn get_proposal(&self, id: u64) -> ProposalOutput {
          self.$instance.get_proposal(id)
          }

          fn get_bounty(&self, id: u64) -> BountyOutput {
          self.$instance.get_bounty(id)
          }

          fn get_last_bounty_id(&self) -> u64 {
          self.$instance.get_last_bounty_id()
          }

          fn get_bounties(&self, from_index: u64, limit: u64) -> Vec<BountyOutput> {
          self.$instance.get_bounties(from_index, limit)
          }

          fn get_bounty_claims(&self, account_id: AccountId) -> Vec<BountyClaim> {
          self.$instance.get_bounty_claims(account_id)
          }

          fn get_bounty_number_of_claims(&self, id: u64) -> u32 {
          self.$instance.get_bounty_number_of_claims(id)
          }
        }
    };
}