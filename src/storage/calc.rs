use near_sdk::{ StorageUsage, env };
use crate::utils::refund_deposit_to_account;

pub struct Storage {
    prev: StorageUsage,
    next: StorageUsage,
    diff: StorageUsage,
}

impl Storage {
    pub fn start() -> Self {
        Self {
            prev: env::storage_usage(),
            next: 0,
            diff: 0,
        }
    }

    pub fn end(&mut self) {
        self.next = env::storage_usage();
        self.diff = self.next - self.diff;
    }

    pub fn refund(&mut self, attached_price: &u128) {
        self.next = env::storage_usage();
        self.diff = self.next - self.prev;

        let refund = attached_price - (self.diff as u128) * env::storage_byte_cost();

        env::log_str(&format!("Refund {}", refund.to_string()));
        refund_deposit_to_account(refund);
    }
}
