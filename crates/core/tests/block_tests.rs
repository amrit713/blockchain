use core::{
    block::{self, Block, BlockHeader},
    constants::DEFAULT_DIFFICULTY,
    transaction::Transaction,
};
use crypto::{Address, Hash, Keypair, PublicKey};

fn create_dummy_tx(
    sender_keypair: &Keypair,
    receiver: PublicKey,
    amount: u64,
    nonce: u64,
    fee: u64,
) -> Transaction {
    Transaction::new(sender_keypair, receiver, amount, nonce, fee).unwrap()
}

#[test]
fn test_merkle_root_sensitivity_to_data() {
    let alice = Keypair::generate();
    let bob_addr = Keypair::generate().public_key();

    let tx1 = create_dummy_tx(&alice, bob_addr, 100, 0, 5);
    let tx2 = create_dummy_tx(&alice, bob_addr, 200, 1, 5);

    let root_1 = Block::calculate_merkle_root(&[tx1.clone(), tx2.clone()]);

    let mut tx2_modified = tx2.clone();
    tx2_modified.amount = 400;

    let root_2 = Block::calculate_merkle_root(&[tx1, tx2_modified]);

    assert_ne!(
        root_1, root_2,
        "Merkle root must change when a transaction field changes"
    );
}

#[test]
fn test_mining_and_difficulty_satisfaction() {
    let alice = Keypair::generate();
    let bob_addr = Keypair::generate().public_key();

    let tx = create_dummy_tx(&alice, bob_addr, 50, 0, 3);
    let previous_block = Block::gensis(DEFAULT_DIFFICULTY);

    let difficulty = DEFAULT_DIFFICULTY;
    let mut block = Block::new(
        previous_block.header.index + 1,
        previous_block.hash,
        vec![tx],
        difficulty,
        0,
    );

    block.mine(difficulty);
    assert!(Block::satisfies_difficulty(&block.hash, difficulty));
    assert_eq!(block.hash, block.header.calculate_hash());
    assert!(block.is_valid(difficulty));
}

#[test]
fn test_validation_fails_on_tampered_header() {
    let alice = Keypair::generate();
    let bob_addr = Keypair::generate().public_key();

    let tx = create_dummy_tx(&alice, bob_addr, 50, 0, 3);

    let previous_block = Block::gensis(DEFAULT_DIFFICULTY);
    let difficulty = DEFAULT_DIFFICULTY;
    let mut block = Block::new(
        previous_block.header.index + 1,
        previous_block.hash,
        vec![tx],
        difficulty,
        0,
    );

    block.header.index = 999;

    assert!(
        !block.is_valid(difficulty),
        "Block validation must fail if header hash no longer matches block.hash"
    );
}

#[test]
fn test_block_header_serialization_roundtrip() {
    let previous_block = Block::gensis(DEFAULT_DIFFICULTY);

    let header = BlockHeader {
        index: 55,
        timestamp: 17000000,
        previous_hash: previous_block.hash,
        merkle_root: Hash::digest(b"test"),
        nonce: 1001,
        difficulty: DEFAULT_DIFFICULTY,
    };

    let bytes = header.to_bytes();
    let deserialized: BlockHeader = bincode::deserialize(&bytes).unwrap();

    assert_eq!(header, deserialized);
    assert_eq!(header.calculate_hash(), deserialized.calculate_hash());
}
