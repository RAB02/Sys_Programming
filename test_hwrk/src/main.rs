use crate::bank_account::BankAccount;

mod bank_account;

fn main() {
    let mut account = BankAccount::new(200.0);
    
    // Deposit some money into the account
    account.deposit(50.0);
    println!("After deposit, balance: {}", account.balance());

    // Withdraw some money from the account
    account.withdraw(30.0);
    println!("After withdrawal, balance: {}", account.balance());

    
}