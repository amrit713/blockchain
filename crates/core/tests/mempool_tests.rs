use core::{
    blockchain::Blockchain, constants::DEFAULT_DIFFICULTY, mempool::Mempool, miner::Miner,
    transaction::Transaction,
};

use crypto::{Address, Keypair};

#[test]
fn test_mempool_miner_workflow() {
    let mut blockchain = Blockchain::new(DEFAULT_DIFFICULTY);
    let mut mempool = Mempool::new();

    let miner_keys = Keypair::generate();
    let miner = Miner::new(miner_keys);

    let alice = Keypair::generate();
    let bob = Keypair::generate();

    let alice_addr = Address::from(alice.public_key());
    let bob_addr = Address::from(bob.public_key());

    //1. credit alice in state
    blockchain.state.credit(alice_addr, 5000);

    //2. Create transactions with low and high fee rates
    // 2. Create transactions with low and high fee rates
    let tx_low = Transaction::new(&alice, bob.public_key(), 100, 0, 5).unwrap();
    let tx_high = Transaction::new(&alice, bob.public_key(), 200, 1, 50).unwrap();

    // 3. Submit both to mempool
    assert!(mempool.add_transaction(tx_low, blockchain.state()).is_ok());
    assert!(mempool.add_transaction(tx_high, blockchain.state()).is_ok());
    assert_eq!(mempool.len(), 2);

    // 4. Confirm highest fee transaction ranks first
    let txs = mempool.get_prioritized_transactions(1);
    assert_eq!(txs[0].fee, 50);

    //5. Mine block
    let block = miner
        .mine_next_block(&mut blockchain, &mut mempool, 10)
        .unwrap();

    // 6. Assert blockchain state and mempool eviction
    assert_eq!(blockchain.height(), 1);
    assert_eq!(block.transactions.len(), 2);
    assert_eq!(mempool.len(), 0);
    assert_eq!(blockchain.state().get_balance(&bob_addr), 300);
}
