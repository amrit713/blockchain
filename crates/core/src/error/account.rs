// account.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },

    #[error("balance arithmetic overflow")]
    BalanceOverflow,

    #[error("transaction cost overflow")]
    CostOverflow,
}
