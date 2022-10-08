use near_sdk::{ require, env };

/// Assert that at least 1 yoctoNEAR was attached.
pub(crate) fn assert_at_least_one_yocto() {
    require!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR")
}