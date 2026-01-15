/*
Section2 - Code for learning will be here
- This was for learning and is not declared in mod.rs. so wont be used
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function ---
pub fn s2_l() {
    greet();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 2 - Bank Learning ".to_string());
    // make_bank();
    // make_and_print_account();
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
}

// Function to make a new bank

//helper fuinction print account

fn pr_acc(account: &Account) {
    println!("{:#?}", account.yellow());
}

fn make_bank() {
    header("Making a new Bank");

    // Mkaing a new bank
    let bank = Bank::new();
    println!("{:#?}", bank.yellow());

    // making a new account
    let account = Account::new(1, String::from("Alice"));

    // here reference it being made with & which points to the main value
    let account_ref = &account;

    pr_acc(account_ref);

    println!("{:#?}", account.magenta())
}

/*
Lesson 36 Commands
- Learning life times in this section
*/

// fn make_and_print_account() -> &Account {
//     header("Based On Excercise");

//     let account = Account::new(1, String::from("me"));

//     println!("{:#?}", account.yellow());

//     &account
// }
