use crypto::CryptoError;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum TransactionError {
    #[error("Transaction amount must be greater than zero")]
    ZeroAmount,

    #[error("Sender and receiver cannot be the same address")]
    SelfTransfer,

    #[error("Invalid cryptographic signature: {0}")]
    InvalidSignature(#[from] CryptoError),

    #[error("Failed to serialize transaction payload")]
    SerializationFailed,
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum StateError {
    #[error("Account {address} does not exist in world state")]
    AccountNotFound { address: String },

    #[error("Insufficient balance. Available: {available}, Required: {required}")]
    InsufficientBalance { available: u64, required: u64 },

    #[error("Invalid nonce for account {address}. Expected: {expected}, Got: {got}")]
    InvalidNonce {
        address: String,
        expected: u64,
        got: u64,
    },

    #[error("Transaction validation failed: {0}")]
    InvalidTransaction(#[from] TransactionError),
}
