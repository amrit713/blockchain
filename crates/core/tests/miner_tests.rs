use core::{
    blockchain::Blockchain, constants::DEFAULT_DIFFICULTY, mempool::Mempool, miner::Miner,
    transaction::Transaction,
};

use crypto::{Address, Keypair};

#[test]
fn test_pow_difficulty_satisfaction() {
    let difficulty = 2; // Requires 2 leading zeroes in block hash
    let mut blockchain = Blockchain::new(difficulty);
    let mut mempool = Mempool::new();
    let miner = Miner::new(Keypair::generate());

    let mined_block = miner
        .mine_next_block(&mut blockchain, &mut mempool, 10)
        .expect("Mining failed");

    // Verify proof-of-work hash satisfies leading zero requirement
    assert!(mined_block.hash.iter().take(difficulty).all(|&b| b == 0));
}

#[test]
fn test_miner_receives_block_subsidy_and_fees() {
    let mut blockchain = Blockchain::new(DEFAULT_DIFFICULTY);
    let mut mempool = Mempool::new();

    let miner_keys = Keypair::generate();
    let miner = Miner::new(miner_keys);

    let alice = Keypair::generate();
    let bob = Keypair::generate();

    let alice_address = Address::from(alice.public_key());
    let bob_address = Address::from(bob.public_key());

    // 1. Credit Alice in state
    blockchain.state.credit(alice_address, 1000);

    // 2. Alice sends 200 coins to Bob with a 15 fee
    let tx = Transaction::new(&alice, bob.public_key(), 200, 0, 15).unwrap();
    assert!(mempool.add_transaction(tx, blockchain.state()).is_ok());

    // Initial miner balance should be 0
    assert_eq!(blockchain.state().get_balance(&miner.address()), 0);

    // 3. Mine next block (Block Subsidy: 50 + Tx Fee: 15 = 65 Total Reward)
    let mined_block = miner
        .mine_next_block(&mut blockchain, &mut mempool, 10)
        .unwrap();

    // *4. Assertions
    assert_eq!(blockchain.height(), 1);
    assert_eq!(mined_block.transactions.len(), 1);

    // Bob receives amount
    assert_eq!(blockchain.state().get_balance(&bob_address), 200);

    // Alice balance drops (1000 - 200 - 15 = 785)
    assert_eq!(blockchain.state().get_balance(&alice_address), 785);

    //TODO: ADD MINER REWARD
    // Miner receives Subsidy (50) + Fee (15) = 65
    assert_eq!(blockchain.state().get_balance(&miner.address()), 15);

    // Mempool is cleared
    assert_eq!(mempool.len(), 0);
}

#[test]
fn test_miner_prioritizes_high_fee_transactions() {
    let mut blockchain = Blockchain::new(DEFAULT_DIFFICULTY);
    let mut mempool = Mempool::new();

    let miner = Miner::new(Keypair::generate());
    let alice = Keypair::generate();
    let bob = Keypair::generate();
    let charlie = Keypair::generate();

    let alice_addr = Address::from(alice.public_key());
    let bob_addr = Address::from(bob.public_key());

    blockchain.state.credit(alice_addr, 2000);
    blockchain.state.credit(bob_addr, 2000);

    // Tx 1: Low fee (Fee = 2)
    let tx_low_fee = Transaction::new(&alice, charlie.public_key(), 100, 0, 2).unwrap();

    // Tx 2: High fee (Fee = 50)
    let tx_high_fee = Transaction::new(&bob, charlie.public_key(), 100, 0, 50).unwrap();

    assert!(mempool
        .add_transaction(tx_low_fee, blockchain.state())
        .is_ok());
    assert!(mempool
        .add_transaction(tx_high_fee, blockchain.state())
        .is_ok());

    // 2 transactions in pool, but limit block size to max 1 transaction
    let mined_block = miner
        .mine_next_block(&mut blockchain, &mut mempool, 1)
        .expect("Mining failed");

    // Block contains only 1 transaction (the high fee one)
    assert_eq!(mined_block.transactions.len(), 1);
    assert_eq!(mined_block.transactions[0].fee, 50);

    // 1 low-fee transaction remains in mempool for next block
    assert_eq!(mempool.len(), 1);
}
