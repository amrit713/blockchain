use crypto::{Address, Keypair};

use crate::{block::Block, blockchain::Blockchain, error::StateError, mempool::Mempool};

pub struct Miner {
    keypair: Keypair,
}

impl Miner {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    pub fn address(&self) -> Address {
        Address::from(self.keypair.public_key())
    }

    pub fn mine_next_block(
        &self,
        blockchain: &mut Blockchain,
        mempool: &mut Mempool,
        max_tx_per_block: usize,
    ) -> Result<Block, String> {
        let candiate_tsx = mempool.get_prioritized_transactions(max_tx_per_block);

        //2. prepare candidate block metadata
        let next_index = blockchain.height() + 1;
        let prev_hash = blockchain.latest_hash();
        let difficulty = blockchain.difficulty();

        // Assemble and execute proof of work miniing loop
        let mut new_block = Block::new(next_index, prev_hash, candiate_tsx.clone(), difficulty, 0);

        new_block.mine(difficulty);

        blockchain
            .add_block(new_block.clone(), &self.address())
            .map_err(|e| format!("Block execution failed: {:?}", e))?;

        //5. Evict confirmed transactions from mempool
        mempool.remove_mined_transactions(&candiate_tsx);

        Ok(new_block)
    }
}
