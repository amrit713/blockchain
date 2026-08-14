use crypto::CryptoError;
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

    #[error("Cost calculation overflowed u64")]
    CostOverflow,

    #[error("Nonce didnot match expected:{expected} got:{got}")]
    InvalidNonce { expected: u64, got: u64 },

    #[error("Required balance:{required} available balance:{available}")]
    InsufficentBalance { required: u64, available: u64 },
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

    #[error("Invalid block height sequence. Current height: {current}, Expected: {expected}, Got: {got}")]
    InvalidBlockHeight {
        current: u64,
        expected: u64,
        got: u64,
    },

    #[error("Invalid block linkage. Expected previous hash: {expected}, Got: {got}")]
    InvalidPreviousHash { expected: String, got: String },

    #[error("Block execution failed {error}")]
    BlockError { error: String },
}
