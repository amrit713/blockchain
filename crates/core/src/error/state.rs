// state.rs

use thiserror::Error
use crypto::Address;

use crate::error::{AccountError, TransactionError};

#[derive(Debug, Error)]
pub enum StateError {
    #[error("account {address} not found")]
    AccountNotFound { address: Address },

    #[error("invalid nonce for account {address}: expected {expected}, got {got}")]
    InvalidNonce {
        address: Address,
        expected: u64,
        got: u64,
    },

    #[error("transaction validation failed: {0}")]
    InvalidTransaction(#[from] TransactionError),

    #[error("account operation failed: {0}")]
    Account(#[from] AccountError),
}
