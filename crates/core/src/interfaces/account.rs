use crate::error::StateError;

pub trait IAccount {
    fn withdraw(&mut self, amount: u64, fee: u64) -> Result<(), StateError>;
    fn increment_nonce(&mut self);
    fn deposit(&mut self, amount: u64);
    fn balance(&self) -> u64;
    fn nonce(&self) -> u64;
}
