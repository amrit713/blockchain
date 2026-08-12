use core::{state::State, transaction::Transaction};
use crypto::{Address, Keypair};

#[test]
fn test_state_root_is_deterministic_regardless_of_insertion_order() {
    let key_a = Keypair::generate();
    let key_b = Keypair::generate();

    let addr_a = Address::from(key_a.public_key());
    let addr_b = Address::from(key_b.public_key());

    // State 1: Insert A then B
    let mut state_1 = State::new();
    state_1.credit(addr_a, 100);
    state_1.credit(addr_b, 200);

    // State 2: Insert B then A
    let mut state_2 = State::new();
    state_2.credit(addr_b, 200);
    state_2.credit(addr_a, 100);

    // Both state roots must be identical despite insertion order difference
    assert_eq!(
        state_1.state_root(),
        state_2.state_root(),
        "State roots must match regardless of insertion order"
    );
}

#[test]
fn test_state_root_changes_on_state_mutation() {
    let alice_keypair = Keypair::generate();
    let bob_keypair = Keypair::generate();

    let alice_addr = Address::from(alice_keypair.public_key());
    let bob_addr = Address::from(bob_keypair.public_key());

    let mut state = State::new();
    state.credit(alice_addr, 500);

    let root_before = state.state_root();

    // Perform transaction
    let tx = Transaction::new(&alice_keypair, *bob_addr.public_key(), 100, 0, 5).unwrap();

    assert!(state.apply_transaction(&tx).is_ok());

    let root_after = state.state_root();

    // State root must change after state transition
    assert_ne!(
        root_before, root_after,
        "State root must mutate when accounts or balances update"
    );
}
