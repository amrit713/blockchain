use std::time::{SystemTime, UNIX_EPOCH};

use crypto::Hash;
use serde::{Deserialize, Serialize};

use crate::{constants::DEFAULT_DIFFICULTY, interfaces::Storable, transaction::Transaction};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub nonce: u64,
    pub difficulty: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Hash,
}

impl BlockHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("Serializationfailed")
    }
    pub fn calculate_hash(&self) -> Hash {
        Hash::digest(&self.to_bytes())
    }
}

impl Block {
    pub fn new(
        index: u64,

        previous_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: usize,
        nonce: u64,
    ) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let header = BlockHeader {
            index,
            timestamp,
            previous_hash,
            merkle_root,
            difficulty,
            nonce,
        };

        let mut block = Self {
            header,
            transactions,
            hash: Hash::default(),
        };
        block.mine(DEFAULT_DIFFICULTY);

        block
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return Hash::digest(b"empty_block");
        }

        let mut combined_bytes = Vec::new();

        for tx in transactions {
            combined_bytes.extend_from_slice(tx.hash().as_bytes());
        }

        Hash::digest(&combined_bytes)
    }

    pub fn satisfies_difficulty(hash: &Hash, difficulty: usize) -> bool {
        hash.iter().take(difficulty).all(|&byte| byte == 0)
    }

    pub fn gensis(&self) -> Self {
        Self::new(0, Hash::default(), Vec::new(), DEFAULT_DIFFICULTY, 0)
    }

    pub fn is_valid(&self, difficulty: usize) -> bool {
        let expected_merkle = Self::calculate_merkle_root(&self.transactions);

        if self.header.merkle_root != expected_merkle {
            return false;
        }

        let calculated_hash = self.header.calculate_hash();

        calculated_hash == self.hash && Self::satisfies_difficulty(&self.hash, difficulty)
    }

    pub fn mine(&mut self, difficulty: usize) {
        loop {
            let hash = self.header.calculate_hash();

            if Self::satisfies_difficulty(&hash, difficulty) {
                self.hash = hash;
                break;
            }

            self.header.nonce += 1;
        }
    }
}
