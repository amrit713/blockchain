use crate::{
    error::{StateError, TransactionError},
    interfaces::{Storable, Verifiable},
    state::State,
    transaction::{self, Transaction},
};
use crypto::{Address, Hash};
use std::collections::HashMap;

pub struct Mempool {
    pending_transactions: HashMap<Hash, Transaction>,
    account_nonces: HashMap<Address, u64>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            pending_transactions: HashMap::new(),
            account_nonces: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.pending_transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending_transactions.is_empty()
    }

    //Validates and adds a transaction to the mempool
    pub fn add_transaction(
        &mut self,
        tx: Transaction,
        state: &State,
    ) -> Result<(), TransactionError> {
        tx.verify()?;

        let sender = Address::from(tx.sender);

        let state_nonce = state.get_nonce(&sender);
        let exptected_nonce = self
            .account_nonces
            .get(&sender)
            .copied()
            .unwrap_or(state_nonce);

        if tx.nonce != exptected_nonce {
            return Err(TransactionError::InvalidNonce {
                expected: exptected_nonce,
                got: tx.nonce,
            });
        }

        let current_balance = state.get_balance(&sender);
        let required = tx
            .amount
            .checked_add(tx.fee)
            .ok_or(TransactionError::CostOverflow)?;

        if current_balance < required {
            return Err(TransactionError::InsufficentBalance {
                required,
                available: current_balance,
            });
        }

        let tx_hash = tx.hash();
        self.account_nonces.insert(sender, tx.nonce + 1);
        self.pending_transactions.insert(tx_hash, tx);

        Ok(())
    }

    // Selects candicate transactions ordered by highest fee priority
    pub fn get_prioritized_transactions(&self, limit: usize) -> Vec<Transaction> {
        let mut transactions: Vec<Transaction> =
            self.pending_transactions.values().cloned().collect();
        transactions.sort_by(|a, b| b.fee.cmp(&a.fee));

        let count = limit.min(transactions.len());

        transactions.into_iter().take(count).collect()
    }

    pub fn remove_mined_transactions(&mut self, mined_tsx: &[Transaction]) {
        for tx in mined_tsx {
            let tx_hash = tx.hash();
            self.pending_transactions.remove(&tx_hash);
        }
    }
}
