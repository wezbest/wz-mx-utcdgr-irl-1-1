//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
// Project Description
/*
Section 3 - Focuses on bank project where you had stoppped
*/
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]


// --- Imports ---
use crate::utilz::{clear_console, header, pswg};
use boxy_cli::prelude::*;
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function---
pub fn s31_main() {
    pswg("Section3".to_string());
    f1();
}

// --- Sun functions ---

// Bank and Account Struct
#[derive(Debug)]
struct Account {
    balance: u32,
    id: i32,
    holder: String,
}
impl Account {
    fn new(id: i32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn f1_print_account(acc: Account) {
    println!("{:#?}", acc.yellow());
}

fn f1() {
    header("Printing Bank");

    let account = Account::new(1, "Dingo".to_string());

    let bank = Bank::new();
    // print!("{:#?}", bank.green());

    header("Printing Account");
    f1_print_account(account);
    f1_print_account(account);
}
