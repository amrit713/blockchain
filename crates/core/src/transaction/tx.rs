use crate::error::TransactionError;
use crypto::{Hash, Keypair, PublicKey, Signature};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    //TODO: pub is used for only testing
    sender: PublicKey,
    receiver: PublicKey,
    amount: u64,
    nonce: u64,
    signature: Signature,
    fee: u64,
}

impl Transaction {
    pub fn new(
        sender_keypair: &Keypair,
        receiver: PublicKey,
        amount: u64,
        nonce: u64,
        fee: u64,
    ) -> Result<Self, TransactionError> {
        let sender = sender_keypair.public_key();

        Self::validate_fields(&sender, &receiver, amount)?;

        let mut tx = Self {
            sender,
            receiver,
            amount,
            nonce,
            fee,
            signature: Signature::from_bytes(&[0u8; 64]), // Placeholder signature for verification
        };

        let signing_hash = tx.signing_hash()?;

        tx.signature = sender_keypair.sign(&signing_hash);

        Ok(tx)
    }

    fn signing_payload(
        sender: &PublicKey,
        receiver: &PublicKey,
        amount: u64,
        nonce: u64,
        fee: u64,
    ) -> ([u8; 32], [u8; 32], u64, u64, u64) {
        (sender.as_bytes(), receiver.as_bytes(), amount, nonce, fee)
    }

    pub fn signing_hash(&self) -> Result<Hash, TransactionError> {
        let payload = (
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            self.amount,
            self.nonce,
            self.fee,
        );

        let bytes = bincode::serialize(&payload).map_err(TransactionError::Serialization)?;

        Ok(Hash::digest(&bytes))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    pub fn validate_fields(
        sender: &PublicKey,
        receiver: &PublicKey,
        amount: u64,
    ) -> Result<(), TransactionError> {
        if amount == 0 {
            return Err(TransactionError::ZeroAmount);
        }

        if sender == receiver {
            return Err(TransactionError::SelfTransfer);
        }

        Ok(())
    }

    pub fn verify(&self) -> Result<(), TransactionError> {
        Self::validate_fields(&self.sender, &self.receiver, self.amount);

        let signing_hash = self.signing_hash()?;

        self.sender
            .verify(&signing_hash, &self.signature)
            .map_err(TransactionError::InvalidSignature)?;

        Ok(())
    }

    pub fn sender(&self) -> &PublicKey {
        &self.sender
    }

    pub fn receiver(&self) -> &PublicKey {
        &self.receiver
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn fee(&self) -> u64 {
        self.fee
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}
