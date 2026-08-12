use crate::{
    account::Account,
    block::Block,
    error::{StateError, TransactionError},
    interfaces::{IAccount, Verifiable},
    transaction::Transaction,
};
use crypto::{Address, Hash};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct State {
    accounts: HashMap<Address, Account>,
    block_height: u64,
    last_block_hash: Hash,
}

impl State {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            block_height: 0,
            last_block_hash: Hash::default(),
        }
    }

    //---getters---
    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    pub fn last_block_hash(&self) -> Hash {
        self.last_block_hash
    }

    pub fn get_account(&self, address: &Address) -> Account {
        self.accounts.get(address).copied().unwrap_or_default()
    }

    pub fn get_balance(&self, address: &Address) -> u64 {
        self.get_account(address).balance()
    }

    pub fn get_nonce(&self, address: &Address) -> u64 {
        self.get_account(address).nonce()
    }

    // ---STATE MUTATIONS--

    //Credits an address directly (used for genesis allocations or block miner rewards)
    pub fn credit(&mut self, address: Address, amount: u64) {
        let account = self.accounts.entry(address).or_default();

        account.deposit(amount);
    }

    //Execute a single transaction against the state state machine
    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<u64, StateError> {
        if tx.amount == 0 {
            return Err(TransactionError::ZeroAmount.into());
        }

        let sender_address = Address::from(tx.sender);
        let receiver_address = Address::from(tx.receiver);

        tx.verify()?;

        {
            let sender = self.accounts.get_mut(&sender_address).ok_or_else(|| {
                StateError::AccountNotFound {
                    address: sender_address.to_string(),
                }
            })?;

            sender.withdraw(tx.amount, tx.fee)?;
            sender.increment_nonce();
        }

        {
            let receiver = self
                .accounts
                .entry(receiver_address)
                .or_insert(Account::new(0, 0));

            receiver.deposit(tx.amount);
        }

        Ok(tx.fee)
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), StateError> {
        //1. Enforce strict chain linkage
        if self.block_height() > 0 {
            if block.header.previous_hash != self.last_block_hash {
                return Err(StateError::InvalidPreviousHash {
                    expected: self.last_block_hash.to_string(),
                    got: block.header.previous_hash.to_string(),
                });
            }
        }

        let expected_height = self.block_height + 1;

        if block.header.index != expected_height {
            return Err(StateError::InvalidBlockHeight {
                current: self.block_height,
                expected: expected_height,
                got: block.header.index,
            });
        }

        let mut total_fee = 0;

        for tx in &block.transactions {
            total_fee += self.apply_transaction(tx)?;
        }

        self.block_height = block.header.index;
        self.last_block_hash = block.hash;

        Ok(())
    }

    /// Generates a deterministic State Root Hash across all account balances
    pub fn state_root(&self) -> Hash {
        let mut sorted_accounts: Vec<(&Address, &Account)> = self.accounts.iter().collect();
        sorted_accounts.sort_by(|a, b| a.0.cmp(&b.0));

        let bytes = bincode::serialize(&sorted_accounts).expect("Failed to serialize");
        Hash::digest(&bytes)
    }
}
