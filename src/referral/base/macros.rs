/// The core methods for a referral. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_referral_core {
    ($contract:ident, $tokens:ident, $assert_access:ident) => {
        use $crate::referral::{ReferralCore, ReferralResolver};

        #[near_bindgen]
        impl ReferralCore for $contract {
          // get influencer address by account
          fn referral_by(&self, contract_id: AccountId, account_id: AccountId) -> Option<AccountId> {
            self.$tokens.referral_by(contract_id, account_id)
          }

          fn referral_program_code(&self, contract_id: $crate::referral::ContractId, influencer_id: $crate::referral::InfluencerId, program_id: $crate::referral::ProgramId) -> Option<String> {
            self.$tokens.referral_program_code(contract_id, influencer_id, program_id)
          }

           #[payable]
          fn referral_create_program(
              &mut self,
              influencer_id: AccountId,
              program_id: $crate::referral::ProgramId,
              royalty_percent: u64
          ) {
            self.$assert_access();
            self.$tokens.referral_create_program(influencer_id, program_id, royalty_percent)
          }

          #[payable]
          fn referral_accept(
              &mut self,
              contract_id: AccountId,
              influencer_id: AccountId,
              program_id: $crate::referral::ProgramId
          ) {
            self.$assert_access();
            self.$tokens.referral_accept(contract_id, influencer_id, program_id)
          }

          fn referral_program_royalty(
              &self,
              contract_id: AccountId,
              influencer_id: $crate::referral::InfluencerId,
              program_id: $crate::referral::ProgramId
          ) -> Option<$crate::referral::InfluencerRoyalty> {
            self.$tokens.referral_program_royalty(contract_id, influencer_id, program_id)
          }
        }

         #[near_bindgen]
        impl ReferralResolver for $contract {
          #[payable]
          fn resolve_on_referral_create(
            &mut self,
            contract_id: AccountId,
            influencer_id: AccountId,
            program_id: String,
            account_id: AccountId,
          ) -> bool {
              self.$tokens.resolve_on_referral_create(contract_id, influencer_id, program_id, account_id)
            }
          }
    };
}
