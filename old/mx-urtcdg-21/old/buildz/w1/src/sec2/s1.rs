//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, header, pswg};
use boxy_cli::prelude::*;
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- main ---
pub fn s1_main() {
    // clear_console();
    s2()
    // s2_boxy();
}

// --- Sub ---

// Test s1 sub function
fn s1() {
    // Function header
    let t1 = "src/sec2/s1 main";
    pswg(t1.to_string())
}

// 7 - Representing data with structs

// Deck struct for s2
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
impl Deck {
    // Method for making a deck of cards
    fn new() -> Self {
        // List of suits
        let suits = ["Hearts ♥️", "Diamonds ♦️", "Spades ♠️", "Clubs ♣️"];
        let values = ["Ace", "Jack", "King", "Queen"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value.red(), suit.green()));
            }
        }

        // Instancing the Deck Struct
        Deck { cards }
    }

    // shuffling the deck
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    // Test function - using boxy
    fn smellpanty(&mut self) {
        let mut my_box = Boxy::builder()
            .box_type(BoxType::Double) // Set border style
            .color("#00ffff") // Set border color
            .padding(
                BoxPad::uniform(1),            // External padding
                BoxPad::from_tldr(2, 2, 1, 1), // Internal padding
            )
            .align(BoxAlign::Left) // Center the box in the terminal
            .add_segment("Smell Panty", "#ffffff", BoxAlign::Center)
            .add_segment("Printing vector", "#663399", BoxAlign::Left)
            .width(40) // Set fixed width
            .build();
        my_box.display();
    }

    // deal function
    fn deal(&mut self, num_card: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_card)
    }
}

// #[allow(unused_variables)]
fn s2() {
    let t1 = "7: Representing data with structs";
    pswg(t1.to_string());

    // Generate a new dec
    let mut deck = Deck::new();

    // Shufle the deck
    deck.shuffle();

    // Print the deck with formatting
    for card in &deck.cards {
        println!("{}", card);
    }

    header("Dealing Cards");

    let cardz = deck.deal(2);
    print!("{} dealt cards:\n", cardz.len());
    for card in &cardz {
        println!("{}", card);
    }

    // After creating the instance of the deck
    // deck.smellpanty();
}

// This is like function s2 but with boxy
// Output is not that good. Requires too much tweaking in this case
fn s2_boxy() {
    let t1 = "Cards printed inside box";
    pswg(t1.to_string());

    let deck = Deck::new();

    // Create a single box with all cards
    let mut boxy = BoxyBuilder::default()
        .box_type(BoxType::Single)
        .color("#32CD32")
        .padding(BoxPad::uniform(1), BoxPad::uniform(1))
        .align(BoxAlign::Left)
        // Calculate width based on longest card string + padding
        .width(deck.cards.iter().map(|c| c.len()).max().unwrap_or(20) + 4);

    // Add all cards as one segment with newlines
    let all_cards = deck.cards.join("\n");
    boxy = boxy.add_segment(&all_cards, "#FFFFFF", BoxAlign::Left);

    let mut boxy = boxy.build();
    boxy.display()
}
