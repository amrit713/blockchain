use crate::keys::PublicKey;
use crate::keys::Signature;
use crate::primitives::Hash;
use ed25519_dalek::{Signer, SigningKey as DalekSigningKey};

use rand::rngs::OsRng;

pub struct Keypair(pub(crate) DalekSigningKey);

impl Keypair {
    /// Generates a new cryptographically secure keypair using OS entropy.
    pub fn generate() -> Self {
        // Pass OsRng directly into generate (it implements CryptoRng/RngCore)
        Keypair(DalekSigningKey::generate(&mut OsRng))
    }

    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Keypair(DalekSigningKey::from_bytes(seed))
    }

    pub fn to_seed(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key())
    }

    pub fn sign(&self, hash: &Hash) -> Signature {
        let sig = self.0.sign(hash.as_bytes());
        Signature(sig)
    }
}
