use crate::{error::StateError, interfaces::IAccount};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Account {
    balance: u64,
    nonce: u64,
}

impl Account {
    pub fn new(balance: u64, nonce: u64) -> Self {
        Account { balance, nonce }
    }
}

impl IAccount for Account {
    fn balance(&self) -> u64 {
        self.balance
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn increment_nonce(&mut self) {
        self.nonce = self.nonce.saturating_add(1);
    }

    fn deposit(&mut self, amount: u64) {
        self.balance = self.balance.saturating_add(amount);
    }

    /// Deducts funds from the account if available
    fn withdraw(&mut self, amount: u64, fee: u64) -> Result<(), StateError> {
        if self.balance < amount + fee {
            return Err(StateError::InsufficientBalance {
                available: self.balance,
                required: amount,
            });
        }

        let total_amount = amount + fee;
        self.balance -= total_amount;
        Ok(())
    }
}
