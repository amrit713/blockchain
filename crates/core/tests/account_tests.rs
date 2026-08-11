use core::{account::Account, error::StateError, interfaces::IAccount};

#[test]
fn test_account_creation_and_defaults() {
    let acc = Account::new(500, 2);
    assert_eq!(acc.balance(), 500);
    assert_eq!(acc.nonce(), 2);
}

#[test]
fn test_account_deposit_and_withdraw() {
    let mut acc = Account::new(100, 0);

    acc.deposit(50);
    assert_eq!(acc.balance(), 150);

    assert!(acc.withdraw(30).is_ok());
    assert_eq!(acc.balance(), 120);
}

#[test]
fn test_account_insufficient_balance_error() {
    let mut acc = Account::new(50, 0);

    let result = acc.withdraw(100);
    assert_eq!(
        result,
        Err(StateError::InsufficientBalance {
            available: 50,
            required: 100,
        })
    );
}

#[test]
fn test_nonce_incrementation() {
    let mut acc = Account::new(100, 0);
    acc.increment_nonce();
    assert_eq!(acc.nonce(), 1);
}
