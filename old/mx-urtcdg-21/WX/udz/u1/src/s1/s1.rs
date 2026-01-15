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
pub fn s1_main() {
    greet();
    func1();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 2 - Cards  ".to_string())
}

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// inherent implementation
impl Deck {
    // Inherent implementation
    fn new() -> Self {
        // Alternative method of declaring this Vector
        // let deck = Deck { cards: Vec::new() };

        // List of suits
        let suits = ["hearts", "spades", "clubs", "diamonds"];

        // List of Values
        let values = ["A", "K", "10", "J"];

        // Empty vector to store the cards
        let mut cards = vec![];

        // Doublet nested for loops
        for s in suits {
            for v in values {
                let card = format!("{}-oo-{}", v, s);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    // Shuffling Deck method - needs RNG - Random Number Generator
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    // Deal method
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn func1() {
    header("Deck Function");

    let mut deck = Deck::new();

    deck.shuffle();

    println!("Deck = {:#?}", deck.green());

    // Reqiires error handling
    let hand = deck.deal(2);
    println!(
        "
    ---
    Using deck.del to get cards 
    ---
    Dealt Cards = {:#?}
    ",
        hand.yellow()
    );
}
