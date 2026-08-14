use crypto::CryptoError;
use thiserror::Error;

// transaction.rs

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("transaction amount must be greater than zero")]
    ZeroAmount,

    #[error("sender and receiver cannot be the same")]
    SelfTransfer,

    #[error("invalid cryptographic signature: {0}")]
    InvalidSignature(#[source] CryptoError),

    #[error("failed to serialize transaction payload: {0}")]
    Serialization(#[from] bincode::Error),
}
