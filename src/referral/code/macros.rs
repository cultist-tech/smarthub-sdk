/// The core methods for a referral. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_referral_code {
    ($contract:ident, $tokens:ident, $assert_access:ident) => {
        use $crate::referral::{ReferralCode};

        #[near_bindgen]
        impl ReferralCode for $contract {

          fn referral_program_code(&self, contract_id: $crate::referral::ContractId, influencer_id: $crate::referral::InfluencerId, program_id: $crate::referral::ProgramId) -> Option<String> {
            self.$tokens.referral_program_code(contract_id, influencer_id, program_id)
          }

          #[payable]
          fn referral_accept_code(
              &mut self,
              code: String,
          ) {
            self.$assert_access();
            self.$tokens.referral_accept_code(code)
          }

          fn referral_code_info(&self, code: String) -> Option<$crate::referral::ReferralInfo> {
            self.$tokens.referral_code_info(code)
          }
        }
    };
}
