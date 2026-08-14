use core::{
    block::Block,
    blockchain::Blockchain,
    constants::DEFAULT_DIFFICULTY,
    miner::{self, Miner},
    transaction::Transaction,
};
use crypto::{Address, Hash, Keypair};

#[test]
fn test_blockchain_initialization() {
    let chain = Blockchain::new(DEFAULT_DIFFICULTY);

    assert_eq!(chain.height(), 0);
    assert_eq!(chain.blocks().len(), 1);

    let genesis = chain.latest_block();

    assert_eq!(genesis.header.index, 0);
    assert_eq!(genesis.header.previous_hash, Hash::default());
    assert_eq!(genesis.header.difficulty, DEFAULT_DIFFICULTY);
    assert!(genesis.transactions.is_empty());
}

#[test]
fn test_successful_block_addition() {
    let difficulty = 1;
    let mut blockchain = Blockchain::new(difficulty);
    let alice = Keypair::generate();
    let bob_key = Keypair::generate().public_key();

    let miner_key = Keypair::generate();
    let miner = Miner::new(miner_key);

    let alice_address = Address::from(alice.public_key());
    let bob_address = Address::from(bob_key);

    blockchain.state.credit(alice_address, 1000);

    // Account nonce validation check
    let tx = Transaction::new(&alice, bob_key, 300, 0, 5).unwrap();

    let block_1 = Block::new(
        1,
        blockchain.latest_hash(),
        vec![tx],
        blockchain.difficulty(),
        0,
    );

    assert!(blockchain.add_block(block_1, &miner.address()).is_ok());
    assert_eq!(blockchain.height(), 1);
    assert_eq!(blockchain.state.get_balance(&bob_address), 300);
    assert_eq!(blockchain.state().get_balance(&alice_address), 695);
}

#[test]
fn test_rejects_out_of_sequence_block() {
    let mut blockchain = Blockchain::new(DEFAULT_DIFFICULTY);

    let miner_key = Keypair::generate();
    let miner = Miner::new(miner_key);

    let invalid_block = Block::new(
        5,
        blockchain.latest_hash(),
        vec![],
        blockchain.difficulty(),
        0,
    );

    assert!(blockchain
        .add_block(invalid_block, &miner.address())
        .is_err());
    assert_eq!(blockchain.height(), 0);
}
