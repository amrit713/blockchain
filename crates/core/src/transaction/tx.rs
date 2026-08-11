use crate::{
    error::{StateError, TransactionError},
    interfaces::{Executable, IAccount, Storable, Verifiable},
};
use crypto::{Hash, Keypair, PublicKey, Signature};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    //TODO: pub is used for only testing purpose
    pub sender: PublicKey,
    pub receiver: PublicKey,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        sender_keypair: &Keypair,
        receiver: PublicKey,
        amount: u64,
        nonce: u64,
    ) -> Result<Self, TransactionError> {
        let sender = sender_keypair.public_key();

        if amount == 0 {
            return Err(TransactionError::ZeroAmount);
        }

        if sender == receiver {
            return Err(TransactionError::SelfTransfer);
        }

        let mut tx = Self {
            sender,
            receiver,
            amount,
            nonce,
            signature: Signature::from_bytes(&[0u8; 64]), // Placeholder signature for verification
        };

        let tx_hash = tx.hash();

        tx.signature = sender_keypair.sign(&tx_hash);

        Ok(tx)
    }
}

impl Storable for Transaction {
    fn hash(&self) -> Hash {
        let payload = (
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            self.amount,
            self.nonce,
        );

        let bytes = bincode::serialize(&payload).unwrap_or_default();

        Hash::digest(&bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }
}

impl Verifiable for Transaction {
    type Error = TransactionError;

    fn verify(&self) -> Result<(), Self::Error> {
        if self.amount == 0 {
            return Err(TransactionError::ZeroAmount);
        }

        if self.sender == self.receiver {
            return Err(TransactionError::SelfTransfer);
        }

        let tx_hash = self.hash();

        self.sender
            .verify(&tx_hash, &self.signature)
            .map_err(TransactionError::InvalidSignature)?;

        Ok(())
    }
}

impl Executable for Transaction {
    type Error = StateError;

    fn execute<A: IAccount>(
        &self,
        sender_acc: &mut A,
        recipient_acc: &mut A,
    ) -> Result<(), Self::Error> {
        self.verify()?;

        // 2. Validate nonce ordering (must match sender's current nonce)
        if sender_acc.nonce() != self.nonce {
            return Err(StateError::InvalidNonce {
                address: format!("{:?}", self.sender),
                expected: sender_acc.nonce(),
                got: self.nonce,
            });
        }

        // 3. Withdraw funds from sender (checks balance internally)
        sender_acc.withdraw(self.amount)?;

        // 4. Increment sender's nonce
        sender_acc.increment_nonce();

        // 5. Deposit funds into recipient account
        recipient_acc.deposit(self.amount);

        Ok(())
    }
}
