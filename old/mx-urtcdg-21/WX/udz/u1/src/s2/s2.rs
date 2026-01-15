/*
Section work will be here , wills start with w1.rs and so on also they called from mod.rs
- Simiulates a decsk of playing cards
- This will have the code for the deck
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function ---
pub fn s2_main() {
    greet();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 2 - Bank  ".to_string());
    make_bank();
}

/*
Buiilding the bank struct and accounts
Step 1 - is defining the types which are being done below
*/

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    // Summary function
    fn summary(&self) -> String {
        format!(
            "Account ID: {}, Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        )
    }

    //deposut function
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    // Withdraw function
    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

// the acccounts element in the struct below as a Vector of Account structs which has been defined above
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent implementation of the bank
impl Bank {
    // new acccount information
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    // Add account
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    // Total balance - Using an iterator function
    fn tt_bal(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    // Bank Summary
    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

// Function to make a new bank

//helper fuinction print account

fn pr_acc(account: &Account) {
    println!("{:#?}", account.yellow());
}

fn make_bank() {
    header("Making a new Bank");

    // Making a new bank
    let mut bank = Bank::new();
    let mut account = Account::new(1, "Booty".to_string());

    // Depositing
    account.deposit(50);
    // Withdraw
    account.withdraw(10);

    // Add Account Function
    bank.add_account(account);

    // Printing using the pr_acc created above
    pr_acc(&bank.accounts[0]);

    // Printing using the print macro
    println!("{:#?}", bank.summary());
    println!("{}", bank.tt_bal());
}
