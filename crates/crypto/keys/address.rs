use crate::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address(pub PublicKey);

impl Address {
    pub fn new(public_key: PublicKey) -> Self {
        Self(public_key)
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.0
    }
}

impl From<PublicKey> for Address {
    fn from(public_key: PublicKey) -> Self {
        Address(public_key)
    }
}
