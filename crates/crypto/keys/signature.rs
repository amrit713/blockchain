use ed25519_dalek::Signature as DalekSignature;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Signature(pub(crate) DalekSignature);

impl Signature {
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        let dalek_signature = DalekSignature::from_bytes(bytes);
        Signature(dalek_signature)
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
}
