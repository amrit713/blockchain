use crypto::Hash;
use serde::{Deserialize, Serialize};

use crate::{block::Block, error::StateError, state::State};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
    pub state: State,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let gensis_block = Block::gensis(difficulty);

        let mut state = State::new();

        state
            .apply_block(&gensis_block)
            .expect("genesis block application failed");

        Self {
            chain: vec![gensis_block],
            state,
            difficulty,
        }
    }

    pub fn blocks(&self) -> &[Block] {
        &self.chain
    }

    pub fn latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("Chain must contain at least genesis block")
    }

    pub fn height(&self) -> u64 {
        self.state.block_height()
    }

    pub fn latest_hash(&self) -> Hash {
        self.latest_block().hash
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    //---Block Processing
    pub fn add_block(&mut self, block: Block) -> Result<(), StateError> {
        if !block.is_valid(self.difficulty) {
            return Err(StateError::InvalidPreviousHash {
                expected: format!("Valid PoW block at diff {}", self.difficulty),
                got: "Invalid block header or proof-of-work".to_string(),
            });
        }

        //2. Validate chain continuity
        if block.header.previous_hash != self.latest_hash() {
            return Err(StateError::InvalidPreviousHash {
                expected: format!("{:?}", self.latest_hash()),
                got: format!("{:?}", block.header.previous_hash),
            });
        }

        if block.header.index != self.height() + 1 {
            return Err(StateError::InvalidBlockHeight {
                current: self.height(),
                expected: self.height() + 1,
                got: block.header.index,
            });
        }

        //3. Execute state transaction(9validates all transaction inside block)
        self.state.apply_block(&block)?;

        self.chain.push(block);

        Ok(())
    }
}
