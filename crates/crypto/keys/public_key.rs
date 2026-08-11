use crate::{Hash, error::CryptoError, keys::Signature};
use ed25519_dalek::{Verifier, VerifyingKey as DalekVerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicKey(pub(crate) DalekVerifyingKey);

impl PublicKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CryptoError> {
        let key =
            DalekVerifyingKey::from_bytes(bytes).map_err(|_| CryptoError::InvalidPublicKeyBytes)?;

        Ok(PublicKey(key))
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn verify(&self, hash: &Hash, signature: &Signature) -> Result<(), CryptoError> {
        self.0
            .verify(hash.as_bytes(), &signature.0)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}
