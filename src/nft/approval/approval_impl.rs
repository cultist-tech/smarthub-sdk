/// Common implementation of the [approval management standard](https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html) for NFTs.
/// on the contract/account that has just been approved. This is not required to implement.
use crate::nft::approval::NonFungibleTokenApproval;
use crate::nft::token::TokenId;
use crate::nft::utils::{ refund_approved_account_ids, refund_approved_account_ids_iter };
use crate::nft::NonFungibleToken;
use near_sdk::{ assert_one_yocto, env, ext_contract, require, AccountId, Gas, Promise };

pub const GAS_FOR_NFT_APPROVE: Gas = Gas(12_000_000_000_000);

fn expect_token_found<T>(option: Option<T>) -> T {
    option.unwrap_or_else(|| env::panic_str("Token not found"))
}

fn expect_approval<T>(option: Option<T>) -> T {
    option.unwrap_or_else(|| env::panic_str("next_approval_by_id must be set for approval ext"))
}

#[ext_contract(ext_approval_receiver)]
pub trait NonFungibleTokenReceiver {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String
    );
}

impl NonFungibleToken {
    // pub(crate) fn assert_approved(&mut self, token_id: &TokenId, remove: bool) {
    //   let approved_account_ids = self.approvals_by_id.as_mut().unwrap().get(&token_id);
    //   let app_acc_ids = approved_account_ids.as_ref().unwrap_or_else(|| env::panic_str("Unauthorized"));
    //
    //   let sender_id = env::predecessor_account_id();
    //
    //   let actual_approval_id = app_acc_ids.get(&sender_id);
    //
    //   if actual_approval_id.is_none() {
    //     env::panic_str("Sender not approved");
    //   }
    //
    //   if remove {
    //     self.approvals_by_id.as_mut().unwrap().remove(&token_id);
    //   }
    // }
}

impl NonFungibleTokenApproval for NonFungibleToken {
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>
    ) -> Option<Promise> {
        // assert_at_least_one_yocto();

        let approvals_by_id = self.approvals_by_id
            .as_mut()
            .unwrap_or_else(|| env::panic_str("NFT does not support Approval Management"));

        let owner_id = expect_token_found(self.owner_by_id.get(&token_id));

        require!(env::predecessor_account_id() == owner_id, "Predecessor must be token owner.");

        let next_approval_id_by_id = expect_approval(self.next_approval_id_by_id.as_mut());
        // update HashMap of approvals for this token
        let approved_account_ids = &mut approvals_by_id.get(&token_id).unwrap_or_default();
        let approval_id: u64 = next_approval_id_by_id.get(&token_id).unwrap_or(1u64);
        let _old_approval_id = approved_account_ids.insert(account_id.clone(), approval_id);

        // save updated approvals HashMap to contract's LookupMap
        approvals_by_id.insert(&token_id, approved_account_ids);

        // increment next_approval_id for this token
        next_approval_id_by_id.insert(&token_id, &(approval_id + 1));

        // If this approval replaced existing for same account, no storage was used.
        // Otherwise, require that enough deposit was attached to pay for storage, and refund
        // excess.
        // let storage_used =
        //   if old_approval_id.is_none() { bytes_for_approved_account_id(&account_id) } else { 0 };
        // refund_deposit(storage_used);

        let final_msg = msg.clone();

        // if given `msg`, schedule call to `nft_on_approve` and return it. Else, return None.
        final_msg.map(|final_msg| {
            ext_approval_receiver
                ::ext(account_id)
                .with_static_gas(env::prepaid_gas() - GAS_FOR_NFT_APPROVE)
                .nft_on_approve(token_id, owner_id, approval_id, final_msg)

            // ext_approval_receiver::nft_on_approve(
            //   token_id,
            //   owner_id,
            //   approval_id,
            //   final_msg,
            //
            //   account_id,
            //   NO_DEPOSIT,
            //   env::prepaid_gas() - GAS_FOR_NFT_APPROVE,
            // )
        })
    }

    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto();
        let approvals_by_id = self.approvals_by_id.as_mut().unwrap_or_else(|| {
            env::panic_str("NFT does not support Approval Management");
        });

        let owner_id = expect_token_found(self.owner_by_id.get(&token_id));
        let predecessor_account_id = env::predecessor_account_id();

        require!(predecessor_account_id == owner_id, "Predecessor must be token owner.");

        // if token has no approvals, do nothing
        if let Some(approved_account_ids) = &mut approvals_by_id.get(&token_id) {
            // if account_id was already not approved, do nothing
            if approved_account_ids.remove(&account_id).is_some() {
                refund_approved_account_ids_iter(
                    predecessor_account_id,
                    core::iter::once(&account_id)
                );
                // if this was the last approval, remove the whole HashMap to save space.
                if approved_account_ids.is_empty() {
                    approvals_by_id.remove(&token_id);
                } else {
                    // otherwise, update approvals_by_id with updated HashMap
                    approvals_by_id.insert(&token_id, approved_account_ids);
                }
            }
        }
    }

    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();
        let approvals_by_id = self.approvals_by_id.as_mut().unwrap_or_else(|| {
            env::panic_str("NFT does not support Approval Management");
        });

        let owner_id = expect_token_found(self.owner_by_id.get(&token_id));
        let predecessor_account_id = env::predecessor_account_id();

        require!(predecessor_account_id == owner_id, "Predecessor must be token owner.");

        // if token has no approvals, do nothing
        if let Some(approved_account_ids) = &mut approvals_by_id.get(&token_id) {
            // otherwise, refund owner for storage costs of all approvals...
            refund_approved_account_ids(predecessor_account_id, approved_account_ids);
            // ...and remove whole HashMap of approvals
            approvals_by_id.remove(&token_id);
        }
    }

    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool {
        expect_token_found(self.owner_by_id.get(&token_id));

        let approvals_by_id = if let Some(a) = self.approvals_by_id.as_ref() {
            a
        } else {
            // contract does not support approval management
            return false;
        };

        let approved_account_ids = if let Some(ids) = approvals_by_id.get(&token_id) {
            ids
        } else {
            // token has no approvals
            return false;
        };

        let actual_approval_id = if let Some(id) = approved_account_ids.get(&approved_account_id) {
            id
        } else {
            // account not in approvals HashMap
            return false;
        };

        if let Some(given_approval_id) = approval_id {
            &given_approval_id == actual_approval_id
        } else {
            // account approved, no approval_id given
            true
        }
    }
}